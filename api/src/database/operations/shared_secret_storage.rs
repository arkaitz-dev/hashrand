//! Shared secret storage operations
//!
//! Provides database storage functions for shared secrets including
//! storage, retrieval, deletion, tracking, and cleanup operations.

use super::shared_secret_types::{SecretRole, constants::*};
use crate::database::get_database_connection;
use chrono::Utc;
use spin_sdk::sqlite::{Error as SqliteError, Value};
use tracing::{debug, warn};

/// Type alias for secret retrieval result tuple: (encrypted_payload, expires_at, role)
type SecretData = (Vec<u8>, i64, SecretRole);

/// Type alias for secret retrieval result tuple v2: (encrypted_payload, expires_at) - NO ROLE
#[allow(dead_code)]
type SecretDataV2 = (Vec<u8>, i64);

/// Shared secret storage operations
pub struct SharedSecretStorage;

impl SharedSecretStorage {
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
        let connection = get_database_connection()?;

        // println!(
        //     "🔒 SharedSecret: Storing secret with role '{}', expires_at={} (using db_index)",
        //     role.to_str(),
        //     expires_at
        // );
        debug!(
            "🔒 SharedSecret: Storing secret with role '{}', expires_at={} (using db_index)",
            role.to_str(),
            expires_at
        );

        connection.execute(
            "INSERT INTO shared_secrets (id, encrypted_payload, expires_at, role) VALUES (?, ?, ?, ?)",
            &[
                Value::Blob(db_index.to_vec()),
                Value::Blob(encrypted_payload.to_vec()),
                Value::Integer(expires_at),
                Value::Text(role.to_str().to_string()),
            ],
        )?;

        // println!("✅ SharedSecret: Stored successfully with db_index");
        debug!("✅ SharedSecret: Stored successfully with db_index");
        Ok(())
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
        let connection = get_database_connection()?;

        // println!(
        //     "🔒 SharedSecret: Storing secret with role '{}', expires_at={}",
        //     role.to_str(),
        //     expires_at
        // );
        debug!(
            "🔒 SharedSecret: Storing secret with role '{}', expires_at={}",
            role.to_str(),
            expires_at
        );

        connection.execute(
            "INSERT INTO shared_secrets (id, encrypted_payload, expires_at, role) VALUES (?, ?, ?, ?)",
            &[
                Value::Blob(id.to_vec()),
                Value::Blob(encrypted_payload.to_vec()),
                Value::Integer(expires_at),
                Value::Text(role.to_str().to_string()),
            ],
        )?;

        // println!("✅ SharedSecret: Stored successfully");
        debug!("✅ SharedSecret: Stored successfully");
        Ok(())
    }

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
        let connection = get_database_connection()?;

        let result = connection.execute(
            "SELECT encrypted_payload, expires_at, role FROM shared_secrets WHERE id = ?",
            &[Value::Blob(db_index.to_vec())],
        )?;

        if let Some(row) = result.rows.first() {
            let encrypted_payload = match &row.values[0] {
                Value::Blob(data) => data.clone(),
                _ => {
                    return Err(SqliteError::Io(
                        "Invalid encrypted_payload type".to_string(),
                    ));
                }
            };

            let expires_at = match &row.values[1] {
                Value::Integer(val) => *val,
                _ => return Err(SqliteError::Io("Invalid expires_at type".to_string())),
            };

            let role_str = match &row.values[2] {
                Value::Text(val) => val.clone(),
                _ => return Err(SqliteError::Io("Invalid role type".to_string())),
            };

            let role = SecretRole::from_str(&role_str)
                .ok_or_else(|| SqliteError::Io(format!("Invalid role value: {}", role_str)))?;

            // println!(
            //     "🔍 SharedSecret: Retrieved with db_index (role={}, expires_at={})",
            //     role.to_str(),
            //     expires_at
            // );
            debug!(
                "🔍 SharedSecret: Retrieved with db_index (role={}, expires_at={})",
                role.to_str(),
                expires_at
            );

            Ok(Some((encrypted_payload, expires_at, role)))
        } else {
            // println!("🔍 SharedSecret: Not found (db_index)");
            warn!("🔍 SharedSecret: Not found (db_index)");
            Ok(None)
        }
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
        let connection = get_database_connection()?;

        let result = connection.execute(
            "SELECT encrypted_payload, expires_at, role FROM shared_secrets WHERE id = ?",
            &[Value::Blob(id.to_vec())],
        )?;

        if let Some(row) = result.rows.first() {
            let encrypted_payload = match &row.values[0] {
                Value::Blob(data) => data.clone(),
                _ => {
                    return Err(SqliteError::Io(
                        "Invalid encrypted_payload type".to_string(),
                    ));
                }
            };

            let expires_at = match &row.values[1] {
                Value::Integer(val) => *val,
                _ => return Err(SqliteError::Io("Invalid expires_at type".to_string())),
            };

            let role_str = match &row.values[2] {
                Value::Text(val) => val.clone(),
                _ => return Err(SqliteError::Io("Invalid role type".to_string())),
            };

            let role = SecretRole::from_str(&role_str)
                .ok_or_else(|| SqliteError::Io(format!("Invalid role value: {}", role_str)))?;

            // println!(
            //     "🔍 SharedSecret: Retrieved (role={}, expires_at={})",
            //     role.to_str(),
            //     expires_at
            // );
            debug!(
                "🔍 SharedSecret: Retrieved (role={}, expires_at={})",
                role.to_str(),
                expires_at
            );

            Ok(Some((encrypted_payload, expires_at, role)))
        } else {
            // println!("⚠️  SharedSecret: Not found in database");
            warn!("⚠️  SharedSecret: Not found in database");
            Ok(None)
        }
    }

    /// Delete a shared secret by db_index (v2 - with db_index)
    ///
    /// # Arguments
    /// * `db_index` - Database index (32 bytes) - PRIMARY KEY
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if deleted, false if not found
    pub fn delete_secret(db_index: &[u8; DB_INDEX_LENGTH]) -> Result<bool, SqliteError> {
        let connection = get_database_connection()?;

        // Check if exists first
        if Self::retrieve_secret(db_index)?.is_none() {
            return Ok(false);
        }

        connection.execute(
            "DELETE FROM shared_secrets WHERE id = ?",
            &[Value::Blob(db_index.to_vec())],
        )?;

        // println!("🗑️  SharedSecret: Deleted successfully (db_index)");
        debug!("🗑️  SharedSecret: Deleted successfully (db_index)");
        Ok(true)
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
        let connection = get_database_connection()?;

        // Check if exists first
        if Self::retrieve_secret_old(id)?.is_none() {
            return Ok(false);
        }

        connection.execute(
            "DELETE FROM shared_secrets WHERE id = ?",
            &[Value::Blob(id.to_vec())],
        )?;

        // println!("🗑️  SharedSecret: Deleted successfully");
        debug!("🗑️  SharedSecret: Deleted successfully");
        Ok(true)
    }

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
        let connection = get_database_connection()?;

        let result = connection.execute(
            "SELECT pending_reads FROM shared_secrets_tracking WHERE reference_hash = ?",
            &[Value::Blob(reference_hash.to_vec())],
        )?;

        if let Some(row) = result.rows.first() {
            let pending_reads = match &row.values[0] {
                Value::Integer(val) => *val,
                _ => return Err(SqliteError::Io("Invalid pending_reads type".to_string())),
            };
            Ok(Some(pending_reads))
        } else {
            Ok(None)
        }
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
        Ok(Self::get_pending_reads_from_tracking(reference_hash)?.is_some())
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
        let connection = get_database_connection()?;

        // Get current pending_reads
        let pending_reads = Self::get_pending_reads_from_tracking(reference_hash)?
            .ok_or_else(|| SqliteError::Io("Tracking record not found".to_string()))?;

        // Don't decrement if sender (unlimited reads = -1)
        if pending_reads == UNLIMITED_READS {
            // println!("📖 SharedSecret: Sender has unlimited reads, not decrementing");
            debug!("📖 SharedSecret: Sender has unlimited reads, not decrementing");
            return Ok(UNLIMITED_READS);
        }

        // Don't decrement if already at 0 or negative
        if pending_reads <= 0 {
            // println!("⚠️  SharedSecret: Already at 0 reads");
            warn!("⚠️  SharedSecret: Already at 0 reads");
            return Ok(0);
        }

        let new_reads = pending_reads - 1;

        // Update counter
        connection.execute(
            "UPDATE shared_secrets_tracking SET pending_reads = ? WHERE reference_hash = ?",
            &[
                Value::Integer(new_reads),
                Value::Blob(reference_hash.to_vec()),
            ],
        )?;

        // println!("📖 SharedSecret: Decremented to {} reads", new_reads);
        debug!("📖 SharedSecret: Decremented to {} reads", new_reads);
        Ok(new_reads)
    }

    /// Store tracking record for a shared secret
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    /// * `pending_reads` - Initial pending_reads counter
    /// * `expires_at` - Expiration timestamp in hours
    /// * `created_at` - Creation timestamp in seconds
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or error
    pub fn store_tracking(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
        pending_reads: i64,
        expires_at: i64,
        created_at: i64,
    ) -> Result<(), SqliteError> {
        let connection = get_database_connection()?;

        // println!(
        //     "📊 SharedSecret: Storing tracking record (pending_reads={}, expires_at={}, created_at={})",
        //     pending_reads, expires_at, created_at
        // );
        debug!(
            "📊 SharedSecret: Storing tracking record (pending_reads={}, expires_at={}, created_at={})",
            pending_reads, expires_at, created_at
        );

        connection.execute(
            "INSERT INTO shared_secrets_tracking (reference_hash, pending_reads, read_at, expires_at, created_at) VALUES (?, ?, NULL, ?, ?)",
            &[
                Value::Blob(reference_hash.to_vec()),
                Value::Integer(pending_reads),
                Value::Integer(expires_at),
                Value::Integer(created_at),
            ],
        )?;

        // println!("✅ SharedSecret: Tracking record stored");
        debug!("✅ SharedSecret: Tracking record stored");
        Ok(())
    }

    /// Store tracking record with encrypted payload (v3 - NEW)
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    /// * `pending_reads` - Initial pending_reads counter
    /// * `expires_at` - Expiration timestamp in hours
    /// * `created_at` - Creation timestamp in seconds
    /// * `encrypted_payload` - Encrypted payload blob (NEW)
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or error
    pub fn store_tracking_with_payload(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
        pending_reads: i64,
        expires_at: i64,
        created_at: i64,
        encrypted_payload: &[u8],
    ) -> Result<(), SqliteError> {
        let connection = get_database_connection()?;

        debug!(
            "📊 SharedSecret: Storing tracking record WITH payload (size={}, pending_reads={}, expires_at={}, created_at={})",
            encrypted_payload.len(), pending_reads, expires_at, created_at
        );

        connection.execute(
            "INSERT INTO shared_secrets_tracking (reference_hash, pending_reads, read_at, expires_at, created_at, encrypted_payload) VALUES (?, ?, NULL, ?, ?, ?)",
            &[
                Value::Blob(reference_hash.to_vec()),
                Value::Integer(pending_reads),
                Value::Integer(expires_at),
                Value::Integer(created_at),
                Value::Blob(encrypted_payload.to_vec()),
            ],
        )?;

        debug!("✅ SharedSecret: Tracking record stored WITH payload");
        Ok(())
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
        let connection = get_database_connection()?;

        let result = connection.execute(
            "SELECT encrypted_payload FROM shared_secrets_tracking WHERE reference_hash = ?",
            &[Value::Blob(reference_hash.to_vec())],
        )?;

        if let Some(row) = result.rows.first() {
            let encrypted_payload = match &row.values[0] {
                Value::Blob(data) => data.clone(),
                _ => {
                    return Err(SqliteError::Io(
                        "Invalid encrypted_payload type in tracking".to_string(),
                    ));
                }
            };

            debug!("🔍 SharedSecret: Retrieved encrypted_payload from tracking (size={})", encrypted_payload.len());
            Ok(Some(encrypted_payload))
        } else {
            warn!("⚠️  SharedSecret: Tracking payload not found");
            Ok(None)
        }
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
        let connection = get_database_connection()?;

        debug!("🗑️ SharedSecret: Deleting tracking record by reference_hash");

        connection.execute(
            "DELETE FROM shared_secrets_tracking WHERE reference_hash = ?",
            &[Value::Blob(reference_hash.to_vec())],
        )?;

        // Check if row was deleted (rowcount would be ideal but not available in Spin SDK)
        // For now, return true (assume deletion happened if no error)
        debug!("✅ SharedSecret: Tracking record deleted (or didn't exist)");
        Ok(true)
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
        let connection = get_database_connection()?;

        let now = Utc::now().timestamp();

        // Update only if read_at is NULL (idempotent)
        let _result = connection.execute(
            "UPDATE shared_secrets_tracking SET read_at = ? WHERE reference_hash = ? AND read_at IS NULL",
            &[
                Value::Integer(now),
                Value::Blob(reference_hash.to_vec()),
            ],
        )?;

        // SQLite in Spin doesn't provide rows_affected, so we check by querying
        let check = connection.execute(
            "SELECT read_at FROM shared_secrets_tracking WHERE reference_hash = ?",
            &[Value::Blob(reference_hash.to_vec())],
        )?;

        if let Some(row) = check.rows.first() {
            let read_at = match &row.values[0] {
                Value::Integer(val) => Some(*val),
                Value::Null => None,
                _ => return Err(SqliteError::Io("Invalid read_at type".to_string())),
            };

            if read_at.is_some() {
                // println!("📖 SharedSecret: Tracking updated with read_at={}", now);
                debug!("📖 SharedSecret: Tracking updated with read_at={}", now);
                Ok(true)
            } else {
                // println!("ℹ️  SharedSecret: Tracking read_at was already set");
                debug!("ℹ️  SharedSecret: Tracking read_at was already set");
                Ok(false)
            }
        } else {
            // println!("⚠️  SharedSecret: Tracking record not found");
            warn!("⚠️  SharedSecret: Tracking record not found");
            Ok(false)
        }
    }

    /// Clean up expired shared secrets and tracking records
    ///
    /// # Returns
    /// * `Result<(u32, u32), SqliteError>` - (secrets_deleted, tracking_deleted) or error
    #[allow(dead_code)]
    pub fn cleanup_expired() -> Result<(u32, u32), SqliteError> {
        let connection = get_database_connection()?;

        let now_hours = Utc::now().timestamp() / 3600;

        // ============================================================================
        // CRITICAL: Delete ORDER matters (v3)
        // ============================================================================
        // shared_secrets contains: encrypted_key_material[44]
        // tracking contains: encrypted_payload[~100-200 bytes]
        //
        // If we delete tracking first:
        //   → shared_secrets remains with key_material but no payload to decrypt
        //   → key_material becomes useless orphan
        //
        // Correct order: Delete key_material FIRST, then payload
        // ============================================================================

        // Delete expired secrets (key_material) - FIRST
        connection.execute(
            "DELETE FROM shared_secrets WHERE expires_at < ?",
            &[Value::Integer(now_hours)],
        )?;

        // Delete expired tracking records (payload) - SECOND
        connection.execute(
            "DELETE FROM shared_secrets_tracking WHERE expires_at < ?",
            &[Value::Integer(now_hours)],
        )?;

        // println!("🧹 SharedSecret: Cleaned up expired records");
        debug!("🧹 SharedSecret: Cleaned up expired records (shared_secrets first, then tracking)");
        // Spin SQLite doesn't provide rows_affected, return placeholder
        Ok((1, 1))
    }
}
