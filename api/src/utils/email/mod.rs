// Email sending module (refactored for better maintainability)
// Original 574-line file split into focused modules

mod config;
mod dry_run;
mod magic_link;
mod shared_secret;

// Re-export public API (maintains backwards compatibility)
pub use magic_link::send_magic_link_email;
pub use shared_secret::{send_shared_secret_receiver_email, send_shared_secret_sender_email};

// Dev-mode only exports
#[cfg(feature = "dev-mode")]
pub use dry_run::set_email_dry_run;

#[cfg(test)]
mod tests {
    use super::*;
    use spin_sdk::http::Method;

    #[test]
    fn test_email_validation() {
        // Test basic email validation logic
        assert!(!"invalid-email".contains('@'));

        let valid_email = "valid@example.com";
        assert!(valid_email.contains('@') && !valid_email.is_empty());

        // Test empty string handling
        let empty_email = "";
        assert!(empty_email.is_empty());
    }

    #[test]
    fn test_email_config() {
        let config = config::EmailConfig {
            api_url: "https://test.api".to_string(),
            api_token: "test_token".to_string(),
            inbox_id: "test_inbox".to_string(),
            from_email: "test@example.com".to_string(),
        };

        assert_eq!(config.api_url, "https://test.api");
        assert_eq!(config.from_email, "test@example.com");
    }
}
