//! JWT token utilities for authentication
//!
//! Provides functions for creating and validating JWT access and refresh tokens
//! with proper expiration times and security claims.

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT Claims structure for access tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    /// Subject (user email)
    pub sub: String,
    /// Expiration time (unix timestamp)
    pub exp: i64,
    /// Issued at (unix timestamp)
    pub iat: i64,
    /// Token type
    pub token_type: String,
}

/// JWT Claims structure for refresh tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    /// Subject (user email)
    pub sub: String,
    /// Expiration time (unix timestamp)
    pub exp: i64,
    /// Issued at (unix timestamp)
    pub iat: i64,
    /// Token type
    pub token_type: String,
    /// Session ID for token revocation
    pub session_id: i64,
}

/// JWT token generation and validation utilities
pub struct JwtUtils;

impl JwtUtils {
    /// Get JWT secret key (in production this should be from environment variable)
    /// For development, we generate a consistent secret
    fn get_jwt_secret() -> String {
        // In production, this should come from environment variable
        // For development, we use a consistent secret
        "hashrand-jwt-secret-key-development-only-change-in-production".to_string()
    }

    /// Create access token with 15 minutes expiration
    ///
    /// # Arguments
    /// * `email` - User email address
    ///
    /// # Returns
    /// * `Result<(String, DateTime<Utc>), String>` - JWT token and expiration time or error
    pub fn create_access_token(email: &str) -> Result<(String, DateTime<Utc>), String> {
        let now = Utc::now();
        let expires_at = now + Duration::minutes(15);

        let claims = AccessTokenClaims {
            sub: email.to_string(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            token_type: "access".to_string(),
        };

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(Self::get_jwt_secret().as_ref());

        match encode(&header, &claims, &encoding_key) {
            Ok(token) => Ok((token, expires_at)),
            Err(e) => Err(format!("Failed to create access token: {}", e)),
        }
    }

    /// Create refresh token with 1 week expiration
    ///
    /// # Arguments
    /// * `email` - User email address
    /// * `session_id` - Database session ID for token revocation
    ///
    /// # Returns
    /// * `Result<(String, DateTime<Utc>), String>` - JWT token and expiration time or error
    pub fn create_refresh_token(
        email: &str,
        session_id: i64,
    ) -> Result<(String, DateTime<Utc>), String> {
        let now = Utc::now();
        let expires_at = now + Duration::days(7);

        let claims = RefreshTokenClaims {
            sub: email.to_string(),
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            token_type: "refresh".to_string(),
            session_id,
        };

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(Self::get_jwt_secret().as_ref());

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
        let decoding_key = DecodingKey::from_secret(Self::get_jwt_secret().as_ref());
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
        let decoding_key = DecodingKey::from_secret(Self::get_jwt_secret().as_ref());
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

    /// Generate secure magic token for email authentication
    ///
    /// # Returns
    /// * `String` - Base58 encoded magic token (URL safe)
    pub fn generate_magic_token() -> String {
        let uuid = Uuid::new_v4();
        bs58::encode(uuid.as_bytes()).into_string()
    }

    /// Create magic link URL for development logging
    ///
    /// # Arguments
    /// * `host_url` - Base URL from request (e.g., "http://localhost:5173")
    /// * `magic_token` - Magic token to include in URL
    ///
    /// # Returns
    /// * `String` - Complete magic link URL
    pub fn create_magic_link_url(host_url: &str, magic_token: &str) -> String {
        format!(
            "{}/?magiclink={}",
            host_url.trim_end_matches('/'),
            magic_token
        )
    }

    /// Extract host URL from request for magic link generation
    ///
    /// # Arguments
    /// * `req` - HTTP request to extract host from
    ///
    /// # Returns
    /// * `String` - Host URL (e.g., "https://example.com" or "http://localhost:5173")
    pub fn get_host_url_from_request(req: &spin_sdk::http::Request) -> String {
        // Try to get host from headers
        let host = req
            .header("host")
            .and_then(|h| h.as_str())
            .unwrap_or("localhost:5173");

        // Check if it's a development host
        let scheme = if host.contains("localhost") || host.contains("127.0.0.1") {
            "http"
        } else {
            "https"
        };

        format!("{}://{}", scheme, host)
    }
}
