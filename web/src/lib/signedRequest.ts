/**
 * Universal signed request system for all API endpoints
 *
 * Provides deterministic JSON serialization and Ed25519 signing
 * for secure communication with backend API
 */

import { getOrCreateKeyPair, signMessage } from './ed25519';

/**
 * Universal signed request structure
 */
export interface SignedRequest<T = unknown> {
	payload: T;
	signature: string;
}

/**
 * Deterministic JSON serialization for consistent signing
 * Ensures frontend and backend serialize identically
 */
export function serializePayload(payload: unknown): string {
	// Sort object keys recursively for deterministic serialization
	const sortedPayload = sortObjectKeys(payload);
	return JSON.stringify(sortedPayload);
}

/**
 * Recursively sort object keys for deterministic serialization
 */
function sortObjectKeys(obj: unknown): unknown {
	if (obj === null || typeof obj !== 'object') {
		return obj;
	}

	if (Array.isArray(obj)) {
		return obj.map(sortObjectKeys);
	}

	const sorted: Record<string, unknown> = {};
	const keys = Object.keys(obj).sort();

	for (const key of keys) {
		sorted[key] = sortObjectKeys((obj as any)[key]);
	}

	return sorted;
}

/**
 * Create signed request with Ed25519 signature
 *
 * @param payload - Data to be sent to API
 * @returns SignedRequest with payload and signature
 */
export async function createSignedRequest<T>(payload: T): Promise<SignedRequest<T>> {
	// Get or generate Ed25519 keypair
	const keyPair = await getOrCreateKeyPair();

	// Serialize payload deterministically
	const serializedPayload = serializePayload(payload);

	// Sign serialized payload
	const signature = await signMessage(serializedPayload, keyPair);

	// Created signed request

	return {
		payload,
		signature
	};
}

/**
 * Serialize query parameters deterministically for Ed25519 signing
 *
 * Converts query parameters to a deterministic JSON string matching
 * the backend's serialize_query_params_deterministic function
 */
export function serializeQueryParams(params: Record<string, string>): string {
	// Convert to object and apply same sorting logic as payload serialization
	const sortedParams = sortObjectKeys(params);
	return JSON.stringify(sortedParams);
}

/**
 * Create Ed25519 signature for GET request query parameters
 *
 * @param params - Query parameters to sign
 * @returns Signature string for the serialized parameters
 */
export async function signQueryParams(params: Record<string, string>): Promise<string> {
	// Get or generate Ed25519 keypair
	const keyPair = await getOrCreateKeyPair();

	// Serialize parameters deterministically (matching backend)
	const serializedParams = serializeQueryParams(params);

	// Sign serialized parameters
	return await signMessage(serializedParams, keyPair);
}

/**
 * Verify signed request structure (client-side validation)
 */
export function isSignedRequest(obj: unknown): obj is SignedRequest {
	return (
		obj !== null &&
		typeof obj === 'object' &&
		'payload' in obj &&
		'signature' in obj &&
		typeof (obj as any).signature === 'string'
	);
}
