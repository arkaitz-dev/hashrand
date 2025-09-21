/**
 * Session Crypto Module - Cryptographic Token Management
 *
 * Single Responsibility: Handle crypto tokens and prehash seeds management
 * Part of session-manager.ts refactorization to apply SOLID principles
 */

import { sessionDB } from './session-db';

/**
 * Get crypto tokens for URL encryption
 */
export async function getCryptoTokens(): Promise<{
	cipher: string | null;
	nonce: string | null;
	hmac: string | null;
}> {
	const session = await sessionDB.getSession();
	return {
		cipher: session.cipher_token,
		nonce: session.nonce_token,
		hmac: session.hmac_key
	};
}

/**
 * Set crypto tokens for URL encryption
 */
export async function setCryptoTokens(cipher: string, nonce: string, hmac: string): Promise<void> {
	await sessionDB.updateSession({
		cipher_token: cipher,
		nonce_token: nonce,
		hmac_key: hmac
	});
}

/**
 * Check if crypto tokens exist
 */
export async function hasCryptoTokens(): Promise<boolean> {
	const tokens = await getCryptoTokens();
	return !!(tokens.cipher && tokens.nonce && tokens.hmac);
}

/**
 * Add prehash seed to FIFO store
 */
export async function addPrehashSeed(key: string, prehashSeed: string): Promise<void> {
	const session = await sessionDB.getSession();

	// Add new seed
	session.prehashSeeds.push({
		key,
		prehashSeed,
		timestamp: Date.now()
	});

	// Enforce FIFO limit of 20
	if (session.prehashSeeds.length > 20) {
		session.prehashSeeds = session.prehashSeeds.slice(-20);
	}

	await sessionDB.saveSession(session);
}

/**
 * Get prehash seed by key
 */
export async function getPrehashSeed(key: string): Promise<string | null> {
	const session = await sessionDB.getSession();
	const seedEntry = session.prehashSeeds.find((entry) => entry.key === key);
	return seedEntry ? seedEntry.prehashSeed : null;
}
