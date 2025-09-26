//! Magic link request validation logic
//!
//! Provides validation functions for magic link requests including
//! rate limiting, email validation, and signature verification.

use spin_sdk::http::{Request, Response};

use super::types::{ErrorResponse, MagicLinkSignedRequest};
use crate::utils::{SignedRequestValidator, check_rate_limit, extract_client_ip, validate_email};

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
    // DELETED: Legacy function validate_ed25519_signature removed - was completely unused, replaced by universal SignedRequest validation
    fn _deleted_validate_ed25519_signature() {}

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
                println!("✅ Universal SignedRequest validation successful (Base64 JSON payload)");
                Ok(pub_key_hex)
            }
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
