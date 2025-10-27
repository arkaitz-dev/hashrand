/**
 * Universal Signed Response Handler
 *
 * Provides STRICT validation for ALL SignedResponse from backend
 * No fallback allowed - prevents server impersonation attacks
 */

import {
	isSignedResponse,
	extractServerPubKey,
	extractServerX25519PubKey,
	validateSignedResponse
} from './signedResponse';
import { sessionManager } from './session-manager';
import { logger } from './utils/logger';

/**
 * STRICT handler for signed responses from backend
 *
 * SECURITY: Always validates with stored OLD server_pub_key first
 * Only after successful validation, accepts NEW server_pub_key from payload (if present)
 * This prevents MITM attacks where attacker tries to inject their own server_pub_key
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

		let serverPubKey: string | null;

		if (isFirstSignedResponse) {
			// First signed response: extract and store server public key
			serverPubKey = extractServerPubKey(responseData);
			if (!serverPubKey) {
				throw new Error('Server public key not found in first signed response');
			}

			// Store server public key for future response validations
			await sessionManager.setServerPubKey(serverPubKey);
		} else {
			// Subsequent responses: ALWAYS validate with stored OLD server_pub_key
			const storedServerPubKey = await sessionManager.getServerPubKey();
			if (!storedServerPubKey) {
				throw new Error('No server public key available for response validation');
			}
			serverPubKey = storedServerPubKey;
		}

		// Validate signed response with server public key (OLD for key rotation)
		const validatedPayload = await validateSignedResponse<T>(responseData, serverPubKey);

		// SECURITY: After validation succeeds, check if response contains NEW server_pub_key
		// This means backend is initiating key rotation (PERIOD 2/3)
		if (!isFirstSignedResponse) {
			const newServerPubKey = extractServerPubKey(responseData);
			if (newServerPubKey && newServerPubKey !== serverPubKey) {
				// Key rotation detected: update stored server_pub_key
				await sessionManager.setServerPubKey(newServerPubKey);
			}
		}

		// Extract and store server X25519 public key for E2E encryption (if present)
		// This key is ALWAYS included in login/refresh responses (unlike server_pub_key which is only in rotation)
		const serverX25519PubKey = extractServerX25519PubKey(responseData);
		logger.info('🔑 Extracted server_x25519_pub_key from response:', {
			exists: !!serverX25519PubKey,
			length: serverX25519PubKey?.length,
			firstChars: serverX25519PubKey?.substring(0, 16),
			isValidHex: serverX25519PubKey ? /^[0-9a-f]{64}$/i.test(serverX25519PubKey) : false
		});
		if (serverX25519PubKey) {
			logger.info('📦 Storing server X25519 public key in IndexedDB for E2E encryption');
			await sessionManager.setServerX25519PubKey(serverX25519PubKey);
			logger.info('✅ Server X25519 public key stored successfully');
		} else {
			logger.warn('⚠️ No server_x25519_pub_key found in response - E2E encryption will fail');
		}

		return validatedPayload;
	} catch (error) {
		// Security violation: Invalid or missing signature
		logger.error('SignedResponse validation failed:', error);

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

		// Import translation system to get current language
		const { currentLanguage, t } = await import('./stores/i18n');

		// Get current language value
		let lang = 'en';
		const unsubscribe = currentLanguage.subscribe((l) => (lang = l));
		unsubscribe();

		// Add security error message (translated)
		const errorMessage = t('common.signatureValidationError', lang);
		flashMessagesStore.addMessage(errorMessage);

		// Import navigation utility
		const { goto } = await import('$app/navigation');

		// Redirect to home page
		await goto('/');
	} catch (redirectError) {
		logger.error('Failed to redirect after security violation:', redirectError);
		// Force page reload as last resort
		window.location.href = '/';
	}
}
