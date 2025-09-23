use crate::handlers::custom::handle_custom_request;
use crate::handlers::login::handle_refresh;
use crate::handlers::{
    handle_api_key_request, handle_from_seed, handle_login,
    handle_mnemonic_request, handle_password_request, handle_version,
};
use crate::utils::jwt_middleware::{requires_authentication, with_auth_and_renewal};
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
pub async fn route_request_with_req(
    req: Request,
    path: &str,
    query_params: HashMap<String, String>,
) -> anyhow::Result<Response> {
    let method = req.method();

    match path {
        // Protected endpoints with proactive token renewal
        path if path.ends_with("/api/custom") => {
            match *method {
                Method::Get => {
                    // GET requests: JWT validation + SignedResponse output (no Ed25519 signature required)
                    if requires_authentication(path) {
                        with_auth_and_renewal(req, |req| {
                            crate::handlers::custom::handle_custom_get(req)
                        })
                    } else {
                        crate::handlers::custom::handle_custom_get(req)
                    }
                }
                Method::Post => {
                    // POST requests: Full Ed25519 + JWT validation + SignedResponse output
                    handle_custom_request(req).await
                }
                _ => handle_method_not_allowed(),
            }
        }
        path if path.ends_with("/api/password") => {
            match *method {
                Method::Get => {
                    // GET requests now use SignedResponse like POST requests
                    handle_password_request(req).await
                }
                Method::Post => {
                    // POST requests use SignedRequest validation internally
                    handle_password_request(req).await
                }
                _ => handle_method_not_allowed(),
            }
        }
        path if path.ends_with("/api/api-key") => {
            match *method {
                Method::Get => {
                    // GET requests now use SignedResponse like POST requests
                    handle_api_key_request(req).await
                }
                Method::Post => {
                    // POST requests use SignedRequest validation internally
                    handle_api_key_request(req).await
                }
                _ => handle_method_not_allowed(),
            }
        }
        path if path.ends_with("/api/mnemonic") => {
            match *method {
                Method::Get => {
                    // GET requests now use SignedResponse like POST requests
                    handle_mnemonic_request(req).await
                }
                Method::Post => {
                    // POST requests use SignedRequest validation internally
                    handle_mnemonic_request(req).await
                }
                _ => handle_method_not_allowed(),
            }
        }

        // GET-only endpoints
        path if path.ends_with("/api/generate") => {
            match *method {
                Method::Get => {
                    // Legacy alias for /api/custom - now uses SignedResponse
                    handle_custom_request(req).await
                }
                _ => handle_method_not_allowed(),
            }
        }
        path if path.ends_with("/api/version") => match *method {
            Method::Get => handle_version(),
            _ => handle_method_not_allowed(),
        },

        // POST-only endpoints
        path if path.ends_with("/api/from-seed") => match *method {
            Method::Post => {
                if requires_authentication(path) {
                    with_auth_and_renewal(req, |req| handle_from_seed(req.body()))
                } else {
                    handle_from_seed(req.body())
                }
            }
            _ => handle_method_not_allowed(),
        },

        // Authentication endpoints (support GET and POST)
        path if path.starts_with("/api/login") => handle_login(req, query_params).await,

        // Token refresh endpoint
        path if path.ends_with("/api/refresh") => handle_refresh(req).await,

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
- POST /api/login/ (Generate magic link - JSON: {"email": "user@example.com"})
- POST /api/login/magiclink/ (Validate magic link with Ed25519 signature and get JWT tokens)
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
