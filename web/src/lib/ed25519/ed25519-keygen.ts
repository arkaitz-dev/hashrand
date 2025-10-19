/**
 * Ed25519 Key Generation Module - Cryptographic Key Generation
 *
 * Single Responsibility: Handle Ed25519 key pair generation using Noble curves
 * Part of ed25519.ts refactorization to apply SOLID principles
 *
 * Architecture Decision: Uses Noble curves exclusively (not WebCrypto)
 * Rationale: E2E encryption (ECDH) requires extractable private key bytes
 * WebCrypto's non-extractable keys incompatible with ChaCha20-Poly1305 encryption
 */

import { ed25519 } from '@noble/curves/ed25519';
import type { Ed25519KeyPair } from './ed25519-types';

/**
 * Generate Ed25519 key pair using Noble curves
 *
 * Returns keypair with extractable privateKeyBytes for:
 * - Ed25519 signatures (authentication)
 * - Ed25519 → X25519 conversion (E2E encryption via ECDH)
 *
 * Security: Private key managed exclusively in IndexedDB, never transmitted
 */
export async function generateEd25519KeyPair(): Promise<Ed25519KeyPair> {
	// Generate random private key (32 bytes)
	const privateKeyBytes = crypto.getRandomValues(new Uint8Array(32));
	const publicKeyBytes = ed25519.getPublicKey(privateKeyBytes);

	// Return Noble-based keypair (consistent architecture for signatures + E2E encryption)
	return {
		publicKey: null, // Not using WebCrypto
		privateKey: null, // Not using WebCrypto
		publicKeyBytes: new Uint8Array(publicKeyBytes),
		privateKeyBytes: privateKeyBytes,
		isNoble: true
	};
}
