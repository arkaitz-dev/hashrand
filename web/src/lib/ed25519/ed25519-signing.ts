/**
 * Ed25519 Signing Module - Digital Signature Operations
 *
 * Single Responsibility: Handle Ed25519 message signing and verification
 * Part of ed25519.ts refactorization to apply SOLID principles
 */

import { ed25519 } from '@noble/curves/ed25519';
import { hexToBytes } from '@noble/hashes/utils';
import { signatureBytesToBase58 } from './ed25519-core';
import type { Ed25519KeyPair } from './ed25519-types';

/**
 * Sign message using Ed25519 private key
 * @param message - Message to sign (string or bytes)
 * @param privateKey - Ed25519 private key (CryptoKey)
 * @returns Signature as base58 string (~88 chars, 31% shorter than hex)
 */
export async function signMessage(
	message: string | Uint8Array,
	keyPair: Ed25519KeyPair
): Promise<string> {
	const messageBytes = typeof message === 'string' ? new TextEncoder().encode(message) : message;

	if (keyPair.isNoble && keyPair.privateKeyBytes) {
		// Use Noble curves for signing
		// Signing with Noble curves
		const signature = ed25519.sign(new Uint8Array(messageBytes), keyPair.privateKeyBytes);
		return signatureBytesToBase58(signature);
	} else if (keyPair.privateKey) {
		// Use WebCrypto for signing
		// Signing with WebCrypto
		try {
			const signature = await crypto.subtle.sign(
				'Ed25519',
				keyPair.privateKey,
				new Uint8Array(messageBytes)
			);
			return signatureBytesToBase58(new Uint8Array(signature));
		} catch (error) {
			throw new Error(`WebCrypto signing failed: ${error}`);
		}
	} else {
		throw new Error('No valid private key available for signing');
	}
}

/**
 * Verify Ed25519 signature
 * @param message - Original message (string or bytes)
 * @param signature - Signature as hex string
 * @param publicKeyBytes - Ed25519 public key (32 bytes)
 * @returns True if signature is valid
 */
export async function verifySignature(
	message: string | Uint8Array,
	signature: string,
	publicKeyBytes: Uint8Array
): Promise<boolean> {
	if (publicKeyBytes.length !== 32) {
		throw new Error(`Invalid Ed25519 public key length: ${publicKeyBytes.length}, expected 32`);
	}

	const messageBytes = typeof message === 'string' ? new TextEncoder().encode(message) : message;

	const signatureBytes = hexToBytes(signature);

	if (signatureBytes.length !== 64) {
		throw new Error(`Invalid Ed25519 signature length: ${signatureBytes.length}, expected 64`);
	}

	try {
		// Try WebCrypto verification first
		const publicKey = await crypto.subtle.importKey(
			'raw',
			new Uint8Array(publicKeyBytes),
			{ name: 'Ed25519', namedCurve: 'Ed25519' },
			false,
			['verify']
		);

		return await crypto.subtle.verify(
			'Ed25519',
			publicKey,
			new Uint8Array(signatureBytes),
			new Uint8Array(messageBytes)
		);
	} catch {
		// Fallback to Noble curves
		return ed25519.verify(signatureBytes, messageBytes, publicKeyBytes);
	}
}
