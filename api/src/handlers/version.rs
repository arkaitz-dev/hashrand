use spin_sdk::http::Response;
use crate::types::VersionResponse;

/// Handles the /api/version endpoint to get version information
pub fn handle_version() -> anyhow::Result<Response> {
    // Read UI version from package.json at compile time
    let ui_version = get_ui_version();
    
    let version_response = VersionResponse {
        api_version: env!("CARGO_PKG_VERSION").to_string(),
        ui_version,
    };
    
    let json_body = serde_json::to_string(&version_response)?;
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_body)
        .build())
}

/// Gets the UI version from package.json at compile time
fn get_ui_version() -> String {
    // Include package.json content at compile time
    let package_json_content = include_str!("../../../package.json");
    
    // Parse and extract version
    if let Ok(package_json) = serde_json::from_str::<serde_json::Value>(package_json_content) {
        if let Some(version) = package_json["version"].as_str() {
            return version.to_string();
        }
    }
    
    "unknown".to_string()
}