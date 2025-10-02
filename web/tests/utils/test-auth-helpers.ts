/**
 * Test Auth Helpers - Reusable Authentication Functions for E2E Tests
 *
 * Provides Playwright test fixtures and authentication helper functions
 * that replicate bash test patterns using universal core modules.
 *
 * Follows patterns from:
 * - scripts/final_test.sh (authentication flow)
 * - scripts/test_2_3_system.sh (key rotation flow)
 * - scripts/create_signed_request.js (SignedRequest creation)
 * - scripts/verify_signed_response.js (SignedResponse verification)
 */

import { test as base, expect, type Page, type APIRequestContext } from '@playwright/test';
import { TestSessionManager } from './test-session-manager';
import {
	createSignedRequestWithKeyPair,
	signQueryParamsWithKeyPair,
	decodePayloadBase64
} from '../../src/lib/crypto/signedRequest-core';
import { publicKeyBytesToHex } from '../../src/lib/ed25519/ed25519-core';
import { ed25519 } from '@noble/curves/ed25519.js';

/**
 * SignedResponse structure from backend
 */
export interface SignedResponse<_T = unknown> {
	/** Base64 URL-safe encoded deterministic JSON payload */
	payload: string;
	/** Ed25519 signature of the Base64 payload string */
	signature: string;
}

/**
 * Extended test fixture with TestSessionManager
 *
 * Usage:
 * ```typescript
 * import { test, expect } from '../utils/test-auth-helpers';
 *
 * test('should authenticate', async ({ page, session }) => {
 *   // session is automatically created and cleaned up
 * });
 * ```
 */
export const test = base.extend<{ session: TestSessionManager }>({
	session: async (_context, use) => {
		const session = new TestSessionManager();
		await use(session);
		await session.clear();
	}
});

export { expect };

/**
 * Verify Ed25519 signature of SignedResponse
 *
 * Replicates verify_signed_response.js logic
 *
 * @param signedResponse - Response from backend with payload and signature
 * @param serverPubKeyHex - Server's Ed25519 public key (64 hex chars)
 * @returns True if signature is valid
 */
export function verifySignedResponse(
	signedResponse: SignedResponse,
	serverPubKeyHex: string
): boolean {
	try {
		// Convert hex strings to bytes
		const publicKeyBytes = new Uint8Array(
			serverPubKeyHex.match(/.{2}/g)?.map((byte) => parseInt(byte, 16)) || []
		);
		const signatureBytes = new Uint8Array(
			signedResponse.signature.match(/.{2}/g)?.map((byte) => parseInt(byte, 16)) || []
		);

		// Message to verify is the Base64 payload string (what backend signed)
		const messageBytes = new TextEncoder().encode(signedResponse.payload);

		// Verify Ed25519 signature using Noble curves
		return ed25519.verify(signatureBytes, messageBytes, publicKeyBytes);
	} catch (error) {
		console.error('Signature verification error:', error);
		return false;
	}
}

/**
 * Decode and extract field from SignedResponse payload
 *
 * Replicates verify_signed_response.js extract-field logic
 *
 * @param signedResponse - Response with Base64 payload
 * @param fieldName - Field name to extract from decoded payload
 * @returns Field value or null if not found
 */
export function extractFromPayload(signedResponse: SignedResponse, fieldName: string): unknown {
	try {
		// Decode Base64 to JSON string
		const jsonString = decodePayloadBase64(signedResponse.payload);

		// Parse JSON to object
		const payloadObj = JSON.parse(jsonString) as Record<string, unknown>;

		return payloadObj[fieldName] ?? null;
	} catch (error) {
		console.error('Payload extraction error:', error);
		return null;
	}
}

/**
 * Request magic link and return magic link URL
 *
 * Replicates bash test pattern:
 * 1. Generate Ed25519 keypair
 * 2. Create signed request with email + pub_key
 * 3. POST to /api/login/
 * 4. Extract dev_magic_link and server_pub_key from response
 *
 * @param request - Playwright APIRequestContext
 * @param session - TestSessionManager instance
 * @param email - Email address for magic link
 * @param emailLang - Language for email (default: 'en')
 * @param nextPath - Path to redirect after login (default: '/')
 * @returns Magic link URL
 */
export async function requestMagicLink(
	request: APIRequestContext,
	session: TestSessionManager,
	email: string,
	emailLang: string = 'en',
	nextPath: string = '/'
): Promise<string> {
	// Step 1: Generate Ed25519 keypair
	const keyPair = await session.generateKeyPair();
	const pubKeyHex = publicKeyBytesToHex(keyPair.publicKeyBytes);

	console.log(`🔑 Generated Ed25519 keypair: ${pubKeyHex.substring(0, 20)}...`);

	// Step 2: Create signed request payload
	const payload = {
		email,
		email_lang: emailLang,
		next: nextPath,
		pub_key: pubKeyHex,
		ui_host: 'localhost' // Matches bash test pattern
	};

	const signedRequest = createSignedRequestWithKeyPair(payload, keyPair);

	console.log('📤 Sending signed magic link request to /api/login/');

	// Step 3: POST to /api/login/
	const response = await request.post('http://localhost:3000/api/login/', {
		headers: { 'Content-Type': 'application/json' },
		data: signedRequest
	});

	expect(response.ok()).toBeTruthy();

	const signedResponse = (await response.json()) as SignedResponse;

	// Step 4: Verify signed response structure
	expect(signedResponse.payload).toBeDefined();
	expect(signedResponse.signature).toBeDefined();

	// Step 5: Extract server_pub_key from first signed response
	const serverPubKey = extractFromPayload(signedResponse, 'server_pub_key') as string;
	expect(serverPubKey).toBeTruthy();
	expect(serverPubKey).toHaveLength(64); // 32 bytes = 64 hex chars

	// Store server public key for future response validations
	await session.setServerPubKey(serverPubKey);

	console.log(`🔐 Stored server pub_key: ${serverPubKey.substring(0, 20)}...`);

	// Step 6: Verify signature with server public key
	const isValid = verifySignedResponse(signedResponse, serverPubKey);
	expect(isValid).toBe(true);

	console.log('✅ Server signature verified');

	// Step 7: Extract dev_magic_link from payload
	const devMagicLink = extractFromPayload(signedResponse, 'dev_magic_link') as string;
	expect(devMagicLink).toBeTruthy();
	expect(devMagicLink).toContain('http://localhost:5173/auth/callback');

	console.log(`🔗 Magic link received: ${devMagicLink.substring(0, 60)}...`);

	return devMagicLink;
}

/**
 * Complete login flow by navigating to magic link
 *
 * Replicates bash test pattern:
 * 1. Navigate to magic link URL
 * 2. Wait for redirect to home page
 * 3. Extract auth data from IndexedDB (via page.evaluate)
 * 4. Store access token and user_id in test session
 *
 * @param page - Playwright Page instance
 * @param session - TestSessionManager instance
 * @param magicLink - Magic link URL from requestMagicLink()
 */
export async function loginWithMagicLink(
	page: Page,
	session: TestSessionManager,
	magicLink: string
): Promise<void> {
	console.log('🔗 Navigating to magic link...');

	// Step 1: Navigate to magic link
	await page.goto(magicLink);

	// Step 2: Wait for redirect to home
	console.log('⏳ Waiting for redirect to home page...');
	await page.waitForURL('http://localhost:5173/', { timeout: 10000 });

	console.log('✅ Redirected to home page');

	// Step 3: Extract auth data from page context (IndexedDB)
	const authData = await page.evaluate(async () => {
		// @ts-expect-error - Dynamic import path works in browser runtime via Vite
		const { sessionManager } = await import('/src/lib/session-manager');
		return await sessionManager.getAuthData();
	});

	// Step 4: Store in test session manager
	if (authData.user && authData.access_token) {
		await session.setAuthData(authData.user.user_id, authData.access_token);
		console.log(`✅ Logged in as user: ${authData.user.user_id.substring(0, 20)}...`);
		console.log(`🔑 Access token stored (${authData.access_token.length} chars)`);
	} else {
		throw new Error('Login failed: No auth data available after magic link navigation');
	}
}

/**
 * Generate custom hash with authenticated request
 *
 * Replicates bash test pattern for /api/custom:
 * 1. Get keypair and auth data
 * 2. Sign query parameters with Ed25519
 * 3. Send GET request with Authorization header
 * 4. Verify signed response
 * 5. Extract hash, seed, otp from payload
 *
 * @param request - Playwright APIRequestContext
 * @param session - TestSessionManager instance
 * @param params - Optional hash generation parameters (length, alphabet)
 * @returns Hash generation result
 */
export async function generateCustomHash(
	request: APIRequestContext,
	session: TestSessionManager,
	params: { length?: number; alphabet?: string } = {}
): Promise<{ hash: string; seed: string; otp: string }> {
	// Step 1: Get keypair and auth data
	const keyPair = await session.getKeyPair();
	if (!keyPair) throw new Error('No keypair available - call requestMagicLink first');

	const authData = await session.getAuthData();
	if (!authData.access_token) throw new Error('Not authenticated - call loginWithMagicLink first');

	const serverPubKey = await session.getServerPubKey();
	if (!serverPubKey) throw new Error('No server public key - call requestMagicLink first');

	// Step 2: Convert params to string record for signing
	const stringParams: Record<string, string> = {};
	Object.entries(params).forEach(([key, value]) => {
		if (value !== undefined) {
			stringParams[key] = value.toString();
		}
	});

	// Step 3: Sign query params with Ed25519
	const signature = signQueryParamsWithKeyPair(stringParams, keyPair);

	// Step 4: Build URL with signed parameters
	const searchParams = new URLSearchParams({ ...stringParams, signature });
	const url = `http://localhost:3000/api/custom?${searchParams}`;

	console.log(`📤 Sending authenticated request to /api/custom`);

	// Step 5: Make authenticated GET request with JWT Bearer token
	const response = await request.get(url, {
		headers: {
			Authorization: `Bearer ${authData.access_token}`
		}
	});

	expect(response.ok()).toBeTruthy();

	const signedResponse = (await response.json()) as SignedResponse;

	// Step 6: Verify signed response
	const isValid = verifySignedResponse(signedResponse, serverPubKey);
	expect(isValid).toBe(true);

	console.log('✅ Response signature verified');

	// Step 7: Extract hash data from payload
	const hash = extractFromPayload(signedResponse, 'hash') as string;
	const seed = extractFromPayload(signedResponse, 'seed') as string;
	const otp = extractFromPayload(signedResponse, 'otp') as string;

	expect(hash).toBeTruthy();
	expect(seed).toBeTruthy();
	expect(otp).toBeTruthy();

	console.log(`✅ Generated hash: ${hash.substring(0, 20)}...`);

	return { hash, seed, otp };
}

/**
 * Generate password hash with authenticated request
 *
 * Similar to generateCustomHash but for /api/password endpoint
 *
 * @param request - Playwright APIRequestContext
 * @param session - TestSessionManager instance
 * @param params - Optional password parameters (length, symbols, etc.)
 * @returns Password generation result
 */
export async function generatePassword(
	request: APIRequestContext,
	session: TestSessionManager,
	params: {
		length?: number;
		uppercase?: boolean;
		lowercase?: boolean;
		numbers?: boolean;
		symbols?: boolean;
	} = {}
): Promise<{ password: string; seed: string; otp: string }> {
	const keyPair = await session.getKeyPair();
	if (!keyPair) throw new Error('No keypair available');

	const authData = await session.getAuthData();
	if (!authData.access_token) throw new Error('Not authenticated');

	const serverPubKey = await session.getServerPubKey();
	if (!serverPubKey) throw new Error('No server public key');

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
	const url = `http://localhost:3000/api/password?${searchParams}`;

	console.log(`📤 Sending authenticated request to /api/password`);

	// Make authenticated request
	const response = await request.get(url, {
		headers: {
			Authorization: `Bearer ${authData.access_token}`
		}
	});

	expect(response.ok()).toBeTruthy();

	const signedResponse = (await response.json()) as SignedResponse;

	// Verify signed response
	const isValid = verifySignedResponse(signedResponse, serverPubKey);
	expect(isValid).toBe(true);

	// Extract password data
	const password = extractFromPayload(signedResponse, 'password') as string;
	const seed = extractFromPayload(signedResponse, 'seed') as string;
	const otp = extractFromPayload(signedResponse, 'otp') as string;

	expect(password).toBeTruthy();
	expect(seed).toBeTruthy();
	expect(otp).toBeTruthy();

	console.log(`✅ Generated password (${password.length} chars)`);

	return { password, seed, otp };
}

/**
 * Generate API key with authenticated request
 *
 * Similar to generateCustomHash but for /api/api-key endpoint
 *
 * @param request - Playwright APIRequestContext
 * @param session - TestSessionManager instance
 * @returns API key generation result
 */
export async function generateApiKey(
	request: APIRequestContext,
	session: TestSessionManager
): Promise<{ api_key: string; seed: string; otp: string }> {
	const keyPair = await session.getKeyPair();
	if (!keyPair) throw new Error('No keypair available');

	const authData = await session.getAuthData();
	if (!authData.access_token) throw new Error('Not authenticated');

	const serverPubKey = await session.getServerPubKey();
	if (!serverPubKey) throw new Error('No server public key');

	// Sign empty query params (no parameters for api-key endpoint)
	const signature = signQueryParamsWithKeyPair({}, keyPair);

	// Build URL
	const url = `http://localhost:3000/api/api-key?signature=${signature}`;

	console.log(`📤 Sending authenticated request to /api/api-key`);

	// Make authenticated request
	const response = await request.get(url, {
		headers: {
			Authorization: `Bearer ${authData.access_token}`
		}
	});

	expect(response.ok()).toBeTruthy();

	const signedResponse = (await response.json()) as SignedResponse;

	// Verify signed response
	const isValid = verifySignedResponse(signedResponse, serverPubKey);
	expect(isValid).toBe(true);

	// Extract API key data
	const api_key = extractFromPayload(signedResponse, 'api_key') as string;
	const seed = extractFromPayload(signedResponse, 'seed') as string;
	const otp = extractFromPayload(signedResponse, 'otp') as string;

	expect(api_key).toBeTruthy();
	expect(seed).toBeTruthy();
	expect(otp).toBeTruthy();

	console.log(`✅ Generated API key: ${api_key.substring(0, 20)}...`);

	return { api_key, seed, otp };
}

/**
 * Generate BIP39 mnemonic with authenticated request
 *
 * Similar to generateCustomHash but for /api/mnemonic endpoint
 *
 * @param request - Playwright APIRequestContext
 * @param session - TestSessionManager instance
 * @param params - Optional mnemonic parameters (word_count, language)
 * @returns Mnemonic generation result
 */
export async function generateMnemonic(
	request: APIRequestContext,
	session: TestSessionManager,
	params: { word_count?: number; language?: string } = {}
): Promise<{ mnemonic: string; seed: string; otp: string }> {
	const keyPair = await session.getKeyPair();
	if (!keyPair) throw new Error('No keypair available');

	const authData = await session.getAuthData();
	if (!authData.access_token) throw new Error('Not authenticated');

	const serverPubKey = await session.getServerPubKey();
	if (!serverPubKey) throw new Error('No server public key');

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
	const url = `http://localhost:3000/api/mnemonic?${searchParams}`;

	console.log(`📤 Sending authenticated request to /api/mnemonic`);

	// Make authenticated request
	const response = await request.get(url, {
		headers: {
			Authorization: `Bearer ${authData.access_token}`
		}
	});

	expect(response.ok()).toBeTruthy();

	const signedResponse = (await response.json()) as SignedResponse;

	// Verify signed response
	const isValid = verifySignedResponse(signedResponse, serverPubKey);
	expect(isValid).toBe(true);

	// Extract mnemonic data
	const mnemonic = extractFromPayload(signedResponse, 'mnemonic') as string;
	const seed = extractFromPayload(signedResponse, 'seed') as string;
	const otp = extractFromPayload(signedResponse, 'otp') as string;

	expect(mnemonic).toBeTruthy();
	expect(seed).toBeTruthy();
	expect(otp).toBeTruthy();

	console.log(`✅ Generated mnemonic (${mnemonic.split(' ').length} words)`);

	return { mnemonic, seed, otp };
}

/**
 * Check if access token is expired (for timing tests)
 *
 * Tries to make an authenticated request and checks for 401
 *
 * @param request - Playwright APIRequestContext
 * @param session - TestSessionManager instance
 * @returns True if token is expired (401 response)
 */
export async function isAccessTokenExpired(
	request: APIRequestContext,
	session: TestSessionManager
): Promise<boolean> {
	try {
		await generateCustomHash(request, session, {});
		return false; // Request succeeded, token is valid
	} catch {
		// If we get a 401, token is expired
		return true;
	}
}

/**
 * Wait for specified duration with progress logging
 *
 * Useful for timing-sensitive tests (token expiration, key rotation)
 *
 * @param seconds - Seconds to wait
 * @param reason - Reason for waiting (for logging)
 */
export async function waitForSeconds(seconds: number, reason: string): Promise<void> {
	console.log(`⏳ Waiting ${seconds}s - ${reason}`);

	const startTime = Date.now();
	const intervalMs = 1000; // Log every 1 second

	return new Promise((resolve) => {
		const interval = setInterval(() => {
			const elapsed = Math.floor((Date.now() - startTime) / 1000);
			const remaining = seconds - elapsed;

			if (remaining <= 0) {
				clearInterval(interval);
				console.log(`✅ Wait complete (${seconds}s elapsed)`);
				resolve();
			} else {
				console.log(`⏳ ${remaining}s remaining...`);
			}
		}, intervalMs);
	});
}
