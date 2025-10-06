# API Endpoints

**🔒 Authentication Required**: All generation endpoints require a valid Bearer token in the Authorization header. Obtain tokens through the [magic link authentication flow](./authentication.md).

**🛡️ SignedRequest Security (v1.6.10+)**: All POST endpoints implement **strict authentication method separation** with enterprise-grade security validation. See [SignedRequest Documentation](./authentication.md#signedrequest-universal-structure-with-strict-security-v16100) for details.

**✨ SignedResponse Architecture (v1.6.11+)**: ALL endpoints now return **Ed25519-signed responses** with cryptographic integrity validation. Complete SignedResponse implementation includes secure HTTP cookie delivery for authentication endpoints.

**🍪 Secure Cookie Implementation**: Authentication endpoints deliver refresh tokens via secure HTTP cookies with enterprise security attributes (HttpOnly, Secure, SameSite=Strict, Max-Age, Path=/).

## Quick Reference

| Endpoint                          | Method          | Auth    | Description                           |
| --------------------------------- | --------------- | ------- | ------------------------------------- |
| `/api/version`                    | GET             | No      | Get API version                       |
| `/api/login/`                     | POST            | No      | Magic link auth (email_lang required) |
| `/api/custom`                     | GET/POST        | Yes     | Custom hashes                         |
| `/api/password`                   | GET/POST        | Yes     | Secure passwords                      |
| `/api/api-key`                    | GET/POST        | Yes     | API keys                              |
| `/api/mnemonic`                   | GET/POST        | Yes     | BIP39 mnemonics                       |
| `/api/shared-secret/create`       | POST            | Yes     | Create shared secret (v1.8.0+)        |
| `/api/shared-secret/{hash}`       | GET/POST/DELETE | Partial | Retrieve/delete shared secret         |
| `/api/shared-secret/confirm-read` | GET             | No      | Confirm read tracking                 |
| `/api/users`                      | GET/POST/DELETE | Yes     | User management ⚠️ **FUTURE**         |

## Generate Custom Hashes

```
GET /api/custom         # Random generation (requires authentication)
POST /api/custom        # Deterministic generation with seed (requires authentication)
```

**GET Parameters:**

- `length` (2-128, default: 21) - Length of generated hash
- `alphabet` (string, default: "base58") - Character set to use
- `prefix` (string, max 32 chars) - Text to prepend
- `suffix` (string, max 32 chars) - Text to append
- `raw` (boolean, default: true) - If false, adds newline

**POST Body (SignedRequest v1.6.10+):**

```json
{
  "payload": {
    "seed": "44-character base58 string (required)",
    "length": 21,
    "alphabet": "base58",
    "prefix": "optional_prefix",
    "suffix": "optional_suffix"
  },
  "signature": "ed25519_signature_hex_128_chars"
}
```

**Payload Fields:**

- `seed` (required) - 44-character base58 string for deterministic generation
- `length` (2-128) - Length of generated hash
- `alphabet` (string) - Character set to use
- `prefix` (string, max 32 chars) - Text to prepend
- `suffix` (string, max 32 chars) - Text to append

**Response Format (SignedResponse v1.6.11+):**

```json
{
  "payload": {
    "hash": "generated_hash_here",
    "seed": "base58_seed_string",
    "otp": "123456789",
    "timestamp": 1692812400
  },
  "signature": "ed25519_signature_hex_128_chars_server_signed"
}
```

**Response Fields:**

- `payload.hash` - The generated hash string
- `payload.seed` - Base58 seed used for generation
- `payload.otp` - Numeric timestamp-based one-time pad
- `payload.timestamp` - Unix timestamp of generation
- `signature` - Ed25519 digital signature from server for response integrity

**Examples:**

```bash
# Random generation (requires Bearer token)
curl -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  "http://localhost:3000/api/custom?length=16&alphabet=full&prefix=app_&suffix=_key"
# Response: {"hash":"app_A1b2C3d4E5f6G7h8_key","seed":"2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR","otp":"743628951","timestamp":1692812400}

# Deterministic generation with seed (requires Bearer token)
curl -X POST "http://localhost:3000/api/custom" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"seed":"2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR","length":16,"alphabet":"full","prefix":"app_","suffix":"_key"}'
# Response: {"hash":"app_T4sHeyqXb1on6mAH_key","seed":"2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR","otp":"743628951","timestamp":1692812400}
```

## Generate Secure Passwords

```
GET /api/password        # Random generation
POST /api/password       # Deterministic generation with seed
```

**GET Parameters:**

- `length` (21-44, default: 21) - Length of password
- `alphabet` (string, default: "full-with-symbols") - Character set
- `raw` (boolean, default: true) - Output formatting

**POST Body (JSON):**

- `seed` (required) - 44-character base58 string for deterministic generation
- `length` (21-44) - Length of password
- `alphabet` (string) - Character set

**Examples:**

```bash
# Random generation
curl "http://localhost:3000/api/password?length=32&alphabet=no-look-alike"
# Response: {"hash":"mKp7qR9tYwX4zV8nBfGhJ3dCxL6sWe2A","seed":"64edd1cfcc17..."}

# Deterministic generation with seed
curl -X POST "http://localhost:3000/api/password" \
  -H "Content-Type: application/json" \
  -d '{"seed":"2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR","length":25,"alphabet":"full-with-symbols"}'
# Response: {"hash":"xxFu2q4H4al2vNkW7r*uJoe!C","seed":"2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR"}
```

## Generate API Keys

```
GET /api/api-key         # Random generation
POST /api/api-key        # Deterministic generation with seed
```

**GET Parameters (API Key):**

- `length` (44-64, default: 44) - Length of key part (excluding ak\_ prefix)
- `alphabet` (string, default: "full") - Character set
- `raw` (boolean, default: true) - Output formatting

**POST Body (API Key - JSON):**

- `seed` (required) - 44-character base58 string for deterministic generation
- `length` (44-64) - Length of key part (excluding ak\_ prefix)
- `alphabet` (string) - Character set

**API Key Examples:**

```bash
# Random generation
curl "http://localhost:3000/api/api-key?length=50"
# Response: {"hash":"ak_A1b2C3d4E5f6G7h8I9j0K1l2M3n4O5p6Q7r8S9t0U1v2W3x4Y5z6","seed":"c2ae94ad78525..."}

# Deterministic generation with seed
curl -X POST "http://localhost:3000/api/api-key" \
  -H "Content-Type: application/json" \
  -d '{"seed":"2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR","length":50,"alphabet":"full"}'
# Response: {"hash":"ak_T4sHeyqXb1on6mAHwhLo9Nl0HZFc0dDR91qitMPziLJwQghFqq","seed":"2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR"}
```

## Generate BIP39 Mnemonic Phrases

```
GET /api/mnemonic        # Random generation
POST /api/mnemonic       # Deterministic generation with seed
```

**GET Parameters:**

- `language` (string, default: "english") - Language for mnemonic words
- `words` (12 or 24, default: 12) - Number of words to generate

**POST Body (JSON):**

- `seed` (required) - 44-character base58 string for deterministic generation
- `language` (string) - Language for mnemonic words
- `words` (12 or 24) - Number of words to generate

**Supported Languages (10 total):**

- **English** (english, en) - Default language
- **Spanish** (spanish, es) - Español
- **French** (french, fr) - Français
- **Portuguese** (portuguese, pt) - Português
- **Japanese** (japanese, ja) - 日本語
- **Chinese Simplified** (chinese, zh) - 中文简体
- **Chinese Traditional** (chinese-traditional, zh-tw) - 中文繁體
- **Italian** (italian, it) - Italiano
- **Korean** (korean, ko) - 한국어
- **Czech** (czech, cs) - Čeština

**Examples:**

```bash
# Random 12-word English mnemonic
curl "http://localhost:3000/api/mnemonic"
# Response: {"hash":"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about","seed":"2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR","otp":"123456789","timestamp":1692812400}

# Random 24-word Spanish mnemonic
curl "http://localhost:3000/api/mnemonic?language=spanish&words=24"
# Response: {"hash":"ábaco ábaco ábaco...","seed":"...","otp":"...","timestamp":...}

# Deterministic generation with seed
curl -X POST "http://localhost:3000/api/mnemonic" \
  -H "Content-Type: application/json" \
  -d '{"seed":"2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR","language":"japanese","words":24}'
# Response: {"hash":"あいこくしん あいこくしん...","seed":"2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR","otp":"...","timestamp":...}
```

## User Management System ⚠️ **PREPARATORY/FUTURE FUNCTIONALITY**

> **🚨 IMPORTANT**: This is **PREPARATORY CODE** for future use.
>
> **CURRENT STATUS:**
>
> - ❌ **NOT USED** by frontend (no UI implementation)
> - ❌ **NOT TESTED** (excluded from test suite)
> - ❌ **NO ACTUAL FUNCTIONALITY** in production app
> - ✅ **BACKEND READY** for future development
>
> **PURPOSE**: Reserved for potential admin panel or user management features.

```
GET /api/users            # List all users
GET /api/users/:id        # Get specific user
POST /api/users           # Create new user
DELETE /api/users/:id     # Delete user
```

**GET /api/users Parameters:**

- `limit` (optional) - Maximum number of users to return

**POST /api/users Body (JSON):**

```json
{
  "username": "user123",
  "email": "user@example.com"
}
```

**User Response Format:**

```json
{
  "id": 1,
  "username": "user123",
  "email": "user@example.com",
  "created_at": "2025-08-27 01:18:42",
  "updated_at": "2025-08-27 01:18:42"
}
```

**Examples:**

```bash
# List all users
curl "http://localhost:3000/api/users"
# Response: {"count":2,"users":[{"id":1,"username":"admin","email":"admin@example.com",...}]}

# Get specific user
curl "http://localhost:3000/api/users/1"
# Response: {"id":1,"username":"admin","email":"admin@example.com",...}

# Create new user
curl -X POST "http://localhost:3000/api/users" \
  -H "Content-Type: application/json" \
  -d '{"username":"newuser","email":"newuser@example.com"}'
# Response: {"id":3,"username":"newuser","email":"newuser@example.com",...}

# Delete user
curl -X DELETE "http://localhost:3000/api/users/3"
# Response: {"message": "User deleted successfully"}
```

## Shared Secret System (v1.8.0+)

Secure text sharing with encryption, dual-URL system, OTP protection, and multi-language email support (13 languages).

### Create Shared Secret

```
POST /api/shared-secret/create
```

**POST Body (JSON with SignedRequest):**

```json
{
  "payload": {
    "content": "Secret text to share (max 10,000 characters)",
    "max_reads": 1,
    "expiration_hours": 24,
    "receiver_email": "receiver@example.com",
    "sender_name": "John Doe",
    "email_lang": "en"
  },
  "signature": "ed25519_signature_hex_128_chars"
}
```

**Payload Fields:**

- `content` (required, max 10,000 chars) - Secret text to share
- `max_reads` (1-100, default: 1) - Maximum number of reads allowed
- `expiration_hours` (1-720, default: 24) - Hours until expiration
- `receiver_email` (required) - Receiver's email for notifications
- `sender_name` (optional) - Sender's display name
- `email_lang` (required) - Email language (en, es, fr, de, pt, ru, zh, ja, ar, hi, ca, gl, eu)

**Response (SignedResponse):**

```json
{
  "payload": {
    "sender_url": "http://localhost:5173/shared-secret/sender?h=abc123...",
    "receiver_url": "http://localhost:5173/shared-secret/receiver?h=def456...",
    "expires_at": 1234567890
  },
  "signature": "ed25519_signature_hex_128_chars"
}
```

### Retrieve Shared Secret

```
GET /api/shared-secret/{hash}      # Initial check (may require OTP)
POST /api/shared-secret/{hash}     # With OTP validation
```

**GET Response (if OTP required):**

```json
{
  "status": "OTP_REQUIRED",
  "message": "OTP sent to receiver's email"
}
```

**POST Body (with OTP):**

```json
{
  "payload": {
    "otp": "123456"
  },
  "signature": "ed25519_signature_hex_128_chars"
}
```

**Success Response:**

```json
{
  "payload": {
    "content": "Secret text content",
    "reads_remaining": 0,
    "expires_at": 1234567890
  },
  "signature": "ed25519_signature_hex_128_chars"
}
```

### Delete Shared Secret

```
DELETE /api/shared-secret/{hash}
```

**Response:**

```json
{
  "message": "Shared secret deleted successfully"
}
```

### Confirm Read Tracking

```
GET /api/shared-secret/confirm-read?hash={hash}&signature={sig}
```

**Response:**

```json
{
  "message": "Read confirmed successfully"
}
```

## Get Version Information

```
GET /api/version
```

**Response:**

```json
{
  "api_version": "1.8.4",
  "ui_version": "0.27.1"
}
```

## Alphabet Types

| Type                | Characters                                                                  | Count | Description                                           |
| ------------------- | --------------------------------------------------------------------------- | ----- | ----------------------------------------------------- |
| `base58`            | `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`                | 58    | Bitcoin alphabet (excludes 0, O, I, l)                |
| `no-look-alike`     | `346789ABCDEFGHJKLMNPQRTUVWXYabcdefghijkmnpqrtwxyz`                         | 49    | Maximum readability (excludes confusing chars)        |
| `full`              | `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`            | 62    | Complete alphanumeric                                 |
| `full-with-symbols` | `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_*^@#+!?$%` | 73    | Maximum entropy with symbols                          |
| `numeric`           | `0123456789`                                                                | 10    | Only digits 0-9 (requires longer length for security) |

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK` - Successful generation
- `400 Bad Request` - Invalid parameters (with descriptive error message)
- `404 Not Found` - Invalid endpoint (with available endpoints list)

**Example error response:**

```
HTTP/1.1 400 Bad Request
Content-Type: text/plain

Length must be between 2 and 128
```

---

_For authentication details, see [Authentication Documentation](./authentication.md)_  
_For cryptographic architecture, see [Cryptography Documentation](./cryptography.md)_
