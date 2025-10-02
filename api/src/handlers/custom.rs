//! Custom hash generation endpoint with SignedResponse
//!
//! Provides GET and POST endpoints for custom hash generation with:
//! - JWT authentication and Ed25519 signature validation
//! - SignedResponse for all outputs (enterprise security)
//! - SOLID/DRY/KISS architecture with <225 lines

use crate::types::{AlphabetType, CustomHashResponse};
use crate::utils::protected_endpoint::{extract_seed_from_payload, payload_to_params};
use crate::utils::{
    ProtectedEndpointMiddleware, ProtectedEndpointResult, create_auth_error_response,
    create_client_error_response, create_server_error_response, create_signed_endpoint_response,
    extract_crypto_material_from_request, generate_avoiding_unwanted_patterns, generate_otp,
    generate_random_seed, generate_with_seed, handle_signed_get_request, seed_to_base58,
    validate_length, validate_prefix_suffix,
};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Main request handler for /api/custom endpoint
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

/// Handle GET requests with SignedResponse (DRY implementation)
pub fn handle_custom_get(req: Request) -> anyhow::Result<Response> {
    handle_signed_get_request(&req, generate_custom_hash_signed)
}

/// Handle POST requests with signed request validation
pub async fn handle_custom_post_signed(req: Request) -> anyhow::Result<Response> {
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
            return Ok(create_auth_error_response(&format!(
                "Crypto extraction failed: {}",
                e
            )));
        }
    };

    // Convert payload to parameters and extract seed
    let mut params = payload_to_params(&result.payload);
    let provided_seed = match extract_seed_from_payload(&result.payload) {
        Ok(seed) => seed,
        Err(e) => return Ok(create_auth_error_response(&e)),
    };

    // Use provided seed or generate fresh one
    if provided_seed.is_none() && !params.contains_key("seed") {
        let seed = generate_random_seed();
        params.insert("seed".to_string(), seed_to_base58(&seed));
    }

    // Generate hash and create signed response
    generate_custom_hash_signed(&params, &crypto_material)
}

/// Generate custom hash and return SignedResponse (DRY implementation)
fn generate_custom_hash_signed(
    params: &HashMap<String, String>,
    crypto_material: &crate::utils::CryptoMaterial,
) -> anyhow::Result<Response> {
    // Parse parameters with default values (inline for DRY)
    // Default length 21: Provides ~110 bits of entropy with Base58 (58^21 ≈ 2^110)
    // Balances strong security with reasonable output length for custom hashes
    let length = params
        .get("length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(21);

    let alphabet_type = params
        .get("alphabet")
        .map(|s| AlphabetType::from_str(s))
        .unwrap_or(AlphabetType::Base58);

    let prefix = params.get("prefix").cloned().unwrap_or_default();
    let suffix = params.get("suffix").cloned().unwrap_or_default();

    // Validate parameters
    if let Err(e) = validate_length(length, 2, 128) {
        return Ok(create_client_error_response(&e.to_string()));
    }
    if let Err(e) = validate_prefix_suffix(&prefix, "Prefix") {
        return Ok(create_client_error_response(&e.to_string()));
    }
    if let Err(e) = validate_prefix_suffix(&suffix, "Suffix") {
        return Ok(create_client_error_response(&e.to_string()));
    }

    // Get or generate seed
    let seed_32 = if let Some(seed_str) = params.get("seed") {
        crate::utils::base58_to_seed(seed_str)
            .map_err(|e| anyhow::anyhow!("Invalid seed: {}", e))?
    } else {
        generate_random_seed()
    };

    let seed_base58 = seed_to_base58(&seed_32);

    // Generate hash using seeded generator
    let alphabet = alphabet_type.as_chars();
    let hash = if alphabet_type == AlphabetType::FullWithSymbols {
        generate_avoiding_unwanted_patterns(length, &alphabet, &prefix, &suffix, seed_32)
    } else {
        let base_hash = generate_with_seed(seed_32, length, &alphabet);
        format!("{}{}{}", prefix, base_hash, suffix)
    };

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
        Err(e) => Ok(create_server_error_response(&format!(
            "Failed to create signed response: {}",
            e
        ))),
    }
}

// DELETED: Legacy function handle_custom_with_params removed - was completely unused legacy code
