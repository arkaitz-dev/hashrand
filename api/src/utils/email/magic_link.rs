use anyhow::{Result, anyhow};
use serde_json::json;
use spin_sdk::http::{Method, Request, Response};
use tracing::{error, info};

use super::config::EmailConfig;
use crate::email_templates::render_magic_link_email;

#[cfg(feature = "dev-mode")]
use super::dry_run::is_email_dry_run_enabled;

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
            let (_subject, _html_content, _text_content) =
                render_magic_link_email(magic_link, language.unwrap_or("en"));

            // Log in INFO level with pattern that tests can extract ("Generated magic_link")
            // while clearly indicating DRY-RUN mode for human readers
            info!("📧 [DRY-RUN] Generated magic_link = {}", magic_link);

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
        info!(
            "📧 Magic link email sent to {} → {}",
            recipient_email, magic_link
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
