use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, Response},
    routing::get,
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::RwLock;
use axum::extract::ConnectInfo;

#[derive(Clone)]
struct RateLimitEntry {
    requests: u32,
    last_reset: Instant,
}

type RateLimitMap = Arc<RwLock<HashMap<SocketAddr, RateLimitEntry>>>;

#[derive(Clone)]
#[allow(dead_code)]
struct ServerConfig {
    max_param_length: usize,
    enable_rate_limiting: bool,
    requests_per_second: u64,
    enable_cors: bool,
    max_request_body_size: usize,
    rate_limiter: Option<RateLimitMap>,
}

#[derive(Parser)]
#[command(name = "hashrand")]
#[command(about = "Generate random hash using base58 alphabet", long_about = None)]
#[command(group = clap::ArgGroup::new("action").args(&["mkdir", "touch"]))]
struct Args {
    /// Length of the hash (between 2 and 128, default: 21)
    #[arg(value_parser = parse_length, default_value = "21", conflicts_with = "api_key")]
    length: usize,

    /// Output without newline character
    #[arg(short = 'r', long = "raw")]
    raw: bool,

    /// Use no look-alike alphabet (excludes: 0, O, I, l, 1)
    #[arg(long = "no-look-alike", conflicts_with_all = &["full", "full_with_symbols"])]
    no_look_alike: bool,

    /// Use full alphanumeric alphabet (uppercase, lowercase, and numbers)
    #[arg(long = "full", conflicts_with_all = &["no_look_alike", "full_with_symbols"])]
    full: bool,

    /// Use full alphabet with symbols (-_*^@#+!?$%)
    #[arg(long = "full-with-symbols", conflicts_with_all = &["no_look_alike", "full"])]
    full_with_symbols: bool,

    /// Check that generated hash doesn't match any existing file or directory name
    #[arg(short = 'c', long = "check")]
    check: bool,

    /// Create a directory with the generated hash as name
    #[arg(long = "mkdir", group = "action")]
    mkdir: bool,

    /// Create a file with the generated hash as name
    #[arg(long = "touch", group = "action")]
    touch: bool,

    /// Prefix to add before the generated hash
    #[arg(long = "prefix")]
    prefix: Option<String>,

    /// Suffix to add after the generated hash
    #[arg(long = "suffix")]
    suffix: Option<String>,

    /// Path where to create the file or directory
    #[arg(long = "path")]
    path: Option<PathBuf>,

    /// Generate API key using full alphabet (ak_ + 44 characters, no customization allowed)
    #[arg(long = "api-key", conflicts_with_all = &["length", "no_look_alike", "full", "full_with_symbols", "password", "check", "mkdir", "touch", "prefix", "suffix", "path"])]
    api_key: bool,

    /// Generate password using full alphabet with symbols (21 characters by default)
    #[arg(long = "password", conflicts_with_all = &["no_look_alike", "full", "full_with_symbols", "api_key", "check", "mkdir", "touch", "prefix", "suffix", "path"])]
    password: bool,

    /// File permissions to use when creating a file (Unix-style octal, e.g., 644)
    #[cfg(unix)]
    #[arg(long = "file-mode", value_parser = parse_mode)]
    file_mode: Option<u32>,

    /// Directory permissions to use when creating a directory (Unix-style octal, e.g., 755)
    #[cfg(unix)]
    #[arg(long = "dir-mode", value_parser = parse_mode)]
    dir_mode: Option<u32>,

    /// Enable audit logging (outputs operations to stderr)
    #[arg(long = "audit-log")]
    audit_log: bool,

    /// Start HTTP server on specified port
    #[arg(short = 's', long = "serve", value_name = "PORT")]
    serve: Option<u16>,

    /// Listen on all network interfaces (0.0.0.0) instead of localhost only (requires --serve)
    #[arg(long = "listen-all-ips", requires = "serve")]
    listen_all_ips: bool,

    /// Maximum length for prefix and suffix parameters in server mode (default: 32)
    #[arg(long = "max-param-length", requires = "serve", default_value = "32")]
    max_param_length: usize,

    /// Enable rate limiting for server mode (default: disabled for better performance)
    #[arg(long = "enable-rate-limiting", requires = "serve")]
    enable_rate_limiting: bool,

    /// Requests per second limit when rate limiting is enabled (default: 100)
    #[arg(long = "rate-limit", requires = "enable_rate_limiting", default_value = "100")]
    rate_limit: u64,

    /// Enable CORS headers for cross-origin requests (default: disabled)
    #[arg(long = "enable-cors", requires = "serve")]
    enable_cors: bool,

    /// Maximum request body size in bytes (default: 1024)
    #[arg(long = "max-body-size", requires = "serve", default_value = "1024")]
    max_body_size: usize,
}

fn parse_length(s: &str) -> Result<usize, String> {
    let length: usize = s.parse().map_err(|_| "Length must be a number")?;

    if !(2..=128).contains(&length) {
        return Err("Length must be between 2 and 128".to_string());
    }

    Ok(length)
}

fn validate_query_params(prefix: &Option<String>, suffix: &Option<String>, max_length: usize) -> Result<(), String> {
    if let Some(p) = prefix {
        if p.len() > max_length {
            return Err(format!("Prefix length ({}) exceeds maximum allowed ({})", p.len(), max_length));
        }
    }
    
    if let Some(s) = suffix {
        if s.len() > max_length {
            return Err(format!("Suffix length ({}) exceeds maximum allowed ({})", s.len(), max_length));
        }
    }
    
    Ok(())
}

async fn check_rate_limit(
    rate_limiter: &RateLimitMap,
    addr: SocketAddr,
    requests_per_second: u64,
) -> bool {
    let mut limiter = rate_limiter.write().await;
    let now = Instant::now();
    
    let entry = limiter.entry(addr).or_insert(RateLimitEntry {
        requests: 0,
        last_reset: now,
    });
    
    // Reset counter if a second has passed
    if now.duration_since(entry.last_reset) >= Duration::from_secs(1) {
        entry.requests = 0;
        entry.last_reset = now;
    }
    
    if entry.requests >= requests_per_second as u32 {
        false // Rate limit exceeded
    } else {
        entry.requests += 1;
        true // Request allowed
    }
}

#[cfg(unix)]
fn parse_mode(s: &str) -> Result<u32, String> {
    // Parse octal string (e.g., "644" or "0644")
    let s = s.trim_start_matches('0');
    u32::from_str_radix(s, 8)
        .map_err(|_| format!("Invalid mode '{}'. Use octal format like 644 or 755", s))
}

fn check_name_exists(name: &str, current_dir: &Path, max_depth: Option<usize>) -> bool {
    let mut walker = WalkDir::new(current_dir);
    
    // Apply depth limit if specified
    if let Some(depth) = max_depth {
        walker = walker.max_depth(depth);
    }
    
    let mut entries_checked = 0;
    const MAX_ENTRIES: usize = 100_000; // Prevent resource exhaustion
    
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        entries_checked += 1;
        if entries_checked > MAX_ENTRIES {
            eprintln!("Warning: Directory traversal limit reached ({}). Collision check may be incomplete.", MAX_ENTRIES);
            break;
        }
        
        if let Some(file_name) = entry.file_name().to_str() {
            if file_name == name {
                return true;
            }
        }
    }
    false
}

fn generate_unique_name(
    alphabet: &[char],
    length: usize,
    prefix: Option<&str>,
    suffix: Option<&str>,
    current_dir: &Path,
) -> String {
    const MAX_ATTEMPTS: usize = 1000;
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        if attempts > MAX_ATTEMPTS {
            eprintln!("Warning: Generated {} hashes without finding unique name. Using last generated.", MAX_ATTEMPTS);
        }
        
        let hash = nanoid::format(nanoid::rngs::default, alphabet, length);
        let full_name = format!(
            "{}{}{}",
            prefix.unwrap_or(""),
            hash,
            suffix.unwrap_or("")
        );
        
        // Use depth limit of 10 for collision checking to prevent deep traversal
        if !check_name_exists(&full_name, current_dir, Some(10)) || attempts > MAX_ATTEMPTS {
            return hash;
        }
    }
}

#[derive(Debug, Clone)]
pub enum AlphabetType {
    Base58,
    NoLookAlike,
    Full,
    FullWithSymbols,
}

#[derive(Debug)]
pub struct HashRequest {
    pub length: usize,
    pub alphabet: AlphabetType,
    pub raw: bool,
    pub check: bool,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

#[derive(Deserialize)]
struct GenerateQuery {
    length: Option<usize>,
    alphabet: Option<String>,
    raw: Option<bool>,
    prefix: Option<String>,
    suffix: Option<String>,
}

#[derive(Deserialize)]
struct ApiKeyQuery {
    raw: Option<bool>,
}

#[derive(Deserialize)]
struct PasswordQuery {
    length: Option<usize>,
    raw: Option<bool>,
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();
    
    // If server mode is enabled, start the HTTP server
    if let Some(port) = args.serve {
        return start_server(
            port,
            args.listen_all_ips,
            args.max_param_length,
            args.enable_rate_limiting,
            args.rate_limit,
            args.enable_cors,
            args.max_body_size,
        ).await;
    }
    
    // Check environment variable for audit logging
    if !args.audit_log && std::env::var("HASHRAND_AUDIT_LOG").is_ok() {
        args.audit_log = true;
    }
    
    // Audit logging helper
    let audit_log = |msg: &str| {
        if args.audit_log {
            use std::time::SystemTime;
            let now = SystemTime::now();
            if let Ok(duration) = now.duration_since(SystemTime::UNIX_EPOCH) {
                eprintln!("[AUDIT] {}: {}", duration.as_secs(), msg);
            } else {
                eprintln!("[AUDIT] {}", msg);
            }
        }
    };
    
    audit_log(&format!("hashrand started with args: length={}, check={}, mkdir={}, touch={}, api_key={}, password={}",
        args.length, args.check, args.mkdir, args.touch, args.api_key, args.password));

    // Set fixed length for api-key and default for password
    if args.api_key {
        args.length = 44;
    } else if args.password {
        // For password mode, validate the length range (21-44)
        if args.length == 21 {
            // Use default password length (already 21)
            // No need to change args.length
        } else if args.length < 21 || args.length > 44 {
            eprintln!("Error: Password length must be between 21 and 44 characters");
            std::process::exit(1);
        }
    }

    // Select alphabet based on arguments
    let alphabet_type = if args.no_look_alike {
        AlphabetType::NoLookAlike
    } else if args.full {
        AlphabetType::Full
    } else if args.full_with_symbols {
        AlphabetType::FullWithSymbols
    } else if args.api_key {
        AlphabetType::Full
    } else if args.password {
        AlphabetType::FullWithSymbols
    } else {
        AlphabetType::Base58
    };

    let alphabet = get_alphabet(&alphabet_type);

    // Determine the base path for operations with validation
    let base_path = if let Some(ref p) = args.path {
        // Validate and canonicalize the path to prevent traversal attacks
        let canonical_path = p.canonicalize()
            .map_err(|e| format!("Invalid path '{}': {}", p.display(), e))?;
        
        // Ensure the path exists and is a directory
        if !canonical_path.is_dir() {
            return Err(format!("Path '{}' is not a directory", canonical_path.display()).into());
        }
        
        canonical_path
    } else {
        std::env::current_dir()?
    };

    // Check is implicit when using mkdir or touch
    let should_check = args.check || args.mkdir || args.touch;

    let hash = if should_check {
        audit_log(&format!("Generating unique name with collision checking in {}", base_path.display()));
        generate_unique_name(
            alphabet,
            args.length,
            args.prefix.as_deref(),
            args.suffix.as_deref(),
            &base_path,
        )
    } else {
        audit_log("Generating random hash without collision checking");
        nanoid::format(nanoid::rngs::default, alphabet, args.length)
    };

    // Build the full name with optional prefix and suffix
    let full_name = if args.api_key {
        // API keys get the 'ak_' prefix
        format!("ak_{}", hash)
    } else {
        format!(
            "{}{}{}",
            args.prefix.as_deref().unwrap_or(""),
            hash,
            args.suffix.as_deref().unwrap_or("")
        )
    };

    if args.mkdir {
        // Create directory with error handling
        let dir_path = base_path.join(&full_name);
        
        // Validate the final path stays within base_path
        let canonical_base = base_path.canonicalize()?;
        let parent_dir = dir_path.parent()
            .ok_or("Invalid directory path")?;
        let canonical_parent = parent_dir.canonicalize()
            .unwrap_or_else(|_| canonical_base.clone());
        
        if !canonical_parent.starts_with(&canonical_base) {
            return Err("Path traversal detected: directory would be created outside of base path".into());
        }
        
        audit_log(&format!("Creating directory: {}", dir_path.display()));
        fs::create_dir(&dir_path)
            .map_err(|e| format!("Failed to create directory '{}': {}", dir_path.display(), e))?;
        
        // Set directory permissions if specified (Unix only)
        #[cfg(unix)]
        if let Some(mode) = args.dir_mode {
            audit_log(&format!("Setting directory permissions to {:o}", mode));
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::Permissions::from_mode(mode);
            fs::set_permissions(&dir_path, perms)
                .map_err(|e| format!("Failed to set directory permissions: {}", e))?;
        }
        
        audit_log(&format!("Successfully created directory: {}", dir_path.display()));
        println!("{}", dir_path.display());
    } else if args.touch {
        // Create file with error handling
        let file_path = base_path.join(&full_name);
        
        // Validate the final path stays within base_path  
        let canonical_base = base_path.canonicalize()?;
        let parent_dir = file_path.parent()
            .ok_or("Invalid file path")?;
        let canonical_parent = parent_dir.canonicalize()
            .unwrap_or_else(|_| canonical_base.clone());
        
        if !canonical_parent.starts_with(&canonical_base) {
            return Err("Path traversal detected: file would be created outside of base path".into());
        }
        
        audit_log(&format!("Creating file: {}", file_path.display()));
        fs::File::create(&file_path)
            .map_err(|e| format!("Failed to create file '{}': {}", file_path.display(), e))?;
        
        // Set file permissions if specified (Unix only)
        #[cfg(unix)]
        if let Some(mode) = args.file_mode {
            audit_log(&format!("Setting file permissions to {:o}", mode));
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::Permissions::from_mode(mode);
            fs::set_permissions(&file_path, perms)
                .map_err(|e| format!("Failed to set file permissions: {}", e))?;
        }
        
        audit_log(&format!("Successfully created file: {}", file_path.display()));
        println!("{}", file_path.display());
    } else {
        // Just output the hash
        audit_log(&format!("Generated hash: {} (length: {})", full_name, full_name.len()));
        if args.raw {
            print!("{full_name}");
        } else {
            println!("{full_name}");
        }
    }
    
    Ok(())
}

fn get_alphabet(alphabet_type: &AlphabetType) -> &'static [char] {
    // Base58 alphabet (Bitcoin alphabet) - default
    const BASE58_ALPHABET: [char; 58] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J',
        'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c',
        'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
        'w', 'x', 'y', 'z',
    ];

    // No look-alike alphabet (excludes: 0, O, I, l, 1)
    const NO_LOOK_ALIKE_ALPHABET: [char; 57] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
        'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd',
        'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
        'x', 'y', 'z',
    ];

    // Full alphanumeric alphabet
    const FULL_ALPHABET: [char; 62] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    // Full alphabet with symbols
    const FULL_WITH_SYMBOLS_ALPHABET: [char; 73] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '-', '_', '*', '^', '@', '#', '+', '!', '?', '$',
        '%',
    ];

    match alphabet_type {
        AlphabetType::Base58 => &BASE58_ALPHABET,
        AlphabetType::NoLookAlike => &NO_LOOK_ALIKE_ALPHABET,
        AlphabetType::Full => &FULL_ALPHABET,
        AlphabetType::FullWithSymbols => &FULL_WITH_SYMBOLS_ALPHABET,
    }
}

const WEB_INTERFACE_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>HashRand - Random Hash Generator</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        
        .container {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
            overflow: hidden;
        }
        
        .header {
            background: #2c3e50;
            color: white;
            padding: 2rem;
            text-align: center;
        }
        
        .header h1 {
            font-size: 2.5rem;
            margin-bottom: 0.5rem;
            font-weight: 700;
        }
        
        .header p {
            opacity: 0.9;
            font-size: 1.1rem;
        }
        
        .content {
            padding: 2rem;
        }
        
        .menu-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 2rem;
            margin: 2rem 0;
        }
        
        .menu-card {
            background: white;
            border: 2px solid #e1e8ed;
            border-radius: 12px;
            padding: 2rem;
            text-align: center;
            cursor: pointer;
            transition: all 0.3s ease;
            position: relative;
            overflow: hidden;
        }
        
        .menu-card::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            opacity: 0;
            transition: opacity 0.3s ease;
        }
        
        .menu-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 12px 24px rgba(0, 0, 0, 0.15);
            border-color: #667eea;
        }
        
        .menu-card:hover::before {
            opacity: 0.05;
        }
        
        .menu-icon {
            font-size: 3rem;
            margin-bottom: 1rem;
            position: relative;
            z-index: 1;
        }
        
        .menu-card h3 {
            color: #2c3e50;
            margin-bottom: 0.5rem;
            font-size: 1.5rem;
            position: relative;
            z-index: 1;
        }
        
        .menu-card p {
            color: #7f8c8d;
            font-size: 0.95rem;
            position: relative;
            z-index: 1;
        }
        
        .view-container {
            display: none;
        }
        
        .view-container.active {
            display: block;
            animation: fadeIn 0.3s ease;
        }
        
        @keyframes fadeIn {
            from { 
                opacity: 0; 
                transform: translateY(10px); 
            }
            to { 
                opacity: 1; 
                transform: translateY(0); 
            }
        }
        
        .back-button {
            background: transparent;
            border: 2px solid #667eea;
            color: #667eea;
            margin-bottom: 1.5rem;
            width: auto;
            padding: 10px 20px;
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            font-weight: 600;
        }
        
        .back-button:hover {
            background: #667eea;
            color: white;
            transform: translateY(0);
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
        }
        
        .form-section {
            margin-bottom: 2rem;
        }
        
        .form-section h2 {
            color: #2c3e50;
            margin-bottom: 1rem;
            font-size: 1.3rem;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }
        
        .form-group {
            margin-bottom: 1.5rem;
        }
        
        label {
            display: block;
            margin-bottom: 0.5rem;
            font-weight: 600;
            color: #34495e;
        }
        
        input, select, button {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e1e8ed;
            border-radius: 8px;
            font-size: 1rem;
            transition: all 0.3s ease;
        }
        
        input:focus, select:focus {
            outline: none;
            border-color: #667eea;
            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }
        
        .range-group {
            display: flex;
            align-items: center;
            gap: 1rem;
        }
        
        input[type="range"] {
            flex: 1;
        }
        
        .range-value {
            background: #667eea;
            color: white;
            padding: 8px 12px;
            border-radius: 6px;
            font-weight: 600;
            min-width: 60px;
            text-align: center;
        }
        
        .button-group {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-top: 1.5rem;
        }
        
        button {
            background: #667eea;
            color: white;
            border: none;
            cursor: pointer;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            transition: all 0.3s ease;
        }
        
        button:hover:not(:disabled) {
            background: #5a67d8;
            transform: translateY(-2px);
            box-shadow: 0 8px 20px rgba(102, 126, 234, 0.3);
        }
        
        button:active {
            transform: translateY(0);
        }
        
        button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
        
        .special-buttons {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 1rem;
        }
        
        .api-key-btn {
            background: #27ae60 !important;
        }
        
        .api-key-btn:hover {
            background: #229954 !important;
        }
        
        .password-btn {
            background: #e74c3c !important;
        }
        
        .password-btn:hover {
            background: #c0392b !important;
        }
        
        .result-section {
            background: #f8f9fa;
            border-radius: 8px;
            padding: 1.5rem;
            margin-top: 2rem;
            min-height: 120px;
        }
        
        .result-section h3 {
            color: #2c3e50;
            margin-bottom: 1rem;
        }
        
        .result-display {
            background: white;
            border: 2px solid #e1e8ed;
            border-radius: 6px;
            padding: 1rem;
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            font-size: 1.1rem;
            word-break: break-all;
            min-height: 60px;
            display: flex;
            align-items: center;
            position: relative;
        }
        
        .result-display.success {
            border-color: #27ae60;
            background: #d5f4e6;
            color: #27ae60;
        }
        
        .result-display.error {
            border-color: #e74c3c;
            background: #f8d7da;
            color: #e74c3c;
        }
        
        .copy-btn {
            position: absolute;
            top: 8px;
            right: 8px;
            width: auto;
            padding: 6px 12px;
            font-size: 0.8rem;
            background: #667eea;
            border-radius: 4px;
        }
        
        .loading {
            display: inline-block;
            width: 20px;
            height: 20px;
            border: 3px solid #f3f3f3;
            border-top: 3px solid #667eea;
            border-radius: 50%;
            animation: spin 1s linear infinite;
            margin-right: 10px;
        }
        
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
        
        .info-box {
            background: #e3f2fd;
            border: 1px solid #bbdefb;
            border-radius: 6px;
            padding: 1rem;
            margin: 1rem 0;
            font-size: 0.9rem;
            color: #1976d2;
        }
        
        @media (max-width: 768px) {
            body {
                padding: 10px;
            }
            
            .header {
                padding: 1.5rem 1rem;
            }
            
            .header h1 {
                font-size: 2rem;
            }
            
            .content {
                padding: 1.5rem;
            }
            
            .button-group {
                grid-template-columns: 1fr;
            }
            
            .special-buttons {
                grid-template-columns: 1fr;
            }
            
            .range-group {
                flex-direction: column;
                align-items: stretch;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🎲 HashRand</h1>
            <p>Secure Random Hash Generator with Multiple Alphabets</p>
        </div>
        
        <div class="content">
            <hash-generator></hash-generator>
        </div>
    </div>
    
    <script>
        class HashGenerator extends HTMLElement {
            constructor() {
                super();
                this.attachShadow({ mode: 'open' });
                this.currentView = 'menu';
                this.render();
                this.bindEvents();
            }
            
            render() {
                this.shadowRoot.innerHTML = `
                    <style>
                        :host {
                            display: block;
                        }
                        
                        /* Menu styles */
                        .menu-grid {
                            display: grid;
                            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                            gap: 2rem;
                            margin: 2rem 0;
                        }
                        
                        .menu-card {
                            background: white;
                            border: 2px solid #e1e8ed;
                            border-radius: 12px;
                            padding: 2rem;
                            text-align: center;
                            cursor: pointer;
                            transition: all 0.3s ease;
                            position: relative;
                            overflow: hidden;
                        }
                        
                        .menu-card::before {
                            content: '';
                            position: absolute;
                            top: 0;
                            left: 0;
                            right: 0;
                            bottom: 0;
                            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                            opacity: 0;
                            transition: opacity 0.3s ease;
                        }
                        
                        .menu-card:hover {
                            transform: translateY(-5px);
                            box-shadow: 0 12px 24px rgba(0, 0, 0, 0.15);
                            border-color: #667eea;
                        }
                        
                        .menu-card:hover::before {
                            opacity: 0.05;
                        }
                        
                        .menu-icon {
                            font-size: 3rem;
                            margin-bottom: 1rem;
                            position: relative;
                            z-index: 1;
                        }
                        
                        .menu-card h3 {
                            color: #2c3e50;
                            margin-bottom: 0.5rem;
                            font-size: 1.5rem;
                            position: relative;
                            z-index: 1;
                        }
                        
                        .menu-card p {
                            color: #7f8c8d;
                            font-size: 0.95rem;
                            position: relative;
                            z-index: 1;
                            margin: 0;
                        }
                        
                        /* View container styles */
                        .view-container {
                            display: none;
                        }
                        
                        .view-container.active {
                            display: block;
                            animation: fadeIn 0.3s ease;
                        }
                        
                        @keyframes fadeIn {
                            from { 
                                opacity: 0; 
                                transform: translateY(10px); 
                            }
                            to { 
                                opacity: 1; 
                                transform: translateY(0); 
                            }
                        }
                        
                        /* Back button styles */
                        .back-button {
                            background: transparent;
                            border: 2px solid #667eea;
                            color: #667eea;
                            margin-bottom: 1.5rem;
                            width: auto;
                            padding: 10px 20px;
                            display: inline-flex;
                            align-items: center;
                            gap: 0.5rem;
                            font-weight: 600;
                            cursor: pointer;
                            border-radius: 8px;
                            transition: all 0.3s ease;
                        }
                        
                        .back-button:hover {
                            background: #667eea;
                            color: white;
                            transform: translateY(0);
                            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
                        }
                        
                        /* Form styles */
                        .form-section {
                            margin-bottom: 2rem;
                        }
                        
                        .form-section h2 {
                            color: #2c3e50;
                            margin-bottom: 1.5rem;
                            font-size: 1.5rem;
                            display: flex;
                            align-items: center;
                            gap: 0.5rem;
                        }
                        
                        .form-group {
                            margin-bottom: 1.5rem;
                        }
                        
                        label {
                            display: block;
                            margin-bottom: 0.5rem;
                            font-weight: 600;
                            color: #34495e;
                        }
                        
                        input, select, button {
                            width: 100%;
                            padding: 12px 16px;
                            border: 2px solid #e1e8ed;
                            border-radius: 8px;
                            font-size: 1rem;
                            transition: all 0.3s ease;
                        }
                        
                        input:focus, select:focus {
                            outline: none;
                            border-color: #667eea;
                            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
                        }
                        
                        .range-group {
                            display: flex;
                            align-items: center;
                            gap: 1rem;
                        }
                        
                        input[type="range"] {
                            flex: 1;
                        }
                        
                        .range-value {
                            background: #667eea;
                            color: white;
                            padding: 8px 12px;
                            border-radius: 6px;
                            font-weight: 600;
                            min-width: 60px;
                            text-align: center;
                        }
                        
                        .button-group {
                            display: grid;
                            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                            gap: 1rem;
                            margin-top: 1.5rem;
                        }
                        
                        button {
                            background: #667eea;
                            color: white;
                            border: none;
                            cursor: pointer;
                            font-weight: 600;
                            text-transform: uppercase;
                            letter-spacing: 0.5px;
                            transition: all 0.3s ease;
                        }
                        
                        button:hover:not(:disabled) {
                            background: #5a67d8;
                            transform: translateY(-2px);
                            box-shadow: 0 8px 20px rgba(102, 126, 234, 0.3);
                        }
                        
                        button:active {
                            transform: translateY(0);
                        }
                        
                        button:disabled {
                            opacity: 0.6;
                            cursor: not-allowed;
                        }
                        
                        .api-key-btn {
                            background: #27ae60 !important;
                        }
                        
                        .api-key-btn:hover {
                            background: #229954 !important;
                        }
                        
                        .password-btn {
                            background: #e74c3c !important;
                        }
                        
                        .password-btn:hover {
                            background: #c0392b !important;
                        }
                        
                        .info-box {
                            background: #e3f2fd;
                            border: 1px solid #bbdefb;
                            border-radius: 6px;
                            padding: 1rem;
                            margin: 1rem 0;
                            font-size: 0.9rem;
                            color: #1976d2;
                        }
                        
                        .result-section {
                            background: #f8f9fa;
                            border-radius: 8px;
                            padding: 1.5rem;
                            margin-top: 2rem;
                            min-height: 120px;
                        }
                        
                        .result-section h3 {
                            color: #2c3e50;
                            margin-bottom: 1rem;
                        }
                        
                        .result-display {
                            background: white;
                            border: 2px solid #e1e8ed;
                            border-radius: 6px;
                            padding: 1rem;
                            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
                            font-size: 1.1rem;
                            word-break: break-all;
                            min-height: 60px;
                            display: flex;
                            align-items: center;
                            position: relative;
                        }
                        
                        .result-display.success {
                            border-color: #27ae60;
                            background: #d5f4e6;
                            color: #27ae60;
                        }
                        
                        .result-display.error {
                            border-color: #e74c3c;
                            background: #f8d7da;
                            color: #e74c3c;
                        }
                        
                        .copy-btn {
                            position: absolute;
                            top: 8px;
                            right: 8px;
                            width: auto;
                            padding: 6px 12px;
                            font-size: 0.8rem;
                            background: #667eea;
                            border-radius: 4px;
                        }
                        
                        .loading {
                            display: inline-block;
                            width: 20px;
                            height: 20px;
                            border: 3px solid #f3f3f3;
                            border-top: 3px solid #667eea;
                            border-radius: 50%;
                            animation: spin 1s linear infinite;
                            margin-right: 10px;
                        }
                        
                        @keyframes spin {
                            0% { transform: rotate(0deg); }
                            100% { transform: rotate(360deg); }
                        }
                        
                        @media (max-width: 768px) {
                            .menu-grid {
                                grid-template-columns: 1fr;
                            }
                            
                            .button-group {
                                grid-template-columns: 1fr;
                            }
                            
                            .range-group {
                                flex-direction: column;
                                align-items: stretch;
                            }
                        }
                    </style>
                    
                    <!-- Menu View -->
                    <div id="menuView" class="view-container active">
                        <h2 style="text-align: center; color: #2c3e50; margin-bottom: 2rem;">Choose Generation Mode</h2>
                        <div class="menu-grid">
                            <div class="menu-card" data-mode="generic">
                                <div class="menu-icon">🎲</div>
                                <h3>Generic Hash</h3>
                                <p>Customizable random strings with various alphabets and options</p>
                            </div>
                            <div class="menu-card" data-mode="password">
                                <div class="menu-icon">🔐</div>
                                <h3>Password</h3>
                                <p>Strong passwords with symbols (21-44 characters)</p>
                            </div>
                            <div class="menu-card" data-mode="apiKey">
                                <div class="menu-icon">🔑</div>
                                <h3>API Key</h3>
                                <p>Secure API keys with ak_ prefix (256-bit entropy)</p>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Generic Hash View -->
                    <div id="genericView" class="view-container">
                        <button class="back-button" data-target="menu">← Back to Menu</button>
                        <div class="form-section">
                            <h2>🎲 Generic Hash Generator</h2>
                            <div class="form-group">
                                <label for="length">Hash Length (2-128):</label>
                                <div class="range-group">
                                    <input type="range" id="length" min="2" max="128" value="21">
                                    <div class="range-value" id="lengthValue">21</div>
                                </div>
                            </div>
                            
                            <div class="form-group">
                                <label for="alphabet">Alphabet Type:</label>
                                <select id="alphabet">
                                    <option value="base58">Base58 (Bitcoin) - Default</option>
                                    <option value="no-look-alike">No Look-Alike (No 0,O,I,l,1)</option>
                                    <option value="full">Full Alphanumeric</option>
                                    <option value="full-with-symbols">Full with Symbols</option>
                                </select>
                            </div>
                            
                            <div class="form-group">
                                <label for="prefix">Prefix (optional):</label>
                                <input type="text" id="prefix" placeholder="e.g., tmp_, file_">
                            </div>
                            
                            <div class="form-group">
                                <label for="suffix">Suffix (optional):</label>
                                <input type="text" id="suffix" placeholder="e.g., _temp, .tmp">
                            </div>
                            
                            <div class="info-box">
                                <strong>💡 Tip:</strong> Base58 avoids confusing characters. Choose alphabet based on your use case.
                            </div>
                            
                            <div class="button-group">
                                <button id="generateBtn" class="generate-button">🎲 Generate Hash</button>
                            </div>
                        </div>
                        
                        <div class="result-section" style="display: none;" id="genericResult">
                            <h3>📋 Generated Result</h3>
                            <div class="result-display" id="genericResultDisplay">
                                <button class="copy-btn" id="genericCopyBtn" style="display: none;">Copy</button>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Password View -->
                    <div id="passwordView" class="view-container">
                        <button class="back-button" data-target="menu">← Back to Menu</button>
                        <div class="form-section">
                            <h2>🔐 Password Generator</h2>
                            <div class="form-group">
                                <label for="passwordLength">Password Length (21-44):</label>
                                <div class="range-group">
                                    <input type="range" id="passwordLength" min="21" max="44" value="21">
                                    <div class="range-value" id="passwordLengthValue">21</div>
                                </div>
                            </div>
                            
                            <div class="info-box">
                                <strong>🔒 Security:</strong> Uses full alphabet with symbols for maximum entropy. Minimum 21 characters ensures strong security.
                            </div>
                            
                            <div class="button-group">
                                <button id="passwordBtn" class="password-btn">🔐 Generate Password</button>
                            </div>
                        </div>
                        
                        <div class="result-section" style="display: none;" id="passwordResult">
                            <h3>📋 Generated Password</h3>
                            <div class="result-display" id="passwordResultDisplay">
                                <button class="copy-btn" id="passwordCopyBtn" style="display: none;">Copy</button>
                            </div>
                        </div>
                    </div>
                    
                    <!-- API Key View -->
                    <div id="apiKeyView" class="view-container">
                        <button class="back-button" data-target="menu">← Back to Menu</button>
                        <div class="form-section">
                            <h2>🔑 API Key Generator</h2>
                            
                            <div class="info-box">
                                <strong>📐 Format:</strong> ak_ prefix + 44 random characters<br>
                                <strong>🔒 Security:</strong> 256-bit entropy, quantum-resistant<br>
                                <strong>📝 Alphabet:</strong> Full alphanumeric (A-Z, a-z, 0-9)
                            </div>
                            
                            <div class="button-group">
                                <button id="apiKeyBtn" class="api-key-btn">🔑 Generate API Key</button>
                            </div>
                        </div>
                        
                        <div class="result-section" style="display: none;" id="apiKeyResult">
                            <h3>📋 Generated API Key</h3>
                            <div class="result-display" id="apiKeyResultDisplay">
                                <button class="copy-btn" id="apiKeyCopyBtn" style="display: none;">Copy</button>
                            </div>
                        </div>
                    </div>
                `;
            }
            
            bindEvents() {
                // Menu navigation
                this.shadowRoot.querySelectorAll('.menu-card').forEach(card => {
                    card.addEventListener('click', () => {
                        const mode = card.dataset.mode;
                        this.switchView(mode);
                    });
                });
                
                // Back buttons
                this.shadowRoot.querySelectorAll('.back-button').forEach(btn => {
                    btn.addEventListener('click', () => {
                        this.switchView('menu');
                    });
                });
                
                // Generic Hash controls
                const lengthSlider = this.shadowRoot.getElementById('length');
                const lengthValue = this.shadowRoot.getElementById('lengthValue');
                if (lengthSlider) {
                    lengthSlider.addEventListener('input', (e) => {
                        lengthValue.textContent = e.target.value;
                    });
                }
                
                // Password controls
                const passwordLengthSlider = this.shadowRoot.getElementById('passwordLength');
                const passwordLengthValue = this.shadowRoot.getElementById('passwordLengthValue');
                if (passwordLengthSlider) {
                    passwordLengthSlider.addEventListener('input', (e) => {
                        passwordLengthValue.textContent = e.target.value;
                    });
                }
                
                // Generate buttons
                const generateBtn = this.shadowRoot.getElementById('generateBtn');
                if (generateBtn) {
                    generateBtn.addEventListener('click', () => {
                        this.generateHash();
                    });
                }
                
                const passwordBtn = this.shadowRoot.getElementById('passwordBtn');
                if (passwordBtn) {
                    passwordBtn.addEventListener('click', () => {
                        this.generatePassword();
                    });
                }
                
                const apiKeyBtn = this.shadowRoot.getElementById('apiKeyBtn');
                if (apiKeyBtn) {
                    apiKeyBtn.addEventListener('click', () => {
                        this.generateApiKey();
                    });
                }
                
                // Copy buttons
                this.shadowRoot.querySelectorAll('.copy-btn').forEach(btn => {
                    btn.addEventListener('click', (e) => {
                        const resultId = btn.id.replace('CopyBtn', 'ResultDisplay');
                        this.copyResult(resultId, btn.id);
                    });
                });
            }
            
            switchView(viewName) {
                // Hide all views
                this.shadowRoot.querySelectorAll('.view-container').forEach(view => {
                    view.classList.remove('active');
                });
                
                // Show the selected view
                const targetView = this.shadowRoot.getElementById(`${viewName}View`);
                if (targetView) {
                    targetView.classList.add('active');
                }
                
                this.currentView = viewName;
            }
            
            async makeRequest(url, resultSectionId, resultDisplayId, copyBtnId) {
                const resultSection = this.shadowRoot.getElementById(resultSectionId);
                const resultDiv = this.shadowRoot.getElementById(resultDisplayId);
                const copyBtn = this.shadowRoot.getElementById(copyBtnId);
                
                resultSection.style.display = 'block';
                
                // Create or update loading indicator without destroying the copy button
                let loadingSpan = resultDiv.querySelector('.result-text');
                if (!loadingSpan) {
                    loadingSpan = document.createElement('span');
                    loadingSpan.className = 'result-text';
                    resultDiv.insertBefore(loadingSpan, copyBtn);
                }
                
                loadingSpan.innerHTML = '<div class="loading"></div>Generating...';
                resultDiv.className = 'result-display';
                copyBtn.style.display = 'none';
                
                try {
                    const response = await fetch(url);
                    const text = await response.text();
                    
                    if (response.ok) {
                        loadingSpan.textContent = text;
                        resultDiv.className = 'result-display success';
                        copyBtn.style.display = 'block';
                    } else {
                        loadingSpan.textContent = `Error: ${response.status} - ${text || 'Request failed'}`;
                        resultDiv.className = 'result-display error';
                        copyBtn.style.display = 'none';
                    }
                } catch (error) {
                    loadingSpan.textContent = `Network error: ${error.message}`;
                    resultDiv.className = 'result-display error';
                    copyBtn.style.display = 'none';
                }
            }
            
            generateHash() {
                const length = this.shadowRoot.getElementById('length').value;
                const alphabet = this.shadowRoot.getElementById('alphabet').value;
                const prefix = this.shadowRoot.getElementById('prefix').value;
                const suffix = this.shadowRoot.getElementById('suffix').value;
                
                let url = `/api/generate?length=${length}&alphabet=${alphabet}`;
                if (prefix) url += `&prefix=${encodeURIComponent(prefix)}`;
                if (suffix) url += `&suffix=${encodeURIComponent(suffix)}`;
                
                this.makeRequest(url, 'genericResult', 'genericResultDisplay', 'genericCopyBtn');
            }
            
            generateApiKey() {
                this.makeRequest('/api/api-key', 'apiKeyResult', 'apiKeyResultDisplay', 'apiKeyCopyBtn');
            }
            
            generatePassword() {
                const length = this.shadowRoot.getElementById('passwordLength').value;
                this.makeRequest(`/api/password?length=${length}`, 'passwordResult', 'passwordResultDisplay', 'passwordCopyBtn');
            }
            
            copyResult(resultDisplayId, copyBtnId) {
                const resultDiv = this.shadowRoot.getElementById(resultDisplayId);
                const resultSpan = resultDiv.querySelector('.result-text');
                const resultText = resultSpan ? resultSpan.textContent : '';
                
                if (resultText && resultText !== 'Generating...') {
                    navigator.clipboard.writeText(resultText).then(() => {
                        const copyBtn = this.shadowRoot.getElementById(copyBtnId);
                        const originalText = copyBtn.textContent;
                        copyBtn.textContent = 'Copied!';
                        setTimeout(() => {
                            copyBtn.textContent = originalText;
                        }, 2000);
                    });
                }
            }
        }
        
        customElements.define('hash-generator', HashGenerator);
    </script>
</body>
</html>"#;

fn generate_hash_from_request(request: &HashRequest) -> Result<String, Box<dyn std::error::Error>> {
    let alphabet = get_alphabet(&request.alphabet);
    let current_dir = std::env::current_dir()?;
    
    let hash = if request.check {
        generate_unique_name(
            alphabet,
            request.length,
            request.prefix.as_deref(),
            request.suffix.as_deref(),
            &current_dir,
        )
    } else {
        nanoid::format(nanoid::rngs::default, alphabet, request.length)
    };

    let full_name = format!(
        "{}{}{}",
        request.prefix.as_deref().unwrap_or(""),
        hash,
        request.suffix.as_deref().unwrap_or("")
    );

    Ok(if request.raw {
        full_name
    } else {
        format!("{}\n", full_name)
    })
}

fn generate_api_key_response(raw: bool) -> Result<String, Box<dyn std::error::Error>> {
    let alphabet = get_alphabet(&AlphabetType::Full);
    let hash = nanoid::format(nanoid::rngs::default, alphabet, 44);
    let api_key = format!("ak_{}", hash);
    
    Ok(if raw {
        api_key
    } else {
        format!("{}\n", api_key)
    })
}

fn generate_password_response(length: Option<usize>, raw: bool) -> Result<String, Box<dyn std::error::Error>> {
    let length = length.unwrap_or(21);
    
    if length < 21 || length > 44 {
        return Err("Password length must be between 21 and 44 characters".into());
    }
    
    let alphabet = get_alphabet(&AlphabetType::FullWithSymbols);
    let password = nanoid::format(nanoid::rngs::default, alphabet, length);
    
    Ok(if raw {
        password
    } else {
        format!("{}\n", password)
    })
}

async fn handle_generate(
    State(config): State<Arc<ServerConfig>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<GenerateQuery>
) -> Result<Response<String>, StatusCode> {
    // Check rate limiting if enabled
    if let Some(ref rate_limiter) = config.rate_limiter {
        if !check_rate_limit(rate_limiter, addr, config.requests_per_second).await {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }
    let length = params.length.unwrap_or(21);
    
    if !(2..=128).contains(&length) {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Validate parameter lengths
    if let Err(_) = validate_query_params(&params.prefix, &params.suffix, config.max_param_length) {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let alphabet = match params.alphabet.as_deref() {
        Some("base58") | None => AlphabetType::Base58,
        Some("no-look-alike") => AlphabetType::NoLookAlike,
        Some("full") => AlphabetType::Full,
        Some("full-with-symbols") => AlphabetType::FullWithSymbols,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    let request = HashRequest {
        length,
        alphabet,
        raw: params.raw.unwrap_or(true),  // Default to true in server mode
        check: false,  // Never check in server mode (no filesystem access)
        prefix: params.prefix,
        suffix: params.suffix,
    };
    
    match generate_hash_from_request(&request) {
        Ok(result) => Ok(Response::builder()
            .header("content-type", "text/plain")
            .body(result)
            .unwrap()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_api_key(
    State(config): State<Arc<ServerConfig>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<ApiKeyQuery>
) -> Result<Response<String>, StatusCode> {
    // Check rate limiting if enabled
    if let Some(ref rate_limiter) = config.rate_limiter {
        if !check_rate_limit(rate_limiter, addr, config.requests_per_second).await {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }
    match generate_api_key_response(params.raw.unwrap_or(true)) {  // Default to true in server mode
        Ok(result) => Ok(Response::builder()
            .header("content-type", "text/plain")
            .body(result)
            .unwrap()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_password(
    State(config): State<Arc<ServerConfig>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<PasswordQuery>
) -> Result<Response<String>, StatusCode> {
    // Check rate limiting if enabled
    if let Some(ref rate_limiter) = config.rate_limiter {
        if !check_rate_limit(rate_limiter, addr, config.requests_per_second).await {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }
    match generate_password_response(params.length, params.raw.unwrap_or(true)) {  // Default to true in server mode
        Ok(result) => Ok(Response::builder()
            .header("content-type", "text/plain")
            .body(result)
            .unwrap()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

async fn handle_root() -> Html<&'static str> {
    Html(WEB_INTERFACE_HTML)
}

async fn start_server(
    port: u16,
    listen_all_ips: bool,
    max_param_length: usize,
    enable_rate_limiting: bool,
    requests_per_second: u64,
    enable_cors: bool,
    max_body_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let rate_limiter = if enable_rate_limiting {
        Some(Arc::new(RwLock::new(HashMap::new())))
    } else {
        None
    };

    let config = Arc::new(ServerConfig {
        max_param_length,
        enable_rate_limiting,
        requests_per_second,
        enable_cors,
        max_request_body_size: max_body_size,
        rate_limiter,
    });

    let api_routes = Router::new()
        .route("/api/generate", get(handle_generate))
        .route("/api/api-key", get(handle_api_key))
        .route("/api/password", get(handle_password))
        .with_state(config.clone());
    
    let mut app = Router::new()
        .route("/", get(handle_root))
        .merge(api_routes);

    // Add middleware layers
    app = app.layer(RequestBodyLimitLayer::new(max_body_size));

    if enable_cors {
        println!("CORS enabled - allowing cross-origin requests");
        app = app.layer(CorsLayer::permissive());
    }

    let host = if listen_all_ips { "0.0.0.0" } else { "127.0.0.1" };
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;
    
    println!("hashrand server listening on http://{}:{}", host, port);
    println!("Web Interface:");
    println!("  GET /                                     - Interactive web interface");
    println!("API Endpoints:");
    println!("  GET /api/generate?length=21&alphabet=base58");
    println!("  GET /api/api-key");
    println!("  GET /api/password?length=21");
    println!("Security features:");
    println!("  Parameter validation: prefix/suffix max {} chars", max_param_length);
    if enable_rate_limiting {
        println!("  Rate limiting: {} requests/second per IP", requests_per_second);
    } else {
        println!("  Rate limiting: disabled (use --enable-rate-limiting to enable)");
    }
    if enable_cors {
        println!("  CORS: enabled for cross-origin requests");
    } else {
        println!("  CORS: disabled (use --enable-cors to enable)");
    }
    println!("  Request body limit: {} bytes", max_body_size);
    println!("Note: All endpoints return raw text by default (no newline)");
    
    axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_parse_length_valid() {
        assert_eq!(parse_length("2").unwrap(), 2);
        assert_eq!(parse_length("64").unwrap(), 64);
        assert_eq!(parse_length("128").unwrap(), 128);
    }

    #[test]
    fn test_parse_length_too_small() {
        assert!(parse_length("0").is_err());
        assert!(parse_length("1").is_err());
    }

    #[test]
    fn test_parse_length_too_large() {
        assert!(parse_length("129").is_err());
        assert!(parse_length("200").is_err());
    }

    #[test]
    fn test_parse_length_invalid_input() {
        assert!(parse_length("abc").is_err());
        assert!(parse_length("").is_err());
        assert!(parse_length("-5").is_err());
    }

    #[test]
    fn test_check_name_exists_no_match() {
        let dir = tempdir().unwrap();
        let name = "test123";
        assert!(!check_name_exists(name, dir.path(), None));
    }

    #[test]
    fn test_check_name_exists_exact_match() {
        let dir = tempdir().unwrap();
        let name = "test123";
        let file_path = dir.path().join(name);
        fs::write(&file_path, "content").unwrap();
        assert!(check_name_exists(name, dir.path(), None));
    }

    #[test]
    fn test_check_name_exists_in_subdirectory() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let name = "test123";
        let file_path = subdir.join(name);
        fs::write(&file_path, "content").unwrap();
        assert!(check_name_exists(name, dir.path(), None));
    }

    #[test]
    fn test_check_name_exists_with_depth_limit() {
        let dir = tempdir().unwrap();
        
        // Create nested directories beyond the depth limit
        let mut current_path = dir.path().to_path_buf();
        for i in 0..5 {
            current_path = current_path.join(format!("level{}", i));
            fs::create_dir(&current_path).unwrap();
        }
        
        // Create a file at depth 5
        let name = "deep_file";
        fs::write(current_path.join(name), "content").unwrap();
        
        // Should not find file with depth limit of 2
        assert!(!check_name_exists(name, dir.path(), Some(2)));
        
        // Should find file with no depth limit
        assert!(check_name_exists(name, dir.path(), None));
    }

    #[test]
    fn test_generate_unique_name_no_prefix_suffix() {
        let dir = tempdir().unwrap();
        let alphabet: Vec<char> = vec!['a', 'b'];
        let length = 3;

        // Create files with all possible 3-character combinations except one
        let combinations = ["aaa", "aab", "aba", "abb", "baa", "bab", "bba"];
        for combo in &combinations {
            fs::write(dir.path().join(combo), "content").unwrap();
        }

        // The only remaining combination is "bbb"
        let unique_hash = generate_unique_name(&alphabet, length, None, None, dir.path());
        assert_eq!(unique_hash, "bbb");
    }

    #[test]
    fn test_generate_unique_name_with_prefix_suffix() {
        let dir = tempdir().unwrap();
        let alphabet: Vec<char> = vec!['a', 'b'];
        let length = 2;
        let prefix = Some("pre_");
        let suffix = Some("_suf");

        // Create files blocking most combinations
        let combinations = ["pre_aa_suf", "pre_ab_suf", "pre_ba_suf"];
        for combo in &combinations {
            fs::write(dir.path().join(combo), "content").unwrap();
        }

        // The only remaining combination is "bb"
        let unique_hash = generate_unique_name(&alphabet, length, prefix, suffix, dir.path());
        assert_eq!(unique_hash, "bb");
    }

    #[cfg(unix)]
    #[test]
    fn test_parse_mode_valid() {
        assert_eq!(parse_mode("644").unwrap(), 0o644);
        assert_eq!(parse_mode("755").unwrap(), 0o755);
        assert_eq!(parse_mode("0600").unwrap(), 0o600);
        assert_eq!(parse_mode("777").unwrap(), 0o777);
    }
    
    #[cfg(unix)]
    #[test]
    fn test_parse_mode_invalid() {
        assert!(parse_mode("888").is_err());
        assert!(parse_mode("abc").is_err());
        assert!(parse_mode("").is_err());
    }

    #[test]
    fn test_api_key_fixed_length() {
        // Test that api-key option uses fixed 44 characters
        let args_vec = vec!["hashrand", "--api-key"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.api_key);
        assert!(!args.password);
        assert_eq!(args.length, 21); // Original default before modification in main
    }

    #[test]
    fn test_password_default_length() {
        // Test that password option uses 21 characters by default
        let args_vec = vec!["hashrand", "--password"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.password);
        assert!(!args.api_key);
        assert_eq!(args.length, 21); // Default password length
    }

    #[test]
    fn test_api_key_cannot_have_custom_length() {
        // Test that api-key option cannot have custom length
        let args_vec = vec!["hashrand", "--api-key", "20"];
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }

    #[test]
    fn test_password_with_custom_length() {
        // Test that password option respects custom length within valid range
        let args_vec = vec!["hashrand", "--password", "24"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.password);
        assert_eq!(args.length, 24);
    }

    #[test] 
    fn test_password_length_too_short() {
        // Test that password length below 21 is invalid (will be checked in main)
        let args_vec = vec!["hashrand", "--password", "15"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.password);
        assert_eq!(args.length, 15); // Parse succeeds, validation happens in main
    }

    #[test]
    fn test_password_length_too_long() {
        // Test that password length above 44 is invalid (will be checked in main)
        let args_vec = vec!["hashrand", "--password", "50"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.password);
        assert_eq!(args.length, 50); // Parse succeeds, validation happens in main
    }

    #[test]
    fn test_password_length_at_boundaries() {
        // Test boundary values for password length
        let test_cases = vec![
            ("21", 21), // minimum
            ("44", 44), // maximum
        ];
        
        for (length_str, expected) in test_cases {
            let args_vec = vec!["hashrand", "--password", length_str];
            let args = Args::try_parse_from(&args_vec).unwrap();
            assert!(args.password);
            assert_eq!(args.length, expected);
        }
    }

    #[test]
    fn test_api_key_password_conflict() {
        // Test that api-key and password cannot be used together
        let args_vec = vec!["hashrand", "--api-key", "--password"];
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }

    #[test]
    fn test_api_key_conflicts_with_alphabets() {
        // Test that api-key conflicts with other alphabet options
        let test_cases = vec![
            vec!["hashrand", "--api-key", "--full"],
            vec!["hashrand", "--api-key", "--no-look-alike"],
            vec!["hashrand", "--api-key", "--full-with-symbols"],
        ];
        
        for args_vec in test_cases {
            let result = Args::try_parse_from(&args_vec);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_password_conflicts_with_alphabets() {
        // Test that password conflicts with other alphabet options
        let test_cases = vec![
            vec!["hashrand", "--password", "--full"],
            vec!["hashrand", "--password", "--no-look-alike"],
            vec!["hashrand", "--password", "--full-with-symbols"],
        ];
        
        for args_vec in test_cases {
            let result = Args::try_parse_from(&args_vec);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_api_key_conflicts_with_all_options() {
        // Test that api-key conflicts with ALL other options except --raw
        let test_cases = vec![
            vec!["hashrand", "--api-key", "--check"],
            vec!["hashrand", "--api-key", "--mkdir"],
            vec!["hashrand", "--api-key", "--touch"],
            vec!["hashrand", "--api-key", "--prefix", "test"],
            vec!["hashrand", "--api-key", "--suffix", "test"],
            vec!["hashrand", "--api-key", "--path", "/tmp"],
        ];
        
        for args_vec in test_cases {
            let result = Args::try_parse_from(&args_vec);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_password_conflicts_with_non_length_options() {
        // Test that password conflicts with all options except length and --raw
        let test_cases = vec![
            vec!["hashrand", "--password", "--check"],
            vec!["hashrand", "--password", "--mkdir"],
            vec!["hashrand", "--password", "--touch"],
            vec!["hashrand", "--password", "--prefix", "test"],
            vec!["hashrand", "--password", "--suffix", "test"],
            vec!["hashrand", "--password", "--path", "/tmp"],
        ];
        
        for args_vec in test_cases {
            let result = Args::try_parse_from(&args_vec);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_raw_with_api_key() {
        // Test that --raw works with --api-key
        let args_vec = vec!["hashrand", "--api-key", "--raw"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.api_key);
        assert!(args.raw);
    }

    #[test]
    fn test_raw_with_password() {
        // Test that --raw works with --password
        let args_vec = vec!["hashrand", "--password", "--raw"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.password);
        assert!(args.raw);
    }

    #[test]
    fn test_raw_with_password_and_length() {
        // Test that --raw works with --password and custom length
        let args_vec = vec!["hashrand", "--password", "20", "--raw"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.password);
        assert!(args.raw);
        assert_eq!(args.length, 20);
    }

    #[test]
    fn test_raw_short_flag() {
        // Test that -r short flag works
        let args_vec = vec!["hashrand", "-r"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.raw);
    }

    #[test]
    fn test_raw_with_various_options() {
        // Test that --raw works with various alphabet options
        let test_cases = vec![
            vec!["hashrand", "--raw", "--full"],
            vec!["hashrand", "--raw", "--no-look-alike"],
            vec!["hashrand", "--raw", "--full-with-symbols"],
            vec!["hashrand", "-r", "--check"],
        ];
        
        for args_vec in test_cases {
            let args = Args::try_parse_from(&args_vec).unwrap();
            assert!(args.raw);
        }
    }

    #[test]
    fn test_api_key_has_prefix() {
        // Test that API keys have the 'ak_' prefix
        // Note: This test only verifies argument parsing, not the actual prefix logic
        let args_vec = vec!["hashrand", "--api-key"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.api_key);
        // The actual prefix logic is tested through integration tests
    }

    #[test]
    fn test_serve_option_parsing() {
        // Test that --serve option parses correctly
        let args_vec = vec!["hashrand", "--serve", "8080"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert_eq!(args.serve, Some(8080));
        assert!(!args.listen_all_ips);
        
        // Test short form
        let args_vec = vec!["hashrand", "-s", "3000"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert_eq!(args.serve, Some(3000));
        assert!(!args.listen_all_ips);
        
        // Test with --listen-all-ips flag
        let args_vec = vec!["hashrand", "--serve", "8080", "--listen-all-ips"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert_eq!(args.serve, Some(8080));
        assert!(args.listen_all_ips);
        
        // Test that --listen-all-ips requires --serve
        let args_vec = vec!["hashrand", "--listen-all-ips"];
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }

    #[test]
    fn test_alphabet_type_selection() {
        assert!(matches!(
            get_alphabet(&AlphabetType::Base58),
            &[_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]
        ));
        assert_eq!(get_alphabet(&AlphabetType::Base58).len(), 58);
        assert_eq!(get_alphabet(&AlphabetType::NoLookAlike).len(), 57);
        assert_eq!(get_alphabet(&AlphabetType::Full).len(), 62);
        assert_eq!(get_alphabet(&AlphabetType::FullWithSymbols).len(), 73);
    }

    #[test]
    fn test_generate_hash_from_request() {
        let request = HashRequest {
            length: 10,
            alphabet: AlphabetType::Base58,
            raw: true,
            check: false,
            prefix: Some("test_".to_string()),
            suffix: Some("_end".to_string()),
        };
        
        let result = generate_hash_from_request(&request).unwrap();
        assert!(result.starts_with("test_"));
        assert!(result.ends_with("_end"));
        assert_eq!(result.len(), 10 + 5 + 4); // length + prefix + suffix
        assert!(!result.contains('\n')); // raw output
    }

    #[test]
    fn test_generate_hash_from_request_with_newline() {
        let request = HashRequest {
            length: 8,
            alphabet: AlphabetType::Full,
            raw: false,
            check: false,
            prefix: None,
            suffix: None,
        };
        
        let result = generate_hash_from_request(&request).unwrap();
        assert_eq!(result.len(), 9); // 8 chars + newline
        assert!(result.ends_with('\n'));
    }

    #[test]
    fn test_generate_api_key_response() {
        let result = generate_api_key_response(true).unwrap();
        assert!(result.starts_with("ak_"));
        assert_eq!(result.len(), 47); // ak_ + 44 chars
        assert!(!result.contains('\n')); // raw output
        
        let result = generate_api_key_response(false).unwrap();
        assert!(result.starts_with("ak_"));
        assert!(result.ends_with('\n'));
        assert_eq!(result.len(), 48); // ak_ + 44 chars + newline
    }

    #[test]
    fn test_generate_password_response() {
        // Default length
        let result = generate_password_response(None, true).unwrap();
        assert_eq!(result.len(), 21);
        assert!(!result.contains('\n'));
        
        // Custom length within range
        let result = generate_password_response(Some(30), false).unwrap();
        assert_eq!(result.len(), 31); // 30 chars + newline
        assert!(result.ends_with('\n'));
        
        // Length too short
        let result = generate_password_response(Some(15), true);
        assert!(result.is_err());
        
        // Length too long
        let result = generate_password_response(Some(50), true);
        assert!(result.is_err());
        
        // Boundary values
        let result = generate_password_response(Some(21), true).unwrap();
        assert_eq!(result.len(), 21);
        
        let result = generate_password_response(Some(44), true).unwrap();
        assert_eq!(result.len(), 44);
    }

    #[test]
    fn test_validate_query_params_valid() {
        // Test valid parameters within limits
        let prefix = Some("test".to_string());
        let suffix = Some("end".to_string());
        assert!(validate_query_params(&prefix, &suffix, 10).is_ok());
        
        // Test with None values
        assert!(validate_query_params(&None, &None, 10).is_ok());
        
        // Test with only prefix
        assert!(validate_query_params(&prefix, &None, 10).is_ok());
        
        // Test with only suffix
        assert!(validate_query_params(&None, &suffix, 10).is_ok());
    }

    #[test]
    fn test_validate_query_params_prefix_too_long() {
        let prefix = Some("this_prefix_is_too_long_for_limit".to_string());
        let suffix = Some("ok".to_string());
        let result = validate_query_params(&prefix, &suffix, 10);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Prefix length"));
    }

    #[test]
    fn test_validate_query_params_suffix_too_long() {
        let prefix = Some("ok".to_string());
        let suffix = Some("this_suffix_is_too_long_for_limit".to_string());
        let result = validate_query_params(&prefix, &suffix, 10);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Suffix length"));
    }

    #[test]
    fn test_validate_query_params_both_too_long() {
        let prefix = Some("very_long_prefix".to_string());
        let suffix = Some("very_long_suffix".to_string());
        let result = validate_query_params(&prefix, &suffix, 5);
        assert!(result.is_err());
        // Should fail on prefix first
        assert!(result.unwrap_err().contains("Prefix length"));
    }

    #[test]
    fn test_max_param_length_option_parsing() {
        // Test default value
        let args_vec = vec!["hashrand", "--serve", "8080"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert_eq!(args.max_param_length, 32);
        
        // Test custom value
        let args_vec = vec!["hashrand", "--serve", "8080", "--max-param-length", "64"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert_eq!(args.max_param_length, 64);
        
        // Test that --max-param-length requires --serve
        let args_vec = vec!["hashrand", "--max-param-length", "64"];
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }

    #[test]
    fn test_rate_limiting_options() {
        // Test enable-rate-limiting flag
        let args_vec = vec!["hashrand", "--serve", "8080", "--enable-rate-limiting"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.enable_rate_limiting);
        assert_eq!(args.rate_limit, 100); // Default
        
        // Test custom rate limit
        let args_vec = vec!["hashrand", "--serve", "8080", "--enable-rate-limiting", "--rate-limit", "50"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.enable_rate_limiting);
        assert_eq!(args.rate_limit, 50);
        
        // Test that --rate-limit requires --enable-rate-limiting
        let args_vec = vec!["hashrand", "--serve", "8080", "--rate-limit", "50"];
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }

    #[test]
    fn test_cors_options() {
        // Test enable-cors flag
        let args_vec = vec!["hashrand", "--serve", "8080", "--enable-cors"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert!(args.enable_cors);
        
        // Test that --enable-cors requires --serve
        let args_vec = vec!["hashrand", "--enable-cors"];
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }

    #[test]
    fn test_body_size_options() {
        // Test default max-body-size
        let args_vec = vec!["hashrand", "--serve", "8080"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert_eq!(args.max_body_size, 1024);
        
        // Test custom max-body-size
        let args_vec = vec!["hashrand", "--serve", "8080", "--max-body-size", "2048"];
        let args = Args::try_parse_from(&args_vec).unwrap();
        assert_eq!(args.max_body_size, 2048);
        
        // Test that --max-body-size requires --serve
        let args_vec = vec!["hashrand", "--max-body-size", "2048"];
        let result = Args::try_parse_from(&args_vec);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        use std::net::{IpAddr, Ipv4Addr};
        
        let rate_limiter = Arc::new(RwLock::new(HashMap::new()));
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        
        // First request should succeed
        assert!(check_rate_limit(&rate_limiter, addr, 2).await);
        
        // Second request should succeed
        assert!(check_rate_limit(&rate_limiter, addr, 2).await);
        
        // Third request should fail (exceeded limit of 2 per second)
        assert!(!check_rate_limit(&rate_limiter, addr, 2).await);
    }
}
