use anyhow::{Result, anyhow};
use serde_json::json;
use spin_sdk::{
    http::{Method, Request, Response},
    variables,
};
use tracing::{info, error};

use crate::email_templates::render_magic_link_email;

// ==================== DEV-MODE ONLY: Email Dry-Run System ====================
// This entire block is ELIMINATED from production builds (cargo build --no-default-features)
// In development: emails are NOT sent by default (dry-run ON), can be toggled via endpoint
// In production: this code doesn't exist, emails ALWAYS sent

#[cfg(feature = "dev-mode")]
use std::sync::atomic::{AtomicBool, Ordering};

/// Global flag to control dry-run mode (DEV-MODE ONLY)
/// Default: true (emails OFF) - must be explicitly disabled for manual browser testing
/// This static is completely removed from production binaries
#[cfg(feature = "dev-mode")]
static EMAIL_DRY_RUN: AtomicBool = AtomicBool::new(true);

/// Toggle email dry-run mode (DEV-MODE ONLY)
/// This function doesn't exist in production builds
///
/// # Safety
/// Thread-safe using atomic operations. Can be called from multiple threads.
#[cfg(feature = "dev-mode")]
pub fn set_email_dry_run(enabled: bool) {
    EMAIL_DRY_RUN.store(enabled, Ordering::Relaxed);
    // eprintln!(
    //     "📧 [DEV-MODE] Email dry-run: {}",
    //     if enabled {
    //         "ON (emails will NOT be sent)"
    //     } else {
    //         "OFF (emails will be sent)"
    //     }
    // );
    info!(
        "📧 [DEV-MODE] Email dry-run: {}",
        if enabled {
            "ON (emails will NOT be sent)"
        } else {
            "OFF (emails will be sent)"
        }
    );
}

/// Check if email dry-run mode is enabled (DEV-MODE ONLY)
/// Returns: true if dry-run active (don't send emails)
#[cfg(feature = "dev-mode")]
fn is_email_dry_run_enabled() -> bool {
    EMAIL_DRY_RUN.load(Ordering::Relaxed)
}

// ==================== End DEV-MODE Block ====================

/// Email configuration loaded from Spin variables
#[derive(Debug)]
pub struct EmailConfig {
    pub api_url: String,
    pub api_token: String,
    pub inbox_id: String,
    pub from_email: String,
}

impl EmailConfig {
    /// Load email configuration from Spin environment variables
    pub fn from_environment() -> Result<Self> {
        let api_url = variables::get("mailtrap_api_url")
            .map_err(|e| anyhow!("Missing mailtrap_api_url: {}", e))?;
        let api_token = variables::get("mailtrap_api_token")
            .map_err(|e| anyhow!("Missing mailtrap_api_token: {}", e))?;
        let inbox_id = variables::get("mailtrap_inbox_id")
            .map_err(|e| anyhow!("Missing mailtrap_inbox_id: {}", e))?;
        let from_email =
            variables::get("from_email").map_err(|e| anyhow!("Missing from_email: {}", e))?;

        Ok(EmailConfig {
            api_url,
            api_token,
            inbox_id,
            from_email,
        })
    }
}

/// Creates an HTTP request for sending email via Mailtrap API
fn create_email_request(
    config: &EmailConfig,
    recipient_email: &str,
    magic_link: &str,
    language: Option<&str>,
) -> Result<Request> {
    let (subject, html_content, text_content) =
        render_magic_link_email(magic_link, language.unwrap_or("en"));

    // Generate unique Message-ID to prevent spam warnings
    let message_id = format!(
        "<{}.{}@mailer.hashrand.com>",
        chrono::Utc::now()
            .timestamp_nanos_opt()
            .unwrap_or_else(|| {
                // println!("⚠️ CRITICAL: timestamp_nanos_opt() overflow - server clock may be misconfigured (date > year 2262)");
                error!("⚠️ CRITICAL: timestamp_nanos_opt() overflow - server clock may be misconfigured (date > year 2262)");
                chrono::Utc::now()
                    .timestamp_millis()
                    .checked_mul(1_000_000)  // Safe multiply - prevents overflow
                    .unwrap_or(0)  // Final fallback if multiplication would overflow
            }),
        nanoid::nanoid!(8)
    );

    // Create email payload according to Mailtrap API format
    let email_payload = json!({
        "from": {
            "email": config.from_email,
            "name": "HashRand"
        },
        "to": [
            {
                "email": recipient_email,
                "name": recipient_email.split('@').next().unwrap_or("User")
            }
        ],
        "subject": subject,
        "text": text_content,
        "html": html_content,
        "category": "Authentication",
        "headers": {
            "Message-ID": message_id,
            "Content-Type": "text/html; charset=UTF-8",
            "Content-Transfer-Encoding": "quoted-printable"
        }
    });

    // Convert payload to JSON string
    let body_json = serde_json::to_string(&email_payload)
        .map_err(|e| anyhow!("Failed to serialize email payload: {}", e))?;

    // Build full URL - for custom domains, don't append inbox ID
    // For sandbox: https://sandbox.api.mailtrap.io/api/send/INBOX_ID
    // For custom domain: https://send.api.mailtrap.io/api/send
    let full_url = if config.api_url.contains("send.api.mailtrap.io") {
        // Custom domain - use URL as-is without inbox ID
        config.api_url.clone()
    } else {
        // Sandbox - append inbox ID
        format!("{}/{}", config.api_url, config.inbox_id)
    };

    // Build HTTP request using Spin's Request builder
    let request = Request::builder()
        .method(Method::Post)
        .uri(&full_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.api_token))
        .body(body_json)
        .build();

    Ok(request)
}

/// Sends a magic link email to the specified recipient using Mailtrap REST API
///
/// # Arguments
/// * `recipient_email` - The email address to send the magic link to
/// * `magic_link` - The full magic link URL for authentication
/// * `language` - Optional language code for email template (e.g., "es", "en")
///
/// # Returns
/// * `Ok(())` if the email was sent successfully
/// * `Err(anyhow::Error)` if there was an error sending the email
pub async fn send_magic_link_email(
    recipient_email: &str,
    magic_link: &str,
    language: Option<&str>,
) -> Result<()> {
    // DEV-MODE ONLY: Check dry-run flag before sending
    // Production builds: this entire block is removed, email always sent
    #[cfg(feature = "dev-mode")]
    {
        if is_email_dry_run_enabled() {
            let (subject, html_content, text_content) =
                render_magic_link_email(magic_link, language.unwrap_or("en"));

            // eprintln!("📧 [DRY-RUN] Magic link email NOT sent (dev-mode, testing)");
            // eprintln!("   📬 To: {}", recipient_email);
            // eprintln!("   📝 Subject: {}", subject);
            // eprintln!("   🔗 Magic Link: {}", magic_link);
            // eprintln!("   🌐 Language: {}", language.unwrap_or("en"));
            // eprintln!("   📄 HTML length: {} bytes", html_content.len());
            // eprintln!("   📄 Text length: {} bytes", text_content.len());
            info!("📧 [DRY-RUN] Magic link email NOT sent (dev-mode, testing)");
            info!("   📬 To: {}", recipient_email);
            info!("   📝 Subject: {}", subject);
            info!("   🔗 Magic Link: {}", magic_link);
            info!("   🌐 Language: {}", language.unwrap_or("en"));
            info!("   📄 HTML length: {} bytes", html_content.len());
            info!("   📄 Text length: {} bytes", text_content.len());

            return Ok(());
        }
    }

    // ALWAYS executed in production, only if dry-run OFF in development
    let config = EmailConfig::from_environment()?;

    // Validate email format (basic validation)
    if recipient_email.is_empty() || !recipient_email.contains('@') {
        return Err(anyhow!(
            "Invalid recipient email address: {}",
            recipient_email
        ));
    }

    // Create HTTP request for Mailtrap API
    let request = create_email_request(&config, recipient_email, magic_link, language)?;

    // Send HTTP request using Spin's outbound HTTP
    let response: Response = spin_sdk::http::send(request)
        .await
        .map_err(|e| anyhow!("Failed to send HTTP request to Mailtrap API: {}", e))?;

    // Check if the request was successful
    let status = response.status();
    if *status == 200 || *status == 202 {
        // println!(
        //     "✅ Magic link email sent successfully to: {} (Status: {})",
        //     recipient_email, status
        // );
        info!(
            "✅ Magic link email sent successfully to: {} (Status: {})",
            recipient_email, status
        );
        Ok(())
    } else {
        let body = String::from_utf8_lossy(response.body());
        Err(anyhow!(
            "Mailtrap API returned error. Status: {}, Body: {}",
            status,
            body
        ))
    }
}

/// Sends a shared secret receiver email using Mailtrap REST API
///
/// # Arguments
/// * `recipient_email` - The receiver email address
/// * `secret_url` - The full secret URL for the receiver
/// * `reference` - The reference hash (Base58)
/// * `otp` - Optional 9-digit OTP
/// * `sender_email` - Email of the sender
/// * `expires_hours` - Expiration time in hours
/// * `max_reads` - Maximum number of reads allowed
/// * `language` - Optional language code for email template (e.g., "es", "en")
///
/// # Returns
/// * `Ok(())` if the email was sent successfully
/// * `Err(anyhow::Error)` if there was an error sending the email
#[allow(clippy::too_many_arguments)]
pub async fn send_shared_secret_receiver_email(
    recipient_email: &str,
    secret_url: &str,
    reference: &str,
    otp: Option<&str>,
    sender_email: &str,
    expires_hours: i64,
    max_reads: i64,
    language: Option<&str>,
) -> Result<()> {
    use crate::email_templates::shared_secret::render_shared_secret_receiver_email;

    // Render email template (needed for both dry-run and real sending)
    let (subject, html_content, text_content) = render_shared_secret_receiver_email(
        secret_url,
        reference,
        otp,
        sender_email,
        expires_hours,
        max_reads,
        language.unwrap_or("en"),
    );

    // DEV-MODE ONLY: Check dry-run flag before sending
    // Production builds: this entire block is removed, email always sent
    #[cfg(feature = "dev-mode")]
    {
        if is_email_dry_run_enabled() {
            // eprintln!("📧 [DRY-RUN] Shared secret receiver email NOT sent (dev-mode, testing)");
            // eprintln!("   📬 To: {}", recipient_email);
            // eprintln!("   📝 Subject: {}", subject);
            // eprintln!("   🔗 Secret URL: {}", secret_url);
            // eprintln!("   🔑 Reference: {}", reference);
            // eprintln!("   🔐 OTP: {}", otp.unwrap_or("N/A"));
            // eprintln!("   👤 Sender: {}", sender_email);
            // eprintln!("   ⏰ Expires: {} hours", expires_hours);
            // eprintln!("   📖 Max reads: {}", max_reads);
            // eprintln!("   🌐 Language: {}", language.unwrap_or("en"));
            // eprintln!("   📄 HTML length: {} bytes", html_content.len());
            // eprintln!("   📄 Text length: {} bytes", text_content.len());
            info!("📧 [DRY-RUN] Shared secret receiver email NOT sent (dev-mode, testing)");
            info!("   📬 To: {}", recipient_email);
            info!("   📝 Subject: {}", subject);
            info!("   🔗 Secret URL: {}", secret_url);
            info!("   🔑 Reference: {}", reference);
            info!("   🔐 OTP: {}", otp.unwrap_or("N/A"));
            info!("   👤 Sender: {}", sender_email);
            info!("   ⏰ Expires: {} hours", expires_hours);
            info!("   📖 Max reads: {}", max_reads);
            info!("   🌐 Language: {}", language.unwrap_or("en"));
            info!("   📄 HTML length: {} bytes", html_content.len());
            info!("   📄 Text length: {} bytes", text_content.len());

            return Ok(());
        }
    }

    // ALWAYS executed in production, only if dry-run OFF in development
    let config = EmailConfig::from_environment()?;

    // Validate email format
    if recipient_email.is_empty() || !recipient_email.contains('@') {
        return Err(anyhow!(
            "Invalid recipient email address: {}",
            recipient_email
        ));
    }

    // Generate unique Message-ID
    let message_id = format!(
        "<{}.{}@mailer.hashrand.com>",
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_else(|| {
            chrono::Utc::now()
                .timestamp_millis()
                .checked_mul(1_000_000)
                .unwrap_or(0)
        }),
        nanoid::nanoid!(8)
    );

    // Create email payload
    let email_payload = json!({
        "from": {
            "email": config.from_email,
            "name": "HashRand"
        },
        "to": [
            {
                "email": recipient_email,
                "name": recipient_email.split('@').next().unwrap_or("User")
            }
        ],
        "subject": subject,
        "text": text_content,
        "html": html_content,
        "category": "Shared Secret",
        "headers": {
            "Message-ID": message_id,
            "X-Priority": "1"
        }
    });

    // Build full URL - same logic as send_magic_link_email
    let full_url = if config.api_url.contains("send.api.mailtrap.io") {
        // Custom domain - use URL as-is without inbox ID
        config.api_url.clone()
    } else {
        // Sandbox - append inbox ID
        format!("{}/{}", config.api_url, config.inbox_id)
    };

    // Create HTTP request
    let request = Request::builder()
        .method(Method::Post)
        .uri(&full_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.api_token))
        .header("Accept", "application/json")
        .body(email_payload.to_string())
        .build();

    // Send HTTP request
    let response: Response = spin_sdk::http::send(request)
        .await
        .map_err(|e| anyhow!("Failed to send HTTP request to Mailtrap API: {}", e))?;

    let status = response.status();
    if *status >= 200 && *status < 300 {
        // println!(
        //     "✅ Shared secret receiver email sent successfully to {}",
        //     recipient_email
        // );
        info!(
            "✅ Shared secret receiver email sent successfully to {}",
            recipient_email
        );
        Ok(())
    } else {
        let body_bytes = response.body();
        let body_str = String::from_utf8_lossy(body_bytes);
        Err(anyhow!(
            "Mailtrap API returned error status {}: {}",
            status,
            body_str
        ))
    }
}

/// Sends a shared secret sender (copy) email using Mailtrap REST API
///
/// # Arguments
/// * `sender_email` - The sender email address
/// * `secret_url` - The full secret URL for the sender
/// * `reference` - The reference hash (Base58)
/// * `receiver_email` - Email of the receiver
/// * `expires_hours` - Expiration time in hours
/// * `language` - Optional language code for email template (e.g., "es", "en")
///
/// # Returns
/// * `Ok(())` if the email was sent successfully
/// * `Err(anyhow::Error)` if there was an error sending the email
pub async fn send_shared_secret_sender_email(
    sender_email: &str,
    secret_url: &str,
    reference: &str,
    receiver_email: &str,
    expires_hours: i64,
    language: Option<&str>,
) -> Result<()> {
    use crate::email_templates::shared_secret::render_shared_secret_sender_email;

    // Render email template (needed for both dry-run and real sending)
    let (subject, html_content, text_content) = render_shared_secret_sender_email(
        secret_url,
        reference,
        receiver_email,
        expires_hours,
        language.unwrap_or("en"),
    );

    // DEV-MODE ONLY: Check dry-run flag before sending
    // Production builds: this entire block is removed, email always sent
    #[cfg(feature = "dev-mode")]
    {
        if is_email_dry_run_enabled() {
            // eprintln!(
            //     "📧 [DRY-RUN] Shared secret sender (copy) email NOT sent (dev-mode, testing)"
            // );
            // eprintln!("   📬 To: {}", sender_email);
            // eprintln!("   📝 Subject: {}", subject);
            // eprintln!("   🔗 Secret URL: {}", secret_url);
            // eprintln!("   🔑 Reference: {}", reference);
            // eprintln!("   👤 Receiver: {}", receiver_email);
            // eprintln!("   ⏰ Expires: {} hours", expires_hours);
            // eprintln!("   🌐 Language: {}", language.unwrap_or("en"));
            // eprintln!("   📄 HTML length: {} bytes", html_content.len());
            // eprintln!("   📄 Text length: {} bytes", text_content.len());
            info!(
                "📧 [DRY-RUN] Shared secret sender (copy) email NOT sent (dev-mode, testing)"
            );
            info!("   📬 To: {}", sender_email);
            info!("   📝 Subject: {}", subject);
            info!("   🔗 Secret URL: {}", secret_url);
            info!("   🔑 Reference: {}", reference);
            info!("   👤 Receiver: {}", receiver_email);
            info!("   ⏰ Expires: {} hours", expires_hours);
            info!("   🌐 Language: {}", language.unwrap_or("en"));
            info!("   📄 HTML length: {} bytes", html_content.len());
            info!("   📄 Text length: {} bytes", text_content.len());

            return Ok(());
        }
    }

    // ALWAYS executed in production, only if dry-run OFF in development
    let config = EmailConfig::from_environment()?;

    // Validate email format
    if sender_email.is_empty() || !sender_email.contains('@') {
        return Err(anyhow!("Invalid sender email address: {}", sender_email));
    }

    // Generate unique Message-ID
    let message_id = format!(
        "<{}.{}@mailer.hashrand.com>",
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_else(|| {
            chrono::Utc::now()
                .timestamp_millis()
                .checked_mul(1_000_000)
                .unwrap_or(0)
        }),
        nanoid::nanoid!(8)
    );

    // Create email payload
    let email_payload = json!({
        "from": {
            "email": config.from_email,
            "name": "HashRand"
        },
        "to": [
            {
                "email": sender_email,
                "name": sender_email.split('@').next().unwrap_or("User")
            }
        ],
        "subject": subject,
        "text": text_content,
        "html": html_content,
        "category": "Shared Secret",
        "headers": {
            "Message-ID": message_id,
            "X-Priority": "3"
        }
    });

    // Build full URL - same logic as send_magic_link_email
    let full_url = if config.api_url.contains("send.api.mailtrap.io") {
        // Custom domain - use URL as-is without inbox ID
        config.api_url.clone()
    } else {
        // Sandbox - append inbox ID
        format!("{}/{}", config.api_url, config.inbox_id)
    };

    // Create HTTP request
    let request = Request::builder()
        .method(Method::Post)
        .uri(&full_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.api_token))
        .header("Accept", "application/json")
        .body(email_payload.to_string())
        .build();

    // Send HTTP request
    let response: Response = spin_sdk::http::send(request)
        .await
        .map_err(|e| anyhow!("Failed to send HTTP request to Mailtrap API: {}", e))?;

    let status = response.status();
    if *status >= 200 && *status < 300 {
        // println!(
        //     "✅ Shared secret sender (copy) email sent successfully to {}",
        //     sender_email
        // );
        info!(
            "✅ Shared secret sender (copy) email sent successfully to {}",
            sender_email
        );
        Ok(())
    } else {
        let body_bytes = response.body();
        let body_str = String::from_utf8_lossy(body_bytes);
        Err(anyhow!(
            "Mailtrap API returned error status {}: {}",
            status,
            body_str
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_email_payload_structure() {
        let config = EmailConfig {
            api_url: "https://test.api".to_string(),
            api_token: "test_token".to_string(),
            inbox_id: "test_inbox".to_string(),
            from_email: "test@example.com".to_string(),
        };

        let request = create_email_request(
            &config,
            "recipient@example.com",
            "https://magic.link",
            Some("es"),
        );
        assert!(request.is_ok());

        if let Ok(req) = request {
            assert_eq!(req.method(), &Method::Post);
            // Test that the request was built successfully
            assert!(!req.body().is_empty()); // Body should contain JSON
        }
    }
}
