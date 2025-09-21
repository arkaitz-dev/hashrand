//! JWT middleware renewal logic - Proactive token renewal and 2/3 system

use crate::utils::JwtUtils;
use crate::utils::jwt::config::get_refresh_token_duration_minutes;
use chrono::DateTime;
use spin_sdk::http::Response;

use super::jwt_middleware_types::RenewedTokens;
use super::jwt_middleware_errors::create_auth_error_response;

/// Check if proactive token renewal is needed based on 2/3 threshold
///
/// # Arguments
/// * `username` - User identifier for token generation
/// * `refresh_expires_at` - Refresh token expiration timestamp
/// * `now` - Current timestamp
///
/// # Returns
/// * `Result<Option<RenewedTokens>, Response>` - New tokens if renewal needed, None otherwise
pub fn check_proactive_renewal(
    username: &str,
    refresh_expires_at: i64,
    now: i64,
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
        }))
    } else {
        // No renewal needed
        Ok(None)
    }
}

/// Add renewed tokens to HTTP response headers and cookies
///
/// # Arguments
/// * `response` - Original response from handler
/// * `renewed_tokens` - New access and refresh tokens to include
///
/// # Returns
/// * `Response` - Response with added token headers and cookies
pub fn add_renewed_tokens_to_response(response: Response, renewed_tokens: RenewedTokens) -> Response {
    // Build new response with original data
    let mut binding = Response::builder();
    let mut builder = binding.status(*response.status());

    // Copy existing headers
    for (name, value) in response.headers() {
        builder = builder.header(name, value.as_str().unwrap_or(""));
    }

    // Add new access token headers
    let expires_in_str = renewed_tokens.expires_in.to_string();
    builder = builder
        .header("x-new-access-token", &renewed_tokens.access_token)
        .header("x-token-expires-in", &expires_in_str);

    // Set new refresh token cookie ONLY if provided (2/3 system logic)
    if !renewed_tokens.refresh_token.is_empty() {
        // println!("🔍 DEBUG: Setting NEW refresh token cookie (2/3 system reset)");
        let refresh_duration_minutes = get_refresh_token_duration_minutes()
            .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");
        let refresh_cookie = format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
            renewed_tokens.refresh_token,
            refresh_duration_minutes * 60
        );
        builder = builder.header("set-cookie", &refresh_cookie);
    } else {
        // println!("🔍 DEBUG: Keeping EXISTING refresh token cookie (within 2/3 threshold)");
        // Empty refresh_token = keep existing cookie (within first 2/3 of refresh token lifetime)
    }

    // Create response with original body
    let body_vec = response.body().to_vec();
    builder.body(body_vec).build()
}