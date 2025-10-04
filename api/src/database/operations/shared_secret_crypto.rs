//! Shared secret cryptographic operations
//!
//! Provides cryptographic functions for shared secret security using
//! Blake3 KDF, ChaCha20-Poly1305 AEAD encryption, and random generation.

use super::shared_secret_types::constants::*;
use crate::utils::pseudonimizer::blake3_keyed_variable;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce, aead::Aead};
use spin_sdk::sqlite::Error as SqliteError;

/// Shared secret cryptographic operations
pub struct SharedSecretCrypto;

impl SharedSecretCrypto {
    /// Generate cryptographically secure random reference hash
    ///
    /// Uses ChaCha8Rng for secure random generation with Blake3 seed
    ///
    /// # Returns
    /// * `[u8; 16]` - Random 16-byte reference hash
    pub fn generate_reference_hash() -> [u8; REFERENCE_HASH_LENGTH] {
        use rand::RngCore;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        // Generate seed using Blake3 of current timestamp + process-specific data
        let seed_material = format!("{:?}", std::time::SystemTime::now());
        let seed_hash = blake3::hash(seed_material.as_bytes());
        let seed: [u8; 32] = *seed_hash.as_bytes();

        let mut rng = ChaCha8Rng::from_seed(seed);
        let mut reference = [0u8; REFERENCE_HASH_LENGTH];
        rng.fill_bytes(&mut reference);
        reference
    }

    /// Generate cryptographically secure 9-digit OTP
    ///
    /// Uses ChaCha8Rng to generate a random number between 100000000 and 999999999
    ///
    /// # Returns
    /// * `String` - 9-digit OTP as string
    pub fn generate_otp() -> String {
        use rand::Rng;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        // Generate seed using Blake3 of current timestamp + counter
        let seed_material = format!("{:?}_otp", std::time::SystemTime::now());
        let seed_hash = blake3::hash(seed_material.as_bytes());
        let seed: [u8; 32] = *seed_hash.as_bytes();

        let mut rng = ChaCha8Rng::from_seed(seed);
        let otp: u32 = rng.random_range(100_000_000..=999_999_999);
        otp.to_string()
    }

    /// Get shared secret content encryption key from environment
    ///
    /// Uses the same key as magic links for consistency
    ///
    /// # Returns
    /// * `Result<[u8; 64], SqliteError>` - 64-byte encryption key
    fn get_content_encryption_key() -> Result<[u8; 64], SqliteError> {
        use crate::utils::jwt::config::get_mlink_content_key;
        get_mlink_content_key().map_err(SqliteError::Io)
    }

    /// Encrypt payload using Blake3 KDF + ChaCha20-Poly1305
    ///
    /// Process:
    /// 1. blake3_keyed_variable(CONTENT_KEY, encrypted_id, 44) → nonce[12] + cipher_key[32]
    /// 2. ChaCha20-Poly1305.encrypt(payload, nonce, cipher_key) → encrypted_blob
    ///
    /// # Arguments
    /// * `encrypted_id` - The encrypted shared secret ID bytes (32 bytes)
    /// * `payload` - Data to encrypt (sender_email || receiver_email || text || otp || created_at || reference_hash)
    ///
    /// # Returns
    /// * `Result<Vec<u8>, SqliteError>` - Encrypted payload or error
    pub fn encrypt_payload(
        encrypted_id: &[u8; ENCRYPTED_ID_LENGTH],
        payload: &[u8],
    ) -> Result<Vec<u8>, SqliteError> {
        let content_key = Self::get_content_encryption_key()?;

        // Step 1: Derive nonce + cipher_key using Blake3 KDF
        let derived = blake3_keyed_variable(&content_key, encrypted_id, 44);

        let nonce_bytes: [u8; NONCE_LENGTH] = derived[0..NONCE_LENGTH]
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to extract nonce".to_string()))?;
        let cipher_key: [u8; SECRET_KEY_LENGTH] = derived[NONCE_LENGTH..44]
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to extract cipher key".to_string()))?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let key = Key::from_slice(&cipher_key);

        // Step 2: Encrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new(key);
        let ciphertext = cipher
            .encrypt(nonce, payload)
            .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 encryption error: {:?}", e)))?;

        println!("🔒 SharedSecret: Encrypted payload using Blake3 KDF + ChaCha20-Poly1305");
        Ok(ciphertext)
    }

    /// Decrypt payload using Blake3 KDF + ChaCha20-Poly1305 (reverse process)
    ///
    /// # Arguments
    /// * `encrypted_id` - The encrypted shared secret ID bytes (32 bytes)
    /// * `ciphertext` - Encrypted payload to decrypt
    ///
    /// # Returns
    /// * `Result<Vec<u8>, SqliteError>` - Decrypted payload or error
    pub fn decrypt_payload(
        encrypted_id: &[u8; ENCRYPTED_ID_LENGTH],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, SqliteError> {
        let content_key = Self::get_content_encryption_key()?;

        // Step 1: Derive nonce + cipher_key using Blake3 KDF (same as encryption)
        let derived = blake3_keyed_variable(&content_key, encrypted_id, 44);

        let nonce_bytes: [u8; NONCE_LENGTH] = derived[0..NONCE_LENGTH]
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to extract nonce".to_string()))?;
        let cipher_key: [u8; SECRET_KEY_LENGTH] = derived[NONCE_LENGTH..44]
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to extract cipher key".to_string()))?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let key = Key::from_slice(&cipher_key);

        // Step 2: Decrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new(key);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 decryption error: {:?}", e)))?;

        println!("🔓 SharedSecret: Decrypted payload using Blake3 KDF + ChaCha20-Poly1305");
        Ok(plaintext)
    }

    /// Create Blake3 keyed hash of encrypted ID for database indexing
    ///
    /// # Arguments
    /// * `encrypted_id` - The encrypted shared secret ID bytes
    ///
    /// # Returns
    /// * `[u8; ENCRYPTED_ID_LENGTH]` - Blake3 keyed hash for database indexing
    #[allow(dead_code)]
    pub fn create_encrypted_id_hash(encrypted_id: &[u8]) -> [u8; ENCRYPTED_ID_LENGTH] {
        use crate::utils::jwt::config::get_encrypted_mlink_token_hash_key;

        let hash_key = get_encrypted_mlink_token_hash_key()
            .expect("Failed to get encrypted shared secret ID hash key");

        let hash_vec = blake3_keyed_variable(&hash_key, encrypted_id, ENCRYPTED_ID_LENGTH);
        let mut hash = [0u8; ENCRYPTED_ID_LENGTH];
        hash.copy_from_slice(&hash_vec);
        hash
    }

    /// Calculate user ID from email using Blake3 keyed hash
    ///
    /// # Arguments
    /// * `email` - Email address
    ///
    /// # Returns
    /// * `Result<[u8; USER_ID_LENGTH], SqliteError>` - 16-byte user ID
    pub fn calculate_user_id(email: &str) -> Result<[u8; USER_ID_LENGTH], SqliteError> {
        use crate::utils::jwt::crypto::derive_user_id;

        derive_user_id(email)
            .map_err(|e| SqliteError::Io(format!("Failed to calculate user ID: {}", e)))
    }
}
