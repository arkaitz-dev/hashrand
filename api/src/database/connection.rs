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
/// Creates all application tables: users, magiclinks, shared_secrets, shared_secrets_tracking,
/// user_privkey_context, user_ed25519_keys, user_x25519_keys
///
/// # Returns
/// * `Result<(), SqliteError>` - Success or database error
pub fn initialize_database() -> Result<(), SqliteError> {
    let connection = get_database_connection()?;

    // Create users table for user tracking
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            user_id BLOB PRIMARY KEY,
            logged_in INTEGER,                  -- Unix timestamp (nullable)
            created_at INTEGER DEFAULT (unixepoch())
        )
        "#,
        &[],
    )?;

    // Create index on logged_in for efficient queries
    connection.execute(
        "CREATE INDEX IF NOT EXISTS idx_users_logged_in ON users(logged_in)",
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
            encrypted_payload BLOB NOT NULL   -- v3: Centralized encrypted payload (ChaCha20-Poly1305)
        )
        "#,
        &[],
    )?;

    // Create user_privkey_context table for user private key derivation context
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS user_privkey_context (
            db_index BLOB PRIMARY KEY,        -- 16 bytes: blake3_keyed_variable(INDEX_KEY, argon2_output[32], 16)
            encrypted_privkey BLOB NOT NULL,  -- ChaCha20-Poly1305 encrypted 64 random bytes
            created_year INTEGER NOT NULL     -- 4 digits (2025, 2026, etc.)
        )
        "#,
        &[],
    )?;

    // Create user_ed25519_keys table for permanent Ed25519 public keys (System B - E2EE)
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS user_ed25519_keys (
            user_id BLOB NOT NULL,
            pub_key TEXT NOT NULL,            -- Hex string (64 chars)
            created_at INTEGER NOT NULL,      -- Unix timestamp
            UNIQUE(user_id, pub_key),
            FOREIGN KEY(user_id) REFERENCES users(user_id)
        )
        "#,
        &[],
    )?;

    // Create index for efficient queries by user_id and created_at
    connection.execute(
        "CREATE INDEX IF NOT EXISTS idx_ed25519_user_created ON user_ed25519_keys(user_id, created_at DESC)",
        &[],
    )?;

    // Create user_x25519_keys table for permanent X25519 public keys (System B - E2EE)
    connection.execute(
        r#"
        CREATE TABLE IF NOT EXISTS user_x25519_keys (
            user_id BLOB NOT NULL,
            pub_key TEXT NOT NULL,            -- Hex string (64 chars)
            created_at INTEGER NOT NULL,      -- Unix timestamp
            UNIQUE(user_id, pub_key),
            FOREIGN KEY(user_id) REFERENCES users(user_id)
        )
        "#,
        &[],
    )?;

    // Create index for efficient queries by user_id and created_at
    connection.execute(
        "CREATE INDEX IF NOT EXISTS idx_x25519_user_created ON user_x25519_keys(user_id, created_at DESC)",
        &[],
    )?;

    Ok(())
}
