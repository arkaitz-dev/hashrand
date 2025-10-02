//! Middleware for protected endpoints with universal signed request validation
//!
//! Provides JWT token validation + Ed25519 signature verification
//! for all authenticated API endpoints
//!
//! # Module Organization
//! - `types`: Core type definitions (ProtectedEndpointResult, ProtectedSignedRequest)
//! - `errors`: DRY-unified error response creation
//! - `helpers`: Payload processing utilities
//! - `middleware`: Core JWT + Ed25519 validation logic
//! - `macros`: Helper macros for protected endpoint handlers

// Module declarations
mod errors;
mod helpers;
mod macros;
mod middleware;
mod types;

// Public re-exports
pub use helpers::{extract_seed_from_payload, payload_to_params};
pub use middleware::ProtectedEndpointMiddleware;
pub use types::ProtectedEndpointResult;

// Macro is already exported via #[macro_export] in macros.rs

#[cfg(test)]
mod tests {
    // Original commented tests preserved for backward compatibility

    // #[test]
    // fn test_jwt_public_key_extractor() {
    //     let extractor = JwtPublicKeyExtractor {
    //         public_key: "test_key_123".to_string(),
    //     };
    //
    //     let result = extractor.extract_public_key().unwrap();
    //     assert_eq!(result, "test_key_123");
    // }
}
