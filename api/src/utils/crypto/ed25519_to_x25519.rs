//! Ed25519 to X25519 key conversion
//!
//! Converts Ed25519 keys (used for signatures) to X25519 keys (used for ECDH)
//! following RFC 7748 specification.
//!
//! This allows reusing existing Ed25519 keys for both signing (Ed25519) and
//! encryption (X25519 ECDH), following the approach used by Signal Protocol,
//! Tor, and Age encryption.

use curve25519_dalek::edwards::CompressedEdwardsY;
use ed25519_dalek::VerifyingKey as Ed25519PublicKey;
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519PrivateKey};

/// Convert Ed25519 public key to X25519 public key (Montgomery form)
///
/// This conversion is deterministic and follows RFC 7748. The resulting X25519
/// public key can be used for ECDH key agreement.
///
/// # Arguments
/// * `ed25519_pub` - Ed25519 public key (32 bytes)
///
/// # Returns
/// * `Result<X25519PublicKey, SqliteError>` - X25519 public key or error
///
/// # Example
/// ```ignore
/// let ed25519_pub = [0u8; 32]; // Your Ed25519 public key
/// let x25519_pub = ed25519_public_to_x25519(&ed25519_pub)?;
/// ```
pub fn ed25519_public_to_x25519(ed25519_pub: &[u8; 32]) -> Result<X25519PublicKey, SqliteError> {
    debug!("🔄 Converting Ed25519 public key to X25519");

    // Parse Ed25519 public key
    let ed_pub = Ed25519PublicKey::from_bytes(ed25519_pub)
        .map_err(|e| SqliteError::Io(format!("Invalid Ed25519 public key: {}", e)))?;

    // Convert to compressed Edwards Y point
    let compressed = CompressedEdwardsY(ed_pub.to_bytes());

    // Decompress to Edwards point
    let edwards_point = compressed
        .decompress()
        .ok_or_else(|| SqliteError::Io("Failed to decompress Ed25519 point".to_string()))?;

    // Convert to Montgomery u coordinate (X25519)
    let montgomery_u = edwards_point.to_montgomery().to_bytes();

    debug!("✅ Ed25519 → X25519 public key conversion successful");
    Ok(X25519PublicKey::from(montgomery_u))
}

/// Convert Ed25519 private key to X25519 private key
///
/// WARNING: This uses SHA-512(ed25519_seed) to derive X25519 scalar,
/// which is the standard way in Ed25519→X25519 conversion as documented
/// in libsodium and used by Signal Protocol.
///
/// The resulting scalar is properly clamped according to X25519 specification:
/// - Bits 0, 1, 2 cleared (divisible by 8)
/// - Bit 255 cleared (< 2^255)
/// - Bit 254 set (>= 2^254)
///
/// # Arguments
/// * `ed25519_secret` - Ed25519 secret key (32 bytes seed)
///
/// # Returns
/// * `X25519PrivateKey` - X25519 private key (properly clamped)
///
/// # Example
/// ```ignore
/// let ed25519_secret = [1u8; 32]; // Your Ed25519 secret seed
/// let x25519_secret = ed25519_secret_to_x25519(&ed25519_secret);
/// ```
pub fn ed25519_secret_to_x25519(ed25519_secret: &[u8; 32]) -> X25519PrivateKey {
    debug!("🔄 Converting Ed25519 secret key to X25519");

    // Ed25519 uses SHA-512(seed) for key derivation
    // We use the first 32 bytes as X25519 scalar (standard approach)
    use sha2::{Digest, Sha512};

    let mut hasher = Sha512::new();
    hasher.update(ed25519_secret);
    let hash = hasher.finalize();

    let mut x25519_bytes = [0u8; 32];
    x25519_bytes.copy_from_slice(&hash[0..32]);

    // Clamp scalar (required by X25519 spec, RFC 7748)
    x25519_bytes[0] &= 248;  // Clear bits 0, 1, 2 (divisible by 8)
    x25519_bytes[31] &= 127; // Clear bit 255 (< 2^255)
    x25519_bytes[31] |= 64;  // Set bit 254 (>= 2^254)

    debug!("✅ Ed25519 → X25519 secret key conversion successful (clamped)");
    X25519PrivateKey::from(x25519_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;

    #[test]
    fn test_ed25519_to_x25519_public_conversion() {
        // Generate Ed25519 keypair
        let ed_secret = SigningKey::from_bytes(&[1u8; 32]);
        let ed_public = ed_secret.verifying_key();

        // Convert to X25519
        let result = ed25519_public_to_x25519(&ed_public.to_bytes());
        assert!(result.is_ok(), "Public key conversion should succeed");

        let x25519_pub = result.unwrap();
        assert_eq!(
            x25519_pub.as_bytes().len(),
            32,
            "X25519 public key should be 32 bytes"
        );
    }

    #[test]
    fn test_ed25519_to_x25519_secret_conversion() {
        let ed_secret = [2u8; 32];
        let x25519_secret = ed25519_secret_to_x25519(&ed_secret);

        // Check scalar is properly clamped
        let bytes = x25519_secret.to_bytes();
        assert_eq!(bytes[0] & 0x07, 0, "Low 3 bits should be cleared");
        assert_eq!(bytes[31] & 0x80, 0, "Bit 255 should be cleared");
        assert_eq!(bytes[31] & 0x40, 0x40, "Bit 254 should be set");
    }

    #[test]
    fn test_public_conversion_is_deterministic() {
        // Test that Ed→X conversion is consistent
        let ed_secret = SigningKey::from_bytes(&[3u8; 32]);
        let ed_public = ed_secret.verifying_key();

        let x25519_pub1 = ed25519_public_to_x25519(&ed_public.to_bytes()).unwrap();
        let x25519_pub2 = ed25519_public_to_x25519(&ed_public.to_bytes()).unwrap();

        assert_eq!(
            x25519_pub1.as_bytes(),
            x25519_pub2.as_bytes(),
            "Conversion should be deterministic"
        );
    }

    #[test]
    fn test_secret_conversion_is_deterministic() {
        let ed_secret = [4u8; 32];

        let x25519_secret1 = ed25519_secret_to_x25519(&ed_secret);
        let x25519_secret2 = ed25519_secret_to_x25519(&ed_secret);

        assert_eq!(
            x25519_secret1.to_bytes(),
            x25519_secret2.to_bytes(),
            "Secret conversion should be deterministic"
        );
    }

    #[test]
    fn test_clamping_with_different_seeds() {
        // Test clamping works with various seed patterns
        let test_seeds = vec![
            [0u8; 32],
            [255u8; 32],
            [0xAA; 32],
            [0x55; 32],
        ];

        for seed in test_seeds {
            let x25519_secret = ed25519_secret_to_x25519(&seed);
            let bytes = x25519_secret.to_bytes();

            // Verify clamping
            assert_eq!(
                bytes[0] & 0x07,
                0,
                "Low 3 bits should be cleared for seed {:?}",
                seed[0]
            );
            assert_eq!(
                bytes[31] & 0x80,
                0,
                "Bit 255 should be cleared for seed {:?}",
                seed[0]
            );
            assert_eq!(
                bytes[31] & 0x40,
                0x40,
                "Bit 254 should be set for seed {:?}",
                seed[0]
            );
        }
    }

    #[test]
    fn test_edge_case_all_zeros() {
        // Test with all zeros (identity point - technically valid)
        let zero_pub = [0u8; 32];
        let result = ed25519_public_to_x25519(&zero_pub);

        // All zeros is actually a valid Ed25519 public key (identity point)
        // So conversion should succeed
        assert!(
            result.is_ok(),
            "All-zeros Ed25519 public key should convert successfully (identity point)"
        );
    }
}
