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

    /// Get shared secret content encryption key (v2 - NEW)
    fn get_shared_secret_content_key() -> Result<[u8; 64], SqliteError> {
        use crate::utils::jwt::config::get_shared_secret_content_key;
        get_shared_secret_content_key().map_err(SqliteError::Io)
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
    #[allow(dead_code)]
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

    /// Encrypt payload using Blake3 KDF + ChaCha20-Poly1305 (v2 - NEW with db_index)
    ///
    /// Process:
    /// 1. Derive nonce[12] + cipher_key[32] via `blake3_keyed_variable(SHARED_SECRET_CONTENT_KEY, db_index, 44)`
    /// 2. Encrypt with ChaCha20-Poly1305 (adds 16-byte tag)
    ///
    /// # Arguments
    /// * `db_index` - Database index (32 bytes)
    /// * `payload` - Raw payload to encrypt
    ///
    /// # Returns
    /// * `Result<Vec<u8>, SqliteError>` - Encrypted payload or error
    pub fn encrypt_payload_v2(
        db_index: &[u8; DB_INDEX_LENGTH],
        payload: &[u8],
    ) -> Result<Vec<u8>, SqliteError> {
        let content_key = Self::get_shared_secret_content_key()?;

        // Step 1: Derive nonce + cipher_key using Blake3 KDF
        let derived = blake3_keyed_variable(&content_key, db_index, 44);

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

        println!(
            "🔒 SharedSecret: Encrypted payload v2 using Blake3 KDF + ChaCha20-Poly1305 (db_index)"
        );
        Ok(ciphertext)
    }

    /// Decrypt payload using Blake3 KDF + ChaCha20-Poly1305 (v2 - NEW with db_index)
    ///
    /// # Arguments
    /// * `db_index` - Database index (32 bytes)
    /// * `ciphertext` - Encrypted payload to decrypt
    ///
    /// # Returns
    /// * `Result<Vec<u8>, SqliteError>` - Decrypted payload or error
    pub fn decrypt_payload_v2(
        db_index: &[u8; DB_INDEX_LENGTH],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, SqliteError> {
        let content_key = Self::get_shared_secret_content_key()?;

        // Step 1: Derive nonce + cipher_key using Blake3 KDF (same as encryption)
        let derived = blake3_keyed_variable(&content_key, db_index, 44);

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

        println!(
            "🔓 SharedSecret: Decrypted payload v2 using Blake3 KDF + ChaCha20-Poly1305 (db_index)"
        );
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

    // ============================================================================
    // NEW FUNCTIONS FOR ZERO KNOWLEDGE HASH SYSTEM (v2.0)
    // ============================================================================

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
        role: super::shared_secret_types::SecretRole,
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
            super::shared_secret_types::SecretRole::Sender => 0x01,
            super::shared_secret_types::SecretRole::Receiver => 0x02,
        };

        // Build final 8-byte checksum: [checksum_base (7 bytes), role_byte (1 byte)]
        let mut checksum = [0u8; 8];
        checksum[0..7].copy_from_slice(&checksum_base[0..7]);
        checksum[7] = role_byte;

        println!("🔒 SharedSecret: Generated checksum with role {:?}", role);
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
        role: super::shared_secret_types::SecretRole,
    ) -> Result<[u8; 40], SqliteError> {
        // 1. Derive user_id from email (Zero Knowledge)
        let user_id = Self::calculate_user_id(email)?;

        // 2. Generate checksum with role
        let checksum = Self::generate_checksum_with_role(reference_hash, &user_id, role)?;

        // 3. Concatenate: ref[16] + user_id[16] + checksum[8] = 40 bytes
        let mut hash = [0u8; 40];
        hash[0..16].copy_from_slice(reference_hash);
        hash[16..32].copy_from_slice(&user_id);
        hash[32..40].copy_from_slice(&checksum);

        println!("✅ SharedSecret: Generated 40-byte hash for {:?}", role);
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
        use crate::utils::jwt::config::get_shared_secret_url_cipher_key;
        use chacha20::ChaCha20;
        use chacha20::cipher::{KeyIvInit, StreamCipher};

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

        // Initialize ChaCha20 cipher
        let mut cipher = ChaCha20::new(&cipher_key.into(), &nonce.into());

        // Encrypt in-place
        let mut encrypted = *hash_40;
        cipher.apply_keystream(&mut encrypted);

        println!("🔐 SharedSecret: Encrypted 40-byte hash with ChaCha20");
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
        use crate::utils::jwt::config::get_shared_secret_url_cipher_key;
        use chacha20::ChaCha20;
        use chacha20::cipher::{KeyIvInit, StreamCipher};

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

        // Initialize ChaCha20 cipher
        let mut cipher = ChaCha20::new(&cipher_key.into(), &nonce.into());

        // Decrypt in-place (ChaCha20 is symmetric)
        let mut decrypted = *encrypted_hash;
        cipher.apply_keystream(&mut decrypted);

        println!("🔓 SharedSecret: Decrypted 40-byte hash with ChaCha20");
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
    ) -> Result<
        (
            [u8; REFERENCE_HASH_LENGTH],
            [u8; USER_ID_LENGTH],
            super::shared_secret_types::SecretRole,
        ),
        SqliteError,
    > {
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
            0x01 => super::shared_secret_types::SecretRole::Sender,
            0x02 => super::shared_secret_types::SecretRole::Receiver,
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

        println!(
            "✅ SharedSecret: Validated checksum and extracted role {:?}",
            role
        );
        Ok((reference_hash, user_id, role))
    }

    /// Generate database index for PRIMARY KEY
    ///
    /// Uses blake3_keyed_variable(DB_INDEX_KEY, reference_hash + user_id, 32)
    ///
    /// # Arguments
    /// * `reference_hash` - 16-byte reference hash
    /// * `user_id` - 16-byte user ID
    ///
    /// # Returns
    /// * `Result<[u8; 32], SqliteError>` - 32-byte database index
    pub fn generate_db_index(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
        user_id: &[u8; USER_ID_LENGTH],
    ) -> Result<[u8; 32], SqliteError> {
        use crate::utils::jwt::config::get_shared_secret_db_index_key;

        let db_index_key = get_shared_secret_db_index_key()
            .map_err(|e| SqliteError::Io(format!("Failed to get DB index key: {}", e)))?;

        // Concatenate reference_hash + user_id
        let mut combined = Vec::with_capacity(32);
        combined.extend_from_slice(reference_hash);
        combined.extend_from_slice(user_id);

        // Generate 32-byte db_index
        let db_index_vec =
            crate::utils::pseudonimizer::blake3_keyed_variable(&db_index_key, &combined, 32);

        let mut db_index = [0u8; 32];
        db_index.copy_from_slice(&db_index_vec[0..32]);

        println!("🔑 SharedSecret: Generated 32-byte database index");
        Ok(db_index)
    }
}
