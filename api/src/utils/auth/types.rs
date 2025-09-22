//! Authentication types and structures

use crate::utils::SignedRequest;
use serde::{Deserialize, Serialize};

/// Payload for magic link generation (wrapped in SignedRequest)
#[derive(Deserialize, Serialize)]
pub struct MagicLinkPayload {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_host: Option<String>,
    #[serde(default = "default_next_path")]
    pub next: String, // Always a path: "/" for login, or "/mnemonic/?params..." etc
    pub email_lang: String, // Language code for email template (e.g., "es", "en") - REQUIRED
    pub pub_key: String,    // Ed25519 public key (64 hex chars = 32 bytes)
}

fn default_next_path() -> String {
    "/".to_string()
}

/// New signed request structure for magic link generation
pub type MagicLinkSignedRequest = SignedRequest<MagicLinkPayload>;

/// Legacy request body for magic link generation (DEPRECATED - kept for transition)
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct MagicLinkRequest {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>, // Base58-encoded parameters for post-auth redirect
    pub email_lang: String, // Language code for email template (e.g., "es", "en") - REQUIRED, matches user selection
    pub pub_key: String,    // Ed25519 public key (64 hex chars = 32 bytes)
    pub signature: String,  // Ed25519 signature (128 hex chars = 64 bytes)
}

/// Response for magic link generation (development)
#[derive(Serialize)]
#[allow(dead_code)]
pub struct MagicLinkResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_magic_link: Option<String>,
}

/// Payload structure for magic link validation (unified SignedRequest format)
#[derive(Deserialize, Serialize)]
pub struct MagicLinkValidationPayload {
    pub magiclink: String, // Magic link token
}

/// Unified signed request structure for magic link validation
pub type MagicLinkValidationRequest = SignedRequest<MagicLinkValidationPayload>;

/// Error response structure
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
