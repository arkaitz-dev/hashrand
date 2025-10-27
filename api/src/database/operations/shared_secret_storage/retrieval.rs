//! Retrieval operations for shared secrets
//!
//! Handles retrieving shared secret entries from the database.

use super::super::shared_secret_types::{SecretRole, constants::*};
use crate::database::get_database_connection;
use spin_sdk::sqlite::{Error as SqliteError, Value};
use tracing::{debug, warn};

/// Type alias for secret retrieval result tuple: (encrypted_payload, expires_at, role)
pub type SecretData = (Vec<u8>, i64, SecretRole);

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
        warn!("⚠️  SharedSecret: Not found in database");
        Ok(None)
    }
}
