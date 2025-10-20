/**
 * Ed25519 API Module - High-Level API Functions
 *
 * Single Responsibility: Provide high-level API for Ed25519 operations
 * Part of ed25519.ts refactorization to apply SOLID principles
 *
 * UPDATED (v1.9.0): Now uses WebCrypto keypair generation
 */

import type { Ed25519KeyPair } from './ed25519-types';
import { getKeyPair } from './ed25519-database';

/**
 * Get or create Ed25519 key pair for current session
 * Returns existing key pair from IndexedDB or creates new one
 *
 * UPDATED (v1.9.0): Now generates using WebCrypto if keys don't exist
 */
export async function getOrCreateKeyPair(): Promise<Ed25519KeyPair> {
	// Try to get existing key pair from IndexedDB (new WebCrypto storage)
	const existingKeyPair = await getKeyPair();
	if (existingKeyPair) {
		return existingKeyPair;
	}

	// Generate new WebCrypto keypairs if none exist
	const { generateKeypairs } = await import('../crypto/keypair-generation');
	const { storeKeypairs } = await import('../crypto/keypair-storage');

	const newKeypairs = await generateKeypairs();
	await storeKeypairs(newKeypairs);

	// Convert to Ed25519KeyPair format for compatibility
	const publicKeyBuffer = await crypto.subtle.exportKey('raw', newKeypairs.ed25519.publicKey);
	const publicKeyBytes = new Uint8Array(publicKeyBuffer);

	return {
		publicKey: newKeypairs.ed25519.publicKey,
		privateKey: newKeypairs.ed25519.privateKey,
		publicKeyBytes,
		privateKeyBytes: undefined, // Non-extractable
		isNoble: false
	};
}
