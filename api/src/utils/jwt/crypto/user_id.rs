use crate::utils::pseudonimizer::blake3_keyed_variable;

use super::super::config::{get_argon2_salt, get_user_id_argon2_compression, get_user_id_hmac_key};
use super::argon2::derive_with_argon2id;

/// Derive secure user ID with context (exposes argon2_output for db_index generation)
///
/// Complete pipeline:
/// 1. Blake3 XOF of email (64 bytes, no key)
/// 2. blake3_keyed_variable with 64-byte HMAC key (32 bytes)
/// 3. Generate dynamic salt (32 bytes)
/// 4. Argon2id with fixed parameters (32 bytes) ← EXPOSED
/// 5. Blake3 keyed variable compression to 16 bytes
///
/// # Arguments
/// * `email` - User email address
///
/// # Returns
/// * `Result<([u8; 16], [u8; 32]), String>` - (user_id[16], argon2_output[32]) or error
pub fn derive_user_id_with_context(email: &str) -> Result<([u8; 16], [u8; 32]), String> {
    // Step 1: Blake3 XOF of email (64 bytes, no key)
    let mut blake3_hasher = blake3::Hasher::new();
    blake3_hasher.update(email.to_lowercase().trim().as_bytes());
    let mut xof_reader = blake3_hasher.finalize_xof();
    let mut paso1_output = [0u8; 64];
    xof_reader.fill(&mut paso1_output);

    // Step 2: blake3_keyed_variable with 64-byte HMAC key
    let hmac_key = get_user_id_hmac_key()?;
    let paso2_output = blake3_keyed_variable(&hmac_key, &paso1_output, 32);

    let mut hmac_result = [0u8; 32];
    copy_bytes_to_array(&mut hmac_result, &paso2_output);

    // Step 3: Generate dynamic salt using Blake3 pseudonimizer
    let dynamic_salt = generate_dynamic_salt(&paso1_output)?;

    // Step 4: Argon2id with fixed parameters (using Blake3-keyed result as data input)
    let argon2_output = derive_with_argon2id(&hmac_result[..], &dynamic_salt)?;

    // Step 5: Blake3 keyed variable via pseudonimizer to compress to 16 bytes
    let compression_key = get_user_id_argon2_compression()?;
    let user_id_output = blake3_keyed_variable(&compression_key, &argon2_output, 16);

    let mut user_id = [0u8; 16];
    copy_bytes_to_array(&mut user_id, &user_id_output);

    Ok((user_id, argon2_output))
}

/// Derive secure user ID from email using Blake3 + Pseudonimizer + Argon2id
///
/// Wrapper for backwards compatibility - delegates to derive_user_id_with_context()
///
/// # Arguments
/// * `email` - User email address
///
/// # Returns
/// * `Result<[u8; 16], String>` - 16-byte user ID or error
pub fn derive_user_id(email: &str) -> Result<[u8; 16], String> {
    derive_user_id_with_context(email).map(|(user_id, _argon2_output)| user_id)
}

/// Generate dynamic salt using Blake3 pseudonimizer
///
/// Process: salt = Blake3-keyed-variable(ARGON2_SALT, data, 32)
///
/// # Arguments
/// * `data` - Input data to generate salt from
///
/// # Returns
/// * `Result<[u8; 32], String>` - 32-byte dynamic salt or error
pub fn generate_dynamic_salt(data: &[u8]) -> Result<[u8; 32], String> {
    let argon2_salt = get_argon2_salt()?;

    let salt_output = blake3_keyed_variable(&argon2_salt, data, 32);

    let mut dynamic_salt = [0u8; 32];
    copy_bytes_to_array(&mut dynamic_salt, &salt_output);

    Ok(dynamic_salt)
}

/// Convert user ID to Base58 username
///
/// # Arguments
/// * `user_id` - 16-byte user ID
///
/// # Returns
/// * `String` - Base58 encoded username
pub fn user_id_to_username(user_id: &[u8; 16]) -> String {
    bs58::encode(user_id).into_string()
}

/// Convert email to username (convenience method)
///
/// Combines derive_user_id + user_id_to_username
///
/// # Arguments
/// * `email` - User email address
///
/// # Returns
/// * `Result<String, String>` - Base58 username or error
pub fn email_to_username(email: &str) -> Result<String, String> {
    let user_id = derive_user_id(email)?;
    Ok(user_id_to_username(&user_id))
}

/// Copy bytes to fixed-size array (DRY utility)
fn copy_bytes_to_array<const N: usize>(dest: &mut [u8; N], src: &[u8]) {
    dest.copy_from_slice(src);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_user_id() {
        let email = "test@example.com";
        let user_id = derive_user_id(email).unwrap();
        assert_eq!(user_id.len(), 16);
    }

    #[test]
    fn test_user_id_to_username() {
        let user_id = [0u8; 16];
        let username = user_id_to_username(&user_id);
        assert!(!username.is_empty());
    }

    #[test]
    fn test_email_to_username() {
        let email = "test@example.com";
        let username = email_to_username(email).unwrap();
        assert!(!username.is_empty());
    }

    #[test]
    fn test_deterministic_user_id() {
        let email = "test@example.com";
        let user_id1 = derive_user_id(email).unwrap();
        let user_id2 = derive_user_id(email).unwrap();
        assert_eq!(
            user_id1, user_id2,
            "User ID derivation must be deterministic"
        );
    }
}
