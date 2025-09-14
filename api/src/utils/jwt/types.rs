//! JWT token types and structures
//!
//! Defines the claim structures for access and refresh tokens used in authentication.

use serde::{Deserialize, Serialize};

/// JWT Claims structure for access tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    /// Subject (user_id derived from email)
    pub sub: String,
    /// Expiration time (unix timestamp)
    pub exp: i64,
    /// Issued at (unix timestamp)
    pub iat: i64,
    /// Token type
    pub token_type: String,
    /// Refresh token expiration time (unix timestamp) for proactive renewal
    pub refresh_expires_at: i64,
}

/// JWT Claims structure for refresh tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    /// Subject (user_id derived from email)
    pub sub: String,
    /// Expiration time (unix timestamp)
    pub exp: i64,
    /// Issued at (unix timestamp)
    pub iat: i64,
    /// Token type
    pub token_type: String,
    /// Random ID for cryptographic uniqueness (not persisted)
    pub session_id: i64,
}
