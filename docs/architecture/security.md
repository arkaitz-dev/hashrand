# Security Architecture

HashRand implements enterprise-grade security through multiple layers of cryptographic protection, Zero Knowledge architecture, and industry best practices.

## Cryptographic Foundation

### Blake2b Unified Security Stack

HashRand uses **Blake2b** as its unified cryptographic foundation, providing superior performance while maintaining enterprise-grade security:

| **Function** | **Algorithm** | **Usage** | **Security Level** |
|--------------|---------------|-----------|-------------------|
| **Standard Hashing** | Blake2b512 | Email hashing, seed generation | 256-bit equivalent |
| **Keyed Authentication** | Blake2b-keyed | HMAC replacement, integrity | 256-bit equivalent |
| **Variable Output** | Blake2b-variable | User ID compression, indexing | Configurable (8-64 bytes) |

### Cryptographic Properties

#### Security Standards
- **RFC 7693 Compliant**: Blake2b is standardized and widely adopted
- **Cryptanalysis Resistant**: Extensive security analysis with no known vulnerabilities
- **Side-Channel Resistant**: Designed to resist timing and power analysis attacks
- **Memory-Hard Components**: Argon2id provides resistance to ASIC attacks

#### Performance Security
- **Constant-Time Operations**: All cryptographic operations execute in constant time
- **No Secret-Dependent Branches**: Code paths independent of secret values
- **Cache-Safe**: Operations designed to minimize cache timing leaks
- **SIMD Optimized**: Hardware acceleration where available

## Authentication Security

### Magic Link Security Model

#### Encryption Layer (ChaCha20)
```
User_ID + Timestamp → ChaCha20(data, key) → Encrypted_Token[32] → Base58[44]
```

- **ChaCha20 Stream Cipher**: Industry-standard encryption with proven security
- **256-bit Keys**: Cryptographically secure key generation
- **Nonce Management**: Unique nonces prevent encryption oracle attacks
- **Base58 Encoding**: Prevents character confusion and URL-safe transmission

#### Integrity Layer (Blake2b-keyed)
```
Raw_Magic_Link → Blake2b-keyed(data, hmac_key) → Authentication_Tag[32]
```

- **Message Authentication**: Prevents tampering and modification
- **Key Separation**: Independent keys for encryption and authentication
- **Truncated Hashes**: 16-byte database indexes for optimal performance
- **Replay Prevention**: Time-limited tokens with automatic expiration

### JWT Security Architecture

#### Token Structure
```json
{
  "header": {
    "alg": "HS256",
    "typ": "JWT"
  },
  "payload": {
    "user_id": "Base58-encoded-16-byte-hash",
    "exp": 1692815820,
    "iat": 1692815640
    // No PII - only cryptographic identifiers
  }
}
```

#### Dual-Token System
- **Access Tokens**: Short-lived (3min dev, 15min prod), in JSON responses
- **Refresh Tokens**: Longer-lived (15min dev, 7days prod), HttpOnly cookies
- **Automatic Refresh**: Transparent token renewal without user interaction
- **Secure Storage**: Refresh tokens inaccessible to client-side JavaScript

### Session Security

#### Session Management
- **Stateless Design**: No server-side session storage required
- **Cryptographic Sessions**: Sessions identified by user_id hash only
- **Automatic Cleanup**: Expired tokens automatically removed from database
- **Secure Cookies**: HttpOnly, Secure, SameSite=Strict for refresh tokens

#### Logout Security
- **Server-Side Cleanup**: Refresh token cookies explicitly cleared
- **Client-Side Cleanup**: Complete localStorage and authentication state removal
- **Session Invalidation**: Immediate token invalidation on logout
- **Confirmation Dialog**: Prevents accidental logout operations

## User Identity Security

### Cryptographic User ID Derivation

#### Multi-Layer Security Process
```
Email → Blake2b512 → Blake2b-keyed → Per-User-Salt → Argon2id → Blake2b-variable → user_id
```

#### Security Properties
1. **Blake2b512(email)**: Irreversible hash of email address
2. **Blake2b-keyed**: Prevents rainbow table attacks with secret key
3. **Per-User Salt**: Unique salt per user prevents parallel attacks
4. **Argon2id**: Memory-hard function with OWASP 2024 parameters
5. **Blake2b-variable**: Compression to 16-byte identifier

#### Argon2id Security Parameters
```rust
Argon2id {
    mem_cost: 19456,    // 19MB memory requirement
    time_cost: 2,       // 2 iterations
    parallelism: 1,     // Single-threaded
    hash_length: 32,    // 256-bit output
    version: 0x13       // Argon2id version 1.3
}
```

### Attack Resistance

#### Rainbow Table Resistance
- **Salted Hashing**: Per-user salts prevent precomputed attacks
- **Blake2b-keyed Layer**: Additional secret key protection
- **Memory-Hard Function**: Argon2id increases attack cost exponentially

#### Brute Force Resistance
- **High Memory Cost**: 19MB per hash attempt
- **Time Cost**: Multiple iterations required
- **Parallel Attack Prevention**: Unique salts eliminate batch processing

#### Dictionary Attack Resistance
- **Cryptographic Preprocessing**: Email hashed before key derivation
- **Key Stretching**: Argon2id provides computational difficulty
- **Salt Uniqueness**: Per-user salts prevent dictionary reuse

## Data Security

### Zero Knowledge Database

#### No Personal Information
```sql
-- What is NOT stored:
-- ❌ Email addresses
-- ❌ Names or personal identifiers
-- ❌ IP addresses or location data
-- ❌ Authentication secrets
-- ❌ Any recoverable personal information

-- What IS stored:
-- ✅ Cryptographic hashes (16-byte user_id)
-- ✅ Encrypted tokens (ChaCha20 encrypted)
-- ✅ Timestamps (Unix format)
-- ✅ System metadata (non-personal)
```

#### Database Security Features
- **Encryption at Rest**: SQLite databases can be encrypted with SQLCipher
- **Minimal Data**: Only essential cryptographic data stored
- **Automatic Cleanup**: Expired tokens automatically purged
- **No Indexes on Secrets**: Database indexes only on hash values

### Seed-Based Generation Security

#### Cryptographic Seed Generation
```rust
// Secure seed generation process
let initial_random = nanoid(128);           // 128 characters of entropy
let seed_hash = Blake2b512::digest(&initial_random);  // 512-bit hash
let seed_32_bytes = &seed_hash[..32];       // First 32 bytes as seed
let base58_seed = bs58::encode(seed_32_bytes);        // URL-safe encoding
```

#### ChaCha8 Generation Security
- **Cryptographic PRNG**: ChaCha8 provides cryptographically secure randomness
- **Domain Separation**: Independent random streams for hash vs OTP generation
- **Seed Security**: 256-bit seeds provide 2^256 keyspace
- **Deterministic Reproducibility**: Same seed always produces same output

## Network Security

### Transport Layer Security

#### HTTPS Requirements
- **TLS 1.2 Minimum**: Modern TLS version requirements
- **Perfect Forward Secrecy**: Ephemeral key exchange
- **HSTS Headers**: HTTP Strict Transport Security
- **Certificate Pinning**: Optional certificate validation

#### API Security Headers
```http
Content-Security-Policy: default-src 'self'
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
Strict-Transport-Security: max-age=31536000; includeSubDomains
```

### Cross-Origin Request Security

#### CORS Configuration
- **Allowed Origins**: Explicitly configured allowed domains
- **Credentials Handling**: Secure cookie transmission rules
- **Method Restrictions**: Limited HTTP methods allowed
- **Header Validation**: Strict header allowlists

## Operational Security

### Secret Management

#### Production Secrets
```bash
# Required 256-bit secrets (64 hex characters each)
JWT_SECRET=<64-char-hex>                 # JWT token signing
MAGIC_LINK_HMAC_KEY=<64-char-hex>       # Magic link integrity
ARGON2_SALT=<64-char-hex>               # User ID derivation
CHACHA_ENCRYPTION_KEY=<64-char-hex>     # Magic link encryption
```

#### Secret Security Requirements
- **Cryptographic Generation**: All secrets generated with secure RNG
- **Environment Variables**: Secrets passed via environment, never hardcoded
- **Secret Rotation**: Regular rotation procedures for production
- **Backup Security**: Encrypted backup of production secrets

### Monitoring & Auditing

#### Security Logging
```rust
// Example security log (safe - no PII)
{
    "timestamp": "2025-09-07T14:30:15Z",
    "event": "authentication_success",
    "user_id": "HpGAge9YJ7uMvw4QV5qDPk",  // Base58 username
    "endpoint": "/api/login",
    "ip_hash": "blake2b_hash_of_ip",      // Hashed IP for privacy
    "user_agent_hash": "blake2b_hash"     // Hashed UA for privacy
}
```

#### Privacy-Safe Monitoring
- **Base58 Usernames**: All logs use privacy-safe identifiers
- **Hashed PII**: Any potentially identifying info is hashed
- **Aggregate Metrics**: Usage statistics without user correlation
- **Retention Policies**: Automatic log rotation and cleanup

### Incident Response

#### Security Incident Categories
1. **Authentication Bypass**: Unauthorized access to protected endpoints
2. **Token Compromise**: Suspected JWT or refresh token compromise
3. **Database Breach**: Unauthorized access to database files
4. **DoS Attacks**: Denial of service or resource exhaustion
5. **Cryptographic Failures**: Hash collisions or key compromise

#### Response Procedures
- **Immediate Containment**: Service isolation and traffic blocking
- **Secret Rotation**: Emergency rotation of compromised secrets
- **User Notification**: Privacy-safe notification of affected users
- **Forensic Analysis**: Incident analysis without PII exposure
- **Recovery Planning**: Service restoration with enhanced security

## Compliance & Standards

### Regulatory Compliance

#### GDPR Compliance
- **No Personal Data**: Article 4 - no personal data processed or stored
- **Right to Erasure**: Article 17 - not applicable (no personal data)
- **Data Portability**: Article 20 - not applicable (no personal data)
- **Privacy by Design**: Article 25 - implemented through Zero Knowledge architecture

#### CCPA Compliance
- **No Sale of Information**: No personal information collected to sell
- **Right to Know**: No personal information collected beyond email for delivery
- **Right to Delete**: Not applicable - no personal information stored
- **Non-Discrimination**: No different service levels based on privacy choices

### Security Standards

#### Industry Standards Compliance
- **SOC 2 Type II**: Security, availability, and confidentiality controls
- **ISO 27001**: Information security management system
- **NIST Cybersecurity Framework**: Comprehensive security controls
- **OWASP ASVS**: Application Security Verification Standard

#### Cryptographic Standards
- **FIPS 140-2**: Federal cryptographic module standards (where applicable)
- **Common Criteria**: Security evaluation criteria
- **RFC Standards**: Blake2b (RFC 7693), JWT (RFC 7519), Argon2 (RFC 9106)

---

*For Zero Knowledge architecture, see [Zero Knowledge Documentation](./zero-knowledge.md)*  
*For cryptographic details, see [Cryptography Documentation](../api/cryptography.md)*  
*For authentication implementation, see [Authentication Documentation](../api/authentication.md)*