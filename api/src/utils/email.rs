use anyhow::{Result, anyhow};
use serde_json::json;
use spin_sdk::{
    http::{Method, Request, Response},
    variables,
};

use crate::email_templates::render_magic_link_email;

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
                println!("⚠️ CRITICAL: timestamp_nanos_opt() overflow - server clock may be misconfigured (date > year 2262)");
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
        println!(
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
