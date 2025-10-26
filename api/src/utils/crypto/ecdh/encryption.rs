///! ECDH encryption using X25519 + ChaCha20-Poly1305
///!
///! Uses shared KDF logic from kdf module to derive cipher keys.

use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret as X25519PrivateKey};

use super::kdf::derive_cipher_and_nonce;

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

    // 2. Derive cipher_key[32] + nonce[12] using Blake3 KDF (shared helper)
    let (cipher_key, nonce_bytes) = derive_cipher_and_nonce(&shared_secret)?;

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
