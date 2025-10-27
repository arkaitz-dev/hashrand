//! Helper functions for shared secret cryptographic operations
//!
//! Provides utility functions for ID hashing, user ID calculation, and DB indexing.

use super::super::shared_secret_types::constants::*;
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;

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
    use crate::utils::pseudonimizer::blake3_keyed_variable;

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
    use crate::utils::pseudonimizer::blake3_keyed_variable;

    let db_index_key = get_shared_secret_db_index_key()
        .map_err(|e| SqliteError::Io(format!("Failed to get DB index key: {}", e)))?;

    // Concatenate reference_hash + user_id
    let mut combined = Vec::with_capacity(32);
    combined.extend_from_slice(reference_hash);
    combined.extend_from_slice(user_id);

    // Generate 32-byte db_index
    let db_index_vec = blake3_keyed_variable(&db_index_key, &combined, 32);

    let mut db_index = [0u8; 32];
    db_index.copy_from_slice(&db_index_vec[0..32]);

    debug!("🔑 SharedSecret: Generated 32-byte database index");
    Ok(db_index)
}
