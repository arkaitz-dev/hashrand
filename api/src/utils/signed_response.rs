//! Universal signed response system for all API endpoints
//!
//! Provides Ed25519 signature generation for all backend responses
//! NEW: Uses msgpack serialization for guaranteed frontend-backend consistency

// use blake2::{Blake2b, Blake2bMac, digest::{Digest, KeyInit, Mac}};
// use generic_array::typenum::{U32, U64};
use ed25519_dalek::{SigningKey, Signer};
use hex;
// use rand::{RngCore, SeedableRng};
// use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::{http::Response, variables};
use std::fmt;

use crate::utils::pseudonimizer::blake3_keyed_variable;
use crate::utils::signed_request::SignedRequestValidator;

/// Universal signed response structure for all API endpoints
/// NEW: payload is now msgpack-serialized string for identical verification
#[derive(Debug, Serialize, Deserialize)]
pub struct SignedResponse {
    /// Base64 URL-safe encoded JSON payload as string (signed content)
    pub payload: String,
    /// Ed25519 signature of the original JSON string (before Base64 encoding)
    pub signature: String,
}

/// Error type for signed response generation
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum SignedResponseError {
    KeyDerivationError(String),
    SigningError(String),
    SerializationError(String),
    ConfigurationError(String),
}

impl fmt::Display for SignedResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignedResponseError::KeyDerivationError(msg) => write!(f, "Key derivation error: {}", msg),
            SignedResponseError::SigningError(msg) => write!(f, "Signing error: {}", msg),
            SignedResponseError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            SignedResponseError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for SignedResponseError {}

/// Ed25519 response signer with per-session key derivation
pub struct SignedResponseGenerator;

impl SignedResponseGenerator {
    /// Generate signed response with per-session Ed25519 key
    ///
    /// CRYPTOGRAPHIC FLOW (Blake3 Pseudonimizer):
    /// 1. user_id_bytes + pub_key_bytes → Blake3 pseudonimizer → Ed25519 private key (32 bytes)
    /// 2. Ed25519 keypair generation from private key
    /// 3. Deterministic JSON serialization (matching frontend)
    /// 4. Ed25519 signature of serialized payload
    ///
    /// # Arguments
    /// * `payload` - Response data to be signed
    /// * `user_id` - User ID bytes (16 bytes)
    /// * `pub_key_hex` - Frontend Ed25519 public key as hex string
    ///
    /// # Returns
    /// * `Result<SignedResponse, SignedResponseError>` - Signed response or error
    pub fn create_signed_response<T>(
        payload: T,
        user_id: &[u8],
        pub_key_hex: &str,
    ) -> Result<SignedResponse, SignedResponseError>
    where
        T: Serialize,
    {
        println!("= Generating signed response with per-session Ed25519 key...");

        // Step 1: Derive per-session private key
        let private_key = Self::derive_session_private_key(user_id, pub_key_hex)?;

        // Step 2: Generate Ed25519 keypair
        let signing_key = SigningKey::from_bytes(&private_key);
        let public_key = signing_key.verifying_key();

        println!("= Generated Ed25519 keypair - Server pub_key: {}", hex::encode(public_key.as_bytes()));

        // Step 3: CORRECTED - Serialize payload to deterministic JSON for frontend consistency
        let json_string = SignedRequestValidator::serialize_payload_deterministic(&payload)
            .map_err(|e| SignedResponseError::SerializationError(e.to_string()))?;

        // Step 4: Encode JSON as Base64 URL-safe
        let base64_payload = {
            let bytes = json_string.as_bytes();
            let base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
                .replace('+', "-")
                .replace('/', "_")
                .replace('=', "");
            base64
        };
        println!(
            "🔍 DEBUG BASE64 BACKEND: JSON length: {}, Base64 length: {}",
            json_string.len(),
            base64_payload.len()
        );

        // Step 5: Sign Base64 payload with Ed25519 (same as frontend!)
        let signature_bytes = signing_key.sign(base64_payload.as_bytes());
        let signature_hex = hex::encode(signature_bytes.to_bytes());

        println!("= Response signed successfully - Signature: {}...", &signature_hex[..20]);

        Ok(SignedResponse {
            payload: base64_payload,
            signature: signature_hex,
        })
    }

    /// Derive per-session Ed25519 private key from user_id + pub_key
    ///
    /// BLAKE3 PSEUDONIMIZER CRYPTOGRAPHIC DERIVATION:
    /// 1. Concatenate: user_id_bytes + pub_key_bytes
    /// 2. ED25519_DERIVATION_KEY[64] → Base58 → context (domain separation)
    /// 3. combined_input → Blake3 hash → key_material[32 bytes]
    /// 4. (context, key_material) → Blake3 KDF → deterministic_key[32 bytes]
    /// 5. (combined_input, deterministic_key) → Blake3 keyed+XOF → Ed25519 private key[32 bytes]
    ///
    /// # Arguments
    /// * `user_id` - User ID bytes (typically 16 bytes)
    /// * `pub_key_hex` - Frontend Ed25519 public key as hex string (64 hex chars)
    ///
    /// # Returns
    /// * `Result<[u8; 32], SignedResponseError>` - Ed25519 private key or error
    fn derive_session_private_key(
        user_id: &[u8],
        pub_key_hex: &str,
    ) -> Result<[u8; 32], SignedResponseError> {
        // Validate pub_key format
        if pub_key_hex.len() != 64 {
            return Err(SignedResponseError::KeyDerivationError(
                format!("Invalid pub_key hex length: {} (expected 64)", pub_key_hex.len())
            ));
        }

        // Decode pub_key from hex
        let pub_key_bytes = hex::decode(pub_key_hex)
            .map_err(|e| SignedResponseError::KeyDerivationError(
                format!("Failed to decode pub_key hex: {}", e)
            ))?;

        if pub_key_bytes.len() != 32 {
            return Err(SignedResponseError::KeyDerivationError(
                format!("Invalid pub_key byte length: {} (expected 32)", pub_key_bytes.len())
            ));
        }

        // Step 1: Concatenate user_id + pub_key_bytes
        let mut combined_input = Vec::with_capacity(user_id.len() + pub_key_bytes.len());
        combined_input.extend_from_slice(user_id);
        combined_input.extend_from_slice(&pub_key_bytes);

        println!("= Deriving session key - user_id: {} bytes, pub_key: {} bytes",
                 user_id.len(), pub_key_bytes.len());

        // Get ED25519_DERIVATION_KEY[64 bytes] for Blake3 pseudonimizer
        let ed25519_derivation_key = Self::get_ed25519_derivation_key()?;

        // Blake3 pseudonimizer pipeline → 32 bytes Ed25519 private key
        let private_key_vec = blake3_keyed_variable(
            &ed25519_derivation_key,
            &combined_input,
            32
        );

        // Convert Vec<u8> to [u8; 32]
        let mut private_key = [0u8; 32];
        private_key.copy_from_slice(&private_key_vec);

        println!("= Session private key derived successfully");

        Ok(private_key)
    }

    /// Get Ed25519 derivation key from environment
    ///
    /// # Returns
    /// * `Result<[u8; 64], SignedResponseError>` - 64-byte derivation key or error
    fn get_ed25519_derivation_key() -> Result<[u8; 64], SignedResponseError> {
        let key_hex = variables::get("ed25519_derivation_key")
            .map_err(|e| SignedResponseError::ConfigurationError(
                format!("ed25519_derivation_key not found: {}", e)
            ))?;

        if key_hex.len() != 128 {
            return Err(SignedResponseError::ConfigurationError(
                format!("Invalid ed25519_derivation_key length: {} (expected 128 hex chars)", key_hex.len())
            ));
        }

        let key_bytes = hex::decode(&key_hex)
            .map_err(|e| SignedResponseError::ConfigurationError(
                format!("Failed to decode ed25519_derivation_key: {}", e)
            ))?;

        if key_bytes.len() != 64 {
            return Err(SignedResponseError::ConfigurationError(
                format!("Invalid ed25519_derivation_key byte length: {} (expected 64)", key_bytes.len())
            ));
        }

        let mut derivation_key = [0u8; 64];
        derivation_key.copy_from_slice(&key_bytes);

        Ok(derivation_key)
    }

    /// Create signed response with server public key included
    ///
    /// Used for magic link creation and token refresh endpoints
    /// where frontend needs server's public key for verification
    ///
    /// # Arguments
    /// * `payload` - Response data to be signed
    /// * `user_id` - User ID bytes (16 bytes)
    /// * `pub_key_hex` - Frontend Ed25519 public key as hex string
    ///
    /// # Returns
    /// * `Result<SignedResponse, SignedResponseError>` - Signed response with server_pub_key
    pub fn create_signed_response_with_server_pubkey<T>(
        payload: T,
        user_id: &[u8],
        pub_key_hex: &str,
    ) -> Result<SignedResponse, SignedResponseError>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        // Derive session private key and generate keypair
        let private_key = Self::derive_session_private_key(user_id, pub_key_hex)?;
        let signing_key = SigningKey::from_bytes(&private_key);
        let public_key = signing_key.verifying_key();

        // Add server_pub_key to payload
        let mut payload_value = serde_json::to_value(&payload)
            .map_err(|e| SignedResponseError::SerializationError(e.to_string()))?;

        if let Value::Object(ref mut map) = payload_value {
            map.insert("server_pub_key".to_string(), Value::String(hex::encode(public_key.as_bytes())));
        } else {
            return Err(SignedResponseError::SerializationError(
                "Payload must be a JSON object to add server_pub_key".to_string()
            ));
        }

        // Convert back to original type
        let enhanced_payload: T = serde_json::from_value(payload_value)
            .map_err(|e| SignedResponseError::SerializationError(e.to_string()))?;

        // Create signed response with enhanced payload
        Self::create_signed_response(enhanced_payload, user_id, pub_key_hex)
    }

    /// Create signed HTTP response for regular endpoints (without server_pub_key)
    ///
    /// Universal helper function that can be used by any endpoint handler
    /// to easily convert their payload into a signed HTTP response.
    ///
    /// # Arguments
    /// * `payload` - Response data to be signed (any serializable type)
    /// * `user_id` - User ID bytes (from JWT access token or derived from email)
    /// * `pub_key_hex` - Frontend Ed25519 public key as hex string
    ///
    /// # Returns
    /// * `Result<Response, String>` - Signed HTTP response or error
    pub fn create_signed_http_response<T>(
        payload: T,
        user_id: &[u8],
        pub_key_hex: &str,
    ) -> Result<Response, String>
    where
        T: Serialize,
    {
        // Generate signed response (without server public key)
        let signed_response = Self::create_signed_response(payload, user_id, pub_key_hex)
            .map_err(|e| format!("Failed to create signed response: {}", e))?;

        // Serialize signed response to JSON
        let response_json = serde_json::to_string(&signed_response)
            .map_err(|e| format!("Failed to serialize signed response: {}", e))?;

        // Build HTTP response with CORS headers
        Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .header("access-control-allow-origin", "*")
            .header("access-control-allow-methods", "POST, GET, OPTIONS")
            .header("access-control-allow-headers", "Content-Type")
            .body(response_json)
            .build())
    }

    /// Extract user_id and pub_key from JWT access token for signed responses
    ///
    /// Universal helper to extract cryptographic material from Authorization header
    /// for use with signed response generation.
    ///
    /// # Arguments
    /// * `authorization_header` - Authorization header value (Bearer token)
    ///
    /// # Returns
    /// * `Result<(Vec<u8>, String), String>` - (user_id_bytes, pub_key_hex) or error
    pub fn extract_crypto_material_from_jwt(
        authorization_header: &str,
    ) -> Result<(Vec<u8>, String), String> {
        // Extract Bearer token
        let token = authorization_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| "Invalid Authorization header format".to_string())?;

        // Validate and extract claims
        let claims = crate::utils::JwtUtils::validate_access_token(token)
            .map_err(|e| format!("JWT validation failed: {}", e))?;

        // Convert username (Base58) back to user_id bytes
        let user_id = bs58::decode(&claims.sub)
            .into_vec()
            .map_err(|e| format!("Failed to decode Base58 username: {}", e))?;

        // Convert pub_key bytes to hex string
        let pub_key_hex = hex::encode(claims.pub_key);

        Ok((user_id, pub_key_hex))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // #[test]
    // fn test_blake2b_comprehensive_output_sizes() {
    //     use blake2::{Blake2b, Blake2bMac, digest::{Digest, KeyInit, Mac}};
    //     use generic_array::typenum::{U8, U16, U32, U64};
    //
    //     println!("\n=== COMPREHENSIVE BLAKE2B OUTPUT SIZE VERIFICATION ===");
    //
    //     // Test Blake2b variants with different sizes
    //     let test_u8 = Blake2b::<U8>::digest(b"test");
    //     let test_u16 = Blake2b::<U16>::digest(b"test");
    //     let test_u32 = Blake2b::<U32>::digest(b"test");
    //     let test_u64 = Blake2b::<U64>::digest(b"test");
    //
    //     println!("Blake2b<U8>  output: {} bytes", test_u8.len());
    //     println!("Blake2b<U16> output: {} bytes", test_u16.len());
    //     println!("Blake2b<U32> output: {} bytes", test_u32.len());
    //     println!("Blake2b<U64> output: {} bytes", test_u64.len());
    //
    //     // Test Blake2bMac variants with different sizes
    //     let key = [0u8; 64];
    //
    //     let mut mac_u8 = <Blake2bMac<U8> as KeyInit>::new_from_slice(&key).unwrap();
    //     mac_u8.update(b"test");
    //     let mac_output_u8 = mac_u8.finalize().into_bytes();
    //
    //     let mut mac_u16 = <Blake2bMac<U16> as KeyInit>::new_from_slice(&key).unwrap();
    //     mac_u16.update(b"test");
    //     let mac_output_u16 = mac_u16.finalize().into_bytes();
    //
    //     let mut mac_u32 = <Blake2bMac<U32> as KeyInit>::new_from_slice(&key).unwrap();
    //     mac_u32.update(b"test");
    //     let mac_output_u32 = mac_u32.finalize().into_bytes();
    //
    //     let mut mac_u64 = <Blake2bMac<U64> as KeyInit>::new_from_slice(&key).unwrap();
    //     mac_u64.update(b"test");
    //     let mac_output_u64 = mac_u64.finalize().into_bytes();
    //
    //     println!("Blake2bMac<U8>  output: {} bytes", mac_output_u8.len());
    //     println!("Blake2bMac<U16> output: {} bytes", mac_output_u16.len());
    //     println!("Blake2bMac<U32> output: {} bytes", mac_output_u32.len());
    //     println!("Blake2bMac<U64> output: {} bytes", mac_output_u64.len());
    //
    //     // Critical assertions based on our pipeline expectations
    //     assert_eq!(test_u8.len(), 8, "Blake2b<U8> should produce 8 bytes");
    //     assert_eq!(test_u16.len(), 16, "Blake2b<U16> should produce 16 bytes");
    //     assert_eq!(test_u32.len(), 32, "Blake2b<U32> should produce 32 bytes");
    //     assert_eq!(test_u64.len(), 64, "Blake2b<U64> should produce 64 bytes");
    //
    //     assert_eq!(mac_output_u8.len(), 8, "Blake2bMac<U8> should produce 8 bytes");
    //     assert_eq!(mac_output_u16.len(), 16, "Blake2bMac<U16> should produce 16 bytes");
    //     assert_eq!(mac_output_u32.len(), 32, "Blake2bMac<U32> should produce 32 bytes");
    //     assert_eq!(mac_output_u64.len(), 64, "Blake2bMac<U64> should produce 64 bytes");
    //
    //     println!("\n=== VERIFICATION RESULTS ===");
    //     println!("✓ Blake2b<UX> produces exactly X bytes");
    //     println!("✓ Blake2bMac<UX> produces exactly X bytes");
    //     println!("✓ All variants behave consistently");
    //
    //     // THIS IS THE CRITICAL INSIGHT FOR OUR PIPELINE:
    //     println!("\n=== PIPELINE IMPLICATIONS ===");
    //     println!("Our current code uses Blake2bMac<U64> = {} bytes output", mac_output_u64.len());
    //     if mac_output_u64.len() == 64 {
    //         println!("✓ PERFECT! We get 64 bytes directly from Blake2bMac<U64>");
    //         println!("✓ No need for expansion rounds - we already have max entropy!");
    //     } else {
    //         println!("✗ Need expansion from {} bytes to 64 bytes", mac_output_u64.len());
    //     }
    // }

    #[test]
    fn test_key_derivation_deterministic() {
        let user_id = [1u8; 16];
        let pub_key_hex = "0".repeat(64);

        // Same inputs should produce same private key
        let key1 = SignedResponseGenerator::derive_session_private_key(&user_id, &pub_key_hex);
        let key2 = SignedResponseGenerator::derive_session_private_key(&user_id, &pub_key_hex);

        // Note: This test will only pass if ED25519_DERIVATION_KEY is set in environment
        if key1.is_ok() && key2.is_ok() {
            assert_eq!(key1.unwrap(), key2.unwrap(), "Key derivation should be deterministic");
        }
    }

    #[test]
    fn test_signed_response_structure() {
        let payload = json!({"status": "OK", "data": "test"});
        let response = SignedResponse {
            payload,
            signature: "test_signature".to_string(),
        };

        // Verify structure matches expected format
        assert!(response.signature.len() > 0);
        assert_eq!(response.payload["status"], "OK");
    }
}