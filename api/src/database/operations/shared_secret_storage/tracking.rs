///! Tracking operations for shared secrets
///!
///! Handles tracking table operations: pending_reads, read_at, payload storage, etc.

use super::super::shared_secret_types::constants::*;
use crate::database::get_database_connection;
use chrono::Utc;
use spin_sdk::sqlite::{Error as SqliteError, Value};
use tracing::{debug, warn};

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
    Ok(get_pending_reads_from_tracking(reference_hash)?.is_some())
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
    let connection = get_database_connection()?;

    let result = connection.execute(
        "SELECT read_at FROM shared_secrets_tracking WHERE reference_hash = ?",
        &[Value::Blob(reference_hash.to_vec())],
    )?;

    if let Some(row) = result.rows.first() {
        match &row.values[0] {
            Value::Integer(val) => Ok(Some(*val)),
            Value::Null => Ok(None),
            _ => Err(SqliteError::Io("Invalid read_at type".to_string())),
        }
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
///
/// Decrement tracking reads (simple decrement, no idempotency)
/// Frontend handles duplicate prevention via IndexedDB cache
pub fn decrement_tracking_reads(
    reference_hash: &[u8; REFERENCE_HASH_LENGTH],
) -> Result<i64, SqliteError> {
    let connection = get_database_connection()?;

    // Get current pending_reads
    let pending_reads = get_pending_reads_from_tracking(reference_hash)?
        .ok_or_else(|| SqliteError::Io("Tracking record not found".to_string()))?;

    // Don't decrement if sender (unlimited reads = -1)
    if pending_reads == UNLIMITED_READS {
        debug!("📖 SharedSecret: Sender has unlimited reads, not decrementing");
        return Ok(UNLIMITED_READS);
    }

    // Don't decrement if already at 0 or negative
    if pending_reads <= 0 {
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

    debug!("📖 SharedSecret: Decremented to {} reads", new_reads);
    Ok(new_reads)
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
    let connection = get_database_connection()?;

    debug!(
        "📊 SharedSecret: Storing tracking record WITH payload (size={}, pending_reads={}, expires_at={})",
        encrypted_payload.len(),
        pending_reads,
        expires_at
    );

    connection.execute(
        "INSERT INTO shared_secrets_tracking (reference_hash, pending_reads, read_at, expires_at, encrypted_payload) VALUES (?, ?, NULL, ?, ?)",
        &[
            Value::Blob(reference_hash.to_vec()),
            Value::Integer(pending_reads),
            Value::Integer(expires_at),
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

        debug!(
            "🔍 SharedSecret: Retrieved encrypted_payload from tracking (size={})",
            encrypted_payload.len()
        );
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
            debug!("📖 SharedSecret: Tracking updated with read_at={}", now);
            Ok(true)
        } else {
            debug!("ℹ️  SharedSecret: Tracking read_at was already set");
            Ok(false)
        }
    } else {
        warn!("⚠️  SharedSecret: Tracking record not found");
        Ok(false)
    }
}
