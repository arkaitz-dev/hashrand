//! DRY utilities for refresh token handler

use spin_sdk::http::Response;

use super::super::types::ErrorResponse;
use crate::utils::SignedResponse;

/// Create error HTTP response (DRY consolidation for 16 duplicated patterns)
///
/// # Arguments
/// * `status` - HTTP status code
/// * `message` - Error message string
///
/// # Returns
/// * `anyhow::Result<Response>` - HTTP response with JSON error body
pub fn create_error_response(status: u16, message: &str) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&ErrorResponse {
            error: message.to_string(),
        })?)
        .build())
}

/// Decode Base58 username to user_id bytes (DRY consolidation for 2 duplicated patterns)
///
/// # Arguments
/// * `username` - Base58 encoded username string
///
/// # Returns
/// * `Result<Vec<u8>, String>` - user_id bytes or error message
pub fn decode_username_to_user_id(username: &str) -> Result<Vec<u8>, String> {
    bs58::decode(username)
        .into_vec()
        .map_err(|e| format!("Failed to decode username: {}", e))
}

/// Serialize signed response to JSON (DRY consolidation for 2 duplicated patterns)
///
/// # Arguments
/// * `signed_response` - SignedResponse to serialize
///
/// # Returns
/// * `Result<String, String>` - JSON string or error message
pub fn serialize_response_to_json(signed_response: &SignedResponse) -> Result<String, String> {
    serde_json::to_string(signed_response)
        .map_err(|e| format!("Failed to serialize response: {}", e))
}

/// Extract refresh_token value from cookie header string
///
/// CRITICAL FIX (v1.6.33): Extract LAST occurrence instead of FIRST
/// When browser sends duplicate cookies after key rotation (OLD + NEW),
/// the LAST cookie is always the most recent one (NEW) after Set-Cookie.
/// This makes the system robust even if cookie deletion doesn't work perfectly.
///
/// # Arguments
/// * `cookie_header` - Cookie header value string
///
/// # Returns
/// * `Option<String>` - refresh_token value or None
pub fn extract_refresh_token_from_cookies(cookie_header: &str) -> Option<String> {
    let mut last_token: Option<String> = None;

    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(stripped) = cookie.strip_prefix("refresh_token=") {
            last_token = Some(stripped.to_string()); // ← Keep updating to get LAST
        }
    }

    last_token
}

/// Extract hostname from Host header for cookie Domain attribute
///
/// SECURITY: Extracts only hostname (no port, no protocol) for use as cookie Domain
///
/// # Arguments
/// * `host_header` - The Host header value (e.g., "localhost:5173" or "app.example.com")
///
/// # Returns
/// * `Option<String>` - The hostname without port, or None if invalid
pub fn extract_hostname_from_host_header(host_header: &str) -> Option<String> {
    // Remove port if present (split by ':' and take first part)
    let hostname = host_header.split(':').next()?.trim();

    // Validate that it's a reasonable hostname (basic validation)
    if hostname.is_empty() || hostname.contains('/') || hostname.contains('@') {
        return None;
    }

    Some(hostname.to_string())
}
