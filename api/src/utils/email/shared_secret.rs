use anyhow::{Result, anyhow};
use serde_json::json;
use spin_sdk::http::{Method, Request, Response};
use tracing::info;

use super::config::EmailConfig;

#[cfg(feature = "dev-mode")]
use super::dry_run::is_email_dry_run_enabled;

/// Sends a shared secret receiver email using Mailtrap REST API
///
/// # Arguments
/// * `recipient_email` - The receiver email address
/// * `secret_url` - The full secret URL for the receiver
/// * `reference` - The reference hash (Base58)
/// * `sender_email` - Email of the sender
/// * `expires_hours` - Expiration time in hours
/// * `max_reads` - Maximum number of reads allowed
/// * `language` - Optional language code for email template (e.g., "es", "en")
///
/// # Returns
/// * `Ok(())` if the email was sent successfully
/// * `Err(anyhow::Error)` if there was an error sending the email
pub async fn send_shared_secret_receiver_email(
    recipient_email: &str,
    secret_url: &str,
    reference: &str,
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
            info!(
                "📧 [DRY-RUN] Shared secret receiver email NOT sent → {}",
                secret_url
            );

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
        info!(
            "📧 Shared secret receiver email sent to {} → {}",
            recipient_email, secret_url
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
            info!(
                "📧 [DRY-RUN] Shared secret sender (copy) email NOT sent → {}",
                secret_url
            );

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
        info!(
            "📧 Shared secret sender (copy) email sent to {} → {}",
            sender_email, secret_url
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
