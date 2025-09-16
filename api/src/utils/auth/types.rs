//! Authentication types and structures

use serde::{Deserialize, Serialize};

/// Request body for magic link generation with Ed25519 authentication
#[derive(Deserialize)]
pub struct MagicLinkRequest {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>, // Base58-encoded parameters for post-auth redirect
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_lang: Option<String>, // Language code for email template (e.g., "es", "en")
    pub pub_key: String,   // Ed25519 public key (64 hex chars = 32 bytes)
    pub signature: String, // Ed25519 signature (128 hex chars = 64 bytes)
}

/// Response for magic link generation (development)
#[derive(Serialize)]
#[allow(dead_code)]
pub struct MagicLinkResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_magic_link: Option<String>,
}

/// Error response structure
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
