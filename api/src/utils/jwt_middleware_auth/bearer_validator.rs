//! JWT Bearer Token Validation
//!
//! Bearer token extraction and validation logic

use chrono::Utc;
use spin_sdk::http::{Request, Response};
use tracing::{error, debug};

use crate::utils::JwtUtils;
use crate::utils::jwt_middleware_errors::create_auth_error_response;
use crate::utils::jwt_middleware_renewal::check_proactive_renewal;
use crate::utils::jwt_middleware_types::AuthContext;

use super::cookie_refresh::handle_token_refresh_from_cookies;
use super::helpers::decode_username_to_user_id;

/// Extract and validate Bearer token from Authorization header
///
/// # Arguments
/// * `req` - HTTP request to check for Authorization header
///
/// # Returns
/// * `Result<AuthContext, Response>` - Either valid auth context or error response
///
/// # Security Note
/// With credentials: 'omit' on frontend, auto-refresh from cookies is disabled.
/// Clients must explicitly call /api/refresh endpoint when access token expires.
pub fn validate_bearer_token(req: &Request) -> Result<AuthContext, Response> {
    // SECURITY: Validate that request doesn't contain both Authorization header AND refresh cookie
    if let Err(e) = crate::utils::validate_no_simultaneous_tokens(req) {
        // println!("🚨 [SECURITY VIOLATION] GET endpoint received request with both tokens");
        error!("🚨 [SECURITY VIOLATION] GET endpoint received request with both tokens");
        return Err(Response::builder()
            .status(403)
            .header("content-type", "application/json")
            .body(
                serde_json::to_string(&serde_json::json!({
                    "error": e,
                }))
                .unwrap_or_default(),
            )
            .build());
    }

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
            // Extract cryptographic information for signed responses
            let user_id = decode_username_to_user_id(&claims.sub)?;
            let pub_key_hex = hex::encode(claims.pub_key);

            let renewed_tokens = check_proactive_renewal(
                &claims.sub,
                refresh_expires_at,
                now,
                user_id,
                pub_key_hex,
            )?;

            Ok(AuthContext {
                username: claims.sub,
                expires_at: claims.exp,
                refresh_expires_at,
                renewed_tokens,
            })
        }
        Err(error_msg) => {
            // println!("🔍 DEBUG: Token validation failed: {}", error_msg);
            debug!("🔍 DEBUG: Token validation failed: {}", error_msg);

            // If token validation fails (any reason), try to refresh using cookies (2/3 system)
            handle_token_refresh_from_cookies(req, &error_msg)
        }
    }
}
