//! TRAMO 1/3: No key rotation logic (simple token refresh)

use spin_sdk::http::Response;
use tracing::error;

use super::utilities::{
    create_error_response, decode_username_to_user_id, serialize_response_to_json,
};
use crate::types::responses::JwtAuthResponse;
use crate::utils::JwtUtils;
use crate::utils::signed_response::SignedResponseGenerator;
use crate::utils::crypto::backend_keys::get_backend_x25519_public_key;

/// Handle token refresh without key rotation (TRAMO 1/3)
///
/// When token is still fresh (< 2/3 consumed), just refresh access token
/// No new refresh token, no key rotation, no cookies
///
/// # Arguments
/// * `username` - Base58 encoded username
/// * `ed25519_pub_key` - Current Ed25519 public key bytes
/// * `x25519_pub_key` - Current X25519 public key bytes
///
/// # Returns
/// * `anyhow::Result<Response>` - HTTP response with new access token
pub fn handle_no_rotation(username: &str, ed25519_pub_key: &[u8; 32], x25519_pub_key: &[u8; 32]) -> anyhow::Result<Response> {
    // Create access token with existing Ed25519 and X25519 pub_keys
    let (access_token, _) = match JwtUtils::create_access_token_from_username(username, ed25519_pub_key, x25519_pub_key) {
        Ok((token, exp)) => (token, exp),
        Err(e) => {
            error!("❌ Refresh: Failed to create access token: {}", e);
            return create_error_response(500, &format!("Failed to create access token: {}", e));
        }
    };

    // Decode username to user_id bytes
    let user_id = match decode_username_to_user_id(username) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("❌ Refresh: {}", e);
            return create_error_response(500, "Invalid username format");
        }
    };

    // Convert X25519 pub_key to hex for per-user X25519 derivation
    let x25519_pub_key_hex = hex::encode(x25519_pub_key);

    // Get backend's per-user X25519 public key for E2E encryption
    // CRITICAL: Use X25519 pub_key for X25519 derivation (not Ed25519!)
    let backend_x25519_public = match get_backend_x25519_public_key(&user_id, &x25519_pub_key_hex) {
        Ok(key) => key,
        Err(e) => {
            error!("❌ Refresh: Failed to derive backend X25519 public key (per-user): {}", e);
            return create_error_response(500, "Failed to derive backend public key");
        }
    };
    let backend_x25519_public_hex = hex::encode(backend_x25519_public.as_bytes());

    // Create payload WITHOUT expires_at (no new refresh cookie)
    let payload = JwtAuthResponse::new(
        access_token,
        username.to_string(),
        None,
        None, // No expires_at - no new refresh cookie
        None, // No server_pub_key - no key rotation
        Some(backend_x25519_public_hex), // server_x25519_pub_key for E2E encryption
    );

    // Generate signed response WITHOUT server_pub_key (no key rotation)
    // Use Ed25519 pub_key for signing (signature validation on frontend)
    let ed25519_pub_key_hex = hex::encode(ed25519_pub_key);
    let signed_response =
        match SignedResponseGenerator::create_signed_response(payload, &user_id, &ed25519_pub_key_hex) {
            Ok(response) => response,
            Err(e) => {
                error!("❌ CRITICAL: Cannot create signed response: {}", e);
                return create_error_response(500, "Cryptographic signature failure");
            }
        };

    // Serialize response to JSON
    let response_json = match serialize_response_to_json(&signed_response) {
        Ok(json) => json,
        Err(e) => {
            error!("❌ Refresh: {}", e);
            return create_error_response(500, "Response serialization failed");
        }
    };

    // Build HTTP response WITHOUT refresh cookie
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(response_json)
        .build())
}
