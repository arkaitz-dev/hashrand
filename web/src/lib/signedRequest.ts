/**
 * Universal signed request system for all API endpoints
 *
 * Browser implementation using signMessage() for WebCrypto/Noble compatibility
 * Part of SOLID refactoring for E2E testing compatibility
 */

import { getOrCreateKeyPair, signMessage } from './ed25519';
import {
	serializePayload,
	encodePayloadBase64,
	serializeQueryParams
} from './crypto/signedRequest-core';

// Re-export core types and utilities for E2E testing compatibility
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
 * Create signed request with Ed25519 signature (Browser implementation)
 *
 * Uses signMessage() which supports both WebCrypto and Noble keypairs
 * This is the production browser implementation
 *
 * @param payload - Data to be sent to API
 * @returns SignedRequest with Base64 payload and signature
 */
export async function createSignedRequest<T>(
	payload: T
): Promise<import('./crypto/signedRequest-core').SignedRequest> {
	// Get or generate Ed25519 keypair from IndexedDB
	const keyPair = await getOrCreateKeyPair();

	// Step 1: Serialize payload to deterministic JSON
	const jsonPayload = serializePayload(payload);

	// Step 2: Encode JSON as Base64 URL-safe for transmission
	const base64Payload = encodePayloadBase64(jsonPayload);

	// Step 3: Sign the Base64 string using signMessage (supports WebCrypto + Noble)
	const signature = await signMessage(base64Payload, keyPair);

	return {
		payload: base64Payload,
		signature
	};
}

/**
 * Sign query parameters with Ed25519 (Browser implementation)
 *
 * Uses signMessage() which supports both WebCrypto and Noble keypairs
 *
 * @param params - Query parameters to sign
 * @returns Signature string for the JSON serialized parameters
 */
export async function signQueryParams(params: Record<string, string>): Promise<string> {
	// Get or generate Ed25519 keypair from IndexedDB
	const keyPair = await getOrCreateKeyPair();

	// Serialize parameters with deterministic JSON
	const serializedParams = serializeQueryParams(params);

	// Sign using signMessage (supports WebCrypto + Noble)
	return await signMessage(serializedParams, keyPair);
}
