/**
 * Crypto Storage Module - Prehash Seed Management
 *
 * Single Responsibility: Handle prehash seed storage/retrieval in IndexedDB with FIFO rotation
 * Part of crypto.ts refactorization to apply SOLID principles
 */

import { cryptoHashGen } from './crypto-core';
import { bytesToBase64Url, base64ToBytes, bytesToBase64 } from './crypto-encoding';

/**
 * Store prehash seed in IndexedDB and return key
 *
 * @param seed - 32-byte prehash seed
 * @param hmacKey - HMAC key for generating unique key
 * @returns Promise<Base64URL key for retrieval>
 */
export async function storePrehashSeed(seed: Uint8Array, hmacKey: string): Promise<string> {
	const { sessionManager } = await import('../session-manager');

	// Generate 8-byte key using cryptoHashGen
	const keyBytes = cryptoHashGen(seed, hmacKey, 8);
	const key = bytesToBase64Url(keyBytes);

	// Store in IndexedDB with FIFO management (max 20)
	const seedBase64 = bytesToBase64(seed);
	await sessionManager.addPrehashSeed(key, seedBase64);

	return key;
}

/**
 * Retrieve prehash seed from IndexedDB by key
 *
 * @param key - Base64URL key to find seed
 * @returns Promise<32-byte prehash seed or null if not found>
 */
export async function getPrehashSeed(key: string): Promise<Uint8Array | null> {
	const { sessionManager } = await import('../session-manager');

	const seedBase64 = await sessionManager.getPrehashSeed(key);
	if (!seedBase64) return null;

	return base64ToBytes(seedBase64);
}
