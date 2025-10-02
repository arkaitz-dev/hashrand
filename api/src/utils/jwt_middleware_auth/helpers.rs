//! JWT Middleware Auth Helper Functions
//!
//! DRY utilities for authentication middleware

use spin_sdk::http::Response;

use crate::utils::jwt_middleware_errors::create_auth_error_response;

/// Decode Base58 username to user_id bytes (DRY utility)
///
/// # Arguments
/// * `username` - Base58-encoded username
///
/// # Returns
/// * `Result<Vec<u8>, Response>` - User ID bytes or error response
pub fn decode_username_to_user_id(username: &str) -> Result<Vec<u8>, Response> {
    bs58::decode(username).into_vec().map_err(|_| {
        println!("🔍 DEBUG: Failed to decode Base58 username");
        create_auth_error_response("Invalid username format", None)
    })
}

/// Create token validation error response
///
/// # Arguments
/// * `error_msg` - Original error message from token validation
///
/// # Returns
/// * `Response` - Formatted error response with appropriate message
pub fn create_token_validation_error(error_msg: &str) -> Response {
    let (error, expires_hint) = if error_msg.contains("expired") || error_msg.contains("exp") {
        (
            "Access token has expired. Use refresh token to obtain a new access token",
            Some("20 seconds from issuance".to_string()),
        )
    } else {
        ("Invalid access token. Please authenticate again", None)
    };

    create_auth_error_response(error, expires_hint)
}
