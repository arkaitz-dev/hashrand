//! Session key derivation for Ed25519 signatures

use spin_sdk::variables;

use super::errors::SignedResponseError;
use crate::utils::pseudonimizer::blake3_keyed_variable;

/// Derive per-session Ed25519 private key from user_id + pub_key
///
/// BLAKE3 PSEUDONIMIZER CRYPTOGRAPHIC DERIVATION:
/// 1. Concatenate: user_id_bytes + pub_key_bytes
/// 2. ED25519_DERIVATION_KEY[64] → Base58 → context (domain separation)
/// 3. combined_input → Blake3 hash → key_material[32 bytes]
/// 4. (context, key_material) → Blake3 KDF → deterministic_key[32 bytes]
/// 5. (combined_input, deterministic_key) → Blake3 keyed+XOF → Ed25519 private key[32 bytes]
///
/// # Arguments
/// * `user_id` - User ID bytes (typically 16 bytes)
/// * `pub_key_hex` - Frontend Ed25519 public key as hex string (64 hex chars)
///
/// # Returns
/// * `Result<[u8; 32], SignedResponseError>` - Ed25519 private key or error
pub fn derive_session_private_key(
    user_id: &[u8],
    pub_key_hex: &str,
) -> Result<[u8; 32], SignedResponseError> {
    // Validate pub_key format
    if pub_key_hex.len() != 64 {
        return Err(SignedResponseError::KeyDerivationError(format!(
            "Invalid pub_key hex length: {} (expected 64)",
            pub_key_hex.len()
        )));
    }

    // Decode pub_key from hex
    let pub_key_bytes = hex::decode(pub_key_hex).map_err(|e| {
        SignedResponseError::KeyDerivationError(format!("Failed to decode pub_key hex: {}", e))
    })?;

    if pub_key_bytes.len() != 32 {
        return Err(SignedResponseError::KeyDerivationError(format!(
            "Invalid pub_key byte length: {} (expected 32)",
            pub_key_bytes.len()
        )));
    }

    // Step 1: Concatenate user_id + pub_key_bytes
    let mut combined_input = Vec::with_capacity(user_id.len() + pub_key_bytes.len());
    combined_input.extend_from_slice(user_id);
    combined_input.extend_from_slice(&pub_key_bytes);

    println!(
        "= Deriving session key - user_id: {} bytes, pub_key: {} bytes",
        user_id.len(),
        pub_key_bytes.len()
    );

    // Get ED25519_DERIVATION_KEY[64 bytes] for Blake3 pseudonimizer
    let ed25519_derivation_key = get_ed25519_derivation_key()?;

    // Blake3 pseudonimizer pipeline → 32 bytes Ed25519 private key
    let private_key_vec = blake3_keyed_variable(&ed25519_derivation_key, &combined_input, 32);

    // Convert Vec<u8> to [u8; 32]
    let mut private_key = [0u8; 32];
    private_key.copy_from_slice(&private_key_vec);

    println!("= Session private key derived successfully");

    Ok(private_key)
}

/// Get Ed25519 derivation key from environment
///
/// # Returns
/// * `Result<[u8; 64], SignedResponseError>` - 64-byte derivation key or error
fn get_ed25519_derivation_key() -> Result<[u8; 64], SignedResponseError> {
    let key_hex = variables::get("ed25519_derivation_key").map_err(|e| {
        SignedResponseError::ConfigurationError(format!("ed25519_derivation_key not found: {}", e))
    })?;

    if key_hex.len() != 128 {
        return Err(SignedResponseError::ConfigurationError(format!(
            "Invalid ed25519_derivation_key length: {} (expected 128 hex chars)",
            key_hex.len()
        )));
    }

    let key_bytes = hex::decode(&key_hex).map_err(|e| {
        SignedResponseError::ConfigurationError(format!(
            "Failed to decode ed25519_derivation_key: {}",
            e
        ))
    })?;

    if key_bytes.len() != 64 {
        return Err(SignedResponseError::ConfigurationError(format!(
            "Invalid ed25519_derivation_key byte length: {} (expected 64)",
            key_bytes.len()
        )));
    }

    let mut derivation_key = [0u8; 64];
    derivation_key.copy_from_slice(&key_bytes);

    Ok(derivation_key)
}
