//! Database connection management with environment detection
//!
//! Provides environment-aware database connection based on request host headers.
//! Development hosts (localhost, elite.faun-pirate.ts.net) use hashrand-dev database,
//! while production hosts use hashrand database.

use spin_sdk::http::IncomingRequest;
use spin_sdk::sqlite::{Connection, Error as SqliteError};

/// Database environment determination based on request context
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum DatabaseEnvironment {
    Development,
    Production,
}

impl DatabaseEnvironment {
    /// Determine environment from HTTP request host header
    #[allow(unused_variables, dead_code)]
    pub fn from_request(req: &IncomingRequest) -> Self {
        // For now, always return Development
        // TODO: Implement proper host header detection
        DatabaseEnvironment::Development
    }

    /// Get database name for the environment
    pub fn database_name(&self) -> &'static str {
        match self {
            DatabaseEnvironment::Development => "hashrand-dev",
            DatabaseEnvironment::Production => "hashrand",
        }
    }
}

/// Get database connection based on environment
///
/// # Arguments
/// * `env` - Database environment (Development or Production)
///
/// # Returns
/// * `Result<Connection, SqliteError>` - Database connection or error
pub fn get_database_connection(env: DatabaseEnvironment) -> Result<Connection, SqliteError> {
    let db_name = env.database_name();
    println!("Database: Connecting to database: '{}'", db_name);
    Connection::open(db_name)
}

/// Initialize database tables for the specified environment
///
/// Creates the users and magiclinks tables if they don't exist.
///
/// # Arguments
/// * `env` - Database environment to initialize
///
/// # Returns
/// * `Result<(), SqliteError>` - Success or database error
pub fn initialize_database(env: DatabaseEnvironment) -> Result<(), SqliteError> {
    let connection = get_database_connection(env)?;

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
            token_hash BLOB PRIMARY KEY,    -- SHAKE-256[16] of encrypted new_raw_magic_link
            expires_at INTEGER NOT NULL,    -- Expiration timestamp in hours since Unix epoch (for cleanup)
            encrypted_payload BLOB NOT NULL -- Merged: encryption_blob[44] + next_param_bytes[variable]
        )
        "#,
        &[],
    )?;

    Ok(())
}
