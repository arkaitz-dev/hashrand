/**
 * Domain Extraction Utility
 *
 * SECURITY: Extracts only the hostname (domain) from the current URL
 * for use in backend API calls that require domain-based cookie management.
 *
 * Returns hostname only, without protocol, port, or path.
 *
 * Examples:
 * - http://localhost:5173 → "localhost"
 * - https://app.example.com:443/path → "app.example.com"
 * - https://subdomain.domain.com → "subdomain.domain.com"
 */

/**
 * Extract domain (hostname only) from current browser location
 *
 * @returns {string} The hostname without protocol or port
 * @throws {Error} If called in non-browser context (e.g., SSR)
 */
export function extractDomain(): string {
	if (typeof window === 'undefined') {
		throw new Error('extractDomain() can only be called in browser context');
	}

	const domain = window.location.hostname;

	console.log(`🔒 [SECURITY] Extracted domain for API: '${domain}'`);

	return domain;
}

/**
 * Validate if a string is a valid domain/hostname format
 *
 * @param {string} domain - The domain string to validate
 * @returns {boolean} True if valid domain format
 */
export function isValidDomain(domain: string): boolean {
	if (!domain || typeof domain !== 'string') {
		return false;
	}

	// Basic domain validation regex
	// Allows: localhost, subdomains, IP addresses
	const domainRegex =
		/^(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)*[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$/i;
	const ipRegex = /^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$/;

	return domainRegex.test(domain) || ipRegex.test(domain) || domain === 'localhost';
}
