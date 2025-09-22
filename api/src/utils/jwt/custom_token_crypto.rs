//! Custom token cryptographic operations
//!
//! Single Responsibility: Low-level cryptographic functions for prehash, encryption, and key derivation

use blake2::{
    Blake2bMac, Blake2bVar,
    digest::{KeyInit as Blake2KeyInit, Mac, Update, VariableOutput},
};
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20poly1305::consts::U32;

/// Generate cryptographically secure prehash seed (32 bytes)
pub fn generate_prehash_seed() -> [u8; 32] {
    use rand::RngCore;
    let mut rng = rand::rng();
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    seed
}

/// Generate prehash using Blake2b-keyed (similar to web UI cryptoHashGen)
pub fn generate_prehash(seed: &[u8; 32], hmac_key: &[u8]) -> Result<[u8; 32], String> {
    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(hmac_key)
        .map_err(|_| "Invalid HMAC key format for prehash".to_string())?;
    Mac::update(&mut keyed_hasher, seed);
    let result = keyed_hasher.finalize().into_bytes();

    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..32]);
    Ok(output)
}

/// Generate 32-byte hash from 64-byte encrypted payload for key derivation
pub fn hash_encrypted_payload(encrypted_payload: &[u8; 64]) -> [u8; 32] {
    let mut hasher = Blake2bVar::new(32).expect("Blake2b initialization should not fail");
    hasher.update(encrypted_payload);
    let mut result = [0u8; 32];
    hasher
        .finalize_variable(&mut result)
        .expect("Blake2b finalization should not fail");
    result
}

/// Generate cipher key from base key and prehash (similar to web UI generateCipherKey)
pub fn generate_cipher_key(base_key: &[u8], prehash: &[u8; 32]) -> Result<[u8; 32], String> {
    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(base_key)
        .map_err(|_| "Invalid base key format for cipher".to_string())?;
    Mac::update(&mut keyed_hasher, prehash);
    let result = keyed_hasher.finalize().into_bytes();

    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..32]);
    Ok(output)
}

/// Generate nonce from base key and prehash (similar to web UI generateCipherNonce)
pub fn generate_cipher_nonce(base_key: &[u8], prehash: &[u8; 32]) -> Result<[u8; 12], String> {
    let mut keyed_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(base_key)
        .map_err(|_| "Invalid base key format for nonce".to_string())?;
    Mac::update(&mut keyed_hasher, prehash);
    let result = keyed_hasher.finalize().into_bytes();

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
