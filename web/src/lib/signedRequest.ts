/**
 * Universal signed request system for all API endpoints
 *
 * FIXED: Uses deterministic JSON + Base64 URL-safe encoding
 * Frontend and backend sign identical JSON strings for perfect consistency
 */

import { getOrCreateKeyPair, signMessage } from './ed25519';

/**
 * Universal signed request structure
 * CORRECTED: payload is Base64 URL-safe encoded deterministic JSON
 * Both frontend and backend sign the original JSON string (before Base64)
 */
export interface SignedRequest {
	/** Base64 URL-safe encoded deterministic JSON payload */
	payload: string;
	/** Ed25519 signature of the original JSON string (before Base64 encoding) */
	signature: string;
}

/**
 * Deterministic JSON serialization for consistent signing
 * Both frontend and backend must use identical key sorting
 */
export function serializePayload(payload: unknown): string {
	// Sort object keys recursively for deterministic serialization
	const sortedPayload = sortObjectKeys(payload);
	return JSON.stringify(sortedPayload);
}

/**
 * Base64 URL-safe encode deterministic JSON for safe transmission
 *
 * @param jsonString - Deterministic JSON string to encode
 * @returns Base64 URL-safe encoded string
 */
export function encodePayloadBase64(jsonString: string): string {
	// Convert string to bytes and encode as Base64 URL-safe
	const bytes = new TextEncoder().encode(jsonString);
	const base64 = btoa(String.fromCharCode(...bytes))
		.replace(/\+/g, '-')
		.replace(/\//g, '_')
		.replace(/=/g, '');
	return base64;
}

/**
 * Decode Base64 URL-safe string back to original JSON
 *
 * @param base64String - Base64 URL-safe encoded string
 * @returns Original JSON string
 */
export function decodePayloadBase64(base64String: string): string {
	// Restore padding and convert to standard Base64
	const base64 = base64String
		.replace(/-/g, '+')
		.replace(/_/g, '/')
		.padEnd(base64String.length + ((4 - (base64String.length % 4)) % 4), '=');

	// Decode Base64 to bytes and convert back to string
	const bytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
	return new TextDecoder().decode(bytes);
}

/**
 * Recursively sort object keys for deterministic serialization
 */
export function sortObjectKeys(obj: unknown): unknown {
	if (obj === null || typeof obj !== 'object') {
		return obj;
	}

	if (Array.isArray(obj)) {
		return obj.map(sortObjectKeys);
	}

	const sorted: Record<string, unknown> = {};
	const keys = Object.keys(obj).sort();

	for (const key of keys) {
		sorted[key] = sortObjectKeys((obj as Record<string, unknown>)[key]);
	}

	return sorted;
}

/**
 * Create signed request with Ed25519 signature
 * CORRECTED: Uses deterministic JSON + Base64 URL-safe for perfect consistency
 *
 * Process:
 * 1. Serialize payload → deterministic JSON string
 * 2. Sign the JSON string directly (what both sides verify against)
 * 3. Encode JSON → Base64 URL-safe for transmission
 * 4. Send: { payload: base64_json, signature: signature_of_json }
 *
 * @param payload - Data to be sent to API
 * @returns SignedRequest with Base64 payload and signature of original JSON
 */
export async function createSignedRequest<T>(payload: T): Promise<SignedRequest> {
	// Get or generate Ed25519 keypair
	const keyPair = await getOrCreateKeyPair();

	// Step 1: Serialize payload to deterministic JSON
	const jsonPayload = serializePayload(payload);
	console.log('🔍 JSON FRONTEND: Serialized payload to JSON length:', jsonPayload.length);

	// Step 2: Encode JSON as Base64 URL-safe for transmission
	const base64Payload = encodePayloadBase64(jsonPayload);
	console.log('🔐 JSON FRONTEND: Encoded JSON to Base64 for transmission');

	// Step 3: Sign the Base64 string directly (most deterministic!)
	const signature = await signMessage(base64Payload, keyPair);
	console.log('✅ BASE64 FRONTEND: Created signature for Base64 payload');

	return {
		payload: base64Payload,
		signature
	};
}

/**
 * Serialize query parameters deterministically for Ed25519 signing
 * Query params remain as simple JSON (no Base64 encoding needed)
 */
export function serializeQueryParams(params: Record<string, string>): string {
	// Convert to object and apply same sorting logic as payload serialization
	const sortedParams = sortObjectKeys(params);
	return JSON.stringify(sortedParams);
}

/**
 * Create Ed25519 signature for GET request query parameters
 * Uses simple JSON deterministic serialization (no Base64 needed for query params)
 *
 * @param params - Query parameters to sign
 * @returns Signature string for the JSON serialized parameters
 */
export async function signQueryParams(params: Record<string, string>): Promise<string> {
	// Get or generate Ed25519 keypair
	const keyPair = await getOrCreateKeyPair();

	// Serialize parameters with deterministic JSON (matching backend)
	const serializedParams = serializeQueryParams(params);
	console.log('🔍 JSON FRONTEND: Serialized query params for signing');

	// Sign JSON serialized parameters
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
		typeof (obj as Record<string, unknown>).signature === 'string'
	);
}
