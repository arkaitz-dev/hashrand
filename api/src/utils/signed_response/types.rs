//! Type definitions for signed responses

use serde::{Deserialize, Serialize};

/// Universal signed response structure for all API endpoints
///
/// The payload is Base64 URL-safe encoded JSON.
/// The signature is an Ed25519 signature of the Base64 payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignedResponse {
    /// Base64 URL-safe encoded JSON payload as string (signed content)
    pub payload: String,
    /// Ed25519 signature of the Base64 payload string
    pub signature: String,
}
