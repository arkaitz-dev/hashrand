//! JWT Authentication middleware for protected endpoints
//!
//! Provides middleware functions to validate Bearer tokens and protect
//! endpoints that require authentication.

use crate::utils::JwtUtils;
use crate::utils::jwt::config::{get_refresh_token_duration_minutes};
use chrono::{DateTime, Utc};
use serde::Serialize;
use spin_sdk::http::{Request, Response};

/// Error response structure for authentication failures
#[derive(Serialize)]
struct AuthErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_in: Option<String>,
}

/// Authentication result with user information
#[allow(dead_code)]
pub struct AuthContext {
    pub username: String,
    pub expires_at: i64,
    pub refresh_expires_at: i64,
    /// New tokens generated due to proactive renewal (if any)
    pub renewed_tokens: Option<RenewedTokens>,
}

/// Renewed tokens for proactive refresh
#[derive(Debug)]
pub struct RenewedTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// Extract and validate Bearer token from Authorization header
///
/// # Arguments
/// * `req` - HTTP request to check for Authorization header
///
/// # Returns
/// * `Result<AuthContext, Response>` - Either valid auth context or error response
pub fn validate_bearer_token(req: &Request) -> Result<AuthContext, Response> {
    // Extract Authorization header
    let auth_header = match req.header("authorization") {
        Some(header) => header.as_str().unwrap_or(""),
        None => {
            return Err(create_auth_error_response(
                "Missing Authorization header. Include 'Authorization: Bearer <token>'",
                None,
            ));
        }
    };

    // Check Bearer token format
    if !auth_header.starts_with("Bearer ") {
        return Err(create_auth_error_response(
            "Invalid Authorization header format. Use 'Bearer <token>'",
            None,
        ));
    }

    // Extract token (skip "Bearer " prefix)
    let token = &auth_header[7..];
    if token.is_empty() {
        return Err(create_auth_error_response(
            "Empty Bearer token provided",
            None,
        ));
    }

    // Validate JWT token
    match JwtUtils::validate_access_token(token) {
        Ok(claims) => {
            let now = Utc::now().timestamp();
            let refresh_expires_at = claims.refresh_expires_at;

            // Check if we need proactive renewal (2/3 threshold)
            let renewed_tokens = check_proactive_renewal(&claims.sub, refresh_expires_at, now)?;

            Ok(AuthContext {
                username: claims.sub,
                expires_at: claims.exp,
                refresh_expires_at,
                renewed_tokens,
            })
        },
        Err(error_msg) => {
            println!("🔍 DEBUG: Token validation failed: {}", error_msg);

            // If token is expired, try to refresh using cookies (2/3 system)
            if error_msg.contains("expired") || error_msg.contains("exp") {
                println!("🔍 DEBUG: Token expired, attempting refresh from cookies...");

                // Try to extract refresh token from cookies and validate
                if let Some(cookie_header) = req.header("cookie") {
                    println!("🔍 DEBUG: Cookie header found");
                    if let Some(cookie_str) = cookie_header.as_str() {
                        // Extract refresh token from cookies (same logic as refresh endpoint)
                        let refresh_token_option = extract_refresh_token_from_cookies(cookie_str);
                        println!("🔍 DEBUG: Cookie string: {}", cookie_str);

                        if let Some(refresh_token) = refresh_token_option {
                            println!("🔍 DEBUG: Refresh token extracted: {}...", &refresh_token[..20.min(refresh_token.len())]);

                            // Validate refresh token
                            match JwtUtils::validate_refresh_token(&refresh_token) {
                                Ok(refresh_claims) => {
                                    println!("🔍 DEBUG: Refresh token validated successfully for user: {}", refresh_claims.sub);

                                    // SISTEMA 2/3: Calcular tiempo transcurrido desde creación del refresh token
                                    let now = Utc::now();
                                    let refresh_duration_minutes = match crate::utils::jwt::config::get_refresh_token_duration_minutes() {
                                        Ok(duration) => duration as i64,
                                        Err(_) => {
                                            println!("🔍 DEBUG: Error getting refresh token duration from .env, using default");
                                            9 // Default fallback only if .env fails
                                        }
                                    };
                                    let refresh_expires_at = match DateTime::from_timestamp(refresh_claims.exp, 0) {
                                        Some(dt) => dt,
                                        None => {
                                            println!("🔍 DEBUG: Invalid refresh token expiration timestamp, failing auth");
                                            return Err(create_auth_error_response("Invalid token timestamp", None));
                                        }
                                    };
                                    let refresh_created_at = refresh_expires_at - chrono::Duration::minutes(refresh_duration_minutes);
                                    let time_elapsed_duration = now - refresh_created_at;
                                    let one_third_threshold = chrono::Duration::minutes(refresh_duration_minutes / 3); // 1/3 transcurrido = 2/3 restante

                                    // println!("🔍 DEBUG 2/3 System: time_elapsed={:.0}min, 1/3_threshold={:.0}min ({}2/3 remaining)",
                                    //          time_elapsed_duration.num_minutes(), one_third_threshold.num_minutes(),
                                    //          if time_elapsed_duration > one_third_threshold { "✅ Activate: " } else { "⏳ Wait: " });

                                    // Create new access token (always)
                                    if let Ok((new_access_token, access_expires)) = JwtUtils::create_access_token(&refresh_claims.sub) {
                                        println!("🔍 DEBUG: New access token created: {}...", &new_access_token[..20.min(new_access_token.len())]);

                                        let now_timestamp = now.timestamp();
                                        let expires_in = access_expires.timestamp() - now_timestamp;

                                        // Check if we need to create new refresh token (2/3 system)
                                        if time_elapsed_duration > one_third_threshold {
                                            // println!("🔍 DEBUG 2/3 System: Beyond 1/3 elapsed (2/3 remaining), creating NEW refresh token (reset)");
                                            // Beyond 1/3 elapsed (2/3 remaining): Create new refresh token (reset the timer)
                                            if let Ok((new_refresh_token, _refresh_expires)) = JwtUtils::create_refresh_token_from_username(&refresh_claims.sub, None) {
                                                let renewed_tokens = Some(RenewedTokens {
                                                    access_token: new_access_token,
                                                    refresh_token: new_refresh_token,
                                                    expires_in,
                                                });

                                                return Ok(AuthContext {
                                                    username: refresh_claims.sub,
                                                    expires_at: access_expires.timestamp(),
                                                    refresh_expires_at: refresh_claims.exp,
                                                    renewed_tokens,
                                                });
                                            } else {
                                                println!("🔍 DEBUG: Failed to create new refresh token");
                                            }
                                        } else {
                                            // println!("🔍 DEBUG 2/3 System: Within first 1/3 (more than 2/3 remaining), keeping EXISTING refresh token");
                                            // Within first 1/3 elapsed (more than 2/3 remaining): Keep existing refresh token, only renew access token
                                            let renewed_tokens = Some(RenewedTokens {
                                                access_token: new_access_token,
                                                refresh_token: String::new(), // Empty = keep existing cookie
                                                expires_in,
                                            });

                                            return Ok(AuthContext {
                                                username: refresh_claims.sub,
                                                expires_at: access_expires.timestamp(),
                                                refresh_expires_at: refresh_claims.exp,
                                                renewed_tokens,
                                            });
                                        }
                                    } else {
                                        println!("🔍 DEBUG: Failed to create new access token");
                                    }
                                },
                                Err(validation_error) => {
                                    println!("🔍 DEBUG: Refresh token validation failed: {}", validation_error);
                                }
                            }
                        } else {
                            println!("🔍 DEBUG: No refresh token found in cookies");
                        }
                    }
                }
            }

            // Check if token is expired for specific error message
            let (error, expires_hint) =
                if error_msg.contains("expired") || error_msg.contains("exp") {
                    (
                        "Access token has expired. Use refresh token to obtain a new access token",
                        Some("20 seconds from issuance".to_string()),
                    )
                } else {
                    ("Invalid access token. Please authenticate again", None)
                };

            Err(create_auth_error_response(error, expires_hint))
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

/// Create standardized authentication error response
fn create_auth_error_response(error: &str, expires_in: Option<String>) -> Response {
    let response = AuthErrorResponse {
        error: error.to_string(),
        expires_in,
    };

    Response::builder()
        .status(401)
        .header("content-type", "application/json")
        .header("www-authenticate", "Bearer")
        .body(
            serde_json::to_string(&response)
                .unwrap_or_else(|_| r#"{"error":"Authentication required"}"#.to_string()),
        )
        .build()
}

/// Check if proactive token renewal is needed based on 2/3 threshold
///
/// # Arguments
/// * `username` - User identifier for token generation
/// * `refresh_expires_at` - Refresh token expiration timestamp
/// * `now` - Current timestamp
///
/// # Returns
/// * `Result<Option<RenewedTokens>, Response>` - New tokens if renewal needed, None otherwise
fn check_proactive_renewal(
    username: &str,
    refresh_expires_at: i64,
    now: i64
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
        println!("Proactive renewal triggered: {}s remaining < {}s threshold",
                time_remaining, two_thirds_threshold);

        // Generate new access token
        let (new_access_token, access_expires) = match JwtUtils::create_access_token(username) {
            Ok((token, exp)) => (token, exp),
            Err(e) => {
                println!("Failed to create new access token during proactive renewal: {}", e);
                return Err(create_auth_error_response(
                    "Failed to renew access token",
                    None,
                ));
            }
        };

        // Generate new refresh token
        let (new_refresh_token, _refresh_expires) = match JwtUtils::create_refresh_token_from_username(username, None) {
            Ok((token, exp)) => (token, exp),
            Err(e) => {
                println!("Failed to create new refresh token during proactive renewal: {}", e);
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

/// Check if endpoint requires authentication
///
/// # Arguments
/// * `path` - URL path to check
///
/// # Returns
/// * `bool` - true if endpoint requires authentication
pub fn requires_authentication(path: &str) -> bool {
    match path {
        // Public endpoints (no authentication required)
        p if p.ends_with("/api/version") => false,
        p if p.starts_with("/api/login") => false,
        p if p.ends_with("/api/refresh") => false,

        // Protected endpoints (authentication required)
        p if p.ends_with("/api/custom") => true,
        p if p.ends_with("/api/generate") => true,
        p if p.ends_with("/api/password") => true,
        p if p.ends_with("/api/api-key") => true,
        p if p.ends_with("/api/mnemonic") => true,
        p if p.ends_with("/api/from-seed") => true,
        p if p.starts_with("/api/users") => true,

        // Default: require authentication for unknown endpoints
        _ => true,
    }
}

/// Enhanced wrapper for protected handlers with proactive token renewal
pub fn with_auth_and_renewal<F>(req: Request, handler: F) -> anyhow::Result<Response>
where
    F: FnOnce(Request) -> anyhow::Result<Response>,
{
    // Validate token and get auth context (with potential renewed tokens)
    let auth_context = match validate_bearer_token(&req) {
        Ok(context) => context,
        Err(auth_error) => return Ok(auth_error),
    };

    // Call the original handler
    let mut response = handler(req)?;

    // If we have renewed tokens, add them to the response
    if let Some(renewed_tokens) = auth_context.renewed_tokens {
        response = add_renewed_tokens_to_response(response, renewed_tokens);
    }

    Ok(response)
}

/// Add renewed tokens to HTTP response headers and cookies
///
/// # Arguments
/// * `response` - Original response from handler
/// * `renewed_tokens` - New access and refresh tokens to include
///
/// # Returns
/// * `Response` - Response with added token headers and cookies
fn add_renewed_tokens_to_response(response: Response, renewed_tokens: RenewedTokens) -> Response {
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
        let refresh_duration_minutes = get_refresh_token_duration_minutes().unwrap_or(9);
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

/// Wrapper for protected handlers (legacy - kept for compatibility)
#[allow(dead_code)]
pub fn with_auth<F>(req: Request, handler: F) -> anyhow::Result<Response>
where
    F: FnOnce(Request, AuthContext) -> anyhow::Result<Response>,
{
    match validate_bearer_token(&req) {
        Ok(auth_context) => handler(req, auth_context),
        Err(auth_error) => Ok(auth_error),
    }
}
