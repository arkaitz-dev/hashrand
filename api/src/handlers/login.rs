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
    ErrorResponse, MagicLinkSignedRequest, generate_magic_link_signed, handle_refresh_token,
    validate_magic_link_secure,
};
use crate::utils::auth::types::MagicLinkPayload;
use crate::utils::SignedRequestValidator;

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
    _query_params: HashMap<String, String>,
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

/// Handle POST /api/login/ - Generate magic link using universal SignedRequest
async fn handle_magic_link_generation(req: Request) -> anyhow::Result<Response> {
    // Parse request body
    let body_bytes = req.body();
    println!(
        "DEBUG: Request body bytes: {:?}",
        std::str::from_utf8(body_bytes)
    );

    // Parse as SignedRequest structure
    let signed_request = match serde_json::from_slice::<MagicLinkSignedRequest>(body_bytes) {
        Ok(req) => {
            println!("🔐 DEBUG: Received SignedRequest structure with Base64-encoded JSON payload");

            // CORRECTED: Deserialize Base64-encoded JSON payload to access fields
            let deserialized_payload: MagicLinkPayload = match SignedRequestValidator::deserialize_base64_payload(&req.payload) {
                Ok(payload) => payload,
                Err(e) => {
                    println!("❌ DEBUG: Failed to deserialize Base64 payload: {}", e);
                    return Ok(Response::builder()
                        .status(400)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&ErrorResponse {
                            error: "Invalid Base64 JSON payload format".to_string(),
                        })?)
                        .build());
                }
            };

            println!(
                "DEBUG: Deserialized Payload - Email: {}, UI Host: {:?}, Email Lang: {:?}",
                deserialized_payload.email,
                deserialized_payload.ui_host,
                deserialized_payload.email_lang
            );
            req
        }
        Err(e) => {
            println!("DEBUG: JSON parse error for SignedRequest: {}", e);
            return Ok(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "Invalid JSON body - must be SignedRequest structure".to_string(),
                })?)
                .build());
        }
    };

    // Use universal SignedRequest handler
    generate_magic_link_signed(&req, &signed_request).await
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
