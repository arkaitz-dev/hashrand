//! Cryptographic utilities module
//!
//! Provides cryptographic operations for the HashRand API including:
//! - Ed25519 to X25519 key conversion (for ECDH)
//! - ECDH-based encryption/decryption
//! - Backend key management for E2E encryption

pub mod backend_keys;
pub mod ecdh_encryption;
pub mod ed25519_to_x25519;

pub use backend_keys::*;
pub use ecdh_encryption::*;
pub use ed25519_to_x25519::*;
