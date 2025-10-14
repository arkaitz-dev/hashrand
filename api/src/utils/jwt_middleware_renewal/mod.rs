//! JWT middleware renewal logic - Proactive token renewal and 2/3 system

mod non_signed_handler;
mod response_utilities;
mod signed_response_handler;
mod threshold;
mod token_generation;

use serde_json::Value;
use spin_sdk::http::Response;
use tracing::error;

use non_signed_handler::add_tokens_to_headers;
use signed_response_handler::add_tokens_to_signed_response;
use threshold::is_in_renewal_window;
use token_generation::generate_renewed_tokens;

use super::jwt_middleware_types::RenewedTokens;

/// Check if proactive token renewal is needed based on 2/3 threshold
///
/// # Arguments
/// * `username` - User identifier for token generation
/// * `refresh_expires_at` - Refresh token expiration timestamp
/// * `now` - Current timestamp
/// * `user_id` - User ID bytes for signed response generation
/// * `pub_key_hex` - Public key hex for signed response generation
///
/// # Returns
/// * `Result<Option<RenewedTokens>, Response>` - New tokens if renewal needed, None otherwise
pub fn check_proactive_renewal(
    username: &str,
    refresh_expires_at: i64,
    now: i64,
    user_id: Vec<u8>,
    pub_key_hex: String,
) -> Result<Option<RenewedTokens>, Response> {
    // Check if we're in 2/3 renewal window
    let needs_renewal = match is_in_renewal_window(refresh_expires_at, now) {
        Ok(result) => result,
        Err(e) => {
            error!("❌ Renewal threshold check failed: {}", e);
            return Err(super::jwt_middleware_errors::create_auth_error_response(
                "Server configuration error",
                None,
            ));
        }
    };

    if needs_renewal {
        // Generate renewed tokens
        let renewed_tokens =
            generate_renewed_tokens(username, refresh_expires_at, pub_key_hex, user_id)?;
        Ok(Some(renewed_tokens))
    } else {
        // No renewal needed
        Ok(None)
    }
}

/// Add renewed tokens to HTTP response headers and cookies
///
/// For signed responses, adds the access token to the payload and re-signs.
/// For non-signed responses, uses headers (backward compatibility).
///
/// # Arguments
/// * `response` - Original response from handler
/// * `renewed_tokens` - New access and refresh tokens to include
///
/// # Returns
/// * `Response` - Response with added token headers/payload and cookies
pub fn add_renewed_tokens_to_response(
    response: Response,
    renewed_tokens: RenewedTokens,
) -> Response {
    // Extract response body
    let body_vec = response.body().to_vec();
    let body_str = String::from_utf8_lossy(&body_vec);

    // Check if response is signed (contains {"payload": ..., "signature": ...})
    let is_signed_response = body_str.trim_start().starts_with('{')
        && serde_json::from_str::<Value>(&body_str)
            .map(|v| v.get("payload").is_some() && v.get("signature").is_some())
            .unwrap_or(false);

    if is_signed_response {
        // Handle signed response: add access token to payload and re-sign
        add_tokens_to_signed_response(response, renewed_tokens, &body_str)
    } else {
        // Handle non-signed response: use headers (backward compatibility)
        add_tokens_to_headers(response, renewed_tokens)
    }
}
