//! Magic link database operations - Main API exports
//!
//! Provides operations for storing and validating encrypted magic links with
//! Blake3 KDF and ChaCha20-Poly1305 AEAD encryption.
//!
//! This module has been refactored into specialized submodules following SOLID principles:
//! - magic_link_types: Type definitions and constants
//! - magic_link_crypto: Cryptographic operations
//! - magic_link_storage: Database storage operations
//! - magic_link_validation: Validation and consumption logic

// Re-export types and constants
pub use super::magic_link_types::{MagicLinkOperations, ValidationResult};

// Re-export storage functions
pub use super::magic_link_storage::{MagicLinkStorage, MagicLinkStorageParams};

// Re-export validation functions
pub use super::magic_link_validation::MagicLinkValidation;

impl MagicLinkOperations {
    /// Store encrypted magic token with Ed25519/X25519 public keys, UI host, and db_index
    ///
    /// Delegates to MagicLinkStorage for backwards compatibility
    pub fn store_magic_link_encrypted(
        params: &MagicLinkStorageParams,
    ) -> Result<(), spin_sdk::sqlite::Error> {
        MagicLinkStorage::store_magic_link_encrypted(params)
    }

    /// Validate and consume encrypted magic token and extract stored Ed25519 public key
    ///
    /// Delegates to MagicLinkValidation for backwards compatibility
    pub fn validate_and_consume_magic_link_encrypted(
        encrypted_token: &str,
    ) -> Result<ValidationResult, spin_sdk::sqlite::Error> {
        MagicLinkValidation::validate_and_consume_magic_link_encrypted(encrypted_token)
    }

    /// Ensure user exists in users table by user_id (insert if not exists)
    ///
    /// Delegates to MagicLinkStorage for backwards compatibility
    pub fn ensure_user_exists(user_id: &[u8; 16]) -> Result<(), spin_sdk::sqlite::Error> {
        MagicLinkStorage::ensure_user_exists(user_id)
    }

    /// Clean up expired magic links
    ///
    /// Delegates to MagicLinkStorage for backwards compatibility
    pub fn cleanup_expired_links() -> Result<u32, spin_sdk::sqlite::Error> {
        MagicLinkStorage::cleanup_expired_links()
    }
}
