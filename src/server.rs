use crate::cli::{AlphabetType, HashRequest};
use axum::{
    extract::{ConnectInfo, Query, State},
    http::StatusCode,
    response::{Html, Response},
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
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer};

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

pub fn get_alphabet(alphabet_type: &AlphabetType) -> &'static [char] {
    // Base58 alphabet (Bitcoin alphabet) - default
    const BASE58_ALPHABET: [char; 58] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M',
        'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

    // No look-alike alphabet (removes: 0, O, I, l, 1)
    const NO_LOOK_ALIKE_ALPHABET: [char; 57] = [
        '2', '3', '4', '5', '6', '7', '8', '9',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M',
        'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

    // Full alphanumeric alphabet (uppercase, lowercase, and numbers)
    const FULL_ALPHABET: [char; 62] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

    // Full alphabet with symbols
    const FULL_WITH_SYMBOLS_ALPHABET: [char; 73] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '-', '_', '*', '^', '@', '#', '+', '!', '?', '$', '%'
    ];

    match alphabet_type {
        AlphabetType::Base58 => &BASE58_ALPHABET,
        AlphabetType::NoLookAlike => &NO_LOOK_ALIKE_ALPHABET,
        AlphabetType::Full => &FULL_ALPHABET,
        AlphabetType::FullWithSymbols => &FULL_WITH_SYMBOLS_ALPHABET,
    }
}

pub fn generate_hash_from_request(request: &HashRequest) -> Result<String, Box<dyn std::error::Error>> {
    let alphabet = get_alphabet(&request.alphabet);
    
    // Generate the random hash
    let hash = nanoid::format(nanoid::rngs::default, alphabet, request.length);
    
    // Build the full name with optional prefix and suffix
    let full_name = format!(
        "{}{}{}",
        request.prefix.as_deref().unwrap_or(""),
        hash,
        request.suffix.as_deref().unwrap_or("")
    );
    
    // Format the output
    Ok(if request.raw {
        full_name
    } else {
        format!("{}\n", full_name)
    })
}

pub fn generate_api_key_response(raw: bool) -> Result<String, Box<dyn std::error::Error>> {
    let alphabet = get_alphabet(&AlphabetType::Full);
    let hash = nanoid::format(nanoid::rngs::default, alphabet, 44);
    let api_key = format!("ak_{}", hash);
    
    Ok(if raw {
        api_key
    } else {
        format!("{}\n", api_key)
    })
}

pub fn generate_password_response(length: Option<usize>, raw: bool) -> Result<String, Box<dyn std::error::Error>> {
    let length = length.unwrap_or(21);
    
    if length < 21 || length > 44 {
        return Err("Password length must be between 21 and 44 characters".into());
    }
    
    let alphabet = get_alphabet(&AlphabetType::FullWithSymbols);
    let password = nanoid::format(nanoid::rngs::default, alphabet, length);
    
    Ok(if raw {
        password
    } else {
        format!("{}\n", password)
    })
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
    
    match generate_hash_from_request(&request) {
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
    
    match generate_api_key_response(params.raw.unwrap_or(true)) {
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
    
    match generate_password_response(params.length, params.raw.unwrap_or(true)) {
        Ok(result) => Ok(Response::builder()
            .header("content-type", "text/plain")
            .body(result)
            .unwrap()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn handle_root() -> Html<&'static str> {
    Html(WEB_INTERFACE_HTML)
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

    let api_routes = Router::new()
        .route("/api/generate", get(handle_generate))
        .route("/api/api-key", get(handle_api_key))
        .route("/api/password", get(handle_password))
        .with_state(config.clone());
    
    let mut app = Router::new()
        .route("/", get(handle_root))
        .merge(api_routes);

    // Add middleware layers
    app = app.layer(RequestBodyLimitLayer::new(max_body_size));

    if enable_cors {
        println!("CORS enabled - allowing cross-origin requests");
        app = app.layer(CorsLayer::permissive());
    }

    let host = if listen_all_ips { "0.0.0.0" } else { "127.0.0.1" };
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;
    
    println!("hashrand server listening on http://{}:{}", host, port);
    println!("Web Interface:");
    println!("  GET /                                     - Interactive web interface");
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
    
    axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await?;
    
    Ok(())
}

pub const WEB_INTERFACE_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>HashRand - Random Hash Generator</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            line-height: 1.6;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        
        .container {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
            overflow: hidden;
        }
        
        .header {
            background: #2c3e50;
            color: white;
            padding: 2rem;
            text-align: center;
        }
        
        .header h1 {
            font-size: 2.5rem;
            margin-bottom: 0.5rem;
            font-weight: 700;
        }
        
        .header p {
            opacity: 0.9;
            font-size: 1.1rem;
        }
        
        .content {
            padding: 2rem;
        }
        
        .menu-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 2rem;
            margin: 2rem 0;
        }
        
        .menu-card {
            background: white;
            border: 2px solid #e1e8ed;
            border-radius: 12px;
            padding: 2rem;
            text-align: center;
            cursor: pointer;
            transition: all 0.3s ease;
            position: relative;
            overflow: hidden;
        }
        
        .menu-card::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            opacity: 0;
            transition: opacity 0.3s ease;
        }
        
        .menu-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 12px 24px rgba(0, 0, 0, 0.15);
            border-color: #667eea;
        }
        
        .menu-card:hover::before {
            opacity: 0.05;
        }
        
        .menu-icon {
            font-size: 3rem;
            margin-bottom: 1rem;
            position: relative;
            z-index: 1;
        }
        
        .menu-card h3 {
            color: #2c3e50;
            margin-bottom: 0.5rem;
            font-size: 1.5rem;
            position: relative;
            z-index: 1;
        }
        
        .menu-card p {
            color: #7f8c8d;
            font-size: 0.95rem;
            position: relative;
            z-index: 1;
        }
        
        .view-container {
            display: none;
        }
        
        .view-container.active {
            display: block;
            animation: fadeIn 0.3s ease;
        }
        
        @keyframes fadeIn {
            from { 
                opacity: 0; 
                transform: translateY(10px); 
            }
            to { 
                opacity: 1; 
                transform: translateY(0); 
            }
        }
        
        .back-button {
            background: transparent;
            border: 2px solid #667eea;
            color: #667eea;
            margin-bottom: 1.5rem;
            width: auto;
            padding: 10px 20px;
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            font-weight: 600;
        }
        
        .back-button:hover {
            background: #667eea;
            color: white;
            transform: translateY(0);
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
        }
        
        .form-section {
            margin-bottom: 2rem;
        }
        
        .form-section h2 {
            color: #2c3e50;
            margin-bottom: 1rem;
            font-size: 1.3rem;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }
        
        .form-group {
            margin-bottom: 1.5rem;
        }
        
        label {
            display: block;
            margin-bottom: 0.5rem;
            font-weight: 600;
            color: #34495e;
        }
        
        input, select, button {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e1e8ed;
            border-radius: 8px;
            font-size: 1rem;
            transition: all 0.3s ease;
        }
        
        input:focus, select:focus {
            outline: none;
            border-color: #667eea;
            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }
        
        .range-group {
            display: flex;
            align-items: center;
            gap: 1rem;
        }
        
        input[type="range"] {
            flex: 1;
        }
        
        .range-value {
            background: #667eea;
            color: white;
            padding: 8px 12px;
            border-radius: 6px;
            font-weight: 600;
            min-width: 60px;
            text-align: center;
        }
        
        .button-group {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin-top: 1.5rem;
        }
        
        button {
            background: #667eea;
            color: white;
            border: none;
            cursor: pointer;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            transition: all 0.3s ease;
        }
        
        button:hover:not(:disabled) {
            background: #5a67d8;
            transform: translateY(-2px);
            box-shadow: 0 8px 20px rgba(102, 126, 234, 0.3);
        }
        
        button:active {
            transform: translateY(0);
        }
        
        button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
        
        .special-buttons {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 1rem;
        }
        
        .api-key-btn {
            background: #27ae60 !important;
        }
        
        .api-key-btn:hover {
            background: #229954 !important;
        }
        
        .password-btn {
            background: #e74c3c !important;
        }
        
        .password-btn:hover {
            background: #c0392b !important;
        }
        
        .result-section {
            background: #f8f9fa;
            border-radius: 8px;
            padding: 1.5rem;
            margin-top: 2rem;
            min-height: 120px;
        }
        
        .result-section h3 {
            color: #2c3e50;
            margin-bottom: 1rem;
        }
        
        .result-display {
            background: white;
            border: 2px solid #e1e8ed;
            border-radius: 6px;
            padding: 1rem;
            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
            font-size: 1.1rem;
            word-break: break-all;
            min-height: 60px;
            display: flex;
            align-items: center;
            position: relative;
        }
        
        .result-display.success {
            border-color: #27ae60;
            background: #d5f4e6;
            color: #27ae60;
        }
        
        .result-display.error {
            border-color: #e74c3c;
            background: #f8d7da;
            color: #e74c3c;
        }
        
        .copy-btn {
            position: absolute;
            top: 8px;
            right: 8px;
            width: auto;
            padding: 6px 12px;
            font-size: 0.8rem;
            background: #667eea;
            border-radius: 4px;
        }
        
        .loading {
            display: inline-block;
            width: 20px;
            height: 20px;
            border: 3px solid #f3f3f3;
            border-top: 3px solid #667eea;
            border-radius: 50%;
            animation: spin 1s linear infinite;
            margin-right: 10px;
        }
        
        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
        
        .info-box {
            background: #e3f2fd;
            border: 1px solid #bbdefb;
            border-radius: 6px;
            padding: 1rem;
            margin: 1rem 0;
            font-size: 0.9rem;
            color: #1976d2;
        }
        
        @media (max-width: 768px) {
            body {
                padding: 10px;
            }
            
            .header {
                padding: 1.5rem 1rem;
            }
            
            .header h1 {
                font-size: 2rem;
            }
            
            .content {
                padding: 1.5rem;
            }
            
            .button-group {
                grid-template-columns: 1fr;
            }
            
            .special-buttons {
                grid-template-columns: 1fr;
            }
            
            .range-group {
                flex-direction: column;
                align-items: stretch;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🎲 HashRand</h1>
            <p>Secure Random Hash Generator with Multiple Alphabets</p>
        </div>
        
        <div class="content">
            <hash-generator></hash-generator>
        </div>
    </div>
    
    <script>
        class HashGenerator extends HTMLElement {
            constructor() {
                super();
                this.attachShadow({ mode: 'open' });
                this.currentView = 'menu';
                this.render();
                this.bindEvents();
            }
            
            render() {
                this.shadowRoot.innerHTML = `
                    <style>
                        :host {
                            display: block;
                        }
                        
                        /* Menu styles */
                        .menu-grid {
                            display: grid;
                            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                            gap: 2rem;
                            margin: 2rem 0;
                        }
                        
                        .menu-card {
                            background: white;
                            border: 2px solid #e1e8ed;
                            border-radius: 12px;
                            padding: 2rem;
                            text-align: center;
                            cursor: pointer;
                            transition: all 0.3s ease;
                            position: relative;
                            overflow: hidden;
                        }
                        
                        .menu-card::before {
                            content: '';
                            position: absolute;
                            top: 0;
                            left: 0;
                            right: 0;
                            bottom: 0;
                            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                            opacity: 0;
                            transition: opacity 0.3s ease;
                        }
                        
                        .menu-card:hover {
                            transform: translateY(-5px);
                            box-shadow: 0 12px 24px rgba(0, 0, 0, 0.15);
                            border-color: #667eea;
                        }
                        
                        .menu-card:hover::before {
                            opacity: 0.05;
                        }
                        
                        .menu-icon {
                            font-size: 3rem;
                            margin-bottom: 1rem;
                            position: relative;
                            z-index: 1;
                        }
                        
                        .menu-card h3 {
                            color: #2c3e50;
                            margin-bottom: 0.5rem;
                            font-size: 1.5rem;
                            position: relative;
                            z-index: 1;
                        }
                        
                        .menu-card p {
                            color: #7f8c8d;
                            font-size: 0.95rem;
                            position: relative;
                            z-index: 1;
                            margin: 0;
                        }
                        
                        /* View container styles */
                        .view-container {
                            display: none;
                        }
                        
                        .view-container.active {
                            display: block;
                            animation: fadeIn 0.3s ease;
                        }
                        
                        @keyframes fadeIn {
                            from { 
                                opacity: 0; 
                                transform: translateY(10px); 
                            }
                            to { 
                                opacity: 1; 
                                transform: translateY(0); 
                            }
                        }
                        
                        /* Back button styles */
                        .back-button {
                            background: transparent;
                            border: 2px solid #667eea;
                            color: #667eea;
                            margin-bottom: 1.5rem;
                            width: auto;
                            padding: 10px 20px;
                            display: inline-flex;
                            align-items: center;
                            gap: 0.5rem;
                            font-weight: 600;
                            cursor: pointer;
                            border-radius: 8px;
                            font-size: 1rem;
                            transition: all 0.3s ease;
                        }
                        
                        .back-button:hover {
                            background: #667eea;
                            color: white;
                            transform: translateY(0);
                            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
                        }
                        
                        /* Form styles */
                        .form-section {
                            margin-bottom: 2rem;
                        }
                        
                        .form-section h2 {
                            color: #2c3e50;
                            margin-bottom: 1rem;
                            font-size: 1.3rem;
                            display: flex;
                            align-items: center;
                            gap: 0.5rem;
                        }
                        
                        .form-group {
                            margin-bottom: 1.5rem;
                        }
                        
                        label {
                            display: block;
                            margin-bottom: 0.5rem;
                            font-weight: 600;
                            color: #34495e;
                        }
                        
                        input, select, button {
                            width: 100%;
                            padding: 12px 16px;
                            border: 2px solid #e1e8ed;
                            border-radius: 8px;
                            font-size: 1rem;
                            transition: all 0.3s ease;
                            font-family: inherit;
                        }
                        
                        input:focus, select:focus {
                            outline: none;
                            border-color: #667eea;
                            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
                        }
                        
                        .range-group {
                            display: flex;
                            align-items: center;
                            gap: 1rem;
                        }
                        
                        input[type="range"] {
                            flex: 1;
                        }
                        
                        .range-value {
                            background: #667eea;
                            color: white;
                            padding: 8px 12px;
                            border-radius: 6px;
                            font-weight: 600;
                            min-width: 60px;
                            text-align: center;
                        }
                        
                        /* Button styles */
                        button {
                            background: #667eea;
                            color: white;
                            border: none;
                            cursor: pointer;
                            font-weight: 600;
                            text-transform: uppercase;
                            letter-spacing: 0.5px;
                            transition: all 0.3s ease;
                        }
                        
                        button:hover:not(:disabled) {
                            background: #5a67d8;
                            transform: translateY(-2px);
                            box-shadow: 0 8px 20px rgba(102, 126, 234, 0.3);
                        }
                        
                        button:active {
                            transform: translateY(0);
                        }
                        
                        button:disabled {
                            opacity: 0.6;
                            cursor: not-allowed;
                        }
                        
                        /* Result styles */
                        .result-section {
                            background: #f8f9fa;
                            border-radius: 8px;
                            padding: 1.5rem;
                            margin-top: 2rem;
                            min-height: 120px;
                        }
                        
                        .result-section h3 {
                            color: #2c3e50;
                            margin-bottom: 1rem;
                            margin-top: 0;
                        }
                        
                        .result-display {
                            background: white;
                            border: 2px solid #e1e8ed;
                            border-radius: 6px;
                            padding: 1rem;
                            font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
                            font-size: 1.1rem;
                            word-break: break-all;
                            min-height: 60px;
                            display: flex;
                            align-items: center;
                            position: relative;
                        }
                        
                        .result-display.success {
                            border-color: #27ae60;
                            background: #d5f4e6;
                            color: #27ae60;
                        }
                        
                        .result-display.error {
                            border-color: #e74c3c;
                            background: #f8d7da;
                            color: #e74c3c;
                        }
                        
                        .copy-btn {
                            position: absolute;
                            top: 8px;
                            right: 8px;
                            width: auto;
                            padding: 6px 12px;
                            font-size: 0.8rem;
                            background: #667eea;
                            border-radius: 4px;
                        }
                        
                        .loading {
                            display: inline-block;
                            width: 20px;
                            height: 20px;
                            border: 3px solid #f3f3f3;
                            border-top: 3px solid #667eea;
                            border-radius: 50%;
                            animation: spin 1s linear infinite;
                            margin-right: 10px;
                        }
                        
                        @keyframes spin {
                            0% { transform: rotate(0deg); }
                            100% { transform: rotate(360deg); }
                        }
                        
                        .info-box {
                            background: #e3f2fd;
                            border: 1px solid #bbdefb;
                            border-radius: 6px;
                            padding: 1rem;
                            margin: 1rem 0;
                            font-size: 0.9rem;
                            color: #1976d2;
                        }
                        
                        @media (max-width: 768px) {
                            .range-group {
                                flex-direction: column;
                                align-items: stretch;
                            }
                        }
                    </style>
                    
                    <!-- Menu View -->
                    <div id="menu-view" class="view-container active">
                        <div class="menu-grid">
                            <div class="menu-card" data-mode="generate">
                                <div class="menu-icon">🎲</div>
                                <h3>Generic Hash</h3>
                                <p>Generate customizable hashes with various alphabets</p>
                            </div>
                            
                            <div class="menu-card" data-mode="password">
                                <div class="menu-icon">🔐</div>
                                <h3>Password</h3>
                                <p>Create strong passwords with symbols</p>
                            </div>
                            
                            <div class="menu-card" data-mode="apiKey">
                                <div class="menu-icon">🔑</div>
                                <h3>API Key</h3>
                                <p>Generate secure API keys (ak_ prefix)</p>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Generic Hash View -->
                    <div id="generate-view" class="view-container">
                        <button class="back-button">← Back to Menu</button>
                        
                        <div class="form-section">
                            <h2>🎲 Generate Custom Hash</h2>
                            
                            <div class="form-group">
                                <label for="generate-length">Length</label>
                                <div class="range-group">
                                    <input type="range" id="generate-length" min="2" max="128" value="21">
                                    <span class="range-value">21</span>
                                </div>
                            </div>
                            
                            <div class="form-group">
                                <label for="generate-alphabet">Alphabet Type</label>
                                <select id="generate-alphabet">
                                    <option value="base58">Base58 (Bitcoin)</option>
                                    <option value="no-look-alike">No Look-alike</option>
                                    <option value="full">Full Alphanumeric</option>
                                    <option value="full-with-symbols">Full with Symbols</option>
                                </select>
                            </div>
                            
                            <div class="form-group">
                                <label for="generate-prefix">Prefix (optional)</label>
                                <input type="text" id="generate-prefix" placeholder="e.g., user_">
                            </div>
                            
                            <div class="form-group">
                                <label for="generate-suffix">Suffix (optional)</label>
                                <input type="text" id="generate-suffix" placeholder="e.g., _temp">
                            </div>
                            
                            <button id="generate-btn">Generate Hash</button>
                        </div>
                        
                        <div class="result-section">
                            <h3>Result</h3>
                            <div id="generate-result" class="result-display">
                                <span>Generated hash will appear here</span>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Password View -->
                    <div id="password-view" class="view-container">
                        <button class="back-button">← Back to Menu</button>
                        
                        <div class="form-section">
                            <h2>🔐 Generate Password</h2>
                            
                            <div class="form-group">
                                <label for="password-length">Length (21-44 characters)</label>
                                <div class="range-group">
                                    <input type="range" id="password-length" min="21" max="44" value="21">
                                    <span class="range-value">21</span>
                                </div>
                            </div>
                            
                            <div class="info-box">
                                <strong>Password Strength:</strong> Uses full alphanumeric alphabet with symbols for maximum security.
                            </div>
                            
                            <button id="password-btn">Generate Password</button>
                        </div>
                        
                        <div class="result-section">
                            <h3>Result</h3>
                            <div id="password-result" class="result-display">
                                <span>Generated password will appear here</span>
                            </div>
                        </div>
                    </div>
                    
                    <!-- API Key View -->
                    <div id="apikey-view" class="view-container">
                        <button class="back-button">← Back to Menu</button>
                        
                        <div class="form-section">
                            <h2>🔑 Generate API Key</h2>
                            
                            <div class="info-box">
                                <strong>Format:</strong> ak_ prefix + 44 random characters using full alphanumeric alphabet (256-bit entropy)
                            </div>
                            
                            <button id="apikey-btn">Generate API Key</button>
                        </div>
                        
                        <div class="result-section">
                            <h3>Result</h3>
                            <div id="apikey-result" class="result-display">
                                <span>Generated API key will appear here</span>
                            </div>
                        </div>
                    </div>
                `;
            }
            
            bindEvents() {
                // Menu navigation
                this.shadowRoot.querySelectorAll('.menu-card').forEach(card => {
                    card.addEventListener('click', (e) => {
                        const mode = e.currentTarget.dataset.mode;
                        this.switchView(mode);
                    });
                });
                
                // Back buttons
                this.shadowRoot.querySelectorAll('.back-button').forEach(btn => {
                    btn.addEventListener('click', () => {
                        this.switchView('menu');
                    });
                });
                
                // Range inputs
                this.shadowRoot.querySelectorAll('input[type="range"]').forEach(range => {
                    range.addEventListener('input', (e) => {
                        const valueSpan = e.target.parentElement.querySelector('.range-value');
                        if (valueSpan) {
                            valueSpan.textContent = e.target.value;
                        }
                    });
                });
                
                // Generate buttons
                this.shadowRoot.getElementById('generate-btn').addEventListener('click', () => {
                    this.generateHash();
                });
                
                this.shadowRoot.getElementById('password-btn').addEventListener('click', () => {
                    this.generatePassword();
                });
                
                this.shadowRoot.getElementById('apikey-btn').addEventListener('click', () => {
                    this.generateApiKey();
                });
            }
            
            switchView(viewName) {
                // Hide all views
                this.shadowRoot.querySelectorAll('.view-container').forEach(view => {
                    view.classList.remove('active');
                });
                
                // Show selected view
                if (viewName === 'menu') {
                    this.shadowRoot.getElementById('menu-view').classList.add('active');
                } else if (viewName === 'generate') {
                    this.shadowRoot.getElementById('generate-view').classList.add('active');
                } else if (viewName === 'password') {
                    this.shadowRoot.getElementById('password-view').classList.add('active');
                } else if (viewName === 'apiKey') {
                    this.shadowRoot.getElementById('apikey-view').classList.add('active');
                }
                
                this.currentView = viewName;
            }
            
            async generateHash() {
                const length = this.shadowRoot.getElementById('generate-length').value;
                const alphabet = this.shadowRoot.getElementById('generate-alphabet').value;
                const prefix = this.shadowRoot.getElementById('generate-prefix').value;
                const suffix = this.shadowRoot.getElementById('generate-suffix').value;
                const resultDiv = this.shadowRoot.getElementById('generate-result');
                
                // Show loading
                resultDiv.innerHTML = '<span class="loading"></span>Generating...';
                resultDiv.className = 'result-display';
                
                try {
                    const params = new URLSearchParams({
                        length: length,
                        alphabet: alphabet,
                        raw: 'true'
                    });
                    
                    if (prefix) params.append('prefix', prefix);
                    if (suffix) params.append('suffix', suffix);
                    
                    const response = await fetch(`/api/generate?${params}`);
                    const result = await response.text();
                    
                    if (response.ok) {
                        // Keep the existing structure, just update the content
                        resultDiv.innerHTML = `<span>${result}</span><button class="copy-btn" onclick="navigator.clipboard.writeText('${result}')">Copy</button>`;
                        resultDiv.className = 'result-display success';
                    } else {
                        resultDiv.innerHTML = `<span>Error: ${response.statusText}</span>`;
                        resultDiv.className = 'result-display error';
                    }
                } catch (error) {
                    resultDiv.innerHTML = `<span>Error: ${error.message}</span>`;
                    resultDiv.className = 'result-display error';
                }
            }
            
            async generatePassword() {
                const length = this.shadowRoot.getElementById('password-length').value;
                const resultDiv = this.shadowRoot.getElementById('password-result');
                
                // Show loading
                resultDiv.innerHTML = '<span class="loading"></span>Generating...';
                resultDiv.className = 'result-display';
                
                try {
                    const params = new URLSearchParams({
                        length: length,
                        raw: 'true'
                    });
                    
                    const response = await fetch(`/api/password?${params}`);
                    const result = await response.text();
                    
                    if (response.ok) {
                        // Keep the existing structure, just update the content
                        resultDiv.innerHTML = `<span>${result}</span><button class="copy-btn" onclick="navigator.clipboard.writeText('${result}')">Copy</button>`;
                        resultDiv.className = 'result-display success';
                    } else {
                        resultDiv.innerHTML = `<span>Error: ${response.statusText}</span>`;
                        resultDiv.className = 'result-display error';
                    }
                } catch (error) {
                    resultDiv.innerHTML = `<span>Error: ${error.message}</span>`;
                    resultDiv.className = 'result-display error';
                }
            }
            
            async generateApiKey() {
                const resultDiv = this.shadowRoot.getElementById('apikey-result');
                
                // Show loading
                resultDiv.innerHTML = '<span class="loading"></span>Generating...';
                resultDiv.className = 'result-display';
                
                try {
                    const params = new URLSearchParams({
                        raw: 'true'
                    });
                    
                    const response = await fetch(`/api/api-key?${params}`);
                    const result = await response.text();
                    
                    if (response.ok) {
                        // Keep the existing structure, just update the content
                        resultDiv.innerHTML = `<span>${result}</span><button class="copy-btn" onclick="navigator.clipboard.writeText('${result}')">Copy</button>`;
                        resultDiv.className = 'result-display success';
                    } else {
                        resultDiv.innerHTML = `<span>Error: ${response.statusText}</span>`;
                        resultDiv.className = 'result-display error';
                    }
                } catch (error) {
                    resultDiv.innerHTML = `<span>Error: ${error.message}</span>`;
                    resultDiv.className = 'result-display error';
                }
            }
        }
        
        customElements.define('hash-generator', HashGenerator);
    </script>
</body>
</html>"#;