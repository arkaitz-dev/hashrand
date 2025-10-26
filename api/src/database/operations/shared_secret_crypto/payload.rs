///! Payload encryption/decryption using random key material
///!
///! Uses ChaCha20-Poly1305 AEAD with nonce and cipher_key extracted from key_material.

use super::super::shared_secret_types::constants::*;
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;

/// Extract nonce and cipher_key from key_material
///
/// Shared helper to eliminate duplication between encrypt/decrypt
///
/// # Arguments
/// * `key_material` - Random key material [44 bytes]
///
/// # Returns
/// * `Result<([u8; 12], [u8; 32]), SqliteError>` - (nonce, cipher_key)
fn extract_key_material(
    key_material: &[u8; KEY_MATERIAL_LENGTH],
) -> Result<([u8; NONCE_LENGTH], [u8; SECRET_KEY_LENGTH]), SqliteError> {
    let nonce_bytes: [u8; NONCE_LENGTH] =
        key_material[0..NONCE_LENGTH].try_into().map_err(|_| {
            SqliteError::Io("Failed to extract nonce from key_material".to_string())
        })?;

    let cipher_key: [u8; SECRET_KEY_LENGTH] = key_material[NONCE_LENGTH..KEY_MATERIAL_LENGTH]
        .try_into()
        .map_err(|_| {
            SqliteError::Io("Failed to extract cipher_key from key_material".to_string())
        })?;

    Ok((nonce_bytes, cipher_key))
}

/// Encrypt payload using random key material (ChaCha20-Poly1305 AEAD)
///
/// Process:
/// 1. Extract nonce[12] + cipher_key[32] from key_material[44]
/// 2. Encrypt with ChaCha20-Poly1305 (adds 16-byte tag)
///
/// # Arguments
/// * `key_material` - Random key material [44 bytes]
/// * `payload` - Raw payload to encrypt
///
/// # Returns
/// * `Result<Vec<u8>, SqliteError>` - Encrypted payload + tag
pub fn encrypt_payload_with_material(
    key_material: &[u8; KEY_MATERIAL_LENGTH],
    payload: &[u8],
) -> Result<Vec<u8>, SqliteError> {
    let (nonce_bytes, cipher_key) = extract_key_material(key_material)?;

    let nonce = Nonce::from_slice(&nonce_bytes);
    let key = Key::from_slice(&cipher_key);

    // Encrypt with ChaCha20-Poly1305 AEAD
    let cipher = ChaCha20Poly1305::new(key);
    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 encryption error: {:?}", e)))?;

    debug!("🔒 SharedSecret: Encrypted payload with key_material (ChaCha20-Poly1305)");
    Ok(ciphertext)
}

/// Decrypt payload using random key material (ChaCha20-Poly1305 AEAD)
///
/// # Arguments
/// * `key_material` - Random key material [44 bytes]
/// * `ciphertext` - Encrypted payload to decrypt
///
/// # Returns
/// * `Result<Vec<u8>, SqliteError>` - Decrypted payload or error
pub fn decrypt_payload_with_material(
    key_material: &[u8; KEY_MATERIAL_LENGTH],
    ciphertext: &[u8],
) -> Result<Vec<u8>, SqliteError> {
    let (nonce_bytes, cipher_key) = extract_key_material(key_material)?;

    let nonce = Nonce::from_slice(&nonce_bytes);
    let key = Key::from_slice(&cipher_key);

    // Decrypt with ChaCha20-Poly1305 AEAD
    let cipher = ChaCha20Poly1305::new(key);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 decryption error: {:?}", e)))?;

    debug!("🔓 SharedSecret: Decrypted payload with key_material (ChaCha20-Poly1305)");
    Ok(plaintext)
}
