use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "hashrand")]
#[command(about = "Generate random hash using base58 alphabet", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(group = clap::ArgGroup::new("action").args(&["mkdir", "touch"]))]
pub struct Args {
    /// Length of the hash (between 2 and 128, default: 21, or 44-64 for API keys)
    #[arg(value_parser = parse_length, default_value = "21")]
    pub length: usize,

    /// Output without newline character
    #[arg(short = 'r', long = "raw")]
    pub raw: bool,

    /// Use no look-alike alphabet (excludes: 0, O, I, l, 1)
    #[arg(long = "no-look-alike", conflicts_with_all = &["full", "full_with_symbols"])]
    pub no_look_alike: bool,

    /// Use full alphanumeric alphabet (uppercase, lowercase, and numbers)
    #[arg(long = "full", conflicts_with_all = &["no_look_alike", "full_with_symbols"])]
    pub full: bool,

    /// Use full alphabet with symbols (-_*^@#+!?$%)
    #[arg(long = "full-with-symbols", conflicts_with_all = &["no_look_alike", "full"])]
    pub full_with_symbols: bool,

    /// Check that generated hash doesn't match any existing file or directory name
    #[arg(short = 'c', long = "check")]
    pub check: bool,

    /// Create a directory with the generated hash as name
    #[arg(long = "mkdir", group = "action")]
    pub mkdir: bool,

    /// Create a file with the generated hash as name
    #[arg(long = "touch", group = "action")]
    pub touch: bool,

    /// Prefix to add before the generated hash
    #[arg(long = "prefix")]
    pub prefix: Option<String>,

    /// Suffix to add after the generated hash
    #[arg(long = "suffix")]
    pub suffix: Option<String>,

    /// Path where to create the file or directory
    #[arg(long = "path")]
    pub path: Option<PathBuf>,

    /// Generate API key using full alphabet (ak_ + 44-64 characters, default 44)
    #[arg(long = "api-key", conflicts_with_all = &["no_look_alike", "full", "full_with_symbols", "password", "check", "mkdir", "touch", "prefix", "suffix", "path"])]
    pub api_key: bool,

    /// Generate password using full alphabet with symbols (21 characters by default)
    #[arg(long = "password", conflicts_with_all = &["no_look_alike", "full", "full_with_symbols", "api_key", "check", "mkdir", "touch", "prefix", "suffix", "path"])]
    pub password: bool,

    /// File permissions to use when creating a file (Unix-style octal, e.g., 644)
    #[cfg(unix)]
    #[arg(long = "file-mode", value_parser = parse_mode)]
    pub file_mode: Option<u32>,

    /// Directory permissions to use when creating a directory (Unix-style octal, e.g., 755)
    #[cfg(unix)]
    #[arg(long = "dir-mode", value_parser = parse_mode)]
    pub dir_mode: Option<u32>,

    /// Enable audit logging (outputs operations to stderr)
    #[arg(long = "audit-log")]
    pub audit_log: bool,

    /// Start HTTP server on specified port
    #[arg(short = 's', long = "serve", value_name = "PORT")]
    pub serve: Option<u16>,

    /// Listen on all network interfaces (0.0.0.0) instead of localhost only (requires --serve)
    #[arg(long = "listen-all-ips", requires = "serve")]
    pub listen_all_ips: bool,

    /// Maximum length for prefix and suffix parameters in server mode (default: 32)
    #[arg(long = "max-param-length", requires = "serve", default_value = "32")]
    pub max_param_length: usize,

    /// Enable rate limiting for server mode (default: disabled for better performance)
    #[arg(long = "enable-rate-limiting", requires = "serve")]
    pub enable_rate_limiting: bool,

    /// Requests per second limit when rate limiting is enabled (default: 100)
    #[arg(long = "rate-limit", requires = "enable_rate_limiting", default_value = "100")]
    pub rate_limit: u64,

    /// Enable CORS headers for cross-origin requests (default: disabled)
    #[arg(long = "enable-cors", requires = "serve")]
    pub enable_cors: bool,

    /// Maximum request body size in bytes (default: 1024)
    #[arg(long = "max-body-size", requires = "serve", default_value = "1024")]
    pub max_body_size: usize,
}

impl Args {
    /// Validate arguments after parsing
    pub fn validate(&self) -> Result<(), String> {
        // Validate API key length if api-key flag is used
        if self.api_key && !(44..=64).contains(&self.length) {
            return Err("API key length must be between 44 and 64 characters".to_string());
        }
        
        Ok(())
    }
}

pub use crate::utils::validation::{parse_length, parse_mode};

#[derive(Debug, Clone)]
pub enum AlphabetType {
    Base58,
    NoLookAlike,
    Full,
    FullWithSymbols,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HashRequest {
    pub length: usize,
    pub alphabet: AlphabetType,
    pub raw: bool,
    pub check: bool,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}