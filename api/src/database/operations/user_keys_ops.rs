//! User public keys database operations (Sistema B - E2EE)
//!
//! Manages permanent Ed25519/X25519 public keys for user-to-user E2EE
//! Separate from temporary session keys (Sistema A)

use crate::database::get_database_connection;
use spin_sdk::sqlite::{Error as SqliteError, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

/// User public keys operations for E2EE (Sistema B)
pub struct UserKeysOperations;

impl UserKeysOperations {
    /// Insert or update user entry with current login timestamp
    ///
    /// Updates logged_in timestamp if user exists, creates new entry if not
    ///
    /// # Arguments
    /// * `user_id` - 16-byte user identifier
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or error
    pub fn insert_or_update_user(user_id: &[u8; 16]) -> Result<(), SqliteError> {
        debug!("Database: insert_or_update_user called for user_id={}", hex::encode(user_id));
        let connection = get_database_connection()?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| SqliteError::Io(format!("Time error: {}", e)))?
            .as_secs() as i64;

        debug!("Database: Executing INSERT OR REPLACE for users table (timestamp={})", now);
        // INSERT OR REPLACE updates logged_in if user exists, creates if not
        connection.execute(
            "INSERT OR REPLACE INTO users (user_id, logged_in, created_at) VALUES (?, ?, COALESCE((SELECT created_at FROM users WHERE user_id = ?), ?))",
            &[
                Value::Blob(user_id.to_vec()),
                Value::Integer(now),
                Value::Blob(user_id.to_vec()),
                Value::Integer(now),
            ],
        )?;

        debug!("Database: ✅ User entry updated successfully (logged_in={})", now);
        Ok(())
    }

    /// Insert Ed25519 public key (idempotent - ignores duplicates)
    ///
    /// # Arguments
    /// * `user_id` - 16-byte user identifier
    /// * `pub_key_hex` - Ed25519 public key as hex string (64 chars)
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or error
    pub fn insert_ed25519_key(user_id: &[u8; 16], pub_key_hex: &str) -> Result<(), SqliteError> {
        debug!("Database: insert_ed25519_key called (user_id={}, pub_key={}...)",
               hex::encode(user_id), &pub_key_hex[..16]);

        if pub_key_hex.len() != 64 {
            return Err(SqliteError::Io(format!(
                "Invalid Ed25519 public key length: {} (expected 64 hex chars)",
                pub_key_hex.len()
            )));
        }

        let connection = get_database_connection()?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| SqliteError::Io(format!("Time error: {}", e)))?
            .as_secs() as i64;

        debug!("Database: Executing INSERT OR IGNORE for user_ed25519_keys table");
        // INSERT OR IGNORE: If (user_id, pub_key) already exists, do nothing (idempotent)
        connection.execute(
            "INSERT OR IGNORE INTO user_ed25519_keys (user_id, pub_key, created_at) VALUES (?, ?, ?)",
            &[
                Value::Blob(user_id.to_vec()),
                Value::Text(pub_key_hex.to_string()),
                Value::Integer(now),
            ],
        )?;

        debug!("Database: ✅ Ed25519 public key inserted/updated successfully");
        Ok(())
    }

    /// Insert X25519 public key (idempotent - ignores duplicates)
    ///
    /// # Arguments
    /// * `user_id` - 16-byte user identifier
    /// * `pub_key_hex` - X25519 public key as hex string (64 chars)
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or error
    pub fn insert_x25519_key(user_id: &[u8; 16], pub_key_hex: &str) -> Result<(), SqliteError> {
        debug!("Database: insert_x25519_key called (user_id={}, pub_key={}...)",
               hex::encode(user_id), &pub_key_hex[..16]);

        if pub_key_hex.len() != 64 {
            return Err(SqliteError::Io(format!(
                "Invalid X25519 public key length: {} (expected 64 hex chars)",
                pub_key_hex.len()
            )));
        }

        let connection = get_database_connection()?;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| SqliteError::Io(format!("Time error: {}", e)))?
            .as_secs() as i64;

        debug!("Database: Executing INSERT OR IGNORE for user_x25519_keys table");
        // INSERT OR IGNORE: If (user_id, pub_key) already exists, do nothing (idempotent)
        connection.execute(
            "INSERT OR IGNORE INTO user_x25519_keys (user_id, pub_key, created_at) VALUES (?, ?, ?)",
            &[
                Value::Blob(user_id.to_vec()),
                Value::Text(pub_key_hex.to_string()),
                Value::Integer(now),
            ],
        )?;

        debug!("Database: ✅ X25519 public key inserted/updated successfully");
        Ok(())
    }

    /// Get latest N public keys for a user (both Ed25519 and X25519)
    ///
    /// Returns most recent keys sorted by created_at DESC
    ///
    /// # Arguments
    /// * `user_id` - 16-byte user identifier
    /// * `limit` - Maximum number of keys to return per type (default: 5)
    ///
    /// # Returns
    /// * `Result<(Vec<UserPublicKey>, Vec<UserPublicKey>), SqliteError>` - (Ed25519 keys, X25519 keys)
    pub fn get_user_keys(
        user_id: &[u8; 16],
        limit: usize,
    ) -> Result<(Vec<UserPublicKey>, Vec<UserPublicKey>), SqliteError> {
        debug!("Database: get_user_keys called (user_id={}, limit={})",
               hex::encode(user_id), limit);

        let connection = get_database_connection()?;

        debug!("Database: Querying Ed25519 keys from user_ed25519_keys table");
        // Query Ed25519 keys (latest N)
        let ed25519_result = connection.execute(
            "SELECT pub_key, created_at FROM user_ed25519_keys WHERE user_id = ? ORDER BY created_at DESC LIMIT ?",
            &[
                Value::Blob(user_id.to_vec()),
                Value::Integer(limit as i64),
            ],
        )?;

        let ed25519_keys: Vec<UserPublicKey> = ed25519_result
            .rows
            .iter()
            .filter_map(|row| {
                if row.values.len() >= 2 {
                    if let (Value::Text(pub_key), Value::Integer(created_at)) =
                        (&row.values[0], &row.values[1])
                    {
                        return Some(UserPublicKey {
                            pub_key: pub_key.clone(),
                            created_at: *created_at,
                        });
                    }
                }
                None
            })
            .collect();

        debug!("Database: Found {} Ed25519 keys", ed25519_keys.len());

        debug!("Database: Querying X25519 keys from user_x25519_keys table");
        // Query X25519 keys (latest N)
        let x25519_result = connection.execute(
            "SELECT pub_key, created_at FROM user_x25519_keys WHERE user_id = ? ORDER BY created_at DESC LIMIT ?",
            &[
                Value::Blob(user_id.to_vec()),
                Value::Integer(limit as i64),
            ],
        )?;

        let x25519_keys: Vec<UserPublicKey> = x25519_result
            .rows
            .iter()
            .filter_map(|row| {
                if row.values.len() >= 2 {
                    if let (Value::Text(pub_key), Value::Integer(created_at)) =
                        (&row.values[0], &row.values[1])
                    {
                        return Some(UserPublicKey {
                            pub_key: pub_key.clone(),
                            created_at: *created_at,
                        });
                    }
                }
                None
            })
            .collect();

        debug!("Database: Found {} X25519 keys", x25519_keys.len());

        debug!(
            "Database: ✅ Retrieved total: {} Ed25519 keys and {} X25519 keys",
            ed25519_keys.len(),
            x25519_keys.len()
        );

        Ok((ed25519_keys, x25519_keys))
    }
}

/// User public key with timestamp
#[derive(Debug, Clone)]
pub struct UserPublicKey {
    pub pub_key: String,
    pub created_at: i64,
}
