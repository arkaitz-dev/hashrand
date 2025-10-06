/**
 * Signed response validation logic
 */

import { decodePayloadBase64, sortObjectKeys } from '../signedRequest';
import { verifyEd25519Signature, isValidHexKey, isValidBase58Signature } from './crypto';
import { parseResponseData, isSignedResponse as checkSignedResponse } from './parsing';
import { SignedResponseError } from './types';

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
		// Parse response data using DRY utility
		const signedResponse = parseResponseData<T>(responseData, 'validateSignedResponse');

		if (!signedResponse) {
			throw new SignedResponseError('Invalid JSON in signed response');
		}

		// Validate response structure
		if (!checkSignedResponse(signedResponse)) {
			throw new SignedResponseError('Response missing payload or signature fields');
		}

		// Validate payload is string (Base64 encoded JSON)
		if (typeof signedResponse.payload !== 'string') {
			throw new SignedResponseError('Payload must be Base64-encoded JSON string');
		}

		// Validate server public key format
		if (!isValidHexKey(serverPubKeyHex, 64)) {
			throw new SignedResponseError('Invalid server public key format (expected 64 hex chars)');
		}

		// Validate signature format (now base58, ~87-88 chars)
		// if (!isValidHexKey(signedResponse.signature, 128)) {
		// 	throw new SignedResponseError('Invalid signature format (expected 128 hex chars)');
		// }
		if (!isValidBase58Signature(signedResponse.signature)) {
			throw new SignedResponseError(
				'Invalid signature format (expected base58 string, 87-88 chars)'
			);
		}

		// Step 1: Decode Base64 to JSON (only for data extraction, NOT for verification!)
		let originalJsonPayload: string;
		try {
			originalJsonPayload = decodePayloadBase64(signedResponse.payload);
		} catch (e) {
			throw new SignedResponseError(`Base64 decoding failed: ${e}`);
		}

		// Step 2: Verify Ed25519 signature against Base64 payload (what server actually signed!)
		const isValid = await verifyEd25519Signature(
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
			// Parse response data using DRY utility
			const signedResponse = parseResponseData(responseData, 'extractServerPubKey');

			if (!signedResponse) {
				return null;
			}

			if (!checkSignedResponse(signedResponse)) {
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
					if (typeof serverKey === 'string' && isValidHexKey(serverKey, 64)) {
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
	 * Serialize payload deterministically (matching backend sortObjectKeys)
	 */
	static serializePayload(payload: unknown): string {
		const sortedPayload = sortObjectKeys(payload);
		return JSON.stringify(sortedPayload);
	}
}
