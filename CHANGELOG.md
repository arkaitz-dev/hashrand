# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),

## [API v1.8.10] - 2025-10-14

### Fixed

**🐛 TEST: Fix email dry-run persistence and hash extraction for query param URLs**

**Problem**:
- Email dry-run mode wasn't persisting between requests in Spin/WASM environment
- Tests were sending real emails instead of using dry-run mode
- Database missing v3 schema column (`encrypted_payload` in tracking table)
- Hash extraction failing with new query param URL format (`?shared=HASH` vs `/shared-secret/HASH`)
- Tests failing with 404 errors due to incorrect hash extraction

**Root Causes**:

1. **Email Dry-Run Persistence Failure**:
   - `static AtomicBool` doesn't work reliably in Spin/WebAssembly
   - Each request may execute in an isolated WASM context
   - State was lost between `/api/test/dry-run` activation and subsequent requests

2. **Hash Extraction Incompatibility**:
   - Recent UX improvement changed URL format from `/shared-secret/HASH` to `?shared=HASH`
   - Tests still using `.split('/').pop()` which extracted `?shared=HASH` instead of `HASH`
   - Backend expects just the hash, not the query param format

3. **Database Schema Mismatch**:
   - Old database didn't have v3 `encrypted_payload` column in tracking table
   - Needed recreation to apply v3 schema changes

**Solution**:

**1. Email Dry-Run Migration to Spin KV Store** (`api/src/utils/email.rs`):

Replaced `AtomicBool` with Spin KV Store for guaranteed state persistence:

```rust
// OLD (broken):
#[cfg(feature = "dev-mode")]
static EMAIL_DRY_RUN: AtomicBool = AtomicBool::new(false);

// NEW (reliable):
#[cfg(feature = "dev-mode")]
const DRY_RUN_KV_KEY: &str = "email_dry_run_mode";

pub fn set_email_dry_run(enabled: bool) {
    let store = Store::open_default()?;
    let value = if enabled { vec![1u8] } else { vec![0u8] };
    store.set(DRY_RUN_KV_KEY, &value)?;
}

fn is_email_dry_run_enabled() -> bool {
    let store = Store::open_default()?;
    match store.get(DRY_RUN_KV_KEY) {
        Ok(Some(value)) => !value.is_empty() && value[0] == 1u8,
        _ => false,
    }
}
```

**2. Hash Extraction Fix**:

**TypeScript/Playwright** (`web/tests/api/shared-secret.spec.ts`):
```typescript
// OLD (broken - extracts "?shared=HASH"):
const senderHash = createData.payload.url_sender.split('/').pop();

// NEW (correct - extracts "HASH"):
const senderHash = new URL(createData.payload.url_sender).searchParams.get('shared');
```

**Bash Tests** (`scripts/final_test.sh`):
```bash
# OLD (broken):
sender_hash="${SENDER_URL##*/}"

# NEW (correct):
sender_hash=$(echo "$SENDER_URL" | sed 's/.*shared=\([^&]*\).*/\1/')
```

**3. Spin Configuration Updates** (`spin-dev.toml`, `spin-prod.toml`):

Added KV Store configuration required for dry-run state persistence:
```toml
[component.hashrand]
key_value_stores = ["default"]  # Required for email dry-run state
```

**Files Modified**:
- `api/src/utils/email.rs` - KV Store implementation (89 lines changed)
- `spin-dev.toml` - Added KV Store config (+1 line)
- `spin-prod.toml` - Added KV Store config (+1 line)
- `web/tests/api/shared-secret.spec.ts` - Fixed 7 hash extractions (14 lines)
- `scripts/final_test.sh` - Fixed 6 hash extractions (16 lines)

**Database Schema Note**:
- v3 schema already defined in `api/src/database/connection.rs` line 74
- Old database deleted to force recreation with `encrypted_payload` column
- No schema changes in this commit (v3 was already in codebase)

**Test Results**:
```
✅ Total: 43/43 (100%)
✅ Bash: 35/35
✅ Playwright: 8/8
✅ Email dry-run: Confirmed working
✅ All shared secret operations: create, retrieve, delete, OTP, cross-user validation
```

**Benefits**:
- ✅ Email dry-run now persists reliably across all requests
- ✅ Tests no longer send real emails (respects Mailtrap quota)
- ✅ Hash extraction compatible with query param URLs
- ✅ Database v3 schema working correctly
- ✅ 100% test pass rate restored

---

**📚 Architecture Documentation: v3 Two-Layer Encryption System**

The v3 architecture (already implemented in codebase, predates this commit) uses a sophisticated two-layer encryption system for shared secrets:

**Layer 1: Key Material Encryption (Per-User Access Control)**
- `random_key_material[44 bytes]` = nonce[12] + cipher_key[32]
- Encrypted with ChaCha20 using db_index-derived key
- Stored separately for sender and receiver in `shared_secrets` table
- Each user has their own `encrypted_key_material` (different ciphertext, same plaintext)

**Layer 2: Payload Encryption (Centralized Storage)**
- Shared payload encrypted ONCE with `random_key_material`
- Uses ChaCha20-Poly1305 AEAD for integrity
- Stored in `shared_secrets_tracking.encrypted_payload`
- Both sender and receiver decrypt Layer 1 to get `random_key_material`, then decrypt Layer 2

**Decryption Flow**:
1. User provides db_index (derived from email + reference_hash + role)
2. Retrieve `encrypted_key_material` from `shared_secrets` table
3. Decrypt Layer 1: `db_index` → `random_key_material`
4. Retrieve `encrypted_payload` from `shared_secrets_tracking` table
5. Decrypt Layer 2: `random_key_material` → `plaintext_payload`

**Benefits**:
- ✅ Storage efficiency: Payload stored once instead of duplicated
- ✅ Cryptographic isolation: Each user has unique encrypted key material
- ✅ Access control: User must have valid db_index to decrypt
- ✅ Integrity: ChaCha20-Poly1305 AEAD ensures tampering detection

**Implementation Files**:
- `api/src/database/operations/shared_secret_crypto.rs` - Crypto primitives (encrypt/decrypt_key_material_v3)
- `api/src/database/operations/shared_secret_ops.rs` - High-level operations (create/read with v3)
- `api/src/database/operations/shared_secret_storage.rs` - Database storage (store/retrieve_encrypted_payload)
- `api/src/database/connection.rs` line 74 - Schema with `encrypted_payload BLOB NOT NULL`

## [Web v0.28.0] - 2025-10-08

### Added

**📊 TRACKING: Comprehensive user interaction logging across entire frontend**

**Problem**:
- Difficult to debug user flows without visibility into actions taken
- No tracking of route navigation, button clicks, form submissions
- Tablet development challenging without DevTools (need terminal logs)
- When errors occur, unclear what user did before the error
- Missing operational visibility for understanding user behavior

**Solution - Systematic Operational Logging**:

Added 46 `logger.info()` calls across 17 files covering ALL user interactions:

**1. Route Loading Logs (9 routes)**:
- Format: `[Route] {RouteName} page loaded`
- Every page logs when it loads via onMount
- Routes covered:
  - Home (`/`)
  - Custom Hash (`/custom`)
  - Password (`/password`)
  - API Key (`/api-key`)
  - Mnemonic (`/mnemonic`)
  - Result (`/result`)
  - Shared Secret creation (`/shared-secret`)
  - Shared Secret view (`/shared-secret/[hash]`)
  - Logout confirmation (`/logout`)

**2. Button/Link Clicks (14 logs)**:
- Format: `[Click] {Description of action}`
- ALL interactive elements tracked:
  - Menu cards (home page navigation)
  - Back to menu button
  - Back button (generic)
  - Auth status button + dropdown menu
  - Copy to clipboard button
  - Regenerate hash button
  - Adjust settings button
  - Logout confirmation/cancel buttons

**3. Form Submissions (8 logs)**:
- Format: `[Form] Submitting {form name}`
- ALL forms tracked:
  - Custom hash generation
  - Password generation
  - API key generation
  - Mnemonic generation
  - Shared secret creation
  - OTP submission (view secret)
  - Login email submission
  - Magic link confirmation

**4. Navigation/Redirects (8 logs)**:
- Format: `[Navigation] Redirecting to: {path}`
- Logged BEFORE every `goto()` call:
  - Menu card navigation
  - Back button navigation
  - Form submission redirects to `/result`
  - Logout redirects
  - Session expiry redirects

**5. Dialog Interactions (10 logs)**:
- Format: `[Dialog] {Action} {dialog name}`
- ALL dialog open/close/actions tracked:
  - Login dialog (session expired / no tokens)
  - Logout confirmation dialog
  - Auth confirmation dialog (magic link sent)
  - Seed reuse dialog
  - Dialog close events
  - User choices within dialogs

**Log Format Categories**:
```
[Route]      → Page loads (onMount)
[Click]      → User clicks buttons/links
[Form]       → Form submissions
[Navigation] → goto() redirects
[Dialog]     → Dialog interactions
```

**Files Modified** (17 total):

*Routes* (9 files):
- `routes/+page.svelte`
- `routes/custom/+page.svelte`
- `routes/password/+page.svelte`
- `routes/api-key/+page.svelte`
- `routes/mnemonic/+page.svelte`
- `routes/result/+page.svelte`
- `routes/shared-secret/+page.svelte`
- `routes/shared-secret/[hash]/+page.svelte`
- `routes/logout/+page.svelte`

*Components* (7 files):
- `lib/components/MenuCard.svelte`
- `lib/components/BackToMenuButton.svelte`
- `lib/components/BackButton.svelte`
- `lib/components/AuthStatusButton.svelte`
- `lib/components/AuthDialogContent.svelte`
- `lib/components/AuthConfirmDialogContent.svelte`
- `lib/components/LogoutDialogContent.svelte`

*Composables* (1 file):
- `lib/composables/useGenerationWorkflow.ts` - Universal form handler

**Benefits**:

✅ **Complete user journey tracking** - Every page load, click, form submission visible
✅ **Pre-error context** - Know exactly what user did before error occurred
✅ **Navigation flow visibility** - See full navigation path through app
✅ **Dialog interaction tracking** - Understand user choices in dialogs
✅ **Tablet debugging** - Logs visible in terminal via WebSocket redirection
✅ **Production safe** - All logs eliminated by terser in production builds
✅ **Systematic coverage** - Zero gaps in user interaction tracking
✅ **Operational level** - Uses info! (not debug), visible in normal development

**Usage**:

Development with full tracking:
```bash
just dev    # All operational logs visible (info level)
```

Example log sequence (user generates password):
```
[Route] Home page loaded
[Click] Menu card: Password (/password)
[Navigation] Redirecting to: /password
[Route] Password page loaded
[Form] Submitting password generation form
[Navigation] Redirecting to: /result?p=...
[Route] Result page loaded
[Click] Copy result to clipboard
[Click] Back to menu button
[Navigation] Redirecting to: /
[Route] Home page loaded
```

**Version Bump Rationale**:
- Minor version bump (0.27 → 0.28) due to significant observability enhancement
- 46 new logging points across entire frontend
- No breaking changes, purely additive observability feature
