use crate::handlers::custom::handle_custom_request;
use crate::handlers::{
    handle_api_key_request, handle_custom, handle_from_seed, handle_mnemonic_request,
    handle_password_request, handle_version,
};
use spin_sdk::http::{Method, Request, Response};
use std::collections::HashMap;

/// Routes the request to the corresponding handler based on the path and method
///
/// # Arguments
/// * `req` - The full HTTP request
/// * `path` - The URL path (e.g., "/api/generate")
/// * `query_params` - Parsed query parameters
///
/// # Returns
/// Appropriate response for the endpoint or 404/405 if not found/method not allowed
pub fn route_request_with_req(
    req: Request,
    path: &str,
    query_params: HashMap<String, String>,
) -> anyhow::Result<Response> {
    let method = req.method();
    let body = req.body();

    match path {
        // Endpoints that support both GET and POST
        path if path.ends_with("/api/custom") => handle_custom_request(req),
        path if path.ends_with("/api/password") => handle_password_request(req),
        path if path.ends_with("/api/api-key") => handle_api_key_request(req),
        path if path.ends_with("/api/mnemonic") => handle_mnemonic_request(req),

        // GET-only endpoints
        path if path.ends_with("/api/generate") => {
            match method {
                &Method::Get => handle_custom(query_params), // Backward compatibility
                _ => handle_method_not_allowed(),
            }
        }
        path if path.ends_with("/api/version") => match method {
            &Method::Get => handle_version(),
            _ => handle_method_not_allowed(),
        },

        // POST-only endpoints
        path if path.ends_with("/api/from-seed") => match method {
            &Method::Post => handle_from_seed(body),
            _ => handle_method_not_allowed(),
        },

        // Not found
        _ => handle_not_found(),
    }
}

/// Legacy routing function for backward compatibility
#[allow(dead_code)]
pub fn route_request(
    path: &str,
    query_params: HashMap<String, String>,
    method: &Method,
    body: &[u8],
) -> anyhow::Result<Response> {
    match (path, method) {
        // GET endpoints
        (path, &Method::Get) if path.ends_with("/api/custom") => handle_custom(query_params),
        (path, &Method::Get) if path.ends_with("/api/generate") => handle_custom(query_params), // Backward compatibility
        (path, &Method::Get) if path.ends_with("/api/password") => {
            // Legacy routing not used in current implementation
            handle_method_not_allowed()
        }
        (path, &Method::Get) if path.ends_with("/api/api-key") => {
            // Legacy routing not used in current implementation
            handle_method_not_allowed()
        }
        (path, &Method::Get) if path.ends_with("/api/version") => handle_version(),

        // POST endpoints
        (path, &Method::Post) if path.ends_with("/api/from-seed") => handle_from_seed(body),

        // Method not allowed for existing endpoints
        (path, _)
            if path.ends_with("/api/custom")
                || path.ends_with("/api/generate")
                || path.ends_with("/api/password")
                || path.ends_with("/api/api-key")
                || path.ends_with("/api/version") =>
        {
            handle_method_not_allowed()
        }

        // Not found
        _ => handle_not_found(),
    }
}

/// Handles not found routes with useful information about available endpoints
fn handle_not_found() -> anyhow::Result<Response> {
    let help_message = r#"Not Found

Available endpoints:
- GET /api/custom?length=21&alphabet=base58&prefix=&suffix=&raw=true
- POST /api/custom (JSON body with optional seed parameter)
- GET /api/generate?length=21&alphabet=base58&prefix=&suffix=&raw=true (alias for /api/custom)
- GET /api/password?length=21&alphabet=full-with-symbols&raw=true  
- POST /api/password (JSON body with optional seed parameter)
- GET /api/api-key?length=44&alphabet=full&raw=true
- POST /api/api-key (JSON body with optional seed parameter)
- GET /api/mnemonic?language=english&words=12 (BIP39 mnemonic phrases)
- POST /api/mnemonic (JSON body with seed parameter)
- GET /api/version
- POST /api/from-seed (JSON body required)

Parameters:
- length: 2-128 (custom), 21-44 (password), 44-64 (api-key)
- alphabet: base58, no-look-alike, full, full-with-symbols
- language: english (default), spanish, french, portuguese, japanese, chinese, chinese-traditional, italian, korean, czech (mnemonic only)
- words: 12 (default), 24 (mnemonic only)
- raw: true (default), false (adds newline)
- prefix/suffix: max 32 chars each (custom only)
- seed: 64 hex characters (optional for POST requests)"#;

    Ok(Response::builder()
        .status(404)
        .header("content-type", "text/plain")
        .body(help_message)
        .build())
}

/// Handles method not allowed for existing endpoints
fn handle_method_not_allowed() -> anyhow::Result<Response> {
    Ok(Response::builder()
        .status(405)
        .header("content-type", "text/plain")
        .header("allow", "GET, POST")
        .body("Method Not Allowed")
        .build())
}
