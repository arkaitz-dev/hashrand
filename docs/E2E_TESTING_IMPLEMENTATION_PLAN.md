# E2E Testing Implementation Plan - Playwright + Frontend Code Reuse

**Date**: 2025-09-30
**Project**: HashRand Spin v1.6.23
**Objective**: Implement E2E tests reusing frontend code following SOLID/DRY/KISS principles

## ✅ IMPLEMENTATION STATUS: COMPLETED (2025-10-01)

**All phases completed successfully + API-only tests added!**

### Phase 1: Universal Core Modules ✅
- ✅ `web/src/lib/ed25519/ed25519-core.ts` (167 lines) - Universal Ed25519 operations
- ✅ `web/src/lib/crypto/signedRequest-core.ts` (166 lines) - Pure SignedRequest logic
- ✅ Updated existing modules to use universal core with backward compatibility
- ✅ Fixed all TypeScript errors and build issues

### Phase 2: Test Utilities ✅
- ✅ `tests/utils/test-session-manager.ts` (140 lines) - In-memory session management
- ✅ `tests/utils/test-auth-helpers.ts` (575 lines) - Complete auth helper functions

### Phase 3: E2E Tests (21 tests total) ✅
- ✅ `tests/e2e/auth-flow.spec.ts` (153 lines, 3 tests) - Magic link authentication
- ✅ `tests/e2e/hash-generation.spec.ts` (302 lines, 12 tests) - All hash endpoints
- ✅ `tests/e2e/token-refresh.spec.ts` (238 lines, 3 tests) - Token refresh (~25s wait)
- ✅ `tests/e2e/key-rotation.spec.ts` (322 lines, 3 tests) - Ed25519 key rotation (~110s wait)

### Phase 4: Configuration & Documentation ✅
- ✅ `web/playwright.config.ts` (146 lines) - Complete Playwright configuration
- ✅ `web/package.json` - Added test:e2e and test:api scripts
- ✅ Playwright installed (@playwright/test v1.55.1)
- ✅ Chromium browser installed

### Phase 5: API-Only Tests (16 tests total) ✅ **NEW (2025-10-01)**
- ✅ `tests/api/auth-api.spec.ts` (226 lines, 4 tests) - Authentication endpoints without browser
- ✅ `tests/api/auth-full-flow.spec.ts` (202 lines, 2 tests) - Full auth flow with magic link extraction
- ✅ `tests/api/crypto-validation.spec.ts` (346 lines, 10 tests) - Cryptographic functions validation
- ✅ `tests/README.md` - Complete test suite documentation

**API Tests Key Features**:
- ✅ **No browser dependencies** - Perfect for Arch Linux and CI/CD environments
- ✅ **Magic link extraction** - Reads backend logs (`.spin-dev.log`) matching bash test pattern
- ✅ **Ed25519 validation** - Full cryptographic signature verification using @noble/curves
- ✅ **Universal modules** - Reuses production frontend code (SOLID/DRY/KISS)
- ✅ **100% success rate** - All 16 tests passing consistently
- ✅ **Real timestamps** - Uses `Math.floor(Date.now() / 1000)` for realistic validation
- ✅ **Authorized emails** - Only `me@arkaitz.dev`, `arkaitzmugica@protonmail.com`, `arkaitzmugica@gmail.com`

### Implementation Statistics
- **Total Lines Added**: 2,504 (tests) + 479 (core modules + config) = **2,983 lines**
- **Files Created**: 9 test files + 3 core modules + 1 config + 1 README = **14 new files**
- **Files Modified**: 8 frontend modules for universal core integration
- **Tests Created**: **37 Playwright tests** (21 E2E + 16 API-only)
- **Zero Functionality Lost**: 100% backward compatible with existing frontend code
- **Build Status**: ✅ All TypeScript checks pass with zero errors

### Ready to Run
```bash
# Start dev servers (backend + frontend)
just dev

# Run API-only tests (no browser required)
cd web && npm run test:api          # Standard output
cd web && npm run test:api:verbose  # Detailed logs

# Run E2E tests (requires browser dependencies)
cd web && npm run test:e2e
cd web && npm run test:e2e:ui

# Run all Playwright tests
cd web && npm run test:all

# View report
cd web && npm run test:report
```

---

## 📊 Executive Summary

After exhaustive analysis of the frontend codebase, **90% of the code needed for E2E tests already exists** and is well-modularized. The main challenge is that some modules depend on browser-specific APIs (IndexedDB, WebCrypto) which won't work in Playwright's Node.js context.

**Strategy**: Extract pure crypto/signing logic into universal modules that work in both browser and Node.js, while keeping browser-specific code (IndexedDB, UI) separate.

---

## 🔍 Current State Analysis

### ✅ EXCELLENT: Already Portable Code (Can Use Directly in Tests)

These modules are **pure functions** with no browser dependencies:

#### 1. **Ed25519 Core Operations** (`web/src/lib/ed25519/`)
- ✅ `ed25519-signing.ts` - Pure signing/verification using `@noble/curves`
- ✅ `ed25519-utils.ts` - Hex/bytes conversions
- ✅ `ed25519-types.ts` - Type definitions

**Usage in tests**: Import directly, works in Node.js

#### 2. **SignedRequest Core Logic** (`web/src/lib/signedRequest.ts`)
- ✅ `serializePayload()` - Deterministic JSON serialization
- ✅ `sortObjectKeys()` - Recursive key sorting
- ✅ `encodePayloadBase64()` - Base64 URL-safe encoding
- ✅ `decodePayloadBase64()` - Base64 decoding
- ✅ `serializeQueryParams()` - Query param serialization

**Usage in tests**: Pure functions, work anywhere

#### 3. **SignedResponse Validation** (`web/src/lib/signedResponse.ts`)
- ✅ `SignedResponseValidator.verifyEd25519Signature()` - Pure verification
- ✅ `SignedResponseValidator.serializePayload()` - Uses sortObjectKeys
- ✅ `isSignedResponse()` - Type guard

**Usage in tests**: Import and use directly

#### 4. **Types** (`web/src/lib/types/index.ts`)
- ✅ All TypeScript interfaces are portable

---

### ⚠️ NEEDS REFACTORING: Browser-Dependent Code

These modules mix pure logic with browser APIs:

#### 1. **Ed25519 Key Generation** (`ed25519-keygen.ts`)
**Problem**: Uses `crypto.subtle.generateKey()` (WebCrypto API)

**Current code**:
```typescript
export async function generateEd25519KeyPair(): Promise<Ed25519KeyPair> {
  // Uses WebCrypto API - NOT available in Node.js without polyfill
  const keyPair = await crypto.subtle.generateKey(...)
  ...
}

export async function generateEd25519KeyPairFallback(): Promise<Ed25519KeyPair> {
  // Uses Noble curves - WORKS in Node.js ✅
  const privateKeyBytes = crypto.getRandomValues(new Uint8Array(32));
  ...
}
```

**Solution**: Extract fallback logic to universal module

#### 2. **Ed25519 Database** (`ed25519-database.ts`)
**Problem**: Uses IndexedDB (browser-only)

**Current code**:
```typescript
export async function storeKeyPair(keyPair: Ed25519KeyPair): Promise<void> {
  const db = await openKeyDatabase(); // IndexedDB - NOT in Node.js
  ...
}
```

**Solution**: Create abstraction layer with in-memory implementation for tests

#### 3. **SignedRequest Creation** (`signedRequest.ts`)
**Problem**: Calls `getOrCreateKeyPair()` which uses IndexedDB

**Current code**:
```typescript
export async function createSignedRequest<T>(payload: T): Promise<SignedRequest> {
  const keyPair = await getOrCreateKeyPair(); // Uses IndexedDB ❌
  const jsonPayload = serializePayload(payload); // Pure ✅
  const signature = await signMessage(base64Payload, keyPair); // Pure ✅
  ...
}
```

**Solution**: Split into two functions:
- `createSignedRequestWithKeyPair(payload, keyPair)` - Pure, universal
- `createSignedRequest(payload)` - Browser wrapper that gets keypair from IndexedDB

#### 4. **HTTP Signed Requests** (`httpSignedRequests.ts`)
**Problem**: Imports sessionManager (IndexedDB) and uses fetch

**Current code**:
```typescript
export async function httpAuthenticatedSignedGETRequest<TResponse>(...) {
  const { sessionManager } = await import('./session-manager'); // IndexedDB ❌
  const authData = await sessionManager.getAuthData();
  ...
  const response = await fetch(url, ...); // Fetch ✅ (Playwright has it)
  ...
}
```

**Solution**: Extract HTTP logic to universal functions that accept dependencies as parameters

#### 5. **Session Manager** (`session-manager.ts`)
**Problem**: Uses IndexedDB exclusively

**Solution**: Create in-memory session manager for tests

---

## 🎯 Refactoring Strategy - SOLID/DRY/KISS

### Principle 1: **Dependency Injection** (Inversion of Control)

Instead of:
```typescript
async function doSomething() {
  const db = await openDatabase(); // Hard dependency ❌
  ...
}
```

Use:
```typescript
async function doSomething(storage: Storage) { // Injected dependency ✅
  const data = await storage.get();
  ...
}
```

### Principle 2: **Interface Segregation**

Create minimal interfaces for what tests need:
```typescript
interface KeyStorage {
  getKeyPair(): Promise<Ed25519KeyPair | null>;
  setKeyPair(keyPair: Ed25519KeyPair): Promise<void>;
}

// Browser implementation uses IndexedDB
class IndexedDBKeyStorage implements KeyStorage { ... }

// Test implementation uses in-memory Map
class InMemoryKeyStorage implements KeyStorage { ... }
```

### Principle 3: **Pure Core, Impure Shell**

Keep crypto/business logic pure, isolate side effects:

```
┌─────────────────────────────────────┐
│   Pure Core (Universal)             │
│   - Ed25519 signing/verification    │
│   - JSON serialization              │
│   - Base64 encoding                 │
│   - SignedRequest creation          │
└─────────────────────────────────────┘
            ▲
            │ Used by both
            │
┌───────────┴─────────────┬───────────────────────┐
│  Browser Shell          │  Test Shell           │
│  - IndexedDB storage    │  - In-memory storage  │
│  - WebCrypto keygen     │  - Noble keygen       │
│  - Fetch API            │  - Playwright fetch   │
└─────────────────────────┴───────────────────────┘
```

---

## 📋 Detailed Refactoring Plan

### Phase 1: Extract Pure Ed25519 Logic (1-2 hours)

#### Task 1.1: Create `ed25519-core.ts` (Universal Module)

**File**: `web/src/lib/ed25519/ed25519-core.ts`

**Content**: Pure signing functions that don't depend on storage

```typescript
/**
 * Ed25519 Core Module - Universal Cryptographic Operations
 *
 * Pure functions with ZERO browser dependencies
 * Can be used in Node.js, Deno, Browser, Bun, etc.
 */

import { ed25519 } from '@noble/curves/ed25519';
import { bytesToHex, hexToBytes } from '@noble/hashes/utils';
import type { Ed25519KeyPair } from './ed25519-types';

/**
 * Generate Ed25519 keypair using Noble curves (universal)
 * Works in any JavaScript runtime
 */
export function generateKeyPairNoble(): Ed25519KeyPair {
  const privateKeyBytes = new Uint8Array(32);

  // Use crypto.getRandomValues if available (browser/Node 20+)
  if (typeof crypto !== 'undefined' && crypto.getRandomValues) {
    crypto.getRandomValues(privateKeyBytes);
  } else {
    // Fallback for older Node.js
    const nodeCrypto = require('crypto');
    nodeCrypto.randomFillSync(privateKeyBytes);
  }

  const publicKeyBytes = ed25519.getPublicKey(privateKeyBytes);

  return {
    publicKey: null,
    privateKey: null,
    publicKeyBytes: new Uint8Array(publicKeyBytes),
    privateKeyBytes: privateKeyBytes,
    isNoble: true
  };
}

/**
 * Sign message with Ed25519 keypair (universal)
 */
export function signMessageWithKeyPair(
  message: string | Uint8Array,
  keyPair: Ed25519KeyPair
): string {
  const messageBytes = typeof message === 'string'
    ? new TextEncoder().encode(message)
    : message;

  if (!keyPair.privateKeyBytes) {
    throw new Error('Private key bytes required for signing');
  }

  const signature = ed25519.sign(messageBytes, keyPair.privateKeyBytes);
  return bytesToHex(signature);
}

/**
 * Verify Ed25519 signature (universal)
 */
export function verifySignatureWithPublicKey(
  message: string | Uint8Array,
  signatureHex: string,
  publicKeyBytes: Uint8Array
): boolean {
  const messageBytes = typeof message === 'string'
    ? new TextEncoder().encode(message)
    : message;

  const signatureBytes = hexToBytes(signatureHex);

  return ed25519.verify(signatureBytes, messageBytes, publicKeyBytes);
}

/**
 * Convert hex string to Ed25519 keypair (for loading from storage)
 */
export function keyPairFromHex(
  privateKeyHex: string,
  publicKeyHex: string
): Ed25519KeyPair {
  const privateKeyBytes = hexToBytes(privateKeyHex);
  const publicKeyBytes = hexToBytes(publicKeyHex);

  return {
    publicKey: null,
    privateKey: null,
    publicKeyBytes,
    privateKeyBytes,
    isNoble: true
  };
}

/**
 * Convert Ed25519 keypair to hex strings (for storage)
 */
export function keyPairToHex(keyPair: Ed25519KeyPair): {
  privateKeyHex: string;
  publicKeyHex: string;
} {
  if (!keyPair.privateKeyBytes) {
    throw new Error('Private key bytes required');
  }

  return {
    privateKeyHex: bytesToHex(keyPair.privateKeyBytes),
    publicKeyHex: bytesToHex(keyPair.publicKeyBytes)
  };
}
```

**Validation**:
- ✅ Zero browser dependencies
- ✅ Works in Node.js (Playwright context)
- ✅ Works in browser (production)
- ✅ Pure functions (deterministic, testable)

---

#### Task 1.2: Create `signedRequest-core.ts` (Universal Module)

**File**: `web/src/lib/crypto/signedRequest-core.ts`

**Content**: Pure SignedRequest creation without storage dependencies

```typescript
/**
 * SignedRequest Core Module - Universal Signed Request Creation
 *
 * Pure functions with ZERO storage dependencies
 */

import { signMessageWithKeyPair } from '../ed25519/ed25519-core';
import type { Ed25519KeyPair } from '../ed25519/ed25519-types';

export interface SignedRequest {
  payload: string;
  signature: string;
}

/**
 * Deterministic JSON serialization (already exists, re-export)
 */
export { serializePayload, sortObjectKeys, encodePayloadBase64, decodePayloadBase64 } from '../signedRequest';

/**
 * Create signed request with provided keypair (UNIVERSAL)
 *
 * This is the pure core function that tests can use directly
 */
export function createSignedRequestWithKeyPair<T>(
  payload: T,
  keyPair: Ed25519KeyPair
): SignedRequest {
  // Step 1: Serialize to deterministic JSON
  const jsonPayload = serializePayload(payload);

  // Step 2: Encode as Base64 URL-safe
  const base64Payload = encodePayloadBase64(jsonPayload);

  // Step 3: Sign the Base64 string
  const signature = signMessageWithKeyPair(base64Payload, keyPair);

  return {
    payload: base64Payload,
    signature
  };
}

/**
 * Sign query parameters with provided keypair (UNIVERSAL)
 */
export function signQueryParamsWithKeyPair(
  params: Record<string, string>,
  keyPair: Ed25519KeyPair
): string {
  const serializedParams = serializeQueryParams(params);
  return signMessageWithKeyPair(serializedParams, keyPair);
}
```

**Update existing `signedRequest.ts`** to use the universal core:

```typescript
import { createSignedRequestWithKeyPair, signQueryParamsWithKeyPair } from './crypto/signedRequest-core';
import { getOrCreateKeyPair } from './ed25519';

/**
 * Browser-specific wrapper that gets keypair from IndexedDB
 */
export async function createSignedRequest<T>(payload: T): Promise<SignedRequest> {
  const keyPair = await getOrCreateKeyPair(); // IndexedDB
  return createSignedRequestWithKeyPair(payload, keyPair); // Pure function ✅
}

/**
 * Browser-specific wrapper for query param signing
 */
export async function signQueryParams(params: Record<string, string>): Promise<string> {
  const keyPair = await getOrCreateKeyPair(); // IndexedDB
  return signQueryParamsWithKeyPair(params, keyPair); // Pure function ✅
}
```

**Validation**:
- ✅ No functionality lost (100% backward compatible)
- ✅ DRY: Universal core extracted, browser wrapper delegates
- ✅ Tests can import `createSignedRequestWithKeyPair` directly
- ✅ Browser code continues working unchanged

---

### Phase 2: Create Test Utilities (1 hour)

#### Task 2.1: In-Memory Session Manager for Tests

**File**: `tests/utils/test-session-manager.ts`

```typescript
/**
 * Test Session Manager - In-Memory Implementation
 *
 * Mimics sessionManager API without IndexedDB
 */

import type { Ed25519KeyPair } from '../../web/src/lib/ed25519/ed25519-types';
import { generateKeyPairNoble, keyPairToHex, keyPairFromHex } from '../../web/src/lib/ed25519/ed25519-core';

export class TestSessionManager {
  private keyPair: Ed25519KeyPair | null = null;
  private serverPubKey: string | null = null;
  private accessToken: string | null = null;
  private userId: string | null = null;
  private cryptoTokens: {
    cipher: string | null;
    nonce: string | null;
    hmac: string | null;
  } = { cipher: null, nonce: null, hmac: null };

  /**
   * Generate new Ed25519 keypair
   */
  async generateKeyPair(): Promise<Ed25519KeyPair> {
    this.keyPair = generateKeyPairNoble();
    return this.keyPair;
  }

  /**
   * Get current keypair
   */
  async getKeyPair(): Promise<Ed25519KeyPair | null> {
    return this.keyPair;
  }

  /**
   * Set keypair from hex strings (for rotation)
   */
  async setKeyPairFromHex(privateKeyHex: string, publicKeyHex: string): Promise<void> {
    this.keyPair = keyPairFromHex(privateKeyHex, publicKeyHex);
  }

  /**
   * Get keypair as hex strings
   */
  async getKeyPairHex(): Promise<{ privateKeyHex: string; publicKeyHex: string } | null> {
    if (!this.keyPair) return null;
    return keyPairToHex(this.keyPair);
  }

  /**
   * Set server public key
   */
  async setServerPubKey(pubKey: string): Promise<void> {
    this.serverPubKey = pubKey;
  }

  /**
   * Get server public key
   */
  async getServerPubKey(): Promise<string | null> {
    return this.serverPubKey;
  }

  /**
   * Set auth data
   */
  async setAuthData(userId: string, accessToken: string): Promise<void> {
    this.userId = userId;
    this.accessToken = accessToken;
  }

  /**
   * Get auth data
   */
  async getAuthData(): Promise<{
    user: { user_id: string; isAuthenticated: boolean } | null;
    access_token: string | null;
  }> {
    if (!this.userId || !this.accessToken) {
      return { user: null, access_token: null };
    }

    return {
      user: { user_id: this.userId, isAuthenticated: true },
      access_token: this.accessToken
    };
  }

  /**
   * Set crypto tokens
   */
  async setCryptoTokens(cipher: string, nonce: string, hmac: string): Promise<void> {
    this.cryptoTokens = { cipher, nonce, hmac };
  }

  /**
   * Get crypto tokens
   */
  async getCryptoTokens(): Promise<{
    cipher: string | null;
    nonce: string | null;
    hmac: string | null;
  }> {
    return this.cryptoTokens;
  }

  /**
   * Clear all data (logout)
   */
  async clear(): Promise<void> {
    this.keyPair = null;
    this.serverPubKey = null;
    this.accessToken = null;
    this.userId = null;
    this.cryptoTokens = { cipher: null, nonce: null, hmac: null };
  }
}
```

---

#### Task 2.2: Test Helper Functions

**File**: `tests/utils/test-auth-helpers.ts`

```typescript
/**
 * Test Auth Helpers - Reusable Authentication Functions for E2E Tests
 */

import { test as base, expect, type Page } from '@playwright/test';
import { TestSessionManager } from './test-session-manager';
import { createSignedRequestWithKeyPair, signQueryParamsWithKeyPair } from '../../web/src/lib/crypto/signedRequest-core';
import type { Ed25519KeyPair } from '../../web/src/lib/ed25519/ed25519-types';

/**
 * Extended test fixture with session manager
 */
export const test = base.extend<{ session: TestSessionManager }>({
  session: async ({}, use) => {
    const session = new TestSessionManager();
    await use(session);
    await session.clear();
  }
});

export { expect };

/**
 * Request magic link and return magic link URL
 */
export async function requestMagicLink(
  page: Page,
  session: TestSessionManager,
  email: string
): Promise<string> {
  // Generate keypair
  const keyPair = await session.generateKeyPair();

  // Create signed request payload
  const payload = {
    email,
    ui_host: 'http://localhost:5173',
    next: '/',
    email_lang: 'en',
    pub_key: bytesToHex(keyPair.publicKeyBytes)
  };

  const signedRequest = createSignedRequestWithKeyPair(payload, keyPair);

  // Send POST request to /api/login/
  const response = await page.request.post('http://localhost:3000/api/login/', {
    headers: { 'Content-Type': 'application/json' },
    data: signedRequest
  });

  expect(response.ok()).toBeTruthy();

  const data = await response.json();

  // Extract magic link from dev response
  expect(data.payload).toBeDefined();
  const decoded = JSON.parse(atob(data.payload));

  // Store server public key
  await session.setServerPubKey(decoded.server_pub_key);

  return decoded.dev_magic_link;
}

/**
 * Complete login flow by clicking magic link
 */
export async function loginWithMagicLink(
  page: Page,
  session: TestSessionManager,
  magicLink: string
): Promise<void> {
  // Navigate to magic link
  await page.goto(magicLink);

  // Wait for redirect to home
  await page.waitForURL('http://localhost:5173/');

  // Extract auth data from page context (IndexedDB)
  const authData = await page.evaluate(async () => {
    const { sessionManager } = await import('/src/lib/session-manager');
    return await sessionManager.getAuthData();
  });

  // Store in test session manager
  if (authData.user && authData.access_token) {
    await session.setAuthData(authData.user.user_id, authData.access_token);
  }
}

/**
 * Generate custom hash (authenticated request)
 */
export async function generateCustomHash(
  page: Page,
  session: TestSessionManager,
  params: { length?: number; alphabet?: string } = {}
): Promise<{ hash: string; seed: string; otp: string }> {
  const keyPair = await session.getKeyPair();
  if (!keyPair) throw new Error('No keypair available');

  const authData = await session.getAuthData();
  if (!authData.access_token) throw new Error('Not authenticated');

  // Convert params to string record
  const stringParams: Record<string, string> = {};
  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined) {
      stringParams[key] = value.toString();
    }
  });

  // Sign query params
  const signature = signQueryParamsWithKeyPair(stringParams, keyPair);

  // Build URL
  const searchParams = new URLSearchParams({ ...stringParams, signature });
  const url = `http://localhost:3000/api/custom?${searchParams}`;

  // Make authenticated request
  const response = await page.request.get(url, {
    headers: {
      'Authorization': `Bearer ${authData.access_token}`
    }
  });

  expect(response.ok()).toBeTruthy();

  const data = await response.json();

  // Verify signed response
  const serverPubKey = await session.getServerPubKey();
  if (!serverPubKey) throw new Error('No server public key');

  // TODO: Add signature verification here using signedResponse validation

  // Decode payload
  const decoded = JSON.parse(atob(data.payload));

  return {
    hash: decoded.hash,
    seed: decoded.seed,
    otp: decoded.otp
  };
}
```

---

### Phase 3: Write E2E Tests (2-3 hours)

#### Task 3.1: Setup Playwright

**File**: `playwright.config.ts`

```typescript
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: false, // Run tests sequentially for now
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: 1, // Single worker for timing-sensitive tests
  reporter: 'html',

  use: {
    baseURL: 'http://localhost:5173',
    trace: 'on-first-retry',
  },

  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],

  webServer: [
    {
      command: 'just dev',
      url: 'http://localhost:5173',
      reuseExistingServer: !process.env.CI,
      timeout: 120000,
    },
  ],
});
```

---

#### Task 3.2: Auth Flow E2E Test

**File**: `tests/e2e/auth-flow.spec.ts`

```typescript
import { test, expect } from '../utils/test-auth-helpers';
import { requestMagicLink, loginWithMagicLink } from '../utils/test-auth-helpers';

test.describe('Authentication Flow', () => {
  test('should complete full magic link authentication flow', async ({ page, session }) => {
    // Step 1: Request magic link
    const magicLink = await requestMagicLink(page, session, 'me@arkaitz.dev');

    expect(magicLink).toContain('http://localhost:5173/login');
    expect(magicLink).toContain('magiclink=');

    // Step 2: Complete login by clicking magic link
    await loginWithMagicLink(page, session, magicLink);

    // Step 3: Verify authenticated state
    const authData = await session.getAuthData();
    expect(authData.user).not.toBeNull();
    expect(authData.access_token).not.toBeNull();

    // Step 4: Verify UI shows authenticated state
    await expect(page.locator('[data-testid="logout-button"]')).toBeVisible();
  });

  test('should logout successfully', async ({ page, session }) => {
    // Login first
    const magicLink = await requestMagicLink(page, session, 'me@arkaitz.dev');
    await loginWithMagicLink(page, session, magicLink);

    // Logout
    await page.click('[data-testid="logout-button"]');
    await page.click('[data-testid="logout-confirm"]'); // Confirm dialog

    // Verify logged out
    await expect(page.locator('[data-testid="login-button"]')).toBeVisible();
  });
});
```

---

#### Task 3.3: Token Refresh E2E Test

**File**: `tests/e2e/token-refresh.spec.ts`

```typescript
import { test, expect } from '../utils/test-auth-helpers';
import { requestMagicLink, loginWithMagicLink, generateCustomHash } from '../utils/test-auth-helpers';

test.describe('Token Refresh', () => {
  test('should auto-refresh access token after 20s expiration', async ({ page, session }) => {
    // Login
    const magicLink = await requestMagicLink(page, session, 'me@arkaitz.dev');
    await loginWithMagicLink(page, session, magicLink);

    // Generate hash at t=0s (should work)
    const result1 = await generateCustomHash(page, session);
    expect(result1.hash).toBeDefined();

    // Wait 25 seconds (access token expires at 20s)
    console.log('⏱️ Waiting 25 seconds for access token to expire...');
    await page.waitForTimeout(25000);

    // Generate hash at t=25s (should trigger refresh and work transparently)
    const result2 = await generateCustomHash(page, session);
    expect(result2.hash).toBeDefined();

    // Verify no error message shown to user
    await expect(page.locator('.error-message')).not.toBeVisible();
  });
});
```

---

#### Task 3.4: Key Rotation E2E Test

**File**: `tests/e2e/key-rotation.spec.ts`

```typescript
import { test, expect } from '../utils/test-auth-helpers';
import { requestMagicLink, loginWithMagicLink, generateCustomHash } from '../utils/test-auth-helpers';

test.describe('Ed25519 Key Rotation (2/3 System)', () => {
  test.slow(); // Mark as slow test (takes ~3 minutes)

  test('should perform automatic key rotation in 2/3 time window', async ({ page, session }) => {
    // Login
    const magicLink = await requestMagicLink(page, session, 'me@arkaitz.dev');
    await loginWithMagicLink(page, session, magicLink);

    // Get initial keypair
    const initialKeyPair = await session.getKeyPairHex();
    expect(initialKeyPair).not.toBeNull();

    console.log('🔑 Initial pub_key:', initialKeyPair!.publicKeyHex.substring(0, 16) + '...');

    // t=0s: Generate hash (should work)
    const result1 = await generateCustomHash(page, session);
    expect(result1.hash).toBeDefined();

    // Wait 62s (TRAMO 1/3: partial refresh, no rotation)
    console.log('⏱️ Waiting 62s for TRAMO 1/3...');
    await page.waitForTimeout(62000);

    // t=62s: Generate hash (should refresh token only, no key rotation)
    const result2 = await generateCustomHash(page, session);
    expect(result2.hash).toBeDefined();

    const keyPairAfterTramo1 = await session.getKeyPairHex();
    expect(keyPairAfterTramo1!.publicKeyHex).toBe(initialKeyPair!.publicKeyHex); // Same key ✅

    // Wait 48s more (total 110s - TRAMO 2/3: full rotation)
    console.log('⏱️ Waiting 48s more for TRAMO 2/3...');
    await page.waitForTimeout(48000);

    // t=110s: Generate hash (should trigger KEY ROTATION)
    const result3 = await generateCustomHash(page, session);
    expect(result3.hash).toBeDefined();

    const keyPairAfterRotation = await session.getKeyPairHex();
    expect(keyPairAfterRotation).not.toBeNull();
    expect(keyPairAfterRotation!.publicKeyHex).not.toBe(initialKeyPair!.publicKeyHex); // DIFFERENT key ✅

    console.log('🔑 Rotated pub_key:', keyPairAfterRotation!.publicKeyHex.substring(0, 16) + '...');

    // t=110s+: Generate another hash with NEW keypair (should work)
    const result4 = await generateCustomHash(page, session);
    expect(result4.hash).toBeDefined();
  });
});
```

---

### Phase 4: Validation & Documentation (1 hour)

#### Task 4.1: Run Tests and Fix Issues

```bash
# Install Playwright
npm install -D @playwright/test

# Install browsers
npx playwright install chromium

# Run tests
npx playwright test

# Run specific test
npx playwright test tests/e2e/auth-flow.spec.ts

# Debug mode
npx playwright test --debug
```

#### Task 4.2: Update Documentation

**Add to `README.md`**:

```markdown
## E2E Testing

We have comprehensive E2E tests using Playwright:

- **39 backend tests** (bash scripts)
- **XX frontend E2E tests** (Playwright)

### Run E2E Tests

```bash
# Run all E2E tests
npx playwright test

# Run specific test
npx playwright test auth-flow

# Debug mode
npx playwright test --debug

# Show test report
npx playwright show-report
```

### E2E Test Coverage

- ✅ Full magic link authentication flow
- ✅ Automatic token refresh (20s expiration)
- ✅ Ed25519 key rotation (2/3 system)
- ✅ Hash generation (custom, password, api-key, mnemonic)
- ✅ Logout flow
- ✅ Multi-language UI switching
```

---

## 📊 Success Criteria

### Phase 1 Validation
- [ ] `ed25519-core.ts` can be imported in Node.js
- [ ] `signedRequest-core.ts` works without browser APIs
- [ ] All existing browser code still works (no regressions)
- [ ] `just check` passes (no lint/type errors)

### Phase 2 Validation
- [ ] `TestSessionManager` can generate and store keypairs
- [ ] `test-auth-helpers.ts` can create signed requests
- [ ] Helper functions work in Playwright context

### Phase 3 Validation
- [ ] Auth flow test passes (login + logout)
- [ ] Token refresh test passes (25s wait)
- [ ] Key rotation test passes (~3min wait)
- [ ] No false positives/negatives

### Phase 4 Validation
- [ ] All tests run in CI/CD
- [ ] Documentation updated
- [ ] Test coverage report generated

---

## 🎯 File Structure After Refactoring

```
web/src/lib/
├── ed25519/
│   ├── ed25519-core.ts          # ✨ NEW: Universal crypto (Node.js + Browser)
│   ├── ed25519-keygen.ts        # UPDATED: Uses ed25519-core
│   ├── ed25519-signing.ts       # UPDATED: Uses ed25519-core
│   ├── ed25519-database.ts      # Unchanged (browser-only)
│   ├── ed25519-api.ts           # Unchanged (browser wrapper)
│   └── index.ts                 # UPDATED: Export ed25519-core
├── crypto/
│   ├── signedRequest-core.ts    # ✨ NEW: Universal SignedRequest
│   └── ... (existing crypto modules)
├── signedRequest.ts             # UPDATED: Uses signedRequest-core
├── signedResponse.ts            # Unchanged (already universal)
└── httpSignedRequests.ts        # UPDATED: Uses signedRequest-core

tests/
├── utils/
│   ├── test-session-manager.ts  # ✨ NEW: In-memory session for tests
│   └── test-auth-helpers.ts     # ✨ NEW: Reusable test functions
└── e2e/
    ├── auth-flow.spec.ts        # ✨ NEW: Auth E2E tests
    ├── token-refresh.spec.ts    # ✨ NEW: Refresh E2E tests
    ├── key-rotation.spec.ts     # ✨ NEW: Rotation E2E tests
    └── hash-generation.spec.ts  # ✨ NEW: Generation E2E tests

playwright.config.ts              # ✨ NEW: Playwright configuration
```

---

## 🚧 Potential Challenges & Solutions

### Challenge 1: WebCrypto in Playwright

**Problem**: Playwright's page.request context doesn't have WebCrypto

**Solution**: Use Noble curves exclusively in test context (already works)

### Challenge 2: IndexedDB in Tests

**Problem**: Can't access IndexedDB from Playwright's request context

**Solution**: Use `TestSessionManager` (in-memory) for test logic, but can still read real IndexedDB from browser context using `page.evaluate()`

### Challenge 3: Timing-Sensitive Tests

**Problem**: Key rotation test takes ~3 minutes

**Solution**:
- Mark with `test.slow()`
- Run in separate test suite
- Use shorter timeouts in dev mode (via environment variable)

### Challenge 4: Magic Link Email

**Problem**: Can't click real email in automated tests

**Solution**: Backend already returns `dev_magic_link` in development mode ✅

---

## 📈 Metrics & KPIs

### Code Reuse
- **Before**: 0% frontend code reuse in tests
- **After**: ~90% frontend code reused in E2E tests

### Test Coverage
- **Backend**: 39 tests (100% endpoints)
- **Frontend E2E**: ~15 tests (auth flow, refresh, rotation, generation)
- **Total**: ~54 automated tests

### DRY Violations Eliminated
- SignedRequest creation: 2 implementations → 1 universal core
- Ed25519 signing: 2 implementations → 1 universal core
- Session management: 1 interface, 2 implementations (IndexedDB + in-memory)

### Lines of Code
- **New code**: ~800 lines (test utils + E2E tests)
- **Refactored code**: ~400 lines (extracted to universal modules)
- **Deleted code**: 0 lines (100% backward compatible)

---

## ✅ Final Checklist

### Before Starting
- [x] Read and understand entire plan
- [ ] Backup current codebase (`git commit`)
- [ ] Create feature branch (`git checkout -b feature/e2e-tests`)

### During Implementation
- [ ] Follow plan step-by-step
- [ ] Test each phase before moving to next
- [ ] Run `just check` after each file change
- [ ] Document any deviations from plan

### After Completion
- [ ] All tests passing (`npx playwright test`)
- [ ] No regressions (`just check` + `just test`)
- [ ] Documentation updated (README, CHANGELOG)
- [ ] Commit with detailed message
- [ ] Ask for user approval before merging

---

## 🎓 Lessons & Principles Applied

1. **SOLID**: Single Responsibility (pure core vs impure shell)
2. **DRY**: Universal modules reused in browser + tests
3. **KISS**: Simple abstractions (KeyStorage interface)
4. **Dependency Injection**: Functions accept dependencies as parameters
5. **Interface Segregation**: Minimal test interfaces
6. **Open/Closed**: Extended with tests without modifying existing code

---

**End of Implementation Plan**

**Estimated Total Time**: 7-10 hours

**Status**: ✅ Ready for Implementation

**Next Step**: Get user approval and start Phase 1
