use super::super::magic_link_types::ValidationResult;
use super::super::magic_link_types::constants::*;
use super::utilities::{copy_to_array, create_validation_error, extract_utf8_string};
use tracing::{info, warn, error, debug};

/// Type alias for extracted payload components (encryption_blob, pub_key, ui_host, next_param)
type PayloadComponents = ([u8; 44], [u8; 32], Option<String>, Option<String>);

/// Extract encryption blob, Ed25519 public key, ui_host, and next_param from decrypted payload
///
/// # Arguments
/// * `payload_plain` - Decrypted payload bytes
///
/// # Returns
/// * `Result<PayloadComponents, ValidationResult>`
///   - (encryption_blob, pub_key, ui_host, next_param) or validation error
pub fn extract_payload_components(
    payload_plain: &[u8],
) -> Result<PayloadComponents, ValidationResult> {
    // Validate minimum payload length
    if payload_plain.len() < MIN_PAYLOAD_LENGTH {
        // println!("Database: Invalid decrypted payload length (minimum 76 bytes)");
        error!("Database: Invalid decrypted payload length (minimum 76 bytes)");
        return Err(create_validation_error());
    }

    // Extract encryption_blob (first 44 bytes)
    let mut encryption_blob = [0u8; ENCRYPTION_BLOB_LENGTH];
    copy_to_array(
        &mut encryption_blob,
        &payload_plain[..ENCRYPTION_BLOB_LENGTH],
    );

    // Extract stored pub_key (next 32 bytes)
    let stored_pub_key_bytes = &payload_plain[ENCRYPTION_BLOB_LENGTH..MIN_PAYLOAD_LENGTH];
    let mut pub_key_array = [0u8; ED25519_BYTES_LENGTH];
    copy_to_array(&mut pub_key_array, stored_pub_key_bytes);

    // println!("Database: Successfully extracted Ed25519 public key from stored payload");
    // println!(
    //     "🔍 DEBUG EXTRACT: payload_plain.len() = {}",
    //     payload_plain.len()
    // );
    info!("Database: Successfully extracted Ed25519 public key from stored payload");
    debug!(
        "🔍 DEBUG EXTRACT: payload_plain.len() = {}",
        payload_plain.len()
    );

    // Extract ui_host and next_param
    let (ui_host, next_param) = extract_ui_host_and_next_param(payload_plain)?;

    // println!(
    //     "🔍 DEBUG EXTRACT: Final ui_host: {:?}, next_param: {:?}",
    //     ui_host, next_param
    // );
    debug!(
        "🔍 DEBUG EXTRACT: Final ui_host: {:?}, next_param: {:?}",
        ui_host, next_param
    );

    Ok((encryption_blob, pub_key_array, ui_host, next_param))
}

/// Extract ui_host and next_param with backward compatibility
///
/// Handles both old format (no ui_host) and new format (with ui_host length prefix)
fn extract_ui_host_and_next_param(
    payload_plain: &[u8],
) -> Result<(Option<String>, Option<String>), ValidationResult> {
    // Check if we have the new format with ui_host
    if payload_plain.len() >= MIN_PAYLOAD_LENGTH + 2 {
        extract_new_format(payload_plain)
    } else {
        extract_old_format(payload_plain)
    }
}

/// Extract ui_host and next_param from new format (with ui_host length prefix)
fn extract_new_format(
    payload_plain: &[u8],
) -> Result<(Option<String>, Option<String>), ValidationResult> {
    // Extract ui_host_len
    let ui_host_len_bytes = &payload_plain[MIN_PAYLOAD_LENGTH..MIN_PAYLOAD_LENGTH + 2];
    let ui_host_len = u16::from_be_bytes([ui_host_len_bytes[0], ui_host_len_bytes[1]]) as usize;

    // println!(
    //     "🔍 DEBUG EXTRACT: Detected new format with ui_host_len: {}",
    //     ui_host_len
    // );
    debug!(
        "🔍 DEBUG EXTRACT: Detected new format with ui_host_len: {}",
        ui_host_len
    );

    // Verify we have enough bytes for ui_host
    if payload_plain.len() < MIN_PAYLOAD_LENGTH + 2 + ui_host_len {
        // println!("❌ Database: Insufficient bytes for ui_host extraction");
        error!("❌ Database: Insufficient bytes for ui_host extraction");
        return Err(create_validation_error());
    }

    let ui_host_start = MIN_PAYLOAD_LENGTH + 2;
    let ui_host_end = ui_host_start + ui_host_len;
    let next_param_start = ui_host_end;

    // Extract ui_host
    let ui_host_str = extract_utf8_string(&payload_plain[ui_host_start..ui_host_end], "ui_host")?;
    // println!(
    //     "🔒 [SECURITY] Extracted ui_host from blob: '{}'",
    //     ui_host_str
    // );
    info!(
        "🔒 [SECURITY] Extracted ui_host from blob: '{}'",
        ui_host_str
    );

    // Extract next_param (remaining bytes)
    let next_param_opt = if payload_plain.len() > next_param_start {
        Some(extract_utf8_string(
            &payload_plain[next_param_start..],
            "next_param",
        )?)
    } else {
        // println!("🔍 DEBUG EXTRACT: No next_param in new format");
        debug!("🔍 DEBUG EXTRACT: No next_param in new format");
        None
    };

    Ok((Some(ui_host_str), next_param_opt))
}

/// Extract next_param from old format (backward compatibility, no ui_host)
fn extract_old_format(
    payload_plain: &[u8],
) -> Result<(Option<String>, Option<String>), ValidationResult> {
    // println!("⚠️ DEBUG EXTRACT: Old format detected (no ui_host) - backward compatibility mode");
    warn!("⚠️ DEBUG EXTRACT: Old format detected (no ui_host) - backward compatibility mode");

    let next_param_opt = if payload_plain.len() > MIN_PAYLOAD_LENGTH {
        Some(extract_utf8_string(
            &payload_plain[MIN_PAYLOAD_LENGTH..],
            "next_param (old format)",
        )?)
    } else {
        // println!("🔍 DEBUG EXTRACT: No next_param (old format)");
        debug!("🔍 DEBUG EXTRACT: No next_param (old format)");
        None
    };

    Ok((None, next_param_opt)) // No ui_host in old format
}
