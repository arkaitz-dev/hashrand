//! Universal handler helpers for DRY endpoint implementation
//!
//! Provides common functionality for all generation endpoints:
//! - JWT crypto material extraction
//! - SignedResponse creation
//! - Error handling consistency

use crate::utils::SignedResponseGenerator;
use spin_sdk::http::Request;

// Crypto material extracted from JWT for SignedResponse generation
pub struct CryptoMaterial {
    pub user_id: Vec<u8>,
    pub pub_key_hex: String,        // Ed25519 public key hex (for signatures)
    pub x25519_pub_key_hex: String, // X25519 public key hex (for ECDH)
}

// Universal function to extract crypto material from JWT Authorization header
//
// This DRY function eliminates code duplication across all protected endpoints
// by providing a single source of truth for JWT crypto material extraction.
//
// # Arguments
// * `req` - HTTP request with Authorization header
//
// # Returns
// * `Result<CryptoMaterial, String>` - Extracted crypto material or error
pub fn extract_crypto_material_from_request(req: &Request) -> Result<CryptoMaterial, String> {
    // Extract Authorization header
    let auth_header = req
        .header("authorization")
        .and_then(|h| h.as_str())
        .ok_or_else(|| "Missing Authorization header".to_string())?;

    // Extract Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| "Invalid Authorization header format".to_string())?;

    // Validate and extract claims
    let claims = crate::utils::JwtUtils::validate_access_token(token)
        .map_err(|e| format!("JWT validation failed: {}", e))?;

    // Convert username (Base58) back to user_id bytes
    let user_id = bs58::decode(&claims.sub)
        .into_vec()
        .map_err(|e| format!("Failed to decode Base58 username: {}", e))?;

    // Convert both pub_keys to hex strings
    let pub_key_hex = hex::encode(claims.ed25519_pub_key);
    let x25519_pub_key_hex = hex::encode(claims.x25519_pub_key);

    Ok(CryptoMaterial {
        user_id,
        pub_key_hex,
        x25519_pub_key_hex,
    })
}

// Universal function to create signed HTTP response
//
// This DRY function provides consistent SignedResponse generation for all endpoints,
// eliminating code duplication and ensuring uniform response format.
//
// # Arguments
// * `payload` - Response data to be signed
// * `crypto_material` - JWT crypto material for signing
//
// # Returns
// * `Result<spin_sdk::http::Response, String>` - Signed HTTP response or error
pub fn create_signed_endpoint_response<T>(
    payload: T,
    crypto_material: &CryptoMaterial,
) -> Result<spin_sdk::http::Response, String>
where
    T: serde::Serialize,
{
    SignedResponseGenerator::create_signed_http_response(
        payload,
        &crypto_material.user_id,
        &crypto_material.pub_key_hex,
    )
}

// Universal function to create signed response struct (for further processing)
//
// Alternative to HTTP response when you need the SignedResponse struct directly.
//
// # Arguments
// * `payload` - Response data to be signed
// * `crypto_material` - JWT crypto material for signing
// DELETED: Legacy function create_signed_response_struct removed - was completely unused, replaced by create_signed_endpoint_response
