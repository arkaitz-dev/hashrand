//! Magic link cryptographic operations
//!
//! Provides cryptographic functions for magic link security using
//! Blake3 KDF and ChaCha20-Poly1305 AEAD encryption.

use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce, aead::Aead};
use crate::utils::pseudonimizer::blake3_keyed_variable;
use spin_sdk::sqlite::Error as SqliteError;

/// Magic link cryptographic operations
pub struct MagicLinkCrypto;

impl MagicLinkCrypto {
    /// Get magic link content encryption key from environment
    ///
    /// # Returns
    /// * `Result<[u8; 64], SqliteError>` - 64-byte encryption key
    pub fn get_mlink_content_key() -> Result<[u8; 64], SqliteError> {
        use crate::utils::jwt::config::get_mlink_content_key;
        get_mlink_content_key().map_err(|e| SqliteError::Io(e))
    }

    /// Encrypt payload using Blake3 KDF + ChaCha20-Poly1305
    ///
    /// Process:
    /// 1. blake3_keyed_variable(MLINK_CONTENT, encrypted_data, 44) → nonce[12] + cipher_key[32]
    /// 2. ChaCha20-Poly1305.encrypt(payload, nonce, cipher_key) → encrypted_blob
    ///
    /// # Arguments
    /// * `encrypted_data` - The encrypted magic token bytes (32 bytes)
    /// * `payload` - Data to encrypt
    ///
    /// # Returns
    /// * `Result<Vec<u8>, SqliteError>` - Encrypted payload or error
    pub fn encrypt_payload_content(
        encrypted_data: &[u8; 32],
        payload: &[u8],
    ) -> Result<Vec<u8>, SqliteError> {
        let mlink_key = Self::get_mlink_content_key()?;

        // Step 1: Derive nonce + cipher_key using Blake3 KDF
        let derived = blake3_keyed_variable(&mlink_key, encrypted_data, 44);

        let nonce_bytes: [u8; 12] = derived[0..12].try_into()
            .map_err(|_| SqliteError::Io("Failed to extract nonce".to_string()))?;
        let cipher_key: [u8; 32] = derived[12..44].try_into()
            .map_err(|_| SqliteError::Io("Failed to extract cipher key".to_string()))?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let key = Key::from_slice(&cipher_key);

        // Step 2: Encrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new(key);
        let ciphertext = cipher
            .encrypt(nonce, payload)
            .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 encryption error: {:?}", e)))?;

        println!("Database: Encrypted payload using Blake3 KDF");
        Ok(ciphertext)
    }

    /// Decrypt payload using Blake3 KDF + ChaCha20-Poly1305 (reverse process)
    ///
    /// # Arguments
    /// * `encrypted_data` - The encrypted magic token bytes (32 bytes)
    /// * `ciphertext` - Encrypted payload to decrypt
    ///
    /// # Returns
    /// * `Result<Vec<u8>, SqliteError>` - Decrypted payload or error
    pub fn decrypt_payload_content(
        encrypted_data: &[u8; 32],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, SqliteError> {
        let mlink_key = Self::get_mlink_content_key()?;

        // Step 1: Derive nonce + cipher_key using Blake3 KDF (same as encryption)
        let derived = blake3_keyed_variable(&mlink_key, encrypted_data, 44);

        let nonce_bytes: [u8; 12] = derived[0..12].try_into()
            .map_err(|_| SqliteError::Io("Failed to extract nonce".to_string()))?;
        let cipher_key: [u8; 32] = derived[12..44].try_into()
            .map_err(|_| SqliteError::Io("Failed to extract cipher key".to_string()))?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let key = Key::from_slice(&cipher_key);

        // Step 2: Decrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new(key);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 decryption error: {:?}", e)))?;

        println!("Database: Decrypted payload using Blake3 KDF");
        Ok(plaintext)
    }

    /// Create Blake3 keyed hash of encrypted magic token for database indexing
    ///
    /// # Arguments
    /// * `encrypted_data` - The encrypted magic token bytes
    ///
    /// # Returns
    /// * `[u8; 16]` - 16-byte Blake3 keyed hash for database indexing
    pub fn create_encrypted_token_hash(encrypted_data: &[u8]) -> [u8; 16] {
        // Blake3-keyed-variable(hash_key, encrypted_data, 16) → [16 bytes]
        use crate::utils::jwt::config::get_encrypted_mlink_token_hash_key;
        use crate::utils::pseudonimizer::blake3_keyed_variable;

        let hash_key = get_encrypted_mlink_token_hash_key()
            .expect("Failed to get encrypted magic link token hash key");

        let hash_vec = blake3_keyed_variable(&hash_key, encrypted_data, 16);
        let mut hash = [0u8; 16];
        hash.copy_from_slice(&hash_vec);
        hash
    }
}
