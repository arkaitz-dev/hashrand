//! Authentication login handler
//!
//! Handles magic link authentication flow:
//! 1. POST /api/login/ - Generate magic link and send via email (logged in development)
//! 2. POST /api/login/magiclink/ - Validate magic link with Ed25519 signature and get JWT tokens
//! 3. DELETE /api/login/ - Clear refresh token cookie (logout)

use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;

use crate::database::connection::initialize_database;
use crate::utils::auth::{
    ErrorResponse, MagicLinkRequest, generate_magic_link, handle_refresh_token,
    validate_magic_link_secure,
};

/// Handle login authentication requests
///
/// # Arguments
/// * `req` - HTTP request
/// * `query_params` - Query parameters from URL
///
/// # Returns
/// * `Result<Response, anyhow::Error>` - HTTP response
pub async fn handle_login(
    req: Request,
    query_params: HashMap<String, String>,
) -> anyhow::Result<Response> {
    // Extract the full path from the request to handle specific endpoints
    let full_url = req
        .header("spin-full-url")
        .and_then(|h| h.as_str())
        .unwrap_or("")
        .to_string();

    // Parse path from URL
    let url_parts: Vec<&str> = full_url.split('?').collect();
    let full_path = url_parts.first().unwrap_or(&"");
    let path = if let Some(path_start) = full_path.find("/api") {
        &full_path[path_start..]
    } else {
        full_path
    };
    // Determine database environment
    // For now use Development since we don't have access to IncomingRequest
    // Initialize database to ensure auth_sessions table exists
    println!("Initializing database...");
    if let Err(e) = initialize_database() {
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

    // Handle specific endpoint: POST /api/login/magiclink/ (secure validation with Ed25519)
    if path == "/api/login/magiclink/" && *req.method() == Method::Post {
        println!("🔐 Handling secure magic link validation with Ed25519 verification");
        return validate_magic_link_secure(req.body());
    }

    // Handle default login endpoints: /api/login/
    match *req.method() {
        Method::Post => handle_magic_link_generation(req).await,
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

/// Handle POST /api/login/ - Generate magic link (HTTP routing wrapper)
async fn handle_magic_link_generation(req: Request) -> anyhow::Result<Response> {
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

    // Delegate to business logic
    generate_magic_link(&req, &magic_request).await
}

/// Handle DELETE /api/login/ - Clear refresh token cookie (logout)
fn handle_logout() -> anyhow::Result<Response> {
    // Create expired cookie to clear the refresh token
    let expired_cookie = "refresh_token=; HttpOnly; Secure; SameSite=Strict; Max-Age=0; Path=/";

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("set-cookie", expired_cookie)
        .body("{\"message\":\"Logged out successfully\"}")
        .build())
}

/// Public export for refresh token handling
pub async fn handle_refresh(req: Request) -> anyhow::Result<Response> {
    handle_refresh_token(req).await
}
