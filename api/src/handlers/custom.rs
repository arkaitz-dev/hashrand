use crate::types::{AlphabetType, CustomHashResponse};
use crate::utils::auth::ErrorResponse;
use crate::utils::protected_endpoint_middleware::{extract_seed_from_payload, payload_to_params};
use crate::utils::{
    ProtectedEndpointMiddleware, ProtectedEndpointResult, SignedRequestValidator, generate_otp,
    generate_random_seed, generate_with_seed, seed_to_base58, validate_length,
    validate_prefix_suffix,
};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// External crates
extern crate hex;

/// Helper function to check if a string contains unwanted patterns
fn contains_unwanted_patterns(s: &str) -> bool {
    s.contains("--") || s.contains("__")
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
            eprintln!(
                "[WARN] Reached max attempts ({}) avoiding unwanted patterns. Using last result.",
                MAX_ATTEMPTS
            );
        }
    }

    // Fallback: return a result even if it contains unwanted patterns
    let hash = generate_with_seed(seed, length, alphabet);
    format!("{}{}{}", prefix, hash, suffix)
}

/// Handles the /api/custom endpoint for customizable hash generation
pub async fn handle_custom_request(req: Request) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => handle_custom_get(req),
        Method::Post => handle_custom_post_signed(req).await,
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method not allowed")
            .build()),
    }
}

/// Handle GET requests with Ed25519 signature validation
pub fn handle_custom_get(req: Request) -> anyhow::Result<Response> {
    // Extract and validate Bearer token to get public key
    let auth_header = req
        .header("authorization")
        .and_then(|h| h.as_str())
        .unwrap_or("");

    if !auth_header.starts_with("Bearer ") {
        return Ok(Response::builder()
            .status(401)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Missing or invalid Authorization header".to_string(),
            })?)
            .build());
    }

    let access_token = &auth_header[7..]; // Remove "Bearer " prefix
    let claims = match crate::utils::jwt::tokens::validate_access_token(access_token) {
        Ok(claims) => claims,
        Err(e) => {
            return Ok(Response::builder()
                .status(401)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: format!("Invalid access token: {}", e),
                })?)
                .build());
        }
    };

    // Convert public key to hex string for signature validation
    let public_key_hex = hex::encode(claims.pub_key);

    // Extract query parameters from request URI
    let uri_str = req.uri().to_string();
    let query = if let Some(idx) = uri_str.find('?') {
        &uri_str[idx + 1..]
    } else {
        ""
    };

    let mut params: HashMap<String, String> = query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect();

    // Validate Ed25519 signature
    if let Err(e) = SignedRequestValidator::validate_query_params(&mut params, &public_key_hex) {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: format!("Signature validation failed: {}", e),
            })?)
            .build());
    }

    println!(
        "✅ Custom GET: Ed25519 signature validated for user {}",
        claims.sub
    );

    handle_custom_with_params(params, None)
}

/// Handle POST requests with signed request validation (UNIVERSAL)
pub async fn handle_custom_post_signed(req: Request) -> anyhow::Result<Response> {
    let body_bytes = req.body();

    // Validate signed request and extract payload (UNIVERSAL)
    let result: ProtectedEndpointResult<serde_json::Value> =
        match ProtectedEndpointMiddleware::validate_request(&req, body_bytes).await {
            Ok(result) => result,
            Err(error_response) => return Ok(error_response),
        };

    println!(
        "✅ Custom endpoint: validated signed request for user {}",
        result.user_id
    );

    // Convert payload to parameter map using UNIVERSAL function
    let params = payload_to_params(&result.payload);

    // Extract seed using UNIVERSAL function
    let provided_seed = match extract_seed_from_payload(&result.payload) {
        Ok(seed) => seed,
        Err(e) => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse { error: e })?)
                .build());
        }
    };

    // Use existing business logic
    handle_custom_with_params(params, provided_seed)
}

/// Core logic for handling custom hash generation
pub fn handle_custom_with_params(
    params: HashMap<String, String>,
    provided_seed: Option<[u8; 32]>,
) -> anyhow::Result<Response> {
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
    if let Err(e) = validate_length(length, 2, 128) {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body(e.to_string())
            .build());
    }

    // Validate prefix and suffix (4 bytes max, safe characters)
    if let Err(e) = validate_prefix_suffix(&prefix, "Prefix") {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body(e.to_string())
            .build());
    }
    if let Err(e) = validate_prefix_suffix(&suffix, "Suffix") {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body(e.to_string())
            .build());
    }

    // Use provided seed or generate random one
    let seed_32 = provided_seed.unwrap_or_else(generate_random_seed);
    let seed_base58 = seed_to_base58(&seed_32);

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

    // Generate OTP using the same seed
    let otp = generate_otp(seed_32);

    // Get current timestamp
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    // Create JSON response
    let response = CustomHashResponse::new(hash, seed_base58, otp, timestamp);
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
