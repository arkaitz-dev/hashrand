use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

// Project modules organized by functionality
mod types;
mod handlers;
mod utils;

use utils::{parse_query_params, route_request};

/// Main Spin HTTP component function
/// 
/// Handles all HTTP requests and routes them to the corresponding handlers.
/// Supports the following endpoints:
/// - GET /api/generate - Customizable hash generation
/// - GET /api/password - Secure password generation  
/// - GET /api/api-key - API key generation with ak_ prefix
/// - GET /api/version - Version information
#[http_component]
fn handle_hashrand_spin(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Get the full URL from the spin-full-url header
    let full_url = req.header("spin-full-url")
        .and_then(|h| h.as_str())
        .unwrap_or("");
    
    println!("Handling request to: {}", full_url);
    
    // Parse the URL to get path and query parameters
    let url_parts: Vec<&str> = full_url.split('?').collect();
    let path = url_parts.get(0).unwrap_or(&"");
    let query_string = url_parts.get(1).unwrap_or(&"");
    
    // Parse query parameters
    let query_params = parse_query_params(query_string);
    
    // Route according to path using the modular system
    route_request(path, query_params)
}
