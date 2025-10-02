//! Magic link validation operations
//!
//! Provides validation and consumption functions for encrypted magic links
//! with complete payload decryption and JWT token validation.

// Submodules
mod extraction;
mod utilities;
mod validation;

// Re-export public API
pub use validation::MagicLinkValidation;
