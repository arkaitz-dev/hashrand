//! Ed25519 Type Definitions
//!
//! Core types for Ed25519 signature verification

use serde::{Deserialize, Serialize};

/// Ed25519 signature verification result
#[derive(Debug, Clone, PartialEq)]
pub enum SignatureVerificationResult {
    Valid,
    Invalid,
    MalformedPublicKey,
    MalformedSignature,
    MalformedMessage,
}

/// Ed25519 public key and signature container for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ed25519SignatureData {
    /// Ed25519 public key as hex string (64 hex chars = 32 bytes)
    pub public_key: String,
    /// Ed25519 signature as hex string (128 hex chars = 64 bytes)
    pub signature: String,
}
