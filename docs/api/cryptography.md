# Cryptographic Architecture

HashRand uses **Blake2b** as its unified cryptographic foundation, providing superior performance while maintaining enterprise-grade security standards.

## Blake2b Unified Architecture

### Core Cryptographic Stack

| **Function** | **Algorithm** | **Usage** | **Output** |
|--------------|---------------|-----------|------------|
| **Standard Hashing** | Blake2b512 | Email hashing, seed generation | 64 bytes |
| **Keyed Authentication** | Blake2b-keyed | HMAC replacement, integrity verification | 32 bytes |
| **Variable Output** | Blake2b-variable | User ID compression, database indexing | 8-64 bytes |

### Migration from SHA3 Stack

**Previous (SHA3/HMAC/SHAKE):**
```
SHA3-256 → HMAC-SHA3-256 → SHAKE256
Multiple algorithms, complex interactions
```

**Current (Blake2b Unified):**
```
Blake2b512 → Blake2b-keyed → Blake2b-variable
Single cryptographic family, simplified architecture
```

## User ID Derivation System

### Complete Cryptographic Flow

```
Email Input → Blake2b512(email) → Blake2b-keyed(result, hmac_key) → derive_user_salt() → Argon2id() → Blake2b-variable(16) → user_id
```

### Implementation Details

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

### Security Properties

#### ✅ Cryptographic Security
- **Industry Standards**: Blake2b, Blake2b-keyed, Argon2id, and Blake2b-variable are industry-standard approved algorithms
- **Multi-Layer Defense**: Blake2b-keyed layer adds protection against rainbow table and precomputation attacks
- **Per-User Salt**: Each user gets unique Argon2id salt preventing parallel dictionary attacks
- **High Security Parameters**: Argon2id with mem_cost=19456KB, time_cost=2 exceeds current security recommendations
- **Blake2b-variable Compression**: Optimal entropy distribution in reduced 16-byte output
- **Enhanced Secrets**: Dedicated Blake2b-keyed key separate from Argon2id salt for additional security layers
- **Forward Secrecy**: User identity derives from email but email is never stored

## Magic Link Cryptography

### Encryption & Integrity Flow

```
User_ID + Timestamp → ChaCha8RNG[44] → nonce[12] + secret_key[32] → ChaCha20 Encrypt → Base58 Token
                                                                         ↓
Blake2b-keyed(raw_magic_link, hmac_key) → Blake2b-variable[16] → Database Hash Index
```

### Security Architecture

- **ChaCha20 Encryption**: 32-byte encrypted magic link data using ChaCha20 stream cipher
- **Blake2b-keyed Integrity**: Prevents modification and tampering of magic links (replaces HMAC)
- **Blake2b-variable Compression**: Optimal 16-byte hash indexing for database storage
- **Time-Limited**: 5-minute expiration prevents replay attacks (development: 15 minutes)
- **One-Time Use**: Magic links consumed immediately after validation
- **Optimized Length**: 44-character Base58 tokens (33% reduction from previous 66-character)

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

## Implementation Architecture

### Rust Dependencies

```toml
[dependencies]
blake2 = "0.10"              # Blake2b hashing for unified cryptographic operations
argon2 = "0.5.3"            # Argon2id for secure user ID derivation
chacha20 = "0.9.1"          # ChaCha20 stream cipher for magic link encryption
chacha20poly1305 = "0.10.1" # ChaCha20-Poly1305 AEAD for secure magic link encryption
base64 = "0.22.1"           # Base64 encoding for JWT tokens
```

### Key Implementation Files

- **api/src/utils/jwt.rs**: User ID derivation and magic link generation
- **api/src/database/operations.rs**: Magic link encryption/decryption
- **api/src/utils/random_generator.rs**: Seed generation with Blake2b512

## Security Considerations

### Cryptographic Strength

- **Blake2b**: RFC 7693 standardized, widely adopted, equivalent security to SHA3
- **ChaCha20**: Industry-standard stream cipher, resistance to timing attacks
- **Argon2id**: Winner of Password Hashing Competition, memory-hard function
- **Base58**: Bitcoin-standard encoding, eliminates character confusion

### Attack Resistance

- **Rainbow Tables**: Blake2b-keyed with unique keys prevents precomputation attacks
- **Timing Attacks**: Constant-time implementations in all cryptographic operations
- **Side-Channel**: ChaCha20 and Blake2b designed for side-channel resistance
- **Quantum Resistance**: While not post-quantum, provides maximal classical security

---

*For API usage, see [API Endpoints](./endpoints.md)*  
*For authentication flow, see [Authentication Documentation](./authentication.md)*