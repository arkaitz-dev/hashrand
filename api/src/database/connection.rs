//! Database connection management
//!
//! Provides database connection using Spin variables.
//! Database name is configured in spin configuration files.

use spin_sdk::sqlite::{Connection, Error as SqliteError};
use spin_sdk::variables;

/// Get database connection using configured database name
///
/// # Returns
/// * `Result<Connection, SqliteError>` - Database connection or error
pub fn get_database_connection() -> Result<Connection, SqliteError> {
    let db_name = variables::get("database_name").map_err(|_| SqliteError::AccessDenied)?;
    println!("Database: Connecting to database: '{}'", db_name);
    Connection::open(&db_name)
}

/// Initialize database tables
///
/// Creates the users and magiclinks tables if they don't exist.
///
/// # Returns
/// * `Result<(), SqliteError>` - Success or database error
pub fn initialize_database() -> Result<(), SqliteError> {
    let connection = get_database_connection()?;

    // Create users table if it doesn't exist
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            user_id BLOB PRIMARY KEY,
            created_at INTEGER DEFAULT (unixepoch())
        )
        "#,
        &[],
    )?;

    // Create magiclinks table for magic link validation with ChaCha20-Poly1305 encryption
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS magiclinks (
            token_hash BLOB PRIMARY KEY,    -- Blake2b-var[16] of encrypted magic link token
            expires_at INTEGER NOT NULL,    -- Expiration timestamp in hours since Unix epoch (for cleanup)
            encrypted_payload BLOB NOT NULL -- Merged: encryption_blob[44] + auth_data[32] + next_param_bytes[variable]
        )
        "#,
        &[],
    )?;

    Ok(())
}
