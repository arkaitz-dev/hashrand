//! JWT token operations
//!
//! Handles creation and validation of access and refresh tokens.

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

use super::config::get_jwt_secret;
use super::crypto::email_to_username;
use super::types::{AccessTokenClaims, RefreshTokenClaims};

/// Create access token with 20 seconds expiration (for testing)
///
/// # Arguments
/// * `email` - User email address (will be converted to user_id)
///
/// # Returns
/// * `Result<(String, DateTime<Utc>), String>` - JWT token and expiration time or error
pub fn create_access_token(email: &str) -> Result<(String, DateTime<Utc>), String> {
    let now = Utc::now();
    let expires_at = now + Duration::minutes(3); // 3 minutes

    // Derive user_id from email for JWT subject
    let username = email_to_username(email)?;

    let claims = AccessTokenClaims {
        sub: username,
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
        token_type: "access".to_string(),
    };

    let header = Header::new(Algorithm::HS256);
    let jwt_secret = get_jwt_secret()?;
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

    match encode(&header, &claims, &encoding_key) {
        Ok(token) => Ok((token, expires_at)),
        Err(e) => Err(format!("Failed to create access token: {}", e)),
    }
}

/// Create access token directly from username (used for refresh)
///
/// # Arguments
/// * `username` - Base58 encoded username
///
/// # Returns
/// * `Result<(String, DateTime<Utc>), String>` - JWT token and expiration time or error
pub fn create_access_token_from_username(
    username: &str,
) -> Result<(String, DateTime<Utc>), String> {
    let now = Utc::now();
    let expires_at = now + Duration::minutes(3); // 3 minutes

    let claims = AccessTokenClaims {
        sub: username.to_string(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
        token_type: "access".to_string(),
    };

    let header = Header::new(Algorithm::HS256);
    let jwt_secret = get_jwt_secret()?;
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

    match encode(&header, &claims, &encoding_key) {
        Ok(token) => Ok((token, expires_at)),
        Err(e) => Err(format!("Failed to create access token: {}", e)),
    }
}

/// Create refresh token with 2 minutes expiration (for testing)
///
/// # Arguments
/// * `email` - User email address (will be converted to user_id)
/// * `session_id` - Database session ID for token revocation
///
/// # Returns
/// * `Result<(String, DateTime<Utc>), String>` - JWT token and expiration time or error
pub fn create_refresh_token(
    email: &str,
    session_id: i64,
) -> Result<(String, DateTime<Utc>), String> {
    let now = Utc::now();
    let expires_at = now + Duration::minutes(15); // 15 minutes

    // Derive user_id from email for JWT subject
    let username = email_to_username(email)?;

    let claims = RefreshTokenClaims {
        sub: username,
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
        token_type: "refresh".to_string(),
        session_id,
    };

    let header = Header::new(Algorithm::HS256);
    let jwt_secret = get_jwt_secret()?;
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_ref());

    match encode(&header, &claims, &encoding_key) {
        Ok(token) => Ok((token, expires_at)),
        Err(e) => Err(format!("Failed to create refresh token: {}", e)),
    }
}

/// Validate and decode access token
///
/// # Arguments
/// * `token` - JWT access token to validate
///
/// # Returns
/// * `Result<AccessTokenClaims, String>` - Decoded claims or validation error
#[allow(dead_code)]
pub fn validate_access_token(token: &str) -> Result<AccessTokenClaims, String> {
    let jwt_secret = get_jwt_secret().map_err(|e| format!("JWT secret error: {}", e))?;
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    match decode::<AccessTokenClaims>(token, &decoding_key, &validation) {
        Ok(token_data) => {
            if token_data.claims.token_type != "access" {
                return Err("Invalid token type".to_string());
            }
            Ok(token_data.claims)
        }
        Err(e) => Err(format!("Invalid access token: {}", e)),
    }
}

/// Validate and decode refresh token
///
/// # Arguments
/// * `token` - JWT refresh token to validate
///
/// # Returns
/// * `Result<RefreshTokenClaims, String>` - Decoded claims or validation error
#[allow(dead_code)]
pub fn validate_refresh_token(token: &str) -> Result<RefreshTokenClaims, String> {
    let jwt_secret = get_jwt_secret().map_err(|e| format!("JWT secret error: {}", e))?;
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    match decode::<RefreshTokenClaims>(token, &decoding_key, &validation) {
        Ok(token_data) => {
            if token_data.claims.token_type != "refresh" {
                return Err("Invalid token type".to_string());
            }
            Ok(token_data.claims)
        }
        Err(e) => Err(format!("Invalid refresh token: {}", e)),
    }
}
