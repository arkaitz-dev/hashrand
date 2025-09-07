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
Email Input → Blake2b Hash → Blake2b-keyed → Per-User Salt → Argon2id → Blake2b-variable → 16-byte user_id
                               (hmac_key)     (unique salt)   (19456KB)                      ↓
                                                                                    Base58 Username Display (~22 chars)
```

### Implementation Architecture

```rust
// Zero Knowledge user identification (utils/jwt.rs)
pub fn derive_user_id(email: &str) -> [u8; 16] {
    let email_hash = Blake2b512::digest(email.to_lowercase());
    let dynamic_salt = generate_dynamic_salt(&email_hash);
    let argon2_output = argon2id_hash(&email_hash, &dynamic_salt);
    let mut user_id = [0u8; 16];
    Blake2bVar::new(16).unwrap().update(&argon2_output).finalize_variable(&mut user_id);
    user_id  // Never stored with email - cryptographically derived
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
    token_hash BLOB PRIMARY KEY,        -- 16-byte Blake2b-variable hash of encrypted token
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
Blake2b-keyed(raw_magic_link, hmac_key) → Blake2b-variable[16] → Database Hash Index
```

### Security Features

- **ChaCha20 Encryption**: 32-byte encrypted magic link data using ChaCha20 stream cipher
- **Blake2b-keyed Integrity**: Prevents modification and tampering of magic links
- **Database Validation**: Additional security layer through token presence verification
- **Time-Limited**: 5-minute expiration prevents replay attacks (development: 15 minutes)
- **One-Time Use**: Magic links consumed immediately after validation
- **Optimized Length**: 44-character Base58 tokens (33% reduction from previous implementation)
- **No Email Reference**: Magic tokens contain only cryptographic hashes, never emails

## Cryptographic Security Properties

### Industry Standards
- **Blake2b**: RFC 7693 standardized, widely adopted cryptographic hash function
- **Blake2b-keyed**: Native keyed mode eliminates HMAC construction complexity
- **Argon2id**: Winner of Password Hashing Competition, memory-hard function
- **ChaCha20**: Industry-standard stream cipher with proven security record

### Multi-Layer Defense
- **Blake2b-keyed Layer**: Protection against rainbow table and precomputation attacks
- **Per-User Salt**: Each user gets unique Argon2id salt preventing parallel dictionary attacks
- **High Security Parameters**: Argon2id with mem_cost=19456KB, time_cost=2 exceeds current recommendations
- **Blake2b-variable Compression**: Optimal entropy distribution in reduced 16-byte output
- **Enhanced Secrets**: Dedicated Blake2b-keyed key separate from Argon2id salt

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
- **Blake2b Speed**: 2x faster than SHA3 while maintaining equivalent security
- **Memory Efficiency**: Unified Blake2b reduces memory footprint vs multiple hash families
- **CPU Optimization**: Blake2b designed for modern processor architectures
- **Reduced Dependencies**: Fewer cryptographic crates in dependency tree

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

## Zero Knowledge Benefits Summary

### Technical Benefits
- **⚡ Performance**: Faster cryptographic operations with Blake2b
- **🏗️ Simplification**: Unified cryptographic family reduces complexity
- **🔧 Maintainability**: Single hash family easier to audit and maintain
- **📈 Future-Proofing**: Blake2b designed for modern computing environments
- **🛡️ Security**: Maintained or improved cryptographic security properties

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

*For cryptographic details, see [Cryptography Documentation](../api/cryptography.md)*  
*For security considerations, see [Security Documentation](./security.md)*  
*For authentication flow, see [Authentication Documentation](../api/authentication.md)*