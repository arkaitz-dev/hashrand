/**
 * Cryptographic utilities for Ed25519 signature verification
 */

import { ed25519 } from '@noble/curves/ed25519.js';

/**
 * Convert hex string to Uint8Array (DRY utility)
 *
 * @param hex - Hex string to convert
 * @returns Uint8Array of bytes
 */
export function hexToBytes(hex: string): Uint8Array {
	return new Uint8Array(hex.match(/.{2}/g)?.map((byte) => parseInt(byte, 16)) || []);
}

/**
 * Verify Ed25519 signature using @noble/curves
 *
 * @param message - Message that was signed
 * @param signatureHex - Ed25519 signature as hex string
 * @param publicKeyHex - Ed25519 public key as hex string
 * @returns True if signature is valid
 */
export async function verifyEd25519Signature(
	message: string,
	signatureHex: string,
	publicKeyHex: string
): Promise<boolean> {
	try {
		// Convert message to bytes
		const messageBytes = new TextEncoder().encode(message);

		// Convert hex strings to bytes (DRY utility)
		const signatureBytes = hexToBytes(signatureHex);
		const publicKeyBytes = hexToBytes(publicKeyHex);

		// Verify signature using @noble/curves Ed25519
		return ed25519.verify(signatureBytes, messageBytes, publicKeyBytes);
	} catch {
		return false;
	}
}

/**
 * Validate hex string format and length
 *
 * @param hex - Hex string to validate
 * @param expectedLength - Expected length in characters
 * @returns True if valid hex string with expected length
 */
export function isValidHexKey(hex: string, expectedLength: number): boolean {
	return typeof hex === 'string' && hex.length === expectedLength && /^[0-9a-fA-F]+$/.test(hex);
}
