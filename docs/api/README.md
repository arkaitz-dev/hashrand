# API Endpoints

HashRand's REST API provides secure random generation and encrypted secret sharing capabilities.

## Public Endpoints

### `/api/version`
- **Method**: GET
- **Auth**: None
- **Function**: Returns API version information

## Generation Endpoints

All generation endpoints require (custom) JWT authentication.

### `/api/custom`
- **Function**: Generate custom-length random hashes (16-512 bytes)
- **Options**: Hash type (SHA-256, SHA3-256, BLAKE3), output encoding (hex, base58, base64)

### `/api/password`
- **Function**: Generate secure passwords
- **Options**: Length (8-128 chars), character sets (uppercase, lowercase, numbers, symbols)

### `/api/api-key`
- **Function**: Generate API keys for service integrations
- **Options**: Length (16-64 bytes), encoding format

### `/api/mnemonic`
- **Function**: Generate BIP39 mnemonic phrases
- **Options**: Word count (12, 15, 18, 21, 24 words), language (Czech, Chinese Simplified, Chinese Traditional, English, French, Italian, Japanese, Korean, Portuguese, Spanish)

## Sharing Endpoints

All sharing endpoints require JWT authentication.

### `/api/shared-secret` (POST)
- **Function**: Create encrypted shared secret
- **Options**:
  - Read limits (1-10)
  - Expiration time (1 hour to 3 days)
  - Recipient email (required for magic link delivery)
  - One-time password (OTP) generation
- **Returns**: Two URLs (sender view with OTP, receiver access link)

### `/api/shared-secret` (GET with hash)
- **Function**: Access shared secret
- **Auth**: Requires OTP validation
- **Returns**: Decrypted secret content (client-side decryption)

### `/api/shared-secret/tracking` (GET)
- **Function**: Check read status of shared secret
- **Auth**: JWT required (sender only)
- **Returns**: Read count, reads remaining, first read timestamp

### `/api/shared-secret` (DELETE)
- **Function**: Delete shared secret before expiration
- **Auth**: JWT required (sender only)

## Authentication Endpoints

### `/api/login/request`
- **Function**: Request magic link for authentication
- **Input**: Email address, preferred language
- **Returns**: Confirmation (magic link sent to email)

### `/api/login/validate`
- **Function**: Validate magic link token
- **Input**: Magic link token
- **Returns**: JWT access token, encrypted session keys

## Cryptographic Key Management

### `/api/keys/rotate` (POST)
- **Function**: Publish permanent user public keys (Ed25519 + X25519)
- **Auth**: JWT required
- **Purpose**: Enable future user-to-user encryption features

### `/api/user/keys/` (GET)
- **Function**: Retrieve public keys for target user
- **Auth**: JWT required
- **Query**: `target_user=email@example.com`
- **Returns**: User's latest Ed25519 and X25519 public keys

---

**Note**: All authenticated endpoints use Ed25519 signed requests to prevent tampering and replay attacks. Sensitive data is encrypted client-side before transmission.
