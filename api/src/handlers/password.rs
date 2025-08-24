use crate::types::{AlphabetType, HashResponse};
use crate::utils::{generate_random_seed, generate_with_seed, seed_to_base58, base58_to_seed};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;

/// Helper function to check if a string contains unwanted patterns
fn contains_unwanted_patterns(s: &str) -> bool {
    s.contains("--") || s.contains("__")
}

/// Handle password requests (both GET and POST)
pub fn handle_password_request(req: Request) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => handle_password_get(req),
        Method::Post => handle_password_post(req),
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method not allowed")
            .build()),
    }
}

/// Handle GET request for password generation
fn handle_password_get(req: Request) -> anyhow::Result<Response> {
    let uri_string = req.uri().to_string();
    let query_string = uri_string.split('?').nth(1).unwrap_or("");
    let params = crate::utils::query::parse_query_params(query_string);
    handle_password(params)
}

/// Handle POST request for password generation with seed
fn handle_password_post(req: Request) -> anyhow::Result<Response> {
    let body = req.body();
    let json_str = String::from_utf8(body.to_vec())
        .map_err(|_| anyhow::anyhow!("Invalid UTF-8 in request body"))?;

    let json_value: serde_json::Value = serde_json::from_str(&json_str)
        .map_err(|_| anyhow::anyhow!("Invalid JSON in request body"))?;

    let seed_str = json_value
        .get("seed")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing 'seed' field in JSON body"))?;

    let seed_32 = base58_to_seed(seed_str).map_err(|e| anyhow::anyhow!("Invalid seed: {}", e))?;

    // Extract other parameters from JSON
    let mut params = HashMap::new();
    if let Some(length) = json_value.get("length").and_then(|v| v.as_u64()) {
        params.insert("length".to_string(), length.to_string());
    }
    if let Some(alphabet) = json_value.get("alphabet").and_then(|v| v.as_str()) {
        params.insert("alphabet".to_string(), alphabet.to_string());
    }

    handle_password_with_seed(params, seed_32)
}

/// Handles the /api/password endpoint for secure password generation
fn handle_password(params: HashMap<String, String>) -> anyhow::Result<Response> {
    // Generate random 32-byte seed
    let seed_32 = generate_random_seed();
    handle_password_with_seed(params, seed_32)
}

/// Handle password generation with provided seed
fn handle_password_with_seed(
    params: HashMap<String, String>,
    seed_32: [u8; 32],
) -> anyhow::Result<Response> {
    let alphabet_type = params
        .get("alphabet")
        .map(|s| AlphabetType::from_str(s))
        .unwrap_or(AlphabetType::FullWithSymbols);

    // Dynamic minimum length based on alphabet
    let min_length = match alphabet_type {
        AlphabetType::FullWithSymbols => 21,
        AlphabetType::NoLookAlike => 24,
        _ => 21,
    };

    let length = params
        .get("length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(min_length);

    let _raw = params
        .get("raw")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true);

    // Validate minimum and maximum length (44)
    if length < min_length || length > 44 {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body(format!(
                "Password length must be between {} and 44",
                min_length
            ))
            .build());
    }

    let seed_base58 = seed_to_base58(&seed_32);

    // Generate password using seeded generator with unwanted pattern avoidance
    let alphabet = alphabet_type.as_chars();
    let password = generate_password_avoiding_unwanted_patterns(length, &alphabet, seed_32);

    // Create JSON response
    let response = HashResponse::new(password, seed_base58);
    let json_body = serde_json::to_string(&response)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_body)
        .build())
}

/// Generate password avoiding unwanted patterns using seeded generator
fn generate_password_avoiding_unwanted_patterns(
    length: usize,
    alphabet: &[char],
    seed: [u8; 32],
) -> String {
    const MAX_ATTEMPTS: usize = 50; // Reasonable limit to avoid infinite loops

    for attempt in 1..=MAX_ATTEMPTS {
        // Create a slightly different seed for each attempt to ensure different results
        let mut attempt_seed = seed;
        attempt_seed[0] = attempt_seed[0].wrapping_add(attempt as u8);

        let password = generate_with_seed(attempt_seed, length, alphabet);

        if !contains_unwanted_patterns(&password) {
            return password;
        }

        // Log warning if we're having trouble finding a good result
        if attempt == MAX_ATTEMPTS {
            eprintln!(
                "[WARN] Reached max attempts ({}) avoiding unwanted patterns in password. Using last result.",
                MAX_ATTEMPTS
            );
        }
    }

    // Fallback: return a result even if it contains unwanted patterns
    generate_with_seed(seed, length, alphabet)
}
