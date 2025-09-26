//! JWT middleware renewal logic - Proactive token renewal and 2/3 system

use crate::utils::JwtUtils;
use crate::utils::jwt::config::get_refresh_token_duration_minutes;
use crate::utils::{SignedRequestValidator, SignedResponse, SignedResponseGenerator};
use chrono::DateTime;
use serde_json::Value;
use spin_sdk::http::Response;

use super::jwt_middleware_errors::create_auth_error_response;
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
    // Get refresh token duration in minutes
    let refresh_duration_minutes = match get_refresh_token_duration_minutes() {
        Ok(duration) => duration,
        Err(e) => {
            println!("Failed to get refresh token duration: {}", e);
            return Err(create_auth_error_response(
                "Server configuration error",
                None,
            ));
        }
    };

    let refresh_duration_seconds = refresh_duration_minutes * 60;
    let time_remaining = refresh_expires_at - now;

    // Calculate 2/3 threshold: if remaining time is less than 2/3 of total duration
    let two_thirds_threshold = (refresh_duration_seconds * 2) / 3;

    if time_remaining < two_thirds_threshold as i64 {
        println!(
            "Proactive renewal triggered: {}s remaining < {}s threshold",
            time_remaining, two_thirds_threshold
        );

        // Generate new access token - PRESERVE refresh context for 2/3 system
        let refresh_expires_datetime = DateTime::from_timestamp(refresh_expires_at, 0)
            .ok_or("Invalid refresh token expiration timestamp")
            .map_err(|e| create_auth_error_response(e, None))?;
        // TODO: Extract pub_key from refresh token claims instead of using placeholder
        let placeholder_pub_key = [0u8; 32];
        let (new_access_token, access_expires) =
            match JwtUtils::create_access_token_from_username_with_refresh_context(
                username,
                refresh_expires_datetime,
                &placeholder_pub_key,
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

        // Generate new refresh token
        let (new_refresh_token, _refresh_expires) =
            match JwtUtils::create_refresh_token_from_username(username, None) {
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

        Ok(Some(RenewedTokens {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
            expires_in,
            user_id,
            pub_key_hex,
        }))
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
        handle_signed_response_with_tokens(response, renewed_tokens, &body_str)
    } else {
        // Handle non-signed response: use headers (backward compatibility)
        handle_non_signed_response_with_tokens(response, renewed_tokens)
    }
}

/// Handle signed response: add access token to payload and re-sign
fn handle_signed_response_with_tokens(
    response: Response,
    renewed_tokens: RenewedTokens,
    body_str: &str,
) -> Response {
    // Parse existing signed response (now with Base64-encoded JSON payload)
    let signed_response: SignedResponse = match serde_json::from_str(body_str) {
        Ok(resp) => resp,
        Err(_) => {
            println!("🔍 DEBUG: Failed to parse signed response, falling back to headers");
            return handle_non_signed_response_with_tokens(response, renewed_tokens);
        }
    };

    // CORRECTED: Deserialize Base64-encoded JSON payload, modify, and re-serialize
    let json_string = match SignedRequestValidator::decode_payload_base64(&signed_response.payload)
    {
        Ok(json) => json,
        Err(e) => {
            println!("❌ DEBUG: Failed to decode Base64 payload: {}", e);
            return handle_non_signed_response_with_tokens(response, renewed_tokens);
        }
    };

    let mut enhanced_payload: Value = match serde_json::from_str(&json_string) {
        Ok(payload) => payload,
        Err(e) => {
            println!("❌ DEBUG: Failed to parse JSON payload: {}", e);
            return handle_non_signed_response_with_tokens(response, renewed_tokens);
        }
    };

    // Add access token to deserialized payload
    if let Value::Object(ref mut map) = enhanced_payload {
        map.insert(
            "access_token".to_string(),
            Value::String(renewed_tokens.access_token.clone()),
        );
        // Reactive auth: no expires_in needed - frontend responds to 401s
    } else {
        println!("🔍 DEBUG: Payload is not an object, cannot add access_token");
        return handle_non_signed_response_with_tokens(response, renewed_tokens);
    }

    // Generate new signed response with enhanced payload
    let new_signed_response = match SignedResponseGenerator::create_signed_response(
        enhanced_payload,
        &renewed_tokens.user_id,
        &renewed_tokens.pub_key_hex,
    ) {
        Ok(resp) => resp,
        Err(e) => {
            println!(
                "🔍 DEBUG: Failed to create signed response: {}, falling back to headers",
                e
            );
            return handle_non_signed_response_with_tokens(response, renewed_tokens);
        }
    };

    // Build response with new signed body
    let new_body = match serde_json::to_string(&new_signed_response) {
        Ok(body) => body,
        Err(_) => {
            println!("🔍 DEBUG: Failed to serialize signed response, falling back to headers");
            return handle_non_signed_response_with_tokens(response, renewed_tokens);
        }
    };

    let mut binding = Response::builder();
    let mut builder = binding.status(*response.status());

    // Copy existing headers (except content-length which will change)
    for (name, value) in response.headers() {
        if name.to_lowercase() != "content-length" {
            builder = builder.header(name, value.as_str().unwrap_or(""));
        }
    }

    // Add refresh token cookie if provided (2/3 system logic)
    if !renewed_tokens.refresh_token.is_empty() {
        let refresh_duration_minutes = get_refresh_token_duration_minutes()
            .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");
        let refresh_cookie = format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
            renewed_tokens.refresh_token,
            refresh_duration_minutes * 60
        );
        builder = builder.header("set-cookie", &refresh_cookie);
    }

    builder.body(new_body).build()
}

/// Handle non-signed response: use headers (backward compatibility)
fn handle_non_signed_response_with_tokens(
    response: Response,
    renewed_tokens: RenewedTokens,
) -> Response {
    let mut binding = Response::builder();
    let mut builder = binding.status(*response.status());

    // Copy existing headers
    for (name, value) in response.headers() {
        builder = builder.header(name, value.as_str().unwrap_or(""));
    }

    // Add access token headers (legacy method)
    let expires_in_str = renewed_tokens.expires_in.to_string();
    builder = builder
        .header("x-new-access-token", &renewed_tokens.access_token)
        .header("x-token-expires-in", &expires_in_str);

    // Set new refresh token cookie if provided (2/3 system logic)
    if !renewed_tokens.refresh_token.is_empty() {
        let refresh_duration_minutes = get_refresh_token_duration_minutes()
            .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");
        let refresh_cookie = format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
            renewed_tokens.refresh_token,
            refresh_duration_minutes * 60
        );
        builder = builder.header("set-cookie", &refresh_cookie);
    }

    // Create response with original body
    let body_vec = response.body().to_vec();
    builder.body(body_vec).build()
}
