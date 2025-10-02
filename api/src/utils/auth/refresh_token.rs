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

    // Extract hostname from Host header for cookie Domain attribute
    let domain = req
        .header("host")
        .and_then(|h| h.as_str())
        .and_then(extract_hostname_from_host_header);

    if domain.is_none() {
        println!("⚠️ [SECURITY] No valid Host header - cookie will not have Domain attribute");
    }

    // Extract refresh token from cookies
    let refresh_token = match req.header("cookie") {
        Some(cookie_header) => {
            let cookie_str = cookie_header.as_str().unwrap_or("");
            extract_refresh_token_from_cookies(cookie_str)
        }
        None => None
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
    let claims = match JwtUtils::validate_refresh_token(&refresh_token) {
        Ok(claims) => claims,
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

    let username = &claims.sub;
    let pub_key = &claims.pub_key;
    let pub_key_hex = hex::encode(pub_key);

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

    if is_in_renewal_window {
        // ===== TRAMO 2/3: KEY ROTATION =====

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
                Ok((token, exp)) => (token, exp),
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
            match create_custom_refresh_token_from_username(username, &new_pub_key_array) {
                Ok((token, exp)) => (token, exp),
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
        // SECURITY: Sign with OLD pub_key but include NEW server_pub_key in payload
        let new_pub_key_hex = hex::encode(new_pub_key_array);
        let signed_response = match SignedResponseGenerator::create_signed_response_with_rotation(
            payload,
            &user_id,
            &pub_key_hex,     // ✅ OLD: derive signing key (MITM protection)
            &new_pub_key_hex, // ✅ NEW: derive server_pub_key for payload (rotation)
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

        // Create cookie with Domain attribute if available
        let cookie_value = if let Some(ref domain_str) = domain {
            format!(
                "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Domain={}; Path=/",
                new_refresh_token,
                refresh_duration_minutes * 60,
                domain_str
            )
        } else {
            println!("⚠️ [COMPAT] Creating refresh cookie WITHOUT Domain attribute");
            format!(
                "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
                new_refresh_token,
                refresh_duration_minutes * 60
            )
        };

        // 🍪 CRITICAL FIX: Delete OLD cookie explicitly before creating NEW one
        // Prevents duplicate cookies (OLD + NEW) in browser after key rotation
        // IMPORTANT: Delete cookie MUST have EXACT same Domain/Path as original cookie (RFC 6265)
        let delete_old_cookie = if let Some(ref domain_str) = domain {
            format!(
                "refresh_token=; Max-Age=0; HttpOnly; Secure; SameSite=Strict; Domain={}; Path=/",
                domain_str
            )
        } else {
            "refresh_token=; Max-Age=0; HttpOnly; Secure; SameSite=Strict; Path=/".to_string()
        };

        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .header("set-cookie", &delete_old_cookie)  // ✅ Delete OLD cookie first (exact match)
            .header("set-cookie", &cookie_value)        // ✅ Create NEW cookie second
            .body(response_json)
            .build())
    } else {
        // ===== TRAMO 1/3: NO KEY ROTATION =====
        let (access_token, _) = match JwtUtils::create_access_token_from_username(username, pub_key)
        {
            Ok((token, exp)) => (token, exp),
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

        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(response_json)
            .build())
    }
}

/// Extract refresh_token value from cookie header string
///
/// CRITICAL FIX (v1.6.33): Extract LAST occurrence instead of FIRST
/// When browser sends duplicate cookies after key rotation (OLD + NEW),
/// the LAST cookie is always the most recent one (NEW) after Set-Cookie.
/// This makes the system robust even if cookie deletion doesn't work perfectly.
fn extract_refresh_token_from_cookies(cookie_header: &str) -> Option<String> {
    let mut last_token: Option<String> = None;

    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(stripped) = cookie.strip_prefix("refresh_token=") {
            last_token = Some(stripped.to_string());  // ← Keep updating to get LAST
        }
    }

    last_token
}

/// Extract hostname from Host header for cookie Domain attribute
///
/// SECURITY: Extracts only hostname (no port, no protocol) for use as cookie Domain
///
/// # Arguments
/// * `host_header` - The Host header value (e.g., "localhost:5173" or "app.example.com")
///
/// # Returns
/// * `Option<String>` - The hostname without port, or None if invalid
fn extract_hostname_from_host_header(host_header: &str) -> Option<String> {
    // Remove port if present (split by ':' and take first part)
    let hostname = host_header.split(':').next()?.trim();

    // Validate that it's a reasonable hostname (basic validation)
    if hostname.is_empty() || hostname.contains('/') || hostname.contains('@') {
        println!(
            "⚠️ [SECURITY] Invalid Host header format: '{}'",
            host_header
        );
        return None;
    }

    println!(
        "🔒 [SECURITY] Extracted hostname for cookie Domain: '{}'",
        hostname
    );
    Some(hostname.to_string())
}
