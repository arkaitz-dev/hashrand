//! Universal signed request validation system
//!
//! Provides Ed25519 signature verification for all API endpoints
//! with deterministic JSON serialization matching frontend

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

use crate::utils::ed25519::{Ed25519Utils, SignatureVerificationResult};

/// Universal signed request structure for all API endpoints
#[derive(Debug, Deserialize, Serialize)]
pub struct SignedRequest<T> {
    pub payload: T,
    pub signature: String,
}

/// Error type for signed request validation
#[derive(Debug)]
pub enum SignedRequestError {
    InvalidSignature(String),
    MissingPublicKey(String),
    SerializationError(String),
    Ed25519Error(String),
}

impl fmt::Display for SignedRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignedRequestError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
            SignedRequestError::MissingPublicKey(msg) => write!(f, "Missing public key: {}", msg),
            SignedRequestError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            SignedRequestError::Ed25519Error(msg) => write!(f, "Ed25519 validation failed: {}", msg),
        }
    }
}

impl std::error::Error for SignedRequestError {}

/// Signed request validator with Ed25519 verification
pub struct SignedRequestValidator;

impl SignedRequestValidator {
    /// Validate signed request with Ed25519 signature
    ///
    /// # Arguments
    /// * `signed_request` - The signed request to validate
    /// * `public_key_hex` - Ed25519 public key as hex string
    ///
    /// # Returns
    /// * `Result<T, SignedRequestError>` - Validated payload or error
    pub fn validate<T>(
        signed_request: &SignedRequest<T>,
        public_key_hex: &str,
    ) -> Result<(), SignedRequestError>
    where
        T: Serialize,
    {
        // Serialize payload deterministically (matching frontend)
        let serialized_payload = Self::serialize_payload_deterministic(&signed_request.payload)
            .map_err(|e| SignedRequestError::SerializationError(e.to_string()))?;

        println!(
            "🔍 Validating signed request - Payload size: {}, Signature: {}...",
            serialized_payload.len(),
            &signed_request.signature[..20.min(signed_request.signature.len())]
        );

        // Verify Ed25519 signature
        let verification_result = Ed25519Utils::verify_signature_string(
            &serialized_payload,
            &signed_request.signature,
            public_key_hex,
        );

        match verification_result {
            SignatureVerificationResult::Valid => {
                println!("✅ Signed request signature validation successful");
                Ok(())
            }
            SignatureVerificationResult::Invalid => Err(
                SignedRequestError::InvalidSignature("Ed25519 signature verification failed".to_string()),
            ),
            SignatureVerificationResult::MalformedPublicKey => Err(
                SignedRequestError::InvalidSignature("Invalid Ed25519 public key format".to_string()),
            ),
            SignatureVerificationResult::MalformedSignature => Err(
                SignedRequestError::InvalidSignature("Invalid signature format".to_string()),
            ),
            SignatureVerificationResult::MalformedMessage => Err(
                SignedRequestError::InvalidSignature("Invalid message format".to_string()),
            ),
        }
    }

    /// Deterministic JSON serialization (matching frontend sortObjectKeys)
    ///
    /// Recursively sorts object keys to ensure identical serialization
    /// between frontend JavaScript and backend Rust
    fn serialize_payload_deterministic<T>(payload: &T) -> Result<String, serde_json::Error>
    where
        T: Serialize,
    {
        // First serialize to Value to manipulate structure
        let value = serde_json::to_value(payload)?;

        // Sort keys recursively
        let sorted_value = Self::sort_json_keys(value);

        // Serialize to string with no whitespace (compact)
        serde_json::to_string(&sorted_value)
    }

    /// Recursively sort JSON object keys for deterministic serialization
    fn sort_json_keys(value: Value) -> Value {
        match value {
            Value::Object(map) => {
                let mut sorted_map = serde_json::Map::new();

                // Sort keys and recursively process values
                let mut keys: Vec<_> = map.keys().collect();
                keys.sort();

                for key in keys {
                    if let Some(val) = map.get(key) {
                        sorted_map.insert(key.clone(), Self::sort_json_keys(val.clone()));
                    }
                }

                Value::Object(sorted_map)
            }
            Value::Array(array) => {
                // Recursively sort array elements (but preserve order)
                Value::Array(array.into_iter().map(Self::sort_json_keys).collect())
            }
            other => other, // Primitives remain unchanged
        }
    }
}

/// Public key extraction trait for different endpoint types
pub trait PublicKeyExtractor {
    /// Extract Ed25519 public key for signature validation
    fn extract_public_key(&self) -> Result<String, SignedRequestError>;
}

/// Extract public key from payload (for /api/login/)
pub struct PayloadPublicKeyExtractor<'a> {
    pub payload: &'a Value,
}

impl<'a> PublicKeyExtractor for PayloadPublicKeyExtractor<'a> {
    fn extract_public_key(&self) -> Result<String, SignedRequestError> {
        self.payload
            .get("pub_key")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                SignedRequestError::MissingPublicKey("pub_key not found in payload".to_string())
            })
    }
}

/// Extract public key from JWT token (for protected endpoints)
pub struct TokenPublicKeyExtractor {
    pub public_key: String,
}

impl PublicKeyExtractor for TokenPublicKeyExtractor {
    fn extract_public_key(&self) -> Result<String, SignedRequestError> {
        Ok(self.public_key.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deterministic_serialization() {
        let payload = json!({
            "email": "test@example.com",
            "ui_host": "https://example.com",
            "pub_key": "abc123"
        });

        let serialized = SignedRequestValidator::serialize_payload_deterministic(&payload).unwrap();

        // Should be deterministic regardless of object key order
        assert!(serialized.contains("\"email\":\"test@example.com\""));
        assert!(serialized.contains("\"pub_key\":\"abc123\""));
        assert!(serialized.contains("\"ui_host\":\"https://example.com\""));
    }

    #[test]
    fn test_sort_json_keys() {
        let unsorted = json!({
            "z_field": "last",
            "a_field": "first",
            "nested": {
                "z_nested": "nested_last",
                "a_nested": "nested_first"
            }
        });

        let sorted = SignedRequestValidator::sort_json_keys(unsorted);
        let serialized = serde_json::to_string(&sorted).unwrap();

        // Keys should be sorted alphabetically
        let a_pos = serialized.find("\"a_field\"").unwrap();
        let z_pos = serialized.find("\"z_field\"").unwrap();
        assert!(a_pos < z_pos, "Keys should be sorted alphabetically");
    }
}