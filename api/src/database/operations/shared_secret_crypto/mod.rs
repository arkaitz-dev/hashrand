//! Shared secret cryptographic operations
//!
//! Provides cryptographic functions for shared secret security using
//! Blake3 KDF, ChaCha20-Poly1305 AEAD encryption, and random generation.

mod helpers;
mod key_material;
mod payload;
mod random;
mod url_hash;

use super::shared_secret_types::constants::*;
use super::shared_secret_types::SecretRole;
use spin_sdk::sqlite::Error as SqliteError;

/// Shared secret cryptographic operations
pub struct SharedSecretCrypto;

impl SharedSecretCrypto {
    // ============================================================================
    // RANDOM GENERATION (delegated to random module)
    // ============================================================================

    /// Generate cryptographically secure random reference hash
    ///
    /// Uses ChaCha8Rng for secure random generation with Blake3 seed
    ///
    /// # Returns
    /// * `[u8; 16]` - Random 16-byte reference hash
    pub fn generate_reference_hash() -> [u8; REFERENCE_HASH_LENGTH] {
        random::generate_reference_hash()
    }

    /// Generate cryptographically secure 9-digit OTP
    ///
    /// Uses ChaCha8Rng to generate a random number between 100000000 and 999999999
    ///
    /// # Returns
    /// * `String` - 9-digit OTP as string
    pub fn generate_otp() -> String {
        random::generate_otp()
    }

    /// Generate cryptographically secure random key material for payload encryption
    ///
    /// Uses ChaCha8Rng for secure random generation with Blake3 seed
    ///
    /// # Returns
    /// * `[u8; 44]` - Random 44-byte key material (nonce[12] + cipher_key[32])
    #[allow(dead_code)]
    pub fn generate_random_key_material() -> [u8; KEY_MATERIAL_LENGTH] {
        random::generate_random_key_material()
    }

    // ============================================================================
    // KEY MATERIAL ENCRYPTION (delegated to key_material module)
    // ============================================================================

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
        key_material::encrypt_key_material(db_index, key_material)
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
        key_material::decrypt_key_material(db_index, ciphertext)
    }

    // ============================================================================
    // PAYLOAD ENCRYPTION (delegated to payload module)
    // ============================================================================

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
        payload::encrypt_payload_with_material(key_material, payload)
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
        payload::decrypt_payload_with_material(key_material, ciphertext)
    }

    // ============================================================================
    // HELPERS (delegated to helpers module)
    // ============================================================================

    /// Create Blake3 keyed hash of encrypted ID for database indexing
    ///
    /// # Arguments
    /// * `encrypted_id` - The encrypted shared secret ID bytes
    ///
    /// # Returns
    /// * `[u8; ENCRYPTED_ID_LENGTH]` - Blake3 keyed hash for database indexing
    #[allow(dead_code)]
    pub fn create_encrypted_id_hash(encrypted_id: &[u8]) -> [u8; ENCRYPTED_ID_LENGTH] {
        helpers::create_encrypted_id_hash(encrypted_id)
    }

    /// Calculate user ID from email using Blake3 keyed hash
    ///
    /// # Arguments
    /// * `email` - Email address
    ///
    /// # Returns
    /// * `Result<[u8; USER_ID_LENGTH], SqliteError>` - 16-byte user ID
    pub fn calculate_user_id(email: &str) -> Result<[u8; USER_ID_LENGTH], SqliteError> {
        helpers::calculate_user_id(email)
    }

    // ============================================================================
    // URL HASH OPERATIONS (delegated to url_hash module)
    // ============================================================================

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
        role: SecretRole,
    ) -> Result<[u8; 40], SqliteError> {
        url_hash::generate_shared_secret_hash(reference_hash, email, role)
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
        url_hash::encrypt_url_hash(hash_40)
    }

    /// Decrypt 40-byte hash using ChaCha20 stream cipher
    ///
    /// # Arguments
    /// * `encrypted_hash` - Encrypted 40-byte hash from URL (Base58 decoded)
    ///
    /// # Returns
    /// * `Result<[u8; 40], SqliteError>` - Decrypted 40-byte hash
    pub fn decrypt_url_hash(encrypted_hash: &[u8; 40]) -> Result<[u8; 40], SqliteError> {
        url_hash::decrypt_url_hash(encrypted_hash)
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
            SecretRole,
        ),
        SqliteError,
    > {
        url_hash::validate_and_extract_hash(hash_40)
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
        helpers::generate_db_index(reference_hash, user_id)
    }
}
