//! Shared secret storage operations
//!
//! Provides database storage functions for shared secrets including
//! storage, retrieval, deletion, tracking, and cleanup operations.

mod cleanup;
mod deletion;
mod retrieval;
mod storage;
mod tracking;

use super::shared_secret_types::{SecretRole, constants::*};
use spin_sdk::sqlite::Error as SqliteError;

// Re-export type aliases
pub use retrieval::SecretData;

/// Type alias for secret retrieval result tuple v2: (encrypted_payload, expires_at) - NO ROLE
#[allow(dead_code)]
pub type SecretDataV2 = (Vec<u8>, i64);

/// Shared secret storage operations
pub struct SharedSecretStorage;

impl SharedSecretStorage {
    // ============================================================================
    // STORAGE OPERATIONS (delegated to storage module)
    // ============================================================================

    /// Store a shared secret entry in the database (v2 - with db_index)
    ///
    /// # Arguments
    /// * `db_index` - Database index (32 bytes) - PRIMARY KEY
    /// * `encrypted_payload` - Encrypted payload blob
    /// * `expires_at` - Expiration timestamp in hours since Unix epoch
    /// * `role` - 'sender' or 'receiver' (TEMPORARY - will be removed when schema updated)
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or database error
    pub fn store_shared_secret(
        db_index: &[u8; DB_INDEX_LENGTH],
        encrypted_payload: &[u8],
        expires_at: i64,
        role: SecretRole,
    ) -> Result<(), SqliteError> {
        storage::store_shared_secret(db_index, encrypted_payload, expires_at, role)
    }

    /// Store a shared secret entry in the database (OLD - deprecated)
    ///
    /// # Arguments
    /// * `id` - Encrypted ID (32 bytes)
    /// * `encrypted_payload` - Encrypted payload blob
    /// * `expires_at` - Expiration timestamp in hours since Unix epoch
    /// * `role` - 'sender' or 'receiver'
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or database error
    #[allow(dead_code)]
    pub fn store_shared_secret_old(
        id: &[u8; ENCRYPTED_ID_LENGTH],
        encrypted_payload: &[u8],
        expires_at: i64,
        role: SecretRole,
    ) -> Result<(), SqliteError> {
        storage::store_shared_secret_old(id, encrypted_payload, expires_at, role)
    }

    // ============================================================================
    // RETRIEVAL OPERATIONS (delegated to retrieval module)
    // ============================================================================

    /// Retrieve a shared secret by db_index (v2 - with db_index)
    ///
    /// # Arguments
    /// * `db_index` - Database index (32 bytes) - PRIMARY KEY
    ///
    /// # Returns
    /// * `Result<Option<SecretData>, SqliteError>` - (encrypted_payload, expires_at, role) or None
    ///
    /// Note: Role is still returned for backward compatibility but will be removed in future
    pub fn retrieve_secret(
        db_index: &[u8; DB_INDEX_LENGTH],
    ) -> Result<Option<SecretData>, SqliteError> {
        retrieval::retrieve_secret(db_index)
    }

    /// Retrieve a shared secret by encrypted ID (OLD - deprecated)
    ///
    /// # Arguments
    /// * `id` - Encrypted ID (32 bytes)
    ///
    /// # Returns
    /// * `Result<Option<SecretData>, SqliteError>` - (encrypted_payload, expires_at, role) or None
    #[allow(dead_code)]
    pub fn retrieve_secret_old(
        id: &[u8; ENCRYPTED_ID_LENGTH],
    ) -> Result<Option<SecretData>, SqliteError> {
        retrieval::retrieve_secret_old(id)
    }

    // ============================================================================
    // DELETION OPERATIONS (delegated to deletion module)
    // ============================================================================

    /// Delete a shared secret by db_index (v2 - with db_index)
    ///
    /// # Arguments
    /// * `db_index` - Database index (32 bytes) - PRIMARY KEY
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if deleted, false if not found
    pub fn delete_secret(db_index: &[u8; DB_INDEX_LENGTH]) -> Result<bool, SqliteError> {
        deletion::delete_secret(db_index)
    }

    /// Delete a shared secret by encrypted ID (OLD - deprecated)
    ///
    /// # Arguments
    /// * `id` - Encrypted ID (32 bytes)
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if deleted, false if not found
    #[allow(dead_code)]
    pub fn delete_secret_old(id: &[u8; ENCRYPTED_ID_LENGTH]) -> Result<bool, SqliteError> {
        deletion::delete_secret_old(id)
    }

    // ============================================================================
    // TRACKING OPERATIONS (delegated to tracking module)
    // ============================================================================

    /// Get pending_reads from tracking table by reference_hash
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<Option<i64>, SqliteError>` - pending_reads or None if not found
    pub fn get_pending_reads_from_tracking(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<Option<i64>, SqliteError> {
        tracking::get_pending_reads_from_tracking(reference_hash)
    }

    /// Check if tracking record exists by reference_hash
    ///
    /// # Arguments
    /// * `reference_hash` - 32-byte reference hash
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if exists, false if not
    pub fn tracking_exists(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<bool, SqliteError> {
        tracking::tracking_exists(reference_hash)
    }

    /// Get read_at timestamp from tracking table by reference_hash
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<Option<i64>, SqliteError>` - read_at timestamp (seconds) or None if not set/not found
    pub fn get_read_at_from_tracking(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<Option<i64>, SqliteError> {
        tracking::get_read_at_from_tracking(reference_hash)
    }

    /// Decrement pending_reads in tracking table
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<i64, SqliteError>` - New pending_reads value
    ///
    /// Decrement tracking reads (simple decrement, no idempotency)
    /// Frontend handles duplicate prevention via IndexedDB cache
    pub fn decrement_tracking_reads(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<i64, SqliteError> {
        tracking::decrement_tracking_reads(reference_hash)
    }

    /// Store tracking record with encrypted payload (v3 - NEW)
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    /// * `pending_reads` - Initial pending_reads counter
    /// * `expires_at` - Expiration timestamp in hours
    /// * `encrypted_payload` - Encrypted payload blob (NEW)
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or error
    pub fn store_tracking_with_payload(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
        pending_reads: i64,
        expires_at: i64,
        encrypted_payload: &[u8],
    ) -> Result<(), SqliteError> {
        tracking::store_tracking_with_payload(
            reference_hash,
            pending_reads,
            expires_at,
            encrypted_payload,
        )
    }

    /// Retrieve encrypted payload from tracking table (v3 - NEW)
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<Option<Vec<u8>>, SqliteError>` - Encrypted payload or None
    pub fn retrieve_tracking_payload(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<Option<Vec<u8>>, SqliteError> {
        tracking::retrieve_tracking_payload(reference_hash)
    }

    /// Delete tracking record by reference_hash
    ///
    /// # Arguments
    /// * `reference_hash` - 32-byte reference hash
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if deleted, false if not found
    pub fn delete_tracking_by_reference_hash(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<bool, SqliteError> {
        tracking::delete_tracking_by_reference_hash(reference_hash)
    }

    /// Update tracking record with read timestamp (only if not already set)
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if updated, false if already set or not found
    pub fn update_tracking_read(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<bool, SqliteError> {
        tracking::update_tracking_read(reference_hash)
    }

    // ============================================================================
    // CLEANUP OPERATIONS (delegated to cleanup module)
    // ============================================================================

    /// Clean up expired shared secrets and tracking records
    ///
    /// # Returns
    /// * `Result<(u32, u32), SqliteError>` - (secrets_deleted, tracking_deleted) or error
    #[allow(dead_code)]
    pub fn cleanup_expired() -> Result<(u32, u32), SqliteError> {
        cleanup::cleanup_expired()
    }
}
