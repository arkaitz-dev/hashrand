/**
 * Authentication utility functions shared across pages
 */
import { base58 } from '@scure/base';
import { logger } from './logger';

/**
 * Convert JSON object to base58 encoded string
 */
export function encodeNextToBase58(nextObj: Record<string, unknown> | null): string | null {
	if (!nextObj) return null;
	try {
		// Convert JSON object to string
		const jsonString = JSON.stringify(nextObj);
		// Convert string to bytes (Uint8Array)
		const encoder = new globalThis.TextEncoder();
		const bytes = encoder.encode(jsonString);
		// Encode bytes to base58
		const base58String = base58.encode(bytes);
		return base58String;
	} catch (error) {
		logger.error('Error encoding next parameter to base58:', error);
		return null;
	}
}

/**
 * Handle email confirmation for authentication
 * Sends magic link request with next parameter using universal signed request
 */
export async function handleEmailConfirmation(
	email: string,
	nextObject: Record<string, unknown> | null,
	onSuccess: () => void,
	onError: (message: string) => void
): Promise<void> {
	try {
		// Get current UI host (domain only) for cookie Domain attribute
		const { extractDomain } = await import('./domain-extractor');
		const ui_host = extractDomain();

		// Encode nextObject to Base58 for backend
		const nextBase58 = encodeNextToBase58(nextObject);

		const requestPayload = {
			email: email,
			ui_host,
			...(nextBase58 && { next: nextBase58 })
		};

		// Use universal signed POST request (first signed response to extract server_pub_key)
		const { httpSignedPOSTRequest } = await import('../httpSignedRequests');
		await httpSignedPOSTRequest('/api/login/', requestPayload, true);

		// Magic link sent successfully
		onSuccess();
	} catch (error) {
		logger.error('Error sending signed magic link request:', error);
		onError('common.connectionError');
	}
}
