//! Type definitions for signed requests

use serde::{Deserialize, Serialize};

/// Universal signed request structure for all API endpoints
///
/// The payload is Base64 URL-safe encoded deterministic JSON.
/// The signature verifies the original JSON string (before Base64 encoding).
#[derive(Debug, Deserialize, Serialize)]
pub struct SignedRequest {
    /// Base64 URL-safe encoded deterministic JSON payload
    pub payload: String,
    /// Ed25519 signature of the original JSON string (before Base64 encoding)
    pub signature: String,
}
