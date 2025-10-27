//! ECDH decryption using X25519 + ChaCha20-Poly1305
//!
//! Uses shared KDF logic from kdf module to derive cipher keys.

use chacha20poly1305::{ChaCha20Poly1305, KeyInit, aead::Aead};
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519PrivateKey};

use super::kdf::derive_cipher_and_nonce;

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
    let (cipher_key, nonce_bytes) = derive_cipher_and_nonce(&shared_secret)?;

    debug!("🔓 ECDH: Derived cipher_key[32] + nonce[12] with Blake3");

    // 3. Decrypt with ChaCha20-Poly1305
    let key = &cipher_key.into();
    let nonce = &nonce_bytes.into();
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
