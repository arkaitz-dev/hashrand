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

use blake3;
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519PrivateKey};

const NONCE_LENGTH: usize = 12;
const CIPHER_KEY_LENGTH: usize = 32;

/// Encrypt key material using ECDH + ChaCha20-Poly1305
///
/// Process:
/// 1. Compute shared_secret = x25519(my_private, their_public)
/// 2. Derive cipher_key[32] + nonce[12] using Blake3 KDF
/// 3. Encrypt with ChaCha20-Poly1305 (adds 16-byte Poly1305 MAC tag)
///
/// # Arguments
/// * `key_material` - Data to encrypt (typically 44 bytes for shared secrets)
/// * `my_private` - My X25519 private key
/// * `their_public` - Recipient's X25519 public key
///
/// # Returns
/// * `Result<Vec<u8>, SqliteError>` - Encrypted data (plaintext_len + 16 bytes MAC)
///
/// # Example
/// ```ignore
/// let my_secret = X25519PrivateKey::from([1u8; 32]);
/// let their_public = X25519PublicKey::from([2u8; 32]);
/// let key_material = [42u8; 44];
/// let encrypted = encrypt_with_ecdh(&key_material, &my_secret, &their_public)?;
/// ```
pub fn encrypt_with_ecdh(
    key_material: &[u8],
    my_private: &X25519PrivateKey,
    their_public: &X25519PublicKey,
) -> Result<Vec<u8>, SqliteError> {
    debug!(
        "🔐 ECDH: Starting key_material encryption (size={})",
        key_material.len()
    );

    // 1. ECDH key exchange
    let shared_secret = my_private.diffie_hellman(their_public);
    debug!("🔐 ECDH: Computed shared secret");

    // 2. Derive cipher_key[32] + nonce[12] using Blake3 KDF
    let context = b"SharedSecretKeyMaterial_v1";

    // Use Blake3 keyed hash with shared secret, then XOF for 44 bytes
    let mut hasher = blake3::Hasher::new_keyed(shared_secret.as_bytes());
    hasher.update(context);
    let mut xof_reader = hasher.finalize_xof();

    let mut derived = vec![0u8; CIPHER_KEY_LENGTH + NONCE_LENGTH];
    xof_reader.fill(&mut derived);

    let cipher_key: [u8; CIPHER_KEY_LENGTH] = derived[0..CIPHER_KEY_LENGTH]
        .try_into()
        .map_err(|_| SqliteError::Io("Failed to extract cipher key".to_string()))?;

    let nonce_bytes: [u8; NONCE_LENGTH] =
        derived[CIPHER_KEY_LENGTH..CIPHER_KEY_LENGTH + NONCE_LENGTH]
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to extract nonce".to_string()))?;

    debug!("🔐 ECDH: Derived cipher_key[32] + nonce[12] with Blake3");

    // 3. Encrypt with ChaCha20-Poly1305
    let key = Key::from_slice(&cipher_key);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    let encrypted = cipher
        .encrypt(nonce, key_material)
        .map_err(|e| SqliteError::Io(format!("ECDH encryption error: {:?}", e)))?;

    debug!(
        "✅ ECDH: Encrypted key_material (output_size={})",
        encrypted.len()
    );
    Ok(encrypted)
}

/// Decrypt key material using ECDH + ChaCha20-Poly1305
///
/// Reverses the encryption process using the same shared secret derivation.
///
/// # Arguments
/// * `ciphertext` - Encrypted data (includes 16-byte MAC tag)
/// * `my_private` - My X25519 private key
/// * `their_public` - Sender's X25519 public key
///
/// # Returns
/// * `Result<Vec<u8>, SqliteError>` - Decrypted key material
///
/// # Errors
/// Returns error if:
/// - ECDH key exchange fails
/// - KDF derivation fails
/// - ChaCha20-Poly1305 decryption fails (wrong key or tampered data)
///
/// # Example
/// ```ignore
/// let my_secret = X25519PrivateKey::from([2u8; 32]);
/// let their_public = X25519PublicKey::from([1u8; 32]);
/// let decrypted = decrypt_with_ecdh(&encrypted, &my_secret, &their_public)?;
/// ```
pub fn decrypt_with_ecdh(
    ciphertext: &[u8],
    my_private: &X25519PrivateKey,
    their_public: &X25519PublicKey,
) -> Result<Vec<u8>, SqliteError> {
    debug!(
        "🔓 ECDH: Starting key_material decryption (ciphertext_size={})",
        ciphertext.len()
    );

    // 1. ECDH key exchange (same shared secret as encryption)
    let shared_secret = my_private.diffie_hellman(their_public);
    debug!("🔓 ECDH: Computed shared secret");

    // 2. Derive same cipher_key + nonce (deterministic with same shared secret)
    let context = b"SharedSecretKeyMaterial_v1";

    // Use Blake3 keyed hash with shared secret, then XOF for 44 bytes
    let mut hasher = blake3::Hasher::new_keyed(shared_secret.as_bytes());
    hasher.update(context);
    let mut xof_reader = hasher.finalize_xof();

    let mut derived = vec![0u8; CIPHER_KEY_LENGTH + NONCE_LENGTH];
    xof_reader.fill(&mut derived);

    let cipher_key: [u8; CIPHER_KEY_LENGTH] = derived[0..CIPHER_KEY_LENGTH]
        .try_into()
        .map_err(|_| SqliteError::Io("Failed to extract cipher key".to_string()))?;

    let nonce_bytes: [u8; NONCE_LENGTH] =
        derived[CIPHER_KEY_LENGTH..CIPHER_KEY_LENGTH + NONCE_LENGTH]
            .try_into()
            .map_err(|_| SqliteError::Io("Failed to extract nonce".to_string()))?;

    debug!("🔓 ECDH: Derived cipher_key[32] + nonce[12] with Blake3");

    // 3. Decrypt with ChaCha20-Poly1305
    let key = Key::from_slice(&cipher_key);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| SqliteError::Io(format!("ECDH decryption error: {:?}", e)))?;

    debug!(
        "✅ ECDH: Decrypted key_material (output_size={})",
        plaintext.len()
    );
    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

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
