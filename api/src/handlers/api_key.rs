use crate::types::{AlphabetType, CustomHashResponse};
use crate::utils::{
    base58_to_seed, generate_otp, generate_random_seed, generate_with_seed, seed_to_base58,
};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Handle API key requests (both GET and POST)
pub fn handle_api_key_request(req: Request) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => handle_api_key_get(req),
        Method::Post => handle_api_key_post(req),
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method not allowed")
            .build()),
    }
}

/// Handle GET request for API key generation
fn handle_api_key_get(req: Request) -> anyhow::Result<Response> {
    let uri_string = req.uri().to_string();
    let query_string = uri_string.split('?').nth(1).unwrap_or("");
    let params = crate::utils::query::parse_query_params(query_string);
    handle_api_key(params)
}

/// Handle POST request for API key generation with seed
fn handle_api_key_post(req: Request) -> anyhow::Result<Response> {
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

    handle_api_key_with_seed(params, seed_32)
}

/// Handles the /api/api-key endpoint for API key generation with ak_ prefix
fn handle_api_key(params: HashMap<String, String>) -> anyhow::Result<Response> {
    // Generate random 32-byte seed
    let seed_32 = generate_random_seed();
    handle_api_key_with_seed(params, seed_32)
}

/// Handle API key generation with provided seed
fn handle_api_key_with_seed(
    params: HashMap<String, String>,
    seed_32: [u8; 32],
) -> anyhow::Result<Response> {
    let alphabet_type = params
        .get("alphabet")
        .map(|s| AlphabetType::from_str(s))
        .unwrap_or(AlphabetType::Full);

    // Dynamic minimum length based on alphabet
    let min_length = match alphabet_type {
        AlphabetType::Full => 44,
        AlphabetType::NoLookAlike => 47,
        _ => 44,
    };

    let length = params
        .get("length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(min_length);

    let _raw = params
        .get("raw")
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true);

    // Validate minimum and maximum length (64)
    if length < min_length || length > 64 {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body(format!(
                "API key length must be between {} and 64",
                min_length
            ))
            .build());
    }

    let seed_base58 = seed_to_base58(&seed_32);

    // Generate API key with ak_ prefix using seeded generator
    let alphabet = alphabet_type.as_chars();
    let key_part = generate_with_seed(seed_32, length, &alphabet);
    let api_key = format!("ak_{}", key_part);

    // Generate OTP from seed
    let otp = generate_otp(seed_32);

    // Get current timestamp
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    // Create JSON response
    let response = CustomHashResponse::new(api_key, seed_base58, otp, timestamp);
    let json_body = serde_json::to_string(&response)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_body)
        .build())
}
