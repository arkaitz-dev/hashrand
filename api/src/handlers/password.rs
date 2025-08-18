use spin_sdk::http::Response;
use std::collections::HashMap;
use crate::types::AlphabetType;

/// Handles the /api/password endpoint for secure password generation
pub fn handle_password(params: HashMap<String, String>) -> anyhow::Result<Response> {
    let alphabet_type = params.get("alphabet")
        .map(|s| AlphabetType::from_str(s))
        .unwrap_or(AlphabetType::FullWithSymbols);
    
    // Dynamic minimum length based on alphabet
    let min_length = match alphabet_type {
        AlphabetType::FullWithSymbols => 21,
        AlphabetType::NoLookAlike => 24,
        _ => 21,
    };
    
    let length = params.get("length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(min_length);
    
    let raw = params.get("raw")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true);
    
    // Validate minimum and maximum length (44)
    if length < min_length || length > 44 {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body(format!("Password length must be between {} and 44", min_length))
            .build());
    }
    
    // Generate password using nanoid
    let alphabet = alphabet_type.as_chars();
    let password = nanoid::nanoid!(length, &alphabet);
    
    // Format response according to raw parameter
    let body = if raw {
        password
    } else {
        format!("{}\n", password)
    };
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(body)
        .build())
}