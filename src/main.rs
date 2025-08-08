use clap::Parser;
use std::fs;

mod cli;
mod server;
mod generators;
mod utils;
#[cfg(test)]
mod tests;

use cli::{Args, AlphabetType};
use server::start_server;
use generators::get_alphabet;
use utils::{generate_unique_name, create_audit_logger};


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
    let audit_log = create_audit_logger(args.audit_log);
    
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