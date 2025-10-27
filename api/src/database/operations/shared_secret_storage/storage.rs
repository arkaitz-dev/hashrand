//! Storage operations for shared secrets
//!
//! Handles storing shared secret entries in the database.

use super::super::shared_secret_types::{SecretRole, constants::*};
use crate::database::get_database_connection;
use spin_sdk::sqlite::{Error as SqliteError, Value};
use tracing::debug;

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

    debug!("✅ SharedSecret: Stored successfully");
    Ok(())
}
