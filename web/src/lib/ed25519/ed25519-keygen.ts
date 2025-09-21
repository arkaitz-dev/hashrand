/**
 * Ed25519 Key Generation Module - Cryptographic Key Generation
 *
 * Single Responsibility: Handle Ed25519 key pair generation with WebCrypto and Noble fallback
 * Part of ed25519.ts refactorization to apply SOLID principles
 */

import { ed25519 } from '@noble/curves/ed25519';
import type { Ed25519KeyPair } from './ed25519-types';

/**
 * Generate Ed25519 key pair using Web Crypto API
 * Private key is non-extractable for security
 */
export async function generateEd25519KeyPair(): Promise<Ed25519KeyPair> {
	// Check if Web Crypto API supports Ed25519
	if (!('subtle' in crypto)) {
		throw new Error('Web Crypto API not available');
	}

	try {
		// Generate key pair using Web Crypto API (non-extractable private key)
		const keyPair = await crypto.subtle.generateKey(
			{
				name: 'Ed25519',
				namedCurve: 'Ed25519'
			},
			false, // extractable: false - Private key cannot be extracted as raw bytes
			['sign', 'verify']
		);

		// Export public key for serialization/transmission
		const publicKeyRaw = await crypto.subtle.exportKey('raw', keyPair.publicKey);
		const publicKeyBytes = new Uint8Array(publicKeyRaw);

		if (publicKeyBytes.length !== 32) {
			throw new Error(`Invalid Ed25519 public key length: ${publicKeyBytes.length}, expected 32`);
		}

		return {
			publicKey: keyPair.publicKey,
			privateKey: keyPair.privateKey,
			publicKeyBytes
		};
	} catch {
		// Fallback to Noble curves if WebCrypto Ed25519 not supported
		return generateEd25519KeyPairFallback();
	}
}

/**
 * Fallback Ed25519 key generation using Noble curves
 * Used when WebCrypto doesn't support Ed25519
 */
export async function generateEd25519KeyPairFallback(): Promise<Ed25519KeyPair> {
	// Using Noble curves fallback (WebCrypto not supported)

	// Generate random private key (32 bytes)
	const privateKeyBytes = crypto.getRandomValues(new Uint8Array(32));
	const publicKeyBytes = ed25519.getPublicKey(privateKeyBytes);

	// Return Noble-based keypair (no WebCrypto dependency)
	return {
		publicKey: null, // Not using WebCrypto
		privateKey: null, // Not using WebCrypto
		publicKeyBytes: new Uint8Array(publicKeyBytes),
		privateKeyBytes: privateKeyBytes,
		isNoble: true
	};
}
