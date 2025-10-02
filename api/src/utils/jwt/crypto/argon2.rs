use argon2::Argon2;
use base64::{Engine, engine::general_purpose};

/// Argon2id configuration constants
pub const ARGON2_MEM_COST: u32 = 19456;
pub const ARGON2_TIME_COST: u32 = 2;
pub const ARGON2_LANES: u32 = 1;
pub const ARGON2_HASH_LENGTH: usize = 32;

/// Hash input using Argon2id with fixed parameters
///
/// # Arguments
/// * `input` - Input bytes to hash
/// * `salt` - Salt bytes for hashing
///
/// # Returns
/// * `Result<[u8; 32], String>` - 32-byte hash or error
pub fn derive_with_argon2id(input: &[u8], salt: &[u8]) -> Result<[u8; 32], String> {
    use argon2::password_hash::{PasswordHasher, SaltString};

    // Convert salt to base64 format required by Argon2
    let salt_base64 = general_purpose::STANDARD_NO_PAD.encode(salt);
    let salt_string = SaltString::encode_b64(&salt_base64.as_bytes()[..16])
        .map_err(|e| format!("Failed to create salt string: {}", e))?;

    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(
            ARGON2_MEM_COST,
            ARGON2_TIME_COST,
            ARGON2_LANES,
            Some(ARGON2_HASH_LENGTH),
        )
        .map_err(|e| format!("Failed to create Argon2 params: {}", e))?,
    );

    let hash_result = argon2
        .hash_password(input, &salt_string)
        .map_err(|e| format!("Argon2id hashing failed: {}", e))?;

    let hash_string = hash_result.to_string();

    // Extract hash part after last '$' and decode from base64
    let hash_parts: Vec<&str> = hash_string.split('$').collect();
    if hash_parts.len() < 6 {
        return Err("Invalid Argon2id hash format".to_string());
    }

    let base64_hash = hash_parts[hash_parts.len() - 1];

    // Decode base64 to get raw bytes (Argon2 uses base64 without padding)
    let decoded_hash = general_purpose::STANDARD_NO_PAD
        .decode(base64_hash)
        .map_err(|e| format!("Base64 decode failed: {}", e))?;

    // Convert to [u8; 32]
    validate_byte_length(&decoded_hash, 32)?;

    let mut final_result = [0u8; 32];
    copy_bytes_to_array(&mut final_result, &decoded_hash);

    Ok(final_result)
}

/// Validate byte slice length (DRY utility)
fn validate_byte_length(data: &[u8], expected: usize) -> Result<(), String> {
    if data.len() != expected {
        return Err(format!("Expected {} bytes, got {}", expected, data.len()));
    }
    Ok(())
}

/// Copy bytes to fixed-size array (DRY utility)
fn copy_bytes_to_array<const N: usize>(dest: &mut [u8; N], src: &[u8]) {
    dest.copy_from_slice(src);
}
