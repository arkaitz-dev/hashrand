//! JWT token utilities for authentication
//!
//! Provides functions for creating and validating JWT access and refresh tokens
//! with proper expiration times and security claims.
//!
//! This module has been refactored into smaller, focused submodules for better
//! maintainability while preserving the original API for backwards compatibility.

pub mod config;
pub mod crypto;
pub mod custom_tokens;
pub mod custom_token_api;
pub mod custom_token_compat;
pub mod custom_token_crypto;
pub mod custom_token_encryption;
pub mod custom_token_serialization;
pub mod custom_token_types;
pub mod magic_links;
pub mod tokens;
pub mod types;
pub mod utils;

// Re-export main API
pub use utils::JwtUtils;
