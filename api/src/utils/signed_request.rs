//! Universal signed request validation system
//!
//! Provides Ed25519 signature verification for all API endpoints
//! CORRECTED: Uses deterministic JSON + Base64 URL-safe for perfect consistency

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

use crate::database::operations::magic_link_ops::MagicLinkOperations;
use crate::utils::ed25519::{Ed25519Utils, SignatureVerificationResult};
use crate::utils::jwt::utils::JwtUtils;
use spin_sdk::http::Request;

/// Universal signed request structure for all API endpoints
/// CORRECTED: payload is Base64 URL-safe encoded deterministic JSON
/// Signature verifies the original JSON string (before Base64 encoding)
#[derive(Debug, Deserialize, Serialize)]
pub struct SignedRequest {
    /// Base64 URL-safe encoded deterministic JSON payload
    pub payload: String,
    /// Ed25519 signature of the original JSON string (before Base64 encoding)
    pub signature: String,
}

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

/// Signed request validator with Ed25519 verification
pub struct SignedRequestValidator;

impl SignedRequestValidator {
    /// Universal validation with strict auth method separation
    /// CORRECTED: Now validates Base64 URL-safe encoded JSON payload for perfect consistency
    ///
    /// SECURITY RULES:
    /// 1. Bearer token present: ONLY Bearer allowed, NO pub_key/magiclink in payload
    /// 2. No Bearer token: EXACTLY one of pub_key OR magiclink in payload (never both, never none)
    ///
    /// # Arguments
    /// * `signed_request` - The signed request with Base64-encoded JSON payload
    /// * `request` - HTTP request (for Bearer token extraction)
    ///
    /// # Returns
    /// * `Result<String, SignedRequestError>` - pub_key_hex or error
    pub fn validate_universal(
        signed_request: &SignedRequest,
        request: &Request,
    ) -> Result<String, SignedRequestError> {
        println!("🔍 Universal SignedRequest validation with strict auth separation...");

        // Decode Base64 payload to check auth method contents
        println!("🔍 DEBUG BASE64: Decoding Base64 payload for auth method detection...");
        let json_string = Self::decode_payload_base64(&signed_request.payload).map_err(|e| {
            SignedRequestError::SerializationError(format!("Base64 decoding failed: {}", e))
        })?;

        let payload_value: Value = serde_json::from_str(&json_string).map_err(|e| {
            SignedRequestError::SerializationError(format!("JSON parsing failed: {}", e))
        })?;

        println!("🔍 DEBUG BASE64: Decoded payload: {}", payload_value);

        // Check what auth methods are present in payload
        let has_pub_key = payload_value
            .get("pub_key")
            .and_then(|v| v.as_str())
            .is_some();
        let has_magiclink = payload_value
            .get("magiclink")
            .and_then(|v| v.as_str())
            .is_some();

        // Check if Bearer token is present
        let has_bearer = Self::extract_pub_key_from_bearer(request).is_ok();

        println!(
            "🔍 Auth method detection - Bearer: {}, pub_key: {}, magiclink: {}",
            has_bearer, has_pub_key, has_magiclink
        );

        // STRICT VALIDATION RULES
        if has_bearer {
            // Rule 1: Bearer token present - NO other auth methods allowed in payload
            if has_pub_key || has_magiclink {
                return Err(SignedRequestError::ConflictingAuthMethods(
                    "Bearer token present but payload contains pub_key/magiclink - only Bearer allowed".to_string()
                ));
            }

            // Use Bearer token for validation
            let pub_key_hex = Self::extract_pub_key_from_bearer(request)?;
            println!("✅ Using ONLY Bearer token (strict mode)");
            Self::validate_base64_payload(
                &signed_request.payload,
                &signed_request.signature,
                &pub_key_hex,
            )?;
            Ok(pub_key_hex)
        } else {
            // Rule 2: No Bearer token - EXACTLY one payload auth method required
            match (has_pub_key, has_magiclink) {
                (true, true) => {
                    Err(SignedRequestError::AmbiguousPayloadAuth(
                        "Both pub_key and magiclink found in payload - only one allowed".to_string()
                    ))
                }
                (true, false) => {
                    // Use pub_key from payload
                    let pub_key_hex = Self::extract_pub_key_from_payload(&payload_value)?;
                    println!("✅ Using ONLY pub_key from payload (strict mode)");
                    Self::validate_base64_payload(&signed_request.payload, &signed_request.signature, &pub_key_hex)?;
                    Ok(pub_key_hex)
                }
                (false, true) => {
                    // Use magiclink from payload
                    let pub_key_hex = Self::extract_pub_key_from_magiclink(&payload_value)?;
                    println!("✅ Using ONLY magiclink from payload (strict mode)");
                    Self::validate_base64_payload(&signed_request.payload, &signed_request.signature, &pub_key_hex)?;
                    Ok(pub_key_hex)
                }
                (false, false) => {
                    Err(SignedRequestError::MissingPublicKey(
                        "No Bearer token and no pub_key/magiclink in payload - exactly one auth method required".to_string()
                    ))
                }
            }
        }
    }

    /// CORRECTED: Validate Base64 payload with Ed25519 signature
    /// The Base64 string itself is signed/verified directly (most deterministic!)
    ///
    /// # Arguments
    /// * `base64_payload` - Base64 URL-safe encoded payload string (this is what was signed)
    /// * `signature` - Ed25519 signature hex string
    /// * `public_key_hex` - Ed25519 public key as hex string
    ///
    /// # Returns
    /// * `Result<(), SignedRequestError>` - Success or error
    pub fn validate_base64_payload(
        base64_payload: &str,
        signature: &str,
        public_key_hex: &str,
    ) -> Result<(), SignedRequestError> {
        println!(
            "🔍 DEBUG BASE64: Validating signature directly against Base64 string - Length: {}, Signature: {}...",
            base64_payload.len(),
            &signature[..20.min(signature.len())]
        );

        // Validate signature directly against the Base64 string (most deterministic approach!)
        Self::validate_signature_string(base64_payload, signature, public_key_hex)
    }

    /// Method 1: Extract pub_key from Bearer token (JWT)
    fn extract_pub_key_from_bearer(request: &Request) -> Result<String, SignedRequestError> {
        let auth_header = request
            .header("authorization")
            .and_then(|h| h.as_str())
            .ok_or_else(|| {
                SignedRequestError::MissingPublicKey("No Authorization header".to_string())
            })?;

        let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
            SignedRequestError::MissingPublicKey("Invalid Bearer token format".to_string())
        })?;

        let claims = JwtUtils::validate_access_token(token).map_err(|e| {
            SignedRequestError::InvalidSignature(format!("JWT validation failed: {}", e))
        })?;

        Ok(hex::encode(claims.pub_key))
    }

    /// Method 2: Extract pub_key directly from payload
    fn extract_pub_key_from_payload(payload: &Value) -> Result<String, SignedRequestError> {
        payload
            .get("pub_key")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                SignedRequestError::MissingPublicKey("pub_key not found in payload".to_string())
            })
    }

    /// Method 3: Extract pub_key from magiclink via database lookup
    fn extract_pub_key_from_magiclink(payload: &Value) -> Result<String, SignedRequestError> {
        let magiclink = payload
            .get("magiclink")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                SignedRequestError::MissingPublicKey("magiclink not found in payload".to_string())
            })?;

        // Validate magiclink and extract pub_key from database
        let (_is_valid, _next_param, _user_id, pub_key_bytes, _ui_host) =
            MagicLinkOperations::validate_and_consume_magic_link_encrypted(magiclink).map_err(
                |e| {
                    SignedRequestError::InvalidSignature(format!(
                        "Magiclink validation failed: {}",
                        e
                    ))
                },
            )?;

        let pub_key_array = pub_key_bytes.ok_or_else(|| {
            SignedRequestError::MissingPublicKey("No pub_key found in magiclink data".to_string())
        })?;

        Ok(hex::encode(pub_key_array))
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

    /// Decode Base64 URL-safe payload back to original JSON string
    ///
    /// # Arguments
    /// * `base64_payload` - Base64 URL-safe encoded JSON string
    ///
    /// # Returns
    /// * `Result<String, String>` - Original JSON string or error
    pub fn decode_payload_base64(base64_payload: &str) -> Result<String, String> {
        // Convert Base64 URL-safe to standard Base64
        let base64_standard = base64_payload.replace('-', "+").replace('_', "/");

        // Add padding if needed
        let padding_len = (4 - (base64_standard.len() % 4)) % 4;
        let base64_padded = format!("{}{}", base64_standard, "=".repeat(padding_len));

        // Decode Base64 to bytes
        let bytes = general_purpose::STANDARD
            .decode(&base64_padded)
            .map_err(|e| format!("Base64 decoding failed: {}", e))?;

        // Convert bytes to UTF-8 string
        String::from_utf8(bytes).map_err(|e| format!("UTF-8 conversion failed: {}", e))
    }

    /// Deserialize JSON payload back to typed structure
    /// First decodes Base64, then parses JSON
    ///
    /// # Arguments
    /// * `base64_payload` - Base64 URL-safe encoded JSON string
    ///
    /// # Returns
    /// * `Result<T, String>` - Deserialized payload or error
    pub fn deserialize_base64_payload<T>(base64_payload: &str) -> Result<T, String>
    where
        T: for<'de> Deserialize<'de>,
    {
        // Step 1: Decode Base64 to original JSON string
        let json_string = Self::decode_payload_base64(base64_payload)?;

        // Step 2: Parse JSON to typed structure
        serde_json::from_str(&json_string)
            .map_err(|e| format!("JSON deserialization failed: {}", e))
    }

    /// Deterministic JSON serialization for consistent signing
    /// This function creates identical JSON strings between frontend and backend
    pub fn serialize_payload_deterministic<T>(payload: &T) -> Result<String, serde_json::Error>
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
    pub fn sort_json_keys(value: Value) -> Value {
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

// DELETED: PublicKeyExtractor trait and PayloadPublicKeyExtractor struct removed - were completely unused legacy experimental code

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
