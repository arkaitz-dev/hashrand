# Ed25519 Key Rotation Testing Guide

## Overview
This guide explains how to manually verify the 2/3 time window key rotation system for Ed25519 cryptographic keys.

## Test Configuration (Development Environment)

- **Access Token Duration**: 20 seconds
- **Refresh Token Duration**: 120 seconds (2 minutes)
- **TRAMO 1/3 Window**: 0-40 seconds (no rotation)
- **TRAMO 2/3 Window**: 40-120 seconds (full rotation)

## Testing Prerequisites

1. **Environment Running**: `just dev` command executed
2. **Browser Access**: http://localhost:5173
3. **Log Monitoring**: Terminal with `tail -f .spin-dev.log`

## Test 1: Verify NO Rotation in TRAMO 1/3 (0-40s)

### Expected Behavior
- Access token expires after 20s
- Automatic refresh occurs
- Backend detects TRAMO 1/3 (time_remaining > 40s)
- Backend uses OLD pub_key
- Backend emits NO server_pub_key in response
- Frontend does NOT rotate keys

### Testing Steps

1. **Initial Login** (time = 0s)
   ```
   Open http://localhost:5173
   Click "Login" button
   Enter email (me@arkaitz.dev)
   Check email for magic link
   Click magic link
   ```

2. **Wait for First Refresh** (time ≈ 22-25s)
   - Access token expires at 20s
   - Auto-refresh will trigger shortly after
   - Watch for flash messages in UI (visible even on tablet)
   - Monitor backend logs

3. **Expected Flash Messages** (Frontend)
   ```
   🔄 Iniciando renovación de token...
   🔑 Nuevo keypair generado para rotación
   📤 Enviando request a /api/refresh...
   📥 Respuesta recibida del servidor
   ⏭️ Token renovado sin rotación (1/3)     ← KEY MESSAGE
   ✅ Token renovado exitosamente
   ```

4. **Expected Backend Logs**
   ```
   🔍 Refresh: Attempting to validate refresh token...
   ✅ Refresh: Token validation successful
   ⏱️ Refresh: Expires at: [timestamp], Now: [timestamp]
   📊 Refresh: Time remaining: 95s, 2/3 threshold: 80s   ← Remaining > 40s
   🎯 Refresh: Decision -> TRAMO 1/3 (NO ROTATION)      ← KEY LOG
   ⏸️ Refresh: ===== TRAMO 1/3: NO KEY ROTATION =====
   🔑 Refresh: Using OLD pub_key: [hex]...
   ✅ Refresh: Access token created with OLD pub_key
   🔐 Refresh: Generating SignedResponse WITHOUT server_pub_key
   ✅ Refresh: Token refresh completed (no rotation)
   ```

5. **Verification Checklist**
   - [ ] Flash message shows "⏭️ Token renovado sin rotación (1/3)"
   - [ ] Backend logs show "TRAMO 1/3 (NO ROTATION)"
   - [ ] Time remaining in logs > 40 seconds
   - [ ] Backend uses "OLD pub_key"
   - [ ] No "server_pub_key" in SignedResponse
   - [ ] Frontend console shows "===== TRAMO 1/3: NO KEY ROTATION ====="

### ✅ Success Criteria
If all checklist items are true, **TRAMO 1/3 test PASSES** - no key rotation occurred as expected.

---

## Test 2: Verify Rotation DOES Occur in TRAMO 2/3 (40-120s)

### Expected Behavior
- Refresh token has < 40s remaining
- Backend detects TRAMO 2/3 (time_remaining < 80s)
- Backend uses NEW pub_key for tokens
- Backend emits server_pub_key in response
- Frontend rotates both client priv_key and server_pub_key

### Testing Steps

1. **Continue from Previous Test** (time ≈ 25s)
   - You are already logged in
   - Refresh cookie still valid (120s total)

2. **Wait for Second Refresh** (time ≈ 45-48s)
   - Another access token expiry at ~42s
   - Auto-refresh will trigger in TRAMO 2/3 window
   - Watch for different flash messages
   - Monitor backend logs for rotation

3. **Expected Flash Messages** (Frontend)
   ```
   🔄 Iniciando renovación de token...
   🔑 Nuevo keypair generado para rotación
   📤 Enviando request a /api/refresh...
   📥 Respuesta recibida del servidor
   🔄 TRAMO 2/3: Iniciando rotación de claves...      ← KEY MESSAGE
   ✅ Rotación de claves completada (2/3)           ← KEY MESSAGE
   ✅ Token renovado exitosamente
   ```

4. **Expected Backend Logs**
   ```
   🔍 Refresh: Attempting to validate refresh token...
   ✅ Refresh: Token validation successful
   ⏱️ Refresh: Expires at: [timestamp], Now: [timestamp]
   📊 Refresh: Time remaining: 72s, 2/3 threshold: 80s   ← Remaining < 80s
   🎯 Refresh: Decision -> TRAMO 2/3 (KEY ROTATION)     ← KEY LOG
   🔄 Refresh: ===== TRAMO 2/3: KEY ROTATION =====
   🔑 Refresh: NEW pub_key: [hex]...
   ✅ Refresh: Access token created with NEW pub_key
   ✅ Refresh: Refresh token created with NEW pub_key
   🔐 Refresh: Generating SignedResponse WITH server_pub_key
   🎉 Refresh: Key rotation completed successfully
   ```

5. **Verification Checklist**
   - [ ] Flash message shows "🔄 TRAMO 2/3: Iniciando rotación de claves..."
   - [ ] Flash message shows "✅ Rotación de claves completada (2/3)"
   - [ ] Backend logs show "TRAMO 2/3 (KEY ROTATION)"
   - [ ] Time remaining in logs < 80 seconds
   - [ ] Backend uses "NEW pub_key"
   - [ ] Backend includes "server_pub_key" in SignedResponse
   - [ ] Frontend console shows "===== TRAMO 2/3: KEY ROTATION ====="
   - [ ] Frontend console shows both priv_key and server_pub_key rotated

### ✅ Success Criteria
If all checklist items are true, **TRAMO 2/3 test PASSES** - full key rotation occurred as expected.

---

## Monitoring Commands

### Backend Logs (Terminal)
```bash
# Real-time monitoring
tail -f .spin-dev.log | grep -E "Refresh:|TRAMO"

# Filter for refresh flow only
tail -f .spin-dev.log | grep "🔄"
```

### Frontend Console (Browser DevTools - if available)
```javascript
// Filter refresh logs
console.log('[REFRESH]')
```

### Alternative Monitoring (Tablet without DevTools)
- **Flash Messages**: Visible at top of screen
- **Flash Messages Store**: Persists last 10 messages
- **Color Coding**:
  - 🔄 Blue - Info/Progress
  - ✅ Green - Success
  - ⏭️ Yellow - TRAMO 1/3 (no rotation)
  - 🔄 Purple - TRAMO 2/3 (rotation)
  - ❌ Red - Error

---

## Timing Reference

| Time | Event | Expected Behavior |
|------|-------|-------------------|
| 0s   | Initial Login | Magic link authentication |
| 20s  | First Access Token Expiry | Auto-refresh triggered |
| 22s  | First Refresh Request | TRAMO 1/3 - NO rotation |
| 40s  | Second Access Token Expiry | Auto-refresh triggered |
| 42s  | Second Refresh Request | TRAMO 2/3 - FULL rotation |
| 60s  | Third Access Token Expiry | Auto-refresh triggered |
| 62s  | Third Refresh Request | TRAMO 2/3 - FULL rotation |
| 120s | Refresh Token Expiry | Session ends, new login required |

---

## Troubleshooting

### No Flash Messages Appearing
- Check `flashMessagesStore` is imported in UI
- Verify messages component is rendered
- Check browser console for JavaScript errors

### Backend Logs Not Showing
- Verify `just dev` is running
- Check `.spin-dev.log` file exists
- Try `tail -100 .spin-dev.log` for recent logs

### Auto-Refresh Not Triggering
- Check `sessionExpiry.svelte` is loaded
- Verify token expiration values in JWT
- Check browser tab is active (timers may pause in background)

### Wrong TRAMO Decision
- Verify `SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES` in .env
- Check system clock is accurate
- Review calculation logs in backend

---

## Test Automation

### Automated Test Script

**Production Script**: `scripts/test_2_3_system.sh`

Complete automated test of the 2/3 key rotation system with Ed25519 cryptographic verification:

```bash
# Run complete automated test (takes ~7 minutes)
timeout 480 ./scripts/test_2_3_system.sh
```

#### Test Coverage

The automated script performs 4 comprehensive tests:

1. **Test 1 (t=0s)**: Initial API call with valid token
2. **Test 2 (t=62s)**: Partial refresh (TRAMO 1/3) - no rotation
3. **Test 3 (t=110s)**: Full key rotation (TRAMO 2/3)
4. **Test 4 (t=430s)**: Dual token expiration (401 expected)

#### Implementation Details

**Key Rotation Sequence** (Test 3):
```bash
# 1. Preserve OLD private key before generating NEW keypair
cp .test-ed25519-private-key .test-ed25519-private-key.old

# 2. Generate NEW Ed25519 keypair
NEW_PUB_KEY=$(node scripts/generate_hash.js)

# 3. Save NEW private key for later
cp .test-ed25519-private-key .test-ed25519-private-key.new

# 4. Restore OLD private key for signing
cp .test-ed25519-private-key.old .test-ed25519-private-key

# 5. Sign refresh request with OLD key (contains NEW pub_key in payload)
REFRESH_SIGNED_REQUEST=$(node scripts/create_signed_request.js "$PAYLOAD")

# 6. After successful rotation, switch to NEW private key
cp .test-ed25519-private-key.new .test-ed25519-private-key
```

**Why This Sequence is Critical**:
- Request MUST be signed with OLD private key (backend validates with OLD pub_key from refresh token)
- Payload contains NEW pub_key for backend to use in new tokens
- Only after successful rotation does client switch to NEW private key

#### Test Results

**100% success rate after v1.6.23 bug fix**:

```bash
🏆 RESUMEN: Sistema 2/3 funciona PERFECTAMENTE
✅ Test 1: Token válido
✅ Test 2: Refresh parcial (primer 1/3)
✅ Test 3: KEY ROTATION (sistema 2/3)
✅ Test 4: Doble expiración 401
```

#### Requirements

- **Backend running**: `just dev` must be active
- **Dependencies**: `node`, `curl`, `jq`, `bash`
- **Test duration**: ~7 minutes (includes wait times for token expiration)
- **Test files**: Auto-cleaned after completion

**For detailed manual testing procedures**, continue reading the sections below.

---

## Security Validation

After successful testing, verify:
1. Old keys are destroyed after rotation
2. New SignedRequests use rotated keys
3. Backend rejects signatures with old keys after rotation
4. No key material logged in production mode
5. IndexedDB properly stores rotated keys

---

**Document Version**: 1.1.0
**Last Updated**: 2025-09-30
**Related**: CHANGELOG.md v1.6.23, README.md Key Rotation section, [Testing Guide](./testing.md)