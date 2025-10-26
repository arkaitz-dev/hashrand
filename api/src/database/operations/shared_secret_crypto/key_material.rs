///! Key material encryption/decryption using ChaCha20 stream cipher
///!
///! Encrypts random key material for storage in database without Poly1305 MAC
///! (integrity guaranteed by layer 2 encryption).

use super::super::shared_secret_types::constants::*;
use crate::utils::pseudonimizer::blake3_keyed_variable;
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;

/// Derive nonce[12] + cipher_key[32] from db_index using Blake3 KDF
///
/// Shared helper to eliminate duplication between encrypt/decrypt
///
/// # Arguments
/// * `db_index` - Database index (32 bytes)
///
/// # Returns
/// * `Result<([u8; 12], [u8; 32]), SqliteError>` - (nonce, cipher_key)
fn derive_cipher_and_nonce(
    db_index: &[u8; DB_INDEX_LENGTH],
) -> Result<([u8; NONCE_LENGTH], [u8; SECRET_KEY_LENGTH]), SqliteError> {
    use crate::utils::jwt::config::get_shared_secret_content_key;

    let content_key = get_shared_secret_content_key()
        .map_err(|e| SqliteError::Io(format!("Failed to get content key: {}", e)))?;

    // Derive nonce[12] + cipher_key[32] using Blake3 KDF
    let derived = blake3_keyed_variable(&content_key, db_index, 44);

    let nonce_bytes: [u8; NONCE_LENGTH] = derived[0..NONCE_LENGTH]
        .try_into()
        .map_err(|_| SqliteError::Io("Failed to extract nonce".to_string()))?;

    let cipher_key: [u8; SECRET_KEY_LENGTH] = derived[NONCE_LENGTH..44]
        .try_into()
        .map_err(|_| SqliteError::Io("Failed to extract cipher key".to_string()))?;

    Ok((nonce_bytes, cipher_key))
}

/// Encrypt random key material using ChaCha20 stream cipher
///
/// Process:
/// 1. Derive nonce[12] + cipher_key[32] from db_index using Blake3 KDF
/// 2. Encrypt key_material with ChaCha20 (maintains 44-byte size)
///
/// NOTE: Uses ChaCha20 WITHOUT Poly1305 MAC (integrity guaranteed by layer 2)
///
/// # Arguments
/// * `db_index` - Database index (32 bytes) - unique per entry
/// * `key_material` - Random key material [44 bytes] to encrypt
///
/// # Returns
/// * `Result<Vec<u8>, SqliteError>` - Encrypted key material (44 bytes)
pub fn encrypt_key_material(
    db_index: &[u8; DB_INDEX_LENGTH],
    key_material: &[u8; KEY_MATERIAL_LENGTH],
) -> Result<Vec<u8>, SqliteError> {
    use chacha20::ChaCha20;
    use chacha20::cipher::{KeyIvInit, StreamCipher};

    let (nonce_bytes, cipher_key) = derive_cipher_and_nonce(db_index)?;

    // Initialize ChaCha20 cipher (stream cipher, NO Poly1305)
    let mut cipher = ChaCha20::new(&cipher_key.into(), &nonce_bytes.into());

    // Encrypt in-place
    let mut encrypted = *key_material;
    cipher.apply_keystream(&mut encrypted);

    debug!("🔐 SharedSecret: Encrypted key_material[44] with ChaCha20 (no MAC)");
    Ok(encrypted.to_vec())
}

/// Decrypt random key material using ChaCha20 stream cipher
///
/// # Arguments
/// * `db_index` - Database index (32 bytes)
/// * `ciphertext` - Encrypted key material (44 bytes)
///
/// # Returns
/// * `Result<[u8; KEY_MATERIAL_LENGTH], SqliteError>` - Decrypted key material
pub fn decrypt_key_material(
    db_index: &[u8; DB_INDEX_LENGTH],
    ciphertext: &[u8],
) -> Result<[u8; KEY_MATERIAL_LENGTH], SqliteError> {
    use chacha20::ChaCha20;
    use chacha20::cipher::{KeyIvInit, StreamCipher};

    if ciphertext.len() != KEY_MATERIAL_LENGTH {
        return Err(SqliteError::Io(format!(
            "Invalid ciphertext length: expected {}, got {}",
            KEY_MATERIAL_LENGTH,
            ciphertext.len()
        )));
    }

    let (nonce_bytes, cipher_key) = derive_cipher_and_nonce(db_index)?;

    // Initialize ChaCha20 cipher
    let mut cipher = ChaCha20::new(&cipher_key.into(), &nonce_bytes.into());

    // Decrypt in-place (ChaCha20 is symmetric)
    let mut decrypted = [0u8; KEY_MATERIAL_LENGTH];
    decrypted.copy_from_slice(ciphertext);
    cipher.apply_keystream(&mut decrypted);

    debug!("🔓 SharedSecret: Decrypted key_material[44] with ChaCha20");
    Ok(decrypted)
}
