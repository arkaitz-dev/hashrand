//! Custom token cryptographic operations
//!
//! Single Responsibility: Low-level cryptographic functions for prehash, encryption, and key derivation

use crate::utils::pseudonimizer::blake3_keyed_variable;
use blake3;
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};

/// Generate cryptographically secure prehash seed (32 bytes)
pub fn generate_prehash_seed() -> [u8; 32] {
    use rand::RngCore;
    let mut rng = rand::rng();
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    seed
}

/// Generate prehash using Blake3-keyed with base key (64 bytes)
pub fn generate_prehash(seed: &[u8; 32], hmac_key: &[u8; 64]) -> Result<[u8; 32], String> {
    let result = blake3_keyed_variable(hmac_key, seed, 32);
    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..32]);
    Ok(output)
}

/// Generate prehash using Blake3-keyed with derived key (32 bytes)
pub fn generate_prehash_from_derived(
    seed: &[u8; 32],
    hmac_key: &[u8; 32],
) -> Result<[u8; 32], String> {
    let mut key_64 = [0u8; 64];
    key_64[..32].copy_from_slice(hmac_key);
    let result = blake3_keyed_variable(&key_64, seed, 32);
    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..32]);
    Ok(output)
}

/// Generate 32-byte hash from 64-byte encrypted payload for key derivation
pub fn hash_encrypted_payload(encrypted_payload: &[u8; 64]) -> [u8; 32] {
    *blake3::hash(encrypted_payload).as_bytes()
}

/// Generate cipher key from base key (64 bytes) and prehash
pub fn generate_cipher_key(base_key: &[u8; 64], prehash: &[u8; 32]) -> Result<[u8; 32], String> {
    let result = blake3_keyed_variable(base_key, prehash, 32);
    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..32]);
    Ok(output)
}

/// Generate cipher key from derived key (32 bytes) and prehash
pub fn generate_cipher_key_from_derived(
    base_key: &[u8; 32],
    prehash: &[u8; 32],
) -> Result<[u8; 32], String> {
    let mut key_64 = [0u8; 64];
    key_64[..32].copy_from_slice(base_key);
    let result = blake3_keyed_variable(&key_64, prehash, 32);
    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..32]);
    Ok(output)
}

/// Generate nonce from base key (64 bytes) and prehash
pub fn generate_cipher_nonce(base_key: &[u8; 64], prehash: &[u8; 32]) -> Result<[u8; 12], String> {
    let result = blake3_keyed_variable(base_key, prehash, 12);
    let mut output = [0u8; 12];
    output.copy_from_slice(&result[..12]);
    Ok(output)
}

/// Generate nonce from derived key (32 bytes) and prehash
pub fn generate_cipher_nonce_from_derived(
    base_key: &[u8; 32],
    prehash: &[u8; 32],
) -> Result<[u8; 12], String> {
    let mut key_64 = [0u8; 64];
    key_64[..32].copy_from_slice(base_key);
    let result = blake3_keyed_variable(&key_64, prehash, 12);
    let mut output = [0u8; 12];
    output.copy_from_slice(&result[..12]);
    Ok(output)
}

/// Encrypt prehash seed with ChaCha20 (32 bytes)
pub fn encrypt_prehash_seed_data(
    seed: &[u8; 32],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<[u8; 32], String> {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    let mut ciphertext = *seed;
    cipher.apply_keystream(&mut ciphertext);
    Ok(ciphertext)
}

/// Decrypt prehash seed with ChaCha20 (32 bytes)
pub fn decrypt_prehash_seed_data(
    ciphertext: &[u8; 32],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<[u8; 32], String> {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    let mut plaintext = *ciphertext;
    cipher.apply_keystream(&mut plaintext);
    Ok(plaintext)
}

/// Encrypt payload with ChaCha20 (64 bytes)
pub fn encrypt_payload(
    payload: &[u8; 64],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<[u8; 64], String> {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    let mut ciphertext = *payload;
    cipher.apply_keystream(&mut ciphertext);
    Ok(ciphertext)
}

/// Decrypt payload with ChaCha20 (64 bytes)
pub fn decrypt_payload(
    ciphertext: &[u8; 64],
    key: &[u8; 32],
    nonce: &[u8; 12],
) -> Result<[u8; 64], String> {
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    let mut plaintext = *ciphertext;
    cipher.apply_keystream(&mut plaintext);
    Ok(plaintext)
}
