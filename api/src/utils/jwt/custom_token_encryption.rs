//! Custom token encryption operations
//!
//! Single Responsibility: Prehash seed encryption/decryption with circular interdependence

use super::config::{get_prehash_cipher_key, get_prehash_hmac_key, get_prehash_nonce_key};
use super::custom_token_crypto::{
    decrypt_prehash_seed_data, encrypt_prehash_seed_data, generate_cipher_key,
    generate_cipher_key_from_derived, generate_cipher_nonce_from_derived,
    generate_prehash_from_derived, hash_encrypted_payload,
};

/// Type alias for prehash encryption keys (cipher_key, nonce_key, hmac_key)
type PrehashKeys = ([u8; 32], [u8; 32], [u8; 32]);

/// Generate prehash seed encryption keys from encrypted payload (circular interdependence)
pub fn generate_prehash_encryption_keys(
    encrypted_payload: &[u8; 64],
) -> Result<PrehashKeys, String> {
    // Get base keys from environment
    let base_cipher_key = get_prehash_cipher_key()?;
    let base_nonce_key = get_prehash_nonce_key()?;
    let base_hmac_key = get_prehash_hmac_key()?;

    // Hash encrypted_payload to 32 bytes for key derivation
    let payload_hash = hash_encrypted_payload(encrypted_payload);

    // Use payload_hash as prehash to derive actual encryption keys
    let cipher_key = generate_cipher_key(&base_cipher_key, &payload_hash)?;
    let nonce_key = generate_cipher_key(&base_nonce_key, &payload_hash)?;
    let hmac_key = generate_cipher_key(&base_hmac_key, &payload_hash)?;

    Ok((cipher_key, nonce_key, hmac_key))
}

/// Encrypt prehash seed using circular interdependent encryption
pub fn encrypt_prehash_seed(
    prehash_seed: &[u8; 32],
    encrypted_payload: &[u8; 64],
) -> Result<[u8; 32], String> {
    // Generate encryption keys from encrypted_payload (circular dependency)
    let (cipher_key, nonce_key, hmac_key) = generate_prehash_encryption_keys(encrypted_payload)?;

    // Generate prehash from encrypted_payload hash for key derivation
    let payload_hash = hash_encrypted_payload(encrypted_payload);
    let prehash = generate_prehash_from_derived(&payload_hash, &hmac_key)?;

    // Generate actual cipher key and nonce using derived keys
    let final_cipher_key = generate_cipher_key_from_derived(&cipher_key, &prehash)?;
    let final_cipher_nonce = generate_cipher_nonce_from_derived(&nonce_key, &prehash)?;

    // Encrypt prehash_seed with ChaCha20
    encrypt_prehash_seed_data(prehash_seed, &final_cipher_key, &final_cipher_nonce)
}

/// Decrypt prehash seed using circular interdependent decryption
pub fn decrypt_prehash_seed(
    encrypted_prehash_seed: &[u8; 32],
    encrypted_payload: &[u8; 64],
) -> Result<[u8; 32], String> {
    // Generate decryption keys from encrypted_payload (same as encryption)
    let (cipher_key, nonce_key, hmac_key) = generate_prehash_encryption_keys(encrypted_payload)?;

    // Generate prehash from encrypted_payload hash for key derivation
    let payload_hash = hash_encrypted_payload(encrypted_payload);
    let prehash = generate_prehash_from_derived(&payload_hash, &hmac_key)?;

    // Generate actual cipher key and nonce (same as encryption) using derived keys
    let final_cipher_key = generate_cipher_key_from_derived(&cipher_key, &prehash)?;
    let final_cipher_nonce = generate_cipher_nonce_from_derived(&nonce_key, &prehash)?;

    // Decrypt encrypted_prehash_seed with ChaCha20
    decrypt_prehash_seed_data(
        encrypted_prehash_seed,
        &final_cipher_key,
        &final_cipher_nonce,
    )
}
