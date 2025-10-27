//! Sender operations for shared secrets
//!
//! Handles shared secret creation workflow (sender operations).

use super::super::shared_secret_crypto::SharedSecretCrypto;
use super::super::shared_secret_storage::SharedSecretStorage;
use super::super::shared_secret_types::{SecretRole, constants::*};
use crate::utils::crypto::{decrypt_with_ecdh, get_backend_x25519_private_key};
use chrono::Utc;
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;

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
        return Err(SqliteError::Io(
            "Encrypted secret cannot be empty".to_string(),
        ));
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
/// * `sender_ed25519_public_key_hex` - Sender's Ed25519 public key as hex string (64 chars)
/// * `sender_x25519_public_key_hex` - Sender's X25519 public key as hex string (64 chars)
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
    _sender_ed25519_public_key_hex: &str,
    sender_x25519_public_key_hex: &str,
    otp: Option<String>,
    expires_hours: i64,
    max_reads: i64,
    sender_db_index: &[u8; 32],
    receiver_db_index: &[u8; 32],
    reference_hash: &[u8; REFERENCE_HASH_LENGTH],
) -> Result<[u8; REFERENCE_HASH_LENGTH], SqliteError> {
    debug!("🔐 SharedSecret: Starting E2E encryption workflow");

    // 1. Validate sender X25519 public key format
    if sender_x25519_public_key_hex.len() != 64 {
        return Err(SqliteError::Io(format!(
            "Invalid sender X25519 public key hex length: {} (expected 64)",
            sender_x25519_public_key_hex.len()
        )));
    }

    let sender_x25519_public_bytes = hex::decode(sender_x25519_public_key_hex).map_err(|e| {
        SqliteError::Io(format!(
            "Failed to decode sender X25519 public key hex: {}",
            e
        ))
    })?;

    if sender_x25519_public_bytes.len() != 32 {
        return Err(SqliteError::Io(format!(
            "Invalid sender X25519 public key byte length: {} (expected 32)",
            sender_x25519_public_bytes.len()
        )));
    }

    let sender_x25519_public_array: [u8; 32] =
        sender_x25519_public_bytes.try_into().map_err(|_| {
            SqliteError::Io("Failed to convert sender X25519 public key to array".to_string())
        })?;

    // Convert to X25519PublicKey type
    let sender_x25519_public = x25519_dalek::PublicKey::from(sender_x25519_public_array);

    // 2. Calculate sender's user_id for per-user X25519 key derivation
    debug!("🔐 SharedSecret: Calculating sender user_id");
    let sender_user_id = SharedSecretCrypto::calculate_user_id(sender_email)?;

    // 3. Get backend's per-user X25519 private key
    // CRITICAL: Use sender's X25519 pub_key (not Ed25519!) for per-user derivation
    debug!("🔐 SharedSecret: Deriving backend X25519 private key (per-user)");
    let backend_x25519_private =
        get_backend_x25519_private_key(&sender_user_id, sender_x25519_public_key_hex)?;

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
    create_secret_pair(
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
