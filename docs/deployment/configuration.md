# Configuration Guide

Complete guide to environment variables, secrets management, and configuration options for HashRand.

## Environment Variables & Security Configuration

### Required Secrets for Production

HashRand requires **four cryptographically secure secrets** for production deployment:

```bash
# JWT Secret for token signing (64 hex chars = 32 bytes)
JWT_SECRET=your-64-character-hex-secret-here

# HMAC Key for magic link integrity (64 hex chars = 32 bytes) 
MAGIC_LINK_HMAC_KEY=your-64-character-hex-secret-here

# Salt for Argon2id user ID derivation (64 hex chars = 32 bytes)
ARGON2_SALT=your-64-character-hex-secret-here

# ChaCha20 encryption key for magic link encryption (64 hex chars = 32 bytes)
CHACHA_ENCRYPTION_KEY=your-64-character-hex-secret-here
```

### Email Service Configuration

```bash
# Mailtrap API integration for email delivery
MAILTRAP_API_TOKEN=your-mailtrap-api-token
MAILTRAP_INBOX_ID=your-inbox-id

# Optional email settings
FROM_EMAIL=noreply@hashrand.dev  # Default sender address
```

### Application Configuration

```bash
# Environment mode
NODE_ENV=development  # or 'production'

# Database configuration (automatic based on host)
# Development: hashrand-dev.db
# Production: hashrand.db
```

## Secret Generation

### Cryptographically Secure Generation

Generate all secrets using cryptographically secure methods:

```python
import secrets

# Generate all required secrets
print("JWT_SECRET=" + secrets.token_hex(32))
print("MAGIC_LINK_HMAC_KEY=" + secrets.token_hex(32))
print("ARGON2_SALT=" + secrets.token_hex(32))
print("CHACHA_ENCRYPTION_KEY=" + secrets.token_hex(32))
```

### Secret Requirements

- **Length**: All secrets must be exactly 64 hexadecimal characters (32 bytes)
- **Randomness**: Use cryptographically secure random generators only
- **Uniqueness**: Each secret must be unique across all environments
- **Rotation**: Implement secret rotation procedures for production

## Development Setup

### 1. Create `.env` File

Create `.env` file in project root (automatically loaded by `just dev`):

```bash
# Development secrets (example - generate your own)
JWT_SECRET=e6024c8eada7b42bee415ef56eb597c62c170681f1946a8cb899fc5c102e2c11
MAGIC_LINK_HMAC_KEY=464c57289ac9f1a0a93c98ebe1ced0c31ac777798b9ce55cd67a358db5931b26
ARGON2_SALT=637de2cf5c738c757fb4e663685721bf3dca002da5168626dbe07f1b9907e1e3
CHACHA_ENCRYPTION_KEY=8db6db662a0af8881550bbda8dc4c6223c5485bf38964c5181a037d9f95d4a32
NODE_ENV=development

# Email configuration (optional for development)
MAILTRAP_API_TOKEN=your-dev-token
MAILTRAP_INBOX_ID=your-dev-inbox
```

### 2. Start Development Environment

```bash
# Automatically loads secrets from .env
just dev
```

## Production Deployment

### Spin Variables Method

For production, pass secrets as Spin variables:

```bash
# Deploy with secrets as Spin variables
SPIN_VARIABLE_JWT_SECRET="your-production-secret" \
SPIN_VARIABLE_MAGIC_LINK_HMAC_KEY="your-production-secret" \
SPIN_VARIABLE_USER_ID_HMAC_KEY="your-production-secret" \
SPIN_VARIABLE_ARGON2_SALT="your-production-secret" \
SPIN_VARIABLE_MAILTRAP_API_TOKEN="your-mailtrap-token" \
SPIN_VARIABLE_MAILTRAP_INBOX_ID="your-inbox-id" \
spin-cli deploy --runtime-config-file runtime-config.toml
```

### Justfile Deployment

```bash
# Deploy using justfile (loads from .env automatically)
just deploy
```

## Security Architecture

### 🛡️ Spin Variables Security

- **Spin Variables**: Uses Fermyon Spin's native variable system (`spin_sdk::variables::get`)
- **Secret Marking**: Variables marked as `secret = true` in `spin.toml`
- **No Hardcoding**: All secrets externalized and never committed to repository
- **32-Byte Minimum**: All secrets use 256-bit cryptographic strength
- **`.env` Support**: Development environment loads automatically from `.env` file

### Variable Definitions in `spin.toml`

```toml
[variables]
jwt_secret = { required = true, secret = true }
magic_link_hmac_key = { required = true, secret = true }
user_id_hmac_key = { required = true, secret = true }
argon2_salt = { required = true, secret = true }
chacha_encryption_key = { required = true, secret = true }
mailtrap_api_token = { required = false, secret = true }
mailtrap_inbox_id = { required = false }
```

## Email Configuration

### Mailtrap Integration

#### Production Email Delivery

```bash
# Required for production email
MAILTRAP_API_TOKEN=your-production-api-token
MAILTRAP_INBOX_ID=your-production-inbox-id

# Optional custom domain configuration
MAILTRAP_CUSTOM_DOMAIN=mailer.hashrand.com
```

#### Email Endpoints

- **Sandbox**: `https://sandbox.api.mailtrap.io/api/send/{inbox_id}`
- **Production**: `https://send.api.mailtrap.io/api/send/{inbox_id}`
- **Custom Domain**: `https://your-custom-domain.com/api/send/{inbox_id}`

#### Email Template Languages

Supported language codes for email templates:

```bash
# Supported email languages
es, en, fr, de, pt, ru, zh, ja, ar, hi, ca, gl, eu
```

### Development Email Mode

In development mode:
- Magic links are logged to console
- Email sending is optional (fallback mode)
- Extended debug display for mobile development

## Database Configuration

### Environment-Aware Database Selection

Automatic database selection based on request host:

| Environment | Host Pattern | Database File | Features |
|-------------|--------------|---------------|----------|
| Development | `localhost`, `*.ts.net` | `hashrand-dev.db` | Extended timeouts, debug logging |
| Production | All other hosts | `hashrand.db` | Standard security settings |

### Database Paths

```toml
# runtime-config.toml
[variables]
database_url = "sqlite://data/hashrand.db"

[variables.development]
database_url = "sqlite://data/hashrand-dev.db"
```

### Database Directory Structure

```
data/
├── hashrand-dev.db      # Development database
├── hashrand.db          # Production database
├── *.db-journal         # SQLite journal files (ignored in git)
└── *.sqlite*            # Other SQLite files (ignored in git)
```

## JWT Configuration

### Token Durations

| Environment | Access Token | Refresh Token |
|-------------|--------------|---------------|
| Development | 3 minutes | 15 minutes |
| Production | 15 minutes | 7 days |

### Token Security Features

- **Access Tokens**: Short-lived, included in JSON responses
- **Refresh Tokens**: HttpOnly, Secure, SameSite=Strict cookies
- **Automatic Refresh**: Client-side automatic token renewal
- **Secure Storage**: Refresh tokens inaccessible to JavaScript

## Magic Link Configuration

### Security Settings

| Parameter | Development | Production |
|-----------|-------------|------------|
| Expiration | 15 minutes | 5 minutes |
| Token Length | 44 characters (Base58) | 44 characters (Base58) |
| Encryption | ChaCha20 | ChaCha20 |
| Integrity | Blake2b-keyed | Blake2b-keyed |

### Magic Link Features

- **One-Time Use**: Links consumed immediately after validation
- **Time-Limited**: Automatic expiration prevents replay attacks
- **Encrypted**: ChaCha20 encryption protects link content
- **Integrity Protected**: Blake2b-keyed prevents tampering

## Performance Configuration

### WebAssembly Settings

- **Cold Start**: ~5ms
- **Response Time**: <1ms for most requests
- **Memory Usage**: ~2MB baseline
- **Throughput**: >10,000 requests/second

### Optimization Features

- **Blake2b Performance**: 2x faster than SHA3
- **Unified Cryptography**: Single cryptographic family
- **Minimal Dependencies**: Optimized dependency tree
- **Efficient Encoding**: Base58 for optimal character set

---

*For quick setup, see [Quick Start Guide](./quick-start.md)*  
*For production deployment, see [Production Deployment](./production.md)*  
*For development commands, see [Development Guide](./development.md)*