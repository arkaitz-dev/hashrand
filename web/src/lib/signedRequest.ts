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
export interface SignedRequest<T = any> {
	payload: T;
	signature: string;
}

/**
 * Deterministic JSON serialization for consistent signing
 * Ensures frontend and backend serialize identically
 */
export function serializePayload(payload: any): string {
	// Sort object keys recursively for deterministic serialization
	const sortedPayload = sortObjectKeys(payload);
	return JSON.stringify(sortedPayload);
}

/**
 * Recursively sort object keys for deterministic serialization
 */
function sortObjectKeys(obj: any): any {
	if (obj === null || typeof obj !== 'object') {
		return obj;
	}

	if (Array.isArray(obj)) {
		return obj.map(sortObjectKeys);
	}

	const sorted: any = {};
	const keys = Object.keys(obj).sort();

	for (const key of keys) {
		sorted[key] = sortObjectKeys(obj[key]);
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
 * Verify signed request structure (client-side validation)
 */
export function isSignedRequest(obj: any): obj is SignedRequest {
	return (
		obj &&
		typeof obj === 'object' &&
		'payload' in obj &&
		'signature' in obj &&
		typeof obj.signature === 'string'
	);
}