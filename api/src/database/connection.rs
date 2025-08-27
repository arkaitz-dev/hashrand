/// Database connection management with environment detection
/// 
/// Provides environment-aware database connection based on request host headers.
/// Development hosts (localhost, elite.faun-pirate.ts.net) use hashrand-dev database,
/// while production hosts use hashrand database.

use spin_sdk::sqlite::{Connection, Error as SqliteError};
use spin_sdk::http::IncomingRequest;

/// Database environment determination based on request context
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseEnvironment {
    Development,
    Production,
}

impl DatabaseEnvironment {
    /// Determine environment from HTTP request host header
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
    Connection::open(db_name)
}

/// Initialize database tables for the specified environment
/// 
/// Creates the users table if it doesn't exist.
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
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
        &[],
    )?;
    
    Ok(())
}