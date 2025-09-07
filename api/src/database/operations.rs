//! Database operations for user management
//!
//! Provides CRUD operations for the users table with proper error handling
//! and type safety using Spin's SQLite interface.

use crate::database::{connection::DatabaseEnvironment, get_database_connection, models::User};
use argon2::{Algorithm, Argon2, Params, Version};
use bs58;
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit},
};
use chrono::Utc;
use blake2::{Blake2bVar, Blake2bMac, digest::{Update, VariableOutput, Mac, KeyInit as Blake2KeyInit}};
use chacha20poly1305::consts::U32;
use rand_chacha::{ChaCha8Rng, rand_core::RngCore, rand_core::SeedableRng};
type MagicLinkKeys = ([u8; 32], [u8; 32], [u8; 32]);
type ValidationResult = (bool, Option<String>, Option<[u8; 16]>);
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
    /// Get magic link content encryption keys from environment
    fn get_mlink_content_keys() -> Result<MagicLinkKeys, SqliteError> {
        let cipher_key = spin_sdk::variables::get("mlink_content_cipher")
            .map_err(|e| SqliteError::Io(format!("Missing MLINK_CONTENT_CIPHER: {}", e)))?;
        let nonce_key = spin_sdk::variables::get("mlink_content_nonce")
            .map_err(|e| SqliteError::Io(format!("Missing MLINK_CONTENT_NONCE: {}", e)))?;
        let salt_key = spin_sdk::variables::get("mlink_content_salt")
            .map_err(|e| SqliteError::Io(format!("Missing MLINK_CONTENT_SALT: {}", e)))?;

        let cipher_bytes = hex::decode(&cipher_key)
            .map_err(|_| SqliteError::Io("Invalid MLINK_CONTENT_CIPHER format".to_string()))?;
        let nonce_bytes = hex::decode(&nonce_key)
            .map_err(|_| SqliteError::Io("Invalid MLINK_CONTENT_NONCE format".to_string()))?;
        let salt_bytes = hex::decode(&salt_key)
            .map_err(|_| SqliteError::Io("Invalid MLINK_CONTENT_SALT format".to_string()))?;

        if cipher_bytes.len() != 32 || nonce_bytes.len() != 32 || salt_bytes.len() != 32 {
            return Err(SqliteError::Io(
                "Magic link content keys must be 32 bytes each".to_string(),
            ));
        }

        let mut cipher_key = [0u8; 32];
        let mut nonce_key = [0u8; 32];
        let mut salt_key = [0u8; 32];
        cipher_key.copy_from_slice(&cipher_bytes);
        nonce_key.copy_from_slice(&nonce_bytes);
        salt_key.copy_from_slice(&salt_bytes);

        Ok((cipher_key, nonce_key, salt_key))
    }

    /// Encrypt encrypted_payload using multi-layer security
    ///
    /// Process:
    /// 1. Argon2id(encrypted_data, MLINK_CONTENT_SALT) → derived_key
    /// 2. HMAC-SHA3-256(derived_key, MLINK_CONTENT_NONCE) → ChaCha8RNG → nonce[12]
    /// 3. HMAC-SHA3-256(derived_key, MLINK_CONTENT_CIPHER) → ChaCha8RNG → cipher_key[32]
    /// 4. ChaCha20-Poly1305.encrypt(payload, nonce, cipher_key) → encrypted_blob
    fn encrypt_payload_content(
        encrypted_data: &[u8; 32],
        payload: &[u8],
    ) -> Result<Vec<u8>, SqliteError> {
        let (cipher_key_base, nonce_key_base, salt) = Self::get_mlink_content_keys()?;

        // Step 1: Derive key using Argon2id
        let params = Params::new(65536, 3, 4, Some(32))
            .map_err(|e| SqliteError::Io(format!("Argon2 params error: {}", e)))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut derived_key = [0u8; 32];
        argon2
            .hash_password_into(encrypted_data, &salt, &mut derived_key)
            .map_err(|e| SqliteError::Io(format!("Argon2 derivation error: {}", e)))?;

        // Step 2: Generate nonce using Blake2b keyed + ChaCha8RNG
        let mut nonce_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&nonce_key_base)
            .map_err(|_| SqliteError::Io("Invalid nonce key".to_string()))?;
        Mac::update(&mut nonce_hasher, &derived_key);
        let nonce_hmac = nonce_hasher.finalize().into_bytes();

        let mut nonce_seed = [0u8; 32];
        nonce_seed.copy_from_slice(&nonce_hmac[..32]);
        let mut nonce_rng = ChaCha8Rng::from_seed(nonce_seed);
        let mut nonce_bytes = [0u8; 12];
        nonce_rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Step 3: Generate cipher key using Blake2b keyed + ChaCha8RNG
        let mut cipher_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&cipher_key_base)
            .map_err(|_| SqliteError::Io("Invalid cipher key".to_string()))?;
        Mac::update(&mut cipher_hasher, &derived_key);
        let cipher_hmac = cipher_hasher.finalize().into_bytes();

        let mut cipher_seed = [0u8; 32];
        cipher_seed.copy_from_slice(&cipher_hmac[..32]);
        let mut cipher_rng = ChaCha8Rng::from_seed(cipher_seed);
        let mut cipher_key = [0u8; 32];
        cipher_rng.fill_bytes(&mut cipher_key);
        let key = Key::from_slice(&cipher_key);

        // Step 4: Encrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new(key);
        let ciphertext = cipher
            .encrypt(nonce, payload)
            .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 encryption error: {:?}", e)))?;

        println!("Database: Encrypted payload using multi-layer security");
        Ok(ciphertext)
    }

    /// Decrypt encrypted_payload using multi-layer security (reverse process)
    fn decrypt_payload_content(
        encrypted_data: &[u8; 32],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, SqliteError> {
        let (cipher_key_base, nonce_key_base, salt) = Self::get_mlink_content_keys()?;

        // Step 1: Derive key using Argon2id (same as encryption)
        let params = Params::new(65536, 3, 4, Some(32))
            .map_err(|e| SqliteError::Io(format!("Argon2 params error: {}", e)))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut derived_key = [0u8; 32];
        argon2
            .hash_password_into(encrypted_data, &salt, &mut derived_key)
            .map_err(|e| SqliteError::Io(format!("Argon2 derivation error: {}", e)))?;

        // Step 2: Regenerate nonce (same process as encryption)
        let mut nonce_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&nonce_key_base)
            .map_err(|_| SqliteError::Io("Invalid nonce key".to_string()))?;
        Mac::update(&mut nonce_hasher, &derived_key);
        let nonce_hmac = nonce_hasher.finalize().into_bytes();

        let mut nonce_seed = [0u8; 32];
        nonce_seed.copy_from_slice(&nonce_hmac[..32]);
        let mut nonce_rng = ChaCha8Rng::from_seed(nonce_seed);
        let mut nonce_bytes = [0u8; 12];
        nonce_rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Step 3: Regenerate cipher key (same process as encryption)
        let mut cipher_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&cipher_key_base)
            .map_err(|_| SqliteError::Io("Invalid cipher key".to_string()))?;
        Mac::update(&mut cipher_hasher, &derived_key);
        let cipher_hmac = cipher_hasher.finalize().into_bytes();

        let mut cipher_seed = [0u8; 32];
        cipher_seed.copy_from_slice(&cipher_hmac[..32]);
        let mut cipher_rng = ChaCha8Rng::from_seed(cipher_seed);
        let mut cipher_key = [0u8; 32];
        cipher_rng.fill_bytes(&mut cipher_key);
        let key = Key::from_slice(&cipher_key);

        // Step 4: Decrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new(key);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 decryption error: {:?}", e)))?;

        println!("Database: Decrypted payload using multi-layer security");
        Ok(plaintext)
    }

    /// Create Blake2b variable hash of encrypted magic token for database storage
    ///
    /// # Arguments
    /// * `encrypted_data` - The encrypted magic token bytes (32 bytes: ChaCha20 encrypted)
    ///
    /// # Returns
    /// * `[u8; 16]` - 16-byte Blake2b hash for database indexing
    fn create_encrypted_token_hash(encrypted_data: &[u8]) -> [u8; 16] {
        // Blake2b variable output(encrypted_data) → [16 bytes]
        let mut hasher = Blake2bVar::new(16).expect("Blake2b initialization failed");
        hasher.update(encrypted_data);

        let mut hash = [0u8; 16];
        hasher.finalize_variable(&mut hash).expect("Blake2b finalization failed");
        hash
    }

    /// Store encrypted magic token with ChaCha20 encryption data
    ///
    /// # Arguments
    /// * `env` - Database environment to use
    /// * `encrypted_token` - The Base58 encoded encrypted magic token (32 bytes encrypted data)
    /// * `encryption_blob` - 44 bytes: nonce[12] + secret_key[32] from ChaCha8RNG
    /// * `expires_at_nanos` - Expiration timestamp in nanoseconds (will be converted to hours for storage)
    /// * `next_param` - Optional next destination parameter
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or database error
    pub fn store_magic_link_encrypted(
        env: DatabaseEnvironment,
        encrypted_token: &str,
        encryption_blob: &[u8; 44],
        expires_at_nanos: i64,
        next_param: Option<&str>,
        random_hash: Option<&str>,
    ) -> Result<(), SqliteError> {
        let connection = get_database_connection(env)?;

        // Decode Base58 encrypted token
        let encrypted_data = bs58::decode(encrypted_token)
            .into_vec()
            .map_err(|_| SqliteError::Io("Invalid Base58 encrypted token".to_string()))?;

        if encrypted_data.len() != 32 {
            return Err(SqliteError::Io(
                "Encrypted token must be 32 bytes (ChaCha20 encrypted raw magic link)".to_string(),
            ));
        }

        // Create SHAKE-256 hash of encrypted data for database storage (16 bytes)
        let token_hash = Self::create_encrypted_token_hash(&encrypted_data);

        // Decode random_hash from base58 if provided
        let random_hash_bytes = if let Some(hash_str) = random_hash {
            bs58::decode(hash_str)
                .into_vec()
                .map_err(|_| SqliteError::Io("Invalid base58 random hash".to_string()))?
        } else {
            vec![0u8; 32] // Default to zeros if no hash provided
        };

        if random_hash_bytes.len() != 32 {
            return Err(SqliteError::Io(format!(
                "Random hash must be 32 bytes, got {}",
                random_hash_bytes.len()
            )));
        }

        // Create merged payload: encryption_blob[44] + random_hash[32] + next_param_bytes[variable]
        let mut payload_plain = Vec::with_capacity(44 + 32 + next_param.map_or(0, |s| s.len()));
        payload_plain.extend_from_slice(encryption_blob);
        payload_plain.extend_from_slice(&random_hash_bytes);
        if let Some(next) = next_param {
            payload_plain.extend_from_slice(next.as_bytes());
        }

        // Convert encrypted_data to [u8; 32] for encryption function
        let mut encrypted_data_array = [0u8; 32];
        encrypted_data_array.copy_from_slice(&encrypted_data);

        // Encrypt payload using multi-layer security (Argon2id + HMAC + ChaCha20-Poly1305)
        let encrypted_payload =
            Self::encrypt_payload_content(&encrypted_data_array, &payload_plain)?;

        // Convert nanoseconds to hours for storage (cleanup purposes)
        let expires_at_hours = (expires_at_nanos / 1_000_000_000) / 3600;

        println!("Database: Creating encrypted magic link with SHAKE-256 hash");

        connection.execute(
            "INSERT INTO magiclinks (token_hash, expires_at, encrypted_payload) VALUES (?, ?, ?)",
            &[
                Value::Blob(token_hash.to_vec()),
                Value::Integer(expires_at_hours),
                Value::Blob(encrypted_payload),
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
    /// * `Result<ValidationResult, SqliteError>` - (validation_result, next_param, user_id) or error
    pub fn validate_and_consume_magic_link_encrypted(
        env: DatabaseEnvironment,
        encrypted_token: &str,
        provided_hash: Option<&str>,
    ) -> Result<ValidationResult, SqliteError> {
        let connection = get_database_connection(env)?;

        // Decode Base58 encrypted token
        let encrypted_data = bs58::decode(encrypted_token)
            .into_vec()
            .map_err(|_| SqliteError::Io("Invalid Base58 encrypted token".to_string()))?;

        if encrypted_data.len() != 32 {
            return Err(SqliteError::Io(
                "Encrypted token must be 32 bytes (ChaCha20 encrypted raw magic link)".to_string(),
            ));
        }

        // Create SHAKE-256 hash of encrypted data for database lookup
        let token_hash = Self::create_encrypted_token_hash(&encrypted_data);

        println!("Database: Validating encrypted magic link hash");

        // Check if magic link exists and is not expired, get encrypted payload
        let result = connection.execute(
            "SELECT expires_at, encrypted_payload FROM magiclinks WHERE token_hash = ?",
            &[Value::Blob(token_hash.to_vec())],
        )?;

        if let Some(row) = result.rows.first() {
            // Get encrypted payload from database
            let encrypted_payload_blob = match &row.values[1] {
                Value::Blob(blob) => blob,
                _ => {
                    println!("Database: Invalid encrypted_payload type");
                    return Ok((false, None, None));
                }
            };

            // Convert encrypted_data to [u8; 32] for decryption function
            let mut encrypted_data_array = [0u8; 32];
            encrypted_data_array.copy_from_slice(&encrypted_data);

            // Try to decrypt payload - if it fails, magic link is invalid
            let payload_plain = match Self::decrypt_payload_content(
                &encrypted_data_array,
                encrypted_payload_blob,
            ) {
                Ok(payload) => payload,
                Err(e) => {
                    println!("Database: Encrypted payload decryption failed: {}", e);
                    return Ok((false, None, None));
                }
            };

            // Extract encryption_blob, random_hash, and next_param from decrypted payload
            if payload_plain.len() < 76 {
                // 44 + 32 bytes minimum
                println!("Database: Invalid decrypted payload length (minimum 76 bytes)");
                return Ok((false, None, None));
            }

            // Extract encryption_blob (first 44 bytes)
            let mut encryption_blob = [0u8; 44];
            encryption_blob.copy_from_slice(&payload_plain[..44]);

            // Extract stored_hash (next 32 bytes)
            let stored_hash_bytes = &payload_plain[44..76];
            let stored_hash_base58 = bs58::encode(stored_hash_bytes).into_string();

            // Verify hash matches the provided hash from frontend
            if let Some(provided) = provided_hash {
                if provided != stored_hash_base58 {
                    println!("Database: Random hash verification failed");
                    return Ok((false, None, None));
                }
            } else {
                // If no hash provided, reject the magic link
                println!("Database: No random hash provided for validation");
                return Ok((false, None, None));
            }

            // Extract next_param (remaining bytes as UTF-8 string if any)
            let next_param = if payload_plain.len() > 76 {
                match std::str::from_utf8(&payload_plain[76..]) {
                    Ok(s) => Some(s.to_string()),
                    Err(_) => {
                        println!("Database: Invalid UTF-8 in decrypted next_param bytes");
                        return Ok((false, None, None));
                    }
                }
            } else {
                None
            };

            // Extract nonce and secret_key from encryption_blob
            let mut nonce = [0u8; 12];
            let mut secret_key = [0u8; 32];
            nonce.copy_from_slice(&encryption_blob[..12]);
            secret_key.copy_from_slice(&encryption_blob[12..44]);

            // Validate magic token using JWT utils - this validates internal timestamp vs current time
            match crate::utils::jwt::JwtUtils::validate_magic_token_encrypted(
                encrypted_token,
                &nonce,
                &secret_key,
            ) {
                Ok((user_id, _expires_at)) => {
                    // Valid and not expired - delete it (consume)
                    connection.execute(
                        "DELETE FROM magiclinks WHERE token_hash = ?",
                        &[Value::Blob(token_hash.to_vec())],
                    )?;

                    println!("Database: Encrypted magic link validated and consumed");
                    Ok((true, next_param, Some(user_id)))
                }
                Err(e) => {
                    println!("Database: Magic link internal validation failed: {}", e);
                    Ok((false, None, None))
                }
            }
        } else {
            println!("Database: Encrypted magic link not found in database");
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

        let now_hours = (Utc::now().timestamp() / 3600) as u64;
        let _result = connection.execute(
            "DELETE FROM magiclinks WHERE expires_at < ?",
            &[Value::Integer(now_hours as i64)],
        )?;

        // SQLite doesn't provide rows_affected in Spin SDK
        // We'll return 1 as a placeholder for successful cleanup
        Ok(1)
    }
}
