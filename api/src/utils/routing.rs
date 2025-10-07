use crate::handlers::custom::handle_custom_request;
use crate::handlers::login::handle_refresh;
use crate::handlers::{
    handle_api_key_request, handle_confirm_read, handle_create_secret, handle_delete_secret,
    handle_login, handle_mnemonic_request, handle_password_request, handle_retrieve_secret,
    handle_version,
};

// Test endpoint handler (DEV-MODE ONLY - eliminated in production builds)
#[cfg(feature = "dev-mode")]
use crate::handlers::handle_dry_run_toggle;
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
        path if path.ends_with("/api/version") => match *method {
            Method::Get => handle_version(),
            _ => handle_method_not_allowed(),
        },

        // Test endpoints (DEV-MODE ONLY - this entire branch is eliminated in production)
        #[cfg(feature = "dev-mode")]
        path if path.starts_with("/api/test/dry-run") => match *method {
            Method::Get => handle_dry_run_toggle(req).await,
            _ => handle_method_not_allowed(),
        },

        // Authentication endpoints (support GET and POST)
        path if path.starts_with("/api/login") => handle_login(req, query_params).await,

        // Token refresh endpoint
        path if path.ends_with("/api/refresh") => handle_refresh(req).await,

        // Shared Secret endpoints
        path if path.ends_with("/api/shared-secret/create") => match *method {
            Method::Post => handle_create_secret(req).await,
            _ => handle_method_not_allowed(),
        },
        path if path.starts_with("/api/shared-secret/confirm-read") => match *method {
            Method::Get => {
                let hash = query_params.get("hash").map(|s| s.as_str()).unwrap_or("");
                handle_confirm_read(req, hash).await
            }
            _ => handle_method_not_allowed(),
        },
        path if path.starts_with("/api/shared-secret/") => {
            // Extract hash from path: /api/shared-secret/{hash}
            let hash = path.trim_start_matches("/api/shared-secret/");
            if hash.is_empty() {
                return handle_not_found();
            }
            match *method {
                Method::Get | Method::Post => handle_retrieve_secret(req, hash).await,
                Method::Delete => handle_delete_secret(req, hash).await,
                _ => handle_method_not_allowed(),
            }
        }

        // Not found
        _ => handle_not_found(),
    }
}

/// Handles not found routes with useful information about available endpoints
fn handle_not_found() -> anyhow::Result<Response> {
    let help_message = r#"Not Found

Available endpoints:
- GET /api/custom?length=21&alphabet=0&prefix=&suffix=&raw=true
- POST /api/custom (JSON body with optional seed parameter)
- GET /api/password?length=21&alphabet=3&raw=true
- POST /api/password (JSON body with optional seed parameter)
- GET /api/api-key?length=44&alphabet=2&raw=true
- POST /api/api-key (JSON body with optional seed parameter)
- GET /api/mnemonic?language=0&words=12 (BIP39 mnemonic phrases)
- POST /api/mnemonic (JSON body with seed parameter)
- POST /api/login/ (Generate magic link - JSON: {"email": "user@example.com"})
- POST /api/login/magiclink/ (Validate magic link with Ed25519 signature and get JWT tokens)
- POST /api/shared-secret/create (Create shared secret with dual-URL system)
- GET /api/shared-secret/{hash} (Retrieve shared secret, returns OTP_REQUIRED if needed)
- POST /api/shared-secret/{hash} (Retrieve shared secret with OTP validation)
- DELETE /api/shared-secret/{hash} (Delete shared secret if not fully consumed)
- GET /api/shared-secret/confirm-read?hash={hash} (Confirm read tracking)
- GET /api/version

Parameters:
- length: 2-128 (custom), 21-44 (password), 44-64 (api-key)
- alphabet: Integer 0-4 (custom: 0-4, password: 1 or 3, api-key: 1 or 2)
  0=base58 (default custom), 1=no-look-alike, 2=full (default api-key),
  3=full-with-symbols (default password), 4=numeric
- language: Integer 0-9 (mnemonic only, default 0)
  0=english, 1=spanish, 2=french, 3=portuguese, 4=japanese,
  5=chinese-simplified, 6=chinese-traditional, 7=italian, 8=korean, 9=czech
- words: 12 (default), 24 (mnemonic only)
- raw: true (default), false (adds newline)
- prefix/suffix: max 32 chars each (custom only)
- seed: base58-encoded 32 bytes (optional for POST requests)"#;

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
