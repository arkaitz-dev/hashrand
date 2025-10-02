//! Authentication login handler module
//!
//! Handles magic link authentication flow:
//! 1. POST /api/login/ - Generate magic link and send via email (logged in development)
//! 2. POST /api/login/magiclink/ - Validate magic link with Ed25519 signature and get JWT tokens
//! 3. DELETE /api/login/ - Clear refresh token cookie (logout)

use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;

use crate::utils::auth::validate_magic_link_secure;

mod logout;
mod magic_link;
mod routing;
mod utilities;

// Re-export public handler functions
pub use logout::handle_logout;
pub use magic_link::handle_magic_link_generation;

/// Handle login authentication requests
///
/// Main entry point that routes requests to appropriate handlers
/// based on the request path and method
///
/// # Arguments
/// * `req` - HTTP request
/// * `_query_params` - Query parameters from URL (unused, required by signature)
///
/// # Returns
/// * `Result<Response, anyhow::Error>` - HTTP response
pub async fn handle_login(
    req: Request,
    _query_params: HashMap<String, String>,
) -> anyhow::Result<Response> {
    // Initialize database
    if let Some(error_response) = routing::initialize_database_or_error() {
        return Ok(error_response);
    }

    // Extract API path from request
    let path = routing::extract_api_path(&req);

    // Handle specific endpoint: POST /api/login/magiclink/ (secure validation with Ed25519)
    if path == "/api/login/magiclink/" && *req.method() == Method::Post {
        println!("🔐 Handling secure magic link validation with Ed25519 verification");
        return validate_magic_link_secure(req.body());
    }

    // Handle default login endpoints: /api/login/
    match *req.method() {
        Method::Post => handle_magic_link_generation(req).await,
        Method::Delete => handle_logout(req),
        _ => utilities::create_error_response(405, "Method not allowed"),
    }
}

/// Public export for refresh token handling
///
/// Delegates to the auth module's refresh token handler
pub async fn handle_refresh(req: Request) -> anyhow::Result<Response> {
    crate::utils::auth::handle_refresh_token(req).await
}
