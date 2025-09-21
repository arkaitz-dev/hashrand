//! JWT middleware types and data structures

use serde::Serialize;

/// Error response structure for authentication failures
#[derive(Serialize)]
pub struct AuthErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
}

/// Authentication result with user information
#[allow(dead_code)]
pub struct AuthContext {
    pub username: String,
    pub expires_at: i64,
    pub refresh_expires_at: i64,
    /// New tokens generated due to proactive renewal (if any)
    pub renewed_tokens: Option<RenewedTokens>,
}

/// Renewed tokens for proactive refresh
#[derive(Debug)]
pub struct RenewedTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}