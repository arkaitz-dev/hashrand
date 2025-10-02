//! API key generation endpoint with SignedResponse
//!
//! Provides GET and POST endpoints for API key generation with:
//! - JWT authentication and Ed25519 signature validation
//! - SignedResponse for all outputs (enterprise security)
//! - SOLID/DRY/KISS architecture with <225 lines

use crate::types::{AlphabetType, CustomHashResponse};
use crate::utils::protected_endpoint::{extract_seed_from_payload, payload_to_params};
use crate::utils::{
    ProtectedEndpointMiddleware, ProtectedEndpointResult, create_error_response,
    create_signed_endpoint_response, extract_crypto_material_from_request, generate_otp,
    generate_random_seed, generate_with_seed, handle_signed_get_request, seed_to_base58,
    validate_length,
};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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

/// Handle GET requests with SignedResponse (DRY implementation)
fn handle_api_key_get(req: Request) -> anyhow::Result<Response> {
    handle_signed_get_request(&req, generate_api_key_signed)
}

/// Handle POST requests with signed request validation
pub async fn handle_api_key_post_signed(req: Request) -> anyhow::Result<Response> {
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
        Err(e) => {
            return Ok(create_error_response(
                401,
                &format!("Crypto extraction failed: {}", e),
            ));
        }
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

    // Generate API key and create signed response
    generate_api_key_signed(&params, &crypto_material)
}

/// Generate secure API key and return SignedResponse (DRY implementation)
fn generate_api_key_signed(
    params: &HashMap<String, String>,
    crypto_material: &crate::utils::CryptoMaterial,
) -> anyhow::Result<Response> {
    // Parse parameters with API key-specific defaults
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

    // Validate length (min_length-64 for API keys)
    if let Err(e) = validate_length(length, min_length, 64) {
        return Ok(create_error_response(400, &format!("API key {}", e)));
    }

    // Get or generate seed
    let seed_32 = if let Some(seed_str) = params.get("seed") {
        crate::utils::base58_to_seed(seed_str)
            .map_err(|e| anyhow::anyhow!("Invalid seed: {}", e))?
    } else {
        generate_random_seed()
    };

    let seed_base58 = seed_to_base58(&seed_32);

    // Generate API key with ak_ prefix using seeded generator
    let alphabet = alphabet_type.as_chars();
    let key_part = generate_with_seed(seed_32, length, &alphabet);
    let api_key = format!("ak_{}", key_part);

    // Generate OTP and timestamp
    let otp = generate_otp(seed_32);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| anyhow::anyhow!("Time error: {}", e))?
        .as_secs();

    // Create payload directly
    let payload = CustomHashResponse::new(api_key, seed_base58, otp, timestamp);

    // Create signed response using DRY helper
    match create_signed_endpoint_response(payload, crypto_material) {
        Ok(signed_response) => Ok(signed_response),
        Err(e) => Ok(create_error_response(
            500,
            &format!("Failed to create signed response: {}", e),
        )),
    }
}
