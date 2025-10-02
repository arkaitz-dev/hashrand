/**
 * Response parsing utilities
 */

import type { SignedResponse } from './types';

/**
 * Parse response data to SignedResponse structure (DRY utility)
 *
 * @param responseData - Raw response data (string or object)
 * @param errorPrefix - Prefix for error messages
 * @returns Parsed SignedResponse or null on error
 */
export function parseResponseData<T>(
	responseData: unknown,
	errorPrefix = 'Response'
): SignedResponse<T> | null {
	let signedResponse: SignedResponse<T>;

	if (typeof responseData === 'string') {
		try {
			signedResponse = JSON.parse(responseData);
		} catch {
			console.error(`${errorPrefix}: Invalid JSON in signed response`);
			return null;
		}
	} else if (typeof responseData === 'object' && responseData !== null) {
		signedResponse = responseData as SignedResponse<T>;
	} else {
		console.error(`${errorPrefix}: Response data must be string or object`);
		return null;
	}

	return signedResponse;
}

/**
 * Check if object is a valid signed response structure
 */
export function isSignedResponse(obj: unknown): obj is SignedResponse {
	return (
		obj !== null &&
		typeof obj === 'object' &&
		'payload' in obj &&
		'signature' in obj &&
		typeof (obj as Record<string, unknown>).payload === 'string' &&
		typeof (obj as Record<string, unknown>).signature === 'string'
	);
}
