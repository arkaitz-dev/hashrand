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
export function cryptoHashGen(data: string | Uint8Array, key: string | Uint8Array, outputLength: number): Uint8Array {
	// Convert inputs to Uint8Array if needed
	const dataBytes = typeof data === 'string' ? new TextEncoder().encode(data) : data;
	const keyBytes = typeof key === 'string' ? 
		new Uint8Array(atob(key).split('').map(char => char.charCodeAt(0))) : 
		key;

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
 * Generate a 32-byte prehash from parameters using cryptoHashGen
 * 
 * @param paramsString - Serialized parameters string
 * @param hmacKey - 32-byte HMAC key from session (base64 encoded)
 * @returns 32-byte prehash as Uint8Array
 */
export function generatePrehash(paramsString: string, hmacKey: string): Uint8Array {
	return cryptoHashGen(paramsString, hmacKey, 32);
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
 * Convert base64 string back to Uint8Array
 * 
 * @param base64 - base64 encoded string
 * @returns Uint8Array
 */
export function base64ToBytes(base64: string): Uint8Array {
	return new Uint8Array(
		atob(base64)
			.split('')
			.map(char => char.charCodeAt(0))
	);
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
 * Encrypt parameters for secure URL transmission
 * 
 * @param params - Parameters object to encrypt
 * @param cipherToken - Session cipher token (base64)
 * @param nonceToken - Session nonce token (base64)
 * @param hmacKey - Session HMAC key (base64)
 * @returns Encrypted parameters as base64 string
 */
export function encryptUrlParams(
	params: Record<string, any>, 
	cipherToken: string, 
	nonceToken: string, 
	hmacKey: string
): string {
	// 1. Serialize parameters
	const paramsString = serializeParams(params);
	
	// 2. Generate prehash
	const prehash = generatePrehash(paramsString, hmacKey);
	
	// 3. Generate cipher key and nonce for ChaCha20-Poly1305
	const cipherKey = generateCipherKey(cipherToken, prehash);
	const cipherNonce = generateCipherNonce(nonceToken, prehash);
	
	// 4. Encrypt with ChaCha20-Poly1305
	const cipher = chacha20poly1305(cipherKey, cipherNonce);
	const plaintext = new TextEncoder().encode(paramsString);
	const ciphertext = cipher.encrypt(plaintext);
	
	// 5. Return as base64
	return bytesToBase64(ciphertext);
}

/**
 * Decrypt parameters from secure URL transmission
 * 
 * @param encryptedParams - Encrypted parameters as base64 string
 * @param cipherToken - Session cipher token (base64)
 * @param nonceToken - Session nonce token (base64)
 * @param hmacKey - Session HMAC key (base64)
 * @returns Decrypted parameters object
 */
export function decryptUrlParams(
	encryptedParams: string,
	cipherToken: string,
	nonceToken: string,
	hmacKey: string
): Record<string, any> {
	// We need the original paramsString to generate the same prehash
	// This function would be used when we know the expected structure
	// For now, we'll implement a version that works with known context
	
	throw new Error('decryptUrlParams requires the original paramsString for prehash generation');
}

/**
 * Complete URL parameter encryption workflow
 * 
 * @param params - Parameters to encrypt
 * @param sessionTokens - Session tokens from authStore
 * @returns Object with encrypted data and metadata for URL
 */
export function prepareSecureUrlParams(
	params: Record<string, any>,
	sessionTokens: {
		cipherToken: string;
		nonceToken: string;
		hmacKey: string;
	}
): {
	encrypted: string;
	prehash: string; // For verification/reconstruction
} {
	const paramsString = serializeParams(params);
	const prehash = generatePrehash(paramsString, sessionTokens.hmacKey);
	
	const encrypted = encryptUrlParams(
		params,
		sessionTokens.cipherToken,
		sessionTokens.nonceToken,
		sessionTokens.hmacKey
	);
	
	return {
		encrypted,
		prehash: bytesToBase64(prehash)
	};
}