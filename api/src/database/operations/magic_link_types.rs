//! Magic link type definitions and constants
//!
//! Contains type aliases, result types, and constants used across
//! magic link operations modules.

/// Magic link content encryption keys (cipher, nonce, salt)
pub type MagicLinkKeys = ([u8; 32], [u8; 32], [u8; 32]);

/// Magic link validation result tuple
///
/// Returns (validation_success, next_param, user_id, pub_key)
/// - validation_success: boolean indicating if validation succeeded
/// - next_param: optional next destination parameter
/// - user_id: optional 16-byte user identifier
/// - pub_key: optional 32-byte Ed25519 public key
pub type ValidationResult = (bool, Option<String>, Option<[u8; 16]>, Option<[u8; 32]>);

/// Magic link database operations struct
///
/// This struct serves as a namespace for all magic link related
/// database operations including storage, validation, and cleanup.
pub struct MagicLinkOperations;

/// Constants for magic link operations
pub mod constants {
    /// Required length for encrypted token data (ChaCha20 encrypted)
    pub const ENCRYPTED_TOKEN_LENGTH: usize = 32;

    /// Required length for encryption blob (nonce + secret_key)
    pub const ENCRYPTION_BLOB_LENGTH: usize = 44;

    /// Required length for Ed25519 public key in hex format
    pub const ED25519_HEX_LENGTH: usize = 64;

    /// Required length for Ed25519 public key in bytes
    pub const ED25519_BYTES_LENGTH: usize = 32;

    /// Required length for user ID
    pub const USER_ID_LENGTH: usize = 16;

    /// Minimum payload length (encryption_blob + pub_key)
    pub const MIN_PAYLOAD_LENGTH: usize = ENCRYPTION_BLOB_LENGTH + ED25519_BYTES_LENGTH;

    /// Nonce length for ChaCha20-Poly1305
    pub const NONCE_LENGTH: usize = 12;

    /// Secret key length for ChaCha20
    pub const SECRET_KEY_LENGTH: usize = 32;
}
