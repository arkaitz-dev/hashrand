//! Request routing and database initialization
//!
//! Handles URL parsing and request dispatching to appropriate handlers

use spin_sdk::http::{Request, Response};

use super::utilities::create_error_response;
use crate::database::connection::initialize_database;

/// Extract and parse the API path from request
///
/// Extracts the full URL from the "spin-full-url" header
/// and parses the API path portion
pub fn extract_api_path(req: &Request) -> String {
    let full_url = req
        .header("spin-full-url")
        .and_then(|h| h.as_str())
        .unwrap_or("")
        .to_string();

    // Parse path from URL
    let url_parts: Vec<&str> = full_url.split('?').collect();
    let full_path = url_parts.first().unwrap_or(&"");

    if let Some(path_start) = full_path.find("/api") {
        full_path[path_start..].to_string()
    } else {
        full_path.to_string()
    }
}

/// Initialize database and return error response if it fails
///
/// Ensures the auth_sessions table exists before processing requests
/// Returns None if successful, Some(Response) with error if failed
pub fn initialize_database_or_error() -> Option<Response> {
    if initialize_database().is_err() {
        return Some(
            create_error_response(500, "Database initialization failed")
                .expect("Failed to create error response"),
        );
    }

    None
}
