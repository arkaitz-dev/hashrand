use anyhow::{Result, anyhow};
use spin_sdk::variables;

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
