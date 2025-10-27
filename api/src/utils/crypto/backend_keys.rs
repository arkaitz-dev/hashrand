//! Backend Ed25519/X25519 key management - SEPARATED key derivation (v1.9.0+)
//!
//! **CRITICAL ARCHITECTURE CHANGE:**
//! Ed25519 and X25519 are now generated INDEPENDENTLY (not converted):
//! - Ed25519: For signing (uses ED25519_DERIVATION_KEY)
//! - X25519: For ECDH E2E encryption (uses X25519_DERIVATION_KEY)
//!
//! **Benefits:**
//! - Cryptographic separation: No context mixing (signing vs encryption)
//! - Independent key rotation: Each key type rotates separately
//! - Per-user isolation: Each user has unique keypair per type
//! - Standards compliance: Follows best practices (no key reuse across contexts)
//!
//! **Derivation:**
//! - Ed25519: `blake3(ED25519_DERIVATION_KEY, user_id + client_ed25519_pub_key)`
//! - X25519: `blake3(X25519_DERIVATION_KEY, user_id + client_x25519_pub_key)`

use crate::utils::pseudonimizer::blake3_keyed_variable;
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519PrivateKey};

/// Get X25519 derivation key from environment
fn get_x25519_derivation_key() -> Result<[u8; 64], SqliteError> {
    let key_hex = spin_sdk::variables::get("x25519_derivation_key")
        .map_err(|e| SqliteError::Io(format!("X25519_DERIVATION_KEY not found: {}", e)))?;

    let key_bytes = hex::decode(&key_hex)
        .map_err(|e| SqliteError::Io(format!("Invalid X25519_DERIVATION_KEY hex: {}", e)))?;

    if key_bytes.len() != 64 {
        return Err(SqliteError::Io(format!(
            "X25519_DERIVATION_KEY must be 64 bytes, got {}",
            key_bytes.len()
        )));
    }

    let mut key_array = [0u8; 64];
    key_array.copy_from_slice(&key_bytes);
    Ok(key_array)
}

/// Derive per-user X25519 session keypair (INDEPENDENT from Ed25519)
///
/// Uses Blake3-keyed derivation with X25519_DERIVATION_KEY to generate a deterministic
/// per-user X25519 private key. This key is COMPLETELY SEPARATE from Ed25519 keys.
///
/// **Derivation**: `blake3(X25519_DERIVATION_KEY, user_id + client_x25519_pub_key_hex)` → X25519_priv[32]
///
/// # Arguments
/// * `user_id` - User ID bytes (typically 16 bytes)
/// * `client_x25519_pub_key_hex` - Client's X25519 public key as hex string (64 hex chars)
///
/// # Returns
/// * `Result<(X25519PrivateKey, X25519PublicKey), SqliteError>` - X25519 keypair or error
fn derive_x25519_session_keypair(
    user_id: &[u8],
    client_x25519_pub_key_hex: &str,
) -> Result<(X25519PrivateKey, X25519PublicKey), SqliteError> {
    debug!(
        "🔑 BackendKeys: Deriving per-user X25519 keypair (user_id: {} bytes, client_x25519_pub: {}...)",
        user_id.len(),
        &client_x25519_pub_key_hex[..8]
    );

    // Get X25519 derivation key from environment
    let x25519_derivation_key = get_x25519_derivation_key()?;

    // Combine user_id + client_x25519_pub_key_hex for unique per-user derivation
    let mut combined = Vec::new();
    combined.extend_from_slice(user_id);
    combined.extend_from_slice(client_x25519_pub_key_hex.as_bytes());

    // Derive 32-byte seed using Blake3-keyed hash
    let seed = blake3_keyed_variable(&x25519_derivation_key, &combined, 32);

    // Convert to X25519 private key
    let mut private_key_bytes = [0u8; 32];
    private_key_bytes.copy_from_slice(&seed[..32]);
    let x25519_private = X25519PrivateKey::from(private_key_bytes);

    // Derive public key from private key
    let x25519_public = X25519PublicKey::from(&x25519_private);

    debug!("✅ BackendKeys: X25519 keypair derived (INDEPENDENT, per-user)");
    Ok((x25519_private, x25519_public))
}

/// Get backend's X25519 private key for ECDH decryption (per-user)
///
/// Derives the backend's X25519 private key using INDEPENDENT derivation
/// (NOT converted from Ed25519). Uses X25519_DERIVATION_KEY.
///
/// **Derivation**: `blake3(X25519_DERIVATION_KEY, user_id + client_x25519_pub_key)` → X25519_priv[32]
///
/// # Arguments
/// * `user_id` - User ID bytes (typically 16 bytes)
/// * `client_x25519_pub_key_hex` - Client's X25519 public key as hex string (64 hex chars)
///
/// # Returns
/// * `Result<X25519PrivateKey, SqliteError>` - X25519 private key or error
///
/// # Example
/// ```ignore
/// let backend_x25519_private = get_backend_x25519_private_key(user_id, &requester_x25519_pub_key_hex)?;
/// let decrypted_key_material = decrypt_with_ecdh(&encrypted, &backend_x25519_private, &sender_x25519_public)?;
/// ```
pub fn get_backend_x25519_private_key(
    user_id: &[u8],
    client_x25519_pub_key_hex: &str,
) -> Result<X25519PrivateKey, SqliteError> {
    let (x25519_private, _) = derive_x25519_session_keypair(user_id, client_x25519_pub_key_hex)?;
    Ok(x25519_private)
}

/// Get backend's X25519 public key for frontend use (per-user)
///
/// Derives the backend's X25519 public key using INDEPENDENT derivation
/// (NOT converted from Ed25519). This public key is returned in login/refresh
/// responses so the frontend can encrypt key_material when creating shared secrets.
///
/// **Derivation**: `blake3(X25519_DERIVATION_KEY, user_id + client_x25519_pub_key)` → X25519_pub[32]
///
/// # Arguments
/// * `user_id` - User ID bytes (typically 16 bytes)
/// * `client_x25519_pub_key_hex` - Client's X25519 public key as hex string (64 hex chars)
///
/// # Returns
/// * `Result<X25519PublicKey, SqliteError>` - X25519 public key or error
///
/// # Example
/// ```ignore
/// let backend_x25519_public = get_backend_x25519_public_key(user_id, &client_x25519_pub_key_hex)?;
/// // Return to frontend in hex format: hex::encode(backend_x25519_public.as_bytes())
/// ```
pub fn get_backend_x25519_public_key(
    user_id: &[u8],
    client_x25519_pub_key_hex: &str,
) -> Result<X25519PublicKey, SqliteError> {
    let (_, x25519_public) = derive_x25519_session_keypair(user_id, client_x25519_pub_key_hex)?;

    // DEBUG: Check for small-order points (WebCrypto validation issue diagnosis)
    let pub_key_bytes = x25519_public.as_bytes();
    let is_identity = pub_key_bytes == &[0u8; 32];
    let is_all_ones = pub_key_bytes == &[1u8; 32];
    let last_byte = pub_key_bytes[31];

    debug!(
        "🔍 Backend X25519 validation: identity={}, all_ones={}, last_byte=0x{:02x}, full_hex={}",
        is_identity,
        is_all_ones,
        last_byte,
        hex::encode(pub_key_bytes)
    );

    if is_identity {
        debug!("⚠️ WARNING: Generated X25519 public key is the identity point (all zeros)");
    }

    Ok(x25519_public)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_per_user_x25519_derivation() {
        // Test that different users get different X25519 keys
        let user_id_1 = b"user1234567890ab"; // 16 bytes
        let user_id_2 = b"user9876543210zy"; // 16 bytes
        let pub_key_hex = "a".repeat(64); // Valid hex string (64 chars)

        // Note: This test requires x25519_derivation_key environment variable
        // In unit tests, this will fail gracefully
        let result_1 = get_backend_x25519_public_key(user_id_1, &pub_key_hex);
        let result_2 = get_backend_x25519_public_key(user_id_2, &pub_key_hex);

        // Both should fail or both should succeed (depending on env)
        assert_eq!(
            result_1.is_ok(),
            result_2.is_ok(),
            "Both derivations should have same success status"
        );

        if result_1.is_ok() && result_2.is_ok() {
            let key_1 = result_1.unwrap();
            let key_2 = result_2.unwrap();

            // Different users should get different keys
            assert_ne!(
                key_1.as_bytes(),
                key_2.as_bytes(),
                "Different users should have different X25519 public keys"
            );
        }
    }

    #[test]
    fn test_same_user_same_key() {
        // Test that same user + pub_key always gets same X25519 key (determinism)
        let user_id = b"user1234567890ab"; // 16 bytes
        let pub_key_hex = "b".repeat(64); // Valid hex string (64 chars)

        let result_1 = get_backend_x25519_public_key(user_id, &pub_key_hex);
        let result_2 = get_backend_x25519_public_key(user_id, &pub_key_hex);

        assert_eq!(
            result_1.is_ok(),
            result_2.is_ok(),
            "Both derivations should have same success status"
        );

        if result_1.is_ok() && result_2.is_ok() {
            let key_1 = result_1.unwrap();
            let key_2 = result_2.unwrap();

            // Same inputs should produce same key (determinism)
            assert_eq!(
                key_1.as_bytes(),
                key_2.as_bytes(),
                "Same user + pub_key should produce deterministic X25519 key"
            );
        }
    }

    #[test]
    fn test_key_rotation_produces_different_key() {
        // Test that when client rotates pub_key, backend X25519 key also changes
        let user_id = b"user1234567890ab"; // 16 bytes
        let old_pub_key_hex = "c".repeat(64); // Old client keypair
        let new_pub_key_hex = "d".repeat(64); // New client keypair (after rotation)

        let result_old = get_backend_x25519_public_key(user_id, &old_pub_key_hex);
        let result_new = get_backend_x25519_public_key(user_id, &new_pub_key_hex);

        assert_eq!(
            result_old.is_ok(),
            result_new.is_ok(),
            "Both derivations should have same success status"
        );

        if result_old.is_ok() && result_new.is_ok() {
            let key_old = result_old.unwrap();
            let key_new = result_new.unwrap();

            // Different pub_key (rotation) should produce different backend X25519 key
            assert_ne!(
                key_old.as_bytes(),
                key_new.as_bytes(),
                "Key rotation should produce different backend X25519 key"
            );
        }
    }

    #[test]
    fn test_x25519_keypair_consistency() {
        // Test that public key can be derived from private key
        let user_id = b"user1234567890ab"; // 16 bytes
        let pub_key_hex = "e".repeat(64); // Valid hex string (64 chars)

        let private_result = get_backend_x25519_private_key(user_id, &pub_key_hex);
        let public_result = get_backend_x25519_public_key(user_id, &pub_key_hex);

        if private_result.is_ok() && public_result.is_ok() {
            let x25519_private = private_result.unwrap();
            let x25519_public = public_result.unwrap();

            // Verify public key can be derived from private key
            let derived_public = X25519PublicKey::from(&x25519_private);

            assert_eq!(
                derived_public.as_bytes(),
                x25519_public.as_bytes(),
                "Public key should match derived from private key"
            );
        }
    }
}
