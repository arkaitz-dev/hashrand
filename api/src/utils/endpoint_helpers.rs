//! Shared helper functions for endpoint handlers
//!
//! Contains common functionality used across all generation endpoint handlers:
//! - Query parameter extraction
//! - Error response creation
//! - Pattern validation for security
//! - Pattern-avoiding generation logic

use crate::utils::auth::ErrorResponse;
use crate::utils::generate_with_seed;
use spin_sdk::http::{Request, Response};
use std::collections::HashMap;

/// Extract query parameters from request URI (DRY helper)
pub fn extract_query_params(req: &Request) -> HashMap<String, String> {
    let uri_str = req.uri().to_string();
    let query = if let Some(idx) = uri_str.find('?') {
        &uri_str[idx + 1..]
    } else {
        ""
    };

    query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect()
}

/// Create error response with JSON body (DRY helper)
pub fn create_error_response(status: u16, message: &str) -> Response {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&ErrorResponse {
                error: message.to_string(),
            })
            .unwrap_or_else(|_| r#"{"error":"Internal error"}"#.to_string()),
        )
        .build()
}

/// Create authentication error response (401) (DRY helper)
pub fn create_auth_error_response(message: &str) -> Response {
    create_error_response(401, message)
}

/// Create client error response (400) (DRY helper)
pub fn create_client_error_response(message: &str) -> Response {
    create_error_response(400, message)
}

/// Create server error response (500) (DRY helper)
pub fn create_server_error_response(message: &str) -> Response {
    create_error_response(500, message)
}

/// Create forbidden error response (403) (DRY helper)
pub fn create_forbidden_response(message: &str) -> Response {
    create_error_response(403, message)
}

/// Helper function to check unwanted patterns for security
pub fn contains_unwanted_patterns(s: &str) -> bool {
    s.contains("--") || s.contains("__")
}

/// Generate hash avoiding unwanted patterns using seeded generator
/// Used for passwords and custom hashes with FullWithSymbols alphabet
pub fn generate_avoiding_unwanted_patterns(
    length: usize,
    alphabet: &[char],
    prefix: &str,
    suffix: &str,
    seed: [u8; 32],
) -> String {
    const MAX_ATTEMPTS: usize = 50;
    for attempt in 1..=MAX_ATTEMPTS {
        let mut attempt_seed = seed;
        attempt_seed[0] = attempt_seed[0].wrapping_add(attempt as u8);
        let hash = generate_with_seed(attempt_seed, length, alphabet);
        let result = format!("{}{}{}", prefix, hash, suffix);
        if !contains_unwanted_patterns(&result) {
            return result;
        }
    }
    // Fallback: return result even with unwanted patterns
    let hash = generate_with_seed(seed, length, alphabet);
    format!("{}{}{}", prefix, hash, suffix)
}

/// Generate password avoiding unwanted patterns (password-specific wrapper)
pub fn generate_password_avoiding_patterns(
    length: usize,
    alphabet: &[char],
    seed: [u8; 32],
) -> String {
    generate_avoiding_unwanted_patterns(length, alphabet, "", "", seed)
}

/// Universal handler for GET requests with Ed25519 signature validation + SignedResponse
/// Eliminates code duplication across all generation endpoints
pub fn handle_signed_get_request<F>(
    req: &Request,
    generate_signed_fn: F,
) -> anyhow::Result<Response>
where
    F: FnOnce(
        &std::collections::HashMap<String, String>,
        &crate::utils::CryptoMaterial,
    ) -> anyhow::Result<Response>,
{
    // Extract crypto material using DRY helper
    let crypto_material = match crate::utils::extract_crypto_material_from_request(req) {
        Ok(material) => material,
        Err(e) => {
            return Ok(create_auth_error_response(&format!(
                "Authentication failed: {}",
                e
            )));
        }
    };

    // Extract query parameters
    let params = extract_query_params(req);

    // Validate Ed25519 signature (GET must have signature parameter)
    let mut validated_params = params;
    if let Err(e) = crate::utils::SignedRequestValidator::validate_query_params(
        &mut validated_params,
        &crypto_material.pub_key_hex,
    ) {
        return Ok(create_auth_error_response(&format!(
            "Signature validation failed: {}",
            e
        )));
    }

    // Generate fresh seed if not provided
    if !validated_params.contains_key("seed") {
        let seed = crate::utils::generate_random_seed();
        validated_params.insert("seed".to_string(), crate::utils::seed_to_base58(&seed));
    }

    // Call the specific generation function
    generate_signed_fn(&validated_params, &crypto_material)
}
