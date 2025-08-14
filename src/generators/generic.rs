//! Generic hash generation functionality
//!
//! Provides the core hash generation logic with customizable parameters
//! including alphabet selection, length, prefix/suffix support.

use crate::cli::HashRequest;
use super::alphabets::get_alphabet;

/// Generates a cryptographically secure random hash based on request parameters
///
/// # Arguments
/// * `request` - Configuration containing alphabet type, length, prefix/suffix, and output format
///
/// # Returns
/// * `Ok(String)` - The generated hash with optional formatting
/// * `Err(Box<dyn std::error::Error>)` - If generation fails
///
/// # Examples
/// ```
/// use hashrand::cli::{HashRequest, AlphabetType};
/// use hashrand::generators::generate_hash_from_request;
///
/// let request = HashRequest {
///     alphabet: AlphabetType::Base58,
///     length: 12,
///     prefix: Some("test_".to_string()),
///     suffix: None,
///     raw: false,
/// };
/// let result = generate_hash_from_request(&request).unwrap();
/// assert!(result.starts_with("test_"));
/// ```
pub fn generate_hash_from_request(request: &HashRequest) -> Result<String, Box<dyn std::error::Error>> {
    let alphabet = get_alphabet(&request.alphabet);
    
    // Generate the random hash
    let hash = nanoid::format(nanoid::rngs::default, alphabet, request.length);
    
    // Build the full name with optional prefix and suffix
    let full_name = format!(
        "{}{}{}",
        request.prefix.as_deref().unwrap_or(""),
        hash,
        request.suffix.as_deref().unwrap_or("")
    );
    
    // Format the output
    Ok(if request.raw {
        full_name
    } else {
        format!("{}\n", full_name)
    })
}