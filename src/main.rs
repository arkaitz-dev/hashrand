use clap::Parser;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

mod cli;
mod server;
#[cfg(test)]
mod tests;

use cli::{Args, AlphabetType};
use server::{get_alphabet, start_server};

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