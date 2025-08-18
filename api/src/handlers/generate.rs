use spin_sdk::http::Response;
use std::collections::HashMap;
use crate::types::AlphabetType;

/// Handles the /api/generate endpoint for customizable hash generation
pub fn handle_generate(params: HashMap<String, String>) -> anyhow::Result<Response> {
    // Parse parameters with default values
    let length = params.get("length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(21);
    
    let alphabet_type = params.get("alphabet")
        .map(|s| AlphabetType::from_str(s))
        .unwrap_or(AlphabetType::Base58);
    
    let raw = params.get("raw")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true);
    
    let prefix = params.get("prefix").cloned().unwrap_or_default();
    let suffix = params.get("suffix").cloned().unwrap_or_default();
    
    // Validate length (2-128)
    if length < 2 || length > 128 {
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
    
    // Generate hash using nanoid
    let alphabet = alphabet_type.as_chars();
    let hash = nanoid::nanoid!(length, &alphabet);
    
    // Apply prefix and suffix
    let result = format!("{}{}{}", prefix, hash, suffix);
    
    // Format response according to raw parameter
    let body = if raw {
        result
    } else {
        format!("{}\n", result)
    };
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(body)
        .build())
}