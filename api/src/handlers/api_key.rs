use spin_sdk::http::Response;
use std::collections::HashMap;
use crate::types::AlphabetType;

/// Handles the /api/api-key endpoint for API key generation with ak_ prefix
pub fn handle_api_key(params: HashMap<String, String>) -> anyhow::Result<Response> {
    let alphabet_type = params.get("alphabet")
        .map(|s| AlphabetType::from_str(s))
        .unwrap_or(AlphabetType::Full);
    
    // Dynamic minimum length based on alphabet
    let min_length = match alphabet_type {
        AlphabetType::Full => 44,
        AlphabetType::NoLookAlike => 47,
        _ => 44,
    };
    
    let length = params.get("length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(min_length);
    
    let raw = params.get("raw")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true);
    
    // Validate minimum and maximum length (64)
    if length < min_length || length > 64 {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body(format!("API key length must be between {} and 64", min_length))
            .build());
    }
    
    // Generate API key with ak_ prefix
    let alphabet = alphabet_type.as_chars();
    let key_part = nanoid::nanoid!(length, &alphabet);
    let api_key = format!("ak_{}", key_part);
    
    // Format response according to raw parameter
    let body = if raw {
        api_key
    } else {
        format!("{}\n", api_key)
    };
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(body)
        .build())
}