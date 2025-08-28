/// Authentication login handler
/// 
/// Handles magic link authentication flow:
/// 1. POST /api/login/ - Generate magic link and send via email (logged in development)
/// 2. GET /api/login/?magiclink=... - Validate magic link and return JWT tokens

use spin_sdk::http::{Method, Request, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{Duration, Utc};

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
}

/// Response for magic link generation (development)
#[derive(Serialize)]
struct MagicLinkResponse {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    dev_magic_link: Option<String>,
}

/// Response for successful authentication
#[derive(Serialize)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i64, // seconds
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
    println!("DEBUG: Request body bytes: {:?}", std::str::from_utf8(body_bytes));
    
    let magic_request: MagicLinkRequest = match serde_json::from_slice::<MagicLinkRequest>(body_bytes) {
        Ok(req) => {
            println!("DEBUG: Parsed request - Email: {}, UI Host: {:?}", req.email, req.ui_host);
            req
        },
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

    // Generate magic token and expiration (15 minutes)
    let magic_token = JwtUtils::generate_magic_token();
    let magic_expires_at = Utc::now() + Duration::minutes(15);

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
            
            let host_url = magic_request.ui_host
                .as_deref()  // Más limpio que .as_ref().map(|s| s.as_str())
                .unwrap_or(&fallback_host);
            
            println!("DEBUG: Final chosen host_url = {}", host_url);
            let magic_link = JwtUtils::create_magic_link_url(&host_url, &magic_token);
            println!("DEBUG: Generated magic_link = {}", magic_link);

            // In development, log the magic link instead of sending email
            println!("=== MAGIC LINK AUTHENTICATION (DEVELOPMENT MODE) ===");
            println!("Email: {}", magic_request.email);
            println!("UI Host: {:?}", magic_request.ui_host);
            println!("Final Host URL: {}", host_url);
            println!("Magic Link: {}", magic_link);
            println!("Expires: {} UTC", magic_expires_at.format("%Y-%m-%d %H:%M:%S"));
            println!("====================================================");

            // Clean up expired sessions
            let _ = AuthOperations::cleanup_expired_sessions(env);

            let response = MagicLinkResponse {
                message: "Magic link generated successfully. Check development logs for the link.".to_string(),
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

    // Find auth session by magic token
    println!("Searching for magic token in database: '{}'", magic_token);
    match AuthOperations::get_session_by_magic_token(env.clone(), magic_token) {
        Ok(Some(session)) => {
            // Check if magic token has expired
            let now = Utc::now().timestamp() as u64;
            
            if now > session.magic_expires_at {
                return Ok(Response::builder()
                    .status(400)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Magic link has expired".to_string(),
                    })?)
                    .build());
            }

            // Generate JWT tokens
            let (access_token, access_expires_at) = match JwtUtils::create_access_token(&session.email) {
                Ok(tokens) => tokens,
                Err(e) => {
                    println!("Failed to create access token: {}", e);
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Token generation failed".to_string(),
                        })?)
                        .build());
                }
            };

            let session_id = session.id.unwrap_or(0);
            let (refresh_token, refresh_expires_at) = match JwtUtils::create_refresh_token(&session.email, session_id) {
                Ok(tokens) => tokens,
                Err(e) => {
                    println!("Failed to create refresh token: {}", e);
                    return Ok(Response::builder()
                        .status(500)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Token generation failed".to_string(),
                        })?)
                        .build());
                }
            };

            // Update session with tokens using timestamps
            let access_expires_timestamp = access_expires_at.timestamp() as u64;
            let refresh_expires_timestamp = refresh_expires_at.timestamp() as u64;
            
            match AuthOperations::activate_session_tokens(
                env,
                session_id,
                &access_token,
                &refresh_token,
                access_expires_timestamp,
                refresh_expires_timestamp,
            ) {
                Ok(true) => {
                    println!("User {} authenticated successfully", session.email);

                    // Create response with access token and secure refresh token cookie
                    let auth_response = AuthResponse {
                        access_token,
                        token_type: "Bearer".to_string(),
                        expires_in: 15 * 60, // 15 minutes in seconds
                    };

                    // Set refresh token as HttpOnly, Secure, SameSite cookie
                    let cookie_value = format!(
                        "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
                        refresh_token,
                        7 * 24 * 60 * 60 // 1 week in seconds
                    );

                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .header("set-cookie", cookie_value)
                        .body(serde_json::to_string(&auth_response)?)
                        .build())
                }
                Ok(false) => Ok(Response::builder()
                    .status(404)
                    .header("content-type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "Session not found".to_string(),
                    })?)
                    .build()),
                Err(e) => {
                    println!("Failed to activate session tokens: {}", e);
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