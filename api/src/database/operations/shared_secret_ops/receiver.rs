///! Receiver operations for shared secrets
///!
///! Handles shared secret retrieval and validation (receiver operations).

use super::super::shared_secret_crypto::SharedSecretCrypto;
use super::super::shared_secret_storage::SharedSecretStorage;
use super::super::shared_secret_types::{SecretRole, SharedSecretPayload, constants::*};
use super::payload::deserialize_payload;
use chrono::Utc;
use spin_sdk::sqlite::Error as SqliteError;
use tracing::{debug, warn};

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
    let payload = deserialize_payload(&decrypted_payload)?;

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
