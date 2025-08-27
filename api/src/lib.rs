use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

// Project modules organized by functionality
mod database;
mod handlers;
mod types;
mod utils;

use utils::{parse_query_params, route_request_with_req};

/// Main Spin HTTP component function
///
/// Handles all HTTP requests and routes them to the corresponding handlers.
/// Supports the following endpoints:
/// - GET /api/custom - Customizable hash generation
/// - GET /api/generate - Alias for /api/custom (backward compatibility)
/// - GET /api/password - Secure password generation  
/// - GET /api/api-key - API key generation with ak_ prefix
/// - GET /api/version - Version information
/// - POST /api/from-seed - Seed-based hash generation
#[http_component]
fn handle_hashrand_spin(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Get the full URL from the spin-full-url header
    let full_url = req
        .header("spin-full-url")
        .and_then(|h| h.as_str())
        .unwrap_or("")
        .to_string(); // Clone to avoid borrowing issues

    println!("Handling request to: {}", full_url);

    // Parse the URL to get path and query parameters
    let url_parts: Vec<&str> = full_url.split('?').collect();
    let full_path = url_parts.first().unwrap_or(&"");
    let query_string = url_parts.get(1).unwrap_or(&"");
    
    // Extract just the path part from the full URL
    let path = if let Some(path_start) = full_path.find("/api") {
        &full_path[path_start..]
    } else {
        full_path
    }.to_string();

    // Parse query parameters
    let query_params = parse_query_params(query_string);

    // Route according to path and method using the modular system
    route_request_with_req(req, &path, query_params)
}
