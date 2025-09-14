//! Configuration and secrets management
//!
//! Handles retrieval of environment secrets and configuration values from Spin variables.

use spin_sdk::variables;

/// Get JWT secret from Spin variables
///
/// # Returns
/// * `Result<String, String>` - JWT secret or error message
pub fn get_jwt_secret() -> Result<String, String> {
    variables::get("jwt_secret").map_err(|e| format!("Failed to get jwt_secret variable: {}", e))
}

/// Get Argon2id salt from Spin variables as bytes
///
/// # Returns  
/// * `Result<Vec<u8>, String>` - Salt bytes or error message
pub fn get_argon2_salt() -> Result<Vec<u8>, String> {
    let salt_hex = variables::get("argon2_salt")
        .map_err(|e| format!("Failed to get argon2_salt variable: {}", e))?;

    hex::decode(&salt_hex).map_err(|_| "ARGON2_SALT must be a valid hex string".to_string())
}

/// Get magic link HMAC key from Spin variables as bytes
///
/// # Returns
/// * `Result<Vec<u8>, String>` - HMAC key bytes or error message  
pub fn get_magic_link_hmac_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("magic_link_hmac_key")
        .map_err(|e| format!("Failed to get magic_link_hmac_key variable: {}", e))?;

    hex::decode(&key_hex).map_err(|_| "MAGIC_LINK_HMAC_KEY must be a valid hex string".to_string())
}

/// Get user ID HMAC key from Spin variables as bytes
///
/// # Returns
/// * `Result<Vec<u8>, String>` - HMAC key bytes or error message
pub fn get_user_id_hmac_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("user_id_hmac_key")
        .map_err(|e| format!("Failed to get user_id_hmac_key variable: {}", e))?;

    hex::decode(&key_hex).map_err(|_| "USER_ID_HMAC_KEY must be a valid hex string".to_string())
}

/// Get ChaCha20-Poly1305 encryption key from Spin variables as bytes
///
/// # Returns
/// * `Result<Vec<u8>, String>` - Encryption key bytes or error message
pub fn get_chacha_encryption_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("chacha_encryption_key")
        .map_err(|e| format!("Failed to get chacha_encryption_key variable: {}", e))?;

    hex::decode(&key_hex)
        .map_err(|_| "CHACHA_ENCRYPTION_KEY must be a valid hex string".to_string())
}

// Custom Token Security Keys

/// Get access token cipher key from Spin variables as bytes
pub fn get_access_token_cipher_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("access_token_cipher_key")
        .map_err(|e| format!("Failed to get access_token_cipher_key variable: {}", e))?;
    hex::decode(&key_hex)
        .map_err(|_| "ACCESS_TOKEN_CIPHER_KEY must be a valid hex string".to_string())
}

/// Get access token nonce key from Spin variables as bytes
pub fn get_access_token_nonce_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("access_token_nonce_key")
        .map_err(|e| format!("Failed to get access_token_nonce_key variable: {}", e))?;
    hex::decode(&key_hex)
        .map_err(|_| "ACCESS_TOKEN_NONCE_KEY must be a valid hex string".to_string())
}

/// Get access token HMAC key from Spin variables as bytes
pub fn get_access_token_hmac_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("access_token_hmac_key")
        .map_err(|e| format!("Failed to get access_token_hmac_key variable: {}", e))?;
    hex::decode(&key_hex)
        .map_err(|_| "ACCESS_TOKEN_HMAC_KEY must be a valid hex string".to_string())
}

/// Get refresh token cipher key from Spin variables as bytes
pub fn get_refresh_token_cipher_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("refresh_token_cipher_key")
        .map_err(|e| format!("Failed to get refresh_token_cipher_key variable: {}", e))?;
    hex::decode(&key_hex)
        .map_err(|_| "REFRESH_TOKEN_CIPHER_KEY must be a valid hex string".to_string())
}

/// Get refresh token nonce key from Spin variables as bytes
pub fn get_refresh_token_nonce_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("refresh_token_nonce_key")
        .map_err(|e| format!("Failed to get refresh_token_nonce_key variable: {}", e))?;
    hex::decode(&key_hex)
        .map_err(|_| "REFRESH_TOKEN_NONCE_KEY must be a valid hex string".to_string())
}

/// Get refresh token HMAC key from Spin variables as bytes
pub fn get_refresh_token_hmac_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("refresh_token_hmac_key")
        .map_err(|e| format!("Failed to get refresh_token_hmac_key variable: {}", e))?;
    hex::decode(&key_hex)
        .map_err(|_| "REFRESH_TOKEN_HMAC_KEY must be a valid hex string".to_string())
}

/// Get prehash cipher key from Spin variables as bytes
pub fn get_prehash_cipher_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("prehash_cipher_key")
        .map_err(|e| format!("Failed to get prehash_cipher_key variable: {}", e))?;
    hex::decode(&key_hex)
        .map_err(|_| "PREHASH_CIPHER_KEY must be a valid hex string".to_string())
}

/// Get prehash nonce key from Spin variables as bytes
pub fn get_prehash_nonce_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("prehash_nonce_key")
        .map_err(|e| format!("Failed to get prehash_nonce_key variable: {}", e))?;
    hex::decode(&key_hex)
        .map_err(|_| "PREHASH_NONCE_KEY must be a valid hex string".to_string())
}

/// Get prehash HMAC key from Spin variables as bytes
pub fn get_prehash_hmac_key() -> Result<Vec<u8>, String> {
    let key_hex = variables::get("prehash_hmac_key")
        .map_err(|e| format!("Failed to get prehash_hmac_key variable: {}", e))?;
    hex::decode(&key_hex)
        .map_err(|_| "PREHASH_HMAC_KEY must be a valid hex string".to_string())
}

/// Get access token duration in minutes from Spin variables
pub fn get_access_token_duration_minutes() -> Result<u64, String> {
    let duration_str = variables::get("access_token_duration_minutes")
        .map_err(|e| format!("Failed to get access_token_duration_minutes variable: {}", e))?;

    duration_str.parse::<u64>()
        .map_err(|_| "ACCESS_TOKEN_DURATION_MINUTES must be a valid number".to_string())
}

/// Get refresh token duration in minutes from Spin variables
pub fn get_refresh_token_duration_minutes() -> Result<u64, String> {
    let duration_str = variables::get("refresh_token_duration_minutes")
        .map_err(|e| format!("Failed to get refresh_token_duration_minutes variable: {}", e))?;

    duration_str.parse::<u64>()
        .map_err(|_| "REFRESH_TOKEN_DURATION_MINUTES must be a valid number".to_string())
}
