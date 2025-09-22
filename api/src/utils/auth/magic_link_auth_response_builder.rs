//! Magic Link Authentication Response Builder - HTTP Response Construction
//!
//! Single Responsibility: Handle HTTP response construction for magic link validation
//! Part of magic_link_val.rs refactorization to apply SOLID principles

use spin_sdk::http::Response;

use super::magic_link_jwt_generator::JwtTokens;

/// Build successful authentication response with JWT tokens and secure cookies
///
/// # Arguments
/// * `jwt_tokens` - Generated JWT access and refresh tokens
/// * `next_param` - Optional next parameter for post-auth redirect
///
/// # Returns
/// * `anyhow::Result<Response>` - Complete HTTP response with JSON body and cookies
pub fn build_authentication_response(
    jwt_tokens: JwtTokens,
    next_param: Option<String>,
) -> anyhow::Result<Response> {
    // Create JSON response body with access token and user info
    let mut auth_response = serde_json::json!({
        "access_token": jwt_tokens.access_token,
        "token_type": "Bearer",
        "expires_in": (jwt_tokens.access_expires.timestamp() - chrono::Utc::now().timestamp()),
        "user_id": jwt_tokens.username
    });

    // Add next parameter if present for post-auth redirect
    if let Some(next) = next_param {
        auth_response["next"] = serde_json::Value::String(next.clone());
        println!("✅ Added next parameter to response: {}", next);
    }

    // Create secure HttpOnly refresh token cookie
    let cookie_value = create_secure_refresh_cookie(&jwt_tokens.refresh_token)?;

    // Build complete HTTP response
    let response = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("set-cookie", cookie_value)
        .body(auth_response.to_string())
        .build();

    println!("✅ Authentication response built successfully");

    Ok(response)
}

/// Create secure HttpOnly refresh token cookie with proper security attributes
fn create_secure_refresh_cookie(refresh_token: &str) -> anyhow::Result<String> {
    // Get refresh token duration from configuration
    let refresh_duration_minutes = crate::utils::jwt::config::get_refresh_token_duration_minutes()
        .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");

    // Create cookie with security attributes:
    // - HttpOnly: Prevents JavaScript access (XSS protection)
    // - Secure: HTTPS only (when deployed)
    // - SameSite=Strict: CSRF protection
    // - Max-Age: Controlled expiration
    // - Path=/: Available for all routes
    let cookie_value = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
        refresh_token,
        refresh_duration_minutes * 60 // Convert minutes to seconds
    );

    println!("✅ Secure refresh cookie created with {} minute duration", refresh_duration_minutes);

    Ok(cookie_value)
}