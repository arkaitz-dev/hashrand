/**
 * Universal Signed Response Handler
 *
 * Provides STRICT validation for ALL SignedResponse from backend
 * No fallback allowed - prevents server impersonation attacks
 */

import { isSignedResponse, extractServerPubKey, validateSignedResponse } from './signedResponse';
import { sessionManager } from './session-manager';

/**
 * STRICT handler for signed responses from backend
 *
 * If validation fails, redirects to "/" with error message
 * This prevents server impersonation attacks
 *
 * @param responseData - Raw response data from backend (MUST be signed)
 * @param isFirstSignedResponse - If true, extracts and stores server_pub_key
 * @returns Validated payload data or redirects on failure
 */
export async function handleSignedResponseStrict<T>(
	responseData: unknown,
	isFirstSignedResponse: boolean = false
): Promise<T> {
	try {
		// Check if response is signed (required for all endpoints except /api/version)
		if (!isSignedResponse(responseData)) {
			throw new Error('Expected SignedResponse but received non-signed response');
		}

		let serverPubKey: string;

		if (isFirstSignedResponse) {
			// First signed response: extract and store server public key
			serverPubKey = extractServerPubKey(responseData);
			if (!serverPubKey) {
				throw new Error('Server public key not found in first signed response');
			}

			// Store server public key for future response validations
			await sessionManager.setServerPubKey(serverPubKey);
		} else {
			// Subsequent responses: use stored server public key
			const storedServerPubKey = await sessionManager.getServerPubKey();
			if (!storedServerPubKey) {
				throw new Error('No server public key available for response validation');
			}
			serverPubKey = storedServerPubKey;
		}

		// Validate signed response with server public key
		return await validateSignedResponse<T>(responseData, serverPubKey);
	} catch (error) {
		// Security violation: Invalid or missing signature
		console.error('SignedResponse validation failed:', error);

		// Show flash message and redirect to home
		await showSecurityErrorAndRedirect();

		// This should never be reached due to redirect, but TypeScript needs a return
		throw error;
	}
}

/**
 * Show security error flash message and redirect to home
 */
async function showSecurityErrorAndRedirect(): Promise<void> {
	try {
		// Import flash message system
		const { flashMessagesStore } = await import('./stores/flashMessages');

		// Add security error message
		flashMessagesStore.add({
			type: 'error',
			message: 'No se ha recibido una respuesta correcta del servidor',
			duration: 5000
		});

		// Import navigation utility
		const { goto } = await import('$app/navigation');

		// Redirect to home page
		await goto('/');
	} catch (redirectError) {
		console.error('Failed to redirect after security violation:', redirectError);
		// Force page reload as last resort
		window.location.href = '/';
	}
}
