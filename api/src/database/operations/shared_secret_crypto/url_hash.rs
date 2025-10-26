///! URL hash operations for Zero Knowledge shared secret system
///!
///! Handles checksum generation, hash encryption/decryption, and validation.

use super::super::shared_secret_types::constants::*;
use super::super::shared_secret_types::SecretRole;
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;

/// Derive URL cipher_key[32] + nonce[12] using Blake3 KDF
///
/// Shared helper to eliminate duplication between encrypt/decrypt
///
/// # Returns
/// * `Result<([u8; 32], [u8; 12]), SqliteError>` - (cipher_key, nonce)
fn derive_url_cipher_and_nonce() -> Result<([u8; 32], [u8; 12]), SqliteError> {
    use crate::utils::jwt::config::get_shared_secret_url_cipher_key;

    // Get URL cipher key (64 bytes)
    let url_cipher_key = get_shared_secret_url_cipher_key()
        .map_err(|e| SqliteError::Io(format!("Failed to get URL cipher key: {}", e)))?;

    // Derive cipher key (32 bytes) + nonce (12 bytes) using Blake3 KDF
    let derived = crate::utils::pseudonimizer::blake3_keyed_variable(
        &url_cipher_key,
        b"URL_CIPHER_V1",
        44,
    );

    let cipher_key: [u8; 32] = derived[0..32]
        .try_into()
        .map_err(|_| SqliteError::Io("Failed to extract cipher key".to_string()))?;

    let nonce: [u8; 12] = derived[32..44]
        .try_into()
        .map_err(|_| SqliteError::Io("Failed to extract nonce".to_string()))?;

    Ok((cipher_key, nonce))
}

/// Generate 8-byte checksum with embedded role indicator
///
/// Checksum structure: blake3_keyed(ref + user_id)[0..7] + role_byte[1]
/// - role_byte = 0x01 for Sender
/// - role_byte = 0x02 for Receiver
///
/// # Arguments
/// * `reference_hash` - 16-byte reference hash (shared between sender/receiver)
/// * `user_id` - 16-byte user ID (Zero Knowledge derivation from email)
/// * `role` - Sender or Receiver role
///
/// # Returns
/// * `Result<[u8; 8], SqliteError>` - 8-byte checksum (7 bytes hash + 1 byte role)
pub fn generate_checksum_with_role(
    reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    user_id: &[u8; USER_ID_LENGTH],
    role: SecretRole,
) -> Result<[u8; 8], SqliteError> {
    use crate::utils::jwt::config::get_shared_secret_checksum_key;

    let checksum_key = get_shared_secret_checksum_key()
        .map_err(|e| SqliteError::Io(format!("Failed to get checksum key: {}", e)))?;

    // Concatenate reference_hash + user_id
    let mut combined = Vec::with_capacity(32);
    combined.extend_from_slice(reference_hash);
    combined.extend_from_slice(user_id);

    // Generate 7-byte checksum using blake3_keyed_variable
    let checksum_base =
        crate::utils::pseudonimizer::blake3_keyed_variable(&checksum_key, &combined, 7);

    // Determine role byte
    let role_byte = match role {
        SecretRole::Sender => 0x01,
        SecretRole::Receiver => 0x02,
    };

    // Build final 8-byte checksum: [checksum_base (7 bytes), role_byte (1 byte)]
    let mut checksum = [0u8; 8];
    checksum[0..7].copy_from_slice(&checksum_base[0..7]);
    checksum[7] = role_byte;

    debug!("🔒 SharedSecret: Generated checksum with role {:?}", role);
    Ok(checksum)
}

/// Generate 40-byte hash for shared secret URL
///
/// Structure: reference_hash[16] + user_id[16] + checksum[8]
/// - user_id derived from email with Zero Knowledge (Argon2id + Blake3)
/// - checksum includes role indicator in last byte
///
/// # Arguments
/// * `reference_hash` - 16-byte reference hash
/// * `email` - Email address to derive user_id
/// * `role` - Sender or Receiver role
///
/// # Returns
/// * `Result<[u8; 40], SqliteError>` - 40-byte hash ready for encryption
pub fn generate_shared_secret_hash(
    reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    email: &str,
    role: SecretRole,
) -> Result<[u8; 40], SqliteError> {
    use super::helpers::calculate_user_id;

    // 1. Derive user_id from email (Zero Knowledge)
    let user_id = calculate_user_id(email)?;

    // 2. Generate checksum with role
    let checksum = generate_checksum_with_role(reference_hash, &user_id, role)?;

    // 3. Concatenate: ref[16] + user_id[16] + checksum[8] = 40 bytes
    let mut hash = [0u8; 40];
    hash[0..16].copy_from_slice(reference_hash);
    hash[16..32].copy_from_slice(&user_id);
    hash[32..40].copy_from_slice(&checksum);

    debug!("✅ SharedSecret: Generated 40-byte hash for {:?}", role);
    Ok(hash)
}

/// Encrypt 40-byte hash using ChaCha20 stream cipher
///
/// Process:
/// 1. Derive cipher_key[32] + nonce[12] from URL_CIPHER_KEY using Blake3 KDF
/// 2. Encrypt hash with ChaCha20 (maintains 40-byte size)
///
/// # Arguments
/// * `hash_40` - Plaintext 40-byte hash
///
/// # Returns
/// * `Result<[u8; 40], SqliteError>` - Encrypted 40-byte hash
pub fn encrypt_url_hash(hash_40: &[u8; 40]) -> Result<[u8; 40], SqliteError> {
    use chacha20::ChaCha20;
    use chacha20::cipher::{KeyIvInit, StreamCipher};

    let (cipher_key, nonce) = derive_url_cipher_and_nonce()?;

    // Initialize ChaCha20 cipher
    let mut cipher = ChaCha20::new(&cipher_key.into(), &nonce.into());

    // Encrypt in-place
    let mut encrypted = *hash_40;
    cipher.apply_keystream(&mut encrypted);

    debug!("🔐 SharedSecret: Encrypted 40-byte hash with ChaCha20");
    Ok(encrypted)
}

/// Decrypt 40-byte hash using ChaCha20 stream cipher
///
/// # Arguments
/// * `encrypted_hash` - Encrypted 40-byte hash from URL (Base58 decoded)
///
/// # Returns
/// * `Result<[u8; 40], SqliteError>` - Decrypted 40-byte hash
pub fn decrypt_url_hash(encrypted_hash: &[u8; 40]) -> Result<[u8; 40], SqliteError> {
    use chacha20::ChaCha20;
    use chacha20::cipher::{KeyIvInit, StreamCipher};

    let (cipher_key, nonce) = derive_url_cipher_and_nonce()?;

    // Initialize ChaCha20 cipher
    let mut cipher = ChaCha20::new(&cipher_key.into(), &nonce.into());

    // Decrypt in-place (ChaCha20 is symmetric)
    let mut decrypted = *encrypted_hash;
    cipher.apply_keystream(&mut decrypted);

    debug!("🔓 SharedSecret: Decrypted 40-byte hash with ChaCha20");
    Ok(decrypted)
}

/// Validate checksum and extract components from 40-byte hash
///
/// # Arguments
/// * `hash_40` - Decrypted 40-byte hash
///
/// # Returns
/// * `Result<([u8; 16], [u8; 16], SecretRole), SqliteError>` - (reference_hash, user_id, role) or error
pub fn validate_and_extract_hash(
    hash_40: &[u8; 40],
) -> Result<([u8; REFERENCE_HASH_LENGTH], [u8; USER_ID_LENGTH], SecretRole), SqliteError> {
    use crate::utils::jwt::config::get_shared_secret_checksum_key;

    // Extract components
    let reference_hash: [u8; REFERENCE_HASH_LENGTH] = hash_40[0..16]
        .try_into()
        .map_err(|_| SqliteError::Io("Failed to extract reference_hash".to_string()))?;

    let user_id: [u8; USER_ID_LENGTH] = hash_40[16..32]
        .try_into()
        .map_err(|_| SqliteError::Io("Failed to extract user_id".to_string()))?;

    let provided_checksum: [u8; 8] = hash_40[32..40]
        .try_into()
        .map_err(|_| SqliteError::Io("Failed to extract checksum".to_string()))?;

    // Extract role from last byte
    let role_byte = provided_checksum[7];
    let role = match role_byte {
        0x01 => SecretRole::Sender,
        0x02 => SecretRole::Receiver,
        _ => {
            return Err(SqliteError::Io(format!(
                "Invalid role indicator: 0x{:02x}",
                role_byte
            )));
        }
    };

    // Verify checksum (first 7 bytes)
    let checksum_key = get_shared_secret_checksum_key()
        .map_err(|e| SqliteError::Io(format!("Failed to get checksum key: {}", e)))?;

    let mut combined = Vec::with_capacity(32);
    combined.extend_from_slice(&reference_hash);
    combined.extend_from_slice(&user_id);

    let calculated_checksum_base =
        crate::utils::pseudonimizer::blake3_keyed_variable(&checksum_key, &combined, 7);

    if provided_checksum[0..7] != calculated_checksum_base[0..7] {
        return Err(SqliteError::Io(
            "Invalid hash checksum - URL may be manipulated".to_string(),
        ));
    }

    debug!(
        "✅ SharedSecret: Validated checksum and extracted role {:?}",
        role
    );
    Ok((reference_hash, user_id, role))
}
