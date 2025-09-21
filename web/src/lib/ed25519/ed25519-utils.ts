/**
 * Ed25519 Utils Module - Conversion Utilities
 *
 * Single Responsibility: Handle key format conversions (hex/bytes)
 * Part of ed25519.ts refactorization to apply SOLID principles
 */

import { bytesToHex, hexToBytes } from '@noble/hashes/utils';

/**
 * Export public key as hex string for transmission
 */
export function publicKeyToHex(publicKeyBytes: Uint8Array): string {
	return bytesToHex(publicKeyBytes);
}

/**
 * Import public key from hex string
 */
export function publicKeyFromHex(hex: string): Uint8Array {
	return hexToBytes(hex);
}
