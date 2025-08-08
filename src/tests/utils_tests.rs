use crate::utils::{check_name_exists, generate_unique_name};
use std::fs;
use tempfile::tempdir;

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