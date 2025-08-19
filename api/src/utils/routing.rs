use crate::handlers::{handle_api_key, handle_generate, handle_password, handle_version};
use spin_sdk::http::Response;
use std::collections::HashMap;

/// Routes the request to the corresponding handler based on the path
///
/// # Arguments
/// * `path` - The URL path (e.g., "/api/generate")
/// * `query_params` - Parsed query parameters
///
/// # Returns
/// Appropriate response for the endpoint or 404 if not found
pub fn route_request(
    path: &str,
    query_params: HashMap<String, String>,
) -> anyhow::Result<Response> {
    match path {
        path if path.ends_with("/api/generate") => handle_generate(query_params),
        path if path.ends_with("/api/password") => handle_password(query_params),
        path if path.ends_with("/api/api-key") => handle_api_key(query_params),
        path if path.ends_with("/api/version") => handle_version(),
        _ => handle_not_found(),
    }
}

/// Handles not found routes with useful information about available endpoints
fn handle_not_found() -> anyhow::Result<Response> {
    let help_message = r#"Not Found

Available endpoints:
- GET /api/generate?length=21&alphabet=base58&prefix=&suffix=&raw=true
- GET /api/password?length=21&alphabet=full-with-symbols&raw=true  
- GET /api/api-key?length=44&alphabet=full&raw=true
- GET /api/version

Parameters:
- length: 2-128 (generate), 21-44 (password), 44-64 (api-key)
- alphabet: base58, no-look-alike, full, full-with-symbols
- raw: true (default), false (adds newline)
- prefix/suffix: max 32 chars each (generate only)"#;

    Ok(Response::builder()
        .status(404)
        .header("content-type", "text/plain")
        .body(help_message)
        .build())
}
