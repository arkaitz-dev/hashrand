use std::path::Path;
use walkdir::WalkDir;

pub fn check_name_exists(name: &str, current_dir: &Path, max_depth: Option<usize>) -> bool {
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

pub fn generate_unique_name(
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