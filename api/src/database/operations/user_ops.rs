//! User database operations
//!
//! Provides CRUD operations for the users table with proper error handling
//! and type safety using Spin's SQLite interface.

use crate::database::{connection::DatabaseEnvironment, get_database_connection, models::User};
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
