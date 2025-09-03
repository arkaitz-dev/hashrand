//! Authentication login handler
//!
//! Handles magic link authentication flow:
//! 1. POST /api/login/ - Generate magic link and send via email (logged in development)
//! 2. GET /api/login/?magiclink=... - Validate magic link and return JWT tokens

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;

use crate::database::{
    connection::{DatabaseEnvironment, initialize_database},
    operations::MagicLinkOperations,
};
use crate::utils::{JwtUtils, send_magic_link_email};

/// Request body for magic link generation
#[derive(Deserialize)]
struct MagicLinkRequest {
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<String>, // Base58-encoded parameters for post-auth redirect
    #[serde(skip_serializing_if = "Option::is_none")]
    email_lang: Option<String>, // Language code for email template (e.g., "es", "en")
}

/// Response for magic link generation (development)
#[derive(Serialize)]
#[allow(dead_code)]
struct MagicLinkResponse {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    dev_magic_link: Option<String>,
}

/// Error response structure
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// Handle login authentication requests
///
/// # Arguments
/// * `req` - HTTP request
/// * `query_params` - Query parameters from URL
///
/// # Returns
/// * `Result<impl IntoResponse, anyhow::Error>` - HTTP response
pub async fn handle_login(
    req: Request,
    query_params: HashMap<String, String>,
) -> anyhow::Result<Response> {
    // Determine database environment
    // For now use Development since we don't have access to IncomingRequest
    let env = DatabaseEnvironment::Development;

    // Initialize database to ensure auth_sessions table exists
    println!("Initializing database...");
    if let Err(e) = initialize_database(env.clone()) {
        println!("Failed to initialize database: {}", e);
        return Ok(Response::builder()
            .status(500)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Database initialization failed".to_string(),
            })?)
            .build());
    }
    println!("Database initialized successfully");

    match *req.method() {
        Method::Post => handle_magic_link_generation(req, env).await,
        Method::Get => handle_magic_link_validation(req, query_params, env),
        Method::Delete => handle_logout(),
        _ => Ok(Response::builder()
            .status(405)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Method not allowed".to_string(),
            })?)
            .build()),
    }
}

/// Handle POST /api/login/ - Generate magic link
async fn handle_magic_link_generation(
    req: Request,
    env: DatabaseEnvironment,
) -> anyhow::Result<Response> {
    // Parse request body
    let body_bytes = req.body();
    println!(
        "DEBUG: Request body bytes: {:?}",
        std::str::from_utf8(body_bytes)
    );

    let magic_request: MagicLinkRequest =
        match serde_json::from_slice::<MagicLinkRequest>(body_bytes) {
            Ok(req) => {
                println!(
                    "DEBUG: Parsed request - Email: {}, UI Host: {:?}, Email Lang: {:?}",
                    req.email, req.ui_host, req.email_lang
                );
                req
            }
            Err(e) => {
                println!("DEBUG: JSON parse error: {}", e);
                return Ok(Response::builder()
                    .status(400)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Invalid JSON body".to_string(),
                    })?)
                    .build());
            }
        };

    // Validate email format (basic validation)
    if magic_request.email.is_empty() || !magic_request.email.contains('@') {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Invalid email address".to_string(),
            })?)
            .build());
    }

    // Generate magic token with integrity protection (15 minutes)
    let magic_expires_at = Utc::now() + Duration::minutes(15);
    let magic_token = match JwtUtils::generate_magic_token(&magic_request.email, magic_expires_at) {
        Ok(token) => token,
        Err(e) => {
            return Ok(Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(
                    serde_json::to_string(&ErrorResponse {
                        error: format!("Failed to generate magic token: {}", e),
                    })
                    .unwrap_or_default(),
                )
                .build());
        }
    };

    // No need to create AuthSession anymore - magic link contains all user info

    // Get host URL for magic link (prefer ui_host from request, fallback to request host)
    println!("DEBUG: About to choose host URL");
    println!("DEBUG: magic_request.ui_host = {:?}", magic_request.ui_host);

    let fallback_host = JwtUtils::get_host_url_from_request(&req);
    println!("DEBUG: fallback_host from request = {}", fallback_host);

    let host_url = magic_request
        .ui_host
        .as_deref() // Más limpio que .as_ref().map(|s| s.as_str())
        .unwrap_or(&fallback_host);

    println!("DEBUG: Final chosen host_url = {}", host_url);
    let magic_link = JwtUtils::create_magic_link_url(
        host_url,
        &magic_token,
        magic_request.next.as_deref(),
    );
    println!("DEBUG: Generated magic_link = {}", magic_link);

    // Store magic link hash in database
    match MagicLinkOperations::store_magic_link(env.clone(), &magic_link, magic_expires_at.timestamp() as u64) {
        Ok(_) => {

            // Try to send email via Mailtrap, fallback to console logging
            match send_magic_link_email(
                &magic_request.email,
                &magic_link,
                magic_request.email_lang.as_deref(),
            )
            .await
            {
                Ok(()) => {
                    println!("✅ Email sent successfully to: {}", magic_request.email);
                }
                Err(e) => {
                    println!(
                        "⚠️ Email sending failed, falling back to console logging: {}",
                        e
                    );

                    // Fallback: simulate email content in console (development mode)
                    println!("\n🔗 === EMAIL FALLBACK (DEVELOPMENT MODE) ===");
                    println!("📧 TO: {}", magic_request.email);
                    println!("📬 FROM: HashRand Spin <noreply@hashrand.dev>");
                    println!("📝 SUBJECT: Your Magic Link for HashRand Spin");
                    println!("📄 EMAIL BODY:");
                    println!("──────────────────────────────────────────────────");
                    println!("Hi there!");
                    println!();
                    println!(
                        "You requested access to HashRand Spin. Click the link below to sign in:"
                    );
                    println!();
                    println!("🔗 {}", magic_link);
                    println!();
                    println!(
                        "This link will expire at: {}",
                        magic_expires_at.format("%Y-%m-%d %H:%M:%S UTC")
                    );
                    println!();
                    println!("If you didn't request this, you can safely ignore this email.");
                    println!();
                    println!("Best regards,");
                    println!("The HashRand Spin Team");
                    println!("──────────────────────────────────────────────────");
                    println!("🔧 DEVELOPMENT INFO:");
                    println!("   • UI Host: {:?}", magic_request.ui_host);
                    println!("   • Final Host URL: {}", host_url);
                    println!(
                        "   • Token expires: {}",
                        magic_expires_at.format("%Y-%m-%d %H:%M:%S UTC")
                    );
                    println!("   • Email send error: {}", e);
                    println!("═══════════════════════════════════════════════════\n");
                }
            }

            // Clean up expired sessions
            let _ = MagicLinkOperations::cleanup_expired_links(env);

            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .header("access-control-allow-origin", "*")
                .header("access-control-allow-methods", "POST, GET, OPTIONS")
                .header("access-control-allow-headers", "Content-Type")
                .body("{\"status\":\"OK\"}")
                .build())
        }
        Err(e) => {
            println!("Failed to create auth session: {}", e);
            Ok(Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Failed to generate magic link".to_string(),
                })?)
                .build())
        }
    }
}

/// Handle GET /api/login/?magiclink=... - Validate magic link and return tokens
fn handle_magic_link_validation(
    _req: Request,
    query_params: HashMap<String, String>,
    env: DatabaseEnvironment,
) -> anyhow::Result<Response> {
    // Get magic link from query parameters
    let magic_token = match query_params.get("magiclink") {
        Some(token) if !token.is_empty() => token,
        _ => {
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Missing magiclink parameter".to_string(),
                })?)
                .build());
        }
    };

    println!("Magic token received: '{}'", magic_token);

    // Validate magic token integrity and extract user_id + expiration
    let (user_id_bytes, magic_expires_at) = match JwtUtils::validate_magic_token(magic_token) {
        Ok((user_id, expires)) => (user_id, expires),
        Err(error) => {
            println!("Magic token validation failed: {}", error);
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Invalid or expired magic link".to_string(),
                })?)
                .build());
        }
    };

    // Check if magic token has expired
    let now = Utc::now();
    if now > magic_expires_at {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Magic link has expired".to_string(),
            })?)
            .build());
    }

    // Convert user_id to Base58 username
    let username = JwtUtils::user_id_to_username(&user_id_bytes);

    // Search for auth session by user_id and timestamp
    let timestamp = magic_expires_at.timestamp() as u64;
    println!(
        "Searching for session: user_id={}, timestamp={}",
        username, timestamp
    );

    // Build magic link URL for validation
    let host_url = JwtUtils::get_host_url_from_request(&_req);
    let magic_link_url = JwtUtils::create_magic_link_url(
        &host_url,
        magic_token,
        None, // No next parameter during validation
    );

    // Validate and consume magic link
    match MagicLinkOperations::validate_and_consume_magic_link(env.clone(), &magic_link_url) {
        Ok(true) => {
            println!("User {} authenticated successfully", username);

            // Ensure user exists in users table
            let _ = MagicLinkOperations::ensure_user_exists(env.clone(), &user_id_bytes);

            // Generate new access and refresh tokens
            let (access_token, access_expires) = match JwtUtils::create_access_token(&username) {
                Ok((token, exp)) => (token, exp),
                Err(e) => {
                    println!("Failed to create access token: {}", e);
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Failed to create access token".to_string(),
                        })?).build());
                }
            };

            let (refresh_token, _) = match JwtUtils::create_refresh_token(&username, 0) {
                Ok((token, exp)) => (token, exp),
                Err(e) => {
                    println!("Failed to create refresh token: {}", e);
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Failed to create refresh token".to_string(),
                        })?).build());
                }
            };

            // Create response with access token, user_id, and secure refresh token cookie
            let auth_response = serde_json::json!({
                "access_token": access_token,
                "token_type": "Bearer",
                "expires_in": 180, // 3 minutes
                "user_id": username
            });

            // Set refresh token as HttpOnly, Secure, SameSite cookie
            let cookie_value = format!(
                "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
                refresh_token,
                15 * 60 // 15 minutes in seconds
            );

            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .header("set-cookie", cookie_value)
                .body(auth_response.to_string())
                .build())
        }
        Ok(false) => Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Invalid or expired magic link".to_string(),
            })?)
            .build()),
        Err(e) => {
            println!("Database error during magic link validation: {}", e);
            Ok(Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Authentication failed".to_string(),
                })?)
                .build())
        }
    }
}

/// Handle DELETE /api/login/ - Clear refresh token cookie (logout)
fn handle_logout() -> anyhow::Result<Response> {
    // Create expired cookie to clear the refresh token
    let expired_cookie = "refresh_token=; HttpOnly; Secure; SameSite=Strict; Max-Age=0; Path=/";

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("set-cookie", expired_cookie)
        .body(serde_json::to_string(&serde_json::json!({
            "message": "Logged out successfully"
        }))?)
        .build())
}

/// Handle POST /api/refresh - Refresh access token using refresh token cookie
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
            extract_refresh_token_from_cookies(cookie_str)
        }
        None => None,
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
    let (access_token, expires_at) = match JwtUtils::create_access_token_from_username(username) {
        Ok((token, exp)) => (token, exp),
        Err(e) => {
            return Ok(Response::builder()
                .status(500)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: format!("Failed to create access token: {}", e),
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

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
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
