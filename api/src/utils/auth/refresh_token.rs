//! Refresh token business logic

use spin_sdk::http::{Method, Request, Response};

use super::types::ErrorResponse;
use crate::utils::JwtUtils;
use crate::utils::signed_response::SignedResponseGenerator;
use serde_json::json;

/// Handle refresh token request and generate new access token
///
/// This function handles the business logic for token refresh:
/// - Extracts refresh token from HttpOnly cookies
/// - Validates the refresh token
/// - Generates new access token with updated expiration
pub async fn handle_refresh_token(req: Request) -> anyhow::Result<Response> {
    // Only allow POST method
    if *req.method() != Method::Post {
        return Ok(Response::builder()
            .status(405)
            .header("allow", "POST")
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Method not allowed".to_string(),
            })?)
            .build());
    }

    // Extract refresh token from cookies
    let refresh_token = match req.header("cookie") {
        Some(cookie_header) => {
            let cookie_str = cookie_header.as_str().unwrap_or("");
            println!("🍪 Refresh: Cookie header received: '{}'", cookie_str);
            let token = extract_refresh_token_from_cookies(cookie_str);
            if token.is_some() {
                println!("✅ Refresh: Successfully extracted refresh token");
            } else {
                println!("❌ Refresh: Failed to extract refresh token from cookies");
            }
            token
        }
        None => {
            println!("❌ Refresh: No cookie header found in request");
            None
        }
    };

    let refresh_token = match refresh_token {
        Some(token) => token,
        None => {
            return Ok(Response::builder()
                .status(401)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Refresh token not found".to_string(),
                })?)
                .build());
        }
    };

    // Validate refresh token
    println!("🔍 Refresh: Attempting to validate refresh token...");
    let claims = match JwtUtils::validate_refresh_token(&refresh_token) {
        Ok(claims) => {
            println!(
                "✅ Refresh: Token validation successful, user: {}",
                claims.sub
            );
            claims
        }
        Err(e) => {
            println!("❌ Refresh: Token validation failed: {}", e);
            return Ok(Response::builder()
                .status(401)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: format!("Invalid refresh token: {}", e),
                })?)
                .build());
        }
    };

    // Create new access token using the user_id from refresh token claims
    let username = &claims.sub;

    // Convert Base58 username back to email for access token creation
    // For simplicity, we'll use the username directly since access tokens use username as subject
    println!(
        "🎫 Refresh: Creating new access token for user: {}",
        username
    );
    // Extract pub_key from refresh token claims (Ed25519 public key for cryptographic operations)
    let pub_key = &claims.pub_key;
    let (access_token, expires_at) =
        match JwtUtils::create_access_token_from_username(username, pub_key) {
            Ok((token, exp)) => {
                println!("✅ Refresh: New access token created successfully");
                (token, exp)
            }
            Err(e) => {
                println!("❌ Refresh: Failed to create access token: {}", e);
                return Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: format!("Failed to create access token: {}", e),
                    })?)
                    .build());
            }
        };

    // REFRESH TOKEN ROTATION: Generate new refresh token preserving crypto noise ID
    let (new_refresh_token, _refresh_expires_at) =
        match JwtUtils::create_refresh_token_from_username(username, Some(claims.session_id)) {
            Ok((token, exp)) => (token, exp),
            Err(e) => {
                return Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: format!("Failed to create refresh token: {}", e),
                    })?)
                    .build());
            }
        };

    // Create signed response for token refresh (with server_pub_key)
    match create_signed_refresh_response(
        &access_token,
        expires_at.timestamp() - chrono::Utc::now().timestamp(),
        username,
        &new_refresh_token,
        pub_key,
    ) {
        Ok(response) => {
            println!(
                "🎉 Refresh: Token refresh completed successfully for user: {}",
                username
            );
            Ok(response)
        }
        Err(e) => {
            println!("❌ Error creating signed refresh response: {}", e);
            // Fallback to unsigned response
            let fallback_response = json!({
                "access_token": access_token,
                "expires_in": (expires_at.timestamp() - chrono::Utc::now().timestamp()),
                "user_id": username,
                "message": "Token refreshed successfully"
            });

            let refresh_duration_minutes = crate::utils::jwt::config::get_refresh_token_duration_minutes()
                .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");

            let cookie_value = format!(
                "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
                new_refresh_token,
                refresh_duration_minutes * 60
            );

            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .header("set-cookie", cookie_value)
                .body(serde_json::to_string(&fallback_response)?)
                .build())
        }
    }
}

/// Extract refresh_token value from cookie header string
fn extract_refresh_token_from_cookies(cookie_header: &str) -> Option<String> {
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(stripped) = cookie.strip_prefix("refresh_token=") {
            return Some(stripped.to_string());
        }
    }
    None
}

/// Create signed response for token refresh (with server_pub_key)
///
/// Generates a signed response using Ed25519 per-session keypair derived from
/// user_id + frontend_pub_key, includes server public key for verification.
///
/// # Arguments
/// * `access_token` - New access token
/// * `expires_in` - Token expiration seconds
/// * `username` - Base58 encoded user ID
/// * `new_refresh_token` - New refresh token
/// * `pub_key` - Frontend Ed25519 public key (32 bytes)
///
/// # Returns
/// * `Result<Response, String>` - Signed HTTP response with cookies or error
fn create_signed_refresh_response(
    access_token: &str,
    expires_in: i64,
    username: &str,
    new_refresh_token: &str,
    pub_key: &[u8; 32],
) -> Result<Response, String> {
    // Step 1: Convert Base58 username back to user_id bytes
    let user_id = bs58::decode(username)
        .into_vec()
        .map_err(|e| format!("Failed to decode Base58 username: {}", e))?;

    // Step 2: Convert pub_key bytes to hex string
    let pub_key_hex = hex::encode(pub_key);

    // Step 3: Create payload with token refresh data
    let payload = json!({
        "access_token": access_token,
        "expires_in": expires_in,
        "user_id": username,
        "message": "Token refreshed successfully"
    });

    // Step 4: Generate signed response with server public key
    let signed_response = SignedResponseGenerator::create_signed_response_with_server_pubkey(
        payload,
        &user_id,
        &pub_key_hex,
    )
    .map_err(|e| format!("Failed to create signed response: {}", e))?;

    // Step 5: Serialize signed response to JSON
    let response_json = serde_json::to_string(&signed_response)
        .map_err(|e| format!("Failed to serialize signed response: {}", e))?;

    // Step 6: Get refresh token duration and create cookie
    let refresh_duration_minutes = crate::utils::jwt::config::get_refresh_token_duration_minutes()
        .map_err(|e| format!("Failed to get refresh token duration: {}", e))?;

    let cookie_value = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
        new_refresh_token,
        refresh_duration_minutes * 60 // Convert minutes to seconds
    );

    // Step 7: Build HTTP response with signed payload and refresh token cookie
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("set-cookie", cookie_value)
        .body(response_json)
        .build())
}
