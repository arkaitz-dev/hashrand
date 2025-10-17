/**
 * Universal Generation Workflow Composable
 *
 * Eliminates 600+ lines of duplicated code across generation routes.
 * Handles authentication, parameter validation, encryption, and navigation.
 *
 * SOLID Principles Applied:
 * - SRP: Single responsibility for generation workflow
 * - OCP: Open for extension, closed for modification
 * - DRY: Don't repeat yourself - centralized logic
 */

import { goto } from '$app/navigation';
import { authStore } from '$lib/stores/auth';
import { createEncryptedUrl } from '$lib/crypto';
import { checkSessionOrAutoLogout } from '$lib/session-expiry-manager';
import { hasLocalAuthTokens } from '$lib/stores/auth/auth-session';
import { dialogStore } from '$lib/stores/dialog';
import { logger } from '$lib/utils/logger';

export interface GenerationConfig<T = Record<string, unknown>> {
	endpoint: string;
	formValid: boolean;
	getParams: () => T;
	urlProvidedSeed?: string;
}

export function useGenerationWorkflow<T = Record<string, unknown>>(config: GenerationConfig<T>) {
	let pendingGenerationParams: Record<string, unknown> | null = null;

	/**
	 * Main generation handler - CHECK AUTHENTICATION AND SESSION
	 * LOGIC:
	 * 1. Verify form is valid
	 * 2. Check if user has local auth tokens
	 *    - NO tokens → show email dialog for authentication
	 *    - HAS tokens → proceed to step 3
	 * 3. Check if session has expired
	 *    - Expired → auto-logout
	 *    - Valid → proceed with generation
	 */
	async function handleGenerate(event: Event) {
		event.preventDefault();
		logger.info(`[Form] Submitting ${config.endpoint} generation form`);

		if (!config.formValid) {
			logger.warn(`[Form] ${config.endpoint} form validation failed`);
			return;
		}

		// Step 1: Check if user has local auth tokens (NO HTTP call)
		const hasTokens = await hasLocalAuthTokens();

		if (!hasTokens) {
			// User is NOT authenticated → show email dialog
			logger.info('[Auth] User not authenticated - showing email dialog');

			// Import conversion functions
			const { alphabetToInt, mnemonicLangToInt } = await import('$lib/types');

			// Store pending generation parameters for after authentication
			const params = config.getParams();
			const resultParams: Record<string, unknown> = {
				endpoint: config.endpoint,
				...params
			};

			// CRITICAL: Convert alphabet/language strings to integers BEFORE encryption
			// This ensures encrypted URL params use compact integer format (even for non-authenticated users)
			if (resultParams.alphabet && typeof resultParams.alphabet === 'string') {
				resultParams.alphabet = alphabetToInt(
					resultParams.alphabet as import('$lib/types').AlphabetTypeString
				);
			}
			if (resultParams.language && typeof resultParams.language === 'string') {
				resultParams.language = mnemonicLangToInt(
					resultParams.language as import('$lib/types').MnemonicLanguageString
				);
			}

			// Add seed if provided from URL
			if (config.urlProvidedSeed) {
				resultParams.seed = config.urlProvidedSeed;
			}

			pendingGenerationParams = resultParams;

			// Show authentication dialog with destination
			const authConfig = {
				destination: {
					route: '/result',
					params: resultParams
				}
			};

			dialogStore.show('auth', authConfig);
			return;
		}

		// Step 2: User has tokens - check if session is still valid
		// If expired, performs automatic logout (redirect + cleanup + flash)
		const sessionValid = await checkSessionOrAutoLogout();

		if (!sessionValid) {
			// Session expired, auto-logout already performed
			logger.info('[Auth] Session expired - auto-logout triggered');
			return;
		}

		// Step 3: Session is valid - proceed with generation
		logger.info('[Auth] Session valid - proceeding with generation');
		await performGeneration();
	}

	/**
	 * Execute generation with current parameters
	 */
	async function performGeneration() {
		await proceedWithGeneration();
	}

	/**
	 * Core generation logic - creates encrypted URL and navigates
	 */
	async function proceedWithGeneration() {
		// Import conversion functions
		const { alphabetToInt, mnemonicLangToInt } = await import('$lib/types');

		// Create parameters object for result page
		const params = config.getParams();
		const resultParams: Record<string, unknown> = {
			endpoint: config.endpoint,
			...params
		};

		// CRITICAL: Convert alphabet/language strings to integers BEFORE encryption
		// This ensures encrypted URL params use compact integer format
		if (resultParams.alphabet && typeof resultParams.alphabet === 'string') {
			resultParams.alphabet = alphabetToInt(
				resultParams.alphabet as import('$lib/types').AlphabetTypeString
			);
		}
		if (resultParams.language && typeof resultParams.language === 'string') {
			resultParams.language = mnemonicLangToInt(
				resultParams.language as import('$lib/types').MnemonicLanguageString
			);
		}

		// Add seed if provided from URL
		if (config.urlProvidedSeed) {
			resultParams.seed = config.urlProvidedSeed;
		}

		// Get crypto tokens for parameter encryption
		const cipherToken = authStore.getCipherToken();
		const nonceToken = authStore.getNonceToken();
		const hmacKey = authStore.getHmacKey();

		if (cipherToken && nonceToken && hmacKey) {
			// Create encrypted URL for privacy
			const encryptedUrl = await createEncryptedUrl('/result', resultParams, {
				cipherToken,
				nonceToken,
				hmacKey
			});

			// Navigate to result page with encrypted parameters
			logger.info('[Navigation] Redirecting to: /result (encrypted params)');
			goto(encryptedUrl);
		} else {
			// ERROR: Crypto tokens required for secure navigation
			logger.error('[Navigation] Missing crypto tokens - returning to home');
			// Missing crypto tokens - cannot create secure URL
			goto('/'); // Return to home instead of unsecure URL
		}
	}

	/**
	 * Handle successful authentication - resumes generation with pending params
	 */
	function handleAuthenticated() {
		// Perform the generation with the pending parameters
		if (pendingGenerationParams) {
			pendingGenerationParams = null;
			// Perform generation with current form state
			performGeneration();
		}
	}

	// Setup authentication event listener
	if (typeof globalThis.window !== 'undefined') {
		globalThis.window.addEventListener('authenticated', handleAuthenticated as EventListener);
	}

	return {
		handleGenerate,
		performGeneration,
		pendingGenerationParams: () => pendingGenerationParams
	};
}
