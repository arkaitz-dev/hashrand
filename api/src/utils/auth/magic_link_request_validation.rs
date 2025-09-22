//! Magic link request validation logic
//!
//! Provides validation functions for magic link requests including
//! rate limiting, email validation, and signature verification.

use spin_sdk::http::{Request, Response};

use super::types::{ErrorResponse, MagicLinkRequest, MagicLinkSignedRequest};
use crate::utils::{
    SignedRequestValidator, check_rate_limit,
    ed25519::Ed25519Utils, extract_client_ip, validate_email,
};

/// Magic link request validation operations
pub struct MagicLinkRequestValidation;

impl MagicLinkRequestValidation {
    /// Check rate limiting for authentication requests
    ///
    /// # Arguments
    /// * `req` - HTTP request to extract client IP from
    ///
    /// # Returns
    /// * `Result<(), Response>` - Ok if rate limit passed, Error response if exceeded
    pub fn check_rate_limiting(req: &Request) -> Result<(), Response> {
        let client_ip = extract_client_ip(req.headers());
        if let Err(e) = check_rate_limit(&client_ip) {
            return Err(Response::builder()
                .status(429) // Too Many Requests
                .header("content-type", "application/json")
                .header("retry-after", "900") // 15 minutes in seconds
                .body(
                    serde_json::to_string(&ErrorResponse {
                        error: format!("Rate limited: {}", e),
                    })
                    .unwrap_or_default(),
                )
                .build());
        }
        Ok(())
    }

    /// Validate email format using strict validation
    ///
    /// # Arguments
    /// * `email` - Email address to validate
    ///
    /// # Returns
    /// * `Result<(), Response>` - Ok if email valid, Error response if invalid
    pub fn validate_email_format(email: &str) -> Result<(), Response> {
        if let Err(e) = validate_email(email) {
            return Err(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(
                    serde_json::to_string(&ErrorResponse {
                        error: format!("Invalid email: {}", e),
                    })
                    .unwrap_or_default(),
                )
                .build());
        }
        Ok(())
    }

    /// Validate Ed25519 signature for magic link request
    ///
    /// # Arguments
    /// * `magic_request` - Magic link request with Ed25519 signature
    ///
    /// # Returns
    /// * `Result<(), Response>` - Ok if signature valid, Error response if invalid
    pub fn validate_ed25519_signature(magic_request: &MagicLinkRequest) -> Result<(), Response> {
        let pub_key_hex = &magic_request.pub_key;
        let signature_hex = &magic_request.signature;

        println!(
            "🔍 DEBUG Ed25519: Verifying signature for email: {}",
            magic_request.email
        );
        println!("🔍 DEBUG Ed25519: Public key: {}...", &pub_key_hex[..20]);
        println!("🔍 DEBUG Ed25519: Signature: {}...", &signature_hex[..20]);

        // Verify Ed25519 signature for magic link request
        let verification_result = Ed25519Utils::verify_magic_link_request(
            &magic_request.email,
            pub_key_hex,
            magic_request.next.as_deref().unwrap_or("/"),
            signature_hex,
        );

        if verification_result != crate::utils::ed25519::SignatureVerificationResult::Valid {
            println!(
                "🔍 DEBUG Ed25519: Signature verification failed: {:?}",
                verification_result
            );
            return Err(Response::builder()
                .status(400)
                .header("content-type", "application/json")
                .body(
                    serde_json::to_string(&ErrorResponse {
                        error: "Invalid Ed25519 signature: Authentication failed".to_string(),
                    })
                    .unwrap_or_default(),
                )
                .build());
        }

        println!("🔍 DEBUG Ed25519: Signature verification successful!");
        Ok(())
    }

    /// Validate signed request with Ed25519 signature
    ///
    /// # Arguments
    /// * `signed_request` - Signed magic link request
    ///
    /// # Returns
    /// * `Result<String, Response>` - Ok with public key if valid, Error response if invalid
    pub fn validate_signed_request(
        request: &Request,
        signed_request: &MagicLinkSignedRequest,
    ) -> Result<String, Response> {
        // Use universal validation that automatically detects pub_key source
        match SignedRequestValidator::validate_universal(signed_request, request) {
            Ok(pub_key_hex) => {
                println!("✅ Universal SignedRequest validation successful for email: {}", signed_request.payload.email);
                Ok(pub_key_hex)
            },
            Err(e) => {
                println!("❌ Universal SignedRequest validation failed: {}", e);
                Err(Response::builder()
                    .status(400)
                    .header("content-type", "application/json")
                    .body(
                        serde_json::to_string(&ErrorResponse {
                            error: format!("Invalid Ed25519 signature: {}", e),
                        })
                        .unwrap_or_default(),
                    )
                    .build())
            }
        }
    }
}
