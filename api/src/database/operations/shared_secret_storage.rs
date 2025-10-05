//! Shared secret storage operations
//!
//! Provides database storage functions for shared secrets including
//! storage, retrieval, deletion, tracking, and cleanup operations.

use super::shared_secret_types::{SecretRole, constants::*};
use crate::database::get_database_connection;
use chrono::Utc;
use spin_sdk::sqlite::{Error as SqliteError, Value};

/// Type alias for secret retrieval result tuple: (encrypted_payload, expires_at, role)
type SecretData = (Vec<u8>, i64, SecretRole);

/// Shared secret storage operations
pub struct SharedSecretStorage;

impl SharedSecretStorage {
    /// Store a shared secret entry in the database
    ///
    /// # Arguments
    /// * `id` - Encrypted ID (32 bytes)
    /// * `encrypted_payload` - Encrypted payload blob
    /// * `expires_at` - Expiration timestamp in hours since Unix epoch
    /// * `role` - 'sender' or 'receiver'
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or database error
    pub fn store_shared_secret(
        id: &[u8; ENCRYPTED_ID_LENGTH],
        encrypted_payload: &[u8],
        expires_at: i64,
        role: SecretRole,
    ) -> Result<(), SqliteError> {
        let connection = get_database_connection()?;

        println!(
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

        println!("✅ SharedSecret: Stored successfully");
        Ok(())
    }

    /// Retrieve a shared secret by encrypted ID
    ///
    /// # Arguments
    /// * `id` - Encrypted ID (32 bytes)
    ///
    /// # Returns
    /// * `Result<Option<SecretData>, SqliteError>` - (encrypted_payload, expires_at, role) or None
    pub fn retrieve_secret(
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

            println!(
                "🔍 SharedSecret: Retrieved (role={}, expires_at={})",
                role.to_str(),
                expires_at
            );

            Ok(Some((encrypted_payload, expires_at, role)))
        } else {
            println!("⚠️  SharedSecret: Not found in database");
            Ok(None)
        }
    }

    /// Delete a shared secret by encrypted ID
    ///
    /// # Arguments
    /// * `id` - Encrypted ID (32 bytes)
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if deleted, false if not found
    pub fn delete_secret(id: &[u8; ENCRYPTED_ID_LENGTH]) -> Result<bool, SqliteError> {
        let connection = get_database_connection()?;

        // Check if exists first
        if Self::retrieve_secret(id)?.is_none() {
            return Ok(false);
        }

        connection.execute(
            "DELETE FROM shared_secrets WHERE id = ?",
            &[Value::Blob(id.to_vec())],
        )?;

        println!("🗑️  SharedSecret: Deleted successfully");
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

    /// Decrement pending_reads in tracking table
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<i64, SqliteError>` - New pending_reads value
    pub fn decrement_tracking_reads(
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<i64, SqliteError> {
        let connection = get_database_connection()?;

        // Get current pending_reads
        let pending_reads = Self::get_pending_reads_from_tracking(reference_hash)?
            .ok_or_else(|| SqliteError::Io("Tracking record not found".to_string()))?;

        // Don't decrement if sender (unlimited reads = -1)
        if pending_reads == UNLIMITED_READS {
            println!("📖 SharedSecret: Sender has unlimited reads, not decrementing");
            return Ok(UNLIMITED_READS);
        }

        // Don't decrement if already at 0 or negative
        if pending_reads <= 0 {
            println!("⚠️  SharedSecret: Already at 0 reads");
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

        println!("📖 SharedSecret: Decremented to {} reads", new_reads);
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

        println!(
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

        println!("✅ SharedSecret: Tracking record stored");
        Ok(())
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
                println!("📖 SharedSecret: Tracking updated with read_at={}", now);
                Ok(true)
            } else {
                println!("ℹ️  SharedSecret: Tracking read_at was already set");
                Ok(false)
            }
        } else {
            println!("⚠️  SharedSecret: Tracking record not found");
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

        // Delete expired secrets
        connection.execute(
            "DELETE FROM shared_secrets WHERE expires_at < ?",
            &[Value::Integer(now_hours)],
        )?;

        // Delete expired tracking records
        connection.execute(
            "DELETE FROM shared_secrets_tracking WHERE expires_at < ?",
            &[Value::Integer(now_hours)],
        )?;

        println!("🧹 SharedSecret: Cleaned up expired records");
        // Spin SQLite doesn't provide rows_affected, return placeholder
        Ok((1, 1))
    }
}
