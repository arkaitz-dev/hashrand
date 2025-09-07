//! JWT Authentication middleware for protected endpoints
//!
//! Provides middleware functions to validate Bearer tokens and protect
//! endpoints that require authentication.

use crate::utils::JwtUtils;
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
        Ok(claims) => Ok(AuthContext {
            username: claims.sub,
            expires_at: claims.exp,
        }),
        Err(error_msg) => {
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

/// Wrapper for protected handlers (currently unused)
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
