//! Magic link email delivery logic
//!
//! Provides email sending functionality with fallback to console logging
//! for development and testing purposes.

use chrono::{DateTime, Utc};
use tracing::warn;

use crate::utils::send_magic_link_email;

/// Magic link email delivery operations
pub struct MagicLinkEmailDelivery;

impl MagicLinkEmailDelivery {
    /// Send magic link email with fallback to console logging
    ///
    /// Attempts to send email via Mailtrap. If that fails, falls back to
    /// console logging with a detailed development-friendly format.
    ///
    /// # Arguments
    /// * `email` - Recipient email address
    /// * `magic_link` - Complete magic link URL
    /// * `email_lang` - Optional email language (e.g. "en", "es")
    /// * `ui_host` - Optional UI host for debugging info
    /// * `final_host_url` - Final determined host URL for debugging
    /// * `magic_expires_at` - Token expiration timestamp for display
    ///
    /// # Returns
    /// * `Result<(), ()>` - Always succeeds (fallback ensures delivery)
    pub async fn send_with_fallback(
        email: &str,
        magic_link: &str,
        email_lang: Option<&str>,
        ui_host: Option<&str>,
        final_host_url: &str,
        magic_expires_at: DateTime<Utc>,
    ) -> Result<(), ()> {
        // Try to send email via Mailtrap
        match send_magic_link_email(email, magic_link, email_lang).await {
            Ok(()) => {
                // Note: Email sending with URL is already logged in email.rs
                Ok(())
            }
            Err(e) => {
                //     "⚠️ Email sending failed, falling back to console logging: {}",
                //     e
                // );
                warn!(
                    "⚠️ Email sending failed, falling back to console logging: {}",
                    e
                );

                // Fallback: simulate email content in console (development mode)
                Self::log_email_fallback(
                    email,
                    magic_link,
                    ui_host,
                    final_host_url,
                    magic_expires_at,
                    &e.to_string(),
                );

                Ok(())
            }
        }
    }

    /// Log email fallback as debug info (verbose)
    ///
    /// When email sending fails, log complete email content for development debugging.
    /// Only shown when RUST_LOG=debug is enabled.
    ///
    /// # Arguments
    /// * `email` - Recipient email address
    /// * `magic_link` - Complete magic link URL
    /// * `ui_host` - Optional UI host for debugging
    /// * `final_host_url` - Final determined host URL
    /// * `magic_expires_at` - Token expiration timestamp
    /// * `error_msg` - Original email sending error message
    fn log_email_fallback(
        email: &str,
        magic_link: &str,
        ui_host: Option<&str>,
        final_host_url: &str,
        magic_expires_at: DateTime<Utc>,
        error_msg: &str,
    ) {
        use tracing::debug;

        debug!("\n🔗 === EMAIL FALLBACK (DEVELOPMENT MODE) ===");
        debug!("📧 TO: {}", email);
        debug!("📬 FROM: HashRand <noreply@hashrand.dev>");
        debug!("📝 SUBJECT: Your Magic Link for HashRand");
        debug!("📄 EMAIL BODY:");
        debug!("──────────────────────────────────────────────────");
        debug!("Hi there!");
        debug!("");
        debug!("You requested access to HashRand. Click the link below to sign in:");
        debug!("");
        debug!("🔗 {}", magic_link);
        debug!("");
        debug!(
            "This link will expire at: {}",
            magic_expires_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
        debug!("");
        debug!("If you didn't request this, you can safely ignore this email.");
        debug!("");
        debug!("Best regards,");
        debug!("The HashRand Team");
        debug!("──────────────────────────────────────────────────");
        debug!("🔧 DEVELOPMENT INFO:");
        debug!("   • UI Host: {:?}", ui_host);
        debug!("   • Final Host URL: {}", final_host_url);
        debug!(
            "   • Token expires: {}",
            magic_expires_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
        debug!("   • Email send error: {}", error_msg);
        debug!("═══════════════════════════════════════════════════\n");
    }
}
