use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
}

fn parse_length(s: &str) -> Result<usize, String> {
    let length: usize = s.parse().map_err(|_| "Length must be a number")?;

    if !(2..=128).contains(&length) {
        return Err("Length must be between 2 and 128".to_string());
    }

    Ok(length)
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

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();
    
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

    // Select alphabet based on arguments
    let alphabet: &[char] = if args.no_look_alike {
        &NO_LOOK_ALIKE_ALPHABET
    } else if args.full {
        &FULL_ALPHABET
    } else if args.full_with_symbols {
        &FULL_WITH_SYMBOLS_ALPHABET
    } else if args.api_key {
        &FULL_ALPHABET
    } else if args.password {
        &FULL_WITH_SYMBOLS_ALPHABET
    } else {
        &BASE58_ALPHABET
    };

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
}
