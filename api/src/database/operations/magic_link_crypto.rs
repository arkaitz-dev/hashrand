//! Magic link cryptographic operations
//!
//! Provides cryptographic functions for magic link security including
//! multi-layer encryption using Argon2id, Blake2b, and ChaCha20-Poly1305.

use super::magic_link_types::MagicLinkKeys;
use argon2::{Algorithm, Argon2, Params, Version};
use blake2::{
    Blake2bMac, Blake2bVar,
    digest::{KeyInit as Blake2KeyInit, Mac, Update, VariableOutput},
};
use chacha20poly1305::consts::U32;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::Aead};
use rand_chacha::{ChaCha8Rng, rand_core::RngCore, rand_core::SeedableRng};
use spin_sdk::sqlite::Error as SqliteError;

/// Magic link cryptographic operations
pub struct MagicLinkCrypto;

impl MagicLinkCrypto {
    /// Get magic link content encryption keys from environment
    ///
    /// # Returns
    /// * `Result<MagicLinkKeys, SqliteError>` - Tuple of (cipher_key, nonce_key, salt_key)
    pub fn get_mlink_content_keys() -> Result<MagicLinkKeys, SqliteError> {
        let cipher_key = spin_sdk::variables::get("mlink_content_cipher")
            .map_err(|e| SqliteError::Io(format!("Missing MLINK_CONTENT_CIPHER: {}", e)))?;
        let nonce_key = spin_sdk::variables::get("mlink_content_nonce")
            .map_err(|e| SqliteError::Io(format!("Missing MLINK_CONTENT_NONCE: {}", e)))?;
        let salt_key = spin_sdk::variables::get("mlink_content_salt")
            .map_err(|e| SqliteError::Io(format!("Missing MLINK_CONTENT_SALT: {}", e)))?;

        let cipher_bytes = hex::decode(&cipher_key)
            .map_err(|_| SqliteError::Io("Invalid MLINK_CONTENT_CIPHER format".to_string()))?;
        let nonce_bytes = hex::decode(&nonce_key)
            .map_err(|_| SqliteError::Io("Invalid MLINK_CONTENT_NONCE format".to_string()))?;
        let salt_bytes = hex::decode(&salt_key)
            .map_err(|_| SqliteError::Io("Invalid MLINK_CONTENT_SALT format".to_string()))?;

        if cipher_bytes.len() != 32 || nonce_bytes.len() != 32 || salt_bytes.len() != 32 {
            return Err(SqliteError::Io(
                "Magic link content keys must be 32 bytes each".to_string(),
            ));
        }

        let mut cipher_key = [0u8; 32];
        let mut nonce_key = [0u8; 32];
        let mut salt_key = [0u8; 32];
        cipher_key.copy_from_slice(&cipher_bytes);
        nonce_key.copy_from_slice(&nonce_bytes);
        salt_key.copy_from_slice(&salt_bytes);

        Ok((cipher_key, nonce_key, salt_key))
    }

    /// Encrypt payload using multi-layer security
    ///
    /// Process:
    /// 1. Argon2id(encrypted_data, MLINK_CONTENT_SALT) → derived_key
    /// 2. HMAC-SHA3-256(derived_key, MLINK_CONTENT_NONCE) → ChaCha8RNG → nonce[12]
    /// 3. HMAC-SHA3-256(derived_key, MLINK_CONTENT_CIPHER) → ChaCha8RNG → cipher_key[32]
    /// 4. ChaCha20-Poly1305.encrypt(payload, nonce, cipher_key) → encrypted_blob
    ///
    /// # Arguments
    /// * `encrypted_data` - The encrypted magic token bytes (32 bytes)
    /// * `payload` - Data to encrypt
    ///
    /// # Returns
    /// * `Result<Vec<u8>, SqliteError>` - Encrypted payload or error
    pub fn encrypt_payload_content(
        encrypted_data: &[u8; 32],
        payload: &[u8],
    ) -> Result<Vec<u8>, SqliteError> {
        let (cipher_key_base, nonce_key_base, salt) = Self::get_mlink_content_keys()?;

        // Step 1: Derive key using Argon2id
        let params = Params::new(65536, 3, 4, Some(32))
            .map_err(|e| SqliteError::Io(format!("Argon2 params error: {}", e)))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut derived_key = [0u8; 32];
        argon2
            .hash_password_into(encrypted_data, &salt, &mut derived_key)
            .map_err(|e| SqliteError::Io(format!("Argon2 derivation error: {}", e)))?;

        // Step 2: Generate nonce using Blake2b keyed + ChaCha8RNG
        let mut nonce_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&nonce_key_base)
            .map_err(|_| SqliteError::Io("Invalid nonce key".to_string()))?;
        Mac::update(&mut nonce_hasher, &derived_key);
        let nonce_hmac = nonce_hasher.finalize().into_bytes();

        let mut nonce_seed = [0u8; 32];
        nonce_seed.copy_from_slice(&nonce_hmac[..32]);
        let mut nonce_rng = ChaCha8Rng::from_seed(nonce_seed);
        let mut nonce_bytes = [0u8; 12];
        nonce_rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Step 3: Generate cipher key using Blake2b keyed + ChaCha8RNG
        let mut cipher_hasher =
            <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&cipher_key_base)
                .map_err(|_| SqliteError::Io("Invalid cipher key".to_string()))?;
        Mac::update(&mut cipher_hasher, &derived_key);
        let cipher_hmac = cipher_hasher.finalize().into_bytes();

        let mut cipher_seed = [0u8; 32];
        cipher_seed.copy_from_slice(&cipher_hmac[..32]);
        let mut cipher_rng = ChaCha8Rng::from_seed(cipher_seed);
        let mut cipher_key = [0u8; 32];
        cipher_rng.fill_bytes(&mut cipher_key);
        let key = Key::from_slice(&cipher_key);

        // Step 4: Encrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new(key);
        let ciphertext = cipher
            .encrypt(nonce, payload)
            .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 encryption error: {:?}", e)))?;

        println!("Database: Encrypted payload using multi-layer security");
        Ok(ciphertext)
    }

    /// Decrypt payload using multi-layer security (reverse process)
    ///
    /// # Arguments
    /// * `encrypted_data` - The encrypted magic token bytes (32 bytes)
    /// * `ciphertext` - Encrypted payload to decrypt
    ///
    /// # Returns
    /// * `Result<Vec<u8>, SqliteError>` - Decrypted payload or error
    pub fn decrypt_payload_content(
        encrypted_data: &[u8; 32],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, SqliteError> {
        let (cipher_key_base, nonce_key_base, salt) = Self::get_mlink_content_keys()?;

        // Step 1: Derive key using Argon2id (same as encryption)
        let params = Params::new(65536, 3, 4, Some(32))
            .map_err(|e| SqliteError::Io(format!("Argon2 params error: {}", e)))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let mut derived_key = [0u8; 32];
        argon2
            .hash_password_into(encrypted_data, &salt, &mut derived_key)
            .map_err(|e| SqliteError::Io(format!("Argon2 derivation error: {}", e)))?;

        // Step 2: Regenerate nonce (same process as encryption)
        let mut nonce_hasher = <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&nonce_key_base)
            .map_err(|_| SqliteError::Io("Invalid nonce key".to_string()))?;
        Mac::update(&mut nonce_hasher, &derived_key);
        let nonce_hmac = nonce_hasher.finalize().into_bytes();

        let mut nonce_seed = [0u8; 32];
        nonce_seed.copy_from_slice(&nonce_hmac[..32]);
        let mut nonce_rng = ChaCha8Rng::from_seed(nonce_seed);
        let mut nonce_bytes = [0u8; 12];
        nonce_rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Step 3: Regenerate cipher key (same process as encryption)
        let mut cipher_hasher =
            <Blake2bMac<U32> as Blake2KeyInit>::new_from_slice(&cipher_key_base)
                .map_err(|_| SqliteError::Io("Invalid cipher key".to_string()))?;
        Mac::update(&mut cipher_hasher, &derived_key);
        let cipher_hmac = cipher_hasher.finalize().into_bytes();

        let mut cipher_seed = [0u8; 32];
        cipher_seed.copy_from_slice(&cipher_hmac[..32]);
        let mut cipher_rng = ChaCha8Rng::from_seed(cipher_seed);
        let mut cipher_key = [0u8; 32];
        cipher_rng.fill_bytes(&mut cipher_key);
        let key = Key::from_slice(&cipher_key);

        // Step 4: Decrypt with ChaCha20-Poly1305
        let cipher = ChaCha20Poly1305::new(key);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| SqliteError::Io(format!("ChaCha20-Poly1305 decryption error: {:?}", e)))?;

        println!("Database: Decrypted payload using multi-layer security");
        Ok(plaintext)
    }

    /// Create Blake2b variable hash of encrypted magic token for database storage
    ///
    /// # Arguments
    /// * `encrypted_data` - The encrypted magic token bytes
    ///
    /// # Returns
    /// * `[u8; 16]` - 16-byte Blake2b hash for database indexing
    pub fn create_encrypted_token_hash(encrypted_data: &[u8]) -> [u8; 16] {
        // Blake2b variable output(encrypted_data) → [16 bytes]
        let mut hasher = Blake2bVar::new(16).expect("Blake2b initialization failed");
        hasher.update(encrypted_data);

        let mut hash = [0u8; 16];
        hasher
            .finalize_variable(&mut hash)
            .expect("Blake2b finalization failed");
        hash
    }
}
