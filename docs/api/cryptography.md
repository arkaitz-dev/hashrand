# Cryptographic Architecture

HashRand uses a **hybrid Blake2b + Blake3 cryptographic foundation**, combining Blake2b's optimized fixed-length operations with Blake3's unlimited variable-length outputs for maximum efficiency and flexibility.

## Core Cryptographic Stack

### Blake2b Unified Architecture

| **Function** | **Algorithm** | **Usage** | **Output** |
|--------------|---------------|-----------|------------|
| **Standard Hashing** | Blake2b512 | Email hashing, seed generation | 64 bytes |
| **Keyed Authentication** | Blake2b-keyed | HMAC replacement, integrity verification | 32-64 bytes |
| **Variable Output** | Blake2b-variable | User ID compression, database indexing | 8-64 bytes |

### Blake3 Universal Pipeline (v1.6.12+)

**🔐 Enterprise-Grade Variable-Length Cryptography**

| **Function** | **Algorithm** | **Usage** | **Output** |
|--------------|---------------|-----------|------------|
| **Universal Pseudonimizer** | Blake3 KDF + XOF | Deterministic variable-length derivation | 1 to 2^64 bytes |
| **Domain Separation** | Blake3 KDF (Base58 context) | Cryptographic namespace isolation | 32 bytes key |
| **Extended Output** | Blake3 XOF | Unlimited deterministic expansion | Arbitrary length |

#### Blake3 Pseudonimizer Pipeline

```rust
// utils/pseudonimizer.rs - Universal cryptographic pipeline
pub fn blake3_keyed_variable(
    hmac_env_key: &[u8; 64],  // Domain separation key (one per use case)
    data: &[u8],               // Input data (any length)
    output_length: usize       // Desired output (1 to 2^64 bytes)
) -> Vec<u8> {
    // STEP 1: hmac_env_key → Base58 → context (domain separation)
    let context = bs58::encode(hmac_env_key).into_string();

    // STEP 2: data → Blake3 standard hash → key_material[32 bytes]
    let key_material = blake3::hash(data);

    // STEP 3: (context, key_material) → Blake3 KDF → deterministic_key[32 bytes]
    let deterministic_key = blake3::derive_key(&context, key_material.as_bytes());

    // STEP 4: (data, deterministic_key, length) → Blake3 keyed+XOF → output
    let mut hasher = blake3::Hasher::new_keyed(&deterministic_key);
    hasher.update(data);
    let mut output_reader = hasher.finalize_xof();

    let mut output = vec![0u8; output_length];
    output_reader.fill(&mut output);

    output
}
```

**Security Properties:**
- **🔒 Domain Separation**: Different `hmac_env_key` → cryptographically independent outputs
- **🎲 Deterministic**: Same inputs always produce identical output
- **⚡ Variable Output**: Single function handles all length requirements (1 to 2^64 bytes)
- **🛡️ Key Derivation**: Unique 32-byte key derived per data input via Blake3 KDF
- **📊 XOF Properties**: Extended outputs maintain cryptographic relationship (first N bytes consistent)

#### Usage Example: SignedResponse Ed25519 Key Derivation

```rust
// Before (v1.6.11): Complex Blake2b expansion logic
// After (v1.6.12): Direct Blake3 pseudonimizer call
let private_key_vec = blake3_keyed_variable(
    &ed25519_derivation_key,  // 64-byte HMAC environment key
    &combined_input,           // user_id + server_pub_key
    32                         // Ed25519 private key length
);
```

### Migration History

**Previous (SHA3/HMAC/SHAKE - v0.x):**
```
SHA3-256 → HMAC-SHA3-256 → SHAKE256
Multiple algorithms, complex interactions
```

**Current (Hybrid Blake2b + Blake3 - v1.6.12+):**
```
Blake2b512 → Blake2b-keyed → Blake2b-variable (fixed-length operations)
Blake3 KDF → Blake3 XOF (variable-length operations, unlimited output)
Optimal algorithm selection per use case
```

## User ID Derivation System (v1.6.13+)

### Complete Cryptographic Flow - Blake3 Pipeline

```
Email → Blake3 XOF(64) → blake3_keyed_variable(hmac_key[64], 32)
                    ↓
              blake3_keyed_variable(argon2_salt[64], 32) → dynamic_salt
                    ↓
              Argon2id(paso2, dynamic_salt, mem=19456, time=2) → 32 bytes
                    ↓
              blake3_keyed_variable(compression_key[64], 16) → user_id
```

### Enhanced Implementation (v1.6.13)

```rust
// Zero Knowledge user identification (utils/jwt/crypto.rs)
// Enterprise-grade Blake3 + Argon2id pipeline - v1.6.13+
pub fn derive_user_id(email: &str) -> Result<[u8; 16], String> {
    // Step 1: Blake3 XOF (64 bytes, no key)
    let mut blake3_hasher = blake3::Hasher::new();
    blake3_hasher.update(email.to_lowercase().trim().as_bytes());
    let mut xof_reader = blake3_hasher.finalize_xof();
    let mut paso1_output = [0u8; 64];
    xof_reader.fill(&mut paso1_output);

    // Step 2: blake3_keyed_variable (hmac_key[64] → 32 bytes)
    let hmac_key = get_user_id_hmac_key()?;  // 64 bytes
    let paso2_output = blake3_keyed_variable(&hmac_key, &paso1_output, 32);
    let mut hmac_result = [0u8; 32];
    hmac_result.copy_from_slice(&paso2_output);

    // Step 3: blake3_keyed_variable (argon2_salt[64] → 32 bytes dynamic_salt)
    let dynamic_salt = generate_dynamic_salt(&paso1_output)?;

    // Step 4: Argon2id (unchanged)
    let argon2_output = derive_with_argon2id(&hmac_result[..], &dynamic_salt)?;

    // Step 5: blake3_keyed_variable (compression_key[64] → 16 bytes user_id)
    let compression_key = get_user_id_argon2_compression()?;  // 64 bytes
    let user_id_output = blake3_keyed_variable(&compression_key, &argon2_output, 16);
    let mut user_id = [0u8; 16];
    user_id.copy_from_slice(&user_id_output);

    Ok(user_id)  // Never stored with email - cryptographically derived
}

pub fn generate_dynamic_salt(data: &[u8]) -> Result<[u8; 32], String> {
    let argon2_salt = get_argon2_salt()?;  // 64 bytes
    let salt_output = blake3_keyed_variable(&argon2_salt, data, 32);
    let mut dynamic_salt = [0u8; 32];
    dynamic_salt.copy_from_slice(&salt_output);
    Ok(dynamic_salt)
}

pub fn user_id_to_username(user_id: &[u8; 16]) -> String {
    bs58::encode(user_id).into_string()  // Human-readable, no PII
}
```

### Security Properties

#### ✅ Triple-Key Cryptographic Security (v1.6.13+)
- **Modern Cryptography**: Blake3 + Argon2id hybrid achieving maximum security
- **Three Independent 64-byte Keys**: Multi-layer protection with domain separation
  - `USER_ID_HMAC_KEY` (64 bytes) - Keyed hashing in Step 2
  - `ARGON2_SALT` (64 bytes) - Dynamic salt derivation in Step 3
  - `USER_ID_ARGON2_COMPRESSION` (64 bytes) - Final keyed compression in Step 5
- **Rainbow Table Resistance**: Keyed final compression makes precomputation impossible
- **Per-User Salt**: Dynamic Argon2id salt preventing parallel dictionary attacks
- **High Security Parameters**: Argon2id with mem_cost=19456KB, time_cost=2 exceeds recommendations
- **Universal Pseudonimizer**: Consistent Blake3 pipeline used in Steps 2, 3, and 5
- **Key Compromise Mitigation**: Three independent keys required for full system break
- **Forward Secrecy**: User identity derives from email but email is never stored
- **Insider Threat Protection**: Even with database access, cannot derive emails without all three keys

## SignedRequest Universal Cryptographic Architecture (v1.6.10+)

### Universal SignedRequest Structure

**ENTERPRISE CRYPTOGRAPHIC SECURITY**: All API endpoints implement the **universal SignedRequest structure** with **strict authentication method separation** for maximum security and consistency.

```json
{
  "payload": {
    // Endpoint-specific data (deterministically serialized)
  },
  "signature": "ed25519_signature_128_hex_chars"
}
```

#### Deterministic JSON Serialization

Critical for signature consistency across frontend and backend:

```rust
// Backend: Recursive key sorting for deterministic serialization
fn sort_json_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted_map = serde_json::Map::new();
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();  // Alphabetical key ordering
            for key in keys {
                sorted_map.insert(key.clone(), Self::sort_json_keys(val.clone()));
            }
            Value::Object(sorted_map)
        }
        Value::Array(array) => Value::Array(array.into_iter().map(Self::sort_json_keys).collect()),
        other => other
    }
}
```

```javascript
// Frontend: Matching deterministic serialization
function sortObjectKeys(obj) {
    if (obj === null || typeof obj !== 'object') return obj;
    if (Array.isArray(obj)) return obj.map(sortObjectKeys);

    const sorted = {};
    const keys = Object.keys(obj).sort();  // Same alphabetical ordering
    for (const key of keys) {
        sorted[key] = sortObjectKeys(obj[key]);
    }
    return sorted;
}
```

#### Base64 URL-Safe Signature Verification (v1.6.20+)

**CRITICAL ENHANCEMENT**: Implemented universal Base64 URL-safe signature verification system ensuring perfect consistency between frontend and backend Ed25519 operations.

**Verification Flow:**
1. **JSON → Base64**: Deterministic JSON serialized and encoded as Base64 URL-safe
2. **Sign Base64**: Both frontend and backend sign the Base64 string directly
3. **Verify Base64**: Signature verification performed against Base64 payload
4. **Extract Data**: After verification, Base64 decoded to JSON for data extraction

```typescript
// Frontend: Signs Base64 payload directly
export async function createSignedRequest<T>(payload: T): Promise<SignedRequest> {
    const jsonPayload = serializePayload(payload);         // Step 1: JSON deterministic
    const base64Payload = encodePayloadBase64(jsonPayload); // Step 2: JSON → Base64 URL-safe
    const signature = await signMessage(base64Payload, keyPair); // Step 3: Sign Base64!

    return {
        payload: base64Payload,  // Send Base64 (what was signed)
        signature
    };
}
```

```rust
// Backend: Verifies Base64 payload directly
pub fn validate_base64_payload(
    base64_payload: &str,     // The Base64 string that was signed
    signature: &str,
    public_key_hex: &str,
) -> Result<(), SignedRequestError> {
    // Verify signature directly against Base64 payload (maximum determinism!)
    Self::validate_signature_string(base64_payload, signature, public_key_hex)
}
```

**Security Benefits:**
- **🎯 Maximum Determinism**: Base64 strings provide highest consistency for cryptographic operations
- **🔒 Perfect Consistency**: Frontend and backend sign/verify identical content
- **⚡ Performance**: No re-serialization needed during verification
- **🛡️ Error Elimination**: Eliminates serialization format mismatches

**Migration Note (v1.6.20)**: Previous versions attempted msgpack serialization which caused verification failures due to binary data in JSON strings. The Base64 approach resolves all consistency issues.

### Strict Authentication Method Validation (v1.6.10+)

#### Security Validation Matrix

```rust
// api/src/utils/signed_request.rs - Enterprise security implementation
pub fn validate_universal<T>(signed_request: &SignedRequest<T>, request: &Request) -> Result<String, SignedRequestError> {
    // Parse and detect authentication methods
    let has_bearer = Self::extract_pub_key_from_bearer(request).is_ok();
    let has_pub_key = payload_value.get("pub_key").is_some();
    let has_magiclink = payload_value.get("magiclink").is_some();

    // Enforce strict authentication separation
    match (has_bearer, has_pub_key, has_magiclink) {
        (true, false, false) => {
            // ✅ Bearer-only authentication
            let pub_key_hex = Self::extract_pub_key_from_bearer(request)?;
            Self::validate(signed_request, &pub_key_hex)?;
            Ok(pub_key_hex)
        }
        (false, true, false) => {
            // ✅ Payload pub_key authentication
            let pub_key_hex = Self::extract_pub_key_from_payload(&payload_value)?;
            Self::validate(signed_request, &pub_key_hex)?;
            Ok(pub_key_hex)
        }
        (false, false, true) => {
            // ✅ Payload magiclink authentication
            let pub_key_hex = Self::extract_pub_key_from_magiclink(&payload_value)?;
            Self::validate(signed_request, &pub_key_hex)?;
            Ok(pub_key_hex)
        }
        (true, _, _) => {
            // ❌ Bearer + payload conflict
            Err(SignedRequestError::ConflictingAuthMethods(
                "Bearer token present but payload contains pub_key/magiclink - only Bearer allowed"
            ))
        }
        (false, true, true) => {
            // ❌ Ambiguous payload authentication
            Err(SignedRequestError::AmbiguousPayloadAuth(
                "Both pub_key and magiclink found in payload - only one allowed"
            ))
        }
        (false, false, false) => {
            // ❌ No authentication method
            Err(SignedRequestError::MissingPublicKey(
                "No Bearer token and no pub_key/magiclink in payload - exactly one auth method required"
            ))
        }
    }
}
```

#### Public Key Extraction Methods

1. **Bearer Token Extraction** (JWT-protected endpoints):
```rust
fn extract_pub_key_from_bearer(request: &Request) -> Result<String, SignedRequestError> {
    let auth_header = request.header("authorization")?;
    let token = auth_header.strip_prefix("Bearer ")?;
    let claims = JwtUtils::validate_access_token(token)?;
    Ok(hex::encode(claims.pub_key))  // Extract Ed25519 pub_key from JWT
}
```

2. **Payload pub_key Extraction** (Magic link generation):
```rust
fn extract_pub_key_from_payload(payload: &Value) -> Result<String, SignedRequestError> {
    payload.get("pub_key")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| SignedRequestError::MissingPublicKey("pub_key not found in payload"))
}
```

3. **Magiclink Database Lookup** (Magic link validation):
```rust
fn extract_pub_key_from_magiclink(payload: &Value) -> Result<String, SignedRequestError> {
    let magiclink = payload.get("magiclink")?.as_str()?;
    let (_is_valid, _next_param, _user_id, pub_key_bytes) =
        MagicLinkOperations::validate_and_consume_magic_link_encrypted(magiclink)?;
    let pub_key_array = pub_key_bytes.ok_or_else(||
        SignedRequestError::MissingPublicKey("No pub_key found in magiclink data"))?;
    Ok(hex::encode(pub_key_array))
}
```

## Magic Link Cryptography with Ed25519 Authentication

### Ed25519 Digital Signature Layer (Enhanced v1.6.10+)

```
SignedRequest{payload + signature} → validate_universal() → Authentication Method Detection
                                              ↓
                     Bearer Token ←→ pub_key ←→ magiclink (mutually exclusive)
                                              ↓
Email + Pub_Key + Next → Ed25519_Sign(private_key) → Signature[64_bytes] → Backend_Verification
                    ↓                                        ↓
            Pub_Key[32_bytes] → Database_Storage → JWT_Claims[pub_key]
```

#### Ed25519 Signature Security
- **Elliptic Curve**: Curve25519 providing 128-bit security level
- **Signature Size**: 64 bytes (128 hex characters) for compact transmission
- **Public Key Size**: 32 bytes (64 hex characters) for optimal storage
- **Verification Speed**: Microsecond-level verification performance
- **Cryptographic Strength**: Equivalent to 3072-bit RSA

#### Signature Verification Process
```rust
// Ed25519 signature verification (api/src/utils/ed25519.rs)
pub fn verify_magic_link_request(
    email: &str,
    public_key_hex: &str,
    next: Option<&str>,
    signature_hex: &str,
) -> SignatureVerificationResult {
    // 1. Reconstruct signed message
    let message = match next {
        Some(next_param) => format!("{}{}{}", email, public_key_hex, next_param),
        None => format!("{}{}", email, public_key_hex),
    };

    // 2. Parse Ed25519 components
    let public_key = PublicKey::from_bytes(&hex::decode(public_key_hex)?)?;
    let signature = Signature::from_bytes(&hex::decode(signature_hex)?)?;

    // 3. Verify signature
    public_key.verify_strict(message.as_bytes(), &signature)
}
```

### Frontend Ed25519 Implementation (v0.19.13+)

**COMPLETE CRYPTOGRAPHIC INTEGRATION**: The frontend now handles all Ed25519 operations automatically, providing seamless cryptographic security without exposing complexity to developers.

#### Frontend Cryptographic Stack

```typescript
// Frontend Ed25519 module (web/src/lib/ed25519.ts)
import { ed25519 } from '@noble/curves/ed25519';
import { bytesToHex, hexToBytes } from '@noble/hashes/utils';

// Hybrid architecture with Web Crypto API primary + Noble fallback
const keyPair = await generateEd25519KeyPair(); // WebCrypto or Noble
const signature = await signMessage(message, privateKey); // Auto-signs email + pub_key
```

#### Automatic Cryptographic Workflow

1. **Keypair Generation**: `getOrCreateKeyPair()` creates Ed25519 keypair with Web Crypto API
2. **Secure Storage**: Non-extractable private keys stored in IndexedDB
3. **Message Signing**: Automatic signing of `email + pub_key` combination
4. **Backend Integration**: Seamless transmission to backend for verification
5. **Security Cleanup**: Automatic keypair cleanup on logout

#### Security Architecture Features

- **🔐 Non-extractable Keys**: Private keys stored as CryptoKey objects, cannot be extracted
- **💾 IndexedDB Storage**: Browser-native secure database for keypair persistence
- **🔄 Hybrid Cryptography**: Web Crypto API primary with @noble/curves fallback
- **🧹 Automatic Cleanup**: Ed25519 keypairs cleared on logout via `clearAllKeyPairs()`
- **⚡ Performance**: WebCrypto hardware acceleration when available
- **🛡️ Zero Knowledge**: No personal data in cryptographic storage

#### Implementation Functions

```typescript
// Core Ed25519 frontend functions
export async function getOrCreateKeyPair(): Promise<Ed25519KeyPair>
export async function signMessage(message: string, privateKey: CryptoKey): Promise<string>
export async function verifySignature(message: string, signature: string, publicKeyBytes: Uint8Array): Promise<boolean>
export async function clearAllKeyPairs(): Promise<void>
export function publicKeyToHex(publicKeyBytes: Uint8Array): string
```

### Encryption & Integrity Flow

```
User_ID + Pub_Key + Timestamp → ChaCha8RNG[44] → nonce[12] + secret_key[32] → ChaCha20 Encrypt → Base58 Token
                    ↓                                                                    ↓
            Blake2b-keyed(raw_magic_link, hmac_key) → Blake2b-variable[16] → Database Hash Index
```

### Enhanced Security Architecture

- **Ed25519 Authentication**: Cryptographic proof of identity before magic link creation
- **ChaCha20 Encryption**: 32-byte encrypted magic link data using ChaCha20 stream cipher
- **Blake2b-keyed Integrity**: Prevents modification and tampering of magic links (replaces HMAC)
- **Blake2b-variable Compression**: Optimal 16-byte hash indexing for database storage
- **Pub_Key Storage**: Ed25519 public keys stored encrypted in database payloads
- **Time-Limited**: 5-minute expiration prevents replay attacks (development: 15 minutes)
- **One-Time Use**: Magic links consumed immediately after validation
- **Optimized Length**: 44-character Base58 tokens (33% reduction from previous 66-character)

### Magic Link Payload Encryption (v1.6.14+)

**PERFORMANCE OPTIMIZED PIPELINE**: Single Blake3 pseudonimizer call replaces complex multi-layer encryption achieving 100x performance improvement.

#### Blake3 Encryption Architecture

```
encrypted_token[32] + MLINK_CONTENT[64] → blake3_keyed_variable() → 44 bytes
                                              ↓                         ↓
                                         nonce[12]               cipher_key[32]
                                              ↓                         ↓
                           ChaCha20-Poly1305.encrypt(payload, nonce, cipher_key)
                                                    ↓
                                          encrypted_payload (BLOB)
```

#### Encryption/Decryption Flow

**Encryption** (`api/src/database/operations/magic_link_crypto.rs`):
```rust
pub fn encrypt_payload_content(
    encrypted_data: &[u8; 32],  // Encrypted magic token bytes
    payload: &[u8],              // encryption_blob[44] + pub_key[32] + next_param
) -> Result<Vec<u8>, SqliteError> {
    let mlink_key = Self::get_mlink_content_key()?;  // 64-byte MLINK_CONTENT env key

    // STEP 1: Blake3 KDF → nonce[12] + cipher_key[32]
    let derived = blake3_keyed_variable(&mlink_key, encrypted_data, 44);
    let nonce_bytes: [u8; 12] = derived[0..12].try_into()?;
    let cipher_key: [u8; 32] = derived[12..44].try_into()?;

    // STEP 2: ChaCha20-Poly1305 AEAD encryption
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&cipher_key));
    cipher.encrypt(Nonce::from_slice(&nonce_bytes), payload)
}
```

**Decryption** (reverse process):
```rust
pub fn decrypt_payload_content(
    encrypted_data: &[u8; 32],  // Same encrypted magic token
    ciphertext: &[u8],           // Database encrypted_payload
) -> Result<Vec<u8>, SqliteError> {
    let mlink_key = Self::get_mlink_content_key()?;

    // STEP 1: Derive same nonce + cipher_key (deterministic)
    let derived = blake3_keyed_variable(&mlink_key, encrypted_data, 44);
    let nonce_bytes: [u8; 12] = derived[0..12].try_into()?;
    let cipher_key: [u8; 32] = derived[12..44].try_into()?;

    // STEP 2: ChaCha20-Poly1305 AEAD decryption
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&cipher_key));
    cipher.decrypt(Nonce::from_slice(&nonce_bytes), ciphertext)
}
```

#### Security Properties

- **🚀 Performance**: Single Blake3 KDF call (vs previous Argon2id + Blake2b + ChaCha8RNG)
- **🔒 Deterministic**: Same encrypted_token always produces same nonce/cipher_key
- **🎯 Domain Separation**: MLINK_CONTENT key ensures cryptographic independence
- **🛡️ AEAD Security**: ChaCha20-Poly1305 provides both encryption and authentication
- **📊 Zero Storage**: No need to store nonces or IVs - derived on-demand
- **⚡ ~100x Faster**: Eliminated memory-hard Argon2id from hot path

#### Environment Configuration (v1.6.14+)

```bash
# Single unified key (64 bytes = 128 hex chars)
MLINK_CONTENT=<128_hex_chars>  # Development/Production different values

# Previous (v1.6.13 - deprecated):
# MLINK_CONTENT_CIPHER=<64_hex_chars>
# MLINK_CONTENT_NONCE=<64_hex_chars>
# MLINK_CONTENT_SALT=<64_hex_chars>
```

**Migration**: Three 32-byte keys consolidated to single 64-byte key for simplified configuration and enhanced security.

## Seed-Based Generation Cryptography

### ChaCha8 Unified Generation

All pseudorandom generation uses **ChaCha8Rng** for cryptographic consistency:

- **Hash/Password/API Key Generation**: Uses `ChaCha8Rng::from_seed()` with 32-byte seeds
- **OTP Generation**: Uses ChaCha8 with domain separation (last byte XOR) for independent randomness
- **Industry Standard**: ChaCha8 is cryptographically robust and widely audited
- **Domain Separation**: Professional technique ensures hash and OTP are cryptographically independent

### Seed Security Architecture

- **Initial Generation**: Uses `nanoid` (128 characters) → Blake2b512 → 32-byte seed
- **Base58 Encoding**: Eliminates confusing characters (0, O, I, l) for better usability
- **Deterministic Reproducibility**: Same seed always produces same results for audit trails

## Frontend Cryptographic Architecture (v0.21.0)

### Enterprise-Grade Modular Cryptographic System

**Major Architecture Transformation**: Complete refactoring from monolithic cryptographic modules to SOLID-compliant specialized systems.

#### Crypto Module System (`web/src/lib/crypto/`)

**5 Specialized Modules** (94% code reduction from 471→30 lines):

- **`crypto-core.ts`** - Core cryptographic functions
  - `cryptoHashGen()` - Blake2b-keyed + ChaCha8RNG pipeline for unified hash generation
  - `generatePrehash()` - 32-byte prehash from prehash seed using session HMAC key
  - `generateCipherKey()` / `generateCipherNonce()` - ChaCha20-Poly1305 key/nonce derivation
  - `generateCryptoSalt()` / `generatePrehashSeed()` - Cryptographically secure random generation

- **`crypto-encoding.ts`** - Base64/Base64URL conversion utilities
  - `bytesToBase64()` / `base64ToBytes()` - Standard Base64 encoding/decoding
  - `bytesToBase64Url()` / `base64UrlToBytes()` - URL-safe Base64 with automatic padding

- **`crypto-storage.ts`** - Prehash seed IndexedDB management
  - `storePrehashSeed()` / `getPrehashSeed()` - FIFO rotation system (max 20 seeds)
  - 8-byte cryptographic keys for efficient KV storage management
  - Automatic cleanup preventing unlimited memory growth

- **`crypto-url-operations.ts`** - URL parameter encryption/decryption
  - `encryptUrlParams()` / `decryptUrlParams()` - ChaCha20-Poly1305 AEAD encryption
  - `serializeParams()` - Consistent JSON serialization with sorted keys
  - Ultra-compact URL format: single `?p=` parameter combining idx + encrypted data

- **`crypto-utils.ts`** - High-level cryptographic workflows
  - `prepareSecureUrlParams()` - Complete encryption workflow wrapper
  - `parseNextUrl()` / `encryptNextUrl()` - URL manipulation utilities
  - `decryptPageParams()` / `createEncryptedUrl()` - Page-level parameter handling

#### Ed25519 Digital Signature System (`web/src/lib/ed25519/`)

**6 Specialized Modules** (93% code reduction from 303→21 lines):

- **`ed25519-types.ts`** - Type definitions and interfaces
- **`ed25519-keygen.ts`** - Key generation (WebCrypto + Noble fallback)
- **`ed25519-database.ts`** - IndexedDB storage operations
- **`ed25519-signing.ts`** - Digital signature operations
- **`ed25519-utils.ts`** - Hex/bytes conversion utilities
- **`ed25519-api.ts`** - High-level API functions

#### Universal Composables System (`web/src/lib/composables/`)

**DRY Elimination**: 2 composables replacing 840+ lines of duplicate code:

- **`useGenerationWorkflow.ts`** - Unified generation logic across all endpoints
- **`useFormParams.ts`** - Centralized form parameter management

#### Technical Benefits Achieved

- **Zero Breaking Changes**: 100% API compatibility preserved during massive refactoring
- **Enterprise Standards**: All modules under 225 lines following SOLID principles
- **Performance**: Faster compilation with granular imports and smaller modules
- **Maintainability**: Each module easily testable and modifiable in isolation
- **Type Safety**: Complete TypeScript coverage across all new modules

## Database Cryptography

### Zero Knowledge Schema

```sql
-- Zero Knowledge Users Table
CREATE TABLE users (
    user_id BLOB PRIMARY KEY,           -- 16-byte cryptographic hash (no PII)
    created_at INTEGER DEFAULT (unixepoch())  -- Unix timestamp (timezone-agnostic)
);

-- Zero Knowledge Magic Links Table
CREATE TABLE magiclinks (
    token_hash BLOB PRIMARY KEY,        -- 16-byte Blake2b-variable hash of encrypted token
    timestamp INTEGER NOT NULL,         -- Original timestamp used in magic link creation
    encryption_blob BLOB NOT NULL,      -- 44 bytes: nonce[12] + secret_key[32] from ChaCha8RNG
    next_param TEXT,                     -- Optional next destination parameter
    expires_at INTEGER NOT NULL         -- Unix timestamp expiration
    -- No user data, emails, or PII - only cryptographic hashes and encryption metadata
);
```

### Cryptographic Database Features

- **Blake2b-variable Indexing**: All primary keys use Blake2b-variable hashes for optimal distribution
- **No PII Storage**: Database contains zero personal information
- **Cryptographic References Only**: All foreign keys and indexes use hash-based identifiers
- **Time-based Expiration**: Unix timestamps for precise expiration handling

## Performance Benefits

### Blake2b vs SHA3 Performance

| **Metric** | **Blake2b** | **SHA3** | **Improvement** |
|------------|-------------|----------|-----------------|
| **Speed** | ~3.2 GB/s | ~1.6 GB/s | **2x faster** |
| **CPU Cycles** | 2.9 cpb | 5.4 cpb | **46% fewer** |
| **Memory** | Lower | Higher | **Reduced footprint** |
| **Dependencies** | 1 crate | 3 crates | **Simplified** |

### Unified Architecture Benefits

- **⚡ Performance**: Faster cryptographic operations across entire application
- **🏗️ Simplification**: Unified Blake2b family reduces architectural complexity  
- **🔧 Maintainability**: Single cryptographic family easier to audit and maintain
- **📈 Future-Proofing**: Blake2b designed for modern computing environments
- **🛡️ Security**: Maintained or improved cryptographic security properties
- **🎯 Standards Compliance**: RFC 7693 standardized cryptographic implementation

## URL Parameter Encryption System

### Advanced Privacy Protection Architecture

HashRand implements a revolutionary **URL Parameter Encryption System** that protects user privacy by encrypting all URL parameters using ChaCha20-Poly1305 AEAD encryption. This ensures that sensitive information never appears in browser history, server logs, or network monitoring tools.

### Cryptographic Pipeline

```
URL Parameters → Crypto Salt → Prehash Seed → Blake2b-keyed → ChaCha20-Poly1305 → Base64URL
     ↓                ↓              ↓              ↓                ↓              ↓
Plain JSON → 32-byte salt → 32-byte seed → Cipher/Nonce Keys → AEAD Encryption → Encrypted URL
```

### Implementation Details

```typescript
// Complete encryption workflow (web/src/lib/crypto.ts)
export function encryptUrlParams(
    params: Record<string, any>,
    cipherToken: string,
    nonceToken: string, 
    hmacKey: string
): { p: string } {
    // 1. Add 32-byte cryptographic salt for noise generation
    const salt = generateCryptoSalt();
    const paramsWithSalt = { ...params, _salt: bytesToBase64(salt) };
    
    // 2. Generate random 32-byte prehash seed (content-independent)
    const prehashSeed = generatePrehashSeed();
    
    // 3. Store seed with 8-byte cryptographic key (FIFO rotation)
    const idx = storePrehashSeed(prehashSeed, hmacKey);
    
    // 4. Generate encryption keys from prehash
    const prehash = generatePrehash(prehashSeed, hmacKey);
    const cipherKey = generateCipherKey(cipherToken, prehash);
    const cipherNonce = generateCipherNonce(nonceToken, prehash);
    
    // 5. Encrypt with ChaCha20-Poly1305 AEAD
    const cipher = chacha20poly1305(cipherKey, cipherNonce);
    const ciphertext = cipher.encrypt(new TextEncoder().encode(JSON.stringify(paramsWithSalt)));
    
    return {
        encrypted: bytesToBase64Url(ciphertext),
        idx: idx  // 8-byte key for sessionStorage retrieval
    };
}
```

### Security Architecture Features

#### ✅ Triple Token Cryptographic System
- **Cipher Token**: 32-byte session key for ChaCha20-Poly1305 encryption
- **Nonce Token**: 32-byte session key for unique nonce generation  
- **HMAC Key**: 32-byte session key for prehash seed derivation and integrity

#### ✅ Advanced Key Derivation
- **Blake2b-keyed Prehashing**: Content-independent key generation using Blake2b with HMAC key
- **ChaCha8RNG Pipeline**: Cryptographically secure key derivation for cipher and nonce generation
- **Domain Separation**: Cipher and nonce keys derived independently to prevent key reuse

#### ✅ FIFO Storage Management
- **IndexedDB Enterprise Storage**: High-performance browser database with cross-tab synchronization
- **20-Seed Rotation**: FIFO (First In, First Out) automatic cleanup prevents memory bloat
- **8-Byte Cryptographic Keys**: Blake2b-derived identifiers for optimal IndexedDB indexing
- **Cross-Tab Consistency**: Shared encryption keys and prehashseeds across all browser tabs

### Bidirectional Navigation Flow

#### Backend → Frontend (Next Parameter)
```typescript
// Layout interceptor (web/src/routes/+layout.svelte)
if (loginResponse.next) {
    const encryptedNextUrl = encryptNextUrl(loginResponse.next, {
        cipherToken, nonceToken, hmacKey
    });
    await goto(encryptedNextUrl);  // /custom?p=...
}
```

#### Configuration → Result (Generate Buttons)
```typescript
// All config routes (custom/, password/, api-key/, mnemonic/)
const encryptedUrl = createEncryptedUrl('/result', resultParams, {
    cipherToken, nonceToken, hmacKey
});
goto(encryptedUrl);  // /result?p=...
```

#### Universal Route Decryption
```typescript
// All target routes automatically decrypt parameters
const decryptedParams = decryptPageParams(searchParams, {
    cipherToken, nonceToken, hmacKey
});
// Fallback to direct URL parameters if decryption fails
```

### Privacy Protection Benefits

#### 🛡️ Complete Browser History Privacy
- **Zero Plaintext Exposure**: Sensitive parameters never visible in browser history
- **Physical Device Security**: URLs remain private even with device access
- **Web Analytics Protection**: User data hidden from monitoring and analytics tools

#### 🔒 Advanced Security Properties  
- **AEAD Security**: ChaCha20-Poly1305 provides both confidentiality and integrity
- **Content Independence**: Encryption keys completely independent of parameter content
- **Forward Secrecy**: Each parameter set uses unique cryptographic keys
- **Replay Protection**: Time-bounded IndexedDB storage prevents replay attacks

#### ⚡ Performance Optimization
- **Efficient Storage**: 8-byte keys minimize IndexedDB overhead with optimal indexing
- **Automatic Cleanup**: FIFO rotation prevents storage bloat across sessions
- **Fast Crypto**: ChaCha20-Poly1305 optimized for modern web browsers
- **Cross-Tab Performance**: Shared IndexedDB eliminates duplicate storage overhead
- **Minimal Overhead**: URL encryption adds negligible performance impact

### Backward Compatibility

The system maintains **100% backward compatibility**:
- Legacy unencrypted URLs continue to work as fallback
- No breaking changes to existing API or user experience
- Automatic detection of encrypted vs plain parameters
- Seamless migration for existing users

## Implementation Architecture

### Rust Dependencies

#### Backend (Rust)
```toml
[dependencies]
blake2 = "0.10"              # Blake2b hashing for unified cryptographic operations
argon2 = "0.5.3"            # Argon2id for secure user ID derivation
chacha20 = "0.9.1"          # ChaCha20 stream cipher for magic link encryption
chacha20poly1305 = "0.10.1" # ChaCha20-Poly1305 AEAD for secure magic link encryption
ed25519-dalek = "2.2.0"     # Ed25519 digital signatures for magic link authentication
base64 = "0.22.1"           # Base64 encoding for JWT tokens
```

#### Frontend (TypeScript)
```json
"dependencies": {
    "@noble/hashes": "^2.0.0",  // Blake2b and cryptographic hashing
    "@noble/ciphers": "^2.0.0", // ChaCha20-Poly1305 AEAD encryption
    "@scure/base": "^2.0.0"     // Base64URL and Base58 encoding utilities
}
```

### Key Implementation Files

#### Backend (Rust/Spin)
- **api/src/utils/ed25519.rs**: Ed25519 digital signature verification for magic links (NEW)
- **api/src/utils/jwt/crypto.rs**: User ID derivation and cryptographic operations
- **api/src/utils/jwt/magic_links.rs**: Magic link generation and processing
- **api/src/utils/auth/magic_link_gen.rs**: Magic link generation with Ed25519 verification
- **api/src/utils/auth/magic_link_val.rs**: Magic link validation with pub_key extraction
- **api/src/utils/jwt/custom_tokens.rs**: JWT token creation with pub_key claims
- **api/src/database/operations/magic_link_ops.rs**: Magic link encryption/decryption
- **api/src/utils/random_generator.rs**: Seed generation with Blake2b512

#### Frontend (TypeScript/SvelteKit)
- **web/src/lib/crypto.ts**: Complete URL parameter encryption system
  - `encryptUrlParams()` / `decryptUrlParams()`: Core AEAD encryption/decryption
  - `createEncryptedUrl()`: High-level URL generation for navigation
  - `parseNextUrl()` / `encryptNextUrl()`: Backend response processing
  - `storePrehashSeed()` / `getPrehashSeed()`: FIFO IndexedDB management
- **web/src/lib/session-manager.ts**: Unified IndexedDB session management (NEW)
  - `SessionManager`: Enterprise-grade session storage with cross-tab synchronization
  - `addPrehashSeed()` / `getPrehashSeed()`: FIFO prehashseed management
  - `storeAuthTokens()` / `getAuthTokens()`: Secure auth token persistence
  - `storeCryptoTokens()` / `getCryptoTokens()`: Crypto key management
- **web/src/routes/+layout.svelte**: Magic link next parameter encryption
- **web/src/routes/{custom,password,api-key,mnemonic}/+page.svelte**: Route-specific encryption/decryption
- **web/src/routes/result/+page.svelte**: Result page parameter processing and return URL generation

## SignedResponse Architecture (v1.6.11+)

### Universal Ed25519 Response Signing

**Complete Implementation**: All generation endpoints now return cryptographically signed responses with zero legacy code.

#### SignedResponse Structure
```rust
// Universal response format across all endpoints
{
    "payload": {
        // Endpoint-specific data (hash, seed, otp, timestamp, etc.)
    },
    "signature": "ed25519_signature_hex_128_chars"
}
```

#### Ed25519 Response Signing Process
```rust
// Deterministic response signing (api/src/utils/signed_response.rs)
pub fn create_signed_response<T: Serialize>(
    payload: &T,
    server_private_key: &[u8; 32]
) -> Result<SignedResponse, SignedResponseError> {
    // 1. Deterministic serialization
    let sorted_payload = sort_json_keys(payload);
    let serialized = serde_json::to_string(&sorted_payload)?;

    // 2. Ed25519 signature generation
    let signature = ed25519_sign(serialized.as_bytes(), server_private_key);

    // 3. Response assembly
    Ok(SignedResponse {
        payload: sorted_payload,
        signature: hex::encode(signature)
    })
}
```

#### Cryptographic Guarantees
- **Integrity**: Response payload cannot be modified without detection
- **Authenticity**: Responses verifiably originate from legitimate server
- **Non-repudiation**: Server cannot deny having generated specific responses
- **Deterministic**: Consistent signature generation for identical payloads

#### Legacy Elimination Benefits
- **🗑️ Zero Technical Debt**: Complete removal of all legacy response handlers
- **🔒 Universal Security**: Consistent Ed25519 protection across all endpoints
- **⚡ Performance**: Reduced code complexity and improved maintainability
- **🏛️ Clean Architecture**: Enterprise-grade SOLID principles implementation

#### Secure Cookie Integration (v1.6.11+)
- **🍪 HTTP Headers Delivery**: Authentication endpoints deliver refresh tokens via standard HTTP `Set-Cookie` headers
- **🛡️ Enterprise Security Attributes**: HttpOnly, Secure, SameSite=Strict, Max-Age, Path=/ for maximum protection
- **🔒 Dual Security Model**: SignedResponse integrity + secure cookie delivery combining cryptographic and transport security
- **🚫 XSS Protection**: HttpOnly attribute prevents JavaScript access to refresh tokens

## Security Considerations

### Cryptographic Strength

- **Blake2b**: RFC 7693 standardized, widely adopted, equivalent security to SHA3
- **ChaCha20**: Industry-standard stream cipher, resistance to timing attacks
- **Argon2id**: Winner of Password Hashing Competition, memory-hard function
- **Ed25519**: Elliptic curve digital signatures with 128-bit security level
- **Base58**: Bitcoin-standard encoding, eliminates character confusion

### Attack Resistance

- **Rainbow Tables**: Blake2b-keyed with unique keys prevents precomputation attacks
- **Timing Attacks**: Constant-time implementations in all cryptographic operations
- **Side-Channel**: ChaCha20, Blake2b, and Ed25519 designed for side-channel resistance
- **Response Tampering**: Ed25519 signatures prevent response modification attacks
- **Quantum Resistance**: While not post-quantum, provides maximal classical security

---

*For API usage, see [API Endpoints](./endpoints.md)*
*For authentication flow, see [Authentication Documentation](./authentication.md)*