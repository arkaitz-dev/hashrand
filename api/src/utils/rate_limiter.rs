/// Rate limiting module for preventing brute force attacks
use std::collections::HashMap;
use std::time::{Duration, Instant};
use anyhow::Result;

/// Rate limiter for authentication endpoints
pub struct RateLimiter {
    /// IP -> (request_count, window_start_time)
    requests: HashMap<String, (u32, Instant)>,
    /// Maximum requests allowed per window
    max_requests: u32,
    /// Time window duration
    window_duration: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter with specified limits
    pub fn new(max_requests: u32, window_minutes: u64) -> Self {
        Self {
            requests: HashMap::new(),
            max_requests,
            window_duration: Duration::from_secs(window_minutes * 60),
        }
    }
    
    /// Check if request is allowed for the given IP
    pub fn is_allowed(&mut self, ip: &str) -> bool {
        let now = Instant::now();
        
        // Clean up expired entries periodically (every 100 checks)
        if self.requests.len() % 100 == 0 {
            self.cleanup_expired_entries(now);
        }
        
        match self.requests.get_mut(ip) {
            Some((count, window_start)) => {
                // Check if we're still in the same window
                if now.duration_since(*window_start) < self.window_duration {
                    // Same window - check if limit exceeded
                    if *count >= self.max_requests {
                        false // Rate limited
                    } else {
                        *count += 1;
                        true // Request allowed
                    }
                } else {
                    // New window - reset counter
                    *count = 1;
                    *window_start = now;
                    true // Request allowed
                }
            }
            None => {
                // First request from this IP
                self.requests.insert(ip.to_string(), (1, now));
                true // Request allowed
            }
        }
    }
    
    /// Get remaining requests for IP (for informational purposes)
    pub fn get_remaining(&self, ip: &str) -> u32 {
        match self.requests.get(ip) {
            Some((count, window_start)) => {
                let now = Instant::now();
                if now.duration_since(*window_start) < self.window_duration {
                    self.max_requests.saturating_sub(*count)
                } else {
                    self.max_requests // New window
                }
            }
            None => self.max_requests, // No requests yet
        }
    }
    
    /// Get time until window reset for IP
    pub fn get_reset_time(&self, ip: &str) -> Option<Duration> {
        match self.requests.get(ip) {
            Some((_, window_start)) => {
                let now = Instant::now();
                let elapsed = now.duration_since(*window_start);
                if elapsed < self.window_duration {
                    Some(self.window_duration - elapsed)
                } else {
                    None // Window already expired
                }
            }
            None => None, // No requests yet
        }
    }
    
    /// Clean up expired entries to prevent memory leak
    fn cleanup_expired_entries(&mut self, now: Instant) {
        self.requests.retain(|_, (_, window_start)| {
            now.duration_since(*window_start) < self.window_duration
        });
    }
}

use std::sync::Mutex;
use std::sync::OnceLock;

/// Global rate limiter instance (thread-safe)
static RATE_LIMITER: OnceLock<Mutex<RateLimiter>> = OnceLock::new();

/// Initialize the global rate limiter
pub fn init_rate_limiter() {
    RATE_LIMITER.get_or_init(|| {
        Mutex::new(RateLimiter::new(
            5,  // 5 requests
            15, // per 15 minutes
        ))
    });
}

/// Check if request is allowed from the given IP
pub fn check_rate_limit(ip: &str) -> Result<()> {
    if let Some(limiter_mutex) = RATE_LIMITER.get() {
        if let Ok(mut limiter) = limiter_mutex.lock() {
            if limiter.is_allowed(ip) {
                Ok(())
            } else {
                let reset_time = limiter.get_reset_time(ip)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                Err(anyhow::anyhow!(
                    "Rate limit exceeded. Try again in {} seconds", 
                    reset_time
                ))
            }
        } else {
            // Mutex poisoned - allow request (fail-open)
            Ok(())
        }
    } else {
        // Rate limiter not initialized - allow request (fail-open)
        Ok(())
    }
}

/// Extract client IP from request headers (with proxy support)
pub fn extract_client_ip<'a>(headers: impl Iterator<Item = (&'a str, &'a spin_sdk::http::HeaderValue)>) -> String {
    // Convert iterator to HashMap for easier lookup
    let header_map: std::collections::HashMap<&str, &spin_sdk::http::HeaderValue> = 
        headers.collect();
    
    // Check for proxy headers first (X-Forwarded-For)
    if let Some(forwarded_for) = header_map.get("x-forwarded-for") {
        if let Ok(forwarded_str) = std::str::from_utf8(forwarded_for.as_bytes()) {
            // Take first IP if multiple (original client)
            if let Some(first_ip) = forwarded_str.split(',').next() {
                return first_ip.trim().to_string();
            }
        }
    }
    
    // Check for real IP header (X-Real-IP - some proxies use this)
    if let Some(real_ip) = header_map.get("x-real-ip") {
        if let Ok(real_ip_str) = std::str::from_utf8(real_ip.as_bytes()) {
            return real_ip_str.trim().to_string();
        }
    }
    
    // Fallback to unknown (Spin SDK doesn't provide direct connection IP)
    // In production, this would typically come from a reverse proxy
    "unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rate_limiter_allows_under_limit() {
        let mut limiter = RateLimiter::new(3, 1); // 3 requests per minute
        
        assert!(limiter.is_allowed("192.168.1.1"));
        assert!(limiter.is_allowed("192.168.1.1"));
        assert!(limiter.is_allowed("192.168.1.1"));
    }
    
    #[test]
    fn test_rate_limiter_blocks_over_limit() {
        let mut limiter = RateLimiter::new(2, 1); // 2 requests per minute
        
        assert!(limiter.is_allowed("192.168.1.1"));
        assert!(limiter.is_allowed("192.168.1.1"));
        assert!(!limiter.is_allowed("192.168.1.1")); // Should be blocked
    }
    
    #[test]
    fn test_rate_limiter_different_ips() {
        let mut limiter = RateLimiter::new(1, 1); // 1 request per minute
        
        assert!(limiter.is_allowed("192.168.1.1"));
        assert!(limiter.is_allowed("192.168.1.2")); // Different IP, should be allowed
        assert!(!limiter.is_allowed("192.168.1.1")); // Same IP, should be blocked
    }
}