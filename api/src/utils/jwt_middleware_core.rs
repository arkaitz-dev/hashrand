//! JWT middleware core functionality - Main coordination and public API

use spin_sdk::http::{Request, Response};

use super::jwt_middleware_auth::validate_bearer_token;
use super::jwt_middleware_renewal::add_renewed_tokens_to_response;
use super::jwt_middleware_types::AuthContext;

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
