//! Error types for signed request validation

use std::fmt;

/// Error type for signed request validation
#[derive(Debug)]
pub enum SignedRequestError {
    InvalidSignature(String),
    MissingPublicKey(String),
    SerializationError(String),
    ConflictingAuthMethods(String),
    AmbiguousPayloadAuth(String),
}

impl fmt::Display for SignedRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignedRequestError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
            SignedRequestError::MissingPublicKey(msg) => write!(f, "Missing public key: {}", msg),
            SignedRequestError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            SignedRequestError::ConflictingAuthMethods(msg) => {
                write!(f, "Conflicting auth methods: {}", msg)
            }
            SignedRequestError::AmbiguousPayloadAuth(msg) => {
                write!(f, "Ambiguous payload auth: {}", msg)
            }
        }
    }
}

impl std::error::Error for SignedRequestError {}
