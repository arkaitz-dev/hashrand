//! Protected Endpoint Type Definitions
//!
//! Core types for protected endpoint middleware

use super::super::SignedRequest;

/// Universal payload wrapper for protected endpoints
/// CORRECTED: No longer generic since SignedRequest uses Base64-encoded JSON payload
pub type ProtectedSignedRequest = SignedRequest;

/// Protected endpoint middleware result
pub struct ProtectedEndpointResult<T> {
    pub payload: T,
}
