//! PERIOD 2/3: Key rotation logic (complete token + keypair refresh)

use spin_sdk::http::Response;
use tracing::error;

use super::threshold::get_refresh_duration_seconds;
use super::utilities::{
    create_error_response, decode_username_to_user_id, serialize_response_to_json,
};
use crate::types::responses::JwtAuthResponse;
use crate::utils::JwtUtils;
use crate::utils::jwt::custom_token_api::create_custom_refresh_token_from_username;
use crate::utils::signed_response::SignedResponseGenerator;
use crate::utils::crypto::backend_keys::get_backend_x25519_public_key;

/// Handle token refresh with key rotation (PERIOD 2/3)
///
/// When token has consumed 2/3 of its lifetime, perform complete key rotation:
/// - Create new access token with NEW Ed25519 and X25519 pub_keys
/// - Create new refresh token with NEW Ed25519 and X25519 pub_keys
/// - Sign response with OLD Ed25519 key (MITM protection)
/// - Include NEW server_pub_key in payload
/// - Delete old refresh cookie and create new one
///
/// # Arguments
/// * `username` - Base58 encoded username
/// * `old_ed25519_pub_key_hex` - Current (OLD) Ed25519 public key hex string
/// * `new_ed25519_pub_key_hex` - New Ed25519 public key hex string from client
/// * `new_x25519_pub_key_hex` - New X25519 public key hex string from client
/// * `domain` - Optional hostname for cookie Domain attribute
///
/// # Returns
/// * `anyhow::Result<Response>` - HTTP response with new tokens and cookies
pub fn handle_key_rotation(
    username: &str,
    old_ed25519_pub_key_hex: &str,
    new_ed25519_pub_key_hex: &str,
    new_x25519_pub_key_hex: &str,
    domain: Option<String>,
) -> anyhow::Result<Response> {
    // Validate and convert new Ed25519 pub_key
    let new_ed25519_pub_key_array = validate_new_pub_key(new_ed25519_pub_key_hex)?;

    // Validate and convert new X25519 pub_key
    let new_x25519_pub_key_array = validate_new_pub_key(new_x25519_pub_key_hex)?;

    // Create access_token with NEW Ed25519 and X25519 pub_keys
    let (access_token, _) =
        match JwtUtils::create_access_token_from_username(username, &new_ed25519_pub_key_array, &new_x25519_pub_key_array) {
            Ok((token, exp)) => (token, exp),
            Err(e) => {
                error!("❌ Refresh: Failed to create access token: {}", e);
                return create_error_response(
                    500,
                    &format!("Failed to create access token: {}", e),
                );
            }
        };

    // Create refresh_token with NEW Ed25519 and X25519 pub_keys
    let (new_refresh_token, _) =
        match create_custom_refresh_token_from_username(username, &new_ed25519_pub_key_array, &new_x25519_pub_key_array) {
            Ok((token, exp)) => (token, exp),
            Err(e) => {
                error!("❌ Refresh: Failed to create refresh token: {}", e);
                return create_error_response(
                    500,
                    &format!("Failed to create refresh token: {}", e),
                );
            }
        };

    // Calculate expires_at for new refresh cookie
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System clock error")
        .as_secs() as i64;
    let expires_at = now + get_refresh_duration_seconds();

    // Decode username to user_id bytes
    let user_id = match decode_username_to_user_id(username) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("❌ Refresh: {}", e);
            return create_error_response(500, "Invalid username format");
        }
    };

    // Get backend's per-user X25519 public key for E2E encryption
    // CRITICAL: Use NEW X25519 pub_key for derivation (client is rotating to new X25519 keypair)
    let backend_x25519_public = match get_backend_x25519_public_key(&user_id, new_x25519_pub_key_hex) {
        Ok(key) => key,
        Err(e) => {
            error!("❌ Refresh: Failed to derive backend X25519 public key (per-user, NEW): {}", e);
            return create_error_response(500, "Failed to derive backend public key");
        }
    };
    let backend_x25519_public_hex = hex::encode(backend_x25519_public.as_bytes());

    // Create payload with expires_at
    let payload = JwtAuthResponse::new(
        access_token,
        username.to_string(),
        None,
        Some(expires_at),
        None, // server_pub_key will be added by create_signed_response_with_rotation
        Some(backend_x25519_public_hex), // server_x25519_pub_key for E2E encryption
        None, // encrypted_privkey_context only in magic link validation
    );

    // Generate signed response with key rotation
    // SECURITY: Sign with OLD Ed25519 pub_key but include NEW Ed25519 server_pub_key in payload
    let signed_response = match SignedResponseGenerator::create_signed_response_with_rotation(
        payload,
        &user_id,
        old_ed25519_pub_key_hex, // ✅ OLD Ed25519: derive signing key (MITM protection)
        new_ed25519_pub_key_hex, // ✅ NEW Ed25519: derive server_pub_key for payload (rotation)
    ) {
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

    // Build response with cookie rotation
    build_rotation_response(response_json, new_refresh_token, domain)
}

/// Validate new_pub_key from hex string
///
/// # Arguments
/// * `new_pub_key_hex` - New public key as hex string
///
/// # Returns
/// * `anyhow::Result<[u8; 32]>` - Validated pub_key array or error
fn validate_new_pub_key(new_pub_key_hex: &str) -> anyhow::Result<[u8; 32]> {
    // Decode new_pub_key from hex
    let new_pub_key_bytes = hex::decode(new_pub_key_hex).map_err(|e| {
        error!("❌ Refresh: Invalid new_pub_key hex: {}", e);
        anyhow::anyhow!("Invalid new_pub_key format")
    })?;

    let new_pub_key_array: [u8; 32] = new_pub_key_bytes.try_into().map_err(|_| {
        error!("❌ Refresh: new_pub_key must be 32 bytes");
        anyhow::anyhow!("new_pub_key must be 32 bytes")
    })?;

    Ok(new_pub_key_array)
}

/// Build HTTP response with cookie rotation
///
/// Creates response with:
/// 1. First Set-Cookie: Delete old cookie (Max-Age=0)
/// 2. Second Set-Cookie: Create new cookie
///
/// # Arguments
/// * `response_json` - Serialized JSON response body
/// * `new_refresh_token` - New refresh token value
/// * `domain` - Optional hostname for cookie Domain attribute
///
/// # Returns
/// * `anyhow::Result<Response>` - Complete HTTP response
fn build_rotation_response(
    response_json: String,
    new_refresh_token: String,
    domain: Option<String>,
) -> anyhow::Result<Response> {
    let refresh_duration_seconds = get_refresh_duration_seconds();

    // Create cookie with Domain attribute if available
    let cookie_value = if let Some(ref domain_str) = domain {
        format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Domain={}; Path=/",
            new_refresh_token, refresh_duration_seconds, domain_str
        )
    } else {
        format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
            new_refresh_token, refresh_duration_seconds
        )
    };

    // 🍪 CRITICAL FIX: Delete OLD cookie explicitly before creating NEW one
    // Prevents duplicate cookies (OLD + NEW) in browser after key rotation
    // IMPORTANT: Delete cookie MUST have EXACT same Domain/Path as original cookie (RFC 6265)
    let delete_old_cookie = if let Some(ref domain_str) = domain {
        format!(
            "refresh_token=; Max-Age=0; HttpOnly; Secure; SameSite=Strict; Domain={}; Path=/",
            domain_str
        )
    } else {
        "refresh_token=; Max-Age=0; HttpOnly; Secure; SameSite=Strict; Path=/".to_string()
    };

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("set-cookie", &delete_old_cookie) // ✅ Delete OLD cookie first (exact match)
        .header("set-cookie", &cookie_value) // ✅ Create NEW cookie second
        .body(response_json)
        .build())
}
