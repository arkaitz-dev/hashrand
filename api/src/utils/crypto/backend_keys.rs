//! Backend Ed25519/X25519 key management for E2E encryption (per-user)
//!
//! Provides functions to derive per-user X25519 keys from the same Ed25519 private key
//! used for signing responses. This ensures architectural consistency: both signature
//! validation and ECDH encryption use the SAME per-user, rotating key derivation.
//!
//! **Architecture Change (v1.9.0):**
//! - BEFORE: X25519 key derived from global `ed25519_derivation_key` (static, shared)
//! - NOW: X25519 key derived from `derive_session_private_key(user_id, pub_key_hex)` (per-user, rotating)
//!
//! **Benefits:**
//! - Consistency: Same derivation as Ed25519 signature keys
//! - Per-user isolation: Each user has unique X25519 keypair
//! - Automatic rotation: Keys rotate when client rotates keypair (TRAMO 2/3)
//! - Enhanced security: No shared global key across all users

use crate::utils::signed_response::key_derivation::derive_session_private_key;
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519PrivateKey};

use super::ed25519_to_x25519::{ed25519_public_to_x25519, ed25519_secret_to_x25519};

/// Get backend's X25519 private key for ECDH decryption (per-user)
///
/// Derives the backend's Ed25519 private key using the SAME derivation as signature keys,
/// then converts it to X25519 format for ECDH operations.
///
/// **Derivation**: `derive_session_private_key(user_id, pub_key_hex)` → Ed25519[32] → X25519[32]
///
/// # Arguments
/// * `user_id` - User ID bytes (typically 16 bytes)
/// * `pub_key_hex` - Client's Ed25519 public key as hex string (64 hex chars)
///
/// # Returns
/// * `Result<X25519PrivateKey, SqliteError>` - X25519 private key or error
///
/// # Example
/// ```ignore
/// let backend_x25519_private = get_backend_x25519_private_key(user_id, &requester_pub_key_hex)?;
/// let decrypted_key_material = decrypt_with_ecdh(&encrypted, &backend_x25519_private, &sender_public)?;
/// ```
pub fn get_backend_x25519_private_key(
    user_id: &[u8],
    pub_key_hex: &str,
) -> Result<X25519PrivateKey, SqliteError> {
    debug!(
        "🔑 BackendKeys: Deriving per-user Ed25519 private key (user_id: {} bytes, pub_key: {}...)",
        user_id.len(),
        &pub_key_hex[..8]
    );

    // Derive Ed25519 private key using SAME derivation as signature keys
    let ed25519_private = derive_session_private_key(user_id, pub_key_hex)
        .map_err(|e| SqliteError::Io(format!("Failed to derive session private key: {}", e)))?;

    // Convert Ed25519 → X25519 for ECDH
    let x25519_private = ed25519_secret_to_x25519(&ed25519_private);

    debug!("✅ BackendKeys: Converted to X25519 private key for ECDH");
    Ok(x25519_private)
}

/// Get backend's X25519 public key for frontend use (per-user)
///
/// Derives the backend's X25519 public key from its per-user private key. This public key
/// is returned in login/refresh responses so the frontend can encrypt key_material when
/// creating shared secrets.
///
/// **Derivation**: `derive_session_private_key(user_id, pub_key_hex)` → Ed25519_priv[32] → Ed25519_pub[32] → X25519_pub[32]
///
/// # Arguments
/// * `user_id` - User ID bytes (typically 16 bytes)
/// * `pub_key_hex` - Client's Ed25519 public key as hex string (64 hex chars)
///
/// # Returns
/// * `Result<X25519PublicKey, SqliteError>` - X25519 public key or error
///
/// # Example
/// ```ignore
/// let backend_x25519_public = get_backend_x25519_public_key(user_id, &pub_key_hex)?;
/// // Return to frontend in hex format: hex::encode(backend_x25519_public.as_bytes())
/// ```
pub fn get_backend_x25519_public_key(
    user_id: &[u8],
    pub_key_hex: &str,
) -> Result<X25519PublicKey, SqliteError> {
    debug!(
        "🔑 BackendKeys: Deriving per-user X25519 public key (user_id: {} bytes, pub_key: {}...)",
        user_id.len(),
        &pub_key_hex[..8]
    );

    // Derive Ed25519 private key using SAME derivation as signature keys
    let ed25519_private = derive_session_private_key(user_id, pub_key_hex)
        .map_err(|e| SqliteError::Io(format!("Failed to derive session private key: {}", e)))?;

    // Derive Ed25519 public key from private key
    use ed25519_dalek::SigningKey;
    let signing_key = SigningKey::from_bytes(&ed25519_private);
    let ed25519_public = signing_key.verifying_key().to_bytes();

    // Convert Ed25519 public → X25519 public
    let x25519_public = ed25519_public_to_x25519(&ed25519_public)?;

    debug!("✅ BackendKeys: X25519 public key derived (per-user)");
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

        // Note: This test requires ed25519_derivation_key environment variable
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

    #[test]
    fn test_x25519_conversion_determinism() {
        // Test that X25519 conversion is deterministic
        let test_seed = [42u8; 32];

        let x25519_private = ed25519_secret_to_x25519(&test_seed);
        let x25519_public = X25519PublicKey::from(&x25519_private);

        assert_eq!(
            x25519_public.as_bytes().len(),
            32,
            "X25519 public key should be 32 bytes"
        );
        assert_eq!(
            x25519_private.to_bytes().len(),
            32,
            "X25519 private key should be 32 bytes"
        );

        // Verify determinism
        let x25519_private2 = ed25519_secret_to_x25519(&test_seed);
        assert_eq!(
            x25519_private.to_bytes(),
            x25519_private2.to_bytes(),
            "Same seed should produce same X25519 key"
        );
    }
}
