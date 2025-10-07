pub mod api_key;
pub mod custom;
pub mod login;
pub mod mnemonic;
pub mod password;
pub mod shared_secret;
pub mod version;

// Test module (DEV-MODE ONLY - eliminated in production builds)
#[cfg(feature = "dev-mode")]
pub mod test;

pub use api_key::handle_api_key_request;
pub use login::handle_login;
pub use mnemonic::handle_mnemonic_request;
pub use password::handle_password_request;
pub use shared_secret::{
    handle_confirm_read, handle_create_secret, handle_delete_secret, handle_retrieve_secret,
};
pub use version::handle_version;

// Test endpoint export (DEV-MODE ONLY)
#[cfg(feature = "dev-mode")]
pub use test::handle_dry_run_toggle;
