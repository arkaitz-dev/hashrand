//! JWT middleware authentication logic - Bearer token validation and auth flow
//!
//! # Module Organization
//! - `helpers`: DRY utility functions (username decoding, error creation)
//! - `bearer_validator`: Bearer token extraction and validation
//! - `cookie_refresh`: Automatic refresh from HTTP-only cookies with 2/3 system

mod bearer_validator;
mod cookie_refresh;
mod helpers;

// Public API
pub use bearer_validator::validate_bearer_token;
