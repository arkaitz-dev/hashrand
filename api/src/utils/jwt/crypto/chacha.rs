use crate::utils::pseudonimizer::blake3_keyed_variable;

use super::super::config::get_chacha_encryption_key;
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};

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
    copy_bytes_to_array(&mut nonce, &combined_data[..12]);
    copy_bytes_to_array(&mut secret_key, &combined_data[12..44]);

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
    validate_byte_length(encrypted_data, 32)?;

    let mut cipher = ChaCha20::new(secret_key.into(), nonce.into());

    let mut decrypted = [0u8; 32];
    copy_bytes_to_array(&mut decrypted, encrypted_data);
    cipher.apply_keystream(&mut decrypted);

    Ok(decrypted)
}

/// Validate byte slice length (DRY utility)
fn validate_byte_length(data: &[u8], expected: usize) -> Result<(), String> {
    if data.len() != expected {
        return Err(format!("Expected {} bytes, got {}", expected, data.len()));
    }
    Ok(())
}

/// Copy bytes to fixed-size array (DRY utility)
fn copy_bytes_to_array<const N: usize>(dest: &mut [u8; N], src: &[u8]) {
    dest.copy_from_slice(src);
}
