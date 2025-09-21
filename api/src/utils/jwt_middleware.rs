//! JWT Authentication middleware for protected endpoints - Main API exports

// Re-export public API from specialized modules
pub use super::jwt_middleware_core::{requires_authentication, with_auth_and_renewal};

// All implementation has been moved to specialized modules
// This file now serves as the main API export point
