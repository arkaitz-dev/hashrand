//! Magic Link Authentication Response Builder - HTTP Response Construction
//!
//! Single Responsibility: Handle HTTP response construction for magic link validation
//! Part of magic_link_val.rs refactorization to apply SOLID principles

use spin_sdk::http::Response;

use super::magic_link_jwt_generator::JwtTokens;
use crate::types::responses::JwtAuthResponse;
use crate::utils::{CryptoMaterial, create_signed_endpoint_response, create_error_response};

/// Build successful authentication response with JWT tokens and secure cookies (SignedResponse format)
///
/// # Arguments
/// * `jwt_tokens` - Generated JWT access and refresh tokens
/// * `next_param` - Optional next parameter for post-auth redirect
/// * `user_id_bytes` - Raw user ID bytes for crypto material
/// * `pub_key_bytes` - Raw public key bytes for crypto material
///
/// # Returns
/// * `anyhow::Result<Response>` - Complete HTTP response with SignedResponse format and cookies
pub fn build_authentication_response(
    jwt_tokens: JwtTokens,
    next_param: Option<String>,
    user_id_bytes: &[u8],
    pub_key_bytes: &[u8],
) -> anyhow::Result<Response> {
    // Calculate token expiration time
    let expires_in = jwt_tokens.access_expires.timestamp() - chrono::Utc::now().timestamp();

    // Create JWT response payload using new structure
    let payload = JwtAuthResponse::new(
        jwt_tokens.access_token,
        expires_in,
        jwt_tokens.username,
        next_param.clone(),
    );

    // Build crypto material for SignedResponse generation
    let crypto_material = CryptoMaterial {
        user_id: user_id_bytes.to_vec(),
        pub_key_hex: hex::encode(pub_key_bytes),
    };

    // Create signed response using DRY helper (same pattern as all other endpoints)
    let signed_response = match create_signed_endpoint_response(payload, &crypto_material) {
        Ok(response) => response,
        Err(e) => return Ok(create_error_response(500, &format!("Failed to create signed response: {}", e))),
    };

    // Add secure HttpOnly refresh token cookie to signed response
    let cookie_value = create_secure_refresh_cookie(&jwt_tokens.refresh_token)?;

    let response_with_cookie = Response::builder()
        .status(*signed_response.status())
        .header("content-type", "application/json")
        .header("set-cookie", cookie_value)
        .body(signed_response.body().to_owned())
        .build();

    if let Some(next) = next_param {
        println!("✅ Added next parameter to SignedResponse: {}", next);
    }
    println!("✅ SignedResponse authentication response built successfully");

    Ok(response_with_cookie)
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