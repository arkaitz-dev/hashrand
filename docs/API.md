# HashRand API Documentation

## Overview

The HashRand HTTP API provides programmatic access to all hash generation functionality available in the CLI tool, except for file system operations. The API is designed to be simple, secure, and easy to integrate with any programming language or tool that can make HTTP requests.

## Server Configuration

### Starting the Server

Server behavior differs based on build type:

**Development Mode (Debug Build)**:
```bash
# API-only server (frontend handled by Vite dev server)
cargo run -- --serve 8080

# Vite dev server (separate terminal)
npm run dev  # Runs on http://localhost:3000 with HMR
```

**Production Mode (Release Build)**:
```bash
# Self-contained binary with embedded web assets
npm run build                           # Generate optimized assets
cargo build --release                  # Embed assets in binary
./target/release/hashrand --serve 8080 # Serves both API and web UI
```

**General Options**:
```bash
# Start on all network interfaces (use with caution)
hashrand --serve 8080 --listen-all-ips

# Enable security features for production
hashrand --serve 8080 --enable-rate-limiting --rate-limit 100 --max-param-length 32

# Development with CORS enabled
hashrand --serve 8080 --enable-cors
```

### Configuration Options

| Option | Default | Description |
|--------|---------|-------------|
| Port | Required | TCP port number for the server to listen on |
| Host | `127.0.0.1` | Default binding to localhost only |
| `--listen-all-ips` | `false` | When enabled, binds to `0.0.0.0` for network access |
| `--max-param-length` | `32` | Maximum length for prefix and suffix parameters |
| `--enable-rate-limiting` | `false` | Enable per-IP rate limiting |
| `--rate-limit` | `100` | Requests per second limit (requires `--enable-rate-limiting`) |
| `--enable-cors` | `false` | Enable CORS headers for cross-origin requests |
| `--max-body-size` | `1024` | Maximum request body size in bytes |

### Security Considerations

- **Default localhost binding**: The server binds to `127.0.0.1` by default, preventing unintended network exposure
- **Explicit network binding**: Use `--listen-all-ips` only when you need network access and understand the security implications
- **No authentication**: The API has no built-in authentication; use a reverse proxy if authentication is needed
- **No file system access**: API endpoints cannot create files or directories for security
- **Parameter validation**: Prefix and suffix parameters are validated against configured length limits
- **Rate limiting**: Optional per-IP rate limiting prevents request flooding (disabled by default for performance)
- **Request size limits**: Configurable request body size limits prevent resource exhaustion
- **CORS control**: CORS is disabled by default; enable only when cross-origin access is required

## API Endpoints

### Base URL

```
http://localhost:PORT/api
```

All endpoints are under the `/api` path and use HTTP GET method.

---

## GET /api/generate

Generate a random hash with customizable options.

### Request

**URL**: `/api/generate`

**Method**: `GET`

**Query Parameters**:

| Parameter | Type | Default | Description | Valid Values |
|-----------|------|---------|-------------|--------------|
| `length` | integer | `21` | Length of the hash to generate | `2-128` |
| `alphabet` | string | `"base58"` | Character set to use | `"base58"`, `"no-look-alike"`, `"full"`, `"full-with-symbols"` |
| `raw` | boolean | `true` | If false, adds a newline character | `true`, `false` |
| `prefix` | string | none | String to prepend to the hash | Any valid string |
| `suffix` | string | none | String to append to the hash | Any valid string |

### Response

**Content-Type**: `text/plain`

**Body**: The generated hash as plain text

**Status Codes**:
- `200 OK`: Successful generation
- `400 Bad Request`: Invalid parameters (e.g., length out of range, invalid alphabet, parameter too long)
- `429 Too Many Requests`: Rate limit exceeded (when rate limiting is enabled)
- `500 Internal Server Error`: Server error during generation

### Examples

#### Basic hash generation (default settings)
```bash
curl "http://localhost:8080/api/generate"
# Response: 3K9mVnYc5wTjH2XpRsB7L
```

#### Custom length and alphabet
```bash
curl "http://localhost:8080/api/generate?length=32&alphabet=full"
# Response: x7K9mN3pQ5vB8zL2jH6tR4wY1sE9aF0c
```

#### With prefix and suffix
```bash
curl "http://localhost:8080/api/generate?prefix=user_&suffix=_id&length=8"
# Response: user_K3m5Hn2L_id
```

#### Using no-look-alike alphabet
```bash
curl "http://localhost:8080/api/generate?alphabet=no-look-alike&length=16"
# Response: 3K9mVnYc5wTjH2Xp
```

#### With newline character
```bash
curl "http://localhost:8080/api/generate?raw=false"
# Response: 3K9mVnYc5wTjH2XpRsB7L\n
```

### Alphabet Options

| Alphabet | Characters | Count | Use Case |
|----------|------------|-------|----------|
| `base58` | `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz` | 58 | Default, excludes confusing characters (0, O, I, l) |
| `no-look-alike` | `23456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz` | 57 | Extra safety, excludes 0, O, I, l, 1 |
| `full` | `0-9A-Za-z` | 62 | All alphanumeric characters |
| `full-with-symbols` | `0-9A-Za-z-_*^@#+!?$%` | 73 | Maximum entropy with symbols |

---

## GET /api/api-key

Generate a secure API key in the standard format.

### Request

**URL**: `/api/api-key`

**Method**: `GET`

**Query Parameters**:

| Parameter | Type | Default | Description | Valid Values |
|-----------|------|---------|-------------|--------------|
| `length` | integer | `44` | Length of the random part (excludes ak_ prefix) | `44-64` |
| `raw` | boolean | `true` | If false, adds a newline character | `true`, `false` |

### Response

**Content-Type**: `text/plain`

**Body**: API key in format `ak_[44 random characters]` (47 characters total)

**Status Codes**:
- `200 OK`: Successful generation
- `500 Internal Server Error`: Server error during generation

### Examples

#### Generate API key (default)
```bash
curl "http://localhost:8080/api/api-key"
# Response: ak_x7K9mN3pQ5vB8zL2jH6tR4wY1sE9aF0cD6bG4hM7n
```

#### Generate API key with custom length
```bash
curl "http://localhost:8080/api/api-key?length=60"
# Response: ak_x7K9mN3pQ5vB8zL2jH6tR4wY1sE9aF0cD6bG4hM7nQ9kF5mD3pT8zL

#### Generate API key with newline
```bash
curl "http://localhost:8080/api/api-key?raw=false"
# Response: ak_x7K9mN3pQ5vB8zL2jH6tR4wY1sE9aF0cD6bG4hM7n\n
```

### API Key Characteristics

- **Format**: `ak_` prefix followed by 44-64 configurable random characters
- **Length**: 47-67 characters total (3 char prefix + configurable suffix)
- **Alphabet**: Full alphanumeric (62 characters)
- **Entropy**: 256-384 bits (quantum-resistant)
- **Default**: 44 characters (256-bit entropy)
- **Use Case**: Authentication tokens, service keys

---

## GET /api/password

Generate a secure password with symbols.

### Request

**URL**: `/api/password`

**Method**: `GET`

**Query Parameters**:

| Parameter | Type | Default | Description | Valid Values |
|-----------|------|---------|-------------|--------------|
| `length` | integer | `21` | Length of the password | `21-44` |
| `raw` | boolean | `true` | If false, adds a newline character | `true`, `false` |

### Response

**Content-Type**: `text/plain`

**Body**: The generated password as plain text

**Status Codes**:
- `200 OK`: Successful generation
- `400 Bad Request`: Invalid length (must be 21-44)
- `500 Internal Server Error`: Server error during generation

### Examples

#### Generate password (default 21 characters)
```bash
curl "http://localhost:8080/api/password"
# Response: K@9m#N3p$5v*8z!2j^6t
```

#### Generate longer password
```bash
curl "http://localhost:8080/api/password?length=30"
# Response: K@9m#N3p$5v*8z!2j^6tR+4w?1s_9a
```

#### Maximum length password
```bash
curl "http://localhost:8080/api/password?length=44"
# Response: K@9m#N3p$5v*8z!2j^6tR+4w?1s_9aF-0c%6b@4h#7n
```

#### With newline
```bash
curl "http://localhost:8080/api/password?length=25&raw=false"
# Response: K@9m#N3p$5v*8z!2j^6tR+4w?\n
```

### Password Characteristics

- **Alphabet**: Full alphanumeric plus symbols (73 characters total)
- **Symbols included**: `-_*^@#+!?$%`
- **Minimum length**: 21 characters (128-bit entropy)
- **Maximum length**: 44 characters
- **Use Case**: User passwords, secure credentials

---

## Error Handling

All endpoints return appropriate HTTP status codes and plain text error descriptions when applicable.

### Common Error Responses

#### 400 Bad Request
Returned when request parameters are invalid.

Example scenarios:
- Length parameter outside valid range (2-128 for generate, 21-44 for password)
- Invalid alphabet name
- Malformed query parameters
- Prefix or suffix parameter exceeds configured maximum length

#### 429 Too Many Requests
Returned when the client exceeds the configured rate limit (only when rate limiting is enabled).

This response indicates that the client should wait before making additional requests. The specific rate limits are configured via the `--rate-limit` server option.

#### 500 Internal Server Error
Returned when the server encounters an unexpected error during processing.

---

## Integration Examples

### JavaScript/Node.js
```javascript
// Using fetch API
const response = await fetch('http://localhost:8080/api/generate?length=16');
const hash = await response.text();
console.log('Generated hash:', hash);
```

### Python
```python
import requests

response = requests.get('http://localhost:8080/api/generate', 
                        params={'length': 16, 'alphabet': 'full'})
hash = response.text
print(f'Generated hash: {hash}')
```

### Shell Script
```bash
#!/bin/bash
# Generate a unique session ID
SESSION_ID=$(curl -s "http://localhost:8080/api/generate?prefix=session_&length=12")
echo "New session: $SESSION_ID"
```

### PowerShell
```powershell
$hash = Invoke-RestMethod -Uri "http://localhost:8080/api/generate?length=20"
Write-Host "Generated hash: $hash"
```

### Go
```go
resp, err := http.Get("http://localhost:8080/api/generate?length=16")
if err != nil {
    log.Fatal(err)
}
defer resp.Body.Close()
hash, _ := io.ReadAll(resp.Body)
fmt.Printf("Generated hash: %s\n", hash)
```

---

## Performance Considerations

- **Stateless**: Each request is independent; no session management
- **Lightweight**: Minimal overhead, fast response times
- **Concurrent**: Handles multiple simultaneous requests efficiently
- **No caching**: Every request generates a new random value

---

## Security Best Practices

1. **Use HTTPS in production**: Place the API behind a reverse proxy with TLS
2. **Implement rate limiting**: Prevent abuse with rate limiting at proxy level
3. **Add authentication**: Use API keys or tokens via reverse proxy if needed
4. **Network isolation**: Keep the server on internal networks when possible
5. **Monitor usage**: Log and monitor API access for unusual patterns
6. **Regular updates**: Keep dependencies updated for security patches

---

## Comparison with CLI

| Feature | CLI | API |
|---------|-----|-----|
| Hash generation | ✅ | ✅ |
| API key generation | ✅ | ✅ |
| Password generation | ✅ | ✅ |
| File creation | ✅ | ❌ |
| Directory creation | ✅ | ❌ |
| Collision checking | ✅ | ❌ |
| Audit logging | ✅ | ❌ |
| Unix permissions | ✅ | ❌ |

The API focuses on generation functionality while excluding file system operations for security reasons.

---

## Web Interface Development

### Modern Frontend Architecture

The web interface is built with modern web technologies:

- **Backend**: Rust 1.89.0 (latest stable) with tokio async runtime and axum web framework
- **Frontend Framework**: Lit 3.3.1 (Web Components library with standard decorators)
- **Build Tool**: Vite 7.1.1 (Fast build system with HMR)
- **Decorators**: Official Lit + Babel configuration using "@babel/plugin-proposal-decorators" version "2023-05"
- **Architecture**: Component-based with Shadow DOM encapsulation using modern `@property/@state` syntax
- **Styling**: External CSS with "wc-" prefixed classes for reusability

### Development & Production Workflows

**Development Mode (Debug Build)**:
```bash
# Terminal 1: Start Vite dev server (frontend with HMR)
npm run dev                    # Runs on http://localhost:3000

# Terminal 2: Start API-only server  
cargo run -- --serve 8080     # API endpoints only (no static files)
cargo run -- --version        # Check version
```

**Production Mode (Release Build)**:
```bash
# Step 1: Build optimized assets
npm run build                         # Generates optimized files in dist/

# Step 2: Build release binary with embedded assets  
cargo build --release                # Embeds dist/ assets in binary

# Step 3: Deploy single binary
./target/release/hashrand --serve 8080  # Self-contained server (~3.1MB binary)
```

**Benefits of Embedded Assets**:
- ✅ **Single file deployment**: No external dependencies
- ✅ **Version consistency**: Assets always match binary version
- ✅ **Zero configuration**: Works anywhere without setup
- ✅ **Simplified distribution**: Just copy the binary

### File Structure
```
├── web-ui/                          # Frontend source (development)
│   ├── index.html                  # Main HTML template
│   └── src/
│       ├── index.js                # Entry point
│       ├── css/main.css            # Shared CSS styles
│       └── components/             # Lit Web Components
│           ├── hash-generator.js   # Main menu component
│           ├── hash-result.js      # Unified result display
│           ├── generic-hash-view.js
│           ├── password-view.js
│           └── api-key-view.js
├── dist/                           # Compiled files (embedded in release)
│   ├── index.html
│   └── assets/
│       ├── index-*.js              # Bundled JavaScript (~11 kB gzipped)
│       └── index-*.css             # Bundled CSS (~1.2 kB gzipped)
├── src/server/                     # Rust backend with embedded assets
│   └── routes.rs                   # Asset embedding logic
└── vite.config.js                  # Build configuration
```

### Component Architecture

Each view is a Lit component with:
- **@state** reactive properties for UI state
- **@query** decorators for DOM element access
- **Event handlers** using modern `@click`, `@input` syntax
- **Render methods** returning `html` template literals
- **CSS-in-JS** with external CSS file loading for shared styles

### Development Tools

- **Hot Module Replacement**: Instant updates during development
- **API Proxy**: Seamless integration between frontend and backend
- **Bundle Optimization**: Production builds are optimized and minified
- **TypeScript-style**: Uses decorators and modern JavaScript patterns

---

## Versioning

The API version corresponds to the hashrand tool version:
- Current version: 0.2.9
- **CLI Version Option**: Use `hashrand --version` or `hashrand -V` to check current version
- Web Interface: Lit 3.3.1 + Vite 7.1.1 with standard decorators and embedded assets support
- **Development Workflow**: Justfile commands available (`just --list` to see all options)
- Binary distribution: Self-contained (~3.1MB with embedded frontend)
- API stability: Stable
- Backward compatibility: Maintained within major versions
- Embedded assets: Available since v0.2.8

---

## Support

For issues, feature requests, or questions:
- GitHub Issues: [Create an issue](https://github.com/arkaitz-dev/hashrand/issues)
- Security Issues: See [SECURITY.md](../SECURITY.md)
- Email: me@arkaitz.dev