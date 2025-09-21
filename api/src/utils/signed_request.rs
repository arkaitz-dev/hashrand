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
}

impl fmt::Display for SignedRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignedRequestError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
            SignedRequestError::MissingPublicKey(msg) => write!(f, "Missing public key: {}", msg),
            SignedRequestError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
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
    /// * `Result<(), SignedRequestError>` - Success or error
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

        // Use DRY function for validation
        Self::validate_signature_string(
            &serialized_payload,
            &signed_request.signature,
            public_key_hex,
        )
    }

    /// DRY function: Validate Ed25519 signature for any serialized string
    ///
    /// Can be used by both GET (query params) and POST (JSON payload) endpoints
    pub fn validate_signature_string(
        serialized_data: &str,
        signature: &str,
        public_key_hex: &str,
    ) -> Result<(), SignedRequestError> {
        println!(
            "🔍 Validating Ed25519 signature - Data size: {}, Signature: {}...",
            serialized_data.len(),
            &signature[..20.min(signature.len())]
        );

        // Verify Ed25519 signature
        let verification_result =
            Ed25519Utils::verify_signature_string(serialized_data, signature, public_key_hex);

        Self::verify_ed25519_signature_result(verification_result)
    }

    /// DRY function: Process Ed25519 signature verification result
    fn verify_ed25519_signature_result(
        verification_result: SignatureVerificationResult,
    ) -> Result<(), SignedRequestError> {
        match verification_result {
            SignatureVerificationResult::Valid => {
                println!("✅ Ed25519 signature validation successful");
                Ok(())
            }
            SignatureVerificationResult::Invalid => Err(SignedRequestError::InvalidSignature(
                "Ed25519 signature verification failed".to_string(),
            )),
            SignatureVerificationResult::MalformedPublicKey => {
                Err(SignedRequestError::InvalidSignature(
                    "Invalid Ed25519 public key format".to_string(),
                ))
            }
            SignatureVerificationResult::MalformedSignature => Err(
                SignedRequestError::InvalidSignature("Invalid signature format".to_string()),
            ),
            SignatureVerificationResult::MalformedMessage => Err(
                SignedRequestError::InvalidSignature("Invalid message format".to_string()),
            ),
        }
    }

    /// Validate GET request with query parameters + signature
    ///
    /// Query parameters are serialized deterministically and validated with Ed25519
    pub fn validate_query_params(
        query_params: &mut std::collections::HashMap<String, String>,
        public_key_hex: &str,
    ) -> Result<(), SignedRequestError> {
        // Extract signature from query parameters
        let signature = query_params.remove("signature").ok_or_else(|| {
            SignedRequestError::MissingPublicKey("Missing 'signature' query parameter".to_string())
        })?;

        // Serialize remaining query parameters deterministically
        let serialized_params = Self::serialize_query_params_deterministic(query_params)
            .map_err(|e| SignedRequestError::SerializationError(e.to_string()))?;

        // Validate signature using DRY function
        Self::validate_signature_string(&serialized_params, &signature, public_key_hex)
    }

    /// Deterministic query parameters serialization
    ///
    /// Converts HashMap to sorted JSON string for consistent signing
    fn serialize_query_params_deterministic(
        params: &std::collections::HashMap<String, String>,
    ) -> Result<String, serde_json::Error> {
        // Convert HashMap to JSON Value
        let mut json_map = serde_json::Map::new();
        for (key, value) in params {
            json_map.insert(key.clone(), serde_json::Value::String(value.clone()));
        }
        let value = serde_json::Value::Object(json_map);

        // Sort keys recursively and serialize
        let sorted_value = Self::sort_json_keys(value);
        serde_json::to_string(&sorted_value)
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
