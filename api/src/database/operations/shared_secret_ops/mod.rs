//! Shared secret database operations - Business logic
//!
//! Provides high-level business operations for shared secrets including
//! creation, retrieval, validation, and tracking.

pub mod payload;
mod receiver;
mod sender;
mod tracking;

use super::shared_secret_types::{SecretRole, SharedSecretPayload, constants::*};
use spin_sdk::sqlite::Error as SqliteError;

/// Shared secret operations - High-level business logic
pub struct SharedSecretOps;

impl SharedSecretOps {
    // ============================================================================
    // SENDER OPERATIONS (delegated to sender module)
    // ============================================================================

    /// Create a pair of shared secret entries with E2E encryption (high-level ECDH wrapper)
    ///
    /// This function handles the E2E encryption workflow:
    /// 1. Receives encrypted_secret (ChaCha20) + encrypted_key_material (ECDH) from frontend
    /// 2. Decrypts key_material using backend's X25519 private key + sender's X25519 public key
    /// 3. Calls create_secret_pair() with decrypted data
    ///
    /// # Arguments
    /// * `sender_email` - Sender email address
    /// * `receiver_email` - Receiver email address
    /// * `encrypted_secret` - ChaCha20-Poly1305 encrypted secret from frontend
    /// * `encrypted_key_material` - ECDH encrypted key material from frontend (60 bytes: 44 + 16 MAC)
    /// * `sender_ed25519_public_key_hex` - Sender's Ed25519 public key as hex string (64 chars)
    /// * `sender_x25519_public_key_hex` - Sender's X25519 public key as hex string (64 chars)
    /// * `otp` - Optional 9-digit OTP
    /// * `expires_hours` - Expiration in hours (1-72)
    /// * `max_reads` - Maximum reads for receiver (1-10)
    /// * `sender_db_index` - Pre-computed sender database index (32 bytes)
    /// * `receiver_db_index` - Pre-computed receiver database index (32 bytes)
    /// * `reference_hash` - Pre-generated reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<[u8; REFERENCE_HASH_LENGTH], SqliteError>` - Reference hash or error
    ///
    /// # Errors
    /// Returns error if:
    /// - Invalid sender public key format
    /// - ECDH decryption fails
    /// - Key material length mismatch
    /// - Any validation in create_secret_pair() fails
    #[allow(clippy::too_many_arguments)]
    pub fn create_secret_pair_with_ecdh(
        sender_email: &str,
        receiver_email: &str,
        encrypted_secret: &[u8],
        encrypted_key_material: &[u8],
        sender_ed25519_public_key_hex: &str,
        sender_x25519_public_key_hex: &str,
        otp: Option<String>,
        expires_hours: i64,
        max_reads: i64,
        sender_db_index: &[u8; 32],
        receiver_db_index: &[u8; 32],
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<[u8; REFERENCE_HASH_LENGTH], SqliteError> {
        sender::create_secret_pair_with_ecdh(
            sender_email,
            receiver_email,
            encrypted_secret,
            encrypted_key_material,
            sender_ed25519_public_key_hex,
            sender_x25519_public_key_hex,
            otp,
            expires_hours,
            max_reads,
            sender_db_index,
            receiver_db_index,
            reference_hash,
        )
    }

    // ============================================================================
    // RECEIVER OPERATIONS (delegated to receiver module)
    // ============================================================================

    /// Read a secret, decrypt, and get pending_reads from tracking (v3 - NEW with centralized payload)
    ///
    /// # Arguments
    /// * `db_index` - Database index (32 bytes) - PRIMARY KEY
    /// * `reference_hash` - Reference hash (16 bytes) - Required for payload retrieval
    ///
    /// # Returns
    /// * `Result<(SharedSecretPayload, i64, i64, SecretRole), SqliteError>` - (payload, pending_reads, expires_at, role) or error
    ///
    /// Note: Role is returned from database for backward compatibility, but should be validated from hash checksum
    pub fn read_secret(
        db_index: &[u8; DB_INDEX_LENGTH],
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<(SharedSecretPayload, i64, i64, SecretRole), SqliteError> {
        receiver::read_secret(db_index, reference_hash)
    }

    /// Validate OTP against stored OTP in payload
    ///
    /// # Arguments
    /// * `stored_otp` - Optional OTP from payload
    /// * `provided_otp` - Optional OTP provided by user
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if valid, false/error otherwise
    #[allow(dead_code)]
    pub fn validate_otp(
        stored_otp: &Option<String>,
        provided_otp: &Option<String>,
    ) -> Result<bool, SqliteError> {
        receiver::validate_otp(stored_otp, provided_otp)
    }

    // ============================================================================
    // TRACKING OPERATIONS (delegated to tracking module)
    // ============================================================================

    /// Confirm read by updating tracking record
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if updated, false if already set
    pub fn confirm_read(reference_hash: &[u8; REFERENCE_HASH_LENGTH]) -> Result<bool, SqliteError> {
        tracking::confirm_read(reference_hash)
    }

    /// Clean up expired secrets and tracking
    #[allow(dead_code)]
    pub fn cleanup_expired() -> Result<(u32, u32), SqliteError> {
        tracking::cleanup_expired()
    }
}
