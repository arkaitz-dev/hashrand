//! Magic link email delivery logic
//!
//! Provides email sending functionality with fallback to console logging
//! for development and testing purposes.

use chrono::{DateTime, Utc};
use tracing::{info, warn};

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
                // println!("✅ Email sent successfully to: {}", email);
                info!("✅ Email sent successfully to: {}", email);
                Ok(())
            }
            Err(e) => {
                // println!(
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

    /// Log detailed email content to console as fallback
    ///
    /// Provides a development-friendly fallback when email sending fails,
    /// showing the complete email content and debugging information.
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
        info!("\n🔗 === EMAIL FALLBACK (DEVELOPMENT MODE) ===");
        info!("📧 TO: {}", email);
        info!("📬 FROM: HashRand <noreply@hashrand.dev>");
        info!("📝 SUBJECT: Your Magic Link for HashRand");
        info!("📄 EMAIL BODY:");
        info!("──────────────────────────────────────────────────");
        info!("Hi there!");
        info!("");
        info!("You requested access to HashRand. Click the link below to sign in:");
        info!("");
        info!("🔗 {}", magic_link);
        info!("");
        info!(
            "This link will expire at: {}",
            magic_expires_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
        info!("");
        info!("If you didn't request this, you can safely ignore this email.");
        info!("");
        info!("Best regards,");
        info!("The HashRand Team");
        info!("──────────────────────────────────────────────────");
        info!("🔧 DEVELOPMENT INFO:");
        info!("   • UI Host: {:?}", ui_host);
        info!("   • Final Host URL: {}", final_host_url);
        info!(
            "   • Token expires: {}",
            magic_expires_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
        info!("   • Email send error: {}", error_msg);
        info!("═══════════════════════════════════════════════════\n");
    }
}
