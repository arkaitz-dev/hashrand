//! Magic link storage operations
//!
//! Provides database storage functions for magic links including
//! storage, user management, and cleanup operations.

use super::magic_link_crypto::MagicLinkCrypto;
use super::magic_link_types::constants::*;
use crate::database::get_database_connection;
use bs58;
use chrono::Utc;
use spin_sdk::sqlite::{Error as SqliteError, Value};
use tracing::{debug, error};

/// Magic link storage operations
pub struct MagicLinkStorage;

impl MagicLinkStorage {
    /// Store encrypted magic token with Ed25519/X25519 public keys, UI host, and db_index
    ///
    /// # Arguments
    /// * `encrypted_token` - The Base58 encoded encrypted magic token (32 bytes encrypted data)
    /// * `encryption_blob` - 44 bytes: nonce[12] + secret_key[32] from ChaCha8RNG
    /// * `expires_at_nanos` - Expiration timestamp in nanoseconds (will be converted to hours for storage)
    /// * `next_param` - Next destination parameter (always provided, "/" for login)
    /// * `ed25519_pub_key` - Ed25519 public key as hex string (64 chars)
    /// * `x25519_pub_key` - X25519 public key as hex string (64 chars)
    /// * `ui_host` - UI host (domain only, e.g., "localhost" or "app.example.com")
    /// * `db_index` - Database index for user_privkey_context table (16 bytes)
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or database error
    pub fn store_magic_link_encrypted(
        encrypted_token: &str,
        encryption_blob: &[u8; ENCRYPTION_BLOB_LENGTH],
        expires_at_nanos: i64,
        next_param: &str,
        ed25519_pub_key: &str,
        x25519_pub_key: &str,
        ui_host: &str,
        db_index: &[u8; 16],
    ) -> Result<(), SqliteError> {
        let connection = get_database_connection()?;

        // Decode Base58 encrypted token
        let encrypted_data = bs58::decode(encrypted_token)
            .into_vec()
            .map_err(|_| SqliteError::Io("Invalid Base58 encrypted token".to_string()))?;

        if encrypted_data.len() != ENCRYPTED_TOKEN_LENGTH {
            return Err(SqliteError::Io(
                "Encrypted token must be 32 bytes (ChaCha20 encrypted raw magic link)".to_string(),
            ));
        }

        // Create Blake3 hash of encrypted data for database indexing (16 bytes)
        let token_hash = MagicLinkCrypto::create_encrypted_token_hash(&encrypted_data);

        // Decode Ed25519 public key from hex (64 chars = 32 bytes)
        if ed25519_pub_key.len() != ED25519_HEX_LENGTH {
            return Err(SqliteError::Io(format!(
                "Ed25519 public key must be 64 hex chars, got {}",
                ed25519_pub_key.len()
            )));
        }

        let ed25519_bytes = hex::decode(ed25519_pub_key)
            .map_err(|_| SqliteError::Io("Invalid hex Ed25519 public key".to_string()))?;

        if ed25519_bytes.len() != ED25519_BYTES_LENGTH {
            return Err(SqliteError::Io(format!(
                "Ed25519 public key must be 32 bytes, got {}",
                ed25519_bytes.len()
            )));
        }

        // Decode X25519 public key from hex (64 chars = 32 bytes)
        if x25519_pub_key.len() != ED25519_HEX_LENGTH { // Same length as Ed25519 (64 hex chars)
            return Err(SqliteError::Io(format!(
                "X25519 public key must be 64 hex chars, got {}",
                x25519_pub_key.len()
            )));
        }

        let x25519_bytes = hex::decode(x25519_pub_key)
            .map_err(|_| SqliteError::Io("Invalid hex X25519 public key".to_string()))?;

        if x25519_bytes.len() != ED25519_BYTES_LENGTH { // Same length as Ed25519 (32 bytes)
            return Err(SqliteError::Io(format!(
                "X25519 public key must be 32 bytes, got {}",
                x25519_bytes.len()
            )));
        }

        // Create merged payload: encryption_blob[44] + db_index[16] + ed25519_pub_key[32] + x25519_pub_key[32] + ui_host_len[2] + ui_host[variable] + next_param[variable]
        let ui_host_bytes = ui_host.as_bytes();
        let ui_host_len = ui_host_bytes.len() as u16;

        debug!(
            "🔒 [SECURITY] Storing ui_host in encrypted blob: '{}' (len: {})",
            ui_host, ui_host_len
        );

        let mut payload_plain = Vec::with_capacity(
            ENCRYPTION_BLOB_LENGTH
                + 16 // db_index length
                + ED25519_BYTES_LENGTH
                + ED25519_BYTES_LENGTH // X25519 has same length (32 bytes)
                + 2
                + ui_host_bytes.len()
                + next_param.len(),
        );
        payload_plain.extend_from_slice(encryption_blob);
        payload_plain.extend_from_slice(db_index);
        payload_plain.extend_from_slice(&ed25519_bytes);
        payload_plain.extend_from_slice(&x25519_bytes);
        payload_plain.extend_from_slice(&ui_host_len.to_be_bytes()); // Big-endian for consistency
        payload_plain.extend_from_slice(ui_host_bytes);
        payload_plain.extend_from_slice(next_param.as_bytes());

        // Convert encrypted_data to [u8; 32] for encryption function
        let mut encrypted_data_array = [0u8; ENCRYPTED_TOKEN_LENGTH];
        encrypted_data_array.copy_from_slice(&encrypted_data);

        // Encrypt payload using multi-layer security (Argon2id + HMAC + ChaCha20-Poly1305)
        let encrypted_payload =
            MagicLinkCrypto::encrypt_payload_content(&encrypted_data_array, &payload_plain)?;

        // Convert nanoseconds to hours for storage (cleanup purposes)
        let expires_at_hours = (expires_at_nanos / 1_000_000_000) / 3600;

        connection.execute(
            "INSERT INTO magiclinks (token_hash, expires_at, encrypted_payload) VALUES (?, ?, ?)",
            &[
                Value::Blob(token_hash.to_vec()),
                Value::Integer(expires_at_hours),
                Value::Blob(encrypted_payload),
            ],
        )?;

        Ok(())
    }

    /// Ensure user exists in users table by user_id (insert if not exists)
    ///
    /// # Arguments
    /// * `user_id` - User ID bytes (16 bytes)
    ///
    /// # Returns
    /// * `Result<(), SqliteError>` - Success or error
    pub fn ensure_user_exists(user_id: &[u8; USER_ID_LENGTH]) -> Result<(), SqliteError> {
        let connection = get_database_connection()?;

        // Insert user if it doesn't exist (ignore if already exists)
        match connection.execute(
            "INSERT OR IGNORE INTO users (user_id) VALUES (?)",
            &[Value::Blob(user_id.to_vec())],
        ) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Database: Error ensuring user exists: {}", e);
                Err(e)
            }
        }
    }

    /// Clean up expired magic links
    ///
    /// # Returns
    /// * `Result<u32, SqliteError>` - Number of links deleted or database error
    pub fn cleanup_expired_links() -> Result<u32, SqliteError> {
        let connection = get_database_connection()?;

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
