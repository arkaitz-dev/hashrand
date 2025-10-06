//! Ed25519 Digital Signature Verification Module
//!
//! Provides Ed25519 signature verification functionality for magic link authentication.
//! Uses ed25519-dalek for cryptographically secure signature verification.
//!
//! # Module Organization
//! - `types`: Core type definitions (SignatureVerificationResult, Ed25519SignatureData)
//! - `conversion`: Hex encoding/decoding utilities (DRY-unified)
//! - `verification`: Core cryptographic verification logic

// Module declarations
mod conversion;
mod types;
mod verification;

// Public re-exports
pub use types::{Ed25519SignatureData, SignatureVerificationResult};

// Public API wrapper struct (maintains backward compatibility)
pub struct Ed25519Utils;

impl Ed25519Utils {
    /// Verify Ed25519 signature against message
    ///
    /// # Arguments
    /// * `message` - The original message that was signed (as bytes)
    /// * `signature_base58` - The Ed25519 signature as base58 string (~88 chars)
    /// * `public_key_hex` - The Ed25519 public key as hex string (64 chars)
    ///
    /// # Returns
    /// * `SignatureVerificationResult` - Verification result
    pub fn verify_signature(
        message: &[u8],
        signature_base58: &str,
        public_key_hex: &str,
    ) -> SignatureVerificationResult {
        verification::verify_signature(message, signature_base58, public_key_hex)
    }

    /// Verify Ed25519 signature for string message
    ///
    /// # Arguments
    /// * `message` - The original message that was signed (as string)
    /// * `signature_base58` - The Ed25519 signature as base58 string (~88 chars)
    /// * `public_key_hex` - The Ed25519 public key as hex string (64 chars)
    ///
    /// # Returns
    /// * `SignatureVerificationResult` - Verification result
    pub fn verify_signature_string(
        message: &str,
        signature_base58: &str,
        public_key_hex: &str,
    ) -> SignatureVerificationResult {
        verification::verify_signature_string(message, signature_base58, public_key_hex)
    }

    /// Convert public key bytes to hex string
    ///
    /// # Arguments
    /// * `public_key_bytes` - Ed25519 public key as 32 bytes
    ///
    /// # Returns
    /// * `String` - Hex encoded public key (64 chars)
    #[allow(dead_code)]
    pub fn public_key_to_hex(public_key_bytes: &[u8; 32]) -> String {
        conversion::public_key_to_hex(public_key_bytes)
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
        conversion::public_key_from_hex(public_key_hex)
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
        conversion::validate_signature_data_format(
            &signature_data.public_key,
            &signature_data.signature,
        )
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
            // signature: "0".repeat(128), // Old hex format
            signature: "1".repeat(88), // Base58 format (~88 chars for 64 bytes)
        };
        assert!(Ed25519Utils::validate_signature_format(&valid_data).is_ok());

        let invalid_data = Ed25519SignatureData {
            public_key: "0".repeat(63), // Too short
            // signature: "0".repeat(128),
            signature: "1".repeat(88),
        };
        assert!(Ed25519Utils::validate_signature_format(&invalid_data).is_err());
    }
}
