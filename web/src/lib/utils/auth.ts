/**
 * Authentication utility functions shared across pages
 */
import { base58 } from '@scure/base';

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
		console.error('Error encoding next parameter to base58:', error);
		return null;
	}
}

/**
 * Handle email confirmation for authentication
 * Sends magic link request with next parameter
 */
export async function handleEmailConfirmation(
	email: string,
	nextObject: Record<string, unknown> | null,
	onSuccess: () => void,
	onError: (message: string) => void
): Promise<void> {
	try {
		// Get current UI host
		const currentHost = window.location.origin;

		// Encode nextObject to Base58 for backend
		const nextBase58 = encodeNextToBase58(nextObject);

		const requestBody = {
			email: email,
			ui_host: currentHost,
			...(nextBase58 && { next: nextBase58 })
		};

		const response = await fetch('/api/login/', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(requestBody)
		});

		if (response.ok) {
			// Magic link sent successfully
			onSuccess();
		} else {
			// Error sending magic link
			onError('common.sendError');
		}
	} catch (error) {
		console.error('Error sending magic link:', error);
		onError('common.connectionError');
	}
}
