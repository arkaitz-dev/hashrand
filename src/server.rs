use crate::cli::{AlphabetType, HashRequest};
use crate::generators;
use axum::{
    extract::{ConnectInfo, Query, State},
    http::StatusCode,
    response::Response,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer, services::ServeDir};

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

#[derive(Deserialize)]
pub struct GenerateQuery {
    pub length: Option<usize>,
    pub alphabet: Option<String>,
    pub raw: Option<bool>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

#[derive(Deserialize)]
pub struct ApiKeyQuery {
    pub raw: Option<bool>,
}

#[derive(Deserialize)]
pub struct PasswordQuery {
    pub length: Option<usize>,
    pub raw: Option<bool>,
}

pub fn validate_query_params(prefix: &Option<String>, suffix: &Option<String>, max_length: usize) -> Result<(), String> {
    if let Some(p) = prefix {
        if p.len() > max_length {
            return Err(format!("Prefix length ({}) exceeds maximum allowed ({})", p.len(), max_length));
        }
    }
    
    if let Some(s) = suffix {
        if s.len() > max_length {
            return Err(format!("Suffix length ({}) exceeds maximum allowed ({})", s.len(), max_length));
        }
    }
    
    Ok(())
}

pub async fn check_rate_limit(
    rate_limiter: &RateLimitMap,
    addr: SocketAddr,
    requests_per_second: u64,
) -> bool {
    let mut limiter = rate_limiter.write().await;
    let now = Instant::now();
    
    let entry = limiter.entry(addr).or_insert(RateLimitEntry {
        requests: 0,
        last_reset: now,
    });
    
    // Reset counter if a second has passed
    if now.duration_since(entry.last_reset) >= Duration::from_secs(1) {
        entry.requests = 0;
        entry.last_reset = now;
    }
    
    if entry.requests >= requests_per_second as u32 {
        false // Rate limit exceeded
    } else {
        entry.requests += 1;
        true // Request allowed
    }
}



pub async fn handle_generate(
    State(config): State<Arc<ServerConfig>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<GenerateQuery>
) -> Result<Response<String>, StatusCode> {
    // Check rate limiting if enabled
    if let Some(ref rate_limiter) = config.rate_limiter {
        if !check_rate_limit(rate_limiter, addr, config.requests_per_second).await {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }
    let length = params.length.unwrap_or(21);
    
    if !(2..=128).contains(&length) {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Validate parameter lengths
    if let Err(_) = validate_query_params(&params.prefix, &params.suffix, config.max_param_length) {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let alphabet = match params.alphabet.as_deref() {
        Some("base58") | None => AlphabetType::Base58,
        Some("no-look-alike") => AlphabetType::NoLookAlike,
        Some("full") => AlphabetType::Full,
        Some("full-with-symbols") => AlphabetType::FullWithSymbols,
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    
    let request = HashRequest {
        length,
        alphabet,
        raw: params.raw.unwrap_or(true),  // Default to true in server mode
        check: false,  // Never check in server mode (no filesystem access)
        prefix: params.prefix,
        suffix: params.suffix,
    };
    
    match generators::generate_hash_from_request(&request) {
        Ok(result) => Ok(Response::builder()
            .header("content-type", "text/plain")
            .body(result)
            .unwrap()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn handle_api_key(
    State(config): State<Arc<ServerConfig>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<ApiKeyQuery>
) -> Result<Response<String>, StatusCode> {
    // Check rate limiting if enabled
    if let Some(ref rate_limiter) = config.rate_limiter {
        if !check_rate_limit(rate_limiter, addr, config.requests_per_second).await {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }
    
    match generators::generate_api_key_response(params.raw.unwrap_or(true)) {
        Ok(result) => Ok(Response::builder()
            .header("content-type", "text/plain")
            .body(result)
            .unwrap()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn handle_password(
    State(config): State<Arc<ServerConfig>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(params): Query<PasswordQuery>
) -> Result<Response<String>, StatusCode> {
    // Check rate limiting if enabled
    if let Some(ref rate_limiter) = config.rate_limiter {
        if !check_rate_limit(rate_limiter, addr, config.requests_per_second).await {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }
    
    match generators::generate_password_response(params.length, params.raw.unwrap_or(true)) {
        Ok(result) => Ok(Response::builder()
            .header("content-type", "text/plain")
            .body(result)
            .unwrap()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn start_server(
    port: u16,
    listen_all_ips: bool,
    max_param_length: usize,
    enable_rate_limiting: bool,
    requests_per_second: u64,
    enable_cors: bool,
    max_body_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let rate_limiter = if enable_rate_limiting {
        Some(Arc::new(RwLock::new(HashMap::new())))
    } else {
        None
    };

    let config = Arc::new(ServerConfig {
        max_param_length,
        enable_rate_limiting,
        requests_per_second,
        enable_cors,
        max_request_body_size: max_body_size,
        rate_limiter,
    });

    // Serve production build files from the dist directory
    let static_files = ServeDir::new("dist");
    
    let api_routes = Router::new()
        .route("/api/generate", get(handle_generate))
        .route("/api/api-key", get(handle_api_key))
        .route("/api/password", get(handle_password))
        .with_state(config.clone());
    
    let mut app = Router::new()
        .merge(api_routes)
        .fallback_service(static_files);

    // Add middleware layers
    app = app.layer(RequestBodyLimitLayer::new(max_body_size));

    if enable_cors {
        println!("CORS enabled - allowing cross-origin requests");
        app = app.layer(CorsLayer::permissive());
    }

    let host = if listen_all_ips { "0.0.0.0" } else { "127.0.0.1" };
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;
    
    println!("hashrand server listening on http://{}:{}", host, port);
    println!("Web Interface (served from dist/):");
    println!("  GET /                                     - Interactive web interface (production build)");
    println!("API Endpoints:");
    println!("  GET /api/generate?length=21&alphabet=base58");
    println!("  GET /api/api-key");
    println!("  GET /api/password?length=21");
    println!("Security features:");
    println!("  Parameter validation: prefix/suffix max {} chars", max_param_length);
    if enable_rate_limiting {
        println!("  Rate limiting: {} requests/second per IP", requests_per_second);
    } else {
        println!("  Rate limiting: disabled (use --enable-rate-limiting to enable)");
    }
    if enable_cors {
        println!("  CORS: enabled for cross-origin requests");
    } else {
        println!("  CORS: disabled (use --enable-cors to enable)");
    }
    println!("  Request body limit: {} bytes", max_body_size);
    println!("Note: All endpoints return raw text by default (no newline)");
    println!("Development: Use 'npm run dev' for development server with HMR");
    println!("Production: Run 'npm run build' first to generate dist/ files");
    
    axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await?;
    
    Ok(())
}
