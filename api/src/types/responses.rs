use serde::Serialize;

/// Response structure for the /api/version endpoint
#[derive(Serialize)]
pub struct VersionResponse {
    pub api_version: String,
    pub ui_version: String,
}
