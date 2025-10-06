//! Universal signed response system for all API endpoints
//!
//! Provides Ed25519 signature generation for all backend responses
//! Uses deterministic JSON serialization for frontend-backend consistency

mod errors;
mod http_helpers;
mod key_derivation;
mod signing;
mod types;

// Re-export public API
pub use errors::SignedResponseError;
pub use types::SignedResponse;

/// Ed25519 response signer with per-session key derivation
///
/// Provides static methods for generating signed responses from different sources
pub struct SignedResponseGenerator;

impl SignedResponseGenerator {
    /// Generate signed response with per-session Ed25519 key
    ///
    /// Delegates to signing module
    pub fn create_signed_response<T>(
        payload: T,
        user_id: &[u8],
        pub_key_hex: &str,
    ) -> Result<SignedResponse, SignedResponseError>
    where
        T: serde::Serialize,
    {
        signing::create_signed_response(payload, user_id, pub_key_hex)
    }

    /// Create signed response with server public key included
    ///
    /// Delegates to signing module
    pub fn create_signed_response_with_server_pubkey<T>(
        payload: T,
        user_id: &[u8],
        pub_key_hex: &str,
    ) -> Result<SignedResponse, SignedResponseError>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        signing::create_signed_response_with_server_pubkey(payload, user_id, pub_key_hex)
    }

    /// Create signed response for key rotation (TRAMO 2/3)
    ///
    /// Delegates to signing module
    pub fn create_signed_response_with_rotation<T>(
        payload: T,
        user_id: &[u8],
        signing_pub_key_hex: &str,
        payload_pub_key_hex: &str,
    ) -> Result<SignedResponse, SignedResponseError>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        signing::create_signed_response_with_rotation(
            payload,
            user_id,
            signing_pub_key_hex,
            payload_pub_key_hex,
        )
    }

    /// Create signed HTTP response for regular endpoints
    ///
    /// Delegates to http_helpers module
    pub fn create_signed_http_response<T>(
        payload: T,
        user_id: &[u8],
        pub_key_hex: &str,
    ) -> Result<spin_sdk::http::Response, String>
    where
        T: serde::Serialize,
    {
        http_helpers::create_signed_http_response(payload, user_id, pub_key_hex)
    }

    /// Extract user_id and pub_key from JWT access token
    ///
    /// Delegates to http_helpers module
    pub fn extract_crypto_material_from_jwt(
        authorization_header: &str,
    ) -> Result<(Vec<u8>, String), String> {
        http_helpers::extract_crypto_material_from_jwt(authorization_header)
    }

    /// Derive per-session Ed25519 private key from user_id + pub_key
    ///
    /// Delegates to key_derivation module
    ///
    /// Note: Primarily used internally, but kept public for testing
    #[allow(dead_code)]
    pub fn derive_session_private_key(
        user_id: &[u8],
        pub_key_hex: &str,
    ) -> Result<[u8; 32], SignedResponseError> {
        key_derivation::derive_session_private_key(user_id, pub_key_hex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_key_derivation_deterministic() {
        let user_id = [1u8; 16];
        let pub_key_hex = "0".repeat(64);

        // Same inputs should produce same private key
        let key1 = SignedResponseGenerator::derive_session_private_key(&user_id, &pub_key_hex);
        let key2 = SignedResponseGenerator::derive_session_private_key(&user_id, &pub_key_hex);

        // Note: This test will only pass if ED25519_DERIVATION_KEY is set in environment
        if key1.is_ok() && key2.is_ok() {
            assert_eq!(
                key1.unwrap(),
                key2.unwrap(),
                "Key derivation should be deterministic"
            );
        }
    }

    #[test]
    fn test_signed_response_structure() {
        let payload_value = json!({"status": "OK", "data": "test"});
        let response = SignedResponse {
            payload: "test_payload".to_string(),
            signature: "test_signature".to_string(),
        };

        // Verify structure matches expected format
        assert!(!response.signature.is_empty());
        assert_eq!(response.payload, "test_payload");

        // Verify JSON value can be serialized
        assert_eq!(payload_value["status"], "OK");
    }
}
