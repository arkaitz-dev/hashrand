use crate::types::{AlphabetType, CustomHashResponse};
use crate::utils::auth::ErrorResponse;
use crate::utils::protected_endpoint_middleware::{extract_seed_from_payload, payload_to_params};
use crate::utils::{
    ProtectedEndpointMiddleware, ProtectedEndpointResult, SignedRequestValidator, generate_otp,
    generate_random_seed, generate_with_seed, seed_to_base58, validate_length,
};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// External crates
extern crate hex;

/// Handle API key requests (both GET and POST)
pub async fn handle_api_key_request(req: Request) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => handle_api_key_get(req),
        Method::Post => handle_api_key_post_signed(req).await,
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method not allowed")
            .build()),
    }
}

/// Handle GET request for API key generation with Ed25519 signature validation
fn handle_api_key_get(req: Request) -> anyhow::Result<Response> {
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
    let uri_string = req.uri().to_string();
    let query_string = uri_string.split('?').nth(1).unwrap_or("");

    let mut params: HashMap<String, String> = query_string
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
        "✅ API Key GET: Ed25519 signature validated for user {}",
        claims.sub
    );

    handle_api_key(params)
}

/// Handle POST requests with signed request validation (UNIVERSAL)
pub async fn handle_api_key_post_signed(req: Request) -> anyhow::Result<Response> {
    let body_bytes = req.body();

    // Validate signed request and extract payload (UNIVERSAL)
    let result: ProtectedEndpointResult<serde_json::Value> =
        match ProtectedEndpointMiddleware::validate_request(&req, body_bytes).await {
            Ok(result) => result,
            Err(error_response) => return Ok(error_response),
        };

    println!(
        "✅ API Key endpoint: validated signed request for user {}",
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
    handle_api_key_with_params(params, provided_seed)
}

/// Handles the /api/api-key endpoint for API key generation with ak_ prefix
pub fn handle_api_key(params: HashMap<String, String>) -> anyhow::Result<Response> {
    // Generate random 32-byte seed
    let seed_32 = generate_random_seed();
    handle_api_key_with_params(params, Some(seed_32))
}

/// Core logic for handling API key generation
fn handle_api_key_with_params(
    params: HashMap<String, String>,
    provided_seed: Option<[u8; 32]>,
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
    if let Err(e) = validate_length(length, min_length, 64) {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "text/plain")
            .body(format!("API key {}", e))
            .build());
    }

    // Use provided seed or generate random one
    let seed_32 = provided_seed.unwrap_or_else(generate_random_seed);
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
