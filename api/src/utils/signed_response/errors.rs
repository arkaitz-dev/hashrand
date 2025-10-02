//! Error types for signed response generation

use std::fmt;

/// Error type for signed response generation
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum SignedResponseError {
    KeyDerivationError(String),
    SerializationError(String),
    ConfigurationError(String),
}

impl fmt::Display for SignedResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignedResponseError::KeyDerivationError(msg) => {
                write!(f, "Key derivation error: {}", msg)
            }
            SignedResponseError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            SignedResponseError::ConfigurationError(msg) => {
                write!(f, "Configuration error: {}", msg)
            }
        }
    }
}

impl std::error::Error for SignedResponseError {}
