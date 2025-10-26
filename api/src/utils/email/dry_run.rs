// ==================== DEV-MODE ONLY: Email Dry-Run System ====================
// This entire file is ELIMINATED from production builds (cargo build --no-default-features)
// In development: emails ARE sent by default (dry-run OFF), tests activate dry-run explicitly
// In production: this code doesn't exist, emails ALWAYS sent

use tracing::{error, info};

/// KV Store key for dry-run state (DEV-MODE ONLY)
/// Uses Spin KV Store for persistence across requests (AtomicBool doesn't work in WASM)
#[cfg(feature = "dev-mode")]
const DRY_RUN_KV_KEY: &str = "email_dry_run_mode";

/// Toggle email dry-run mode using Spin KV Store (DEV-MODE ONLY)
/// This function doesn't exist in production builds
///
/// # Why KV Store instead of AtomicBool
/// In Spin/WebAssembly, static variables don't reliably persist state between requests
/// because each request may execute in an isolated context. Spin KV Store provides
/// guaranteed persistence across all requests.
///
/// # Safety
/// Thread-safe via Spin's KV Store implementation. Can be called from multiple threads.
#[cfg(feature = "dev-mode")]
pub fn set_email_dry_run(enabled: bool) {
    use spin_sdk::key_value::Store;

    // Open default KV store (handle Result)
    let store = match Store::open_default() {
        Ok(s) => s,
        Err(e) => {
            error!("⚠️ Failed to open KV Store for dry-run mode: {}", e);
            return;
        }
    };

    // Store state as single byte: 1 = enabled, 0 = disabled
    let value = if enabled { vec![1u8] } else { vec![0u8] };

    match store.set(DRY_RUN_KV_KEY, &value) {
        Ok(_) => {
            info!(
                "📧 [DEV-MODE] Email dry-run mode {} via KV Store",
                if enabled {
                    "ENABLED (emails will NOT be sent)"
                } else {
                    "DISABLED (emails will be sent)"
                }
            );
        }
        Err(e) => {
            error!("⚠️ Failed to set dry-run mode in KV Store: {}", e);
        }
    }
}

/// Check if email dry-run mode is enabled using Spin KV Store (DEV-MODE ONLY)
/// Returns: true if dry-run active (don't send emails), false otherwise
/// Default: false (emails ON) if key doesn't exist or KV Store fails to open
#[cfg(feature = "dev-mode")]
pub(super) fn is_email_dry_run_enabled() -> bool {
    use spin_sdk::key_value::Store;

    // Open default KV store (handle Result)
    let store = match Store::open_default() {
        Ok(s) => s,
        Err(e) => {
            error!("⚠️ Failed to open KV Store to check dry-run mode: {}", e);
            return false; // Default: emails ON if KV Store fails
        }
    };

    // Read state from KV store
    match store.get(DRY_RUN_KV_KEY) {
        Ok(Some(value)) => {
            // Interpret: 1 = enabled, anything else = disabled
            !value.is_empty() && value[0] == 1u8
        }
        Ok(None) | Err(_) => {
            // Key doesn't exist or error → default to false (emails ON)
            false
        }
    }
}
