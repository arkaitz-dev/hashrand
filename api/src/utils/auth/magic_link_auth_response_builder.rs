//! Magic Link Authentication Response Builder - HTTP Response Construction
//!
//! Single Responsibility: Handle HTTP response construction for magic link validation
//! Part of magic_link_val.rs refactorization to apply SOLID principles

use spin_sdk::http::Response;
use tracing::{debug, warn};

use super::magic_link_jwt_generator::JwtTokens;
use crate::types::responses::JwtAuthResponse;
use crate::utils::{CryptoMaterial, create_error_response, create_signed_endpoint_response};

/// Build successful authentication response with JWT tokens and secure cookies (SignedResponse format)
///
/// # Arguments
/// * `jwt_tokens` - Generated JWT access and refresh tokens
/// * `next_param` - Optional next parameter for post-auth redirect
/// * `user_id_bytes` - Raw user ID bytes for crypto material
/// * `pub_key_bytes` - Raw public key bytes for crypto material
/// * `ui_host` - Optional UI host (domain) for cookie Domain attribute
///
/// # Returns
/// * `anyhow::Result<Response>` - Complete HTTP response with SignedResponse format and cookies
pub fn build_authentication_response(
    jwt_tokens: JwtTokens,
    next_param: Option<String>,
    user_id_bytes: &[u8],
    pub_key_bytes: &[u8],
    ui_host: Option<String>,
) -> anyhow::Result<Response> {
    // Calculate refresh cookie expiration timestamp
    let refresh_duration_minutes = crate::utils::jwt::config::get_refresh_token_duration_minutes()
        .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");

    let expires_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System clock error")
        .as_secs() as i64
        + (refresh_duration_minutes as i64 * 60);

    // Create JWT response payload with refresh cookie expiration timestamp
    debug!(
        "📍 DEBUG: Building auth response with next parameter: {:?}",
        next_param
    );
    let payload = JwtAuthResponse::new(
        jwt_tokens.access_token,
        jwt_tokens.username,
        next_param.clone(),
        Some(expires_at),
        None, // server_pub_key will be added by create_signed_response_with_server_pubkey
    );

    // Build crypto material for SignedResponse generation
    let crypto_material = CryptoMaterial {
        user_id: user_id_bytes.to_vec(),
        pub_key_hex: hex::encode(pub_key_bytes),
    };

    // Create signed response using DRY helper (same pattern as all other endpoints)
    let signed_response = match create_signed_endpoint_response(payload, &crypto_material) {
        Ok(response) => response,
        Err(e) => {
            return Ok(create_error_response(
                500,
                &format!("Failed to create signed response: {}", e),
            ));
        }
    };

    // Add secure HttpOnly refresh token cookie to signed response with Domain attribute
    let cookie_value = create_secure_refresh_cookie(&jwt_tokens.refresh_token, ui_host.as_deref())?;

    let response_with_cookie = Response::builder()
        .status(*signed_response.status())
        .header("content-type", "application/json")
        .header("set-cookie", cookie_value)
        .body(signed_response.body().to_owned())
        .build();

    Ok(response_with_cookie)
}

/// Create secure HttpOnly refresh token cookie with proper security attributes and Domain
fn create_secure_refresh_cookie(
    refresh_token: &str,
    ui_host: Option<&str>,
) -> anyhow::Result<String> {
    // Get refresh token duration from configuration
    let refresh_duration_minutes = crate::utils::jwt::config::get_refresh_token_duration_minutes()
        .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");

    // Create cookie with security attributes:
    // - HttpOnly: Prevents JavaScript access (XSS protection)
    // - Secure: HTTPS only (when deployed)
    // - SameSite=Strict: CSRF protection
    // - Max-Age: Controlled expiration
    // - Domain: Explicit domain scope (from ui_host)
    // - Path=/: Available for all routes
    let cookie_value = if let Some(domain) = ui_host {
        debug!(
            "🔒 [SECURITY] Creating refresh cookie with Domain: '{}'",
            domain
        );
        format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Domain={}; Path=/",
            refresh_token,
            refresh_duration_minutes * 60, // Convert minutes to seconds
            domain
        )
    } else {
        // Backward compatibility: No Domain attribute (old magic links without ui_host)
        warn!("⚠️ [COMPAT] Creating refresh cookie WITHOUT Domain (old format)");
        format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
            refresh_token,
            refresh_duration_minutes * 60
        )
    };

    Ok(cookie_value)
}
