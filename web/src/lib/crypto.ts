/**
 * Cryptographic utilities for parameter hashing and URL encryption
 *
 * This module provides secure parameter processing using Blake2b keyed hashing
 * followed by ChaCha8 RNG to generate deterministic but unpredictable prehashes.
 */

import { blake2b } from '@noble/hashes/blake2.js';
import { rngChacha8, chacha20poly1305 } from '@noble/ciphers/chacha.js';

/**
 * Generic cryptographic hash generator using Blake2b-keyed + ChaCha8RNG
 *
 * @param data - Input data (string or Uint8Array)
 * @param key - Key for Blake2b keyed hash (string base64 or Uint8Array)
 * @param outputLength - Desired output length in bytes
 * @returns Generated hash as Uint8Array
 */
export function cryptoHashGen(
	data: string | Uint8Array,
	key: string | Uint8Array,
	outputLength: number
): Uint8Array {
	// Convert inputs to Uint8Array if needed
	const dataBytes = typeof data === 'string' ? new TextEncoder().encode(data) : data;
	const keyBytes =
		typeof key === 'string'
			? new Uint8Array(
					atob(key)
						.split('')
						.map((char) => char.charCodeAt(0))
				)
			: key;

	// Step 1: Blake2b keyed hash (32 bytes seed)
	const seed = blake2b(dataBytes, {
		key: keyBytes,
		dkLen: 32
	});

	// Step 2: ChaCha8 RNG using the seed
	const rng = rngChacha8(seed);
	const output = rng.randomBytes(outputLength);

	return output;
}

/**
 * Generate a 32-byte prehash from prehash seed using cryptoHashGen
 *
 * @param prehashSeed - 32-byte prehash seed
 * @param hmacKey - 32-byte HMAC key from session (base64 encoded)
 * @returns 32-byte prehash as Uint8Array
 */
export function generatePrehash(prehashSeed: Uint8Array, hmacKey: string): Uint8Array {
	return cryptoHashGen(prehashSeed, hmacKey, 32);
}

/**
 * Generate ChaCha-Poly cipher key using session cipher token and prehash
 *
 * @param cipherToken - Session cipher token (base64 encoded)
 * @param prehash - 32-byte prehash as key
 * @returns 32-byte cipher key for ChaCha-Poly
 */
export function generateCipherKey(cipherToken: string, prehash: Uint8Array): Uint8Array {
	return cryptoHashGen(cipherToken, prehash, 32);
}

/**
 * Generate ChaCha-Poly nonce using session nonce token and prehash
 *
 * @param nonceToken - Session nonce token (base64 encoded)
 * @param prehash - 32-byte prehash as key
 * @returns 12-byte nonce for ChaCha-Poly
 */
export function generateCipherNonce(nonceToken: string, prehash: Uint8Array): Uint8Array {
	return cryptoHashGen(nonceToken, prehash, 12);
}

/**
 * Convert Uint8Array to base64 string for URL-safe transmission
 *
 * @param bytes - Uint8Array to convert
 * @returns base64 encoded string
 */
export function bytesToBase64(bytes: Uint8Array): string {
	return btoa(String.fromCharCode(...bytes));
}

/**
 * Convert Uint8Array to base64URL string (URL-safe, no padding)
 *
 * @param bytes - Uint8Array to convert
 * @returns base64URL encoded string
 */
export function bytesToBase64Url(bytes: Uint8Array): string {
	return btoa(String.fromCharCode(...bytes))
		.replace(/\+/g, '-')
		.replace(/\//g, '_')
		.replace(/=/g, '');
}

/**
 * Convert base64 string back to Uint8Array
 *
 * @param base64 - base64 encoded string
 * @returns Uint8Array
 */
export function base64ToBytes(base64: string): Uint8Array {
	return new Uint8Array(
		atob(base64)
			.split('')
			.map((char) => char.charCodeAt(0))
	);
}

/**
 * Convert base64URL string back to Uint8Array
 *
 * @param base64Url - base64URL encoded string
 * @returns Uint8Array
 */
export function base64UrlToBytes(base64Url: string): Uint8Array {
	// Add padding if needed
	let base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
	while (base64.length % 4) {
		base64 += '=';
	}
	return base64ToBytes(base64);
}

/**
 * Generate cryptographically secure salt (32 bytes)
 *
 * @returns 32-byte salt as Uint8Array
 */
export function generateCryptoSalt(): Uint8Array {
	return crypto.getRandomValues(new Uint8Array(32));
}

/**
 * Generate cryptographically secure prehash seed (32 bytes)
 *
 * @returns 32-byte prehash seed as Uint8Array
 */
export function generatePrehashSeed(): Uint8Array {
	return crypto.getRandomValues(new Uint8Array(32));
}

/**
 * Store prehash seed in sessionStorage and return key
 *
 * @param seed - 32-byte prehash seed
 * @param hmacKey - HMAC key for generating unique key
 * @returns Base64URL key for retrieval
 */
export function storePrehashSeed(seed: Uint8Array, hmacKey: string): string {
	const seedsJson = sessionStorage.getItem('prehashseeds') || '[]';
	const seeds: { k: string; v: string }[] = JSON.parse(seedsJson);

	// Generate 8-byte key using cryptoHashGen
	const keyBytes = cryptoHashGen(seed, hmacKey, 8);
	const key = bytesToBase64Url(keyBytes);

	// Store as KV pair
	const seedBase64 = bytesToBase64(seed);
	seeds.push({ k: key, v: seedBase64 });

	// FIFO rotation: Remove oldest if limit exceeded (max 20 KV pairs)
	if (seeds.length > 20) {
		seeds.shift(); // Remove first (oldest) element
	}

	sessionStorage.setItem('prehashseeds', JSON.stringify(seeds));

	return key;
}

/**
 * Retrieve prehash seed from sessionStorage by key
 *
 * @param key - Base64URL key to find seed
 * @returns 32-byte prehash seed or null if not found
 */
export function getPrehashSeed(key: string): Uint8Array | null {
	const seedsJson = sessionStorage.getItem('prehashseeds');
	if (!seedsJson) return null;

	const seeds: { k: string; v: string }[] = JSON.parse(seedsJson);
	const found = seeds.find((seed) => seed.k === key);

	if (!found) return null;

	return base64ToBytes(found.v);
}

/**
 * Serialize parameters object to a consistent string representation
 *
 * @param params - Parameters object to serialize
 * @returns Consistent string representation
 */
export function serializeParams(params: Record<string, any>): string {
	// Sort keys for consistent output
	const sortedKeys = Object.keys(params).sort();
	const sortedParams: Record<string, any> = {};

	for (const key of sortedKeys) {
		sortedParams[key] = params[key];
	}

	return JSON.stringify(sortedParams);
}

/**
 * Encrypt parameters for secure URL transmission using prehash seed
 * Returns single compact parameter 'p' containing idx_bytes + encrypted_bytes
 *
 * @param params - Parameters object to encrypt
 * @param cipherToken - Session cipher token (base64)
 * @param nonceToken - Session nonce token (base64)
 * @param hmacKey - Session HMAC key (base64)
 * @returns Single compact parameter 'p' as base64URL string
 */
export function encryptUrlParams(
	params: Record<string, any>,
	cipherToken: string,
	nonceToken: string,
	hmacKey: string
): { p: string } {
	// 1. Add crypto salt to parameters for noise
	const salt = generateCryptoSalt();
	const saltBase64 = bytesToBase64(salt);
	const paramsWithSalt = { ...params, _salt: saltBase64 };

	// 2. Generate random prehash seed (independent of content)
	const prehashSeed = generatePrehashSeed();

	// 3. Store prehash seed and get key (8 bytes)
	const idx = storePrehashSeed(prehashSeed, hmacKey);
	const idxBytes = base64UrlToBytes(idx); // Convert idx back to 8 bytes

	// 4. Generate prehash from seed
	const prehash = generatePrehash(prehashSeed, hmacKey);

	// 5. Generate cipher key and nonce for ChaCha20-Poly1305
	const cipherKey = generateCipherKey(cipherToken, prehash);
	const cipherNonce = generateCipherNonce(nonceToken, prehash);

	// 6. Encrypt params (with salt) using ChaCha20-Poly1305
	const paramsString = serializeParams(paramsWithSalt);
	const cipher = chacha20poly1305(cipherKey, cipherNonce);
	const plaintext = new TextEncoder().encode(paramsString);
	const ciphertext = cipher.encrypt(plaintext);

	// 7. Concatenate idx_bytes (8 bytes) + encrypted_bytes and encode as base64URL
	const combined = new Uint8Array(idxBytes.length + ciphertext.length);
	combined.set(idxBytes, 0);
	combined.set(ciphertext, idxBytes.length);

	return {
		p: bytesToBase64Url(combined)
	};
}

/**
 * Decrypt parameters from secure URL transmission using compact 'p' parameter
 * Extracts idx (first 8 bytes) and encrypted data (remaining bytes)
 *
 * @param compactParam - Compact parameter 'p' containing idx_bytes + encrypted_bytes
 * @param cipherToken - Session cipher token (base64)
 * @param nonceToken - Session nonce token (base64)
 * @param hmacKey - Session HMAC key (base64)
 * @returns Decrypted parameters object (without salt)
 */
export function decryptUrlParams(
	compactParam: string,
	cipherToken: string,
	nonceToken: string,
	hmacKey: string
): Record<string, any> {
	// 1. Decode base64URL to get combined bytes
	const combinedBytes = base64UrlToBytes(compactParam);

	// 2. Extract idx_bytes (first 8 bytes) and encrypted_bytes (remaining)
	if (combinedBytes.length < 8) {
		throw new Error('Invalid compact parameter: too short for idx extraction');
	}

	const idxBytes = combinedBytes.slice(0, 8);
	const encryptedBytes = combinedBytes.slice(8);

	// 3. Convert idx_bytes back to base64URL string for sessionStorage lookup
	const idx = bytesToBase64Url(idxBytes);

	// 4. Retrieve prehash seed from sessionStorage
	const prehashSeed = getPrehashSeed(idx);
	if (!prehashSeed) {
		throw new Error(`Prehash seed not found with key ${idx}`);
	}

	// 5. Regenerate prehash from seed
	const prehash = generatePrehash(prehashSeed, hmacKey);

	// 6. Regenerate cipher key and nonce
	const cipherKey = generateCipherKey(cipherToken, prehash);
	const cipherNonce = generateCipherNonce(nonceToken, prehash);

	// 7. Decrypt with ChaCha20-Poly1305
	const cipher = chacha20poly1305(cipherKey, cipherNonce);
	const plaintext = cipher.decrypt(encryptedBytes);

	// 8. Convert to string and parse JSON
	const paramsString = new TextDecoder().decode(plaintext);
	const paramsWithSalt = JSON.parse(paramsString);

	// 9. Remove internal salt and return clean params
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	const { _salt, ...params } = paramsWithSalt;
	return params;
}

/**
 * Complete URL parameter encryption workflow
 *
 * @param params - Parameters to encrypt
 * @param sessionTokens - Session tokens from authStore
 * @returns Object with compact parameter 'p' for URL
 */
export function prepareSecureUrlParams(
	params: Record<string, any>,
	sessionTokens: {
		cipherToken: string;
		nonceToken: string;
		hmacKey: string;
	}
): {
	p: string;
} {
	return encryptUrlParams(
		params,
		sessionTokens.cipherToken,
		sessionTokens.nonceToken,
		sessionTokens.hmacKey
	);
}

/**
 * Extract route and parameters from a URL
 *
 * @param url - URL string to parse
 * @returns Object with basePath and parameters
 */
export function parseNextUrl(url: string): {
	basePath: string;
	params: Record<string, string>;
} {
	try {
		const urlObj = new globalThis.URL(url, 'http://localhost'); // Use dummy base for relative URLs
		const params: Record<string, string> = {};

		// Extract all search parameters
		urlObj.searchParams.forEach((value, key) => {
			params[key] = value;
		});

		return {
			basePath: urlObj.pathname,
			params
		};
	} catch {
		// If URL parsing fails, treat as a simple path without parameters
		return {
			basePath: url,
			params: {}
		};
	}
}

/**
 * Encrypt parameters in a next URL and create secure URL
 *
 * @param nextUrl - Original next URL from backend
 * @param sessionTokens - Session tokens for encryption
 * @returns Encrypted URL with basePath?p=...
 */
export function encryptNextUrl(
	nextUrl: string,
	sessionTokens: {
		cipherToken: string;
		nonceToken: string;
		hmacKey: string;
	}
): string {
	const { basePath, params } = parseNextUrl(nextUrl);

	// If no parameters, return URL as-is
	if (Object.keys(params).length === 0) {
		return nextUrl;
	}

	// Encrypt parameters
	const { p } = prepareSecureUrlParams(params, sessionTokens);

	// Create new URL with compact encrypted parameter
	return `${basePath}?p=${p}`;
}

/**
 * Decrypt parameters from current page URL if encrypted
 *
 * @param searchParams - URLSearchParams from current page
 * @param sessionTokens - Session tokens for decryption
 * @returns Decrypted parameters or null if not encrypted/failed
 */
export function decryptPageParams(
	searchParams: URLSearchParams,
	sessionTokens: {
		cipherToken: string;
		nonceToken: string;
		hmacKey: string;
	}
): Record<string, any> | null {
	const p = searchParams.get('p');

	// Return null if not encrypted parameters
	if (!p) {
		return null;
	}

	try {
		// Decrypt parameters using compact parameter
		return decryptUrlParams(
			p,
			sessionTokens.cipherToken,
			sessionTokens.nonceToken,
			sessionTokens.hmacKey
		);
	} catch (error) {
		console.error('Failed to decrypt URL parameters:', error);
		return null;
	}
}

/**
 * Create encrypted URL for navigation with parameters
 *
 * @param basePath - Base path for the route (e.g., '/result', '/custom')
 * @param params - Parameters to encrypt and include
 * @param sessionTokens - Session tokens for encryption
 * @returns Full encrypted URL
 */
export function createEncryptedUrl(
	basePath: string,
	params: Record<string, any>,
	sessionTokens: {
		cipherToken: string;
		nonceToken: string;
		hmacKey: string;
	}
): string {
	// If no parameters, return simple base path
	if (!params || Object.keys(params).length === 0) {
		return basePath;
	}

	// Encrypt parameters
	const { p } = prepareSecureUrlParams(params, sessionTokens);

	// Create compact encrypted URL
	return `${basePath}?p=${p}`;
}
