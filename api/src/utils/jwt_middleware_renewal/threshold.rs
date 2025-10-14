//! 2/3 threshold renewal window calculation

use tracing::debug;

use crate::utils::jwt::config::get_refresh_token_duration_minutes;

/// Check if proactive renewal is needed based on 2/3 threshold
///
/// # Arguments
/// * `refresh_expires_at` - Refresh token expiration timestamp
/// * `now` - Current timestamp
///
/// # Returns
/// * `Result<bool, String>` - true if renewal needed, false otherwise, or error
pub fn is_in_renewal_window(refresh_expires_at: i64, now: i64) -> Result<bool, String> {
    // Get refresh token duration
    let refresh_duration_minutes = get_refresh_token_duration_minutes()
        .map_err(|e| format!("Failed to get refresh token duration: {}", e))?;

    let refresh_duration_seconds = refresh_duration_minutes * 60;
    let time_remaining = refresh_expires_at - now;

    // Calculate 2/3 threshold: if remaining time is less than 2/3 of total duration
    let two_thirds_threshold = (refresh_duration_seconds * 2) / 3;

    if time_remaining < two_thirds_threshold as i64 {
        //     "Proactive renewal triggered: {}s remaining < {}s threshold",
        //     time_remaining, two_thirds_threshold
        // );
        debug!(
            "Proactive renewal triggered: {}s remaining < {}s threshold",
            time_remaining, two_thirds_threshold
        );
        Ok(true)
    } else {
        Ok(false)
    }
}
