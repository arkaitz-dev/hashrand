///! Payload serialization/deserialization for shared secrets
///!
///! Handles binary payload format parsing.

use super::super::shared_secret_types::{SharedSecretPayload, constants::*};
use spin_sdk::sqlite::Error as SqliteError;

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
