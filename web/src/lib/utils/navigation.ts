/**
 * Navigation utilities for handling Next parameter structure with DRY principles
 */

import { logger } from './logger';

/**
 * Structure for Next parameter - used consistently across all components
 */
export interface NextParameterStructure {
	route: string; // "/mnemonic", "/custom", "/", etc.
	params?: Record<string, string>; // Parameters to be encrypted if present
}

/**
 * Parse Next parameter JSON and create navigation URL with encrypted parameters
 *
 * This is the central DRY function used by all components that need to:
 * 1. Parse received next JSON string
 * 2. Generate navigation URL with encrypted parameters when needed
 *
 * @param nextJsonString - JSON string from backend (or null)
 * @returns Navigation URL: "/" | "/route" | "/route?p=encrypted"
 */
export async function parseNextParameterJson(
	nextJsonString: string | null | undefined
): Promise<string> {
	// Handle null/undefined/empty cases
	if (!nextJsonString || nextJsonString.trim() === '') {
		return '/';
	}

	try {
		// Parse JSON structure
		const nextStructure: NextParameterStructure = JSON.parse(nextJsonString);

		// Validate structure
		if (!nextStructure.route || typeof nextStructure.route !== 'string') {
			return '/';
		}

		// Clean route (ensure it starts with /)
		const cleanRoute = nextStructure.route.startsWith('/')
			? nextStructure.route
			: `/${nextStructure.route}`;

		// If no parameters, return clean route
		if (!nextStructure.params || Object.keys(nextStructure.params).length === 0) {
			return cleanRoute;
		}

		// Encrypt parameters and create URL with ?p=encrypted
		try {
			const { authStore } = await import('../stores/auth');
			const cipherToken = authStore.getCipherToken();
			const nonceToken = authStore.getNonceToken();
			const hmacKey = authStore.getHmacKey();

			if (cipherToken && nonceToken && hmacKey) {
				const { encryptUrlParams } = await import('../crypto');
				const result = await encryptUrlParams(
					nextStructure.params,
					cipherToken,
					nonceToken,
					hmacKey
				);
				return `${cleanRoute}?p=${result.p}`;
			} else {
				// Crypto tokens missing - use universal recovery handler
				const { ensureCryptoTokensExist } = await import('./auth-recovery');
				await ensureCryptoTokensExist('URL Parameter Encryption');
				// Return home (safe abort - no unencrypted params exposed)
				return '/';
			}
		} catch (error) {
			// Encryption failed - use universal recovery handler
			logger.error('URL parameter encryption failed:', error);
			const { ensureCryptoTokensExist } = await import('./auth-recovery');
			await ensureCryptoTokensExist('URL Parameter Encryption (Error)');
			// Return home (safe abort)
			return '/';
		}
	} catch {
		return '/';
	}
}

/**
 * Create Next parameter JSON string for sending to backend
 *
 * @param route - Target route (e.g., "/mnemonic", "/custom")
 * @param params - Optional parameters to include
 * @returns JSON string for backend
 */
export function createNextParameterJson(route: string, params?: Record<string, string>): string {
	const structure: NextParameterStructure = {
		route,
		...(params && Object.keys(params).length > 0 && { params })
	};

	return JSON.stringify(structure);
}

/**
 * Helper: Create next parameter for logout/login (always "/")
 */
export function createLogoutNextParameter(): string {
	return createNextParameterJson('/');
}

/**
 * Helper: Create next parameter for specific route without parameters
 */
export function createSimpleRouteNextParameter(route: string): string {
	return createNextParameterJson(route);
}

/**
 * Helper: Create next parameter for route with parameters
 */
export function createRouteWithParamsNextParameter(
	route: string,
	params: Record<string, string>
): string {
	return createNextParameterJson(route, params);
}

/**
 * Universal AuthDialog configuration interface
 * Eliminates the need for context detection and hardcoded logic
 */
export interface AuthDialogConfig {
	email?: string; // Optional prefilled email
	destination: {
		route: string; // Explicit destination route: '/', '/result', etc.
		params?: Record<string, unknown>; // Optional parameters for destination
	};
}

/**
 * Universal DRY function to build nextParam from AuthDialog configuration
 *
 * Replaces all hardcoded logic in AuthConfirmDialogContent with clean,
 * explicit configuration. Each caller specifies exactly what they need.
 *
 * @param config - AuthDialog configuration with explicit destination
 * @returns nextParam string for magic link request
 */
export function buildNextParameterFromConfig(config: AuthDialogConfig): string {
	const { destination } = config;

	// Validate destination
	if (!destination.route || typeof destination.route !== 'string') {
		return createLogoutNextParameter();
	}

	// Filter out email from params (security: never include email in next params)
	const filteredParams = destination.params
		? Object.entries(destination.params)
				.filter(([key]) => key !== 'email') // CRITICAL: Never include email
				.reduce(
					(acc, [key, value]) => {
						acc[key] = String(value);
						return acc;
					},
					{} as Record<string, string>
				)
		: undefined;

	// Build nextParam using existing DRY helpers
	if (filteredParams && Object.keys(filteredParams).length > 0) {
		return createRouteWithParamsNextParameter(destination.route, filteredParams);
	} else {
		return createSimpleRouteNextParameter(destination.route);
	}
}
