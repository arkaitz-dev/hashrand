///! Deletion operations for shared secrets
///!
///! Handles deleting shared secret entries from the database.

use super::super::shared_secret_types::constants::*;
use super::retrieval::{retrieve_secret, retrieve_secret_old};
use crate::database::get_database_connection;
use spin_sdk::sqlite::{Error as SqliteError, Value};
use tracing::debug;

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
    if retrieve_secret(db_index)?.is_none() {
        return Ok(false);
    }

    connection.execute(
        "DELETE FROM shared_secrets WHERE id = ?",
        &[Value::Blob(db_index.to_vec())],
    )?;

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
    if retrieve_secret_old(id)?.is_none() {
        return Ok(false);
    }

    connection.execute(
        "DELETE FROM shared_secrets WHERE id = ?",
        &[Value::Blob(id.to_vec())],
    )?;

    debug!("🗑️  SharedSecret: Deleted successfully");
    Ok(true)
}
