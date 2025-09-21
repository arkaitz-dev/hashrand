/**
 * Auth Crypto Tokens Module - Cryptographic Token Management
 *
 * Single Responsibility: Handle all cryptographic token generation and validation
 * Part of auth.ts refactorization to apply SOLID principles
 */

/**
 * Generate cryptographic tokens for URL parameter encryption
 * Called after successful login or when crypto tokens are missing
 */
export async function generateCryptoTokens(): Promise<void> {
	if (typeof window === 'undefined') return;

	try {
		// Generate three 32-byte cryptographically secure tokens
		const cipherToken = generateSecureToken();
		const nonceToken = generateSecureToken();
		const hmacKey = generateSecureToken();

		// Store in IndexedDB via session manager
		const { sessionManager } = await import('../../session-manager');
		await sessionManager.setCryptoTokens(cipherToken, nonceToken, hmacKey);

		// Log successful generation for debugging
		// Generated crypto tokens for URL parameter encryption
	} catch {
		// Failed to generate crypto tokens
	}
}

/**
 * Check if crypto tokens exist in IndexedDB
 */
export async function hasCryptoTokens(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	try {
		const { sessionManager } = await import('../../session-manager');
		return await sessionManager.hasCryptoTokens();
	} catch {
		return false;
	}
}

/**
 * Generate a cryptographically secure 32-byte token
 * @returns Base64 encoded string
 */
function generateSecureToken(): string {
	const array = new Uint8Array(32);
	crypto.getRandomValues(array);
	return btoa(String.fromCharCode(...array));
}

/**
 * Validate if refresh cookie is still valid by making a test request
 */
export async function hasValidRefreshCookie(): Promise<boolean> {
	if (typeof window === 'undefined') return false;

	try {
		// Make a lightweight request to check if refresh cookie is valid
		const response = await fetch('/api/refresh', {
			method: 'POST',
			credentials: 'include' // Include HttpOnly cookies
		});

		// If we get 200, cookie is valid
		// If we get 401, cookie is expired/invalid
		return response.ok;
	} catch {
		return false;
	}
}
