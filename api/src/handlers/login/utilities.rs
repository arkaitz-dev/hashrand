//! Utilities for login handler - DRY helpers
//!
//! This module consolidates repeated patterns:
//! - Error response creation (used 8x in original)
//! - Cookie extraction (refresh_token parsing)
//! - Query parameter parsing (used in multiple handlers)

use spin_sdk::http::Response;
use std::collections::HashMap;

use crate::utils::auth::ErrorResponse;

/// Create error response with JSON body (DRY utility)
///
/// Consolidates the repeated pattern of creating error responses
/// that appeared 8 times in the original handler
pub fn create_error_response(status: u16, error_message: &str) -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&ErrorResponse {
            error: error_message.to_string(),
        })?)
        .build())
}

/// Extract refresh_token value from cookie header string
///
/// Parses the Cookie header to find the refresh_token value
/// Returns None if the cookie is not found
pub fn extract_refresh_token_from_cookies(cookie_header: &str) -> Option<String> {
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if let Some(stripped) = cookie.strip_prefix("refresh_token=") {
            return Some(stripped.to_string());
        }
    }
    None
}

/// Parse query parameters from URI string
///
/// Extracts and parses query parameters from a URI string
/// Returns empty HashMap if no query string found
pub fn parse_query_params(uri_str: &str) -> HashMap<String, String> {
    let query = if let Some(idx) = uri_str.find('?') {
        &uri_str[idx + 1..]
    } else {
        ""
    };

    query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.split('=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect()
}
