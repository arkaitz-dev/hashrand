use crate::cli::{AlphabetType, HashRequest};
use crate::generators::{get_alphabet, generate_hash_from_request, generate_api_key_response, generate_password_response};

#[test]
fn test_alphabet_type_selection() {
    assert!(matches!(
        get_alphabet(&AlphabetType::Base58),
        &[_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]
    ));
    assert_eq!(get_alphabet(&AlphabetType::Base58).len(), 58);
    assert_eq!(get_alphabet(&AlphabetType::NoLookAlike).len(), 49);
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
    // Default with Full alphabet
    let result = generate_api_key_response(44, None, true).unwrap();
    assert!(result.starts_with("ak_"));
    assert_eq!(result.len(), 47); // ak_ + 44 chars
    assert!(!result.contains('\n')); // raw output
    
    let result = generate_api_key_response(44, None, false).unwrap();
    assert!(result.starts_with("ak_"));
    assert!(result.ends_with('\n'));

    // Test custom length with Full alphabet
    let result = generate_api_key_response(60, Some(AlphabetType::Full), true).unwrap();
    assert!(result.starts_with("ak_"));
    assert_eq!(result.len(), 63); // ak_ + 60 characters
    
    // Test NoLookAlike alphabet
    let result = generate_api_key_response(47, Some(AlphabetType::NoLookAlike), true).unwrap();
    assert!(result.starts_with("ak_"));
    assert_eq!(result.len(), 50); // ak_ + 47 chars
    
    // Test NoLookAlike with custom length
    let result = generate_api_key_response(50, Some(AlphabetType::NoLookAlike), true).unwrap();
    assert!(result.starts_with("ak_"));
    assert_eq!(result.len(), 53); // ak_ + 50 characters

    // Test validation for Full alphabet
    assert!(generate_api_key_response(43, Some(AlphabetType::Full), true).is_err()); // Too short
    assert!(generate_api_key_response(65, Some(AlphabetType::Full), true).is_err()); // Too long
    
    // Test validation for NoLookAlike alphabet
    assert!(generate_api_key_response(46, Some(AlphabetType::NoLookAlike), true).is_err()); // Too short for NoLookAlike
    assert!(generate_api_key_response(65, Some(AlphabetType::NoLookAlike), true).is_err()); // Too long
}

#[test]
fn test_generate_password_response() {
    // Default length with default alphabet (FullWithSymbols)
    let result = generate_password_response(None, None, true).unwrap();
    assert_eq!(result.len(), 21);
    assert!(!result.contains('\n'));
    
    // Custom length within range
    let result = generate_password_response(Some(30), None, false).unwrap();
    assert_eq!(result.len(), 31); // 30 chars + newline
    assert!(result.ends_with('\n'));
    
    // Length too short for FullWithSymbols
    let result = generate_password_response(Some(15), None, true);
    assert!(result.is_err());
    
    // Length too long
    let result = generate_password_response(Some(50), None, true);
    assert!(result.is_err());
    
    // Boundary values for FullWithSymbols
    let result = generate_password_response(Some(21), None, true).unwrap();
    assert_eq!(result.len(), 21);
    
    let result = generate_password_response(Some(44), None, true).unwrap();
    assert_eq!(result.len(), 44);
    
    // Test NoLookAlike alphabet
    let result = generate_password_response(None, Some(AlphabetType::NoLookAlike), true).unwrap();
    assert_eq!(result.len(), 24); // Default for NoLookAlike
    
    // Test NoLookAlike with too short length
    let result = generate_password_response(Some(20), Some(AlphabetType::NoLookAlike), true);
    assert!(result.is_err());
    
    // Test NoLookAlike with valid length
    let result = generate_password_response(Some(30), Some(AlphabetType::NoLookAlike), true).unwrap();
    assert_eq!(result.len(), 30);
}