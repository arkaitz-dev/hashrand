//! JWT token utilities for authentication
//!
//! Provides functions for creating and validating JWT access and refresh tokens
//! with proper expiration times and security claims.

use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use pbkdf2::pbkdf2;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

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
    /// Session ID for token revocation
    pub session_id: i64,
}

/// JWT token generation and validation utilities
pub struct JwtUtils;

impl JwtUtils {
    /// PBKDF2 iteration count for current security standards (2024)
    /// Based on OWASP recommendations for password-equivalent security
    const PBKDF2_ITERATIONS: u32 = 600_000;

    // /// Salt for PBKDF2 derivation (should be from environment in production)
    // const PBKDF2_SALT: &'static [u8] = b"hashrand-user-derivation-salt-v1";

    // /// HMAC key for magic link integrity (should be from environment in production)
    // const MAGIC_LINK_HMAC_KEY: &'static [u8] = b"hashrand-magic-link-hmac-key-v1";

    /// Derive secure user ID from email using SHA3-256 + PBKDF2-SHA3-256
    ///
    /// Process:
    /// 1. SHA3-256(email) → 32 bytes
    /// 2. PBKDF2-SHA3-256(hash, salt, 600k iterations) → 32 bytes user_id
    ///
    /// # Arguments
    /// * `email` - User email address
    ///
    /// # Returns
    /// * `Result<[u8; 32], String>` - 256-bit deterministic user ID or error
    pub fn derive_user_id(email: &str) -> Result<[u8; 32], String> {
        // Step 1: SHA3-256 hash of email
        let mut hasher = Sha3_256::new();
        hasher.update(email.to_lowercase().trim().as_bytes());
        let email_hash = hasher.finalize();

        // Step 2: PBKDF2-SHA3-256 with high iteration count
        let mut user_id = [0u8; 32];
        let salt = Self::get_pbkdf2_salt()?;
        
        pbkdf2::<Hmac<Sha3_256>>(
            &email_hash,
            &salt,
            Self::PBKDF2_ITERATIONS,
            &mut user_id,
        )
        .map_err(|_| "PBKDF2 derivation failed".to_string())?;

        Ok(user_id)
    }

    /// Convert user ID to Base58 username for display/API
    ///
    /// # Arguments
    /// * `user_id` - 32-byte user ID
    ///
    /// # Returns
    /// * `String` - Base58 encoded username (~44 characters)
    pub fn user_id_to_username(user_id: &[u8; 32]) -> String {
        bs58::encode(user_id).into_string()
    }

    /// Derive username directly from email (convenience method)
    ///
    /// # Arguments
    /// * `email` - User email address
    ///
    /// # Returns
    /// * `Result<String, String>` - Base58 encoded username or error
    pub fn email_to_username(email: &str) -> Result<String, String> {
        let user_id = Self::derive_user_id(email)?;
        Ok(Self::user_id_to_username(&user_id))
    }
    /// Get JWT secret key (in production this should be from environment variable)
    /// For development, we generate a consistent secret
    // fn get_jwt_secret() -> String {
    //     // In production, this should come from environment variable
    //     // For development, we use a consistent secret
    //     "hashrand-jwt-secret-key-development-only-change-in-production".to_string()
    // }

    /// Get JWT secret from Spin variables
    /// 
    /// # Returns
    /// * `Result<String, String>` - JWT secret or error message
    fn get_jwt_secret() -> Result<String, String> {
        spin_sdk::variables::get("jwt_secret")
            .map_err(|e| format!("Failed to get jwt_secret variable: {}", e))
    }

    /// Get PBKDF2 salt from Spin variables as bytes
    /// 
    /// # Returns  
    /// * `Result<Vec<u8>, String>` - Salt bytes or error message
    fn get_pbkdf2_salt() -> Result<Vec<u8>, String> {
        let salt_hex = spin_sdk::variables::get("pbkdf2_salt")
            .map_err(|e| format!("Failed to get pbkdf2_salt variable: {}", e))?;
        
        hex::decode(&salt_hex)
            .map_err(|_| "PBKDF2_SALT must be a valid hex string".to_string())
    }

    /// Get magic link HMAC key from Spin variables as bytes
    /// 
    /// # Returns
    /// * `Result<Vec<u8>, String>` - HMAC key bytes or error message  
    fn get_magic_link_hmac_key() -> Result<Vec<u8>, String> {
        let key_hex = spin_sdk::variables::get("magic_link_hmac_key")
            .map_err(|e| format!("Failed to get magic_link_hmac_key variable: {}", e))?;
        
        hex::decode(&key_hex)
            .map_err(|_| "MAGIC_LINK_HMAC_KEY must be a valid hex string".to_string())
    }

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
        let username = Self::email_to_username(email)?;

        let claims = AccessTokenClaims {
            sub: username,
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            token_type: "access".to_string(),
        };

        let header = Header::new(Algorithm::HS256);
        let jwt_secret = Self::get_jwt_secret()?;
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
        let username = Self::email_to_username(email)?;

        let claims = RefreshTokenClaims {
            sub: username,
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
            token_type: "refresh".to_string(),
            session_id,
        };

        let header = Header::new(Algorithm::HS256);
        let jwt_secret = Self::get_jwt_secret()?;
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
        let jwt_secret = Self::get_jwt_secret()
            .map_err(|e| format!("JWT secret error: {}", e))?;
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
        let jwt_secret = Self::get_jwt_secret()
            .map_err(|e| format!("JWT secret error: {}", e))?;
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

    /// Generate secure magic token with integrity protection
    ///
    /// Format: user_id (32 bytes) + timestamp (8 bytes) + HMAC-SHA3-256 (32 bytes) = 72 bytes
    /// Encoded in Base58 for email transmission (~98 characters)
    ///
    /// # Arguments
    /// * `email` - User email to derive user_id
    /// * `expires_at` - Magic link expiration timestamp
    ///
    /// # Returns
    /// * `Result<String, String>` - Base58 encoded magic token with integrity protection or error
    pub fn generate_magic_token(email: &str, expires_at: DateTime<Utc>) -> Result<String, String> {
        // Derive deterministic user_id from email
        let user_id = Self::derive_user_id(email)?;

        // Timestamp as 8 bytes (big-endian u64)
        let timestamp = expires_at.timestamp() as u64;
        let timestamp_bytes = timestamp.to_be_bytes();

        // Prepare data for HMAC: user_id + timestamp
        let mut data = Vec::with_capacity(40);
        data.extend_from_slice(&user_id);
        data.extend_from_slice(&timestamp_bytes);

        // Generate HMAC-SHA3-256 for integrity
        let hmac_key = Self::get_magic_link_hmac_key()
            .map_err(|e| format!("HMAC key error: {}", e))?;
        let mut mac = Hmac::<Sha3_256>::new_from_slice(&hmac_key)
            .map_err(|_| "Invalid HMAC key format".to_string())?;
        mac.update(&data);
        let hmac_result = mac.finalize().into_bytes();

        // Final token: user_id + timestamp + hmac (32 + 8 + 32 = 72 bytes)
        let mut token = Vec::with_capacity(72);
        token.extend_from_slice(&user_id);
        token.extend_from_slice(&timestamp_bytes);
        token.extend_from_slice(&hmac_result);

        Ok(bs58::encode(&token).into_string())
    }

    /// Validate magic token and extract user_id and expiration timestamp
    ///
    /// # Arguments
    /// * `magic_token` - Base58 encoded magic token
    ///
    /// # Returns
    /// * `Result<([u8; 32], DateTime<Utc>), String>` - (user_id, expiration) or validation error
    pub fn validate_magic_token(magic_token: &str) -> Result<([u8; 32], DateTime<Utc>), String> {
        // Decode Base58 token
        let token_bytes = bs58::decode(magic_token)
            .into_vec()
            .map_err(|_| "Invalid Base58 encoding")?;

        // Verify token length (32 + 8 + 32 = 72 bytes)
        if token_bytes.len() != 72 {
            return Err("Invalid token length".to_string());
        }

        // Extract components
        let user_id_bytes = &token_bytes[0..32];
        let timestamp_bytes = &token_bytes[32..40];
        let provided_hmac = &token_bytes[40..72];

        // Verify HMAC integrity
        let hmac_key = Self::get_magic_link_hmac_key()
            .map_err(|e| format!("HMAC key error: {}", e))?;
        let mut mac = Hmac::<Sha3_256>::new_from_slice(&hmac_key)
            .map_err(|_| "Invalid HMAC key format".to_string())?;
        mac.update(user_id_bytes);
        mac.update(timestamp_bytes);

        match mac.verify_slice(provided_hmac) {
            Ok(_) => {
                // Extract timestamp
                let timestamp = u64::from_be_bytes(
                    timestamp_bytes
                        .try_into()
                        .map_err(|_| "Invalid timestamp format")?,
                );

                let expires_at = DateTime::from_timestamp(timestamp as i64, 0)
                    .ok_or_else(|| "Invalid timestamp value".to_string())?;

                // Convert user_id bytes to array
                let mut user_id = [0u8; 32];
                user_id.copy_from_slice(user_id_bytes);

                Ok((user_id, expires_at))
            }
            Err(_) => Err("Token integrity verification failed".to_string()),
        }
    }

    /// Create magic link URL for development logging
    ///
    /// # Arguments
    /// * `host_url` - Base URL from request (e.g., "http://localhost:5173")
    /// * `magic_token` - Magic token to include in URL
    /// * `next` - Optional Base58-encoded parameters for post-auth redirect
    ///
    /// # Returns
    /// * `String` - Complete magic link URL
    pub fn create_magic_link_url(host_url: &str, magic_token: &str, next: Option<&str>) -> String {
        let base_url = host_url.trim_end_matches('/');
        match next {
            Some(next_param) => format!(
                "{}/?magiclink={}&next={}",
                base_url, magic_token, next_param
            ),
            None => format!("{}/?magiclink={}", base_url, magic_token),
        }
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
