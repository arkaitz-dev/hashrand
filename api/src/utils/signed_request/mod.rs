//! Universal signed request validation system
//!
//! Provides Ed25519 signature verification for all API endpoints
//! Uses deterministic JSON + Base64 URL-safe for perfect consistency

mod errors;
mod extraction;
mod serialization;
mod types;
mod validation;

// Re-export public API
pub use errors::SignedRequestError;
pub use types::SignedRequest;

/// Signed request validator with Ed25519 verification
///
/// Provides static methods for validating signed requests from different sources
pub struct SignedRequestValidator;

impl SignedRequestValidator {
    /// Universal validation with strict auth method separation
    ///
    /// Delegates to validation module
    pub fn validate_universal(
        signed_request: &SignedRequest,
        request: &spin_sdk::http::Request,
    ) -> Result<String, SignedRequestError> {
        validation::validate_universal(signed_request, request)
    }

    /// Validate Base64 payload with Ed25519 signature
    ///
    /// Delegates to validation module
    pub fn validate_base64_payload(
        base64_payload: &str,
        signature: &str,
        public_key_hex: &str,
    ) -> Result<(), SignedRequestError> {
        validation::validate_base64_payload(base64_payload, signature, public_key_hex)
    }

    /// Validate GET request with query parameters + signature
    ///
    /// Delegates to validation module
    pub fn validate_query_params(
        query_params: &mut std::collections::HashMap<String, String>,
        public_key_hex: &str,
    ) -> Result<(), SignedRequestError> {
        validation::validate_query_params(query_params, public_key_hex)
    }

    /// Validate Ed25519 signature for any serialized string
    ///
    /// Delegates to validation module
    ///
    /// Note: Primarily used in tests, but kept public for flexibility
    #[allow(dead_code)]
    pub fn validate_signature_string(
        serialized_data: &str,
        signature: &str,
        public_key_hex: &str,
    ) -> Result<(), SignedRequestError> {
        validation::validate_signature_string(serialized_data, signature, public_key_hex)
    }

    /// Decode Base64 URL-safe payload back to original JSON string
    ///
    /// Delegates to serialization module
    pub fn decode_payload_base64(base64_payload: &str) -> Result<String, String> {
        serialization::decode_payload_base64(base64_payload)
    }

    /// Deserialize JSON payload back to typed structure
    ///
    /// Delegates to serialization module
    pub fn deserialize_base64_payload<T>(base64_payload: &str) -> Result<T, String>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        serialization::deserialize_base64_payload(base64_payload)
    }

    /// Deterministic JSON serialization for consistent signing
    ///
    /// Delegates to serialization module
    pub fn serialize_payload_deterministic<T>(payload: &T) -> Result<String, serde_json::Error>
    where
        T: serde::Serialize,
    {
        serialization::serialize_payload_deterministic(payload)
    }

    /// Recursively sort JSON object keys for deterministic serialization
    ///
    /// Delegates to serialization module
    ///
    /// Note: Primarily used in tests, but kept public for flexibility
    #[allow(dead_code)]
    pub fn sort_json_keys(value: serde_json::Value) -> serde_json::Value {
        serialization::sort_json_keys(value)
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

    #[test]
    fn debug_frontend_vs_backend_serialization() {
        println!("\n🔍 Backend JSON Serialization Test");
        println!("=====================================");

        // Test 1: Magic link payload - CRITICAL TEST
        println!("\n[1] Magic link payload");
        let magic_payload = json!({
            "magiclink": "8ukaMHhcnJJSEePzD5UYaoHgWib1tr8rS6ms73pC985s"
        });
        println!("Input: {}", magic_payload);
        let serialized =
            SignedRequestValidator::serialize_payload_deterministic(&magic_payload).unwrap();
        println!("Serialized: {}", serialized);
        println!("Length: {}", serialized.len());

        // Test 2: Empty object
        println!("\n[2] Empty object");
        let empty_payload = json!({});
        println!("Input: {}", empty_payload);
        let serialized =
            SignedRequestValidator::serialize_payload_deterministic(&empty_payload).unwrap();
        println!("Serialized: {}", serialized);
        println!("Length: {}", serialized.len());

        // Test 3: Login payload
        println!("\n[3] Login payload");
        let login_payload = json!({
            "email": "me@arkaitz.dev",
            "ui_host": "http://localhost:5173",
            "next": "/",
            "email_lang": "en",
            "pub_key": "abc123"
        });
        println!("Input: {}", login_payload);
        let serialized =
            SignedRequestValidator::serialize_payload_deterministic(&login_payload).unwrap();
        println!("Serialized: {}", serialized);
        println!("Length: {}", serialized.len());

        // Assert it works
        assert!(!serialized.is_empty());
    }
}
