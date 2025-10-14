/**
 * Ed25519 Core Module - Universal Cryptographic Operations
 *
 * Single Responsibility: Pure Ed25519 operations with ZERO browser dependencies
 * Can be used in Node.js, Deno, Browser, Bun, Playwright tests, etc.
 *
 * Part of SOLID refactoring for E2E testing compatibility
 */

import { ed25519 } from '@noble/curves/ed25519.js';
import { bytesToHex, hexToBytes } from '@noble/hashes/utils.js';
import { base58 } from '@scure/base';
import type { Ed25519KeyPair } from './ed25519-types';

/**
 * Generate Ed25519 keypair using Noble curves (universal)
 * Works in any JavaScript runtime (browser, Node.js, Deno, Bun)
 *
 * @returns Ed25519KeyPair with Noble curves implementation
 */
export function generateKeyPairNoble(): Ed25519KeyPair {
	const privateKeyBytes = new Uint8Array(32);

	// Use crypto.getRandomValues if available (browser/Node 20+)
	if (typeof crypto !== 'undefined' && crypto.getRandomValues) {
		crypto.getRandomValues(privateKeyBytes);
	} else {
		// Fallback for older Node.js (should not happen in practice)
		throw new Error('crypto.getRandomValues not available');
	}

	const publicKeyBytes = ed25519.getPublicKey(privateKeyBytes);

	return {
		publicKey: null,
		privateKey: null,
		publicKeyBytes: new Uint8Array(publicKeyBytes),
		privateKeyBytes: privateKeyBytes,
		isNoble: true
	};
}

/**
 * Convert signature bytes to base58 string (Bitcoin alphabet)
 *
 * @param signatureBytes - Ed25519 signature (64 bytes)
 * @returns Base58-encoded signature (~88 chars, 31% shorter than hex)
 */
export function signatureBytesToBase58(signatureBytes: Uint8Array): string {
	if (signatureBytes.length !== 64) {
		throw new Error(`Invalid Ed25519 signature length: ${signatureBytes.length}, expected 64`);
	}
	return base58.encode(signatureBytes);
}

/**
 * Convert base58 string back to signature bytes
 *
 * @param base58Signature - Base58-encoded signature
 * @returns Ed25519 signature bytes (64 bytes)
 */
export function signatureBase58ToBytes(base58Signature: string): Uint8Array {
	const bytes = base58.decode(base58Signature);
	if (bytes.length !== 64) {
		throw new Error(
			`Invalid Ed25519 signature length after base58 decode: ${bytes.length}, expected 64`
		);
	}
	return bytes;
}

/**
 * Sign message with Ed25519 keypair (universal)
 *
 * Uses Noble curves exclusively for maximum compatibility
 *
 * @param message - Message to sign (string or bytes)
 * @param keyPair - Ed25519 keypair with privateKeyBytes
 * @returns Signature as base58 string (~88 chars, 31% shorter than hex 128 chars)
 */
export function signMessageWithKeyPair(
	message: string | Uint8Array,
	keyPair: Ed25519KeyPair
): string {
	const messageBytes = typeof message === 'string' ? new TextEncoder().encode(message) : message;

	if (!keyPair.privateKeyBytes) {
		throw new Error('Private key bytes required for signing');
	}

	const signature = ed25519.sign(messageBytes, keyPair.privateKeyBytes);
	// return bytesToHex(signature);
	return signatureBytesToBase58(signature);
}

/**
 * Verify Ed25519 signature (universal)
 *
 * @param message - Original message (string or bytes)
 * @param signatureBase58 - Signature as base58 string (~88 chars)
 * @param publicKeyBytes - Ed25519 public key (32 bytes)
 * @returns True if signature is valid
 */
export function verifySignatureWithPublicKey(
	message: string | Uint8Array,
	signatureBase58: string,
	publicKeyBytes: Uint8Array
): boolean {
	if (publicKeyBytes.length !== 32) {
		throw new Error(`Invalid Ed25519 public key length: ${publicKeyBytes.length}, expected 32`);
	}

	const messageBytes = typeof message === 'string' ? new TextEncoder().encode(message) : message;

	// const signatureBytes = hexToBytes(signatureHex);
	try {
		const signatureBytes = signatureBase58ToBytes(signatureBase58);

		if (signatureBytes.length !== 64) {
			return false; // Invalid signature length
		}

		return ed25519.verify(signatureBytes, messageBytes, publicKeyBytes);
	} catch {
		// Base58 decode failed or invalid signature format
		return false;
	}
}

/**
 * Convert hex string to Ed25519 keypair (for loading from storage)
 *
 * @param privateKeyHex - Private key as hex string (64 chars)
 * @param publicKeyHex - Public key as hex string (64 chars)
 * @returns Ed25519KeyPair with Noble implementation
 */
export function keyPairFromHex(privateKeyHex: string, publicKeyHex: string): Ed25519KeyPair {
	if (privateKeyHex.length !== 64) {
		throw new Error(`Invalid private key hex length: ${privateKeyHex.length}, expected 64`);
	}

	if (publicKeyHex.length !== 64) {
		throw new Error(`Invalid public key hex length: ${publicKeyHex.length}, expected 64`);
	}

	const privateKeyBytes = hexToBytes(privateKeyHex);
	const publicKeyBytes = hexToBytes(publicKeyHex);

	return {
		publicKey: null,
		privateKey: null,
		publicKeyBytes,
		privateKeyBytes,
		isNoble: true
	};
}

/**
 * Convert Ed25519 keypair to hex strings (for storage)
 *
 * @param keyPair - Ed25519 keypair
 * @returns Object with privateKeyHex and publicKeyHex
 */
export function keyPairToHex(keyPair: Ed25519KeyPair): {
	privateKeyHex: string;
	publicKeyHex: string;
} {
	if (!keyPair.privateKeyBytes) {
		throw new Error('Private key bytes required');
	}

	return {
		privateKeyHex: bytesToHex(keyPair.privateKeyBytes),
		publicKeyHex: bytesToHex(keyPair.publicKeyBytes)
	};
}

/**
 * Convert public key bytes to hex string (utility)
 *
 * @param publicKeyBytes - Public key as Uint8Array (32 bytes)
 * @returns Hex string (64 chars)
 */
export function publicKeyBytesToHex(publicKeyBytes: Uint8Array): string {
	if (publicKeyBytes.length !== 32) {
		throw new Error(`Invalid public key length: ${publicKeyBytes.length}, expected 32`);
	}

	return bytesToHex(publicKeyBytes);
}

/**
 * Convert private key bytes to hex string (utility)
 *
 * @param privateKeyBytes - Private key as Uint8Array (32 bytes)
 * @returns Hex string (64 chars)
 */
export function privateKeyBytesToHex(privateKeyBytes: Uint8Array): string {
	if (privateKeyBytes.length !== 32) {
		throw new Error(`Invalid private key length: ${privateKeyBytes.length}, expected 32`);
	}

	return bytesToHex(privateKeyBytes);
}
