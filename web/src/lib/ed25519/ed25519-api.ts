/**
 * Ed25519 API Module - High-Level API Functions
 *
 * Single Responsibility: Provide high-level API for Ed25519 operations
 * Part of ed25519.ts refactorization to apply SOLID principles
 */

import type { Ed25519KeyPair } from './ed25519-types';
import { generateEd25519KeyPair } from './ed25519-keygen';
import { getKeyPair, storeKeyPair } from './ed25519-database';

/**
 * Get or create Ed25519 key pair for current session
 * Returns existing key pair from IndexedDB or creates new one
 */
export async function getOrCreateKeyPair(): Promise<Ed25519KeyPair> {
	// Try to get existing key pair
	const existingKeyPair = await getKeyPair();
	if (existingKeyPair) {
		return existingKeyPair;
	}

	// Generate new key pair if none exists
	const newKeyPair = await generateEd25519KeyPair();
	await storeKeyPair(newKeyPair);

	// Generated new Ed25519 key pair for magic link authentication
	return newKeyPair;
}
