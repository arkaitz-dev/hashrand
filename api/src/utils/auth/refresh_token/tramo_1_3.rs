//! TRAMO 1/3: No key rotation logic (simple token refresh)

use spin_sdk::http::Response;

use super::utilities::{
    create_error_response, decode_username_to_user_id, serialize_response_to_json,
};
use crate::types::responses::JwtAuthResponse;
use crate::utils::JwtUtils;
use crate::utils::signed_response::SignedResponseGenerator;

/// Handle token refresh without key rotation (TRAMO 1/3)
///
/// When token is still fresh (< 2/3 consumed), just refresh access token
/// No new refresh token, no key rotation, no cookies
///
/// # Arguments
/// * `username` - Base58 encoded username
/// * `pub_key` - Current Ed25519 public key bytes
///
/// # Returns
/// * `anyhow::Result<Response>` - HTTP response with new access token
pub fn handle_no_rotation(username: &str, pub_key: &[u8; 32]) -> anyhow::Result<Response> {
    // Create access token with existing pub_key
    let (access_token, _) = match JwtUtils::create_access_token_from_username(username, pub_key) {
        Ok((token, exp)) => (token, exp),
        Err(e) => {
            println!("❌ Refresh: Failed to create access token: {}", e);
            return create_error_response(500, &format!("Failed to create access token: {}", e));
        }
    };

    // Decode username to user_id bytes
    let user_id = match decode_username_to_user_id(username) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("❌ Refresh: {}", e);
            return create_error_response(500, "Invalid username format");
        }
    };

    // Create payload WITHOUT expires_at (no new refresh cookie)
    let payload = JwtAuthResponse::new(
        access_token,
        username.to_string(),
        None,
        None, // No expires_at - no new refresh cookie
        None, // No server_pub_key - no key rotation
    );

    // Convert pub_key to hex for signed response
    let pub_key_hex = hex::encode(pub_key);

    // Generate signed response WITHOUT server_pub_key (no key rotation)
    let signed_response =
        match SignedResponseGenerator::create_signed_response(payload, &user_id, &pub_key_hex) {
            Ok(response) => response,
            Err(e) => {
                println!("❌ CRITICAL: Cannot create signed response: {}", e);
                return create_error_response(500, "Cryptographic signature failure");
            }
        };

    // Serialize response to JSON
    let response_json = match serialize_response_to_json(&signed_response) {
        Ok(json) => json,
        Err(e) => {
            println!("❌ Refresh: {}", e);
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
