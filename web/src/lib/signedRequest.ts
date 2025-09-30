/**
 * Universal signed request system for all API endpoints
 *
 * REFACTORED: Uses pure core functions from crypto/signedRequest-core
 * Browser wrapper that gets keypair from IndexedDB
 *
 * Part of SOLID refactoring for E2E testing compatibility
 */

import { getOrCreateKeyPair } from './ed25519';
import {
	createSignedRequestWithKeyPair,
	signQueryParamsWithKeyPair
} from './crypto/signedRequest-core';

// Re-export core types and utilities for backward compatibility
export type { SignedRequest } from './crypto/signedRequest-core';
export {
	sortObjectKeys,
	serializePayload,
	encodePayloadBase64,
	decodePayloadBase64,
	serializeQueryParams,
	isSignedRequest,
	createSignedRequestWithKeyPair,
	signQueryParamsWithKeyPair
} from './crypto/signedRequest-core';

/**
 * Create signed request with Ed25519 signature (Browser wrapper)
 *
 * Gets keypair from IndexedDB and delegates to pure core function
 *
 * @param payload - Data to be sent to API
 * @returns SignedRequest with Base64 payload and signature
 */
export async function createSignedRequest<T>(
	payload: T
): Promise<import('./crypto/signedRequest-core').SignedRequest> {
	// Get or generate Ed25519 keypair from IndexedDB
	const keyPair = await getOrCreateKeyPair();

	// Delegate to pure core function
	const signedRequest = createSignedRequestWithKeyPair(payload, keyPair);

	// Log for debugging
	console.log('✅ BASE64 FRONTEND: Created signed request');

	return signedRequest;
}

/**
 * Sign query parameters with Ed25519 (Browser wrapper)
 *
 * Gets keypair from IndexedDB and delegates to pure core function
 *
 * @param params - Query parameters to sign
 * @returns Signature string for the JSON serialized parameters
 */
export async function signQueryParams(params: Record<string, string>): Promise<string> {
	// Get or generate Ed25519 keypair from IndexedDB
	const keyPair = await getOrCreateKeyPair();

	console.log('🔍 JSON FRONTEND: Serialized query params for signing');

	// Delegate to pure core function
	return signQueryParamsWithKeyPair(params, keyPair);
}
