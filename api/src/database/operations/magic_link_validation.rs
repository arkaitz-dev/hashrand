//! Magic link validation operations
//!
//! Provides validation and consumption functions for encrypted magic links
//! with complete payload decryption and JWT token validation.

use super::magic_link_crypto::MagicLinkCrypto;
use super::magic_link_types::{ValidationResult, constants::*};
use crate::database::get_database_connection;
use bs58;
use spin_sdk::sqlite::{Error as SqliteError, Value};

/// Magic link validation operations
pub struct MagicLinkValidation;

impl MagicLinkValidation {
    /// Validate and consume encrypted magic token and extract stored Ed25519 public key
    ///
    /// This function performs the complete validation workflow:
    /// 1. Decode and hash the encrypted token for database lookup
    /// 2. Retrieve and decrypt the stored payload using multi-layer security
    /// 3. Extract encryption blob, Ed25519 public key, and next parameter
    /// 4. Validate the magic token using JWT utils with internal timestamp validation
    /// 5. Consume (delete) the magic link upon successful validation
    ///
    /// # Arguments
    /// * `encrypted_token` - The Base58 encoded encrypted magic token to validate
    ///
    /// # Returns
    /// * `Result<ValidationResult, SqliteError>` - (validation_result, next_param, user_id, pub_key) or error
    pub fn validate_and_consume_magic_link_encrypted(
        encrypted_token: &str,
    ) -> Result<ValidationResult, SqliteError> {
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

        // Create Blake2b hash of encrypted data for database lookup
        let token_hash = MagicLinkCrypto::create_encrypted_token_hash(&encrypted_data);

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
                    return Ok((false, None, None, None));
                }
            };

            // Convert encrypted_data to [u8; 32] for decryption function
            let mut encrypted_data_array = [0u8; ENCRYPTED_TOKEN_LENGTH];
            encrypted_data_array.copy_from_slice(&encrypted_data);

            // Try to decrypt payload - if it fails, magic link is invalid
            let payload_plain = match MagicLinkCrypto::decrypt_payload_content(
                &encrypted_data_array,
                encrypted_payload_blob,
            ) {
                Ok(payload) => payload,
                Err(e) => {
                    println!("Database: Encrypted payload decryption failed: {}", e);
                    return Ok((false, None, None, None));
                }
            };

            // Extract encryption_blob, pub_key, and next_param from decrypted payload
            if payload_plain.len() < MIN_PAYLOAD_LENGTH {
                // 44 + 32 bytes minimum
                println!("Database: Invalid decrypted payload length (minimum 76 bytes)");
                return Ok((false, None, None, None));
            }

            // Extract encryption_blob (first 44 bytes)
            let mut encryption_blob = [0u8; ENCRYPTION_BLOB_LENGTH];
            encryption_blob.copy_from_slice(&payload_plain[..ENCRYPTION_BLOB_LENGTH]);

            // Extract stored pub_key (next 32 bytes) - this is the user's Ed25519 public key
            let stored_pub_key_bytes = &payload_plain[ENCRYPTION_BLOB_LENGTH..MIN_PAYLOAD_LENGTH];
            let mut pub_key_array = [0u8; ED25519_BYTES_LENGTH];
            pub_key_array.copy_from_slice(stored_pub_key_bytes);

            println!("Database: Successfully extracted Ed25519 public key from stored payload");

            // Extract next_param (remaining bytes as UTF-8 string if any)
            println!(
                "🔍 DEBUG EXTRACT: payload_plain.len() = {}",
                payload_plain.len()
            );
            let next_param = if payload_plain.len() > MIN_PAYLOAD_LENGTH {
                match std::str::from_utf8(&payload_plain[MIN_PAYLOAD_LENGTH..]) {
                    Ok(s) => {
                        println!("🔍 DEBUG EXTRACT: Extracted next_param: '{}'", s);
                        Some(s.to_string())
                    }
                    Err(_) => {
                        println!("Database: Invalid UTF-8 in decrypted next_param bytes");
                        return Ok((false, None, None, None));
                    }
                }
            } else {
                println!("🔍 DEBUG EXTRACT: payload <= 76 bytes, next_param = None");
                None
            };
            println!("🔍 DEBUG EXTRACT: Final next_param: {:?}", next_param);

            // Extract nonce and secret_key from encryption_blob
            let mut nonce = [0u8; NONCE_LENGTH];
            let mut secret_key = [0u8; SECRET_KEY_LENGTH];
            nonce.copy_from_slice(&encryption_blob[..NONCE_LENGTH]);
            secret_key.copy_from_slice(&encryption_blob[NONCE_LENGTH..ENCRYPTION_BLOB_LENGTH]);

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
                    Ok((true, next_param, Some(user_id), Some(pub_key_array)))
                }
                Err(e) => {
                    println!("Database: Magic link internal validation failed: {}", e);
                    Ok((false, None, None, None))
                }
            }
        } else {
            println!("Database: Encrypted magic link not found in database");
            Ok((false, None, None, None))
        }
    }
}
