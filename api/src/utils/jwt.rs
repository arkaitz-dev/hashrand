//! JWT token utilities for authentication
//!
//! Provides functions for creating and validating JWT access and refresh tokens
//! with proper expiration times and security claims.

use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
// use pbkdf2::pbkdf2; // Replaced with Argon2id
use argon2::{password_hash::{PasswordHasher, SaltString}, Argon2, Algorithm as Argon2Algorithm, Version as Argon2Version, Params};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256, Shake256, digest::{ExtendableOutput, Update}};
use rand_chacha::ChaCha8Rng;
use rand::{SeedableRng, RngCore};
use chacha20::{ChaCha20, cipher::{KeyIvInit, StreamCipher}};

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
    /// Argon2id parameters for current security standards (2024)
    /// Fixed parameters as requested: mem_cost=19456, time_cost=2, lane=1, hash_length=32
    const ARGON2_MEM_COST: u32 = 19456; // Memory usage in KB
    const ARGON2_TIME_COST: u32 = 2;    // Number of iterations
    const ARGON2_LANES: u32 = 1;        // Parallelism parameter
    const ARGON2_HASH_LENGTH: usize = 32; // Output length in bytes

    // /// Salt for PBKDF2 derivation (should be from environment in production)
    // const PBKDF2_SALT: &'static [u8] = b"hashrand-user-derivation-salt-v1";

    // /// HMAC key for magic link integrity (should be from environment in production)
    // const MAGIC_LINK_HMAC_KEY: &'static [u8] = b"hashrand-magic-link-hmac-key-v1";

    /// Derive secure user ID from email using SHA3-256 + HMAC + Argon2id + SHAKE3
    ///
    /// Enhanced security process with Argon2id:
    /// 1. SHA3-256(email) → 32 bytes
    /// 2. HMAC-SHA3-256(sha3_result, hmac_key) → 32 bytes  
    /// 3. Generate dynamic salt: HMAC-SHA3-256(fixed_salt, email_hash) → ChaCha8Rng[32 bytes] → salt
    /// 4. Argon2id(data=email_hash, salt=dynamic_salt, mem_cost=19456, time_cost=2, lane=1) → 32 bytes
    /// 5. SHAKE256(argon2_result) → 16 bytes user_id
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
        let mut mac = <Hmac<Sha3_256> as Mac>::new_from_slice(&hmac_key)
            .map_err(|_| "Invalid USER_ID_HMAC_KEY format".to_string())?;
        Mac::update(&mut mac, &email_hash);
        let _hmac_result = mac.finalize().into_bytes();

        // Step 3: Generate dynamic salt using HMAC + ChaCha8Rng
        let dynamic_salt = Self::generate_dynamic_salt(&email_hash)?;
        
        // Step 4: Argon2id with fixed parameters
        let argon2_output = Self::derive_with_argon2id(&email_hash, &dynamic_salt)?;

        // Step 5: SHAKE256 to compress to 16 bytes with better entropy distribution
        let mut shake = Shake256::default();
        Update::update(&mut shake, &argon2_output);
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

    /// Get Argon2id salt from Spin variables as bytes
    ///
    /// # Returns  
    /// * `Result<Vec<u8>, String>` - Salt bytes or error message
    fn get_argon2_salt() -> Result<Vec<u8>, String> {
        let salt_hex = spin_sdk::variables::get("argon2_salt")
            .map_err(|e| format!("Failed to get argon2_salt variable: {}", e))?;

        hex::decode(&salt_hex).map_err(|_| "ARGON2_SALT must be a valid hex string".to_string())
    }

    /// Generate dynamic salt using HMAC-SHA3-256 → ChaCha8Rng → salt bytes
    /// 
    /// Process: fixed_salt → HMAC-SHA3-256(fixed_salt, data) → ChaCha8Rng[32 bytes] → salt
    ///
    /// # Arguments
    /// * `data` - Data to derive salt from (typically email hash)
    ///
    /// # Returns
    /// * `Result<[u8; 32], String>` - 32-byte dynamic salt
    fn generate_dynamic_salt(data: &[u8]) -> Result<[u8; 32], String> {
        let fixed_salt = Self::get_argon2_salt()?;
        
        // Generate HMAC-SHA3-256(fixed_salt, data)
        let mut mac = <Hmac<Sha3_256> as Mac>::new_from_slice(&fixed_salt)
            .map_err(|_| "Invalid ARGON2_SALT format for HMAC".to_string())?;
        Mac::update(&mut mac, data);
        let hmac_result = mac.finalize().into_bytes();

        // Use HMAC result as seed for ChaCha8Rng
        let mut chacha_seed = [0u8; 32];
        chacha_seed.copy_from_slice(&hmac_result[..32]);
        
        // Generate 32 bytes using ChaCha8Rng
        let mut rng = ChaCha8Rng::from_seed(chacha_seed);
        let mut dynamic_salt = [0u8; 32];
        rng.fill_bytes(&mut dynamic_salt);
        
        Ok(dynamic_salt)
    }
    
    /// Derive key using Argon2id with fixed parameters
    ///
    /// # Arguments
    /// * `data` - Input data to hash (email hash)
    /// * `salt` - Salt bytes for Argon2id
    ///
    /// # Returns
    /// * `Result<[u8; 32], String>` - 32-byte Argon2id output
    fn derive_with_argon2id(data: &[u8], salt: &[u8; 32]) -> Result<[u8; 32], String> {
        // Create Argon2id instance with fixed parameters
        let params = Params::new(
            Self::ARGON2_MEM_COST,
            Self::ARGON2_TIME_COST,
            Self::ARGON2_LANES,
            Some(Self::ARGON2_HASH_LENGTH)
        ).map_err(|e| format!("Invalid Argon2id parameters: {}", e))?;
        
        let argon2 = Argon2::new(Argon2Algorithm::Argon2id, Argon2Version::V0x13, params);
        
        // Create salt string for argon2 crate
        let salt_string = SaltString::encode_b64(salt)
            .map_err(|e| format!("Failed to encode salt: {}", e))?;
        
        // Hash the data with Argon2id
        let password_hash = argon2.hash_password(data, &salt_string)
            .map_err(|e| format!("Argon2id hashing failed: {}", e))?;
        let hash_string = password_hash.to_string();

        // Extract hash part after last '$' and decode from base64
        let hash_parts: Vec<&str> = hash_string.split('$').collect();
        if hash_parts.len() < 6 {
            return Err("Invalid Argon2id hash format".to_string());
        }
        
        let base64_hash = hash_parts[hash_parts.len() - 1];
        
        // Decode base64 to get raw bytes (Argon2 uses base64 without padding)
        let decoded_hash = general_purpose::STANDARD_NO_PAD.decode(base64_hash)
            .map_err(|e| format!("Base64 decode failed: {}", e))?;

        // Convert to [u8; 32]
        if decoded_hash.len() != 32 {
            return Err(format!("Expected 32 bytes, got {}", decoded_hash.len()));
        }
        
        let mut final_result = [0u8; 32];
        final_result.copy_from_slice(&decoded_hash);
        
        Ok(final_result)
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

    /// Get ChaCha20-Poly1305 encryption key from Spin variables as bytes
    ///
    /// # Returns
    /// * `Result<Vec<u8>, String>` - Encryption key bytes or error message
    fn get_chacha_encryption_key() -> Result<Vec<u8>, String> {
        let key_hex = spin_sdk::variables::get("chacha_encryption_key")
            .map_err(|e| format!("Failed to get chacha_encryption_key variable: {}", e))?;

        hex::decode(&key_hex)
            .map_err(|_| "CHACHA_ENCRYPTION_KEY must be a valid hex string".to_string())
    }

    /// Generate nonce and secret key from HMAC-SHA3-256 → ChaCha8RNG[44]
    ///
    /// Process: HMAC-SHA3-256(raw_magic_link, chacha_key) → ChaCha8RNG[44] → nonce[12] + secret_key[32]
    ///
    /// # Arguments
    /// * `raw_magic_link` - 32-byte raw magic link data
    ///
    /// # Returns
    /// * `Result<([u8; 12], [u8; 32]), String>` - (nonce, secret_key) or error
    fn generate_chacha_nonce_and_key(raw_magic_link: &[u8; 32]) -> Result<([u8; 12], [u8; 32]), String> {
        // Get ChaCha encryption key
        let chacha_key = Self::get_chacha_encryption_key()?;
        
        // Generate HMAC-SHA3-256(raw_magic_link, chacha_key)
        let mut mac = <Hmac<Sha3_256> as Mac>::new_from_slice(&chacha_key)
            .map_err(|_| "Invalid ChaCha encryption key format".to_string())?;
        Mac::update(&mut mac, raw_magic_link);
        let hmac_result = mac.finalize().into_bytes();

        // Use HMAC result as seed for ChaCha8Rng
        let mut chacha_seed = [0u8; 32];
        chacha_seed.copy_from_slice(&hmac_result[..32]);
        
        // Generate 44 bytes using ChaCha8Rng: nonce[12] + secret_key[32]
        let mut rng = ChaCha8Rng::from_seed(chacha_seed);
        let mut combined_data = [0u8; 44];
        rng.fill_bytes(&mut combined_data);
        
        // Extract nonce and secret_key
        let mut nonce = [0u8; 12];
        let mut secret_key = [0u8; 32];
        nonce.copy_from_slice(&combined_data[..12]);
        secret_key.copy_from_slice(&combined_data[12..44]);
        
        Ok((nonce, secret_key))
    }

    /// Encrypt raw magic link using ChaCha20
    ///
    /// # Arguments
    /// * `raw_magic_link` - 32-byte raw magic link data
    /// * `nonce` - 12-byte nonce for encryption
    /// * `secret_key` - 32-byte secret key for encryption
    ///
    /// # Returns
    /// * `Result<Vec<u8>, String>` - Encrypted data (same size as input)
    fn encrypt_magic_link(
        raw_magic_link: &[u8; 32],
        nonce: &[u8; 12],
        secret_key: &[u8; 32]
    ) -> Result<Vec<u8>, String> {
        let mut cipher = ChaCha20::new(secret_key.into(), nonce.into());
        
        let mut encrypted = raw_magic_link.clone();
        cipher.apply_keystream(&mut encrypted);
        
        Ok(encrypted.to_vec())
    }

    /// Decrypt magic link using ChaCha20
    ///
    /// # Arguments
    /// * `encrypted_data` - Encrypted magic link data
    /// * `nonce` - 12-byte nonce for decryption
    /// * `secret_key` - 32-byte secret key for decryption
    ///
    /// # Returns
    /// * `Result<[u8; 32], String>` - Decrypted raw magic link or error
    fn decrypt_magic_link(
        encrypted_data: &[u8],
        nonce: &[u8; 12],
        secret_key: &[u8; 32]
    ) -> Result<[u8; 32], String> {
        if encrypted_data.len() != 32 {
            return Err(format!("Expected 32 bytes, got {}", encrypted_data.len()));
        }
        
        let mut cipher = ChaCha20::new(secret_key.into(), nonce.into());
        
        let mut decrypted = [0u8; 32];
        decrypted.copy_from_slice(encrypted_data);
        cipher.apply_keystream(&mut decrypted);
        
        Ok(decrypted)
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

    /// Generate secure magic token with ChaCha20-Poly1305 encryption
    ///
    /// Enhanced Process:
    /// 1. Create raw_magic_link: user_id (16) + timestamp (8) + SHAKE256(HMAC-SHA3-256) (8) = 32 bytes
    /// 2. Generate nonce[12] + secret_key[32] from HMAC-SHA3-256(raw_magic_link, chacha_key) → ChaCha8RNG[44]  
    /// 3. Encrypt raw_magic_link with ChaCha20-Poly1305 → new_raw_magic_link
    /// 4. Return Base58(new_raw_magic_link) for transmission + encryption_blob + timestamp for database
    ///
    /// # Arguments
    /// * `email` - User email to derive user_id
    /// * `expires_at` - Magic link expiration timestamp
    ///
    /// # Returns
    /// * `Result<(String, [u8; 44], i64), String>` - (Base58 token, encryption_blob, timestamp) or error
    pub fn generate_magic_token_encrypted(email: &str, expires_at: DateTime<Utc>) -> Result<(String, [u8; 44], i64), String> {
        // Derive deterministic user_id from email
        let user_id = Self::derive_user_id(email)?;

        // Timestamp as nanoseconds since Unix epoch (8 bytes, big-endian u64)
        let timestamp_nanos = expires_at.timestamp_nanos_opt()
            .ok_or("Timestamp overflow in nanoseconds conversion")?;
        let timestamp_bytes = timestamp_nanos.to_be_bytes();

        // Prepare data for HMAC: user_id + timestamp
        let mut data = Vec::with_capacity(24);
        data.extend_from_slice(&user_id);
        data.extend_from_slice(&timestamp_bytes);

        // Generate HMAC-SHA3-256 for integrity
        let hmac_key =
            Self::get_magic_link_hmac_key().map_err(|e| format!("HMAC key error: {}", e))?;
        let mut mac = <Hmac<Sha3_256> as Mac>::new_from_slice(&hmac_key)
            .map_err(|_| "Invalid HMAC key format".to_string())?;
        Mac::update(&mut mac, &data);
        let hmac_result = mac.finalize().into_bytes();

        // Compress HMAC to 8 bytes using SHAKE256
        let mut shake = Shake256::default();
        Update::update(&mut shake, &hmac_result);
        let mut compressed_hmac = [0u8; 8];
        shake.finalize_xof_into(&mut compressed_hmac);

        // Create raw_magic_link: user_id + timestamp + compressed_hmac (32 bytes)
        let mut raw_magic_link = [0u8; 32];
        raw_magic_link[..16].copy_from_slice(&user_id);
        raw_magic_link[16..24].copy_from_slice(&timestamp_bytes);
        raw_magic_link[24..32].copy_from_slice(&compressed_hmac);

        // Generate nonce and secret key from raw_magic_link
        let (nonce, secret_key) = Self::generate_chacha_nonce_and_key(&raw_magic_link)?;

        // Encrypt raw_magic_link with ChaCha20
        let encrypted_data = Self::encrypt_magic_link(&raw_magic_link, &nonce, &secret_key)?;
        
        if encrypted_data.len() != 32 {
            return Err(format!("Expected 32 bytes, got {}", encrypted_data.len()));
        }
        
        // Create encryption_blob: nonce[12] + secret_key[32] = 44 bytes
        let mut encryption_blob = [0u8; 44];
        encryption_blob[..12].copy_from_slice(&nonce);
        encryption_blob[12..44].copy_from_slice(&secret_key);

        // Return encrypted data as Base58 token, encryption_blob, and original timestamp
        Ok((
            bs58::encode(&encrypted_data).into_string(),
            encryption_blob,
            timestamp_nanos as i64
        ))
    }

    /// Validate encrypted magic token using ChaCha20-Poly1305 decryption
    ///
    /// Process:
    /// 1. Decode Base58 encrypted token
    /// 2. Decrypt with ChaCha20-Poly1305 using nonce + secret_key → raw_magic_link
    /// 3. Extract and validate HMAC integrity
    /// 4. Return user_id and timestamp from decrypted data
    ///
    /// # Arguments
    /// * `encrypted_token` - Base58 encoded encrypted magic token
    /// * `nonce` - 12-byte nonce from encryption_blob
    /// * `secret_key` - 32-byte secret key from encryption_blob
    ///
    /// # Returns
    /// * `Result<([u8; 16], DateTime<Utc>), String>` - (user_id, expiration) or validation error
    pub fn validate_magic_token_encrypted(
        encrypted_token: &str,
        nonce: &[u8; 12],
        secret_key: &[u8; 32]
    ) -> Result<([u8; 16], DateTime<Utc>), String> {
        // Decode Base58 encrypted token
        let encrypted_data = bs58::decode(encrypted_token)
            .into_vec()
            .map_err(|_| "Invalid Base58 encoding")?;

        // Decrypt with ChaCha20-Poly1305
        let raw_magic_link = Self::decrypt_magic_link(&encrypted_data, nonce, secret_key)?;

        // Extract components from decrypted raw_magic_link
        let user_id_bytes = &raw_magic_link[0..16];
        let timestamp_bytes = &raw_magic_link[16..24];
        let provided_compressed_hmac = &raw_magic_link[24..32];

        // Verify HMAC integrity
        let hmac_key = Self::get_magic_link_hmac_key()
            .map_err(|e| format!("HMAC key error: {}", e))?;
        let mut mac = <Hmac<Sha3_256> as Mac>::new_from_slice(&hmac_key)
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

            let expires_at = DateTime::from_timestamp_nanos(timestamp as i64);

            // Convert user_id bytes to array
            let mut user_id = [0u8; 16];
            user_id.copy_from_slice(user_id_bytes);

            Ok((user_id, expires_at))
        } else {
            Err("Token integrity verification failed".to_string())
        }
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
        let mut mac = <Hmac<Sha3_256> as Mac>::new_from_slice(&hmac_key)
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

            let expires_at = DateTime::from_timestamp_nanos(timestamp as i64);

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
    ///
    /// # Returns
    /// * `String` - Complete magic link URL
    pub fn create_magic_link_url(host_url: &str, magic_token: &str) -> String {
        let base_url = host_url.trim_end_matches('/');
        format!("{}/?magiclink={}", base_url, magic_token)
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
