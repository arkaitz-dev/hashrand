//! Database connection management
//!
//! Provides database connection using Spin variables.
//! Database name is configured in spin configuration files.

use spin_sdk::sqlite::{Connection, Error as SqliteError};
use spin_sdk::variables;
use tracing::debug;

/// Get database connection using configured database name
///
/// # Returns
/// * `Result<Connection, SqliteError>` - Database connection or error
pub fn get_database_connection() -> Result<Connection, SqliteError> {
    let db_name = variables::get("database_name").map_err(|_| SqliteError::AccessDenied)?;
    debug!("Database: Connecting to database: '{}'", db_name);
    Connection::open(&db_name)
}

/// Initialize database tables
///
/// Creates the users, magiclinks, shared_secrets, and shared_secrets_tracking tables if they don't exist.
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
            token_hash BLOB PRIMARY KEY,    -- Blake3-var[16] of encrypted magic link token
            expires_at INTEGER NOT NULL,    -- Expiration timestamp in hours since Unix epoch (for cleanup)
            encrypted_payload BLOB NOT NULL -- Merged: encryption_blob[44] + auth_data[32] + next_param_bytes[variable]
        )
        "#,
        &[],
    )?;

    // Create shared_secrets table for secure text sharing with dual-URL system
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS shared_secrets (
            id BLOB PRIMARY KEY,              -- Encrypted ID (similar to magic_link hash)
            encrypted_payload BLOB NOT NULL,  -- ChaCha20(sender_email || receiver_email || text || otp || created_at || reference_hash || max_reads)
            expires_at INTEGER NOT NULL,      -- Expiration timestamp in hours since Unix epoch (for cleanup)
            role TEXT NOT NULL                -- 'sender' or 'receiver'
        )
        "#,
        &[],
    )?;

    // Create shared_secrets_tracking table for read confirmation tracking
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS shared_secrets_tracking (
            reference_hash BLOB PRIMARY KEY,  -- Random [u8;16] identifier (same for sender/receiver pair)
            pending_reads INTEGER NOT NULL,   -- Countdown reads counter (moved from shared_secrets)
            read_at INTEGER,                  -- Timestamp of first read by receiver (NULL if unread)
            expires_at INTEGER NOT NULL,      -- Expiration timestamp (matches shared_secrets.expires_at)
            created_at INTEGER NOT NULL,      -- Creation timestamp
            encrypted_payload BLOB NOT NULL   -- v3: Centralized encrypted payload (ChaCha20-Poly1305)
        )
        "#,
        &[],
    )?;

    Ok(())
}
