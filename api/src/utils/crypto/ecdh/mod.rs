//! ECDH-based encryption for key material
//!
//! Uses X25519 key exchange + Blake3 KDF + ChaCha20-Poly1305 AEAD
//!
//! This module provides encryption and decryption of key material using Elliptic Curve
//! Diffie-Hellman (ECDH) key exchange with X25519, followed by key derivation using Blake3,
//! and authenticated encryption using ChaCha20-Poly1305.
//!
//! The context string "SharedSecretKeyMaterial_v1" is used as a domain separator to prevent
//! key reuse across different protocols.

mod decryption;
mod encryption;
mod kdf;

// Re-export public API (maintains backwards compatibility)
pub use decryption::decrypt_with_ecdh;
pub use encryption::encrypt_with_ecdh;

#[cfg(test)]
mod tests {
    use super::*;
    use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519PrivateKey};

    #[test]
    fn test_ecdh_encryption_roundtrip() {
        // Alice and Bob keypairs
        let alice_secret = X25519PrivateKey::from([1u8; 32]);
        let alice_public = X25519PublicKey::from(&alice_secret);

        let bob_secret = X25519PrivateKey::from([2u8; 32]);
        let bob_public = X25519PublicKey::from(&bob_secret);

        let key_material = [42u8; 44];

        // Alice encrypts for Bob
        let encrypted = encrypt_with_ecdh(&key_material, &alice_secret, &bob_public).unwrap();

        // Bob decrypts
        let decrypted = decrypt_with_ecdh(&encrypted, &bob_secret, &alice_public).unwrap();

        assert_eq!(decrypted.as_slice(), &key_material);
        assert_eq!(encrypted.len(), 44 + 16); // plaintext + Poly1305 tag
    }

    #[test]
    fn test_ecdh_wrong_key_fails() {
        let alice_secret = X25519PrivateKey::from([1u8; 32]);
        let bob_public = X25519PublicKey::from(&X25519PrivateKey::from([2u8; 32]));
        let charlie_secret = X25519PrivateKey::from([3u8; 32]);
        let charlie_public = X25519PublicKey::from(&charlie_secret);

        let key_material = [42u8; 44];
        let encrypted = encrypt_with_ecdh(&key_material, &alice_secret, &bob_public).unwrap();

        // Charlie tries to decrypt (wrong key pair)
        let result = decrypt_with_ecdh(&encrypted, &charlie_secret, &charlie_public);
        assert!(result.is_err(), "Decryption with wrong key should fail");
    }

    #[test]
    fn test_ecdh_encryption_is_deterministic() {
        // Same inputs should produce same output
        let alice_secret = X25519PrivateKey::from([5u8; 32]);
        let bob_public = X25519PublicKey::from(&X25519PrivateKey::from([6u8; 32]));
        let key_material = [99u8; 44];

        let encrypted1 = encrypt_with_ecdh(&key_material, &alice_secret, &bob_public).unwrap();
        let encrypted2 = encrypt_with_ecdh(&key_material, &alice_secret, &bob_public).unwrap();

        assert_eq!(
            encrypted1, encrypted2,
            "Encryption should be deterministic with same inputs"
        );
    }

    #[test]
    fn test_ecdh_output_size() {
        let alice_secret = X25519PrivateKey::from([7u8; 32]);
        let bob_public = X25519PublicKey::from(&X25519PrivateKey::from([8u8; 32]));

        // Test with different payload sizes
        let test_sizes = vec![32, 44, 64, 128];

        for size in test_sizes {
            let key_material = vec![0u8; size];
            let encrypted = encrypt_with_ecdh(&key_material, &alice_secret, &bob_public).unwrap();

            // Encrypted size should be plaintext + 16 (Poly1305 tag)
            assert_eq!(
                encrypted.len(),
                size + 16,
                "Encrypted size should be plaintext + 16 for size {}",
                size
            );
        }
    }

    #[test]
    fn test_ecdh_tampered_ciphertext_fails() {
        let alice_secret = X25519PrivateKey::from([9u8; 32]);
        let alice_public = X25519PublicKey::from(&alice_secret);
        let bob_secret = X25519PrivateKey::from([10u8; 32]);
        let bob_public = X25519PublicKey::from(&bob_secret);

        let key_material = [123u8; 44];
        let mut encrypted = encrypt_with_ecdh(&key_material, &alice_secret, &bob_public).unwrap();

        // Tamper with ciphertext (flip a bit)
        encrypted[0] ^= 0x01;

        // Decryption should fail due to authentication tag mismatch
        let result = decrypt_with_ecdh(&encrypted, &bob_secret, &alice_public);
        assert!(
            result.is_err(),
            "Decryption of tampered ciphertext should fail"
        );
    }

    #[test]
    fn test_ecdh_empty_payload() {
        let alice_secret = X25519PrivateKey::from([11u8; 32]);
        let bob_secret = X25519PrivateKey::from([12u8; 32]);
        let alice_public = X25519PublicKey::from(&alice_secret);
        let bob_public = X25519PublicKey::from(&bob_secret);

        let empty_payload: &[u8] = &[];

        let encrypted = encrypt_with_ecdh(empty_payload, &alice_secret, &bob_public).unwrap();
        let decrypted = decrypt_with_ecdh(&encrypted, &bob_secret, &alice_public).unwrap();

        assert_eq!(decrypted.len(), 0, "Empty payload should decrypt to empty");
        assert_eq!(
            encrypted.len(),
            16,
            "Empty payload encrypted should be just the 16-byte tag"
        );
    }

    #[test]
    fn test_ecdh_with_different_keypairs() {
        // Test that different keypairs produce different ciphertexts
        let alice1_secret = X25519PrivateKey::from([13u8; 32]);
        let alice2_secret = X25519PrivateKey::from([14u8; 32]);
        let bob_public = X25519PublicKey::from(&X25519PrivateKey::from([15u8; 32]));

        let key_material = [77u8; 44];

        let encrypted1 = encrypt_with_ecdh(&key_material, &alice1_secret, &bob_public).unwrap();
        let encrypted2 = encrypt_with_ecdh(&key_material, &alice2_secret, &bob_public).unwrap();

        assert_ne!(
            encrypted1, encrypted2,
            "Different keypairs should produce different ciphertexts"
        );
    }
}
