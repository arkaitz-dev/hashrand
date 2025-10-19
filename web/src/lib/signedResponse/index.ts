/**
 * Universal Signed Response Validation System
 *
 * Provides Ed25519 signature verification for all backend responses
 * CORRECTED: Uses deterministic JSON + Base64 URL-safe for perfect consistency
 */

// Re-export types
export type { SignedResponse } from './types';
export { SignedResponseError } from './types';

// Re-export validator
export { SignedResponseValidator } from './validation';

// Re-export parsing utilities
export { isSignedResponse } from './parsing';

// Re-export crypto utilities (for external use if needed)
export { isValidHexKey } from './crypto';

// Import for helper functions
import { SignedResponseValidator } from './validation';

/**
 * Helper functions for easier use
 */

/**
 * Validate signed response with automatic error handling
 *
 * @param responseData - Raw response data
 * @param serverPubKeyHex - Server's Ed25519 public key
 * @returns Promise with validated payload or throws
 */
export async function validateSignedResponse<T>(
	responseData: unknown,
	serverPubKeyHex: string
): Promise<T> {
	return SignedResponseValidator.validateSignedResponse<T>(responseData, serverPubKeyHex);
}

/**
 * Extract server public key from response
 *
 * @param responseData - Raw response data
 * @returns Server public key hex string or null
 */
export function extractServerPubKey(responseData: unknown): string | null {
	return SignedResponseValidator.extractServerPubKey(responseData);
}

/**
 * Extract server X25519 public key from response (E2E encryption)
 *
 * @param responseData - Raw response data
 * @returns Server X25519 public key hex string or null
 */
export function extractServerX25519PubKey(responseData: unknown): string | null {
	return SignedResponseValidator.extractServerX25519PubKey(responseData);
}
