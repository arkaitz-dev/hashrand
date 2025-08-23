use crate::types::{AlphabetType, HashResponse};
use crate::utils::{generate_random_seed, generate_with_seed, seed_to_hex};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;

/// Helper function to check if a string contains unwanted patterns
fn contains_unwanted_patterns(s: &str) -> bool {
    s.contains("--") || s.contains("__")
}

/// Convert hex string to 32-byte array
fn hex_to_seed(hex_str: &str) -> Result<[u8; 32], String> {
    if hex_str.len() != 64 {
        return Err("Seed must be exactly 64 hex characters".to_string());
    }
    
    let mut seed = [0u8; 32];
    for i in 0..32 {
        let hex_byte = &hex_str[i*2..i*2+2];
        seed[i] = u8::from_str_radix(hex_byte, 16)
            .map_err(|_| "Invalid hex character in seed".to_string())?;
    }
    Ok(seed)
}

/// Generate hash avoiding unwanted patterns using seeded generator
fn generate_avoiding_unwanted_patterns(
    length: usize,
    alphabet: &[char],
    prefix: &str,
    suffix: &str,
    seed: [u8; 32],
) -> String {
    const MAX_ATTEMPTS: usize = 50; // Reasonable limit to avoid infinite loops
    
    for attempt in 1..=MAX_ATTEMPTS {
        // Create a slightly different seed for each attempt to ensure different results
        let mut attempt_seed = seed;
        attempt_seed[0] = attempt_seed[0].wrapping_add(attempt as u8);
        
        let hash = generate_with_seed(attempt_seed, length, alphabet);
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
    let hash = generate_with_seed(seed, length, alphabet);
    format!("{}{}{}", prefix, hash, suffix)
}

/// Handles the /api/custom endpoint for customizable hash generation
pub fn handle_custom_request(req: Request) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => handle_custom_get(req),
        Method::Post => handle_custom_post(req),
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method not allowed")
            .build()),
    }
}

/// Handle GET requests (generate seed automatically)
pub fn handle_custom_get(req: Request) -> anyhow::Result<Response> {
    // Extract query parameters from request URI
    let uri_str = req.uri().to_string();
    let query = if let Some(idx) = uri_str.find('?') {
        &uri_str[idx + 1..]
    } else {
        ""
    };
    
    let params: HashMap<String, String> = query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect();
    
    handle_custom_with_params(params, None)
}

/// Handle POST requests (use provided seed)
pub fn handle_custom_post(req: Request) -> anyhow::Result<Response> {
    // Parse JSON body
    let body = req.body();
    let json_str = std::str::from_utf8(body)?;
    let json_data: serde_json::Value = serde_json::from_str(json_str)?;
    
    // Extract parameters from JSON
    let mut params = HashMap::new();
    
    if let Some(length) = json_data.get("length") {
        if let Some(n) = length.as_u64() {
            params.insert("length".to_string(), n.to_string());
        }
    }
    
    if let Some(alphabet) = json_data.get("alphabet") {
        if let Some(s) = alphabet.as_str() {
            params.insert("alphabet".to_string(), s.to_string());
        }
    }
    
    if let Some(prefix) = json_data.get("prefix") {
        if let Some(s) = prefix.as_str() {
            params.insert("prefix".to_string(), s.to_string());
        }
    }
    
    if let Some(suffix) = json_data.get("suffix") {
        if let Some(s) = suffix.as_str() {
            params.insert("suffix".to_string(), s.to_string());
        }
    }
    
    // Extract seed
    let seed_opt = if let Some(seed_val) = json_data.get("seed") {
        if let Some(seed_str) = seed_val.as_str() {
            Some(hex_to_seed(seed_str).map_err(|e| anyhow::anyhow!(e))?)
        } else {
            None
        }
    } else {
        None
    };
    
    handle_custom_with_params(params, seed_opt)
}

/// Core logic for handling custom hash generation
pub fn handle_custom_with_params(params: HashMap<String, String>, provided_seed: Option<[u8; 32]>) -> anyhow::Result<Response> {
    // Parse parameters with default values
    let length = params
        .get("length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(21);

    let alphabet_type = params
        .get("alphabet")
        .map(|s| AlphabetType::from_str(s))
        .unwrap_or(AlphabetType::Base58);

    let _raw = params
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

    // Use provided seed or generate random one
    let seed_32 = provided_seed.unwrap_or_else(generate_random_seed);
    let seed_hex = seed_to_hex(&seed_32);

    // Generate hash using seeded generator
    let alphabet = alphabet_type.as_chars();
    let hash = if alphabet_type == AlphabetType::FullWithSymbols {
        // For full-with-symbols, avoid results containing '--' or '__' patterns
        generate_avoiding_unwanted_patterns(length, &alphabet, &prefix, &suffix, seed_32)
    } else {
        // For other alphabets, generate normally with seeded generator
        let base_hash = generate_with_seed(seed_32, length, &alphabet);
        format!("{}{}{}", prefix, base_hash, suffix)
    };

    // Create JSON response
    let response = HashResponse::new(hash, seed_hex);
    let json_body = serde_json::to_string(&response)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_body)
        .build())
}

/// Legacy function for backward compatibility (used by lib.rs routing)
pub fn handle_custom(params: HashMap<String, String>) -> anyhow::Result<Response> {
    handle_custom_with_params(params, None)
}
