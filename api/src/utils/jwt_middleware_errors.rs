//! JWT middleware error handling - Authentication error responses

use spin_sdk::http::Response;
use super::jwt_middleware_types::AuthErrorResponse;

/// Create standardized authentication error response
pub fn create_auth_error_response(error: &str, expires_in: Option<String>) -> Response {
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

/// Create specialized response for dual token expiry (both access and refresh tokens expired)
///
/// This function handles the special case where both tokens have expired:
/// - Sets refresh_token cookie with Max-Age=0 (immediate expiry to clear client-side)
/// - Returns descriptive error message for frontend to handle complete logout
/// - Signals the frontend to clear sessionStorage and request new email authentication
pub fn create_dual_expiry_response() -> Response {
    let response = AuthErrorResponse {
        error: "Both access and refresh tokens have expired. Please authenticate again with your email.".to_string(),
        expires_in: Some("Complete re-authentication required".to_string()),
    };

    // Create response with immediate cookie expiry to clear client-side refresh token
    Response::builder()
        .status(401)
        .header("content-type", "application/json")
        .header("www-authenticate", "Bearer")
        .header(
            "set-cookie",
            "refresh_token=expired; HttpOnly; Secure; SameSite=Strict; Max-Age=0; Path=/",
        )
        .body(
            serde_json::to_string(&response).unwrap_or_else(|_| {
                r#"{"error":"Complete re-authentication required"}"#.to_string()
            }),
        )
        .build()
}