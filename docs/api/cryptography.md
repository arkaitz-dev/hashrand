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
// Zero Knowledge user identification (utils/jwt/crypto.rs)
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
): { encrypted: string; idx: string } {
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
- **SessionStorage KV**: Efficient key-value storage with automatic cleanup
- **20-Seed Rotation**: FIFO (First In, First Out) automatic cleanup prevents memory bloat
- **8-Byte Cryptographic Keys**: Blake2b-derived identifiers for optimal sessionStorage indexing

### Bidirectional Navigation Flow

#### Backend → Frontend (Next Parameter)
```typescript
// Layout interceptor (web/src/routes/+layout.svelte)
if (loginResponse.next) {
    const encryptedNextUrl = encryptNextUrl(loginResponse.next, {
        cipherToken, nonceToken, hmacKey
    });
    await goto(encryptedNextUrl);  // /custom?encrypted=...&idx=...
}
```

#### Configuration → Result (Generate Buttons)
```typescript
// All config routes (custom/, password/, api-key/, mnemonic/)
const encryptedUrl = createEncryptedUrl('/result', resultParams, {
    cipherToken, nonceToken, hmacKey
});
goto(encryptedUrl);  // /result?encrypted=...&idx=...
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
- **Replay Protection**: Time-bounded sessionStorage prevents replay attacks

#### ⚡ Performance Optimization
- **Efficient Storage**: 8-byte keys minimize sessionStorage overhead
- **Automatic Cleanup**: FIFO rotation prevents storage bloat
- **Fast Crypto**: ChaCha20-Poly1305 optimized for modern web browsers
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
- **api/src/utils/jwt/crypto.rs**: User ID derivation and cryptographic operations
- **api/src/utils/jwt/magic_links.rs**: Magic link generation and processing
- **api/src/database/operations/magic_link_ops.rs**: Magic link encryption/decryption
- **api/src/utils/random_generator.rs**: Seed generation with Blake2b512

#### Frontend (TypeScript/SvelteKit)
- **web/src/lib/crypto.ts**: Complete URL parameter encryption system
  - `encryptUrlParams()` / `decryptUrlParams()`: Core AEAD encryption/decryption
  - `createEncryptedUrl()`: High-level URL generation for navigation
  - `parseNextUrl()` / `encryptNextUrl()`: Backend response processing
  - `storePrehashSeed()` / `getPrehashSeed()`: FIFO sessionStorage management
- **web/src/routes/+layout.svelte**: Magic link next parameter encryption
- **web/src/routes/{custom,password,api-key,mnemonic}/+page.svelte**: Route-specific encryption/decryption
- **web/src/routes/result/+page.svelte**: Result page parameter processing and return URL generation

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