//! Token generation logic for proactive renewal

use chrono::DateTime;
use spin_sdk::http::Response;

use super::super::jwt_middleware_errors::create_auth_error_response;
use super::super::jwt_middleware_types::RenewedTokens;
use crate::utils::JwtUtils;

/// Generate renewed access and refresh tokens
///
/// # Arguments
/// * `username` - User identifier
/// * `refresh_expires_at` - Refresh token expiration timestamp
/// * `pub_key_hex` - Public key hex string
/// * `user_id` - User ID bytes
///
/// # Returns
/// * `Result<RenewedTokens, Response>` - Renewed tokens or error response
pub fn generate_renewed_tokens(
    username: &str,
    refresh_expires_at: i64,
    pub_key_hex: String,
    user_id: Vec<u8>,
) -> Result<RenewedTokens, Response> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System clock error")
        .as_secs() as i64;

    // Decode pub_key from hex
    let pub_key = decode_pub_key_from_hex(&pub_key_hex)?;

    // Generate refresh expires datetime
    let refresh_expires_datetime = DateTime::from_timestamp(refresh_expires_at, 0)
        .ok_or("Invalid refresh token expiration timestamp")
        .map_err(|e| create_auth_error_response(e, None))?;

    // Generate new access token - PRESERVE refresh context for 2/3 system
    let (new_access_token, access_expires) =
        match JwtUtils::create_access_token_from_username_with_refresh_context(
            username,
            refresh_expires_datetime,
            &pub_key,
        ) {
            Ok((token, exp)) => (token, exp),
            Err(e) => {
                println!(
                    "Failed to create new access token during proactive renewal: {}",
                    e
                );
                return Err(create_auth_error_response(
                    "Failed to renew access token",
                    None,
                ));
            }
        };

    // Generate new refresh token with pub_key from current token
    let (new_refresh_token, _refresh_expires) =
        match JwtUtils::create_refresh_token_from_username(username, &pub_key) {
            Ok((token, exp)) => (token, exp),
            Err(e) => {
                println!(
                    "Failed to create new refresh token during proactive renewal: {}",
                    e
                );
                return Err(create_auth_error_response(
                    "Failed to renew refresh token",
                    None,
                ));
            }
        };

    let expires_in = access_expires.timestamp() - now;

    Ok(RenewedTokens {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        expires_in,
        user_id,
        pub_key_hex,
    })
}

/// Decode public key from hex string
///
/// # Arguments
/// * `pub_key_hex` - Public key as hex string
///
/// # Returns
/// * `Result<[u8; 32], Response>` - Decoded pub_key or error response
fn decode_pub_key_from_hex(pub_key_hex: &str) -> Result<[u8; 32], Response> {
    let pub_key_bytes = hex::decode(pub_key_hex)
        .map_err(|e| create_auth_error_response(&format!("Invalid pub_key hex: {}", e), None))?;

    if pub_key_bytes.len() != 32 {
        return Err(create_auth_error_response("Invalid pub_key length", None));
    }

    let mut pub_key = [0u8; 32];
    pub_key.copy_from_slice(&pub_key_bytes);

    Ok(pub_key)
}
