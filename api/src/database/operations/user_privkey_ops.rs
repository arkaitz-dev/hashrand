//! User private key context cryptographic operations
//!
//! Provides cryptographic functions for user private key context using
//! Blake3 KDF and ChaCha20-Poly1305 AEAD encryption.

use crate::utils::pseudonimizer::blake3_keyed_variable;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce, aead::Aead};
use spin_sdk::sqlite::Error as SqliteError;

/// User private key context cryptographic operations
pub struct UserPrivkeyCrypto;

impl UserPrivkeyCrypto {
    /// Get user private key index key from environment
    ///
    /// # Returns
    /// * `Result<[u8; 64], SqliteError>` - 64-byte index key
    fn get_index_key() -> Result<[u8; 64], SqliteError> {
        use crate::utils::jwt::config::get_user_privkey_index_key;
        get_user_privkey_index_key().map_err(SqliteError::Io)
    }

    /// Get user private key encryption key from environment
    ///
    /// # Returns
    /// * `Result<[u8; 64], SqliteError>` - 64-byte encryption key
    fn get_encryption_key() -> Result<[u8; 64], SqliteError> {
        use crate::utils::jwt::config::get_user_privkey_encryption_key;
        get_user_privkey_encryption_key().map_err(SqliteError::Io)
    }

    /// Generate database index from argon2 output
    ///
    /// Process: blake3_keyed_variable(INDEX_KEY, argon2_output[32], 16) → db_index[16]
    ///
    /// # Arguments
    /// * `argon2_output` - 32-byte argon2id output from user_id derivation pipeline (step 4)
    ///
    /// # Returns
    /// * `Result<[u8; 16], SqliteError>` - 16-byte database index
    pub fn generate_db_index(argon2_output: &[u8; 32]) -> Result<[u8; 16], SqliteError> {
        let index_key = Self::get_index_key()?;

        // Generate 16-byte database index
        let index_vec = blake3_keyed_variable(&index_key, argon2_output, 16);

        let mut db_index = [0u8; 16];
        db_index.copy_from_slice(&index_vec);

        Ok(db_index)
    }

    /// Encrypt 64 random bytes for private key context
    ///
    /// Process:
    /// 1. blake3_keyed_variable(ENCRYPTION_KEY, db_index[16], 44) → nonce[12] + cipher_key[32]
    /// 2. ChaCha20-Poly1305.encrypt(random_64_bytes, nonce, cipher_key) → encrypted_blob
    ///
    /// # Arguments
    /// * `db_index` - 16-byte database index (used to derive nonce/key deterministically)
    /// * `random_data` - 64 random bytes to encrypt
    ///
    /// # Returns
    /// * `Result<Vec<u8>, SqliteError>` - Encrypted data (64 bytes + 16 bytes MAC = 80 bytes total)
    pub fn encrypt_privkey_context(
        db_index: &[u8; 16],
        random_data: &[u8; 64],
    ) -> Result<Vec<u8>, SqliteError> {
        let encryption_key = Self::get_encryption_key()?;

        // Step 1: Derive nonce + cipher_key using Blake3 KDF from db_index
        let derived = blake3_keyed_variable(&encryption_key, db_index, 44);

        let nonce_bytes: [u8; 12] = derived[0..12]
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to extract nonce".to_string()))?;
        let cipher_key: [u8; 32] = derived[12..44]
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to extract cipher key".to_string()))?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let key = Key::from_slice(&cipher_key);

        // Step 2: Encrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new(key);
        let ciphertext = cipher
            .encrypt(nonce, random_data.as_ref())
            .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 encryption error: {:?}", e)))?;

        Ok(ciphertext)
    }

    /// Decrypt private key context
    ///
    /// # Arguments
    /// * `db_index` - 16-byte database index (used to derive nonce/key deterministically)
    /// * `encrypted_data` - Encrypted private key context (80 bytes: 64 + 16 MAC)
    ///
    /// # Returns
    /// * `Result<[u8; 64], SqliteError>` - Decrypted 64 random bytes
    pub fn decrypt_privkey_context(
        db_index: &[u8; 16],
        encrypted_data: &[u8],
    ) -> Result<[u8; 64], SqliteError> {
        let encryption_key = Self::get_encryption_key()?;

        // Step 1: Derive nonce + cipher_key using Blake3 KDF from db_index (same as encryption)
        let derived = blake3_keyed_variable(&encryption_key, db_index, 44);

        let nonce_bytes: [u8; 12] = derived[0..12]
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to extract nonce".to_string()))?;
        let cipher_key: [u8; 32] = derived[12..44]
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to extract cipher key".to_string()))?;

        let nonce = Nonce::from_slice(&nonce_bytes);
        let key = Key::from_slice(&cipher_key);

        // Step 2: Decrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new(key);
        let plaintext = cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 decryption error: {:?}", e)))?;

        // Convert to [u8; 64]
        if plaintext.len() != 64 {
            return Err(SqliteError::Io(format!(
                "Expected 64 bytes after decryption, got {}",
                plaintext.len()
            )));
        }

        let mut result = [0u8; 64];
        result.copy_from_slice(&plaintext);

        Ok(result)
    }

    /// Ensure user private key context entry exists (create if missing)
    ///
    /// Process:
    /// 1. Check if entry exists with db_index
    /// 2. If NOT exists:
    ///    - Generate 64 random bytes
    ///    - Encrypt with ChaCha20-Poly1305 (nonce/key from db_index)
    ///    - Insert into user_privkey_context
    /// 3. If exists: do nothing (idempotent)
    ///
    /// # Arguments
    /// * `db_index` - 16-byte database index
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or error
    pub fn ensure_user_privkey_context_exists(
        db_index: &[u8; 16],
    ) -> Result<(), SqliteError> {
        use crate::database::get_database_connection;
        use rand::RngCore;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;
        use spin_sdk::sqlite::Value;

        let connection = get_database_connection()?;

        // Check if entry exists
        let result = connection.execute(
            "SELECT db_index FROM user_privkey_context WHERE db_index = ?",
            &[Value::Blob(db_index.to_vec())],
        )?;

        if !result.rows.is_empty() {
            // Entry already exists, nothing to do
            return Ok(());
        }

        // Generate 64 random bytes using ChaCha8Rng
        let seed_material = format!("{:?}_privkey", std::time::SystemTime::now());
        let seed_hash = blake3::hash(seed_material.as_bytes());
        let seed: [u8; 32] = *seed_hash.as_bytes();

        let mut rng = ChaCha8Rng::from_seed(seed);
        let mut random_64_bytes = [0u8; 64];
        rng.fill_bytes(&mut random_64_bytes);

        // Encrypt random bytes
        let encrypted_privkey = UserPrivkeyCrypto::encrypt_privkey_context(db_index, &random_64_bytes)?;

        // Insert into database
        connection.execute(
            "INSERT INTO user_privkey_context (db_index, encrypted_privkey) VALUES (?, ?)",
            &[
                Value::Blob(db_index.to_vec()),
                Value::Blob(encrypted_privkey),
            ],
        )?;

        Ok(())
    }
}
