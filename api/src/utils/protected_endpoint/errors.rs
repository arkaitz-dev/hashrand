//! Protected Endpoint Error Response Utilities
//!
//! DRY-unified error response creation

use crate::utils::auth::ErrorResponse;
use spin_sdk::http::Response;

/// Create standardized error response (DRY utility)
///
/// # Arguments
/// * `status` - HTTP status code
/// * `error_message` - Error message to include
///
/// # Returns
/// * `Response` - Formatted error response
fn create_error_response(status: u16, error_message: String) -> Response {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&ErrorResponse {
                error: error_message,
            })
            .unwrap_or_else(|_| r#"{"error":"Internal error"}"#.to_string()),
        )
        .build()
}

/// Create 400 Bad Request error response
pub fn bad_request(message: impl Into<String>) -> Response {
    create_error_response(400, message.into())
}

/// Create 401 Unauthorized error response
pub fn unauthorized(message: impl Into<String>) -> Response {
    create_error_response(401, message.into())
}

/// Create 403 Forbidden error response
pub fn forbidden(message: impl Into<String>) -> Response {
    create_error_response(403, message.into())
}
