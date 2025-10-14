//! Authentication Validation Middleware
//!
//! SECURITY: Enforces strict separation between access tokens and refresh cookies.
//! Rejects any request that contains both Authorization header and refresh_token cookie.

use spin_sdk::http::Request;
use tracing::{debug, error};

/// Validates that requests don't contain both Authorization header and refresh_token cookie
///
/// This enforces the security principle of channel separation:
/// - Access tokens (JWT) must ONLY be sent via Authorization header
/// - Refresh tokens must ONLY be sent via HttpOnly cookie
///
/// # Arguments
/// * `req` - The incoming HTTP request to validate
///
/// # Returns
/// * `Ok(())` - Request is valid (has only one authentication method or none)
/// * `Err(String)` - Request violates security policy (has both tokens)
///
/// # Security Rationale
/// Allowing both tokens simultaneously creates attack vectors:
/// - Confusion in authentication logic
/// - Possible exploitation if attacker captures access token AND has cookie access
/// - Violates principle of single authentication channel per token type
pub fn validate_no_simultaneous_tokens(req: &Request) -> Result<(), String> {
    // Check for Authorization header
    let has_auth_header = req.header("authorization").is_some();

    // Check for refresh_token cookie
    let has_refresh_cookie = req
        .header("cookie")
        .and_then(|h| h.as_str())
        .map(|cookies| cookies.contains("refresh_token="))
        .unwrap_or(false);

    if has_auth_header && has_refresh_cookie {
        //     "🚨 [SECURITY VIOLATION] Request contains BOTH Authorization header AND refresh_token cookie - REJECTED"
        // );
        error!(
            "🚨 [SECURITY VIOLATION] Request contains BOTH Authorization header AND refresh_token cookie - REJECTED"
        );
        Err("SECURITY: Request contains both Authorization header and refresh cookie - forbidden. Use only one authentication method.".to_string())
    } else {
        if has_auth_header {
            debug!("✅ [SECURITY] Request validated: Authorization header only (no cookie)");
        } else if has_refresh_cookie {
            //     "✅ [SECURITY] Request validated: refresh_token cookie only (no Authorization header)"
            // );
            debug!(
                "✅ [SECURITY] Request validated: refresh_token cookie only (no Authorization header)"
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spin_sdk::http::{Method, Request};

    #[test]
    fn test_no_tokens() {
        let req = Request::builder()
            .method(Method::Get)
            .uri("/api/test")
            .build();
        assert!(validate_no_simultaneous_tokens(&req).is_ok());
    }

    #[test]
    fn test_only_authorization_header() {
        let req = Request::builder()
            .method(Method::Get)
            .uri("/api/test")
            .header("authorization", "Bearer token123")
            .build();
        assert!(validate_no_simultaneous_tokens(&req).is_ok());
    }

    #[test]
    fn test_only_refresh_cookie() {
        let req = Request::builder()
            .method(Method::Get)
            .uri("/api/test")
            .header("cookie", "refresh_token=abc123; other=value")
            .build();
        assert!(validate_no_simultaneous_tokens(&req).is_ok());
    }

    #[test]
    fn test_both_tokens_rejected() {
        let req = Request::builder()
            .method(Method::Get)
            .uri("/api/test")
            .header("authorization", "Bearer token123")
            .header("cookie", "refresh_token=abc123")
            .build();
        assert!(validate_no_simultaneous_tokens(&req).is_err());
    }
}
