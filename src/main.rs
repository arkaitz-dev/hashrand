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
    #[arg(value_parser = parse_length, default_value = "21")]
    length: usize,

    /// Output without newline character
    #[arg(short = 'r', long = "raw", conflicts_with = "action")]
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
}

fn parse_length(s: &str) -> Result<usize, String> {
    let length: usize = s.parse().map_err(|_| "Length must be a number")?;

    if !(2..=128).contains(&length) {
        return Err("Length must be between 2 and 128".to_string());
    }

    Ok(length)
}

fn check_name_exists(name: &str, current_dir: &Path) -> bool {
    for entry in WalkDir::new(current_dir).into_iter().filter_map(|e| e.ok()) {
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
    loop {
        let hash = nanoid::format(nanoid::rngs::default, alphabet, length);
        let full_name = format!(
            "{}{}{}",
            prefix.unwrap_or(""),
            hash,
            suffix.unwrap_or("")
        );
        if !check_name_exists(&full_name, current_dir) {
            return hash;
        }
    }
}

fn main() {
    let args = Args::parse();

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
    } else {
        &BASE58_ALPHABET
    };

    // Determine the base path for operations
    let base_path = if let Some(ref p) = args.path {
        p.clone()
    } else {
        std::env::current_dir().expect("Failed to get current directory")
    };

    // Check is implicit when using mkdir or touch
    let should_check = args.check || args.mkdir || args.touch;

    let hash = if should_check {
        generate_unique_name(
            alphabet,
            args.length,
            args.prefix.as_deref(),
            args.suffix.as_deref(),
            &base_path,
        )
    } else {
        nanoid::format(nanoid::rngs::default, alphabet, args.length)
    };

    // Build the full name with optional prefix and suffix
    let full_name = format!(
        "{}{}{}",
        args.prefix.as_deref().unwrap_or(""),
        hash,
        args.suffix.as_deref().unwrap_or("")
    );

    if args.mkdir {
        // Create directory
        let dir_path = base_path.join(&full_name);
        fs::create_dir(&dir_path).expect("Failed to create directory");
        println!("{}", dir_path.display());
    } else if args.touch {
        // Create file
        let file_path = base_path.join(&full_name);
        fs::File::create(&file_path).expect("Failed to create file");
        println!("{}", file_path.display());
    } else {
        // Just output the hash
        if args.raw {
            print!("{full_name}");
        } else {
            println!("{full_name}");
        }
    }
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
        assert!(!check_name_exists(name, dir.path()));
    }

    #[test]
    fn test_check_name_exists_exact_match() {
        let dir = tempdir().unwrap();
        let name = "test123";
        let file_path = dir.path().join(name);
        fs::write(&file_path, "content").unwrap();
        assert!(check_name_exists(name, dir.path()));
    }

    #[test]
    fn test_check_name_exists_in_subdirectory() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        let name = "test123";
        let file_path = subdir.join(name);
        fs::write(&file_path, "content").unwrap();
        assert!(check_name_exists(name, dir.path()));
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
}
