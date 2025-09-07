//! Cryptographic utilities
//!
//! Provides cryptographic functions for user ID derivation, salt generation,
//! Argon2id hashing, and ChaCha20 encryption/decryption.

use argon2::{
    Algorithm as Argon2Algorithm, Argon2, Params, Version as Argon2Version,
    password_hash::{PasswordHasher, SaltString},
};
use base64::{Engine as _, engine::general_purpose};
use blake2::{
    Blake2b512, Blake2bMac, Blake2bVar, Digest,
    digest::{KeyInit as Blake2KeyInit, Mac, Update, VariableOutput},
};
use chacha20::{
    ChaCha20,
    cipher::{KeyIvInit, StreamCipher},
};
use chacha20poly1305::consts::U32;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use super::config::{get_argon2_salt, get_chacha_encryption_key, get_user_id_hmac_key};

/// Argon2id parameters for current security standards (2024)
/// Fixed parameters as requested: mem_cost=19456, time_cost=2, lane=1, hash_length=32
pub const ARGON2_MEM_COST: u32 = 19456; // Memory usage in KB
pub const ARGON2_TIME_COST: u32 = 2; // Number of iterations
pub const ARGON2_LANES: u32 = 1; // Parallelism parameter
pub const ARGON2_HASH_LENGTH: usize = 32; // Output length in bytes

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
    // Step 1: Blake2b hash of email (32 bytes)
    let email_hash = Blake2b512::digest(email.to_lowercase().trim().as_bytes());

    // Step 2: Blake2b keyed hash of the email hash (replaces HMAC-SHA3-256)
    let hmac_key = get_user_id_hmac_key()?;
    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&hmac_key)
        .map_err(|_| "Invalid USER_ID_HMAC_KEY format".to_string())?;
    Mac::update(&mut keyed_hasher, &email_hash[..32]); // Take first 32 bytes from Blake2b512
    let hmac_result = keyed_hasher.finalize().into_bytes();

    // Step 3: Generate dynamic salt using Blake2b keyed + ChaCha8Rng
    let dynamic_salt = generate_dynamic_salt(&email_hash)?;

    // Step 4: Argon2id with fixed parameters (using Blake2b result as data input)
    let argon2_output = derive_with_argon2id(&hmac_result[..], &dynamic_salt)?;

    // Step 5: Blake2b variable output to compress to 16 bytes (replaces SHAKE256)
    let mut final_hasher =
        Blake2bVar::new(16).map_err(|_| "Blake2b initialization failed".to_string())?;
    Update::update(&mut final_hasher, &argon2_output);
    let mut user_id = [0u8; 16];
    final_hasher
        .finalize_variable(&mut user_id)
        .map_err(|_| "Blake2b finalization failed".to_string())?;

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

/// Generate dynamic salt using HMAC-SHA3-256 → ChaCha8Rng → salt bytes
///
/// Process: fixed_salt → HMAC-SHA3-256(fixed_salt, data) → ChaCha8Rng[32 bytes] → salt
///
/// # Arguments
/// * `data` - Data to derive salt from (typically email hash)
///
/// # Returns
/// * `Result<[u8; 32], String>` - 32-byte dynamic salt
pub fn generate_dynamic_salt(data: &[u8]) -> Result<[u8; 32], String> {
    let fixed_salt = get_argon2_salt()?;

    // Generate Blake2b keyed hash (replaces HMAC-SHA3-256)
    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&fixed_salt)
        .map_err(|_| "Invalid ARGON2_SALT format for Blake2b keyed".to_string())?;
    Mac::update(&mut keyed_hasher, data);
    let hmac_result = keyed_hasher.finalize().into_bytes();

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

/// Generate nonce and secret key from HMAC-SHA3-256 → ChaCha8RNG[44]
///
/// Process: HMAC-SHA3-256(raw_magic_link, chacha_key) → ChaCha8RNG[44] → nonce[12] + secret_key[32]
///
/// # Arguments
/// * `raw_magic_link` - 32-byte raw magic link data
///
/// # Returns
/// * `Result<([u8; 12], [u8; 32]), String>` - (nonce, secret_key) or error
pub fn generate_chacha_nonce_and_key(
    raw_magic_link: &[u8; 32],
) -> Result<([u8; 12], [u8; 32]), String> {
    // Get ChaCha encryption key
    let chacha_key = get_chacha_encryption_key()?;

    // Generate Blake2b keyed hash (replaces HMAC-SHA3-256)
    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&chacha_key)
        .map_err(|_| "Invalid ChaCha encryption key format".to_string())?;
    Mac::update(&mut keyed_hasher, raw_magic_link);
    let hmac_result = keyed_hasher.finalize().into_bytes();

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
