//! Authentication types and structures
//!
//! # ENUM ENCODING POLICY - CRITICAL DESIGN RULE
//!
//! **GENERAL RULE**: All fixed enums/lists in request/response payloads
//! MUST use integer encoding to minimize network payload size.
//!
//! **EXAMPLES**:
//! - ✅ AlphabetType: "base58" → 0 (85% size reduction)
//! - ✅ MnemonicLanguage: "english" → 0 (87% size reduction)
//!
//! **EXCEPTION (ONLY ONE)**:
//! - ❌ email_lang: Uses ISO 639-1 string codes ("es", "en", etc.)
//!   - **Reason**: rust_i18n library requires string codes for set_locale()
//!   - **Trade-off**: +1-2 bytes per request for external library compatibility
//!   - **Location**: MagicLinkPayload, SharedSecretRequest
//!
//! **ENFORCEMENT**: When adding new enum fields, default to integer encoding
//! unless blocked by external dependency. Document exceptions here.

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
    /// EXCEPTION: Uses ISO string instead of integer (see module doc for rationale)
    pub email_lang: String, // Language code for email template (e.g., "es", "en") - REQUIRED
    pub ed25519_pub_key: String, // Ed25519 public key (64 hex chars = 32 bytes) for signatures
    pub x25519_pub_key: String,  // X25519 public key (64 hex chars = 32 bytes) for ECDH
}

fn default_next_path() -> String {
    "/".to_string()
}

/// New signed request structure for magic link generation
/// CORRECTED: No longer generic since SignedRequest uses Base64-encoded JSON payload
pub type MagicLinkSignedRequest = SignedRequest;

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
/// CORRECTED: No longer generic since SignedRequest uses Base64-encoded JSON payload
pub type MagicLinkValidationRequest = SignedRequest;

/// Payload for token refresh (wrapped in SignedRequest)
#[derive(Deserialize, Serialize)]
pub struct RefreshPayload {
    pub new_ed25519_pub_key: String, // Ed25519 public key (64 hex chars = 32 bytes) - REQUIRED
    pub new_x25519_pub_key: String,  // X25519 public key (64 hex chars = 32 bytes) - REQUIRED
}

/// Unified signed request structure for token refresh
pub type RefreshSignedRequest = SignedRequest;

/// Error response structure
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
