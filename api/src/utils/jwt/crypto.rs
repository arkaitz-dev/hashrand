//! Cryptographic utilities
//!
//! Provides cryptographic functions for user ID derivation, salt generation,
//! Argon2id hashing, and ChaCha20 encryption/decryption.

use argon2::{
    Algorithm as Argon2Algorithm, Argon2, Params, Version as Argon2Version,
    password_hash::{PasswordHasher, SaltString},
};
use base64::{Engine as _, engine::general_purpose};
use chacha20::{
    ChaCha20,
    cipher::{KeyIvInit, StreamCipher},
};

use super::config::{get_argon2_salt, get_chacha_encryption_key, get_user_id_hmac_key, get_user_id_argon2_compression};
use crate::utils::pseudonimizer::blake3_keyed_variable;

/// Argon2id parameters for current security standards (2024)
/// Fixed parameters as requested: mem_cost=19456, time_cost=2, lane=1, hash_length=32
pub const ARGON2_MEM_COST: u32 = 19456; // Memory usage in KB
pub const ARGON2_TIME_COST: u32 = 2; // Number of iterations
pub const ARGON2_LANES: u32 = 1; // Parallelism parameter
pub const ARGON2_HASH_LENGTH: usize = 32; // Output length in bytes

/// Derive secure user ID from email using Blake3 + Pseudonimizer + Argon2id
///
/// Enhanced security process with Blake3:
/// 1. Blake3 XOF(email) → 64 bytes (paso1)
/// 2. blake3_keyed_variable(paso1[64], hmac_key[64], 32) → 32 bytes (paso2)
///    - paso1[64] meets Blake3 KDF 32-byte minimum (used directly as key_material)
/// 3. Generate dynamic salt: blake3_keyed_variable(argon2_salt[64], paso1[64], 32) → 32 bytes
///    - paso1[64] meets Blake3 KDF 32-byte minimum (used directly as key_material)
/// 4. Argon2id(data=paso2[32], salt=dynamic_salt[32], mem_cost=19456, time_cost=2, lane=1) → 32 bytes
/// 5. blake3_keyed_variable(argon2_result[32], compression_key[64], 16) → 16 bytes user_id
///    - argon2_result[32] meets Blake3 KDF 32-byte minimum (used directly as key_material)
///
/// # Arguments
/// * `email` - User email address
///
/// # Returns
/// * `Result<[u8; 16], String>` - 128-bit deterministic user ID or error
pub fn derive_user_id(email: &str) -> Result<[u8; 16], String> {
    // Step 1: Blake3 XOF of email (64 bytes, no key)
    let mut blake3_hasher = blake3::Hasher::new();
    blake3_hasher.update(email.to_lowercase().trim().as_bytes());
    let mut xof_reader = blake3_hasher.finalize_xof();
    let mut paso1_output = [0u8; 64];
    xof_reader.fill(&mut paso1_output);

    // Step 2: blake3_keyed_variable with 64-byte HMAC key
    let hmac_key = get_user_id_hmac_key()?;
    let paso2_output = blake3_keyed_variable(&hmac_key, &paso1_output, 32);

    let mut hmac_result = [0u8; 32];
    hmac_result.copy_from_slice(&paso2_output);

    // Step 3: Generate dynamic salt using Blake3 pseudonimizer
    let dynamic_salt = generate_dynamic_salt(&paso1_output)?;

    // Step 4: Argon2id with fixed parameters (using Blake3-keyed result as data input)
    let argon2_output = derive_with_argon2id(&hmac_result[..], &dynamic_salt)?;

    // Step 5: Blake3 keyed variable via pseudonimizer to compress to 16 bytes
    let compression_key = get_user_id_argon2_compression()?;
    let user_id_output = blake3_keyed_variable(&compression_key, &argon2_output, 16);

    let mut user_id = [0u8; 16];
    user_id.copy_from_slice(&user_id_output);

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
    let user_id = derive_user_id(email)?;
    Ok(user_id_to_username(&user_id))
}

/// Generate dynamic salt using Blake3 pseudonimizer
///
/// Process: blake3_keyed_variable(argon2_salt[64], data, 32) → salt[32]
///
/// # Arguments
/// * `data` - Data to derive salt from (Blake3 XOF output from email)
///
/// # Returns
/// * `Result<[u8; 32], String>` - 32-byte dynamic salt
pub fn generate_dynamic_salt(data: &[u8]) -> Result<[u8; 32], String> {
    let argon2_salt = get_argon2_salt()?;

    let salt_output = blake3_keyed_variable(&argon2_salt, data, 32);

    let mut dynamic_salt = [0u8; 32];
    dynamic_salt.copy_from_slice(&salt_output);

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
pub fn derive_with_argon2id(data: &[u8], salt: &[u8; 32]) -> Result<[u8; 32], String> {
    // Create Argon2id instance with fixed parameters
    let params = Params::new(
        ARGON2_MEM_COST,
        ARGON2_TIME_COST,
        ARGON2_LANES,
        Some(ARGON2_HASH_LENGTH),
    )
    .map_err(|e| format!("Invalid Argon2id parameters: {}", e))?;

    let argon2 = Argon2::new(Argon2Algorithm::Argon2id, Argon2Version::V0x13, params);

    // Create salt string for argon2 crate
    let salt_string =
        SaltString::encode_b64(salt).map_err(|e| format!("Failed to encode salt: {}", e))?;

    // Hash the data with Argon2id
    let password_hash = argon2
        .hash_password(data, &salt_string)
        .map_err(|e| format!("Argon2id hashing failed: {}", e))?;
    let hash_string = password_hash.to_string();

    // Extract hash part after last '$' and decode from base64
    let hash_parts: Vec<&str> = hash_string.split('$').collect();
    if hash_parts.len() < 6 {
        return Err("Invalid Argon2id hash format".to_string());
    }

    let base64_hash = hash_parts[hash_parts.len() - 1];

    // Decode base64 to get raw bytes (Argon2 uses base64 without padding)
    let decoded_hash = general_purpose::STANDARD_NO_PAD
        .decode(base64_hash)
        .map_err(|e| format!("Base64 decode failed: {}", e))?;

    // Convert to [u8; 32]
    if decoded_hash.len() != 32 {
        return Err(format!("Expected 32 bytes, got {}", decoded_hash.len()));
    }

    let mut final_result = [0u8; 32];
    final_result.copy_from_slice(&decoded_hash);

    Ok(final_result)
}

/// Generate nonce and secret key using Blake3 pseudonimizer
///
/// Process: Blake3-keyed-variable(chacha_key[64], raw_magic_link, 44) → nonce[12] + secret_key[32]
///
/// # Arguments
/// * `raw_magic_link` - 32-byte raw magic link data
///
/// # Returns
/// * `Result<([u8; 12], [u8; 32]), String>` - (nonce, secret_key) or error
pub fn generate_chacha_nonce_and_key(
    raw_magic_link: &[u8; 32],
) -> Result<([u8; 12], [u8; 32]), String> {
    // Get ChaCha encryption key (64 bytes)
    let chacha_key = get_chacha_encryption_key()?;

    // Generate nonce[12] + secret_key[32] using Blake3 pseudonimizer
    let combined_data = blake3_keyed_variable(&chacha_key, raw_magic_link, 44);

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
pub fn encrypt_magic_link(
    raw_magic_link: &[u8; 32],
    nonce: &[u8; 12],
    secret_key: &[u8; 32],
) -> Result<Vec<u8>, String> {
    let mut cipher = ChaCha20::new(secret_key.into(), nonce.into());

    let mut encrypted = *raw_magic_link;
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
pub fn decrypt_magic_link(
    encrypted_data: &[u8],
    nonce: &[u8; 12],
    secret_key: &[u8; 32],
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
