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
