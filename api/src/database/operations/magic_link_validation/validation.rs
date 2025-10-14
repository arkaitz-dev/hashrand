use super::super::magic_link_crypto::MagicLinkCrypto;
use super::super::magic_link_types::ValidationResult;
use super::super::magic_link_types::constants::*;
use super::extraction::extract_payload_components;
use super::utilities::{copy_to_array, create_validation_error};
use crate::database::get_database_connection;
use spin_sdk::sqlite::{Error as SqliteError, Value};
use tracing::{debug, error, warn};

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
    /// * `Result<ValidationResult, SqliteError>` - (validation_result, next_param, user_id, pub_key, ui_host) or error
    pub fn validate_and_consume_magic_link_encrypted(
        encrypted_token: &str,
    ) -> Result<ValidationResult, SqliteError> {
        let connection = get_database_connection()?;

        // Step 1: Decode and hash encrypted token
        let (encrypted_data, token_hash) = decode_and_hash_token(encrypted_token)?;

        debug!("Database: Validating encrypted magic link hash");

        // Step 2: Retrieve encrypted payload from database
        let encrypted_payload_blob = match retrieve_encrypted_payload(&connection, &token_hash)? {
            Some(blob) => blob,
            None => {
                warn!("Database: Encrypted magic link not found in database");
                return Ok(create_validation_error());
            }
        };

        // Step 3: Decrypt payload
        let payload_plain = match MagicLinkCrypto::decrypt_payload_content(
            &encrypted_data,
            &encrypted_payload_blob,
        ) {
            Ok(payload) => payload,
            Err(e) => {
                error!("Database: Encrypted payload decryption failed: {}", e);
                return Ok(create_validation_error());
            }
        };

        // Step 4: Extract components from payload
        let (encryption_blob, pub_key_array, ui_host, next_param) =
            match extract_payload_components(&payload_plain) {
                Ok(components) => components,
                Err(error_tuple) => return Ok(error_tuple),
            };

        // Step 5: Extract nonce and secret_key from encryption_blob
        let mut nonce = [0u8; NONCE_LENGTH];
        let mut secret_key = [0u8; SECRET_KEY_LENGTH];
        copy_to_array(&mut nonce, &encryption_blob[..NONCE_LENGTH]);
        copy_to_array(
            &mut secret_key,
            &encryption_blob[NONCE_LENGTH..ENCRYPTION_BLOB_LENGTH],
        );

        // Step 6: Validate magic token using JWT utils
        match crate::utils::jwt::JwtUtils::validate_magic_token_encrypted(
            encrypted_token,
            &nonce,
            &secret_key,
        ) {
            Ok((user_id, _expires_at)) => {
                // Valid and not expired - consume (delete) the magic link
                connection.execute(
                    "DELETE FROM magiclinks WHERE token_hash = ?",
                    &[Value::Blob(token_hash.to_vec())],
                )?;

                debug!("Database: Encrypted magic link validated and consumed");
                Ok((
                    true,
                    next_param,
                    Some(user_id),
                    Some(pub_key_array),
                    ui_host,
                ))
            }
            Err(e) => {
                error!("Database: Magic link internal validation failed: {}", e);
                Ok(create_validation_error())
            }
        }
    }
}

/// Decode Base58 encrypted token and create Blake3 hash
fn decode_and_hash_token(encrypted_token: &str) -> Result<([u8; 32], [u8; 16]), SqliteError> {
    // Decode Base58 encrypted token
    let encrypted_data = bs58::decode(encrypted_token)
        .into_vec()
        .map_err(|_| SqliteError::Io("Invalid Base58 encrypted token".to_string()))?;

    if encrypted_data.len() != ENCRYPTED_TOKEN_LENGTH {
        return Err(SqliteError::Io(
            "Encrypted token must be 32 bytes (ChaCha20 encrypted raw magic link)".to_string(),
        ));
    }

    // Convert to fixed-size array
    let mut encrypted_data_array = [0u8; ENCRYPTED_TOKEN_LENGTH];
    copy_to_array(&mut encrypted_data_array, &encrypted_data);

    // Create Blake3 hash of encrypted data for database lookup
    let token_hash = MagicLinkCrypto::create_encrypted_token_hash(&encrypted_data);

    Ok((encrypted_data_array, token_hash))
}

/// Retrieve encrypted payload from database
fn retrieve_encrypted_payload(
    connection: &spin_sdk::sqlite::Connection,
    token_hash: &[u8; 16],
) -> Result<Option<Vec<u8>>, SqliteError> {
    let result = connection.execute(
        "SELECT expires_at, encrypted_payload FROM magiclinks WHERE token_hash = ?",
        &[Value::Blob(token_hash.to_vec())],
    )?;

    if let Some(row) = result.rows.first() {
        match &row.values[1] {
            Value::Blob(blob) => Ok(Some(blob.clone())),
            _ => {
                error!("Database: Invalid encrypted_payload type");
                Ok(None)
            }
        }
    } else {
        Ok(None)
    }
}
