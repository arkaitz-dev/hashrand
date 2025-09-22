//! Magic link HTTP response builder
//!
//! Provides standardized HTTP response builders for magic link operations
//! including success responses and error handling.

use spin_sdk::http::Response;

use super::types::ErrorResponse;

/// Magic link HTTP response builder operations
pub struct MagicLinkResponseBuilder;

impl MagicLinkResponseBuilder {
    /// Build success response for magic link generation
    ///
    /// Creates a standardized 200 OK response with CORS headers
    /// and JSON status confirmation.
    ///
    /// # Returns
    /// * `Response` - HTTP 200 response with CORS headers
    pub fn build_success_response() -> Response {
        Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .header("access-control-allow-origin", "*")
            .header("access-control-allow-methods", "POST, GET, OPTIONS")
            .header("access-control-allow-headers", "Content-Type")
            .body("{\"status\":\"OK\"}")
            .build()
    }

    /// Build error response for database storage failure
    ///
    /// Creates a standardized 500 Internal Server Error response
    /// for when magic link storage in database fails.
    ///
    /// # Arguments
    /// * `error_msg` - Detailed error message for logging
    ///
    /// # Returns
    /// * `Result<Response, serde_json::Error>` - HTTP 500 response or serialization error
    pub fn build_storage_error_response(error_msg: &str) -> Result<Response, serde_json::Error> {
        println!("Failed to create auth session: {}", error_msg);

        Ok(Response::builder()
            .status(500)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&ErrorResponse {
                error: "Failed to generate magic link".to_string(),
            })?)
            .build())
    }
}
