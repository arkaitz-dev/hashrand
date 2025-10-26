//! HTTP response helpers and JWT extraction utilities

use serde::Serialize;
use spin_sdk::http::Response;

use super::signing::create_signed_response;

/// Create signed HTTP response for regular endpoints (without server_pub_key)
///
/// Universal helper function that can be used by any endpoint handler
/// to easily convert their payload into a signed HTTP response.
///
/// # Arguments
/// * `payload` - Response data to be signed (any serializable type)
/// * `user_id` - User ID bytes (from JWT access token or derived from email)
/// * `pub_key_hex` - Frontend Ed25519 public key as hex string
///
/// # Returns
/// * `Result<Response, String>` - Signed HTTP response or error
pub fn create_signed_http_response<T>(
    payload: T,
    user_id: &[u8],
    pub_key_hex: &str,
) -> Result<Response, String>
where
    T: Serialize,
{
    // Generate signed response (without server public key)
    let signed_response = create_signed_response(payload, user_id, pub_key_hex)
        .map_err(|e| format!("Failed to create signed response: {}", e))?;

    // Serialize signed response to JSON
    let response_json = serde_json::to_string(&signed_response)
        .map_err(|e| format!("Failed to serialize signed response: {}", e))?;

    // Build HTTP response with CORS headers
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("access-control-allow-origin", "*")
        .header("access-control-allow-methods", "POST, GET, OPTIONS")
        .header("access-control-allow-headers", "Content-Type")
        .body(response_json)
        .build())
}