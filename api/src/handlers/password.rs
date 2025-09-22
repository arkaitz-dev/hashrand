//! Password generation endpoint with SignedResponse
//!
//! Provides GET and POST endpoints for secure password generation with:
//! - JWT authentication and Ed25519 signature validation
//! - SignedResponse for all outputs (enterprise security)
//! - SOLID/DRY/KISS architecture with <225 lines

use crate::types::{AlphabetType, CustomHashResponse};
use crate::utils::protected_endpoint_middleware::{extract_seed_from_payload, payload_to_params};
use crate::utils::{
    ProtectedEndpointMiddleware, ProtectedEndpointResult, SignedRequestValidator,
    generate_otp, generate_random_seed, generate_with_seed, seed_to_base58,
    validate_length, extract_crypto_material_from_request, create_signed_endpoint_response,
    extract_query_params, create_error_response, generate_password_avoiding_patterns,
    handle_signed_get_request,
};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Main request handler for /api/password endpoint
pub async fn handle_password_request(req: Request) -> anyhow::Result<Response> {
    match req.method() {
        Method::Get => handle_password_get(req),
        Method::Post => handle_password_post_signed(req).await,
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "text/plain")
            .body("Method not allowed")
            .build()),
    }
}

/// Handle GET requests with SignedResponse (DRY implementation)
pub fn handle_password_get(req: Request) -> anyhow::Result<Response> {
    handle_signed_get_request(&req, generate_password_signed)
}

/// Handle POST requests with signed request validation
pub async fn handle_password_post_signed(req: Request) -> anyhow::Result<Response> {
    let body_bytes = req.body();

    // Validate signed request using protected middleware
    let result: ProtectedEndpointResult<serde_json::Value> =
        match ProtectedEndpointMiddleware::validate_request(&req, body_bytes).await {
            Ok(result) => result,
            Err(error_response) => return Ok(error_response),
        };

    // Extract crypto material for signing response
    let crypto_material = match extract_crypto_material_from_request(&req) {
        Ok(material) => material,
        Err(e) => return Ok(create_error_response(401, &format!("Crypto extraction failed: {}", e))),
    };

    // Convert payload to parameters and extract seed
    let mut params = payload_to_params(&result.payload);
    let provided_seed = match extract_seed_from_payload(&result.payload) {
        Ok(seed) => seed,
        Err(e) => return Ok(create_error_response(400, &e)),
    };

    // Use provided seed or generate fresh one
    if provided_seed.is_none() && !params.contains_key("seed") {
        let seed = generate_random_seed();
        params.insert("seed".to_string(), seed_to_base58(&seed));
    }

    // Generate password and create signed response
    generate_password_signed(&params, &crypto_material)
}

/// Generate secure password and return SignedResponse (DRY implementation)
fn generate_password_signed(
    params: &HashMap<String, String>,
    crypto_material: &crate::utils::CryptoMaterial,
) -> anyhow::Result<Response> {
    // Parse parameters with password-specific defaults
    let length = params
        .get("length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(32); // Default password length

    // Password uses FullWithSymbols alphabet for security
    let alphabet_type = AlphabetType::FullWithSymbols;

    // Validate length (21-44 for passwords - security requirement)
    if let Err(e) = validate_length(length, 21, 44) {
        return Ok(create_error_response(400, &format!("Password {}", e)));
    }

    // Get or generate seed
    let seed_32 = if let Some(seed_str) = params.get("seed") {
        crate::utils::base58_to_seed(seed_str)
            .map_err(|e| anyhow::anyhow!("Invalid seed: {}", e))?
    } else {
        generate_random_seed()
    };

    let seed_base58 = seed_to_base58(&seed_32);

    // Generate password avoiding unwanted patterns
    let alphabet = alphabet_type.as_chars();
    let hash = generate_password_avoiding_patterns(length, &alphabet, seed_32);

    // Generate OTP and timestamp
    let otp = generate_otp(seed_32);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| anyhow::anyhow!("Time error: {}", e))?
        .as_secs();

    // Create payload directly
    let payload = CustomHashResponse::new(hash, seed_base58, otp, timestamp);

    // Create signed response using DRY helper
    match create_signed_endpoint_response(payload, crypto_material) {
        Ok(signed_response) => Ok(signed_response),
        Err(e) => Ok(create_error_response(500, &format!("Failed to create signed response: {}", e))),
    }
}



/// Legacy function preserved for existing logic
pub fn handle_password_with_params(
    params: HashMap<String, String>,
    provided_seed: Option<[u8; 32]>,
) -> anyhow::Result<Response> {
    // Set password-specific defaults
    let mut password_params = params;
    password_params.entry("length".to_string()).or_insert("32".to_string());

    // Use DRY crypto material (create dummy for legacy compatibility)
    let dummy_crypto = crate::utils::CryptoMaterial {
        user_id: vec![0; 16],
        pub_key_hex: "".to_string(),
    };

    // Legacy direct implementation (avoid circular dependency)
    let length = password_params.get("length").and_then(|s| s.parse::<usize>().ok()).unwrap_or(32);
    if let Err(e) = validate_length(length, 21, 44) {
        return Ok(create_error_response(400, &format!("Password {}", e)));
    }

    let seed_32 = provided_seed.unwrap_or_else(generate_random_seed);
    let seed_base58 = seed_to_base58(&seed_32);
    let alphabet = AlphabetType::FullWithSymbols.as_chars();
    let hash = generate_password_avoiding_patterns(length, &alphabet, seed_32);
    let otp = generate_otp(seed_32);
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let response = CustomHashResponse::new(hash, seed_base58, otp, timestamp);
    let json_body = serde_json::to_string(&response)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_body)
        .build())
}