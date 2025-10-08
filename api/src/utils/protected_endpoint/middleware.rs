//! Protected Endpoint Core Middleware Logic
//!
//! JWT + Ed25519 signature validation for protected endpoints

use serde::{Deserialize, Serialize};
use spin_sdk::http::{Request, Response};
use tracing::{debug, error};

use super::super::SignedRequestValidator;
use super::errors;
use super::types::{ProtectedEndpointResult, ProtectedSignedRequest};

/// Protected endpoint middleware
pub struct ProtectedEndpointMiddleware;

impl ProtectedEndpointMiddleware {
    /// Validate protected endpoint request with JWT + Ed25519 signature
    ///
    /// # Arguments
    /// * `req` - HTTP request with Authorization header
    /// * `body_bytes` - Request body as bytes
    ///
    /// # Returns
    /// * `Result<ProtectedEndpointResult<T>, Response>` - Validated payload + JWT claims or error response
    pub async fn validate_request<T>(
        req: &Request,
        body_bytes: &[u8],
    ) -> Result<ProtectedEndpointResult<T>, Response>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        // Step 1: Validate JWT token and extract pub_key
        let (user_id, pub_key_hex) = match Self::extract_jwt_info(req) {
            Ok(info) => info,
            Err(response) => return Err(response),
        };

        // Step 2: Parse signed request from body (now with Base64-encoded JSON payload)
        let signed_request: ProtectedSignedRequest = match serde_json::from_slice(body_bytes) {
            Ok(req) => req,
            Err(e) => {
                // println!("🔍 DEBUG: Failed to parse SignedRequest: {}", e);
                debug!("🔍 DEBUG: Failed to parse SignedRequest: {}", e);
                return Err(errors::bad_request("Invalid SignedRequest structure"));
            }
        };

        // Step 3: Validate Ed25519 signature using JWT pub_key (Base64 payload)
        if let Err(e) = SignedRequestValidator::validate_base64_payload(
            &signed_request.payload,
            &signed_request.signature,
            &pub_key_hex,
        ) {
            // println!("🔍 DEBUG: SignedRequest validation failed: {}", e);
            debug!("🔍 DEBUG: SignedRequest validation failed: {}", e);
            return Err(errors::unauthorized(format!("Invalid signature: {}", e)));
        }

        // println!(
        //     "✅ Protected endpoint validation successful for user: {}",
        //     user_id
        // );
        debug!(
            "✅ Protected endpoint validation successful for user: {}",
            user_id
        );

        // Step 4: Deserialize Base64-encoded JSON payload to typed structure
        let deserialized_payload: T =
            match SignedRequestValidator::deserialize_base64_payload(&signed_request.payload) {
                Ok(payload) => payload,
                Err(e) => {
                    // println!("❌ DEBUG: Failed to deserialize Base64 payload: {}", e);
                    error!("❌ Failed to deserialize Base64 payload: {}", e);
                    return Err(errors::bad_request(format!(
                        "Invalid payload format: {}",
                        e
                    )));
                }
            };

        // println!("✅ Base64-encoded JSON payload deserialized successfully");
        debug!("✅ Base64-encoded JSON payload deserialized successfully");

        Ok(ProtectedEndpointResult {
            payload: deserialized_payload,
        })
    }

    /// Extract JWT info (user_id + pub_key_hex) from Authorization header
    fn extract_jwt_info(req: &Request) -> Result<(String, String), Response> {
        // SECURITY: Validate that request doesn't contain both Authorization header AND refresh cookie
        if let Err(e) = crate::utils::validate_no_simultaneous_tokens(req) {
            // println!(
            //     "🚨 [SECURITY VIOLATION] Protected endpoint received request with both tokens"
            // );
            error!("🚨 [SECURITY VIOLATION] Protected endpoint received request with both tokens");
            return Err(errors::forbidden(e));
        }

        // Get Authorization header
        let auth_header = req
            .header("authorization")
            .and_then(|h| h.as_str())
            .ok_or_else(|| errors::unauthorized("Missing Authorization header"))?;

        // Extract Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| errors::unauthorized("Invalid Authorization header format"))?;

        // Validate JWT token and extract claims
        let claims = crate::utils::JwtUtils::validate_access_token(token).map_err(|e| {
            // println!("🔍 DEBUG: JWT validation failed: {}", e);
            debug!("🔍 DEBUG: JWT validation failed: {}", e);
            errors::unauthorized(format!("Invalid JWT token: {}", e))
        })?;

        // Convert pub_key bytes to hex string
        let pub_key_hex = hex::encode(claims.pub_key);

        // Use username (sub field) as user identifier
        let user_id = claims.sub;

        Ok((user_id, pub_key_hex))
    }
}
