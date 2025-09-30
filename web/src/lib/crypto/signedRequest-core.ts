/**
 * SignedRequest Core Module - Universal Signed Request Creation
 *
 * Single Responsibility: Pure SignedRequest creation with ZERO storage dependencies
 * Can be used in Node.js, Playwright tests, and browser environments
 *
 * Part of SOLID refactoring for E2E testing compatibility
 */

import { signMessageWithKeyPair } from '../ed25519/ed25519-core';
import type { Ed25519KeyPair } from '../ed25519/ed25519-types';

/**
 * Universal SignedRequest structure
 */
export interface SignedRequest {
	/** Base64 URL-safe encoded deterministic JSON payload */
	payload: string;
	/** Ed25519 signature of the payload */
	signature: string;
}

/**
 * Recursively sort object keys for deterministic serialization
 * CRITICAL: Must match backend sortObjectKeys logic exactly
 *
 * @param obj - Object to sort
 * @returns Sorted object
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
 * Deterministic JSON serialization for consistent signing
 * CRITICAL: Must match backend serialization exactly
 *
 * @param payload - Payload to serialize
 * @returns Deterministic JSON string
 */
export function serializePayload(payload: unknown): string {
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
	const base64 = base64String
		.replace(/-/g, '+')
		.replace(/_/g, '/')
		.padEnd(base64String.length + ((4 - (base64String.length % 4)) % 4), '=');

	const bytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
	return new TextDecoder().decode(bytes);
}

/**
 * Create signed request with provided keypair (UNIVERSAL - Pure Function)
 *
 * This is the core function that E2E tests can use directly
 * No browser dependencies, no IndexedDB, no side effects
 *
 * @param payload - Data to send (will be signed)
 * @param keyPair - Ed25519 keypair for signing
 * @returns SignedRequest with payload and signature
 */
export function createSignedRequestWithKeyPair<T>(
	payload: T,
	keyPair: Ed25519KeyPair
): SignedRequest {
	// Step 1: Serialize to deterministic JSON
	const jsonPayload = serializePayload(payload);

	// Step 2: Encode JSON as Base64 URL-safe for transmission
	const base64Payload = encodePayloadBase64(jsonPayload);

	// Step 3: Sign the Base64 string directly (matching backend behavior)
	const signature = signMessageWithKeyPair(base64Payload, keyPair);

	return {
		payload: base64Payload,
		signature
	};
}

/**
 * Serialize query parameters deterministically for Ed25519 signing
 * CRITICAL: Must match backend query param serialization
 *
 * @param params - Query parameters object
 * @returns Deterministic JSON string
 */
export function serializeQueryParams(params: Record<string, string>): string {
	const sortedParams = sortObjectKeys(params);
	return JSON.stringify(sortedParams);
}

/**
 * Sign query parameters with provided keypair (UNIVERSAL - Pure Function)
 *
 * This is the core function that E2E tests can use directly
 * No browser dependencies, no IndexedDB, no side effects
 *
 * @param params - Query parameters to sign
 * @param keyPair - Ed25519 keypair for signing
 * @returns Signature string for the serialized parameters
 */
export function signQueryParamsWithKeyPair(
	params: Record<string, string>,
	keyPair: Ed25519KeyPair
): string {
	const serializedParams = serializeQueryParams(params);
	return signMessageWithKeyPair(serializedParams, keyPair);
}

/**
 * Verify signed request structure (client-side validation)
 *
 * @param obj - Object to check
 * @returns True if obj is a valid SignedRequest structure
 */
export function isSignedRequest(obj: unknown): obj is SignedRequest {
	return (
		obj !== null &&
		typeof obj === 'object' &&
		'payload' in obj &&
		'signature' in obj &&
		typeof (obj as Record<string, unknown>).payload === 'string' &&
		typeof (obj as Record<string, unknown>).signature === 'string'
	);
}
