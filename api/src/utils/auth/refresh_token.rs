//! Refresh token business logic

use spin_sdk::http::{Method, Request, Response};

use super::types::{ErrorResponse, RefreshPayload, RefreshSignedRequest};
use crate::types::responses::JwtAuthResponse;
use crate::utils::signed_response::SignedResponseGenerator;
use crate::utils::{JwtUtils, SignedRequestValidator};

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
    let pub_key_hex = hex::encode(pub_key);
    println!(
        "🔑 Refresh: OLD pub_key from JWT: {}...",
        &pub_key_hex[..16.min(pub_key_hex.len())]
    );

    // Parse and validate SignedRequest from body
    let body_bytes = req.body();
    let signed_request: RefreshSignedRequest = match serde_json::from_slice(body_bytes) {
        Ok(req) => req,
        Err(e) => {
            println!("❌ Refresh: Failed to parse SignedRequest: {}", e);
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Invalid SignedRequest structure".to_string(),
                })?)
                .build());
        }
    };

    // Validate Ed25519 signature using pub_key from refresh token JWT
    println!("🔍 Refresh: Validating Ed25519 signature...");
    if let Err(e) = SignedRequestValidator::validate_base64_payload(
        &signed_request.payload,
        &signed_request.signature,
        &pub_key_hex,
    ) {
        println!("❌ Refresh: Signature validation failed: {}", e);
        return Ok(Response::builder()
            .status(401)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: format!("Invalid signature: {}", e),
            })?)
            .build());
    }

    // Deserialize payload to extract new_pub_key
    let refresh_payload: RefreshPayload =
        match SignedRequestValidator::deserialize_base64_payload(&signed_request.payload) {
            Ok(payload) => payload,
            Err(e) => {
                println!("❌ Refresh: Failed to deserialize payload: {}", e);
                return Ok(Response::builder()
                    .status(400)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Invalid payload format".to_string(),
                    })?)
                    .build());
            }
        };

    println!(
        "✅ Refresh: SignedRequest validated, new_pub_key received: {}",
        &refresh_payload.new_pub_key[..16.min(refresh_payload.new_pub_key.len())]
    );

    // Calculate if we're in 2/3 renewal window (same logic as jwt_middleware_renewal)
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System clock error")
        .as_secs() as i64;

    let time_remaining = claims.exp - now;
    let refresh_duration_minutes = crate::utils::jwt::config::get_refresh_token_duration_minutes()
        .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");
    let refresh_duration_seconds = refresh_duration_minutes * 60;
    let two_thirds_threshold = (refresh_duration_seconds * 2) / 3;
    let is_in_renewal_window = time_remaining < two_thirds_threshold as i64;

    println!("⏱️ Refresh: Expires at: {}, Now: {}", claims.exp, now);
    println!(
        "📊 Refresh: Time remaining: {}s, 2/3 threshold: {}s",
        time_remaining, two_thirds_threshold
    );
    println!(
        "🎯 Refresh: Decision -> {}",
        if is_in_renewal_window {
            "TRAMO 2/3 (KEY ROTATION)"
        } else {
            "TRAMO 1/3 (NO ROTATION)"
        }
    );

    if is_in_renewal_window {
        // ===== TRAMO 2/3: KEY ROTATION =====
        println!("🔄 Refresh: ===== TRAMO 2/3: KEY ROTATION =====");
        println!(
            "🔑 Refresh: NEW pub_key: {}...",
            &refresh_payload.new_pub_key[..16.min(refresh_payload.new_pub_key.len())]
        );

        // Decode new_pub_key from hex
        let new_pub_key_bytes = match hex::decode(&refresh_payload.new_pub_key) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("❌ Refresh: Invalid new_pub_key hex: {}", e);
                return Ok(Response::builder()
                    .status(400)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Invalid new_pub_key format".to_string(),
                    })?)
                    .build());
            }
        };

        let new_pub_key_array: [u8; 32] = match new_pub_key_bytes.try_into() {
            Ok(arr) => arr,
            Err(_) => {
                println!("❌ Refresh: new_pub_key must be 32 bytes");
                return Ok(Response::builder()
                    .status(400)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "new_pub_key must be 32 bytes".to_string(),
                    })?)
                    .build());
            }
        };

        // Create access_token with NEW pub_key
        let (access_token, _) =
            match JwtUtils::create_access_token_from_username(username, &new_pub_key_array) {
                Ok((token, exp)) => {
                    println!("✅ Refresh: Access token created with NEW pub_key");
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

        // Create refresh_token with NEW pub_key
        use crate::utils::jwt::custom_token_api::create_custom_refresh_token_from_username;
        let (new_refresh_token, _) =
            match create_custom_refresh_token_from_username(username, Some(&new_pub_key_array)) {
                Ok((token, exp)) => {
                    println!("✅ Refresh: Refresh token created with NEW pub_key");
                    (token, exp)
                }
                Err(e) => {
                    println!("❌ Refresh: Failed to create refresh token: {}", e);
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: format!("Failed to create refresh token: {}", e),
                        })?)
                        .build());
                }
            };

        // Calculate expires_at for new refresh cookie
        let expires_at = now + (refresh_duration_minutes as i64 * 60);

        // Create user_id bytes for signed response
        let user_id = match bs58::decode(username).into_vec() {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("❌ Refresh: Failed to decode username: {}", e);
                return Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Invalid username format".to_string(),
                    })?)
                    .build());
            }
        };

        // Create payload with expires_at
        let payload = JwtAuthResponse::new(
            access_token,
            username.to_string(),
            None,
            Some(expires_at),
            None, // server_pub_key will be added by create_signed_response_with_server_pubkey
        );

        // Generate signed response WITH server_pub_key (key rotation)
        let new_pub_key_hex = hex::encode(&new_pub_key_array);
        println!(
            "🔐 Refresh: Generating SignedResponse WITH server_pub_key for rotation"
        );
        let signed_response =
            match SignedResponseGenerator::create_signed_response_with_server_pubkey(
                payload,
                &user_id,
                &new_pub_key_hex,
            ) {
                Ok(response) => response,
                Err(e) => {
                    println!("❌ CRITICAL: Cannot create signed response: {}", e);
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Cryptographic signature failure".to_string(),
                        })?)
                        .build());
                }
            };

        // Build response with new refresh cookie
        let response_json = match serde_json::to_string(&signed_response) {
            Ok(json) => json,
            Err(e) => {
                println!("❌ Refresh: Failed to serialize response: {}", e);
                return Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Response serialization failed".to_string(),
                    })?)
                    .build());
            }
        };

        let cookie_value = format!(
            "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
            new_refresh_token,
            refresh_duration_minutes * 60
        );

        println!(
            "🎉 Refresh: Key rotation completed successfully for user: {}",
            username
        );

        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .header("set-cookie", cookie_value)
            .body(response_json)
            .build())
    } else {
        // ===== TRAMO 1/3: NO KEY ROTATION =====
        println!("⏸️ Refresh: ===== TRAMO 1/3: NO KEY ROTATION =====");
        println!(
            "🔑 Refresh: Using OLD pub_key: {}...",
            &pub_key_hex[..16.min(pub_key_hex.len())]
        );

        // Create access_token with OLD pub_key
        let (access_token, _) = match JwtUtils::create_access_token_from_username(username, pub_key)
        {
            Ok((token, exp)) => {
                println!("✅ Refresh: Access token created with OLD pub_key");
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

        // Create user_id bytes for signed response
        let user_id = match bs58::decode(username).into_vec() {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("❌ Refresh: Failed to decode username: {}", e);
                return Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Invalid username format".to_string(),
                    })?)
                    .build());
            }
        };

        // Create payload WITHOUT expires_at (no new refresh cookie)
        let payload = JwtAuthResponse::new(
            access_token,
            username.to_string(),
            None,
            None, // No expires_at - no new refresh cookie
            None, // No server_pub_key - no key rotation
        );

        // Generate signed response WITHOUT server_pub_key (no key rotation)
        println!(
            "🔐 Refresh: Generating SignedResponse WITHOUT server_pub_key (no rotation)"
        );
        let signed_response = match SignedResponseGenerator::create_signed_response(
            payload,
            &user_id,
            &pub_key_hex,
        ) {
            Ok(response) => response,
            Err(e) => {
                println!("❌ CRITICAL: Cannot create signed response: {}", e);
                return Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Cryptographic signature failure".to_string(),
                    })?)
                    .build());
            }
        };

        // Build response WITHOUT refresh cookie
        let response_json = match serde_json::to_string(&signed_response) {
            Ok(json) => json,
            Err(e) => {
                println!("❌ Refresh: Failed to serialize response: {}", e);
                return Ok(Response::builder()
                    .status(500)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Response serialization failed".to_string(),
                    })?)
                    .build());
            }
        };

        println!(
            "✅ Refresh: Token refresh completed (no rotation) for user: {}",
            username
        );

        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(response_json)
            .build())
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
