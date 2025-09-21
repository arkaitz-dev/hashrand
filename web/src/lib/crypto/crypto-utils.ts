/**
 * Crypto Utils Module - High-Level Cryptographic Workflows
 *
 * Single Responsibility: High-level URL encryption workflows and utilities
 * Part of crypto.ts refactorization to apply SOLID principles
 */

import { encryptUrlParams, decryptUrlParams } from './crypto-url-operations';

/**
 * Complete URL parameter encryption workflow
 *
 * @param params - Parameters to encrypt
 * @param sessionTokens - Session tokens from authStore
 * @returns Promise<Object with compact parameter 'p' for URL>
 */
export async function prepareSecureUrlParams(
	params: Record<string, unknown>,
	sessionTokens: {
		cipherToken: string;
		nonceToken: string;
		hmacKey: string;
	}
): Promise<{
	p: string;
}> {
	return await encryptUrlParams(
		params,
		sessionTokens.cipherToken,
		sessionTokens.nonceToken,
		sessionTokens.hmacKey
	);
}

/**
 * Extract route and parameters from a URL
 *
 * @param url - URL string to parse
 * @returns Object with basePath and parameters
 */
export function parseNextUrl(url: string): {
	basePath: string;
	params: Record<string, string>;
} {
	try {
		const urlObj = new globalThis.URL(url, 'http://localhost'); // Use dummy base for relative URLs
		const params: Record<string, string> = {};

		// Extract all search parameters
		urlObj.searchParams.forEach((value, key) => {
			params[key] = value;
		});

		return {
			basePath: urlObj.pathname,
			params
		};
	} catch {
		// If URL parsing fails, treat as a simple path without parameters
		return {
			basePath: url,
			params: {}
		};
	}
}

/**
 * Encrypt parameters in a next URL and create secure URL
 *
 * @param nextUrl - Original next URL from backend
 * @param sessionTokens - Session tokens for encryption
 * @returns Promise<Encrypted URL with basePath?p=...>
 */
export async function encryptNextUrl(
	nextUrl: string,
	sessionTokens: {
		cipherToken: string;
		nonceToken: string;
		hmacKey: string;
	}
): Promise<string> {
	const { basePath, params } = parseNextUrl(nextUrl);

	// If no parameters, return URL as-is
	if (Object.keys(params).length === 0) {
		return nextUrl;
	}

	// Encrypt parameters
	const { p } = await prepareSecureUrlParams(params, sessionTokens);

	// Create new URL with compact encrypted parameter
	return `${basePath}?p=${p}`;
}

/**
 * Decrypt parameters from current page URL if encrypted
 *
 * @param searchParams - URLSearchParams from current page
 * @param sessionTokens - Session tokens for decryption
 * @returns Promise<Decrypted parameters or null if not encrypted/failed>
 */
export async function decryptPageParams(
	searchParams: URLSearchParams,
	sessionTokens: {
		cipherToken: string;
		nonceToken: string;
		hmacKey: string;
	}
): Promise<Record<string, unknown> | null> {
	const p = searchParams.get('p');

	// Return null if not encrypted parameters
	if (!p) {
		return null;
	}

	try {
		// Decrypt parameters using compact parameter
		return await decryptUrlParams(
			p,
			sessionTokens.cipherToken,
			sessionTokens.nonceToken,
			sessionTokens.hmacKey
		);
	} catch {
		// Failed to decrypt URL parameters
		return null;
	}
}

/**
 * Create encrypted URL for navigation with parameters
 *
 * @param basePath - Base path for the route (e.g., '/result', '/custom')
 * @param params - Parameters to encrypt and include
 * @param sessionTokens - Session tokens for encryption
 * @returns Promise<Full encrypted URL>
 */
export async function createEncryptedUrl(
	basePath: string,
	params: Record<string, unknown>,
	sessionTokens: {
		cipherToken: string;
		nonceToken: string;
		hmacKey: string;
	}
): Promise<string> {
	// Processing crypto parameters

	// If no parameters, return simple base path
	if (!params || Object.keys(params).length === 0) {
		// No parameters to encrypt
		return basePath;
	}

	// Encrypt parameters
	const { p } = await prepareSecureUrlParams(params, sessionTokens);

	// Create compact encrypted URL
	return `${basePath}?p=${p}`;
}
