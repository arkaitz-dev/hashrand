//! Custom token operations using Blake2b-keyed system
//!
//! Handles creation and validation of access and refresh tokens using our custom token system.

use chrono::{DateTime, Utc};

use super::custom_tokens::{create_custom_access_token, create_custom_access_token_from_username, create_custom_refresh_token, create_custom_refresh_token_from_username, validate_custom_access_token, validate_custom_refresh_token};
use super::types::{AccessTokenClaims, RefreshTokenClaims};

/// Create refresh token using custom token system (with proper 9-minute duration)
pub fn create_refresh_token(
    email: &str,
    _session_id: i64, // Ignored - our custom system doesn't need session_id
) -> Result<(String, DateTime<Utc>), String> {
    create_custom_refresh_token(email)
}

/// Create refresh token from username using custom token system (with proper 9-minute duration)
pub fn create_refresh_token_from_username(
    username: &str,
    _session_id: Option<i64>, // Ignored - our custom system doesn't need session_id
) -> Result<(String, DateTime<Utc>), String> {
    create_custom_refresh_token_from_username(username)
}

/// Validate access token using custom token system
pub fn validate_access_token(token: &str) -> Result<AccessTokenClaims, String> {
    validate_custom_access_token(token)
}

/// Validate refresh token using custom token system (validation logic is same: expiration_timestamp < now)
pub fn validate_refresh_token(token: &str) -> Result<RefreshTokenClaims, String> {
    let access_claims = validate_custom_refresh_token(token)?;

    // Convert AccessTokenClaims to RefreshTokenClaims (add fake session_id for compatibility)
    Ok(RefreshTokenClaims {
        sub: access_claims.sub,
        exp: access_claims.exp,
        iat: access_claims.iat,
        token_type: access_claims.token_type,
        session_id: 0, // Fake session_id for compatibility - not used anywhere
    })
}