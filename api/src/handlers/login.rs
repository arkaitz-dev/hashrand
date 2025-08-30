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
    models::AuthSession,
    operations::AuthOperations,
};
use crate::utils::JwtUtils;

/// Request body for magic link generation
#[derive(Deserialize)]
struct MagicLinkRequest {
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<String>, // Base58-encoded parameters for post-auth redirect
}

/// Response for magic link generation (development)
#[derive(Serialize)]
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
pub fn handle_login(
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
        Method::Post => handle_magic_link_generation(req, env),
        Method::Get => handle_magic_link_validation(req, query_params, env),
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
fn handle_magic_link_generation(
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
                    "DEBUG: Parsed request - Email: {}, UI Host: {:?}",
                    req.email, req.ui_host
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
    let magic_token = JwtUtils::generate_magic_token(&magic_request.email, magic_expires_at);

    // Create auth session
    let auth_session = AuthSession::new_magic_link(
        magic_request.email.clone(),
        magic_token.clone(),
        magic_expires_at,
    );

    // Save to database
    match AuthOperations::create_auth_session(env.clone(), &auth_session) {
        Ok(_) => {
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

            // In development, log the magic link instead of sending email
            println!("=== MAGIC LINK AUTHENTICATION (DEVELOPMENT MODE) ===");
            println!("Email: {}", magic_request.email);
            println!("UI Host: {:?}", magic_request.ui_host);
            println!("Final Host URL: {}", host_url);
            println!("Magic Link: {}", magic_link);
            println!(
                "Expires: {} UTC",
                magic_expires_at.format("%Y-%m-%d %H:%M:%S")
            );
            println!("====================================================");

            // Clean up expired sessions
            let _ = AuthOperations::cleanup_expired_sessions(env);

            let response = MagicLinkResponse {
                message: "Magic link generated successfully. Check development logs for the link."
                    .to_string(),
                dev_magic_link: Some(magic_link),
            };

            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&response)?)
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

    match AuthOperations::get_session_by_user_id_and_timestamp(
        env.clone(),
        &user_id_bytes,
        timestamp,
    ) {
        Ok(Some((access_token, refresh_token))) => {
            println!("User {} authenticated successfully", username);

            // Ensure user exists in users table
            let _ = AuthOperations::ensure_user_exists(env.clone(), &user_id_bytes);

            // Create response with access token, username, and secure refresh token cookie
            let auth_response = serde_json::json!({
                "access_token": access_token,
                "token_type": "Bearer",
                "expires_in": 180, // 3 minutes
                "username": username
            });

            // Set refresh token as HttpOnly, Secure, SameSite cookie
            let cookie_value = format!(
                "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
                refresh_token,
                15 * 60 // 15 minutes in seconds
            );

            // Delete used auth session
            let _ = AuthOperations::delete_session_by_user_id_and_timestamp(
                env,
                &user_id_bytes,
                timestamp,
            );

            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .header("set-cookie", cookie_value)
                .body(auth_response.to_string())
                .build())
        }
        Ok(None) => Ok(Response::builder()
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
