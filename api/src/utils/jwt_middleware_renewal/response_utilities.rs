//! DRY utilities for response building and token management

use crate::utils::jwt::config::get_refresh_token_duration_minutes;

/// Create refresh token cookie (DRY consolidation for 2 duplicated patterns)
///
/// # Arguments
/// * `refresh_token` - Refresh token value
///
/// # Returns
/// * `String` - Formatted Set-Cookie header value
pub fn create_refresh_cookie(refresh_token: &str) -> String {
    let refresh_duration_minutes = get_refresh_token_duration_minutes()
        .expect("CRITICAL: SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES must be set in .env");
    format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/",
        refresh_token,
        refresh_duration_minutes * 60
    )
}
