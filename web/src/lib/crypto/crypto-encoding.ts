/**
 * Crypto Encoding Module - Base64/Base64URL Conversion Functions
 *
 * Single Responsibility: Handle all Base64 and Base64URL encoding/decoding operations
 * Part of crypto.ts refactorization to apply SOLID principles
 */

/**
 * Convert Uint8Array to base64 string for URL-safe transmission
 *
 * @param bytes - Uint8Array to convert
 * @returns base64 encoded string
 */
export function bytesToBase64(bytes: Uint8Array): string {
	return btoa(String.fromCharCode(...bytes));
}

/**
 * Convert Uint8Array to base64URL string (URL-safe, no padding)
 *
 * @param bytes - Uint8Array to convert
 * @returns base64URL encoded string
 */
export function bytesToBase64Url(bytes: Uint8Array): string {
	return btoa(String.fromCharCode(...bytes))
		.replace(/\+/g, '-')
		.replace(/\//g, '_')
		.replace(/=/g, '');
}

/**
 * Convert base64 string back to Uint8Array
 *
 * @param base64 - base64 encoded string
 * @returns Uint8Array
 */
export function base64ToBytes(base64: string): Uint8Array {
	return new Uint8Array(
		atob(base64)
			.split('')
			.map((char) => char.charCodeAt(0))
	);
}

/**
 * Convert base64URL string back to Uint8Array
 *
 * @param base64Url - base64URL encoded string
 * @returns Uint8Array
 */
export function base64UrlToBytes(base64Url: string): Uint8Array {
	// Add padding if needed
	let base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
	while (base64.length % 4) {
		base64 += '=';
	}
	return base64ToBytes(base64);
}
