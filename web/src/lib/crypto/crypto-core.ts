/**
 * Crypto Core Module - Basic Cryptographic Functions
 *
 * Single Responsibility: Core cryptographic hash generation and key derivation
 * Part of crypto.ts refactorization to apply SOLID principles
 */

import { blake2b } from '@noble/hashes/blake2.js';
import { rngChacha8 } from '@noble/ciphers/chacha.js';

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
