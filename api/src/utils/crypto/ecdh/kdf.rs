///! Key Derivation Function for ECDH encryption
///!
///! Derives cipher key and nonce from X25519 shared secret using Blake3 KDF.

use blake3;
use spin_sdk::sqlite::Error as SqliteError;
use tracing::debug;
use x25519_dalek::SharedSecret;

pub(super) const NONCE_LENGTH: usize = 12;
pub(super) const CIPHER_KEY_LENGTH: usize = 32;

/// Derive cipher_key[32] + nonce[12] from X25519 shared secret using Blake3 KDF
///
/// Process:
/// 1. Use Blake3 keyed hash with shared_secret as key
/// 2. Update with context string (domain separator)
/// 3. Use XOF (extendable output) to generate 44 bytes
/// 4. Split into cipher_key[32] + nonce[12]
///
/// # Arguments
/// * `shared_secret` - X25519 ECDH shared secret (32 bytes)
///
/// # Returns
/// * `Result<([u8; 32], [u8; 12]), SqliteError>` - (cipher_key, nonce)
pub(super) fn derive_cipher_and_nonce(
    shared_secret: &SharedSecret,
) -> Result<([u8; CIPHER_KEY_LENGTH], [u8; NONCE_LENGTH]), SqliteError> {
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

    debug!("🔐 ECDH KDF: Derived cipher_key[32] + nonce[12] with Blake3");

    Ok((cipher_key, nonce_bytes))
}
