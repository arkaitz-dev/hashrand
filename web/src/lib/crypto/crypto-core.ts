/**
 * Crypto Core Module - Basic Cryptographic Functions
 *
 * Single Responsibility: Core cryptographic hash generation and key derivation
 * Part of crypto.ts refactorization to apply SOLID principles
 */

import { blake3KeyedVariable } from './blake3-keyed-variable';

/**
 * Generic cryptographic hash generator using Blake3 keyed variable (100% backend compatible)
 *
 * @param data - Input data (string or Uint8Array)
 * @param key - 64-byte key for Blake3 (string base64 or Uint8Array)
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

	// Validate 64-byte key requirement (same as backend)
	if (keyBytes.length !== 64) {
		throw new Error(`cryptoHashGen requires 64-byte key, got ${keyBytes.length} bytes`);
	}

	// Use Blake3 keyed variable (100% compatible with backend)
	return blake3KeyedVariable(keyBytes, dataBytes, outputLength);
}

/**
 * Generate a 64-byte prehash from prehash seed using cryptoHashGen
 *
 * @param prehashSeed - 32-byte prehash seed
 * @param hmacKey - 64-byte HMAC key from session (base64 encoded)
 * @returns 64-byte prehash as Uint8Array
 */
export function generatePrehash(prehashSeed: Uint8Array, hmacKey: string): Uint8Array {
	return cryptoHashGen(prehashSeed, hmacKey, 64);
}

/**
 * Generate ChaCha-Poly cipher key using session cipher token and prehash
 *
 * @param cipherToken - Session cipher token (base64 encoded, 64 bytes)
 * @param prehash - 64-byte prehash as key
 * @returns 32-byte cipher key for ChaCha-Poly
 */
export function generateCipherKey(cipherToken: string, prehash: Uint8Array): Uint8Array {
	return cryptoHashGen(cipherToken, prehash, 32);
}

/**
 * Generate ChaCha-Poly nonce using session nonce token and prehash
 *
 * @param nonceToken - Session nonce token (base64 encoded, 64 bytes)
 * @param prehash - 64-byte prehash as key
 * @returns 12-byte nonce for ChaCha-Poly
 */
export function generateCipherNonce(nonceToken: string, prehash: Uint8Array): Uint8Array {
	return cryptoHashGen(nonceToken, prehash, 12);
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
