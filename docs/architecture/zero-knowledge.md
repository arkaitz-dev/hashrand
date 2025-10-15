# Zero Knowledge Architecture

HashRand implements a **true Zero Knowledge architecture** where the server operates with complete user privacy, never storing or processing personal identifying information.

## Core Zero Knowledge Principles

### Privacy-Preserving Design

The HashRand system is built on the fundamental principle that **personal information should never reach the server**. This is achieved through a sophisticated cryptographic architecture that enables user identification without compromising privacy.

### Complete Data Privacy

- **No PII Storage**: Server databases contain zero personal information
- **Email Privacy**: Emails used only for magic link delivery, never stored
- **Audit Trail Privacy**: All logs use Base58 usernames, not personal data
- **Compliance Ready**: GDPR/CCPA compliant by design - no personal data to manage

## Cryptographic User Identity System

### User ID Derivation Flow

```
Email → Blake3 XOF(64) → blake3_keyed_variable(hmac_key[64], 32)
                    ↓
              blake3_keyed_variable(argon2_salt[64], 32) → dynamic_salt
                    ↓
              Argon2id(paso2, dynamic_salt, mem=19456, time=2) → 32 bytes
                    ↓
              blake3_keyed_variable(compression_key[64], 16) → user_id
                    ↓
              Base58 Username Display (~22 chars)
```

### Implementation Architecture

```rust
// Zero Knowledge user identification (utils/jwt/crypto/user_id.rs)
pub fn derive_user_id(email: &str) -> Result<[u8; 16], String> {
    // Step 1: Blake3 XOF (64 bytes, no key)
    let mut blake3_hasher = blake3::Hasher::new();
    blake3_hasher.update(email.to_lowercase().trim().as_bytes());
    let mut xof_reader = blake3_hasher.finalize_xof();
    let mut paso1_output = [0u8; 64];
    xof_reader.fill(&mut paso1_output);

    // Step 2: blake3_keyed_variable (hmac_key[64] → 32 bytes)
    let hmac_key = get_user_id_hmac_key()?;
    let paso2_output = blake3_keyed_variable(&hmac_key, &paso1_output, 32);

    // Step 3: blake3_keyed_variable (argon2_salt[64] → 32 bytes dynamic_salt)
    let dynamic_salt = generate_dynamic_salt(&paso1_output)?;

    // Step 4: Argon2id (unchanged)
    let argon2_output = derive_with_argon2id(&paso2_output[..], &dynamic_salt)?;

    // Step 5: blake3_keyed_variable (compression_key[64] → 16 bytes user_id)
    let compression_key = get_user_id_argon2_compression()?;
    let user_id_output = blake3_keyed_variable(&compression_key, &argon2_output, 16);
    let mut user_id = [0u8; 16];
    user_id.copy_from_slice(&user_id_output);

    Ok(user_id)  // Never stored with email - cryptographically derived
}

pub fn user_id_to_username(user_id: &[u8; 16]) -> String {
    bs58::encode(user_id).into_string()  // Human-readable, no PII
}
```

### Key Properties

- **Deterministic**: Same email always generates same user_id for consistency
- **One-Way**: Cryptographically impossible to reverse user_id back to email
- **High Security**: Argon2id with 19456KB memory cost following OWASP 2024 standards
- **User-Friendly**: Base58 encoding provides readable usernames without confusing characters

## Zero Knowledge Database Schema

### Users Table

```sql
-- Zero Knowledge Users Table
CREATE TABLE users (
    user_id BLOB PRIMARY KEY,           -- 16-byte cryptographic hash (no PII)
    created_at INTEGER DEFAULT (unixepoch())  -- Unix timestamp (timezone-agnostic)
);
```

### Magic Links Table

```sql
-- Zero Knowledge Magic Links Table
CREATE TABLE magiclinks (
    token_hash BLOB PRIMARY KEY,        -- 16-byte Blake3 keyed hash of encrypted token
    timestamp INTEGER NOT NULL,         -- Original timestamp used in magic link creation
    encryption_blob BLOB NOT NULL,      -- 44 bytes: nonce[12] + secret_key[32] from ChaCha8RNG
    next_param TEXT,                     -- Optional next destination parameter
    expires_at INTEGER NOT NULL         -- Unix timestamp expiration
    -- No user data, emails, or PII - only cryptographic hashes and encryption metadata
);
```

## Magic Link Cryptographic Architecture

### Encryption & Integrity Flow

```
User_ID + Timestamp → ChaCha8RNG[44] → nonce[12] + secret_key[32] → ChaCha20 Encrypt → Base58 Token
                                                                         ↓
blake3_keyed_variable(hash_key, encrypted_token, 16) → Database Hash Index
```

### Security Features

- **ChaCha20 Encryption**: 32-byte encrypted magic link data using ChaCha20 stream cipher
- **Blake3 Keyed Integrity**: Prevents modification and tampering of magic links
- **Database Validation**: Additional security layer through token presence verification
- **Time-Limited**: 5-minute expiration prevents replay attacks (development: 15 minutes)
- **One-Time Use**: Magic links consumed immediately after validation
- **Optimized Length**: 44-character Base58 tokens (33% reduction from previous implementation)
- **No Email Reference**: Magic tokens contain only cryptographic hashes, never emails

## Cryptographic Security Properties

### Industry Standards

- **Blake3**: Modern cryptographic hash with XOF (eXtendable Output Function) for variable-length outputs
- **Blake3 KDF**: Key Derivation Function with domain separation for cryptographic namespace isolation
- **Argon2id**: Winner of Password Hashing Competition, memory-hard function
- **ChaCha20**: Industry-standard stream cipher with proven security record

### Multi-Layer Defense

- **Triple-Key Security (v1.6.13+)**: Three independent 64-byte keys for multi-layer protection
  - `USER_ID_HMAC_KEY` (64 bytes) - Keyed hashing in Step 2
  - `ARGON2_SALT` (64 bytes) - Dynamic salt derivation in Step 3
  - `USER_ID_ARGON2_COMPRESSION` (64 bytes) - Final keyed compression in Step 5
- **Blake3 Universal Pipeline**: Consistent blake3_keyed_variable() used in Steps 2, 3, and 5
- **Per-User Salt**: Each user gets unique Argon2id salt preventing parallel dictionary attacks
- **High Security Parameters**: Argon2id with mem_cost=19456KB, time_cost=2 exceeds current recommendations
- **Rainbow Table Resistance**: Keyed final compression makes precomputation impossible
- **Key Compromise Mitigation**: Three independent keys required for full system break

### Forward Secrecy

- **User Identity Derivation**: User identity derives from email but email is never stored
- **Session Privacy**: Sessions identified by cryptographic user IDs, never by email
- **Zero Knowledge Database**: No PII stored - only cryptographic hashes and timestamps

## Scalability & Performance

### Deterministic Lookups

- **O(1) User Identification**: Same email always produces same user_id
- **No PII Indexes**: Database indexes only on cryptographic hashes, never personal data
- **Stateless Sessions**: JWT tokens eliminate need for server-side session storage
- **Horizontal Scaling**: Zero Knowledge architecture supports distributed deployments

### Performance Benefits

- **Blake3 Performance**: ~100x faster than previous SHA3 implementation for magic links
- **SIMD Optimization**: Blake3 leverages CPU SIMD instructions (wasm32_simd) for parallel processing
- **Variable Output Efficiency**: Single blake3_keyed_variable() function handles all length requirements
- **Unified Architecture**: Blake3 KDF + XOF for all variable-length cryptographic operations
- **Minimal Dependencies**: Single blake3 crate for all hashing operations

## Development & Operations Benefits

### Safe Operations

- **Safe Logging**: All application logs use Base58 usernames, safe to store and analyze
- **Testing Friendly**: Short token durations enable rapid testing cycles (20s access, 2min refresh in dev)
- **Debug Safety**: Development logs never contain personal information
- **Incident Response**: Security incidents don't expose user personal data

### Compliance & Audit

- **GDPR Article 17**: Right to erasure not applicable - no personal data stored
- **CCPA Compliance**: No sale of personal information possible - none collected
- **SOC 2 Ready**: Comprehensive audit trails without privacy concerns
- **Data Breach Resilience**: Data breaches cannot expose personal information

## Authentication Middleware

### JWT Validation System

```rust
// JWT validation middleware (utils/auth.rs)
pub fn validate_bearer_token(req: &Request) -> Result<AuthContext, Response> {
    // 1. Extract Bearer token from Authorization header
    // 2. Validate JWT signature and expiration
    // 3. Return AuthContext with Base58 username (never email)
}

// Automatic endpoint protection
pub fn requires_authentication(path: &str) -> bool {
    // Protected: /api/custom, /api/password, /api/api-key, /api/users/*
    // Public: /api/version, /api/login/*
}
```

### Authentication Context

```rust
pub struct AuthContext {
    pub user_id: [u8; 16],           // Cryptographic user identifier
    pub username: String,            // Base58-encoded username
    pub expires_at: i64,            // Token expiration timestamp
    pub issued_at: i64,             // Token issuance timestamp
    // No email, name, or other PII
}
```

## Shared Secret Zero Knowledge Architecture (v1.8.0+)

### URL Hash Role Encoding

**CRITICAL ZERO KNOWLEDGE PRINCIPLE**: Role discrimination (sender vs receiver) is **encoded directly in the URL hash**, NOT in database queries. This ensures complete privacy and Zero Knowledge compliance.

### Hash Structure (40 bytes total)

```
URL Hash (40 bytes) = reference_hash[16] + user_id[16] + checksum[7] + role[1 bit]
                                                                          ↓
                                                              sender=0, receiver=1
```

### Role Encoding Implementation

```rust
// api/src/utils/shared_secret/hash_generation.rs
pub fn encode_role_in_hash(
    reference_hash: &[u8; 16],
    user_id: &[u8; 16],
    checksum: &[u8; 7],
    role: SecretRole
) -> [u8; 40] {
    let mut hash = [0u8; 40];
    hash[0..16].copy_from_slice(reference_hash);
    hash[16..32].copy_from_slice(user_id);
    hash[32..39].copy_from_slice(checksum);

    // CRITICAL: Role encoded in last bit of final byte
    hash[39] = match role {
        SecretRole::Sender => 0,    // Bit 0 = sender
        SecretRole::Receiver => 1,  // Bit 1 = receiver
    };

    hash
}

pub fn decode_role_from_hash(hash: &[u8; 40]) -> SecretRole {
    // Extract role from last bit of final byte
    match hash[39] & 0x01 {
        0 => SecretRole::Sender,
        _ => SecretRole::Receiver,
    }
}
```

### Zero Knowledge Properties

- **No Database Role Storage**: Database contains ONLY reference_hash - role never stored
- **URL-Based Authorization**: All sender/receiver permissions derived from URL hash itself
- **Client-Side Role Detection**: Frontend extracts role from hash without server queries
- **Complete Privacy**: Server cannot correlate sender/receiver without the URL hash
- **Metadata Leak Prevention**: No `created_at` column in tracking table (removed v1.8.10)

### Security Architecture

```
User Creates Secret → Two Hashes Generated (sender bit=0, receiver bit=1)
                              ↓
                    Database Stores: reference_hash[16] ONLY
                              ↓
                    URL Contains: Full 40-byte hash with role
                              ↓
              Retrieval: Role extracted from hash[39] bit
                              ↓
              Authorization: Server checks role → Shows OTP to sender only
```

### Metadata Leak Prevention (v1.8.10+)

**Security Enhancement**: Removed `created_at` column from `shared_secrets_tracking` table.

**Rationale**:
- Timestamp in database + email receipt time = potential correlation attack vector
- Even remote possibility violates Zero Knowledge principles
- Created timestamp already exists in encrypted payload (sufficient for sender tracking)

### Tracking Table Schema (v1.8.10+)

```sql
CREATE TABLE shared_secrets_tracking (
    reference_hash BLOB PRIMARY KEY,    -- 16-byte reference (no role information)
    pending_reads INTEGER NOT NULL,
    read_at INTEGER,                     -- Timestamp in seconds when first accessed
    expires_at INTEGER NOT NULL,         -- Expiration timestamp in hours
    encrypted_payload BLOB NOT NULL      -- Contains: content + OTP + created_at + metadata
    -- NOTE: No created_at column - prevents metadata correlation attacks
);
```

### OTP and Tracking Visibility

**Sender-Only Information** (role=0):
- **OTP**: 9-digit code visible ONLY to sender (for sharing with receiver)
- **read_at**: Timestamp when secret was first accessed (tracking)
- **Delete Permission**: Can delete secret if wrong recipient

**Receiver Information** (role=1):
- **Content**: Encrypted secret content after OTP validation
- **Reads Remaining**: Counter decremented on each access
- **Expiration**: When secret will be permanently deleted

### Implementation Files

- **api/src/utils/shared_secret/hash_generation.rs**: Role encoding/decoding logic
- **api/src/handlers/shared_secret/retrieval.rs**: Role-based response generation
- **api/src/database/operations/shared_secret_storage.rs**: Zero Knowledge storage operations

## Zero Knowledge Benefits Summary

### Technical Benefits

- **⚡ Performance**: ~100x faster cryptographic operations with Blake3 SIMD
- **🏗️ Unified Architecture**: Single Blake3 pipeline for all variable-length operations
- **🔧 Maintainability**: Universal blake3_keyed_variable() function eliminates code duplication
- **📈 Future-Proofing**: Blake3 optimized for modern SIMD-capable processors (wasm32_simd)
- **🛡️ Security**: Enhanced triple-key cryptographic protection with domain separation
- **🔐 URL-Based Authorization**: Shared secret roles encoded in hash, not database (v1.8.0+)

### Business Benefits

- **📊 Privacy Compliance**: GDPR/CCPA compliant by design
- **🛡️ Breach Resilience**: Data breaches cannot expose personal information
- **⚖️ Legal Protection**: No personal data liability
- **🔍 Audit Simplicity**: Comprehensive logs without privacy concerns
- **🌍 Global Deployment**: No data localization requirements

### User Benefits

- **🔒 Complete Privacy**: Personal information never reaches server
- **🛡️ Breach Protection**: User data cannot be compromised in breaches
- **⚡ Fast Authentication**: Efficient cryptographic authentication
- **🔄 Seamless Experience**: Transparent security without user friction
- **✅ Trust**: Verifiable privacy through open architecture

---

_For cryptographic details, see [Cryptography Documentation](../api/cryptography.md)_
_For security considerations, see [Security Documentation](./security.md)_
_For authentication flow, see [Authentication Documentation](../api/authentication.md)_
_For shared secret endpoints, see [API Endpoints Documentation](../api/endpoints.md)_
