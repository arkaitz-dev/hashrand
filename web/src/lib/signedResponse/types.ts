/**
 * Type definitions for signed response system
 */

/**
 * Universal signed response structure from backend
 * payload is Base64 URL-safe encoded deterministic JSON
 * Signature verifies the original JSON string (before Base64 encoding)
 */
export interface SignedResponse<T = unknown> {
	/** Base64 URL-safe encoded deterministic JSON payload */
	payload: string;
	/** Ed25519 signature of the original JSON string (before Base64 encoding) */
	signature: string;
	/** INTERNAL: Typed payload after JSON deserialization (not transmitted) */
	_deserializedPayload?: T;
}

/**
 * Response validation error types
 */
export class SignedResponseError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'SignedResponseError';
	}
}
