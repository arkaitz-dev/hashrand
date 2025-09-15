//! Ed25519 Digital Signature Verification Module
//!
//! Provides Ed25519 signature verification functionality for magic link authentication.
//! Uses ed25519-dalek for cryptographically secure signature verification.

use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use hex;
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

/// Ed25519 operations for signature verification
pub struct Ed25519Utils;

impl Ed25519Utils {
    /// Verify Ed25519 signature against message
    ///
    /// # Arguments
    /// * `message` - The original message that was signed (as bytes)
    /// * `signature_hex` - The Ed25519 signature as hex string (128 chars)
    /// * `public_key_hex` - The Ed25519 public key as hex string (64 chars)
    ///
    /// # Returns
    /// * `SignatureVerificationResult` - Verification result
    pub fn verify_signature(
        message: &[u8],
        signature_hex: &str,
        public_key_hex: &str,
    ) -> SignatureVerificationResult {
        // Validate input lengths
        if public_key_hex.len() != 64 {
            println!("🔍 DEBUG Ed25519: Invalid public key hex length: {} (expected 64)", public_key_hex.len());
            return SignatureVerificationResult::MalformedPublicKey;
        }

        if signature_hex.len() != 128 {
            println!("🔍 DEBUG Ed25519: Invalid signature hex length: {} (expected 128)", signature_hex.len());
            return SignatureVerificationResult::MalformedSignature;
        }

        if message.is_empty() {
            println!("🔍 DEBUG Ed25519: Empty message provided for verification");
            return SignatureVerificationResult::MalformedMessage;
        }

        // Decode public key from hex
        let public_key_bytes = match hex::decode(public_key_hex) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("🔍 DEBUG Ed25519: Failed to decode public key hex: {}", e);
                return SignatureVerificationResult::MalformedPublicKey;
            }
        };

        if public_key_bytes.len() != 32 {
            println!("🔍 DEBUG Ed25519: Invalid public key byte length: {} (expected 32)", public_key_bytes.len());
            return SignatureVerificationResult::MalformedPublicKey;
        }

        // Decode signature from hex
        let signature_bytes = match hex::decode(signature_hex) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("🔍 DEBUG Ed25519: Failed to decode signature hex: {}", e);
                return SignatureVerificationResult::MalformedSignature;
            }
        };

        if signature_bytes.len() != 64 {
            println!("🔍 DEBUG Ed25519: Invalid signature byte length: {} (expected 64)", signature_bytes.len());
            return SignatureVerificationResult::MalformedSignature;
        }

        // Create Ed25519 verifying key
        let verifying_key = match VerifyingKey::from_bytes(&public_key_bytes.try_into().unwrap()) {
            Ok(key) => key,
            Err(e) => {
                println!("🔍 DEBUG Ed25519: Failed to create verifying key: {}", e);
                return SignatureVerificationResult::MalformedPublicKey;
            }
        };

        // Create Ed25519 signature
        let signature = Signature::from_bytes(&signature_bytes.try_into().unwrap());

        // Verify signature
        match verifying_key.verify(message, &signature) {
            Ok(()) => {
                println!("🔍 DEBUG Ed25519: Signature verification successful");
                SignatureVerificationResult::Valid
            }
            Err(e) => {
                println!("🔍 DEBUG Ed25519: Signature verification failed: {}", e);
                SignatureVerificationResult::Invalid
            }
        }
    }

    /// Verify Ed25519 signature for string message
    ///
    /// # Arguments
    /// * `message` - The original message that was signed (as string)
    /// * `signature_hex` - The Ed25519 signature as hex string (128 chars)
    /// * `public_key_hex` - The Ed25519 public key as hex string (64 chars)
    ///
    /// # Returns
    /// * `SignatureVerificationResult` - Verification result
    pub fn verify_signature_string(
        message: &str,
        signature_hex: &str,
        public_key_hex: &str,
    ) -> SignatureVerificationResult {
        let message_bytes = message.as_bytes();
        Self::verify_signature(message_bytes, signature_hex, public_key_hex)
    }

    /// Convert public key bytes to hex string
    ///
    /// # Arguments
    /// * `public_key_bytes` - Ed25519 public key as 32 bytes
    ///
    /// # Returns
    /// * `Result<String, String>` - Hex string or error
    #[allow(dead_code)]
    pub fn public_key_to_hex(public_key_bytes: &[u8; 32]) -> String {
        hex::encode(public_key_bytes)
    }

    /// Convert hex string to public key bytes
    ///
    /// # Arguments
    /// * `public_key_hex` - Ed25519 public key as hex string (64 chars)
    ///
    /// # Returns
    /// * `Result<[u8; 32], String>` - Public key bytes or error
    #[allow(dead_code)]
    pub fn public_key_from_hex(public_key_hex: &str) -> Result<[u8; 32], String> {
        if public_key_hex.len() != 64 {
            return Err(format!("Invalid public key hex length: {} (expected 64)", public_key_hex.len()));
        }

        let bytes = hex::decode(public_key_hex)
            .map_err(|e| format!("Failed to decode public key hex: {}", e))?;

        if bytes.len() != 32 {
            return Err(format!("Invalid public key byte length: {} (expected 32)", bytes.len()));
        }

        let mut public_key_bytes = [0u8; 32];
        public_key_bytes.copy_from_slice(&bytes);
        Ok(public_key_bytes)
    }

    /// Validate Ed25519 signature format without verification
    ///
    /// # Arguments
    /// * `signature_data` - Ed25519SignatureData to validate
    ///
    /// # Returns
    /// * `Result<(), String>` - Ok if format is valid, error message otherwise
    #[allow(dead_code)]
    pub fn validate_signature_format(signature_data: &Ed25519SignatureData) -> Result<(), String> {
        // Validate public key format
        if signature_data.public_key.len() != 64 {
            return Err(format!("Invalid public key length: {} (expected 64)", signature_data.public_key.len()));
        }

        if hex::decode(&signature_data.public_key).is_err() {
            return Err("Invalid public key hex format".to_string());
        }

        // Validate signature format
        if signature_data.signature.len() != 128 {
            return Err(format!("Invalid signature length: {} (expected 128)", signature_data.signature.len()));
        }

        if hex::decode(&signature_data.signature).is_err() {
            return Err("Invalid signature hex format".to_string());
        }

        Ok(())
    }

    /// Create message for signing: email + public_key + next (if present)
    ///
    /// # Arguments
    /// * `email` - User email
    /// * `public_key_hex` - Ed25519 public key as hex string
    /// * `next` - Optional next parameter
    ///
    /// # Returns
    /// * `String` - Message to be signed
    pub fn create_sign_message(email: &str, public_key_hex: &str, next: Option<&str>) -> String {
        if let Some(next_param) = next {
            format!("{}{}{}", email, public_key_hex, next_param)
        } else {
            format!("{}{}", email, public_key_hex)
        }
    }

    /// Verify complete magic link request signature
    ///
    /// # Arguments
    /// * `email` - User email from request
    /// * `public_key_hex` - Ed25519 public key as hex string
    /// * `next` - Optional next parameter
    /// * `signature_hex` - Ed25519 signature as hex string
    ///
    /// # Returns
    /// * `SignatureVerificationResult` - Verification result
    pub fn verify_magic_link_request(
        email: &str,
        public_key_hex: &str,
        next: Option<&str>,
        signature_hex: &str,
    ) -> SignatureVerificationResult {
        let message = Self::create_sign_message(email, public_key_hex, next);
        println!("🔍 DEBUG Ed25519: Verifying magic link request for email: {}", email);
        println!("🔍 DEBUG Ed25519: Message to verify: {}", message);
        Self::verify_signature_string(&message, signature_hex, public_key_hex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_key_conversion() {
        let original_bytes = [1u8; 32];
        let hex = Ed25519Utils::public_key_to_hex(&original_bytes);
        assert_eq!(hex.len(), 64);

        let converted_bytes = Ed25519Utils::public_key_from_hex(&hex).unwrap();
        assert_eq!(original_bytes, converted_bytes);
    }

    #[test]
    fn test_signature_format_validation() {
        let valid_data = Ed25519SignatureData {
            public_key: "0".repeat(64),
            signature: "0".repeat(128),
        };
        assert!(Ed25519Utils::validate_signature_format(&valid_data).is_ok());

        let invalid_data = Ed25519SignatureData {
            public_key: "0".repeat(63), // Too short
            signature: "0".repeat(128),
        };
        assert!(Ed25519Utils::validate_signature_format(&invalid_data).is_err());
    }

    #[test]
    fn test_create_sign_message() {
        let email = "test@example.com";
        let public_key = "a".repeat(64);

        let message_without_next = Ed25519Utils::create_sign_message(email, &public_key, None);
        assert_eq!(message_without_next, format!("{}{}", email, public_key));

        let message_with_next = Ed25519Utils::create_sign_message(email, &public_key, Some("/dashboard"));
        assert_eq!(message_with_next, format!("{}{}/dashboard", email, public_key));
    }
}