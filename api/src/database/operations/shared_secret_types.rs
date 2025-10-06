//! Shared secret type definitions and constants
//!
//! Contains type aliases, enums, and constants used across
//! shared secret operations modules.

use serde::{Deserialize, Serialize};

/// Role in the shared secret relationship
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecretRole {
    /// Sender (creator) of the secret - has unlimited reads
    Sender,
    /// Receiver (recipient) of the secret - has limited reads
    Receiver,
}

impl SecretRole {
    /// Convert role to database string representation
    pub fn to_str(self) -> &'static str {
        match self {
            SecretRole::Sender => "sender",
            SecretRole::Receiver => "receiver",
        }
    }

    /// Parse role from database string representation
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sender" => Some(SecretRole::Sender),
            "receiver" => Some(SecretRole::Receiver),
            _ => None,
        }
    }

    /// Get pending_reads value for this role
    /// Returns -1 for sender (unlimited), or the provided max_reads for receiver
    #[allow(dead_code)]
    pub fn pending_reads(&self, max_reads: i64) -> i64 {
        match self {
            SecretRole::Sender => -1, // Unlimited reads for sender
            SecretRole::Receiver => max_reads,
        }
    }
}

/// Decrypted shared secret payload structure
#[derive(Debug, Clone)]
pub struct SharedSecretPayload {
    /// Sender email address
    pub sender_email: String,
    /// Receiver email address
    pub receiver_email: String,
    /// Secret text content (max 512 UTF-8 characters)
    pub secret_text: String,
    /// Optional 9-digit OTP
    pub otp: Option<String>,
    /// Creation timestamp (Unix epoch seconds)
    #[allow(dead_code)]
    pub created_at: i64,
    /// Reference hash for tracking ([u8;16] as Base58)
    pub reference_hash: Vec<u8>,
    /// Maximum reads allowed (stored in encrypted payload, used for validation & UI)
    pub max_reads: i64,
}

/// Shared secret database operations struct
///
/// This struct serves as a namespace for all shared secret related
/// database operations including storage, retrieval, validation, and cleanup.
#[allow(dead_code)]
pub struct SharedSecretOperations;

/// Constants for shared secret operations
pub mod constants {
    /// Length of reference hash (random identifier)
    pub const REFERENCE_HASH_LENGTH: usize = 16;

    /// Maximum length for secret text in UTF-8 characters
    pub const MAX_TEXT_LENGTH: usize = 512;

    /// OTP length in digits
    pub const OTP_LENGTH: usize = 9;

    /// Nonce length for ChaCha20-Poly1305
    pub const NONCE_LENGTH: usize = 12;

    /// Secret key length for ChaCha20
    pub const SECRET_KEY_LENGTH: usize = 32;

    /// User ID length (Blake3 hash)
    pub const USER_ID_LENGTH: usize = 16;

    /// Encrypted ID length (similar to magic links)
    pub const ENCRYPTED_ID_LENGTH: usize = 32;

    /// URL hash length (reference_hash + user_id + checksum + role)
    #[allow(dead_code)]
    pub const URL_HASH_LENGTH: usize = 40;

    /// Database index length (for PRIMARY KEY)
    pub const DB_INDEX_LENGTH: usize = 32;

    /// Minimum expiration hours
    pub const MIN_EXPIRES_HOURS: i64 = 1;

    /// Maximum expiration hours (72 hours = 3 days)
    pub const MAX_EXPIRES_HOURS: i64 = 72;

    /// Default expiration hours
    pub const DEFAULT_EXPIRES_HOURS: i64 = 24;

    /// Minimum read limit
    pub const MIN_READS: i64 = 1;

    /// Maximum read limit
    pub const MAX_READS: i64 = 10;

    /// Default read limit
    pub const DEFAULT_READS: i64 = 3;

    /// Unlimited reads (for sender)
    pub const UNLIMITED_READS: i64 = -1;
}
