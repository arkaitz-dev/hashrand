//! JWT token utilities for authentication
//!
//! Provides functions for creating and validating JWT access and refresh tokens
//! with proper expiration times and security claims.

use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use pbkdf2::pbkdf2;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256, Shake256, digest::{ExtendableOutput, Update}};

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

    /// Derive secure user ID from email using SHA3-256 + HMAC + PBKDF2 + SHAKE3
    ///
    /// Enhanced security process:
    /// 1. SHA3-256(email) → 32 bytes
    /// 2. HMAC-SHA3-256(sha3_result, hmac_key) → 32 bytes
    /// 3. Derive unique per-user salt: HMAC-SHA3-256(email, global_salt) → 32 bytes
    /// 4. PBKDF2-SHA3-256(hmac_result, user_salt, 600k iterations) → 32 bytes
    /// 5. SHAKE256(pbkdf2_result) → 16 bytes user_id
    ///
    /// # Arguments
    /// * `email` - User email address
    ///
    /// # Returns
    /// * `Result<[u8; 16], String>` - 128-bit deterministic user ID or error
    pub fn derive_user_id(email: &str) -> Result<[u8; 16], String> {
        // Step 1: SHA3-256 hash of email
        let mut hasher = Sha3_256::new();
        Digest::update(&mut hasher, email.to_lowercase().trim().as_bytes());
        let email_hash = hasher.finalize();

        // Step 2: HMAC-SHA3-256 of the email hash
        let hmac_key = Self::get_user_id_hmac_key()?;
        let mut mac = Hmac::<Sha3_256>::new_from_slice(&hmac_key)
            .map_err(|_| "Invalid USER_ID_HMAC_KEY format".to_string())?;
        Mac::update(&mut mac, &email_hash);
        let hmac_result = mac.finalize().into_bytes();

        // Step 3: PBKDF2-SHA3-256 with unique per-user salt and high iteration count
        let mut pbkdf2_output = [0u8; 32];
        let user_salt = Self::derive_user_salt(&email_hash)?;
        pbkdf2::<Hmac<Sha3_256>>(&hmac_result, &user_salt, Self::PBKDF2_ITERATIONS, &mut pbkdf2_output)
            .map_err(|_| "PBKDF2 derivation failed".to_string())?;

        // Step 4: SHAKE256 to compress to 16 bytes with better entropy distribution
        let mut shake = Shake256::default();
        Update::update(&mut shake, &pbkdf2_output);
        let mut user_id = [0u8; 16];
        shake.finalize_xof_into(&mut user_id);

        Ok(user_id)
    }

    /// Convert user ID to Base58 username for display/API
    ///
    /// # Arguments
    /// * `user_id` - 16-byte user ID
    ///
    /// # Returns
    /// * `String` - Base58 encoded username (~22 characters)
    pub fn user_id_to_username(user_id: &[u8; 16]) -> String {
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
    // /// Get JWT secret key (in production this should be from environment variable)
    // /// For development, we generate a consistent secret
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

        hex::decode(&salt_hex).map_err(|_| "PBKDF2_SALT must be a valid hex string".to_string())
    }

    /// Generate unique salt per user using HMAC-SHA3-256(email_hash, global_salt)
    ///
    /// # Arguments
    /// * `email_hash` - SHA3-256 hash of user email address
    ///
    /// # Returns
    /// * `Result<[u8; 32], String>` - 32-byte unique salt for this user
    fn derive_user_salt(email_hash: &[u8]) -> Result<[u8; 32], String> {
        let global_salt = Self::get_pbkdf2_salt()?;
        let mut mac = Hmac::<Sha3_256>::new_from_slice(&global_salt)
            .map_err(|_| "Invalid PBKDF2_SALT format for HMAC".to_string())?;
        Mac::update(&mut mac, email_hash);
        let hmac_result = mac.finalize().into_bytes();

        let mut user_salt = [0u8; 32];
        user_salt.copy_from_slice(&hmac_result[..32]);
        Ok(user_salt)
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

    /// Get user ID HMAC key from Spin variables as bytes
    ///
    /// # Returns
    /// * `Result<Vec<u8>, String>` - HMAC key bytes or error message
    fn get_user_id_hmac_key() -> Result<Vec<u8>, String> {
        let key_hex = spin_sdk::variables::get("user_id_hmac_key")
            .map_err(|e| format!("Failed to get user_id_hmac_key variable: {}", e))?;

        hex::decode(&key_hex)
            .map_err(|_| "USER_ID_HMAC_KEY must be a valid hex string".to_string())
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
        let jwt_secret = Self::get_jwt_secret().map_err(|e| format!("JWT secret error: {}", e))?;
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
        let jwt_secret = Self::get_jwt_secret().map_err(|e| format!("JWT secret error: {}", e))?;
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
    /// Format: user_id (16 bytes) + timestamp (8 bytes) + SHAKE256(HMAC-SHA3-256) (8 bytes) = 32 bytes
    /// Encoded in Base58 for email transmission (~44 characters)
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
        let mut data = Vec::with_capacity(24);
        data.extend_from_slice(&user_id);
        data.extend_from_slice(&timestamp_bytes);

        // Generate HMAC-SHA3-256 for integrity
        let hmac_key =
            Self::get_magic_link_hmac_key().map_err(|e| format!("HMAC key error: {}", e))?;
        let mut mac = Hmac::<Sha3_256>::new_from_slice(&hmac_key)
            .map_err(|_| "Invalid HMAC key format".to_string())?;
        Mac::update(&mut mac, &data);
        let hmac_result = mac.finalize().into_bytes();

        // Compress HMAC to 8 bytes using SHAKE256
        let mut shake = Shake256::default();
        Update::update(&mut shake, &hmac_result);
        let mut compressed_hmac = [0u8; 8];
        shake.finalize_xof_into(&mut compressed_hmac);

        // Final token: user_id + timestamp + compressed_hmac (16 + 8 + 8 = 32 bytes)
        let mut token = Vec::with_capacity(32);
        token.extend_from_slice(&user_id);
        token.extend_from_slice(&timestamp_bytes);
        token.extend_from_slice(&compressed_hmac);

        Ok(bs58::encode(&token).into_string())
    }

    /// Validate magic token and extract user_id and expiration timestamp
    ///
    /// # Arguments
    /// * `magic_token` - Base58 encoded magic token
    ///
    /// # Returns
    /// * `Result<([u8; 16], DateTime<Utc>), String>` - (user_id, expiration) or validation error
    pub fn validate_magic_token(magic_token: &str) -> Result<([u8; 16], DateTime<Utc>), String> {
        // Decode Base58 token
        let token_bytes = bs58::decode(magic_token)
            .into_vec()
            .map_err(|_| "Invalid Base58 encoding")?;

        // Verify token length (16 + 8 + 8 = 32 bytes)
        if token_bytes.len() != 32 {
            return Err("Invalid token length".to_string());
        }

        // Extract components
        let user_id_bytes = &token_bytes[0..16];
        let timestamp_bytes = &token_bytes[16..24];
        let provided_compressed_hmac = &token_bytes[24..32];

        // Verify HMAC integrity with compression
        let hmac_key =
            Self::get_magic_link_hmac_key().map_err(|e| format!("HMAC key error: {}", e))?;
        let mut mac = Hmac::<Sha3_256>::new_from_slice(&hmac_key)
            .map_err(|_| "Invalid HMAC key format".to_string())?;
        Mac::update(&mut mac, user_id_bytes);
        Mac::update(&mut mac, timestamp_bytes);
        let hmac_result = mac.finalize().into_bytes();

        // Compress HMAC to 8 bytes using SHAKE256 (same as generation)
        let mut shake = Shake256::default();
        Update::update(&mut shake, &hmac_result);
        let mut expected_compressed_hmac = [0u8; 8];
        shake.finalize_xof_into(&mut expected_compressed_hmac);

        // Compare compressed HMAC values
        if provided_compressed_hmac == expected_compressed_hmac {
            // Extract timestamp
            let timestamp = u64::from_be_bytes(
                timestamp_bytes
                    .try_into()
                    .map_err(|_| "Invalid timestamp format")?,
            );

            let expires_at = DateTime::from_timestamp(timestamp as i64, 0)
                .ok_or_else(|| "Invalid timestamp value".to_string())?;

            // Convert user_id bytes to array
            let mut user_id = [0u8; 16];
            user_id.copy_from_slice(user_id_bytes);

            Ok((user_id, expires_at))
        } else {
            Err("Token integrity verification failed".to_string())
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
