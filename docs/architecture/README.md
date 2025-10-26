# Architecture

HashRand is built with modern web technologies and enterprise-grade cryptography, delivering security and performance through WebAssembly.

## Technology Stack

### Frontend
- **Framework**: SvelteKit (TypeScript)
- **Styling**: TailwindCSS
- **Internationalization**: rust-i18n integration (13 languages)
- **Cryptography**: WebCrypto API + custom Ed25519/X25519 implementations
- **Storage**: IndexedDB (encrypted keys, session data)

### Backend
- **Language**: Rust
- **Framework**: Fermyon Spin (WebAssembly runtime)
- **Database**: SQLite via Turso
- **Cryptography**: ring, ed25519-dalek, x25519-dalek, chacha20poly1305, blake3
- **HTTP**: spin-sdk HTTP components

### Infrastructure
- **Deployment**: WebAssembly binaries on Fermyon Spin
- **Ports**: Backend (3000), Frontend (5173)
- **Database**: SQLite with Turso for production scalability

## Cryptographic Architecture

### Dual-Key System

HashRand implements two independent cryptographic key systems:

**System A: Temporary Session Keys**
- **Purpose**: Secure frontend ↔ backend communication
- **Lifecycle**: Short-lived, rotated frequently
- **Keys**: Ed25519 (signing), X25519 (encryption)
- **Storage**: Frontend IndexedDB only, backend derives on-demand
- **Usage**: JWT validation, request signing, response encryption

**System B: Permanent User Keys**
- **Purpose**: User-to-user end-to-end encryption (future features)
- **Lifecycle**: Long-lived, deterministic derivation
- **Keys**: Ed25519 (signing), X25519 (encryption)
- **Storage**: Private keys in IndexedDB, public keys in database
- **Derivation**: `blake3_kdf(email, privkey_context)` for reproducibility

### Zero Knowledge Authentication

**Magic Link Flow**:
1. User requests access via email
2. Server generates encrypted `privkey_context` (unique per user)
3. Magic link contains ECDH-encrypted session data
4. Client decrypts, derives permanent keys (System B)
5. Server stores only `user_id` (Blake3 hash of email, irreversible)
6. Server **never sees or stores** the email in plaintext

**Benefits**:
- Server cannot identify users without email
- Database breach reveals no user identities
- Session tokens cryptographically bound to user keys
- The magic link hash is encrypted, revealing no information to an attacker.
- The magic link can only be validated in the same browser that requested it, which then triggers the session.

### Request/Response Security

**Signed Requests**:
- All API requests include Ed25519 signature
- Signature covers: method, path, timestamp, body
- Prevents tampering, replay attacks, man-in-the-middle

**Encrypted Requests/Responses**:
- Sensitive data encrypted with ChaCha20-Poly1305
- Per-request ephemeral keys (System A)
- Perfect Forward Secrecy: compromised keys don't expose past communications

### Data Encryption

**Shared Secrets**:
- Client-side encryption before transmission
- ChaCha20-Poly1305 authenticated encryption
- Server stores only encrypted blobs
- Decryption keys derived from URL hash (never sent to server)

**Database Encryption**:
- `user_privkey_context`: ChaCha20-Poly1305 encrypted, used to derive private keys System B keys into user's browser.
- `shared_secrets`: ChaCha20-Poly1305 encrypted payloads
- Even with database access, attackers cannot decrypt data without client keys

### URL Hash Role Encoding

Shared secrets use URL hash fragments for access control:

- **Hash Structure**: `reference_hash[16] + user_id[16] + checksum[7] + role[1 bit]`
- **Role 0**: Sender (can view OTP, track reads, delete)
- **Role 1**: Receiver (can access secret with OTP)
- **Zero Knowledge**: Server cannot determine role without URL hash
- **Privacy**: No metadata leakage (no `created_at` correlation attacks)

## Database Schema

### Tables

**`users`**: User session tracking
- `user_id` (BLOB, 16 bytes, primary key) - Multiple keyed blake 3 and Argon2id over email.
- `logged_in` (INTEGER) - Unix timestamp
- `created_at` (INTEGER) - Account creation time

**`user_ed25519_keys`**: Permanent signing keys (System B)
- `user_id` (BLOB, foreign key)
- `pub_key` (TEXT, hex) - Ed25519 public key
- `created_at` (INTEGER) - Publication timestamp

**`user_x25519_keys`**: Permanent encryption keys (System B)
- `user_id` (BLOB, foreign key)
- `pub_key` (TEXT, hex) - X25519 public key
- `created_at` (INTEGER) - Publication timestamp

**`user_privkey_context`**: Encrypted key derivation material
- `db_index` (BLOB, 16 bytes, primary key) - Obtained like user_id but with different keys.
- `encrypted_privkey` (BLOB) - ChaCha20-Poly1305 encrypted context
- `created_year` (INTEGER) - For key rotation strategies

**`magiclinks`**: One-time authentication tokens
- `token_hash` (BLOB, primary key)
- `encrypted_payload` (BLOB) - Contains email, expiration
- `created_at` (INTEGER)

**`shared_secrets`**: Encrypted shared data
- `reference_hash` (BLOB, 16 bytes, primary key)
- `encrypted_secret` (BLOB) - ChaCha20-Poly1305 encrypted
- `created_at` (INTEGER)
- `expires_at` (INTEGER)

**`shared_secrets_tracking`**: Read limits and monitoring
- `reference_hash` (BLOB, primary key)
- `max_reads` (INTEGER) - NULL = unlimited
- `current_reads` (INTEGER)
- `read_at` (INTEGER, nullable) - First read timestamp
- `otp_hash` (BLOB) - SHA-256(OTP) for verification

## Security Principles

- **Client-Side Encryption**: Server never sees plaintext secrets
- **Zero Knowledge**: Server cannot identify users or correlate data
- **Perfect Forward Secrecy**: Past communications remain secure
- **Defense in Depth**: Multiple layers (transport, application, database)
- **Principle of Least Privilege**: Server stores minimal sensitive data
- **Cryptographic Agility**: Modern, proven algorithms (Ed25519, X25519, ChaCha20, Blake3)

---

**Result**: Military-grade security with user-friendly experience. Even with full server compromise, user data remains protected by client-side cryptography.
