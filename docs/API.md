# HashRand API Documentation

## Overview

REST API for generating cryptographically secure random strings. Available as HTTP server with embedded web interface.

## Quick Start

```bash
# Start server
hashrand --serve 8080

# Or with justfile
just dev                    # Development (API:8080 + Frontend:3000)
just serve                  # Production (single server:8080)
```

**Web Interface**: Visit `http://localhost:8080`
**API Base URL**: `http://localhost:8080/api`

## Server Modes

| Mode | Command | Description |
|------|---------|-------------|
| **Development** | `just dev` | API-only server + Vite dev server |
| **Production** | `just build && just serve` | Self-contained binary with embedded UI |

## API Endpoints

All endpoints return plain text (no newline by default).

### GET /api/generate

Generate random hash with customizable options.

**Parameters:**
- `length` (optional): 2-128, default 21
- `alphabet` (optional): `base58` (default), `no-look-alike`, `full`, `full-with-symbols`
- `prefix` (optional): String to prepend
- `suffix` (optional): String to append
- `raw` (optional): `true` (default), `false` adds newline

**Examples:**
```bash
curl "localhost:8080/api/generate"
curl "localhost:8080/api/generate?length=16&alphabet=full"
curl "localhost:8080/api/generate?prefix=user_&suffix=_id&length=12"
```

### GET /api/password

Generate secure password.

**Parameters:**
- `length` (optional): Dynamic minimum based on alphabet (21+ symbols, 24+ no-look-alike)
- `alphabet` (optional): `full-with-symbols` (default), `no-look-alike`
- `raw` (optional): `true` (default), `false` adds newline

**Examples:**
```bash
curl "localhost:8080/api/password"                              # aB3*fG7$hI9@kL2#mN5^pQ8!
curl "localhost:8080/api/password?alphabet=no-look-alike"       # K9mN3pQwTjH6XpRaBcDeFgHkMn
curl "localhost:8080/api/password?length=30"                    # Longer password
```

### GET /api/api-key

Generate API key with `ak_` prefix.

**Parameters:**
- `length` (optional): Dynamic minimum based on alphabet (44+ full, 47+ no-look-alike)
- `alphabet` (optional): `full` (default), `no-look-alike`
- `raw` (optional): `true` (default), `false` adds newline

**Examples:**
```bash
curl "localhost:8080/api/api-key"                               # ak_x7K9mN3pQ5vB8zL2jH6tR4wY1sE9aF0cD6bG4hM7n
curl "localhost:8080/api/api-key?alphabet=no-look-alike"        # ak_K9mN3pQwTjH6XpRaBcDeFgHkMnPqRtWxYzAbCdEf
curl "localhost:8080/api/api-key?length=60"                     # Longer key
```

### GET /api/version

Get version information (JSON).

```bash
curl "localhost:8080/api/version"
# Response: {"version":"0.2.9"}
```

## Alphabets

| Alphabet | Characters | Count | Use Case |
|----------|------------|-------|----------|
| `base58` | Bitcoin alphabet, excludes 0,O,I,l | 58 | Default, readable |
| `no-look-alike` | Extra safe, excludes 0,O,I,l,1,2,5,S,s,o,u,v,Z | 49 | Easy typing |
| `full` | All alphanumeric 0-9A-Za-z | 62 | Maximum charset |
| `full-with-symbols` | Alphanumeric + `-_*^@#+!?$%` | 73 | Maximum entropy |

## Security

### Default Settings (Fast)
- Localhost binding (`127.0.0.1`)
- No rate limiting
- No CORS
- Basic parameter validation

### Production Settings
```bash
hashrand --serve 8080 \
  --enable-rate-limiting --rate-limit 100 \
  --max-param-length 32 \
  --listen-all-ips  # Only if needed for network access
```

**⚠️ Important**: Always use HTTPS reverse proxy in production (nginx, Caddy, etc.)

## Error Responses

- `400 Bad Request`: Invalid parameters
- `429 Too Many Requests`: Rate limit exceeded (if enabled)
- `500 Internal Server Error`: Server error

## Integration Examples

### JavaScript
```javascript
const response = await fetch('http://localhost:8080/api/generate?length=16');
const hash = await response.text();
```

### Python
```python
import requests
hash = requests.get('http://localhost:8080/api/password').text
```

### Shell
```bash
API_KEY=$(curl -s "http://localhost:8080/api/api-key")
echo "Generated: $API_KEY"
```

## Development

```bash
# Start development environment
just dev                    # API server + Vite dev server

# Manual development
npm run dev                 # Frontend (localhost:3000)
cargo run -- --serve 8080  # API server (localhost:8080)

# Production build
just build                  # Build optimized version
just serve                  # Run production server
```

## Web Interface

The embedded web interface provides:
- Menu-based navigation (Generic Hash, Password, API Key)
- Real-time generation with instant results
- Copy-to-clipboard functionality
- Alphabet selection with automatic length adjustment
- Responsive design for all devices

Built with Lit 3 framework and Vite build system for optimal performance.