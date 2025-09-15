//! Refresh token business logic

use spin_sdk::http::{Method, Request, Response};

use super::types::ErrorResponse;
use crate::utils::JwtUtils;

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
    // TODO: Extract pub_key from existing refresh token claims instead of using placeholder
    let placeholder_pub_key = [0u8; 32]; // Temporary until we implement pub_key extraction
    let (access_token, expires_at) = match JwtUtils::create_access_token_from_username(username, &placeholder_pub_key) {
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

    // Return new access token with same format as login
    let response = serde_json::json!({
        "access_token": access_token,
        "expires_in": (expires_at.timestamp() - chrono::Utc::now().timestamp()),
        "user_id": username,
        "message": "Token refreshed successfully"
    });

    // Get refresh token duration from configuration
    let refresh_duration_minutes = crate::utils::jwt::config::get_refresh_token_duration_minutes().expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");

    // Set new refresh token as HttpOnly, Secure, SameSite cookie
    let cookie_value = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
        new_refresh_token,
        refresh_duration_minutes * 60 // Convert minutes to seconds
    );

    println!(
        "🎉 Refresh: Token refresh completed successfully for user: {}",
        username
    );

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("set-cookie", cookie_value)
        .body(serde_json::to_string(&response)?)
        .build())
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
