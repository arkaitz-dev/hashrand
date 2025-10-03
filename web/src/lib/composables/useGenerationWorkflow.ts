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
import { page } from '$app/stores';
import { authStore } from '$lib/stores/auth';
import { createEncryptedUrl } from '$lib/crypto';
import { checkSessionAndHandle } from '$lib/session-expiry-manager';
import { get } from 'svelte/store';

export interface GenerationConfig<T = Record<string, unknown>> {
	endpoint: string;
	formValid: boolean;
	getParams: () => T;
	urlProvidedSeed?: string;
}

export function useGenerationWorkflow<T = Record<string, unknown>>(config: GenerationConfig<T>) {
	let pendingGenerationParams: Record<string, unknown> | null = null;

	/**
	 * Main generation handler - CHECK SESSION EXPIRATION FIRST
	 * NEW LOGIC: Check session expiration before any generation logic
	 */
	async function handleGenerate(event: Event) {
		event.preventDefault();

		if (!config.formValid) {
			return;
		}

		// Check session expiration before generation
		const currentPage = get(page);
		const currentRoute = currentPage.url.pathname;

		const sessionValid = await checkSessionAndHandle({
			onExpired: 'launch-auth', // Launch auth dialog if expired
			next: currentRoute // Current generation route as next parameter
		});

		if (!sessionValid) {
			// Session was expired and auth dialog launched - stop generation
			return;
		}

		// Session is valid - proceed with generation
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
		// Create parameters object for result page
		const params = config.getParams();
		const resultParams: Record<string, unknown> = {
			endpoint: config.endpoint,
			...params
		};

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
			goto(encryptedUrl);
		} else {
			// ERROR: Crypto tokens required for secure navigation
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
