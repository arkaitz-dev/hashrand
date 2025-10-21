//! Configuration and secrets management
//!
//! Handles retrieval of environment secrets and configuration values from Spin variables.

use spin_sdk::variables;

/// Generic hex configuration retrieval - DRY consolidation
///
/// Retrieves a hex-encoded configuration value from Spin variables and converts to byte array
///
/// # Type Parameters
/// * `N` - Size of the byte array to return
///
/// # Arguments
/// * `var_name` - Name of the Spin variable to retrieve
/// * `display_name` - User-friendly name for error messages
///
/// # Returns
/// * `Result<[u8; N], String>` - Byte array of size N or error message
fn get_config_bytes<const N: usize>(var_name: &str, display_name: &str) -> Result<[u8; N], String> {
    let hex_value = variables::get(var_name)
        .map_err(|e| format!("Failed to get {} variable: {}", var_name, e))?;

    let decoded = hex::decode(&hex_value)
        .map_err(|_| format!("{} must be a valid hex string", display_name))?;

    if decoded.len() != N {
        return Err(format!(
            "{} must be exactly {} bytes, got {}",
            display_name,
            N,
            decoded.len()
        ));
    }

    let mut result = [0u8; N];
    result.copy_from_slice(&decoded);
    Ok(result)
}

/// Get JWT secret from Spin variables
///
/// # Returns
/// * `Result<String, String>` - JWT secret or error message
#[allow(dead_code)]
pub fn get_jwt_secret() -> Result<String, String> {
    variables::get("jwt_secret").map_err(|e| format!("Failed to get jwt_secret variable: {}", e))
}

/// Get Argon2id salt from Spin variables as bytes
///
/// # Returns
/// * `Result<[u8; 64], String>` - 64-byte salt or error message
pub fn get_argon2_salt() -> Result<[u8; 64], String> {
    get_config_bytes("argon2_salt", "ARGON2_SALT")
}

/// Get user ID Argon2 compression key from Spin variables as bytes
///
/// # Returns
/// * `Result<[u8; 64], String>` - 64-byte compression key or error message
pub fn get_user_id_argon2_compression() -> Result<[u8; 64], String> {
    get_config_bytes("user_id_argon2_compression", "USER_ID_ARGON2_COMPRESSION")
}

/// Get magic link HMAC key from Spin variables as bytes
///
/// # Returns
/// * `Result<[u8; 64], String>` - 64-byte HMAC key or error message
pub fn get_magic_link_hmac_key() -> Result<[u8; 64], String> {
    get_config_bytes("magic_link_hmac_key", "MAGIC_LINK_HMAC_KEY")
}

/// Get user ID HMAC key from Spin variables as bytes
///
/// # Returns
/// * `Result<[u8; 64], String>` - 64-byte HMAC key or error message
pub fn get_user_id_hmac_key() -> Result<[u8; 64], String> {
    get_config_bytes("user_id_hmac_key", "USER_ID_HMAC_KEY")
}

/// Get ChaCha20 encryption key from Spin variables as bytes
///
/// # Returns
/// * `Result<[u8; 64], String>` - 64-byte encryption key or error message
pub fn get_chacha_encryption_key() -> Result<[u8; 64], String> {
    get_config_bytes("chacha_encryption_key", "CHACHA_ENCRYPTION_KEY")
}

/// Get encrypted magic link token hash key from Spin variables as bytes
///
/// # Returns
/// * `Result<[u8; 64], String>` - 64-byte hash key or error message
pub fn get_encrypted_mlink_token_hash_key() -> Result<[u8; 64], String> {
    get_config_bytes(
        "encrypted_mlink_token_hash_key",
        "ENCRYPTED_MLINK_TOKEN_HASH_KEY",
    )
}

/// Get magic link payload encryption key from Spin variables as bytes
///
/// # Returns
/// * `Result<[u8; 64], String>` - 64-byte encryption key or error message
pub fn get_mlink_content_key() -> Result<[u8; 64], String> {
    get_config_bytes("mlink_content", "MLINK_CONTENT")
}

// Custom Token Security Keys

/// Get access token cipher key from Spin variables as bytes (64 bytes required)
pub fn get_access_token_cipher_key() -> Result<[u8; 64], String> {
    get_config_bytes("access_token_cipher_key", "ACCESS_TOKEN_CIPHER_KEY")
}

/// Get access token nonce key from Spin variables as bytes (64 bytes required)
pub fn get_access_token_nonce_key() -> Result<[u8; 64], String> {
    get_config_bytes("access_token_nonce_key", "ACCESS_TOKEN_NONCE_KEY")
}

/// Get access token HMAC key from Spin variables as bytes (64 bytes required)
pub fn get_access_token_hmac_key() -> Result<[u8; 64], String> {
    get_config_bytes("access_token_hmac_key", "ACCESS_TOKEN_HMAC_KEY")
}

/// Get refresh token cipher key from Spin variables as bytes (64 bytes required)
#[allow(dead_code)]
pub fn get_refresh_token_cipher_key() -> Result<[u8; 64], String> {
    get_config_bytes("refresh_token_cipher_key", "REFRESH_TOKEN_CIPHER_KEY")
}

/// Get refresh token nonce key from Spin variables as bytes (64 bytes required)
#[allow(dead_code)]
pub fn get_refresh_token_nonce_key() -> Result<[u8; 64], String> {
    get_config_bytes("refresh_token_nonce_key", "REFRESH_TOKEN_NONCE_KEY")
}

/// Get refresh token HMAC key from Spin variables as bytes (64 bytes required)
#[allow(dead_code)]
pub fn get_refresh_token_hmac_key() -> Result<[u8; 64], String> {
    get_config_bytes("refresh_token_hmac_key", "REFRESH_TOKEN_HMAC_KEY")
}

/// Get prehash cipher key from Spin variables as bytes (64 bytes required)
pub fn get_prehash_cipher_key() -> Result<[u8; 64], String> {
    get_config_bytes("prehash_cipher_key", "PREHASH_CIPHER_KEY")
}

/// Get prehash nonce key from Spin variables as bytes (64 bytes required)
pub fn get_prehash_nonce_key() -> Result<[u8; 64], String> {
    get_config_bytes("prehash_nonce_key", "PREHASH_NONCE_KEY")
}

/// Get prehash HMAC key from Spin variables as bytes (64 bytes required)
pub fn get_prehash_hmac_key() -> Result<[u8; 64], String> {
    get_config_bytes("prehash_hmac_key", "PREHASH_HMAC_KEY")
}

/// Get access token duration in minutes from Spin variables
pub fn get_access_token_duration_minutes() -> Result<u64, String> {
    let duration_str = variables::get("access_token_duration_minutes").map_err(|e| {
        format!(
            "Failed to get access_token_duration_minutes variable: {}",
            e
        )
    })?;

    duration_str
        .parse::<u64>()
        .map_err(|_| "ACCESS_TOKEN_DURATION_MINUTES must be a valid number".to_string())
}

/// Get refresh token duration in minutes from Spin variables
pub fn get_refresh_token_duration_minutes() -> Result<u64, String> {
    let duration_str = variables::get("refresh_token_duration_minutes").map_err(|e| {
        format!(
            "Failed to get refresh_token_duration_minutes variable: {}",
            e
        )
    })?;

    duration_str
        .parse::<u64>()
        .map_err(|_| "REFRESH_TOKEN_DURATION_MINUTES must be a valid number".to_string())
}

// Shared Secret Security Keys

/// Get shared secret URL cipher key from Spin variables as bytes (64 bytes required)
pub fn get_shared_secret_url_cipher_key() -> Result<[u8; 64], String> {
    get_config_bytes(
        "shared_secret_url_cipher_key",
        "SHARED_SECRET_URL_CIPHER_KEY",
    )
}

/// Get shared secret content encryption key from Spin variables as bytes (64 bytes required)
pub fn get_shared_secret_content_key() -> Result<[u8; 64], String> {
    get_config_bytes("shared_secret_content_key", "SHARED_SECRET_CONTENT_KEY")
}

/// Get shared secret checksum key from Spin variables as bytes (64 bytes required)
pub fn get_shared_secret_checksum_key() -> Result<[u8; 64], String> {
    get_config_bytes("shared_secret_checksum_key", "SHARED_SECRET_CHECKSUM_KEY")
}

/// Get shared secret database index key from Spin variables as bytes (64 bytes required)
pub fn get_shared_secret_db_index_key() -> Result<[u8; 64], String> {
    get_config_bytes("shared_secret_db_index_key", "SHARED_SECRET_DB_INDEX_KEY")
}

// User Private Key Context Security Keys

/// Get user private key context index key from Spin variables as bytes (64 bytes required)
pub fn get_user_privkey_index_key() -> Result<[u8; 64], String> {
    get_config_bytes("user_privkey_index_key", "USER_PRIVKEY_INDEX_KEY")
}

/// Get user private key context encryption key from Spin variables as bytes (64 bytes required)
pub fn get_user_privkey_encryption_key() -> Result<[u8; 64], String> {
    get_config_bytes("user_privkey_encryption_key", "USER_PRIVKEY_ENCRYPTION_KEY")
}
