// ==================== DEV-MODE ONLY: Test Endpoints ====================
// This ENTIRE FILE is eliminated from production builds (cargo build --no-default-features)
// The /api/test/* endpoints DO NOT EXIST in production binaries

use anyhow::Result;
use serde_json::json;
use spin_sdk::http::{Request, Response};

/// Handles the /api/test/dry-run endpoint to toggle email dry-run mode (DEV-MODE ONLY)
///
/// This endpoint allows runtime control of email sending during development and testing.
/// In production builds, this function is completely removed from the binary.
///
/// # Query Parameters
/// * `enabled` - Optional boolean ("true" or "false", defaults to "true")
///
/// # Returns
/// * `200` with JSON status and current dry-run state
///
/// # Security
/// This endpoint is IMPOSSIBLE to call in production because the code doesn't exist.
/// The compiler eliminates it when building with --no-default-features.
pub async fn handle_dry_run_toggle(req: Request) -> Result<Response> {
    // Extract query string from spin-full-url header
    let full_url = req
        .header("spin-full-url")
        .and_then(|h| h.as_str())
        .unwrap_or("");

    // Parse query parameter 'enabled' (defaults to true)
    let enabled = full_url
        .split('?')
        .nth(1)
        .and_then(|query| {
            query
                .split('&')
                .find(|param| param.starts_with("enabled="))
                .and_then(|param| param.strip_prefix("enabled="))
        })
        .map(|val| val == "true")
        .unwrap_or(true);

    // Toggle dry-run mode using the email module's function
    // Note: set_email_dry_run() is also dev-mode only
    crate::utils::email::set_email_dry_run(enabled);

    // Return success response
    let response_body = json!({
        "email_dry_run": enabled,
        "message": if enabled {
            "Email dry-run mode ENABLED - emails will NOT be sent (only logged)"
        } else {
            "Email dry-run mode DISABLED - emails will be sent normally"
        }
    });

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(response_body.to_string())
        .build())
}
