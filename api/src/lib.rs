use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;
use std::sync::Once;
use tracing::debug;

// Initialize rust-i18n for email templates
rust_i18n::i18n!("locales");

// Project modules organized by functionality
mod database;
mod email_templates;
mod handlers;
mod types;
mod utils;

use utils::{init_rate_limiter, parse_query_params, route_request_with_req};

// Static initialization for tracing subscriber (only once)
static INIT_TRACING: Once = Once::new();

/// Initialize tracing subscriber with compilation-time defined log level
///
/// DEVELOPMENT MODE (with dev-mode feature):
/// - Default: RUST_LOG=info (info, warn, error)
/// - Override with RUST_LOG environment variable for debugging
/// - Use `just dev-debug` to start with RUST_LOG=debug
///
/// PRODUCTION MODE (without dev-mode feature):
/// - Fixed: error level only (security: prevents info leak)
/// - No environment variable override possible
/// - Compiled-in behavior for safety
fn init_tracing() {
    INIT_TRACING.call_once(|| {
        use tracing_subscriber::{EnvFilter, fmt};

        #[cfg(feature = "dev-mode")]
        {
            // DEVELOPMENT: Read logging level from Spin variable
            // Usage: just dev (info), just dev-debug (hashrand=debug,info - no Spin/Wasmtime noise)
            let log_level = spin_sdk::variables::get("rust_log").unwrap_or_else(|_| "info".to_string());

            fmt()
                .with_env_filter(EnvFilter::new(log_level))
                .with_target(false)
                .with_thread_ids(false)
                .with_line_number(false)
                .init();
        }

        #[cfg(not(feature = "dev-mode"))]
        {
            // PRODUCTION: Fixed "error" level only, no environment variable override
            // Security: Prevents accidental verbose logging in production (info leak protection)
            fmt()
                .with_env_filter(EnvFilter::new("error"))
                .with_target(false)
                .with_thread_ids(false)
                .with_line_number(false)
                .init();
        }
    });
}

/// Main Spin HTTP component function
///
/// Handles all HTTP requests and routes them to the corresponding handlers.
/// Supports the following endpoints:
/// - GET /api/custom - Customizable hash generation
/// - GET /api/password - Secure password generation
/// - GET /api/api-key - API key generation with ak_ prefix
/// - GET /api/mnemonic - BIP39 mnemonic phrase generation
/// - GET /api/version - Version information
/// - POST /api/login/ - Magic link generation
/// - POST /api/login/magiclink/ - Magic link validation
/// - POST /api/refresh - Token refresh with key rotation
#[http_component]
async fn handle_hashrand_spin(req: Request) -> anyhow::Result<impl IntoResponse> {
    // Initialize tracing subscriber (only once)
    init_tracing();

    // Initialize rate limiter on first request
    init_rate_limiter();

    // Get the full URL from the spin-full-url header
    let full_url = req
        .header("spin-full-url")
        .and_then(|h| h.as_str())
        .unwrap_or("")
        .to_string(); // Clone to avoid borrowing issues

    debug!("Handling request to: {}", full_url);

    // Parse the URL to get path and query parameters
    let url_parts: Vec<&str> = full_url.split('?').collect();
    let full_path = url_parts.first().unwrap_or(&"");
    let query_string = url_parts.get(1).unwrap_or(&"");

    // Extract just the path part from the full URL
    let path = if let Some(path_start) = full_path.find("/api") {
        &full_path[path_start..]
    } else {
        full_path
    }
    .to_string();

    // Parse query parameters
    let query_params = parse_query_params(query_string);

    // Route according to path and method using the modular system
    route_request_with_req(req, &path, query_params).await
}
