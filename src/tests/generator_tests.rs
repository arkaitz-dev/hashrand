use crate::cli::{AlphabetType, HashRequest};
use crate::generators::{get_alphabet, generate_hash_from_request, generate_api_key_response, generate_password_response};

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