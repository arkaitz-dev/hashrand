/**
 * Universal Signed Response Validation System
 *
 * Provides Ed25519 signature verification for all backend responses
 * CORRECTED: Uses deterministic JSON + Base64 URL-safe for perfect consistency
 */

import { ed25519 } from '@noble/curves/ed25519.js';
import { sortObjectKeys, decodePayloadBase64 } from './signedRequest';

/**
 * Universal signed response structure from backend
 * CORRECTED: payload is Base64 URL-safe encoded deterministic JSON
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

/**
 * Signed response validator with Ed25519 verification
 */
export class SignedResponseValidator {
	/**
	 * Validate signed response from server
	 *
	 * @param responseData - Raw response data (object or string)
	 * @param serverPubKeyHex - Server's Ed25519 public key as hex string
	 * @returns Validated payload data or throws SignedResponseError
	 */
	static async validateSignedResponse<T>(
		responseData: unknown,
		serverPubKeyHex: string
	): Promise<T> {
		// Parse response data if it's a string
		let signedResponse: SignedResponse<T>;
		if (typeof responseData === 'string') {
			try {
				signedResponse = JSON.parse(responseData);
			} catch {
				throw new SignedResponseError('Invalid JSON in signed response');
			}
		} else if (typeof responseData === 'object' && responseData !== null) {
			signedResponse = responseData as SignedResponse<T>;
		} else {
			throw new SignedResponseError('Response data must be string or object');
		}

		// Validate response structure
		if (!this.isSignedResponse(signedResponse)) {
			throw new SignedResponseError('Response missing payload or signature fields');
		}

		// Validate payload is string (Base64 encoded JSON)
		if (typeof signedResponse.payload !== 'string') {
			throw new SignedResponseError('Payload must be Base64-encoded JSON string');
		}

		// Validate server public key format
		if (!this.isValidHexKey(serverPubKeyHex, 64)) {
			throw new SignedResponseError('Invalid server public key format (expected 64 hex chars)');
		}

		// Validate signature format
		if (!this.isValidHexKey(signedResponse.signature, 128)) {
			throw new SignedResponseError('Invalid signature format (expected 128 hex chars)');
		}

		// Step 1: Decode Base64 to JSON (only for data extraction, NOT for verification!)
		let originalJsonPayload: string;
		try {
			originalJsonPayload = decodePayloadBase64(signedResponse.payload);
			console.log(
				'🔓 BASE64 FRONTEND: Decoded Base64 to JSON for data extraction, length:',
				originalJsonPayload.length
			);
		} catch (e) {
			throw new SignedResponseError(`Base64 decoding failed: ${e}`);
		}

		// Step 2: Verify Ed25519 signature against Base64 payload (what server actually signed!)
		const isValid = await this.verifyEd25519Signature(
			signedResponse.payload, // Verify the Base64 directly!
			signedResponse.signature,
			serverPubKeyHex
		);

		if (!isValid) {
			throw new SignedResponseError('Ed25519 signature verification failed');
		}

		// Step 3: Parse JSON back to typed object
		try {
			const deserializedPayload = JSON.parse(originalJsonPayload) as T;
			console.log(
				'✅ BASE64 FRONTEND: Signature verified against Base64, data extracted from JSON'
			);
			return deserializedPayload;
		} catch (e) {
			throw new SignedResponseError(`JSON parsing failed: ${e}`);
		}
	}

	/**
	 * Extract server public key from signed response payload
	 *
	 * @param responseData - Raw response data
	 * @returns Server public key hex string or null if not found
	 */
	static extractServerPubKey(responseData: unknown): string | null {
		try {
			let signedResponse: SignedResponse;
			if (typeof responseData === 'string') {
				signedResponse = JSON.parse(responseData);
			} else if (typeof responseData === 'object' && responseData !== null) {
				signedResponse = responseData as SignedResponse;
			} else {
				return null;
			}

			if (!this.isSignedResponse(signedResponse)) {
				return null;
			}

			// Decode Base64 payload and parse JSON to extract server_pub_key
			try {
				const originalJsonPayload = decodePayloadBase64(signedResponse.payload);
				const deserializedPayload = JSON.parse(originalJsonPayload) as Record<string, unknown>;

				if (
					deserializedPayload &&
					typeof deserializedPayload === 'object' &&
					'server_pub_key' in deserializedPayload
				) {
					const serverKey = deserializedPayload.server_pub_key;
					if (typeof serverKey === 'string' && this.isValidHexKey(serverKey, 64)) {
						return serverKey;
					}
				}
			} catch {
				// Base64 decoding or JSON parsing failed
				return null;
			}

			return null;
		} catch {
			return null;
		}
	}

	/**
	 * Check if response is a valid signed response structure
	 * NEW: Validates that payload is msgpack-serialized string
	 */
	static isSignedResponse(obj: unknown): obj is SignedResponse {
		return (
			obj !== null &&
			typeof obj === 'object' &&
			'payload' in obj &&
			'signature' in obj &&
			typeof (obj as Record<string, unknown>).payload === 'string' &&
			typeof (obj as Record<string, unknown>).signature === 'string'
		);
	}

	/**
	 * Serialize payload deterministically (matching backend sortObjectKeys)
	 */
	static serializePayload(payload: unknown): string {
		const sortedPayload = sortObjectKeys(payload);
		return JSON.stringify(sortedPayload);
	}

	/**
	 * Verify Ed25519 signature using @noble/curves
	 */
	private static async verifyEd25519Signature(
		message: string,
		signatureHex: string,
		publicKeyHex: string
	): Promise<boolean> {
		try {
			// Convert hex strings to bytes
			const messageBytes = new TextEncoder().encode(message);
			const signatureBytes = new Uint8Array(
				signatureHex.match(/.{2}/g)?.map((byte) => parseInt(byte, 16)) || []
			);
			const publicKeyBytes = new Uint8Array(
				publicKeyHex.match(/.{2}/g)?.map((byte) => parseInt(byte, 16)) || []
			);

			// Verify signature using @noble/curves Ed25519
			return ed25519.verify(signatureBytes, messageBytes, publicKeyBytes);
		} catch {
			return false;
		}
	}

	/**
	 * Validate hex string format and length
	 */
	private static isValidHexKey(hex: string, expectedLength: number): boolean {
		return typeof hex === 'string' && hex.length === expectedLength && /^[0-9a-fA-F]+$/.test(hex);
	}
}

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
 * Check if data is a signed response
 *
 * @param obj - Data to check
 * @returns True if obj is a signed response structure
 */
export function isSignedResponse(obj: unknown): obj is SignedResponse {
	return SignedResponseValidator.isSignedResponse(obj);
}
