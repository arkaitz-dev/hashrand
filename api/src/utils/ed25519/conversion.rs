//! Ed25519 Conversion Utilities
//!
//! DRY-unified hex and base58 encoding/decoding with validation

use hex;

/// Validate hex string length
///
/// # Arguments
/// * `hex_str` - Hex string to validate
/// * `expected_len` - Expected length in characters
/// * `name` - Name for error messages
///
/// # Returns
/// * `Result<(), String>` - Ok if valid, error message otherwise
fn validate_hex_length(hex_str: &str, expected_len: usize, name: &str) -> Result<(), String> {
    if hex_str.len() != expected_len {
        return Err(format!(
            "Invalid {} hex length: {} (expected {})",
            name,
            hex_str.len(),
            expected_len
        ));
    }
    Ok(())
}

/// Decode hex string to bytes with validation
///
/// # Arguments
/// * `hex_str` - Hex string to decode
/// * `expected_byte_len` - Expected byte length
/// * `name` - Name for error messages
///
/// # Returns
/// * `Result<Vec<u8>, String>` - Decoded bytes or error message
fn decode_hex_with_validation(
    hex_str: &str,
    expected_byte_len: usize,
    name: &str,
) -> Result<Vec<u8>, String> {
    let bytes =
        hex::decode(hex_str).map_err(|e| format!("Failed to decode {} hex: {}", name, e))?;

    if bytes.len() != expected_byte_len {
        return Err(format!(
            "Invalid {} byte length: {} (expected {})",
            name,
            bytes.len(),
            expected_byte_len
        ));
    }

    Ok(bytes)
}

/// Convert public key bytes to hex string
///
/// # Arguments
/// * `public_key_bytes` - Ed25519 public key as 32 bytes
///
/// # Returns
/// * `String` - Hex encoded public key (64 chars)
#[allow(dead_code)]
pub fn public_key_to_hex(public_key_bytes: &[u8; 32]) -> String {
    hex::encode(public_key_bytes)
}

/// Convert hex string to public key bytes
///
/// # Arguments
/// * `public_key_hex` - Ed25519 public key as hex string (64 chars)
///
/// # Returns
/// * `Result<[u8; 32], String>` - Public key bytes or error
#[allow(dead_code)]
pub fn public_key_from_hex(public_key_hex: &str) -> Result<[u8; 32], String> {
    // Validate hex length (64 chars = 32 bytes)
    validate_hex_length(public_key_hex, 64, "public key")?;

    // Decode and validate byte length
    let bytes = decode_hex_with_validation(public_key_hex, 32, "public key")?;

    // Convert to fixed-size array
    let mut public_key_bytes = [0u8; 32];
    public_key_bytes.copy_from_slice(&bytes);
    Ok(public_key_bytes)
}

/// Decode and validate public key hex for verification
///
/// # Arguments
/// * `public_key_hex` - Ed25519 public key as hex string (64 chars)
///
/// # Returns
/// * `Result<[u8; 32], String>` - Public key bytes or error
pub(super) fn decode_public_key(public_key_hex: &str) -> Result<[u8; 32], String> {
    validate_hex_length(public_key_hex, 64, "public key")?;
    let bytes = decode_hex_with_validation(public_key_hex, 32, "public key")?;

    let mut public_key_bytes = [0u8; 32];
    public_key_bytes.copy_from_slice(&bytes);
    Ok(public_key_bytes)
}

/// Decode base58 string to bytes with validation
///
/// # Arguments
/// * `base58_str` - Base58 string to decode
/// * `expected_byte_len` - Expected byte length
/// * `name` - Name for error messages
///
/// # Returns
/// * `Result<Vec<u8>, String>` - Decoded bytes or error message
fn decode_base58_with_validation(
    base58_str: &str,
    expected_byte_len: usize,
    name: &str,
) -> Result<Vec<u8>, String> {
    let bytes = bs58::decode(base58_str)
        .into_vec()
        .map_err(|e| format!("Failed to decode {} base58: {}", name, e))?;

    if bytes.len() != expected_byte_len {
        return Err(format!(
            "Invalid {} byte length after base58 decode: {} (expected {})",
            name,
            bytes.len(),
            expected_byte_len
        ));
    }

    Ok(bytes)
}

/// Decode and validate signature base58 for verification
///
/// # Arguments
/// * `signature_base58` - Ed25519 signature as base58 string (~88 chars)
///
/// # Returns
/// * `Result<[u8; 64], String>` - Signature bytes or error
pub(super) fn decode_signature(signature_base58: &str) -> Result<[u8; 64], String> {
    // validate_hex_length(signature_hex, 128, "signature")?;
    // let bytes = decode_hex_with_validation(signature_hex, 64, "signature")?;
    let bytes = decode_base58_with_validation(signature_base58, 64, "signature")?;

    let mut signature_bytes = [0u8; 64];
    signature_bytes.copy_from_slice(&bytes);
    Ok(signature_bytes)
}

/// Validate Ed25519 signature data format without decoding
///
/// # Arguments
/// * `public_key_hex` - Ed25519 public key as hex string
/// * `signature_base58` - Ed25519 signature as base58 string
///
/// # Returns
/// * `Result<(), String>` - Ok if format is valid, error message otherwise
pub fn validate_signature_data_format(
    public_key_hex: &str,
    signature_base58: &str,
) -> Result<(), String> {
    // Validate public key format (still hex)
    validate_hex_length(public_key_hex, 64, "public key")?;
    hex::decode(public_key_hex).map_err(|_| "Invalid public key hex format".to_string())?;

    // Validate signature format (now base58)
    // validate_hex_length(signature_hex, 128, "signature")?;
    // hex::decode(signature_hex).map_err(|_| "Invalid signature hex format".to_string())?;
    let decoded = bs58::decode(signature_base58)
        .into_vec()
        .map_err(|_| "Invalid signature base58 format".to_string())?;

    if decoded.len() != 64 {
        return Err(format!(
            "Invalid signature byte length: {} (expected 64)",
            decoded.len()
        ));
    }

    Ok(())
}
