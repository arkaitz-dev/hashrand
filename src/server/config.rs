use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct RateLimitEntry {
    pub requests: u32,
    pub last_reset: Instant,
}

pub type RateLimitMap = Arc<RwLock<HashMap<SocketAddr, RateLimitEntry>>>;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ServerConfig {
    pub max_param_length: usize,
    pub enable_rate_limiting: bool,
    pub requests_per_second: u64,
    pub enable_cors: bool,
    pub max_request_body_size: usize,
    pub rate_limiter: Option<RateLimitMap>,
}