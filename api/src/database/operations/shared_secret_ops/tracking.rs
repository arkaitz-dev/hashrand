//! Tracking operations for shared secrets
//!
//! Handles tracking-related operations (read confirmation, cleanup).

use super::super::shared_secret_storage::SharedSecretStorage;
use super::super::shared_secret_types::constants::*;
use spin_sdk::sqlite::Error as SqliteError;

/// Confirm read by updating tracking record
///
/// # Arguments
/// * `reference_hash` - Reference hash (16 bytes)
///
/// # Returns
/// * `Result<bool, SqliteError>` - true if updated, false if already set
pub fn confirm_read(reference_hash: &[u8; REFERENCE_HASH_LENGTH]) -> Result<bool, SqliteError> {
    SharedSecretStorage::update_tracking_read(reference_hash)
}

/// Clean up expired secrets and tracking
#[allow(dead_code)]
pub fn cleanup_expired() -> Result<(u32, u32), SqliteError> {
    SharedSecretStorage::cleanup_expired()
}
