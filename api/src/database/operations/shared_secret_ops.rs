//! Shared secret database operations - Business logic
//!
//! Provides high-level business operations for shared secrets including
//! creation, retrieval, validation, and tracking.

use super::shared_secret_crypto::SharedSecretCrypto;
use super::shared_secret_storage::SharedSecretStorage;
use super::shared_secret_types::{SecretRole, SharedSecretPayload, constants::*};
use crate::utils::crypto::{decrypt_with_ecdh, ed25519_public_to_x25519, get_backend_x25519_private_key};
use chrono::Utc;
use spin_sdk::sqlite::Error as SqliteError;
use tracing::{debug, warn};

/// Shared secret operations - High-level business logic
pub struct SharedSecretOps;

impl SharedSecretOps {
    /// Create a pair of shared secret entries with E2E encrypted data (sender + receiver)
    ///
    /// # Arguments
    /// * `sender_email` - Sender email address
    /// * `receiver_email` - Receiver email address
    /// * `encrypted_secret` - ChaCha20-Poly1305 encrypted secret from frontend
    /// * `key_material` - Decrypted key material (nonce[12] + cipher_key[32])
    /// * `otp` - Optional 9-digit OTP
    /// * `expires_hours` - Expiration in hours (1-72)
    /// * `max_reads` - Maximum reads for receiver (1-10)
    /// * `sender_db_index` - Pre-computed sender database index (32 bytes)
    /// * `receiver_db_index` - Pre-computed receiver database index (32 bytes)
    /// * `reference_hash` - Pre-generated reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<[u8; REFERENCE_HASH_LENGTH], SqliteError>` - Reference hash or error
    #[allow(clippy::too_many_arguments)]
    pub fn create_secret_pair(
        sender_email: &str,
        receiver_email: &str,
        encrypted_secret: &[u8],
        key_material: &[u8; KEY_MATERIAL_LENGTH],
        otp: Option<String>,
        expires_hours: i64,
        max_reads: i64,
        sender_db_index: &[u8; 32],                   // DB_INDEX_LENGTH
        receiver_db_index: &[u8; 32],                 // DB_INDEX_LENGTH
        reference_hash: &[u8; REFERENCE_HASH_LENGTH], // Pre-generated reference hash
    ) -> Result<[u8; REFERENCE_HASH_LENGTH], SqliteError> {
        // Validate inputs
        if encrypted_secret.is_empty() {
            return Err(SqliteError::Io("Encrypted secret cannot be empty".to_string()));
        }

        if !(MIN_EXPIRES_HOURS..=MAX_EXPIRES_HOURS).contains(&expires_hours) {
            return Err(SqliteError::Io(format!(
                "Expiration must be between {} and {} hours",
                MIN_EXPIRES_HOURS, MAX_EXPIRES_HOURS
            )));
        }

        if !(MIN_READS..=MAX_READS).contains(&max_reads) {
            return Err(SqliteError::Io(format!(
                "Max reads must be between {} and {}",
                MIN_READS, MAX_READS
            )));
        }

        if let Some(ref otp_val) = otp
            && (otp_val.len() != OTP_LENGTH || !otp_val.chars().all(|c| c.is_ascii_digit()))
        {
            return Err(SqliteError::Io(format!(
                "OTP must be exactly {} digits",
                OTP_LENGTH
            )));
        }

        // ============================================================================
        // v4: E2E ENCRYPTION - Store encrypted_secret + key_material in payload
        // ============================================================================

        // 1. Create payload (same for both sender and receiver)
        let created_at = Utc::now().timestamp();
        let mut payload = Vec::new();

        // Serialize: sender_email_len[2] + sender_email + receiver_email_len[2] + receiver_email +
        //            encrypted_secret_len[4] + encrypted_secret + key_material[44] +
        //            otp_len[1] + otp + created_at[8] + reference_hash[16] + max_reads[8]
        let sender_email_bytes = sender_email.as_bytes();
        let receiver_email_bytes = receiver_email.as_bytes();

        payload.extend_from_slice(&(sender_email_bytes.len() as u16).to_be_bytes());
        payload.extend_from_slice(sender_email_bytes);
        payload.extend_from_slice(&(receiver_email_bytes.len() as u16).to_be_bytes());
        payload.extend_from_slice(receiver_email_bytes);
        payload.extend_from_slice(&(encrypted_secret.len() as u32).to_be_bytes());
        payload.extend_from_slice(encrypted_secret);
        payload.extend_from_slice(key_material); // Fixed 44 bytes

        if let Some(otp_val) = &otp {
            payload.push(OTP_LENGTH as u8);
            payload.extend_from_slice(otp_val.as_bytes());
        } else {
            payload.push(0); // No OTP
        }

        payload.extend_from_slice(&created_at.to_be_bytes());
        payload.extend_from_slice(reference_hash); // Already a reference
        payload.extend_from_slice(&max_reads.to_be_bytes());

        // ============================================================================
        // 2. LAYER 2: Encrypt payload ONCE for tracking (ChaCha20-Poly1305 with key_material)
        // ============================================================================
        let encrypted_payload_tracking =
            SharedSecretCrypto::encrypt_payload_with_material(key_material, &payload)?;

        // ============================================================================
        // 3. LAYER 1: Encrypt key_material TWICE (ChaCha20 for sender + receiver)
        // ============================================================================
        let encrypted_key_material_sender =
            SharedSecretCrypto::encrypt_key_material(sender_db_index, key_material)?;

        let encrypted_key_material_receiver =
            SharedSecretCrypto::encrypt_key_material(receiver_db_index, key_material)?;

        // ============================================================================
        // 4. Calculate expiration timestamp
        // ============================================================================
        let expires_at = (Utc::now().timestamp() / 3600) + expires_hours;

        // ============================================================================
        // 5. CRITICAL ORDER: Store tracking FIRST, then shared_secrets
        // ============================================================================
        // Reason: If tracking fails, we don't create orphaned key_material entries
        // If shared_secrets fail, we have orphaned payload (cleaned up later)

        // FIRST: Store tracking with encrypted payload
        SharedSecretStorage::store_tracking_with_payload(
            reference_hash,
            max_reads,
            expires_at,
            &encrypted_payload_tracking,
        )?;

        // SECOND: Store sender entry (encrypted key_material, unlimited reads)
        SharedSecretStorage::store_shared_secret(
            sender_db_index,
            &encrypted_key_material_sender,
            expires_at,
            SecretRole::Sender,
        )?;

        // THIRD: Store receiver entry (encrypted key_material, limited reads)
        SharedSecretStorage::store_shared_secret(
            receiver_db_index,
            &encrypted_key_material_receiver,
            expires_at,
            SecretRole::Receiver,
        )?;

        debug!(
            "✅ SharedSecret: Created pair (tracking → sender → receiver) with centralized payload (expires in {}h)",
            expires_hours
        );

        Ok(*reference_hash) // Dereference to return owned array
    }

    /// Create a pair of shared secret entries with E2E encryption (high-level ECDH wrapper)
    ///
    /// This function handles the E2E encryption workflow:
    /// 1. Receives encrypted_secret (ChaCha20) + encrypted_key_material (ECDH) from frontend
    /// 2. Decrypts key_material using backend's X25519 private key + sender's X25519 public key
    /// 3. Calls create_secret_pair() with decrypted data
    ///
    /// # Arguments
    /// * `sender_email` - Sender email address
    /// * `receiver_email` - Receiver email address
    /// * `encrypted_secret` - ChaCha20-Poly1305 encrypted secret from frontend
    /// * `encrypted_key_material` - ECDH encrypted key material from frontend (60 bytes: 44 + 16 MAC)
    /// * `sender_public_key_hex` - Sender's Ed25519 public key as hex string (64 chars)
    /// * `otp` - Optional 9-digit OTP
    /// * `expires_hours` - Expiration in hours (1-72)
    /// * `max_reads` - Maximum reads for receiver (1-10)
    /// * `sender_db_index` - Pre-computed sender database index (32 bytes)
    /// * `receiver_db_index` - Pre-computed receiver database index (32 bytes)
    /// * `reference_hash` - Pre-generated reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<[u8; REFERENCE_HASH_LENGTH], SqliteError>` - Reference hash or error
    ///
    /// # Errors
    /// Returns error if:
    /// - Invalid sender public key format
    /// - ECDH decryption fails
    /// - Key material length mismatch
    /// - Any validation in create_secret_pair() fails
    #[allow(clippy::too_many_arguments)]
    pub fn create_secret_pair_with_ecdh(
        sender_email: &str,
        receiver_email: &str,
        encrypted_secret: &[u8],
        encrypted_key_material: &[u8],
        sender_public_key_hex: &str,
        otp: Option<String>,
        expires_hours: i64,
        max_reads: i64,
        sender_db_index: &[u8; 32],
        receiver_db_index: &[u8; 32],
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<[u8; REFERENCE_HASH_LENGTH], SqliteError> {
        debug!("🔐 SharedSecret: Starting E2E encryption workflow");

        // 1. Validate sender public key format
        if sender_public_key_hex.len() != 64 {
            return Err(SqliteError::Io(format!(
                "Invalid sender public key hex length: {} (expected 64)",
                sender_public_key_hex.len()
            )));
        }

        let sender_ed25519_public = hex::decode(sender_public_key_hex).map_err(|e| {
            SqliteError::Io(format!("Failed to decode sender public key hex: {}", e))
        })?;

        if sender_ed25519_public.len() != 32 {
            return Err(SqliteError::Io(format!(
                "Invalid sender public key byte length: {} (expected 32)",
                sender_ed25519_public.len()
            )));
        }

        let sender_ed25519_public_array: [u8; 32] = sender_ed25519_public
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to convert sender public key to array".to_string()))?;

        // 2. Convert sender's Ed25519 public key → X25519 public key
        debug!("🔐 SharedSecret: Converting sender Ed25519 → X25519 public key");
        let sender_x25519_public = ed25519_public_to_x25519(&sender_ed25519_public_array)?;

        // 3. Calculate sender's user_id for per-user X25519 key derivation
        debug!("🔐 SharedSecret: Calculating sender user_id");
        let sender_user_id = SharedSecretCrypto::calculate_user_id(sender_email)?;

        // 4. Get backend's per-user X25519 private key
        debug!("🔐 SharedSecret: Deriving backend X25519 private key (per-user)");
        let backend_x25519_private = get_backend_x25519_private_key(&sender_user_id, sender_public_key_hex)?;

        // 5. Decrypt key_material using ECDH
        debug!("🔐 SharedSecret: Decrypting key_material with ECDH");
        let decrypted_key_material = decrypt_with_ecdh(
            encrypted_key_material,
            &backend_x25519_private,
            &sender_x25519_public,
        )?;

        // 6. Validate decrypted key_material length
        if decrypted_key_material.len() != KEY_MATERIAL_LENGTH {
            return Err(SqliteError::Io(format!(
                "Invalid decrypted key_material length: {} (expected {})",
                decrypted_key_material.len(),
                KEY_MATERIAL_LENGTH
            )));
        }

        let key_material: [u8; KEY_MATERIAL_LENGTH] = decrypted_key_material
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to convert key_material to array".to_string()))?;

        debug!("✅ SharedSecret: Key material decrypted successfully, calling create_secret_pair");

        // 7. Call create_secret_pair with decrypted data
        Self::create_secret_pair(
            sender_email,
            receiver_email,
            encrypted_secret,
            &key_material,
            otp,
            expires_hours,
            max_reads,
            sender_db_index,
            receiver_db_index,
            reference_hash,
        )
    }

    /// Deserialize payload bytes into SharedSecretPayload
    ///
    /// # Arguments
    /// * `payload` - Decrypted payload bytes
    ///
    /// # Returns
    /// * `Result<SharedSecretPayload, SqliteError>` - Deserialized payload or error
    pub fn deserialize_payload(payload: &[u8]) -> Result<SharedSecretPayload, SqliteError> {
        let mut offset = 0;

        // Read sender_email
        if payload.len() < offset + 2 {
            return Err(SqliteError::Io(
                "Payload too short for sender_email_len".to_string(),
            ));
        }
        let sender_email_len = u16::from_be_bytes([payload[offset], payload[offset + 1]]) as usize;
        offset += 2;

        if payload.len() < offset + sender_email_len {
            return Err(SqliteError::Io(
                "Payload too short for sender_email".to_string(),
            ));
        }
        let sender_email =
            String::from_utf8(payload[offset..offset + sender_email_len].to_vec())
                .map_err(|_| SqliteError::Io("Invalid UTF-8 in sender_email".to_string()))?;
        offset += sender_email_len;

        // Read receiver_email
        if payload.len() < offset + 2 {
            return Err(SqliteError::Io(
                "Payload too short for receiver_email_len".to_string(),
            ));
        }
        let receiver_email_len =
            u16::from_be_bytes([payload[offset], payload[offset + 1]]) as usize;
        offset += 2;

        if payload.len() < offset + receiver_email_len {
            return Err(SqliteError::Io(
                "Payload too short for receiver_email".to_string(),
            ));
        }
        let receiver_email =
            String::from_utf8(payload[offset..offset + receiver_email_len].to_vec())
                .map_err(|_| SqliteError::Io("Invalid UTF-8 in receiver_email".to_string()))?;
        offset += receiver_email_len;

        // Read encrypted_secret
        if payload.len() < offset + 4 {
            return Err(SqliteError::Io(
                "Payload too short for encrypted_secret_len".to_string(),
            ));
        }
        let encrypted_secret_len = u32::from_be_bytes([
            payload[offset],
            payload[offset + 1],
            payload[offset + 2],
            payload[offset + 3],
        ]) as usize;
        offset += 4;

        if payload.len() < offset + encrypted_secret_len {
            return Err(SqliteError::Io(
                "Payload too short for encrypted_secret".to_string(),
            ));
        }
        let encrypted_secret = payload[offset..offset + encrypted_secret_len].to_vec();
        offset += encrypted_secret_len;

        // Read key_material (fixed 44 bytes)
        if payload.len() < offset + KEY_MATERIAL_LENGTH {
            return Err(SqliteError::Io(
                "Payload too short for key_material".to_string(),
            ));
        }
        let key_material = payload[offset..offset + KEY_MATERIAL_LENGTH].to_vec();
        offset += KEY_MATERIAL_LENGTH;

        // Read OTP
        if payload.len() < offset + 1 {
            return Err(SqliteError::Io("Payload too short for otp_len".to_string()));
        }
        let otp_len = payload[offset] as usize;
        offset += 1;

        let otp = if otp_len > 0 {
            if payload.len() < offset + otp_len {
                return Err(SqliteError::Io("Payload too short for otp".to_string()));
            }
            let otp_str = String::from_utf8(payload[offset..offset + otp_len].to_vec())
                .map_err(|_| SqliteError::Io("Invalid UTF-8 in OTP".to_string()))?;
            offset += otp_len;
            Some(otp_str)
        } else {
            None
        };

        // Read created_at
        if payload.len() < offset + 8 {
            return Err(SqliteError::Io(
                "Payload too short for created_at".to_string(),
            ));
        }
        let created_at = i64::from_be_bytes([
            payload[offset],
            payload[offset + 1],
            payload[offset + 2],
            payload[offset + 3],
            payload[offset + 4],
            payload[offset + 5],
            payload[offset + 6],
            payload[offset + 7],
        ]);
        offset += 8;

        // Read reference_hash
        if payload.len() < offset + REFERENCE_HASH_LENGTH {
            return Err(SqliteError::Io(
                "Payload too short for reference_hash".to_string(),
            ));
        }
        let reference_hash = payload[offset..offset + REFERENCE_HASH_LENGTH].to_vec();
        offset += REFERENCE_HASH_LENGTH;

        // Read max_reads
        if payload.len() < offset + 8 {
            return Err(SqliteError::Io(
                "Payload too short for max_reads".to_string(),
            ));
        }
        let max_reads = i64::from_be_bytes([
            payload[offset],
            payload[offset + 1],
            payload[offset + 2],
            payload[offset + 3],
            payload[offset + 4],
            payload[offset + 5],
            payload[offset + 6],
            payload[offset + 7],
        ]);

        Ok(SharedSecretPayload {
            sender_email,
            receiver_email,
            encrypted_secret,
            key_material,
            otp,
            created_at,
            reference_hash,
            max_reads,
        })
    }

    /// Read a secret, decrypt, and get pending_reads from tracking (v3 - NEW with centralized payload)
    ///
    /// # Arguments
    /// * `db_index` - Database index (32 bytes) - PRIMARY KEY
    /// * `reference_hash` - Reference hash (16 bytes) - Required for payload retrieval
    ///
    /// # Returns
    /// * `Result<(SharedSecretPayload, i64, i64, SecretRole), SqliteError>` - (payload, pending_reads, expires_at, role) or error
    ///
    /// Note: Role is returned from database for backward compatibility, but should be validated from hash checksum
    pub fn read_secret(
        db_index: &[u8; DB_INDEX_LENGTH],
        reference_hash: &[u8; REFERENCE_HASH_LENGTH],
    ) -> Result<(SharedSecretPayload, i64, i64, SecretRole), SqliteError> {
        // ============================================================================
        // 1. RETRIEVE ENCRYPTED_KEY_MATERIAL from shared_secrets
        // ============================================================================
        let (encrypted_key_material, expires_at, role) =
            SharedSecretStorage::retrieve_secret(db_index)?
                .ok_or_else(|| SqliteError::Io("Secret not found in shared_secrets".to_string()))?;

        // Validate length (must be 44 bytes - key_material)
        if encrypted_key_material.len() != KEY_MATERIAL_LENGTH {
            return Err(SqliteError::Io(format!(
                "Invalid encrypted_key_material length: expected {}, got {}",
                KEY_MATERIAL_LENGTH,
                encrypted_key_material.len()
            )));
        }

        // ============================================================================
        // 2. VALIDATE EXPIRATION (before decryption - optimization)
        // ============================================================================
        let now_hours = Utc::now().timestamp() / 3600;
        if expires_at < now_hours {
            SharedSecretStorage::delete_secret(db_index)?;
            return Err(SqliteError::Io("Secret has expired".to_string()));
        }

        // ============================================================================
        // 3. DECRYPT KEY_MATERIAL (Layer 1: ChaCha20)
        // ============================================================================
        let random_key_material =
            SharedSecretCrypto::decrypt_key_material(db_index, &encrypted_key_material)?;

        // ============================================================================
        // 4. RETRIEVE ENCRYPTED_PAYLOAD from tracking
        // ============================================================================
        let encrypted_payload_tracking =
            SharedSecretStorage::retrieve_tracking_payload(reference_hash)?.ok_or_else(|| {
                SqliteError::Io("Payload not found in tracking table".to_string())
            })?;

        // ============================================================================
        // 5. DECRYPT PAYLOAD (Layer 2: ChaCha20-Poly1305)
        // ============================================================================
        let decrypted_payload = SharedSecretCrypto::decrypt_payload_with_material(
            &random_key_material,
            &encrypted_payload_tracking,
        )?;

        // ============================================================================
        // 6. DESERIALIZE PAYLOAD
        // ============================================================================
        let payload = Self::deserialize_payload(&decrypted_payload)?;

        // ============================================================================
        // 7. VALIDATION: reference_hash must match payload
        // ============================================================================
        let reference_hash_from_payload: [u8; REFERENCE_HASH_LENGTH] =
            payload.reference_hash.as_slice().try_into().map_err(|_| {
                SqliteError::Io("Invalid reference_hash length in payload".to_string())
            })?;

        // Debug assertion - detects bugs in derivation logic
        debug_assert_eq!(
            &reference_hash_from_payload, reference_hash,
            "Reference hash mismatch - impossible condition"
        );

        if &reference_hash_from_payload != reference_hash {
            return Err(SqliteError::Io(
                "CRITICAL: Reference hash mismatch - data corruption detected".to_string(),
            ));
        }

        // ============================================================================
        // 8. GET PENDING_READS from tracking
        // ============================================================================
        let pending_reads =
            SharedSecretStorage::get_pending_reads_from_tracking(&reference_hash_from_payload)?
                .unwrap_or(0);

        debug!(
            "✅ SharedSecret: Read secret successfully (role={}, pending_reads={})",
            role.to_str(),
            pending_reads
        );

        Ok((payload, pending_reads, expires_at, role))
    }

    // Read a secret, decrypt, and get pending_reads from tracking (OLD - deprecated)
    // OBSOLETE: Not compatible with v3 (centralized payload architecture)
    // Use read_secret() instead
    /*
    #[allow(dead_code)]
    pub fn read_secret_old(
        id: &[u8; ENCRYPTED_ID_LENGTH],
    ) -> Result<(SharedSecretPayload, i64, i64, SecretRole), SqliteError> {
        // This function is obsolete and not compatible with v3
        Err(SqliteError::Io("read_secret_old() is obsolete - use read_secret() instead".to_string()))
    }
    */

    /// Validate OTP against stored OTP in payload
    ///
    /// # Arguments
    /// * `stored_otp` - Optional OTP from payload
    /// * `provided_otp` - Optional OTP provided by user
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if valid, false/error otherwise
    #[allow(dead_code)]
    pub fn validate_otp(
        stored_otp: &Option<String>,
        provided_otp: &Option<String>,
    ) -> Result<bool, SqliteError> {
        match (stored_otp, provided_otp) {
            (Some(stored), Some(provided)) => {
                if stored == provided {
                    debug!("✅ SharedSecret: OTP validated successfully");
                    Ok(true)
                } else {
                    warn!("❌ SharedSecret: Invalid OTP");
                    Ok(false)
                }
            }
            (Some(_), None) => {
                warn!("⚠️  SharedSecret: OTP required but not provided");
                Err(SqliteError::Io("OTP required".to_string()))
            }
            (None, _) => {
                debug!("ℹ️  SharedSecret: No OTP required");
                Ok(true)
            }
        }
    }

    /// Confirm read by updating tracking record
    ///
    /// # Arguments
    /// * `reference_hash` - Reference hash (16 bytes)
    ///
    /// # Returns
    /// * `Result<bool, SqliteError>` - true if updated, false if already set
    pub fn confirm_read(reference_hash: &[u8; REFERENCE_HASH_LENGTH]) -> Result<bool, SqliteError> {
        SharedSecretStorage::update_tracking_read(reference_hash)
    }

    /// Clean up expired secrets and tracking
    #[allow(dead_code)]
    pub fn cleanup_expired() -> Result<(u32, u32), SqliteError> {
        SharedSecretStorage::cleanup_expired()
    }
}
