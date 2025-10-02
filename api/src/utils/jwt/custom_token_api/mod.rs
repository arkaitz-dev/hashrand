//! Custom token API operations - High-level API functions for token creation and validation

// Submodules
mod conversion;
mod creation;
mod validation;

// Import compatibility module to make trait implementations available
#[allow(unused_imports)]
use super::custom_token_compat;

// Re-export public API

// Token creation
pub use creation::{
    create_custom_access_token, create_custom_access_token_from_username,
    create_custom_access_token_from_username_with_refresh_context, create_custom_refresh_token,
    create_custom_refresh_token_from_username,
};

// Token validation
pub use validation::{validate_custom_access_token, validate_custom_refresh_token};

// Conversion utilities (not exported - internal only)
// pub use conversion::username_to_user_id;
