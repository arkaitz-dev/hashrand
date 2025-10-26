//! Cryptographic utilities module
//!
//! Provides cryptographic operations for the HashRand API including:
//! - ECDH-based encryption/decryption (X25519)
//! - Backend per-user X25519 key derivation for E2E encryption

pub mod backend_keys;
pub mod ecdh;

pub use backend_keys::*;
pub use ecdh::*;
