# Shared Secret Implementation - TO-DO List

**Status**: 🚧 In Progress
**Version Target**: API v1.8.0 + Web v0.25.0
**Created**: 2025-10-04

---

## 🎯 Implementation Principles

### CRITICAL RULES
- ✅ **ALL API calls MUST be signed** (Ed25519) - NO exceptions for shared secrets
- ✅ **Maximum UX/Visual coherence** with existing application
- ✅ **DO NOT touch existing code** outside shared secrets functionality
- ✅ **Independent visualization** - NOT reusing `/result/...` (that's for future)
- ✅ **DRY/SOLID/KISS** - Modules <225 lines
- ✅ **Use Ultrathink** throughout entire process

---

## 📋 Implementation Checklist

### **BACKEND** (Rust + Spin + SQLite)

#### ✅ Database (`api/src/database/connection.rs`)
- [ ] Modify `initialize_database()` to add two new tables:
  - [ ] `shared_secrets` table (id, encrypted_payload, expires_at, pending_reads, role)
  - [ ] `shared_secrets_tracking` table (reference_hash, read_at, expires_at, created_at)

#### ✅ Database Operations (`api/src/database/operations/`)
- [ ] Create `shared_secret_types.rs`
  - [ ] Constants: REFERENCE_HASH_LENGTH=16, MAX_TEXT_LENGTH=512, OTP_LENGTH=9
  - [ ] Types: SharedSecret, TrackingRecord, SecretRole enum
- [ ] Create `shared_secret_crypto.rs`
  - [ ] Function: `generate_reference_hash()` → [u8;16] random
  - [ ] Function: `generate_otp()` → String (9 digits)
  - [ ] Function: `encrypt_payload()` → ChaCha20 encryption
  - [ ] Function: `decrypt_payload()` → ChaCha20 decryption
  - [ ] Function: `create_encrypted_id()` → Similar to magic_link hash
- [ ] Create `shared_secret_storage.rs`
  - [ ] Function: `store_shared_secret()` → Insert into shared_secrets
  - [ ] Function: `retrieve_secret()` → SELECT + decrypt
  - [ ] Function: `delete_secret()` → DELETE by id
  - [ ] Function: `decrement_pending_reads()` → UPDATE pending_reads
  - [ ] Function: `store_tracking()` → Insert into shared_secrets_tracking
  - [ ] Function: `update_tracking_read()` → UPDATE read_at
  - [ ] Function: `cleanup_expired()` → DELETE expired secrets
- [ ] Create `shared_secret_ops.rs`
  - [ ] Function: `create_secret_pair()` → Creates 2 entries (sender + receiver)
  - [ ] Function: `read_secret()` → Validates, decrypts, decrements
  - [ ] Function: `validate_otp()` → Compares OTP
  - [ ] Function: `confirm_read()` → Updates tracking
- [ ] Update `mod.rs` with re-exports

#### ✅ Handlers (`api/src/handlers/shared_secret/`)
- [ ] Create `creation.rs`
  - [ ] Handler: `POST /api/shared-secret/create` (JWT + Ed25519)
  - [ ] Validate: receiver_email, secret_text (max 512 UTF-8), expires_hours (1-72), max_reads (1-10)
  - [ ] Generate: reference_hash, OTP (optional), 2 encrypted IDs
  - [ ] Store: 2 rows in shared_secrets, 1 row in tracking
  - [ ] Send: emails (always receiver, optional sender)
  - [ ] Return: url_sender, url_receiver, otp, reference
- [ ] Create `retrieval.rs`
  - [ ] Handler: `GET /api/shared-secret/{hash}` (JWT + Ed25519)
  - [ ] Validate: hash, user_id match, expiration, signature
  - [ ] Check: OTP required? → Return 400 OTP_REQUIRED
  - [ ] Decrypt: payload
  - [ ] Decrement: pending_reads (only receiver and if >0)
  - [ ] Auto-delete: if pending_reads reaches 0
  - [ ] Return: secret data
  - [ ] Handler: `POST /api/shared-secret/{hash}` (JWT + Ed25519 + OTP)
  - [ ] Same as GET but validates OTP first
  - [ ] If OTP wrong → 401 (NO decrement)
- [ ] Create `deletion.rs`
  - [ ] Handler: `DELETE /api/shared-secret/{hash}` (JWT + Ed25519)
  - [ ] Validate: hash, user_id, signature
  - [ ] Check: pending_reads > 0 (don't allow delete if consumed)
  - [ ] Delete: specific row (sender or receiver)
- [ ] Create `tracking.rs`
  - [ ] Handler: `GET /api/shared-secret/confirm-read?hash={hash}&signature={sig}` (JWT + Ed25519)
  - [ ] Validate: hash, user_id match, signature
  - [ ] Extract: reference_hash from encrypted_payload
  - [ ] Update: shared_secrets_tracking SET read_at=NOW() WHERE reference_hash AND read_at IS NULL
  - [ ] Idempotent: ignore if already set
- [ ] Create `routing.rs`
  - [ ] Route dispatcher for all shared_secret endpoints
- [ ] Create `mod.rs` with re-exports
- [ ] Update `api/src/handlers/mod.rs` to include shared_secret module

#### ✅ Email Templates (`api/src/utils/email/`)
- [ ] Create `shared_secret_receiver.html` (13 languages)
  - [ ] Subject: "Secreto Compartido #{reference_base58}"
  - [ ] Body: Link + sender info + expiration
- [ ] Create `shared_secret_sender.html` (13 languages)
  - [ ] Subject: "Copia: Secreto #{reference_base58}"
  - [ ] Body: Link + receiver info + confirmation
- [ ] Update email dispatcher to handle new templates

---

### **FRONTEND** (SvelteKit + TypeScript + TailwindCSS)

#### ✅ Navigation (`web/src/lib/stores/navigation.ts`)
- [ ] Add new navigation item:
  ```typescript
  {
    id: 'shared-secret',
    title: 'Shared Secret',
    description: 'Share text securely with encryption',
    path: '/shared-secret',
    icon: '🔒'
  }
  ```

#### ✅ Creation Route (`web/src/routes/shared-secret/`)
- [ ] Create `+page.ts` with AuthGuard
- [ ] Create `+page.svelte` with form:
  - [ ] Input: receiver_email (email validation)
  - [ ] Textarea: secret_text (max 512 chars UTF-8, character counter, auto-resize)
  - [ ] Slider: expires_hours (1-72h, default 24h, dynamic label)
  - [ ] Slider: max_reads (1-10, default 3, dynamic label)
  - [ ] Checkbox: require_otp
  - [ ] Checkbox: send_copy_to_sender
  - [ ] Button: "Create Shared Secret" (disabled if invalid)
  - [ ] **IMPORTANT**: POST request with Ed25519 signature
- [ ] Create success modal/dialog:
  - [ ] Display: url_receiver (readonly + copy button)
  - [ ] Display: url_sender (readonly + copy button)
  - [ ] Display: OTP (if required, highlighted + copy button)
  - [ ] Display: Reference #ABC123
  - [ ] Button: "Close" → navigate to `/`
- [ ] **UX Coherence**: Match styling of existing forms (custom/password/api-key/mnemonic)

#### ✅ Reading Route (`web/src/routes/shared-secret/[hash]/`)
- [ ] Create `+page.ts` with:
  - [ ] AuthGuard
  - [ ] GET request to `/api/shared-secret/{hash}` with Ed25519 signature
  - [ ] Handle redirects if not authenticated
- [ ] Create `+page.svelte`:
  - [ ] If OTP_REQUIRED → Show dialog with OTP input (9 digits)
  - [ ] POST with OTP + Ed25519 signature
  - [ ] Display secret card:
    - [ ] 🔒 Icon
    - [ ] Secret text (copyable)
    - [ ] Sender email / Receiver email
    - [ ] Pending reads counter
    - [ ] Expiration countdown/timestamp
    - [ ] Reference #ABC123
    - [ ] Button "Delete" (only if pending_reads > 0) with confirmation dialog
    - [ ] Button "Back to Menu"
  - [ ] **Auto-confirmation (only if role==='receiver')**:
    ```typescript
    if (secretData.role === 'receiver') {
      const signature = await generateEd25519Signature(hash);
      await fetch(`/api/shared-secret/confirm-read?hash=${hash}&signature=${signature}`, {
        headers: { Authorization: `Bearer ${accessToken}` }
      });
    }
    ```
- [ ] **UX Coherence**: Match styling of existing result pages

#### ✅ i18n (`web/src/lib/languageConfig.ts`)
- [ ] Add translations for 13 languages:
  - [ ] sharedSecret.title
  - [ ] sharedSecret.description
  - [ ] sharedSecret.receiverEmail
  - [ ] sharedSecret.receiverEmailPlaceholder
  - [ ] sharedSecret.secretText
  - [ ] sharedSecret.secretTextPlaceholder
  - [ ] sharedSecret.expiresHours
  - [ ] sharedSecret.maxReads
  - [ ] sharedSecret.requireOtp
  - [ ] sharedSecret.sendCopy
  - [ ] sharedSecret.create
  - [ ] sharedSecret.creating
  - [ ] sharedSecret.successTitle
  - [ ] sharedSecret.urlReceiver
  - [ ] sharedSecret.urlSender
  - [ ] sharedSecret.otp
  - [ ] sharedSecret.reference
  - [ ] sharedSecret.copy
  - [ ] sharedSecret.copied
  - [ ] sharedSecret.viewSecret
  - [ ] sharedSecret.enterOtp
  - [ ] sharedSecret.otpPlaceholder
  - [ ] sharedSecret.submit
  - [ ] sharedSecret.invalidOtp
  - [ ] sharedSecret.delete
  - [ ] sharedSecret.deleteConfirm
  - [ ] sharedSecret.deleted
  - [ ] sharedSecret.pendingReads
  - [ ] sharedSecret.expiresAt
  - [ ] sharedSecret.expired
  - [ ] sharedSecret.sender
  - [ ] sharedSecret.receiver
  - [ ] sharedSecret.backToMenu
  - [ ] sharedSecret.characterCount

#### ✅ Utilities/Composables
- [ ] Review existing Ed25519 signature generation utilities
- [ ] Ensure all API calls use proper signature generation
- [ ] Verify JWT token handling in all requests

---

### **TESTING**

#### ✅ Backend Tests (`scripts/test_shared_secret.sh`)
- [ ] Test: Create secret without OTP
- [ ] Test: Create secret with OTP
- [ ] Test: Sender reads unlimited times (pending_reads=-1)
- [ ] Test: Receiver reads and decrements pending_reads
- [ ] Test: Receiver with correct OTP
- [ ] Test: Receiver with incorrect OTP (no decrement)
- [ ] Test: Auto-delete when pending_reads reaches 0
- [ ] Test: Manual delete with confirmation
- [ ] Test: Expiration and auto-cleanup
- [ ] Test: Tracking confirmation (read_at update)
- [ ] Test: All endpoints validate Ed25519 signatures

#### ✅ Frontend Tests (`web/tests/shared-secret.spec.ts`)
- [ ] Test: Login → Create secret → Copy URLs
- [ ] Test: Login as receiver → Read secret
- [ ] Test: OTP flow (wrong OTP → correct OTP)
- [ ] Test: Delete with confirmation dialog
- [ ] Test: Expiration message in UI
- [ ] Test: Character counter (512 max)
- [ ] Test: Form validation

---

### **DOCUMENTATION**

#### ✅ CHANGELOG.md
- [ ] Add entry for v1.8.0 (API)
- [ ] Add entry for v0.25.0 (Web)
- [ ] Document architecture:
  - [ ] Database schema
  - [ ] Encryption flow (ChaCha20)
  - [ ] Dual-URL system (sender/receiver)
  - [ ] Reference hash system
  - [ ] Tracking mechanism
  - [ ] OTP validation

#### ✅ CLAUDE.md
- [ ] Update version numbers
- [ ] Add ultra-brief summary of Shared Secret feature
- [ ] Reference this TODO file for detailed implementation tracking

#### ✅ README.md (if needed)
- [ ] Add Shared Secret to features list
- [ ] Document new endpoints

---

## 📊 Progress Tracking

**Overall Progress**: 0/100 tasks completed

**Backend**: 0/XX tasks
**Frontend**: 0/XX tasks
**Testing**: 0/XX tasks
**Documentation**: 0/XX tasks

---

## 🔍 Implementation Notes

### Encryption Flow
```
reference_hash = random([u8;16])
otp = generate_9_digit_number() // if required
payload = sender_email || receiver_email || secret_text || otp || created_at || reference_hash
encrypted_payload = ChaCha20(payload)

// Two encrypted IDs (like magic links)
hash_sender = encrypt(sender_user_id || expires_at || metadata)
hash_receiver = encrypt(receiver_user_id || expires_at || metadata)

// Database
INSERT shared_secrets: (hash_sender, encrypted_payload, expires_at, pending_reads=-1, role='sender')
INSERT shared_secrets: (hash_receiver, encrypted_payload, expires_at, pending_reads=max_reads, role='receiver')
INSERT shared_secrets_tracking: (reference_hash, read_at=NULL, expires_at, created_at)
```

### Signature Validation (ALL endpoints)
```rust
// Every endpoint must validate:
1. JWT token → user_id
2. Ed25519 signature → verify(public_key, message, signature)
3. user_id from JWT === user_id from hash (for GET/POST/DELETE /api/shared-secret/{hash})
```

### UX Consistency
- Follow exact same patterns as existing routes:
  - `/custom`, `/password`, `/api-key`, `/mnemonic` for creation forms
  - `/result/...` for result display patterns (styling only, NOT code reuse)
- Use existing TailwindCSS classes
- Match dark/light mode behavior
- Consistent button styles, input fields, sliders, checkboxes
- Same error/success message patterns

---

## ⚠️ Critical Reminders

1. **NO changes to existing code** outside shared_secret scope
2. **ALL API calls signed** with Ed25519 (no exceptions)
3. **Maximum UX coherence** with existing app
4. **Modules <225 lines** (DRY/SOLID/KISS)
5. **Use Ultrathink** for complex decisions
6. **Test thoroughly** before marking complete

---

**Last Updated**: 2025-10-04
**Next Session**: Continue from first unchecked item
