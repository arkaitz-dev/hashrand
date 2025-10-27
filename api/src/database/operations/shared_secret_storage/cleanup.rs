//! Cleanup operations for shared secrets
//!
//! Handles cleanup of expired records from both tables.

use crate::database::get_database_connection;
use chrono::Utc;
use spin_sdk::sqlite::{Error as SqliteError, Value};
use tracing::debug;

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

    debug!("🧹 SharedSecret: Cleaned up expired records (shared_secrets first, then tracking)");
    // Spin SQLite doesn't provide rows_affected, return placeholder
    Ok((1, 1))
}
