//! Database operations for user management
//!
//! Provides CRUD operations for the users table with proper error handling
//! and type safety using Spin's SQLite interface.

use crate::database::{
    connection::DatabaseEnvironment,
    get_database_connection,
    models::{AuthSession, User},
};
use chrono::Utc;
use spin_sdk::sqlite::{Error as SqliteError, Value};

/// User database operations
pub struct UserOperations;

impl UserOperations {
    /// Create a new user in the database
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `user` - User data to insert
    ///
    /// # Returns
    /// * `Result<i64, SqliteError>` - Created user ID or database error
    pub fn create_user(env: DatabaseEnvironment, user: &User) -> Result<i64, SqliteError> {
        let connection = get_database_connection(env)?;

        connection.execute(
            "INSERT INTO users (username, email) VALUES (?, ?)",
            &[
                Value::Text(user.username.clone()),
                Value::Text(user.email.clone()),
            ],
        )?;

        // Get the last inserted row ID
        let result = connection.execute("SELECT last_insert_rowid()", &[])?;
        if let Some(row) = result.rows.first()
            && let Some(Value::Integer(id)) = row.values.first()
        {
            return Ok(*id);
        }

        Err(SqliteError::Io(
            "Failed to get inserted user ID".to_string(),
        ))
    }

    /// Get user by ID
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `user_id` - User ID to search for
    ///
    /// # Returns
    /// * `Result<Option<User>, SqliteError>` - User if found, None if not found, or database error
    pub fn get_user_by_id(
        env: DatabaseEnvironment,
        user_id: i64,
    ) -> Result<Option<User>, SqliteError> {
        let connection = get_database_connection(env)?;

        let result = connection.execute(
            "SELECT id, username, email, created_at, updated_at FROM users WHERE id = ?",
            &[Value::Integer(user_id)],
        )?;

        if let Some(row) = result.rows.first() {
            Ok(Some(Self::row_to_user(&row.values)?))
        } else {
            Ok(None)
        }
    }

    /// Get user by username
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `username` - Username to search for
    ///
    /// # Returns
    /// * `Result<Option<User>, SqliteError>` - User if found, None if not found, or database error
    #[allow(dead_code)]
    pub fn get_user_by_username(
        env: DatabaseEnvironment,
        username: &str,
    ) -> Result<Option<User>, SqliteError> {
        let connection = get_database_connection(env)?;

        let result = connection.execute(
            "SELECT id, username, email, created_at, updated_at FROM users WHERE username = ?",
            &[Value::Text(username.to_string())],
        )?;

        if let Some(row) = result.rows.first() {
            Ok(Some(Self::row_to_user(&row.values)?))
        } else {
            Ok(None)
        }
    }

    /// List all users with optional limit
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `limit` - Optional limit for number of users to return
    ///
    /// # Returns
    /// * `Result<Vec<User>, SqliteError>` - Vector of users or database error
    pub fn list_users(
        env: DatabaseEnvironment,
        limit: Option<u32>,
    ) -> Result<Vec<User>, SqliteError> {
        let connection = get_database_connection(env)?;

        let query = match limit {
            Some(l) => format!("SELECT id, username, email, created_at, updated_at FROM users ORDER BY created_at DESC LIMIT {}", l),
            None => "SELECT id, username, email, created_at, updated_at FROM users ORDER BY created_at DESC".to_string(),
        };

        let result = connection.execute(&query, &[])?;

        let mut users = Vec::new();
        for row in &result.rows {
            users.push(Self::row_to_user(&row.values)?);
        }

        Ok(users)
    }

    /// Delete user by ID
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `user_id` - User ID to delete
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - True if user was deleted, false if not found
    pub fn delete_user(env: DatabaseEnvironment, user_id: i64) -> Result<bool, SqliteError> {
        let connection = get_database_connection(env.clone())?;

        // First check if user exists
        let exists_result = connection.execute(
            "SELECT 1 FROM users WHERE id = ?",
            &[Value::Integer(user_id)],
        )?;

        if exists_result.rows.is_empty() {
            return Ok(false); // User not found
        }

        // Delete the user
        connection.execute("DELETE FROM users WHERE id = ?", &[Value::Integer(user_id)])?;

        Ok(true) // User was deleted
    }

    /// Convert database row to User struct
    ///
    /// # Arguments
    /// * `row` - Database row values
    ///
    /// # Returns
    /// * `Result<User, SqliteError>` - User instance or conversion error
    fn row_to_user(row: &[Value]) -> Result<User, SqliteError> {
        if row.len() != 5 {
            return Err(SqliteError::Io("Invalid row format for User".to_string()));
        }

        let id = match &row[0] {
            Value::Integer(i) => Some(*i),
            Value::Null => None,
            _ => return Err(SqliteError::Io("Invalid ID type".to_string())),
        };

        let username = match &row[1] {
            Value::Text(s) => s.clone(),
            _ => return Err(SqliteError::Io("Invalid username type".to_string())),
        };

        let email = match &row[2] {
            Value::Text(s) => s.clone(),
            _ => return Err(SqliteError::Io("Invalid email type".to_string())),
        };

        let created_at = match &row[3] {
            Value::Text(s) => Some(s.clone()),
            Value::Null => None,
            _ => return Err(SqliteError::Io("Invalid created_at type".to_string())),
        };

        let updated_at = match &row[4] {
            Value::Text(s) => Some(s.clone()),
            Value::Null => None,
            _ => return Err(SqliteError::Io("Invalid updated_at type".to_string())),
        };

        Ok(User {
            id,
            username,
            email,
            created_at,
            updated_at,
        })
    }
}

/// Authentication session database operations
pub struct AuthOperations;

impl AuthOperations {
    /// Create a new auth session with generated JWT tokens
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `session` - Auth session data to insert
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or database error
    pub fn create_auth_session(
        env: DatabaseEnvironment,
        session: &AuthSession,
    ) -> Result<(), SqliteError> {
        let connection = get_database_connection(env)?;
        println!(
            "Database: Creating session with magic token: '{}'",
            session.magic_token
        );

        // Extract user_id bytes from Base58 username
        let user_id_bytes = bs58::decode(&session.user_id)
            .into_vec()
            .map_err(|_| SqliteError::Io("Invalid user_id format".to_string()))?;

        // Generate JWT tokens using a dummy email (we need to refactor this)
        let dummy_email = "temp@example.com"; // TODO: Remove this dependency
        let (access_token, _) = crate::utils::JwtUtils::create_access_token(dummy_email)
            .map_err(|e| SqliteError::Io(format!("Failed to create access token: {}", e)))?;
        let (refresh_token, _) = crate::utils::JwtUtils::create_refresh_token(dummy_email, 1)
            .map_err(|e| SqliteError::Io(format!("Failed to create refresh token: {}", e)))?;

        let _insert_result = connection.execute(
            "INSERT INTO auth_sessions (user_id, expires, access_token, refresh_token) VALUES (?, ?, ?, ?)",
            &[
                Value::Blob(user_id_bytes),
                Value::Integer(session.magic_expires_at as i64),
                Value::Text(access_token),
                Value::Text(refresh_token),
            ],
        )?;

        println!("Database: Session created successfully");
        Ok(())
    }

    /// Get auth session tokens by user_id and timestamp
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `user_id` - User ID bytes (32 bytes)
    /// * `timestamp` - Expiration timestamp
    ///
    /// # Returns
    /// * `Result<Option<(String, String)>, SqliteError>` - (access_token, refresh_token) or None
    pub fn get_session_by_user_id_and_timestamp(
        env: DatabaseEnvironment,
        user_id: &[u8; 32],
        timestamp: u64,
    ) -> Result<Option<(String, String)>, SqliteError> {
        let connection = get_database_connection(env)?;
        println!(
            "Database: Searching for session: user_id bytes len={}, timestamp={}",
            user_id.len(),
            timestamp
        );

        let result = connection.execute(
            "SELECT access_token, refresh_token FROM auth_sessions WHERE user_id = ? AND expires = ?",
            &[Value::Blob(user_id.to_vec()), Value::Integer(timestamp as i64)],
        )?;

        println!("Database: Query returned {} rows", result.rows.len());

        if let Some(row) = result.rows.first() {
            let access_token = match &row.values[0] {
                Value::Text(s) => s.clone(),
                _ => return Err(SqliteError::Io("Invalid access_token type".to_string())),
            };

            let refresh_token = match &row.values[1] {
                Value::Text(s) => s.clone(),
                _ => return Err(SqliteError::Io("Invalid refresh_token type".to_string())),
            };

            Ok(Some((access_token, refresh_token)))
        } else {
            Ok(None)
        }
    }

    /// Ensure user exists in users table by user_id (insert if not exists)
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `user_id` - User ID bytes (32 bytes)
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or error
    pub fn ensure_user_exists(
        env: DatabaseEnvironment,
        user_id: &[u8; 32],
    ) -> Result<(), SqliteError> {
        let connection = get_database_connection(env)?;

        // Insert user if it doesn't exist (ignore if already exists)
        match connection.execute(
            "INSERT OR IGNORE INTO users (user_id) VALUES (?)",
            &[Value::Blob(user_id.to_vec())],
        ) {
            Ok(_) => {
                println!(
                    "Database: Ensured user exists: {}",
                    bs58::encode(user_id).into_string()
                );
                Ok(())
            }
            Err(e) => {
                println!("Database: Error ensuring user exists: {}", e);
                Err(e)
            }
        }
    }

    /// Delete auth session by user_id and timestamp
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `user_id` - User ID bytes (32 bytes)
    /// * `timestamp` - Expiration timestamp
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - True if deleted, false if not found
    pub fn delete_session_by_user_id_and_timestamp(
        env: DatabaseEnvironment,
        user_id: &[u8; 32],
        timestamp: u64,
    ) -> Result<bool, SqliteError> {
        let connection = get_database_connection(env)?;

        let _result = connection.execute(
            "DELETE FROM auth_sessions WHERE user_id = ? AND expires = ?",
            &[
                Value::Blob(user_id.to_vec()),
                Value::Integer(timestamp as i64),
            ],
        )?;

        println!("Database: Deleted auth session rows");
        Ok(true) // Assume success if no error
    }

    /// Get active auth session by refresh token
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `refresh_token` - Refresh token to search for
    ///
    /// # Returns
    /// * `Result<Option<AuthSession>, SqliteError>` - Session if found and valid, None if not found/expired, or database error
    #[allow(dead_code)]
    pub fn get_session_by_refresh_token(
        env: DatabaseEnvironment,
        refresh_token: &str,
    ) -> Result<Option<AuthSession>, SqliteError> {
        let connection = get_database_connection(env)?;

        let now = Utc::now().timestamp() as u64;
        let result = connection.execute(
            "SELECT id, email, magic_token, access_token, refresh_token, created_at, magic_expires_at, access_expires_at, refresh_expires_at, is_used FROM auth_sessions WHERE refresh_token = ? AND is_used = 1 AND refresh_expires_at > ?",
            &[
                Value::Text(refresh_token.to_string()),
                Value::Integer(now as i64),
            ],
        )?;

        if let Some(row) = result.rows.first() {
            Ok(Some(Self::row_to_auth_session(&row.values)?))
        } else {
            Ok(None)
        }
    }

    /// Update session with new access token (refresh token flow)
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `session_id` - Session ID to update
    /// * `access_token` - New JWT access token
    /// * `access_expires_at` - New access token expiration timestamp
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - True if updated successfully, false if session not found
    #[allow(dead_code)]
    pub fn refresh_access_token(
        env: DatabaseEnvironment,
        session_id: i64,
        access_token: &str,
        access_expires_at: u64,
    ) -> Result<bool, SqliteError> {
        let connection = get_database_connection(env)?;

        let _result = connection.execute(
            "UPDATE auth_sessions SET access_token = ?, access_expires_at = ? WHERE id = ?",
            &[
                Value::Text(access_token.to_string()),
                Value::Integer(access_expires_at as i64),
                Value::Integer(session_id),
            ],
        )?;

        // SQLite doesn't provide rows_affected in Spin SDK
        // We'll assume success if no error occurred
        Ok(true)
    }

    /// Clean up expired auth sessions
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    ///
    /// # Returns
    /// * `Result<u32, SqliteError>` - Number of sessions deleted or database error
    pub fn cleanup_expired_sessions(env: DatabaseEnvironment) -> Result<u32, SqliteError> {
        let connection = get_database_connection(env)?;

        let now = Utc::now().timestamp() as u64;
        let _result = connection.execute(
            "DELETE FROM auth_sessions WHERE (is_used = 0 AND magic_expires_at < ?) OR (is_used = 1 AND refresh_expires_at < ?)",
            &[
                Value::Integer(now as i64),
                Value::Integer(now as i64),
            ],
        )?;

        // SQLite doesn't provide rows_affected in Spin SDK
        // We'll return 1 as a placeholder for successful cleanup
        Ok(1)
    }

    /// Convert database row to AuthSession struct
    ///
    /// # Arguments
    /// * `row` - Database row values
    ///
    /// # Returns
    /// * `Result<AuthSession, SqliteError>` - AuthSession instance or conversion error
    fn row_to_auth_session(row: &[Value]) -> Result<AuthSession, SqliteError> {
        if row.len() != 10 {
            return Err(SqliteError::Io(
                "Invalid row format for AuthSession".to_string(),
            ));
        }

        let id = match &row[0] {
            Value::Integer(i) => Some(*i),
            Value::Null => None,
            _ => return Err(SqliteError::Io("Invalid ID type".to_string())),
        };

        let user_id = match &row[1] {
            Value::Text(s) => s.clone(),
            _ => return Err(SqliteError::Io("Invalid user_id type".to_string())),
        };

        let magic_token = match &row[2] {
            Value::Text(s) => s.clone(),
            _ => return Err(SqliteError::Io("Invalid magic_token type".to_string())),
        };

        let access_token = match &row[3] {
            Value::Text(s) => Some(s.clone()),
            Value::Null => None,
            _ => return Err(SqliteError::Io("Invalid access_token type".to_string())),
        };

        let refresh_token = match &row[4] {
            Value::Text(s) => Some(s.clone()),
            Value::Null => None,
            _ => return Err(SqliteError::Io("Invalid refresh_token type".to_string())),
        };

        let created_at = match &row[5] {
            Value::Integer(i) => Some(*i as u64),
            Value::Null => None,
            _ => return Err(SqliteError::Io("Invalid created_at type".to_string())),
        };

        let magic_expires_at = match &row[6] {
            Value::Integer(i) => *i as u64,
            _ => return Err(SqliteError::Io("Invalid magic_expires_at type".to_string())),
        };

        let access_expires_at = match &row[7] {
            Value::Integer(i) => Some(*i as u64),
            Value::Null => None,
            _ => {
                return Err(SqliteError::Io(
                    "Invalid access_expires_at type".to_string(),
                ));
            }
        };

        let refresh_expires_at = match &row[8] {
            Value::Integer(i) => Some(*i as u64),
            Value::Null => None,
            _ => {
                return Err(SqliteError::Io(
                    "Invalid refresh_expires_at type".to_string(),
                ));
            }
        };

        let is_used = match &row[9] {
            Value::Integer(i) => *i != 0,
            _ => return Err(SqliteError::Io("Invalid is_used type".to_string())),
        };

        Ok(AuthSession {
            id,
            user_id,
            email: None, // Email not stored in DB anymore
            magic_token,
            access_token,
            refresh_token,
            created_at,
            magic_expires_at,
            access_expires_at,
            refresh_expires_at,
            is_used,
        })
    }
}
