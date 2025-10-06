# E2E and API Testing for HashRand

Complete testing suite for HashRand frontend using Playwright.

## 📊 Test Suite Overview

### ✅ API-Only Tests (16 tests) - **100% Passing**

Tests that work **without browser dependencies** - perfect for Arch Linux and CI/CD environments.

**Location**: `tests/api/`

#### Authentication Tests (4 tests)

File: `tests/api/auth-api.spec.ts`

- ✅ Request magic link with Ed25519 signature
- ✅ Reject unsigned request
- ✅ Reject invalid Ed25519 signature
- ✅ Handle multiple magic link requests

#### Full Authentication Flow Tests (2 tests)

File: `tests/api/auth-full-flow.spec.ts`

- ✅ Complete full authentication flow with magic link extraction from backend logs
- ✅ Extract multiple magic links correctly (validates uniqueness)

#### Cryptographic Validation Tests (10 tests)

File: `tests/api/crypto-validation.spec.ts`

**Ed25519 Operations (3 tests)**:

- ✅ Generate Ed25519 keypair
- ✅ Sign and verify messages
- ✅ Convert keypair to/from hex

**SignedRequest Creation (3 tests)**:

- ✅ Create SignedRequest with deterministic serialization
- ✅ Create identical signatures for same payload
- ✅ Sign query parameters

**Base64 and JSON (3 tests)**:

- ✅ Encode and decode Base64 URL-safe
- ✅ Sort object keys recursively
- ✅ Serialize payload deterministically

**TestSessionManager (1 test)**:

- ✅ Manage session state in memory

### ⚠️ E2E Tests (21 tests) - Require Browser

Full end-to-end tests with browser automation. **Requires system dependencies** (Chromium, etc.)

**Location**: `tests/e2e/`

**Files**:

- `auth-flow.spec.ts` (3 tests) - Magic link authentication flow
- `hash-generation.spec.ts` (12 tests) - All generation endpoints
- `token-refresh.spec.ts` (3 tests) - Token refresh system (~25s wait)
- `key-rotation.spec.ts` (3 tests) - Ed25519 key rotation (~110s wait)

**Note**: E2E tests require browser dependencies not available on all systems (e.g., Arch Linux).

## 🚀 Running Tests

### API-Only Tests (Recommended for Arch Linux)

```bash
# Run all API tests (fast, no browser required)
npm run test:api

# Verbose output with logs
npm run test:api:verbose

# Alternative: Direct Playwright command
npx playwright test api/
```

### E2E Tests (Requires Browser Dependencies)

```bash
# Run all E2E tests
npm run test:e2e

# Run with UI
npm run test:e2e:ui

# Debug mode
npm run test:e2e:debug
```

### All Tests

```bash
# Run everything (API + E2E)
npm run test:all

# View HTML report
npm run test:report
```

## 📋 Test Results Example

```bash
$ npm run test:api:verbose

Running 16 tests using 1 worker

🧪 TEST: Request magic link (API only)
============================================================
🔑 Generated keypair: ecf4ca5e93eac02ba122...
📤 Sending signed request to /api/login/
✅ Received SignedResponse
🔐 Server pub_key: 874f7e31cf73ef735759...
✅ Server signature verified
✅ Status: OK
📧 Magic link sent to email (Mailtrap)
🎉 TEST PASSED: Magic link request successful
============================================================

  ✓  1 Request magic link with Ed25519 signature (959ms)
  ✓  2 Reject unsigned request (18ms)
  ✓  3 Reject invalid Ed25519 signature (12ms)
  ✓  4 Handle multiple magic link requests (904ms)

🧪 TEST: Full authentication flow (API + log extraction)
============================================================
✅ Backend logs cleared
🔑 Generated keypair: dbcb077756cf8a4d45d8...
📤 Sending signed request to /api/login/
🔐 Server pub_key: 30ce94bdf5a59dffe26e...

📧 Extracting magic link from backend logs...
📋 Magic link line: DEBUG: Generated magic_link = localhost/?magiclink=6iF8CtA2J44H3q8CBDp2f3MGHeyB2hKXBrxd3tPWiWLQ
✅ Magic token extracted: 6iF8CtA2J44H3q8CBDp2...
🔗 Magic link URL: http://localhost:5173/?magiclink=6iF8CtA2J44H3q8CBDp2f3MGHey...
✅ Magic link token validated from backend logs

🎉 TEST PASSED: Full authentication flow complete
============================================================

  ✓  5 Full authentication flow with magic link extraction (2.3s)
  ✓  6 Extract multiple magic links correctly (10.0s)
  ...

  16 passed (15.5s)
```

## 🔧 System Requirements

### For API Tests (Minimal)

- ✅ Node.js 18+
- ✅ npm packages installed
- ✅ Backend running (`just dev`)

### For E2E Tests (Full)

- ✅ Node.js 18+
- ✅ npm packages installed
- ✅ Backend + Frontend running
- ✅ System dependencies:
  - **Debian/Ubuntu**: `sudo npx playwright install-deps`
  - **Arch Linux**: `sudo pacman -S nss atk cups libdrm mesa libxkbcommon libxrandr libxcomposite libxdamage libxfixes pango cairo alsa-lib`
  - **Docker**: Use `mcr.microsoft.com/playwright:v1.55.1-jammy`

## 📁 Test Structure

```
tests/
├── api/                    # API-only tests (no browser)
│   ├── auth-api.spec.ts    # Authentication endpoints (4 tests)
│   ├── auth-full-flow.spec.ts  # Full auth flow with magic link extraction (2 tests)
│   └── crypto-validation.spec.ts  # Cryptographic functions (10 tests)
├── e2e/                    # Full E2E tests (browser required)
│   ├── auth-flow.spec.ts
│   ├── hash-generation.spec.ts
│   ├── token-refresh.spec.ts
│   └── key-rotation.spec.ts
└── utils/                  # Shared test utilities
    ├── test-session-manager.ts    # In-memory session management
    └── test-auth-helpers.ts        # Authentication helpers
```

## 🎯 What Tests Validate

### Core Functionality

- ✅ Ed25519 keypair generation (Noble curves)
- ✅ Message signing and verification
- ✅ SignedRequest creation with deterministic serialization
- ✅ SignedResponse validation
- ✅ Base64 URL-safe encoding/decoding
- ✅ Recursive object key sorting

### API Endpoints

- ✅ `/api/login/` - Magic link request
- ✅ Signature validation (reject unsigned/invalid requests)
- ✅ Magic link extraction from backend logs (matches bash test pattern)
- ✅ Full authentication flow validation (without browser)
- ⚠️ `/api/custom` - Requires full auth flow (E2E only)
- ⚠️ `/api/password` - Requires full auth flow (E2E only)
- ⚠️ `/api/api-key` - Requires full auth flow (E2E only)
- ⚠️ `/api/mnemonic` - Requires full auth flow (E2E only)

### Security

- ✅ Ed25519 signature creation and verification
- ✅ Invalid signature rejection
- ✅ Unsigned request rejection
- ✅ Deterministic JSON serialization (prevents replay attacks)

## 🏗️ Architecture

Tests follow **SOLID/DRY/KISS principles**:

- **Pure Core, Impure Shell**: Cryptographic logic separated from browser APIs
- **Dependency Injection**: TestSessionManager replaces browser IndexedDB
- **Universal Modules**: Core modules work in Node.js and browser
- **Zero Duplication**: Reuses production frontend code

## 🐛 Troubleshooting

### Tests Won't Run

**Error**: `Cannot find module '@playwright/test'`

**Solution**: Make sure you're in the `/web` directory:

```bash
cd /home/arkaitz/proyectos/spin/hashrand/web
npm run test:api
```

### Browser Not Found (E2E Tests)

**Error**: `Host system is missing dependencies to run browsers`

**Solution**:

1. Install Chromium: `npm run test:install`
2. Install system deps (see System Requirements above)
3. OR use API-only tests: `npm run test:api`

### Backend Not Running

**Error**: Connection refused on port 3000

**Solution**:

```bash
# Start dev servers
just dev

# Check status
just status
```

## 📝 Writing New Tests

### API Test Template

```typescript
import { test, expect } from '@playwright/test';
import { TestSessionManager } from '../utils/test-session-manager';
import {
	createSignedRequestWithKeyPair,
	signQueryParamsWithKeyPair
} from '../../src/lib/crypto/signedRequest-core';

test('should validate something', async ({ request }) => {
	const session = new TestSessionManager();
	const keyPair = await session.generateKeyPair();

	// IMPORTANT: Only use authorized emails
	// Allowed: me@arkaitz.dev, arkaitzmugica@protonmail.com, arkaitzmugica@gmail.com
	const payload = {
		email: 'me@arkaitz.dev'
		// ... rest of payload
	};

	await session.clear();
});
```

### E2E Test Template

```typescript
import { test, expect } from '../utils/test-auth-helpers';
import { requestMagicLink, loginWithMagicLink } from '../utils/test-auth-helpers';

test('should do something with authentication', async ({ page, request, session }) => {
	// IMPORTANT: Only use authorized emails
	// Allowed: me@arkaitz.dev, arkaitzmugica@protonmail.com, arkaitzmugica@gmail.com
	const magicLink = await requestMagicLink(request, session, 'me@arkaitz.dev');
	await loginWithMagicLink(page, session, magicLink);

	// Your test logic here
});
```

## 🎯 Next Steps

1. **Run API tests** to validate current implementation
2. **Fix any failing tests** before deployment
3. **Add to CI/CD pipeline** (use API tests for fast validation)
4. **Consider E2E tests** for comprehensive validation (requires browser setup)

## 📚 Related Documentation

- [E2E Testing Implementation Plan](../../docs/E2E_TESTING_IMPLEMENTATION_PLAN.md)
- [Playwright Configuration](../playwright.config.ts)
- [Testing Guide](../../docs/guides/testing.md)
