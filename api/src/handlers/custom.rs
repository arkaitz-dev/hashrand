use crate::types::AlphabetType;
use spin_sdk::http::Response;
use std::collections::HashMap;

/// Helper function to check if a string contains unwanted patterns
fn contains_unwanted_patterns(s: &str) -> bool {
    s.contains("--") || s.contains("__")
}

/// Generate hash avoiding unwanted patterns for full-with-symbols alphabet
fn generate_avoiding_unwanted_patterns(
    length: usize,
    alphabet: &[char],
    prefix: &str,
    suffix: &str,
) -> String {
    const MAX_ATTEMPTS: usize = 50; // Reasonable limit to avoid infinite loops
    
    for attempt in 1..=MAX_ATTEMPTS {
        let hash = nanoid::nanoid!(length, alphabet);
        let result = format!("{}{}{}", prefix, hash, suffix);
        
        if !contains_unwanted_patterns(&result) {
            return result;
        }
        
        // Log warning if we're having trouble finding a good result
        if attempt == MAX_ATTEMPTS {
            eprintln!("[WARN] Reached max attempts ({}) avoiding unwanted patterns. Using last result.", MAX_ATTEMPTS);
        }
    }
    
    // Fallback: return a result even if it contains unwanted patterns
    let hash = nanoid::nanoid!(length, alphabet);
    format!("{}{}{}", prefix, hash, suffix)
}

/// Handles the /api/custom endpoint for customizable hash generation
pub fn handle_custom(params: HashMap<String, String>) -> anyhow::Result<Response> {
    // Parse parameters with default values
    let length = params
        .get("length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(21);

    let alphabet_type = params
        .get("alphabet")
        .map(|s| AlphabetType::from_str(s))
        .unwrap_or(AlphabetType::Base58);

    let raw = params
        .get("raw")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true);

    let prefix = params.get("prefix").cloned().unwrap_or_default();
    let suffix = params.get("suffix").cloned().unwrap_or_default();

    // Validate length (2-128)
    if !(2..=128).contains(&length) {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body("Length must be between 2 and 128")
            .build());
    }

    // Validate prefix and suffix length (maximum 32 each)
    if prefix.len() > 32 || suffix.len() > 32 {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body("Prefix and suffix must be 32 characters or less")
            .build());
    }

    // Generate hash using nanoid with unwanted pattern avoidance for full-with-symbols
    let alphabet = alphabet_type.as_chars();
    let result = if alphabet_type == AlphabetType::FullWithSymbols {
        // For full-with-symbols, avoid results containing '--' or '__' patterns
        generate_avoiding_unwanted_patterns(length, &alphabet, &prefix, &suffix)
    } else {
        // For other alphabets, generate normally
        let hash = nanoid::nanoid!(length, &alphabet);
        format!("{}{}{}", prefix, hash, suffix)
    };

    // Format response according to raw parameter
    let body = if raw { result } else { format!("{}\n", result) };

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(body)
        .build())
}
