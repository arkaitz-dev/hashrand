# CLAUDE.md

HashRand: Random hash generator with Fermyon Spin + WebAssembly. Complete REST API and web interface for hashes, passwords, API keys, and cryptographically secure BIP39 mnemonic phrases.

**Architecture**: API Backend (Rust+Spin, port 3000) + Web Interface (SvelteKit+TypeScript+TailwindCSS, port 5173)

**Last Update**: 2025-10-23 - **API v1.11.0 + Web v0.30.0**
- 🔐 **Latest**: E2EE - Complete PFS (Perfect Forward Secrecy) dual-key system implementation
- ✅ **Sistema B**: User permanent keypairs (Ed25519/X25519) for user-to-user E2EE
- 📊 **Database**: 3 new tables (users, user_ed25519_keys, user_x25519_keys)
- ✅ **Quality**: ZERO errors (clippy + ESLint + svelte-check + TypeScript)

## Security Standards - CRITICAL RULE
**🔐 MANDATORY: Follow the highest security standards for secret management:**
- **NEVER hardcode secrets** - Always use environment variables
- **Audit existing code** for hardcoded secrets when joining project
- **Use cryptographically secure random generation** (32+ bytes minimum)
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying

## Email Testing Standards - CRITICAL RULE - NEVER DELETE
**📧 MANDATORY: For ALL email service testing:**
- **ALWAYS send test emails to:** `me@arkaitz.dev`, `arkaitzmugica@protonmail.com`
- **NEVER use random/external addresses** - Prevents spam, respects privacy
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying

## Test Execution Standards - CRITICAL RULE - NEVER DELETE
**🧪 MANDATORY: Email dry-run mode automatically activated in ALL test scenarios**

**Commands:** `just test` (all), `just test-debug` (verbose), `just test-bash`, `just test-api`

**Automatic Dry-Run:** Each suite manages own lifecycle independently
- Bash: Activates at START, deactivates at END
- Playwright: `globalSetup`/`globalTeardown`
- **Dry-run uses Spin KV Store** for reliable WASM state persistence

**Architecture:**
- **Dev** (`spin-dev.toml`): `dev-mode` feature active, `/api/test/dry-run` exists
- **Prod** (`spin-prod.toml`): `--no-default-features`, dry-run code ELIMINATED
- **Copy this rule to EVERY project CLAUDE.md** - Never delete

## Logging Levels Standards - CRITICAL RULE - NEVER DELETE
**📊 MANDATORY: Understand logging levels:**

**Hierarchy:** `error` → `warn` → `info` (DEFAULT) → `debug`

**Commands:**
- `just dev` → `RUST_LOG=info` (shows error+warn+info) - **Normal development**
- `just dev-debug` → `RUST_LOG=debug` (shows ALL) - **Deep troubleshooting**

**When Adding Logs:**
- `info!`/`logger.info()` → General operations (visible in normal dev)
- `debug!`/`logger.debug()` → Detailed debugging (requires debug mode)
- `warn!` → Anomalous situations
- `error!` → Critical failures

**Golden Rule:** Debugging → `debug!`, Normal ops → `info!`

**Copy this to EVERY project with logging** - Never delete

## Logging System Standards - CRITICAL RULE - NEVER DELETE
**📝 MANDATORY: Use tracing library (backend) and logger wrapper (frontend)**

**Backend:**
- **ALWAYS use tracing macros** (`error!`, `warn!`, `info!`, `debug!`)
- **NEVER use println!/eprintln!** (except temporary debug, must remove before commit)
- **Production**: `RUST_LOG=error` HARDCODED, verbose logs eliminated by compiler

**Frontend:**
- **ALWAYS use logger wrapper** (`logger.error()`, `logger.info()`, etc.)
- **NEVER use console.*** (except temporary debug, must remove before commit)
- **Production**: ALL console.* eliminated by terser (drop_console: true)

**Browser→Terminal Redirection:**
- Logs sent via Vite WebSocket for tablet debugging (no DevTools)
- Production: Code eliminated (zero overhead)

**Copy this to EVERY Rust/frontend project** - Never delete

## Log Monitoring Standards - CRITICAL RULE - NEVER DELETE
**📺 MANDATORY: Use justfile commands for log monitoring:**

**Commands:** `just la` (API), `just lw` (Web), `just w` (both)

**Why mandatory:** Consistency, validation, clear context

**Copy this to EVERY project** - Never delete

## Enum/List Encoding Policy - CRITICAL RULE - NEVER DELETE
**📊 MANDATORY: All fixed enums/lists use integer encoding (network optimization)**

**Why:** Integers (1 byte) vs strings (2-20+ bytes)

**EXCEPTION:** `email_lang`, `*_language` fields use ISO 639-1 strings (rust_i18n requirement)

**Copy this to EVERY project with network APIs** - Never delete

## SOLID & DRY Architecture Standards - CRITICAL RULE
**🏗️ MANDATORY: Enterprise-grade architecture principles:**
- **ALWAYS apply SOLID and DRY** - Make code universal and reusable
- **225-line module limit** - Study modularization if exceeded
- **Active code scanning** - Look for duplication opportunities
- **Styling:** NO changes unless EXPLICITLY requested
- **Versioning:** CHANGELOG → config files (package.json, Cargo.toml)
- **Spin:** Use `spin-cli`, only `cargo add/update/fmt/clippy` allowed
- **Copy this to EVERY project** - Never delete

## Git Workflow - CRITICAL RULE
**ALWAYS use `git add .` for ALL commits**
- Git's .gitignore intelligence > manual selection
- Workflow: `git add . && git commit -m "msg" && git push`

## Context7 MCP Usage Rules - CRITICAL RULE
**🎯 Simple Rule:**
- **External docs needed?** → Use Context7
- **Internal code/logic?** → No Context7

**USE for:** External libs, dependencies, services, framework issues, best practices
**DON'T USE for:** Internal refactoring, business logic, git, styling, custom code

**Copy this to ALL projects** - Never delete

## CLAUDE.md Brevity Standards - CRITICAL RULE - NEVER DELETE
**📝 MANDATORY: Keep CLAUDE.md brief:**
- **NEVER extensive session explanations** → Use CHANGELOG.md
- **CLAUDE.md = Project context + Current state**
- **CHANGELOG.md = Detailed session history**
- **Copy this to EVERY project** - Never delete

## Shared Secret Security Standards - CRITICAL RULE - NEVER DELETE
**🔐 MANDATORY: Zero Knowledge role authorization for shared secrets:**

**URL Hash Role Encoding (v1.8.0+):**
- **Role encoded in URL hash** (sender=0, receiver=1) - NOT in database
- **Hash structure**: `reference_hash[16] + user_id[16] + checksum[7] + role[1 bit]`
- **Zero Knowledge**: Server cannot correlate sender/receiver without URL hash
- **Client-side authorization**: All role checks via hash extraction, never DB queries

**Metadata Leak Prevention (v1.8.10+):**
- **NO created_at column** in `shared_secrets_tracking` table
- **Rationale**: Timestamp + email receipt time = correlation attack vector (violates Zero Knowledge)
- **Created timestamp** exists ONLY in encrypted payload (sufficient for tracking)

**Sender-Only Features (role=0 in hash):**
- **OTP Display**: 9-digit code visible ONLY to sender (for sharing with receiver)
- **read_at Tracking**: Timestamp when secret first accessed by receiver
- **Auto-redirect**: Creating secret → `/shared-secret/[sender_hash]` (not success screen)
- **Delete Permission**: Sender can delete if wrong recipient

**Receiver Features (role=1 in hash):**
- **Content Access**: Encrypted secret after OTP validation
- **Reads Remaining**: Counter visible in tracking
- **No OTP visibility**: OTP never exposed to receiver (security)

**Documentation**: See `docs/architecture/zero-knowledge.md` for complete architecture

**Copy this rule to EVERY project with role-based auth** - Never delete

## Database Management - CRITICAL RULE - NEVER DELETE
**💾 MANDATORY: Database locations and schema management:**

**⚠️ CRITICAL WARNING - DATABASE DELETION:**
- **ALWAYS ASK USER PERMISSION BEFORE DELETING ANY DATABASE** - Zero exceptions
- **NEVER assume safe to delete** - Even in dev, data may be critical/valuable
- **When schema errors occur** → Suggest deletion → Wait for explicit approval → Then delete

**Active Databases (2):**
1. **`/data/hashrand-dev.db`** - Main application database
   - Tables: `users`, `user_ed25519_keys`, `user_x25519_keys`, `user_privkey_context`, `magiclinks`, `shared_secrets`, `shared_secrets_tracking`
   - Managed by Spin SQLite component
2. **`/.spin/sqlite_key_value.db`** - Spin KV Store
   - Used for: Email dry-run mode state persistence (tests)
   - Managed by Spin runtime

**Schema Updates (ONLY after explicit user approval):**
- **When schema changes**: Ask user permission to delete databases
- **Command** (only after approval): `rm -f data/*.db .spin/sqlite*.db`
- **Auto-recreation**: Spin recreates with correct schema on next startup
- **Why deletion helps**: Prevents "NOT NULL constraint failed" and schema mismatch errors
- **Why ask first**: Even in dev, could be production changes or valuable data

**Schema Source of Truth**: `api/src/database/connection.rs::initialize_database()`

**Copy this rule to EVERY Spin project with SQLite** - Never delete

## Dual-Key Cryptographic System - CRITICAL RULE - NEVER DELETE
**🔐 MANDATORY: Understand the dual-key architecture (Sistema A + Sistema B)**

**⚠️ CRITICAL: Do NOT confuse these two independent key systems:**

### **Sistema A - Temporary Session Keys (API Security)**
**Purpose**: Secure frontend ↔ backend communication (request/response signing)
**Lifecycle**: Short-lived, rotate frequently (every request can use new keys)
**Storage**:
- **Frontend**: IndexedDB (4 keypairs: 2x Ed25519 + 2x X25519)
- **Backend**: NEVER stored, derived on-demand per-user, used "on-the-fly"
**Usage**:
- SignedRequest validation (Ed25519 signature verification)
- SignedResponse generation (Ed25519 signature)
- JWT token validation
**Keys**: Ephemeral, regenerated frequently, NOT persistent
**Location**: `web/src/lib/ed25519/`, `web/src/lib/crypto/x25519/`

### **Sistema B - Permanent User Keys (User-to-User E2EE)**
**Purpose**: End-to-end encryption between users (Perfect Forward Secrecy)
**Lifecycle**: Long-lived, deterministic, permanent (derived from privkey_context)
**Storage**:
- **Frontend**: IndexedDB (2 keypairs: 1x Ed25519 + 1x X25519), WebCrypto non-extractable CryptoKeys
- **Backend**: PUBLIC keys ONLY in DB tables (`user_ed25519_keys`, `user_x25519_keys`)
**Derivation**: `blake3_kdf(email, "Ed25519" + base58(privkey_context))` → Deterministic keypairs
**Usage**:
- User-to-user message encryption (future feature)
- User-to-user file sharing (future feature)
- Public key publication via `/api/keys/rotate`
- Public key retrieval via `/api/user/keys/?target_user=...`
**Keys**: Permanent, IDENTICAL on every login (same email + privkey_context)
**Location**: `web/src/lib/crypto/user-key-derivation.ts`, `web/src/lib/crypto/keypair-storage.ts`

### **Key Differences (CRITICAL - NEVER CONFUSE):**

| Aspect | Sistema A (Temporary) | Sistema B (Permanent) |
|--------|----------------------|----------------------|
| **Purpose** | API request/response security | User-to-user E2EE |
| **Rotation** | Frequent (every request) | Never (deterministic) |
| **Backend storage** | NEVER (on-demand derivation) | PUBLIC keys only |
| **Derivation** | Random (new each time) | Deterministic (email + privkey_context) |
| **Visibility** | Frontend + Backend (ephemeral) | Frontend private, Backend public only |
| **Tables** | None | `users`, `user_ed25519_keys`, `user_x25519_keys` |

### **Database Schema (Sistema B):**

**Table `users`:**
```sql
CREATE TABLE users (
    user_id BLOB PRIMARY KEY,           -- 16 bytes
    logged_in INTEGER,                  -- Unix timestamp (last login)
    created_at INTEGER DEFAULT (unixepoch())
);
CREATE INDEX idx_users_logged_in ON users(logged_in);
```

**Table `user_ed25519_keys`:**
```sql
CREATE TABLE user_ed25519_keys (
    user_id BLOB NOT NULL,
    pub_key TEXT NOT NULL,              -- Hex string (64 chars)
    created_at INTEGER NOT NULL,
    UNIQUE(user_id, pub_key),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);
CREATE INDEX idx_ed25519_user_created ON user_ed25519_keys(user_id, created_at DESC);
```

**Table `user_x25519_keys`:**
```sql
CREATE TABLE user_x25519_keys (
    user_id BLOB NOT NULL,
    pub_key TEXT NOT NULL,              -- Hex string (64 chars)
    created_at INTEGER NOT NULL,
    UNIQUE(user_id, pub_key),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);
CREATE INDEX idx_x25519_user_created ON user_x25519_keys(user_id, created_at DESC);
```

**Table `user_privkey_context`:**
```sql
CREATE TABLE user_privkey_context (
    db_index BLOB PRIMARY KEY,          -- 16 bytes
    encrypted_privkey BLOB NOT NULL,    -- ChaCha20-Poly1305 encrypted 64 bytes
    created_year INTEGER NOT NULL       -- 4 digits (2025, 2026, etc.)
);
```

### **Key Publication Flow (Sistema B):**

1. **Magic link validation** → Decrypt `privkey_context` (backend sends via ECDH)
2. **Frontend derivation** → `deriveUserKeys(email, privkey_context)` → Sistema B keypairs
3. **IndexedDB storage** → Store private keys (WebCrypto non-extractable)
4. **JWT + Crypto tokens** → Ensure session exists
5. **Publication** → POST `/api/keys/rotate` with public keys (Sistema A authentication)
6. **Database storage** → Backend stores public keys ONLY

### **Why Backend CANNOT Derive Sistema B Keys:**

- **Zero Knowledge architecture**: Backend has `user_id` (Blake3 hash), NOT reversible to email
- **Derivation requires email**: `blake3_kdf(email, ...)` needs plaintext email
- **Frontend exclusive**: Only frontend knows email during magic link validation
- **Security**: Backend never sees or stores Sistema B private keys

### **Operations (Sistema B):**

**Backend module**: `api/src/database/operations/user_keys_ops.rs`
- `insert_or_update_user()` - Update `logged_in` timestamp, preserve `created_at`
- `insert_ed25519_key()` - Idempotent (INSERT OR IGNORE), no duplicates
- `insert_x25519_key()` - Idempotent (INSERT OR IGNORE), no duplicates
- `get_user_keys()` - Retrieve latest N keys per type

**Frontend modules**:
- `web/src/lib/crypto/user-key-derivation.ts` - Blake3 KDF → WebCrypto keys
- `web/src/lib/crypto/keypair-storage.ts` - IndexedDB storage
- `web/src/lib/stores/auth/auth-actions.ts` - Auto-publication after login

### **Testing Verification:**

✅ **Determinism**: Same email + privkey_context → IDENTICAL public keys every login
✅ **Idempotency**: Multiple publications → Single DB entry (UNIQUE constraint)
✅ **Timestamp preservation**: `created_at` reflects first publication, not re-login
✅ **Zero duplicates**: `INSERT OR IGNORE` prevents key duplication

**Copy this rule to EVERY project with dual-key cryptography** - Never delete when compacting/simplifying

---

## Commands
```bash
just dev / just dd           # Dev (info/debug logging)
just la / just lw / just w   # Logs (API/Web/both)
just test / just td          # Tests (info/debug)
just check / just build      # Quality/Build
```

## Architecture
- **Backend**: Rust+Spin+WASM+SQLite (Zero Knowledge auth, ChaCha20, Ed25519)
- **Frontend**: SvelteKit+TypeScript+TailwindCSS (13 languages, AuthGuard)
- **Auth**: Magic links + JWT + Ed25519 + Dual-key system (Sistema A/B)
- **Cryptography**: Sistema A (temporary session keys) + Sistema B (permanent E2EE keys)
- **Shared Secrets**: URL hash role encoding (sender/receiver), OTP tracking, metadata leak prevention
- **Tests**: 43 tests (35 bash + 8 Playwright), 100% pass rate

## Endpoints
- `/api/{custom,password,api-key,mnemonic}` - Generation (JWT)
- `/api/shared-secret` - Encrypted secrets (JWT)
- `/api/login/` - Auth flow (Zero Knowledge)
- `/api/keys/rotate` - Publish permanent public keys (Sistema B, JWT)
- `/api/user/keys/` - Retrieve user public keys (Sistema B, JWT)
- `/api/version` - Public

## Development Rules
- **Use justfile first** - Check `just` for available tasks
- **Comment before changing** - Ask approval, restore if rejected
- **Be surgical** - Modify only necessary code
- **Follow DRY/KISS** - Always

**📚 Session History**: See [CHANGELOG.md](CHANGELOG.md) for implementation details.
