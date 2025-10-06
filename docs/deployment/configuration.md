# Configuration Guide

Complete guide to environment variables, secrets management, and configuration options for HashRand.

## Environment Variables & Security Configuration

### Required Secrets for Production (v1.6.13+)

HashRand requires **cryptographically secure secrets** for production deployment. Starting from v1.6.13, user ID derivation uses **three 64-byte keys** for maximum security:

```bash
# JWT Secret for token signing (64 hex chars = 32 bytes)
JWT_SECRET=your-64-character-hex-secret-here

# HMAC Key for magic link integrity (64 hex chars = 32 bytes)
MAGIC_LINK_HMAC_KEY=your-64-character-hex-secret-here

# === User ID Derivation Keys (v1.6.13+) ===
# HMAC Key for user ID keyed hashing (128 hex chars = 64 bytes)
USER_ID_HMAC_KEY=your-128-character-hex-secret-here

# Salt for Argon2id dynamic salt derivation (128 hex chars = 64 bytes)
ARGON2_SALT=your-128-character-hex-secret-here

# Compression key for final user ID derivation (128 hex chars = 64 bytes)
USER_ID_ARGON2_COMPRESSION=your-128-character-hex-secret-here

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

# Database configuration (Spin variable-based)
# Development: database_name = "hashrand-dev" (spin-dev.toml)
# Production: database_name = "hashrand" (spin-prod.toml)
```

## Secret Generation

### Cryptographically Secure Generation (v1.6.13+)

Generate all secrets using cryptographically secure methods:

```python
import secrets

# Standard 32-byte secrets (64 hex chars)
print("JWT_SECRET=" + secrets.token_hex(32))
print("MAGIC_LINK_HMAC_KEY=" + secrets.token_hex(32))
print("CHACHA_ENCRYPTION_KEY=" + secrets.token_hex(32))

# Enhanced 64-byte secrets for user ID derivation (v1.6.13+)
print("USER_ID_HMAC_KEY=" + secrets.token_hex(64))
print("ARGON2_SALT=" + secrets.token_hex(64))
print("USER_ID_ARGON2_COMPRESSION=" + secrets.token_hex(64))
```

### Secret Requirements (v1.6.13+)

- **Standard Keys**: JWT, Magic Link, ChaCha20 - 64 hexadecimal characters (32 bytes)
- **User ID Keys**: HMAC, ARGON2_SALT, COMPRESSION - 128 hexadecimal characters (64 bytes)
- **Randomness**: Use cryptographically secure random generators only
- **Uniqueness**: Each secret must be unique across all environments
- **Domain Separation**: Development and production must use different secrets
- **Rotation**: Implement secret rotation procedures for production

## Development Setup

### 1. Create `.env` File

Create `.env` file in project root (automatically loaded by `just dev`):

```bash
# Development secrets (example - generate your own)
# Standard 32-byte secrets (64 hex chars)
JWT_SECRET=e6024c8eada7b42bee415ef56eb597c62c170681f1946a8cb899fc5c102e2c11
MAGIC_LINK_HMAC_KEY=464c57289ac9f1a0a93c98ebe1ced0c31ac777798b9ce55cd67a358db5931b26
CHACHA_ENCRYPTION_KEY=8db6db662a0af8881550bbda8dc4c6223c5485bf38964c5181a037d9f95d4a32

# Enhanced 64-byte secrets for user ID derivation (v1.6.13+)
USER_ID_HMAC_KEY=571ea410cd63ab391278171f6045d9f9dfc1b78644edb6a3182f56fc4833726ef1780c2a0d22de5b3ad84e54ac2bef7790a982570ddcc21c8774de931ea2e771
ARGON2_SALT=3592e268b87094380a640edfdfa94ffc97baecd47b83ef0989bee489fae0b15e81c87da37f1e22b544798392d73775e3035349e725553ec2c8548847871a92fd
USER_ID_ARGON2_COMPRESSION=63c604db6c10c875338be1ce921f1796f22e52ef1ff3cf140b8d896d73901f38b8de8bc8c4fd01647818fc2994c3179147265430b1d1c1da02ad4839047383be

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
# Standard 32-byte secrets
jwt_secret = { required = true, secret = true }
magic_link_hmac_key = { required = true, secret = true }
chacha_encryption_key = { required = true, secret = true }

# Enhanced 64-byte secrets for user ID derivation (v1.6.13+)
user_id_hmac_key = { required = true, secret = true }
argon2_salt = { required = true, secret = true }
user_id_argon2_compression = { required = true, secret = true }

# Email configuration
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

### Spin Variable-Based Database Selection

Database selection using Spin configuration variables:

| Environment | Configuration File | Database Variable                | Database File     | Features                         |
| ----------- | ------------------ | -------------------------------- | ----------------- | -------------------------------- |
| Development | `spin-dev.toml`    | `database_name = "hashrand-dev"` | `hashrand-dev.db` | Extended timeouts, debug logging |
| Production  | `spin-prod.toml`   | `database_name = "hashrand"`     | `hashrand.db`     | Standard security settings       |

### Runtime Configuration

```toml
# runtime-config.toml - Database path definitions
[sqlite_database.hashrand-dev]
type = "spin"
path = "./data/hashrand-dev.db"

[sqlite_database.hashrand]
type = "spin"
path = "./data/hashrand.db"
```

### Configuration Files

```toml
# spin-dev.toml - Development
[variables]
database_name = { default = "hashrand-dev" }

# spin-prod.toml - Production
[variables]
database_name = { default = "hashrand" }
```

### Database Directory Structure

```
data/
├── hashrand-dev.db      # Development database (spin-dev.toml)
├── hashrand.db          # Production database (spin-prod.toml)
├── *.db-journal         # SQLite journal files (ignored in git)
└── *.sqlite*            # Other SQLite files (ignored in git)
```

## JWT Configuration

### Token Durations

| Environment | Access Token | Refresh Token |
| ----------- | ------------ | ------------- |
| Development | 3 minutes    | 15 minutes    |
| Production  | 15 minutes   | 7 days        |

### Token Security Features

- **Access Tokens**: Short-lived, included in JSON responses
- **Refresh Tokens**: HttpOnly, Secure, SameSite=Strict cookies
- **Automatic Refresh**: Client-side automatic token renewal
- **Secure Storage**: Refresh tokens inaccessible to JavaScript

## Magic Link Configuration

### Security Settings

| Parameter    | Development            | Production             |
| ------------ | ---------------------- | ---------------------- |
| Expiration   | 15 minutes             | 5 minutes              |
| Token Length | 44 characters (Base58) | 44 characters (Base58) |
| Encryption   | ChaCha20               | ChaCha20               |
| Integrity    | Blake3 keyed           | Blake3 keyed           |

### Magic Link Features

- **One-Time Use**: Links consumed immediately after validation
- **Time-Limited**: Automatic expiration prevents replay attacks
- **Encrypted**: ChaCha20 encryption protects link content
- **Integrity Protected**: Blake3 keyed prevents tampering

## Performance Configuration

### WebAssembly Settings

- **Cold Start**: ~5ms
- **Response Time**: <1ms for most requests
- **Memory Usage**: ~2MB baseline
- **Throughput**: >10,000 requests/second

### Optimization Features

- **Blake3 Performance**: ~6x faster than SHA3 with WASM SIMD
- **Unified Cryptography**: Single cryptographic family
- **Minimal Dependencies**: Optimized dependency tree
- **Efficient Encoding**: Base58 for optimal character set

---

_For quick setup, see [Quick Start Guide](./quick-start.md)_  
_For production deployment, see [Production Deployment](./production.md)_  
_For development commands, see [Development Guide](./development.md)_
