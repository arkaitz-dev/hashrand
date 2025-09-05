//! Database operations for user management
//!
//! Provides CRUD operations for the users table with proper error handling
//! and type safety using Spin's SQLite interface.

use crate::database::{
    connection::DatabaseEnvironment,
    get_database_connection,
    models::User,
};
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha3::{Sha3_256, Shake256, digest::{Update, ExtendableOutput, XofReader}};
use bs58;

type HmacSha3_256 = Hmac<Sha3_256>;
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

/// Magic link database operations
pub struct MagicLinkOperations;

impl MagicLinkOperations {
    /// Create hash of raw magic link using HMAC-SHA3-256 + SHAKE-256 compression
    ///
    /// # Arguments
    /// * `raw_magic_link` - The raw magic link bytes (32 bytes)
    /// * `hmac_key` - HMAC secret key
    ///
    /// # Returns
    /// * `[u8; 16]` - 16-byte hash
    fn create_token_hash(raw_magic_link: &[u8], hmac_key: &[u8]) -> [u8; 16] {
        // Step 1: HMAC-SHA3-256(raw_magic_link, MAGIC_LINK_HMAC_KEY)
        let mut mac = HmacSha3_256::new_from_slice(hmac_key)
            .expect("HMAC can take key of any size");
        Mac::update(&mut mac, raw_magic_link);
        let hmac_result = mac.finalize().into_bytes();
        
        // Step 2: SHAKE-256(hmac_result) → [16 bytes]
        let mut hasher = Shake256::default();
        hasher.update(&hmac_result);
        
        let mut reader = hasher.finalize_xof();
        let mut hash = [0u8; 16];
        reader.read(&mut hash);
        hash
    }

    /// Create SHAKE-256 hash of encrypted magic token for database storage
    ///
    /// # Arguments
    /// * `encrypted_data` - The encrypted magic token bytes (48 bytes: 32 + 16 auth tag)
    ///
    /// # Returns
    /// * `[u8; 16]` - 16-byte SHAKE-256 hash for database indexing
    fn create_encrypted_token_hash(encrypted_data: &[u8]) -> [u8; 16] {
        // SHAKE-256(encrypted_data) → [16 bytes]
        let mut hasher = Shake256::default();
        hasher.update(encrypted_data);
        
        let mut reader = hasher.finalize_xof();
        let mut hash = [0u8; 16];
        reader.read(&mut hash);
        hash
    }

    /// Store encrypted magic token with ChaCha20 encryption data
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `encrypted_token` - The Base58 encoded encrypted magic token (32 bytes encrypted data)
    /// * `encryption_blob` - 44 bytes: nonce[12] + secret_key[32] from ChaCha8RNG
    /// * `timestamp` - Original timestamp used in raw_magic_link creation
    /// * `next_param` - Optional next destination parameter
    /// * `expires_at` - Magic link expiration timestamp
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or database error
    pub fn store_magic_link_encrypted(
        env: DatabaseEnvironment,
        encrypted_token: &str,
        encryption_blob: &[u8; 44],
        timestamp: i64,
        next_param: Option<&str>,
        expires_at: u64,
    ) -> Result<(), SqliteError> {
        let connection = get_database_connection(env)?;
        
        // Decode Base58 encrypted token
        let encrypted_data = bs58::decode(encrypted_token)
            .into_vec()
            .map_err(|_| SqliteError::Io("Invalid Base58 encrypted token".to_string()))?;
            
        if encrypted_data.len() != 32 {
            return Err(SqliteError::Io("Encrypted token must be 32 bytes (ChaCha20 encrypted raw magic link)".to_string()));
        }
        
        // Create SHAKE-256 hash of encrypted data for database storage (16 bytes)
        let token_hash = Self::create_encrypted_token_hash(&encrypted_data);
        
        println!("Database: Creating encrypted magic link with SHAKE-256 hash");

        connection.execute(
            "INSERT INTO magiclinks (token_hash, timestamp, encryption_blob, next_param, expires_at) VALUES (?, ?, ?, ?, ?)",
            &[
                Value::Blob(token_hash.to_vec()),
                Value::Integer(timestamp),
                Value::Blob(encryption_blob.to_vec()),
                match next_param {
                    Some(next) => Value::Text(next.to_string()),
                    None => Value::Null,
                },
                Value::Integer(expires_at as i64),
            ],
        )?;

        println!("Database: Encrypted magic link stored successfully");
        Ok(())
    }

    /// Validate and consume encrypted magic token with ChaCha20 decryption
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `encrypted_token` - The Base58 encoded encrypted magic token to validate
    ///
    /// # Returns
    /// * `Result<(bool, Option<String>, Option<[u8; 16]>), SqliteError>` - (validation_result, next_param, user_id) or error
    pub fn validate_and_consume_magic_link_encrypted(
        env: DatabaseEnvironment,
        encrypted_token: &str,
    ) -> Result<(bool, Option<String>, Option<[u8; 16]>), SqliteError> {
        let connection = get_database_connection(env)?;
        
        // Decode Base58 encrypted token
        let encrypted_data = bs58::decode(encrypted_token)
            .into_vec()
            .map_err(|_| SqliteError::Io("Invalid Base58 encrypted token".to_string()))?;
            
        if encrypted_data.len() != 32 {
            return Err(SqliteError::Io("Encrypted token must be 32 bytes (ChaCha20 encrypted raw magic link)".to_string()));
        }
        
        // Create SHAKE-256 hash of encrypted data for database lookup
        let token_hash = Self::create_encrypted_token_hash(&encrypted_data);
        let now = Utc::now().timestamp() as u64;
        
        println!("Database: Validating encrypted magic link hash");

        // Check if magic link exists and is not expired, get encryption data
        let result = connection.execute(
            "SELECT timestamp, encryption_blob, next_param, expires_at FROM magiclinks WHERE token_hash = ?",
            &[Value::Blob(token_hash.to_vec())],
        )?;

        if let Some(row) = result.rows.first() {
            let expires_at = match &row.values[3] {
                Value::Integer(i) => *i as u64,
                _ => return Err(SqliteError::Io("Invalid expires_at type".to_string())),
            };

            if expires_at > now {
                // Get encryption data
                let encryption_blob = match &row.values[1] {
                    Value::Blob(blob) => {
                        if blob.len() != 44 {
                            return Err(SqliteError::Io("Invalid encryption_blob length".to_string()));
                        }
                        let mut blob_array = [0u8; 44];
                        blob_array.copy_from_slice(blob);
                        blob_array
                    },
                    _ => return Err(SqliteError::Io("Invalid encryption_blob type".to_string())),
                };

                let next_param = match &row.values[2] {
                    Value::Text(text) => Some(text.clone()),
                    Value::Null => None,
                    _ => return Err(SqliteError::Io("Invalid next_param type".to_string())),
                };

                // Extract nonce and secret_key from encryption_blob
                let mut nonce = [0u8; 12];
                let mut secret_key = [0u8; 32];
                nonce.copy_from_slice(&encryption_blob[..12]);
                secret_key.copy_from_slice(&encryption_blob[12..44]);

                // Validate magic token using JWT utils (this will decrypt and verify)
                match crate::utils::jwt::JwtUtils::validate_magic_token_encrypted(
                    encrypted_token, 
                    &nonce, 
                    &secret_key
                ) {
                    Ok((user_id, _expires_at)) => {
                        // Valid and not expired - delete it (consume)
                        connection.execute(
                            "DELETE FROM magiclinks WHERE token_hash = ?",
                            &[Value::Blob(token_hash.to_vec())],
                        )?;
                        
                        println!("Database: Encrypted magic link validated and consumed");
                        Ok((true, next_param, Some(user_id)))
                    },
                    Err(e) => {
                        println!("Database: Encrypted magic link validation failed: {}", e);
                        Ok((false, None, None))
                    }
                }
            } else {
                println!("Database: Encrypted magic link expired");
                Ok((false, None, None))
            }
        } else {
            println!("Database: Encrypted magic link not found");
            Ok((false, None, None))
        }
    }

    /// Ensure user exists in users table by user_id (insert if not exists)
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `user_id` - User ID bytes (16 bytes)
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or error
    pub fn ensure_user_exists(
        env: DatabaseEnvironment,
        user_id: &[u8; 16],
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

    /// Clean up expired magic links
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    ///
    /// # Returns
    /// * `Result<u32, SqliteError>` - Number of links deleted or database error
    pub fn cleanup_expired_links(env: DatabaseEnvironment) -> Result<u32, SqliteError> {
        let connection = get_database_connection(env)?;

        let now = Utc::now().timestamp() as u64;
        let _result = connection.execute(
            "DELETE FROM magiclinks WHERE expires_at < ?",
            &[Value::Integer(now as i64)],
        )?;

        // SQLite doesn't provide rows_affected in Spin SDK
        // We'll return 1 as a placeholder for successful cleanup
        Ok(1)
    }
}
