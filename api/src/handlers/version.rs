use spin_sdk::http::Response;
use crate::types::VersionResponse;

/// Handles the /api/version endpoint to get version information
pub fn handle_version() -> anyhow::Result<Response> {
    let version_response = VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    
    let json_body = serde_json::to_string(&version_response)?;
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_body)
        .build())
}