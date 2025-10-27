//! Magic link type definitions and constants
//!
//! Contains type aliases, result types, and constants used across
//! magic link operations modules.

// DELETED: Magic link content encryption keys type removed - was completely unused

/// Magic link validation result tuple
///
/// Returns (validation_success, next_param, user_id, ed25519_pub_key, x25519_pub_key, ui_host, privkey_context)
/// - validation_success: boolean indicating if validation succeeded
/// - next_param: optional next destination parameter
/// - user_id: optional 16-byte user identifier
/// - ed25519_pub_key: optional 32-byte Ed25519 public key
/// - x25519_pub_key: optional 32-byte X25519 public key
/// - ui_host: optional UI host (domain) extracted from encrypted blob
/// - privkey_context: 64-byte decrypted private key context (ALWAYS present on successful validation)
pub type ValidationResult = (
    bool,
    Option<String>,
    Option<[u8; 16]>,
    Option<[u8; 32]>,
    Option<[u8; 32]>,
    Option<String>,
    [u8; 64],
);

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

    /// Database index length for user_privkey_context
    pub const DB_INDEX_LENGTH: usize = 16;

    /// Minimum payload length (encryption_blob + db_index + ed25519_pub_key + x25519_pub_key)
    pub const MIN_PAYLOAD_LENGTH: usize =
        ENCRYPTION_BLOB_LENGTH + DB_INDEX_LENGTH + ED25519_BYTES_LENGTH + ED25519_BYTES_LENGTH; // 44 + 16 + 32 + 32 = 124 bytes

    /// Nonce length for ChaCha20-Poly1305
    pub const NONCE_LENGTH: usize = 12;

    /// Secret key length for ChaCha20
    pub const SECRET_KEY_LENGTH: usize = 32;
}
