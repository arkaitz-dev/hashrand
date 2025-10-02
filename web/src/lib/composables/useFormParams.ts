/**
 * Universal Form Parameters Management Composable
 *
 * Eliminates 240+ lines of duplicated parameter handling across routes.
 * Handles URL decryption, initialization, and parameter application.
 *
 * SOLID Principles Applied:
 * - SRP: Single responsibility for parameter management
 * - DRY: Centralized parameter handling logic
 * - ISP: Interface segregation - each route only gets what it needs
 */

import { onMount } from 'svelte';
import { writable } from 'svelte/store';
import { get } from 'svelte/store';
import { authStore } from '$lib/stores/auth';
import { resultState } from '$lib/stores/result';
import { decryptPageParams } from '$lib/crypto';
import { page } from '$app/stores';

export interface FormParamsConfig<T> {
	endpoint: string;
	getDefaultParams: () => T;
	validateAndApplyParams(urlParams: Record<string, unknown>, currentParams: T): T;
}

export function useFormParams<T>(config: FormParamsConfig<T>) {
	// Create writable stores for reactivity
	const paramsStore = writable<T>(config.getDefaultParams());
	const urlProvidedSeedStore = writable<string>('');

	/**
	 * Initialize parameters from various sources
	 */
	function initializeParams() {
		onMount(async () => {
			// Check if we're coming from result page with existing params
			const currentResultState = get(resultState);
			if (
				currentResultState &&
				currentResultState.endpoint === config.endpoint &&
				currentResultState.params
			) {
				// Coming from result page - use existing params
				paramsStore.set({ ...currentResultState.params } as T);
			} else {
				// Coming from menu or fresh load - use defaults
				paramsStore.set(config.getDefaultParams());
			}

			// Override with URL parameters if present
			await applyUrlParameters();
		});
	}

	/**
	 * Decrypt and apply URL parameters to form state
	 */
	async function applyUrlParameters() {
		let urlParams: Record<string, unknown> = {};

		// Get current page search params
		const currentPage = get(page);
		const searchParams = currentPage.url.searchParams;

		// Try to decrypt if encrypted parameters are present
		const cipherToken = authStore.getCipherToken();
		const nonceToken = authStore.getNonceToken();
		const hmacKey = authStore.getHmacKey();

		if (cipherToken && nonceToken && hmacKey) {
			try {
				const decryptedParams = await decryptPageParams(searchParams, {
					cipherToken,
					nonceToken,
					hmacKey
				});

				if (decryptedParams) {
					urlParams = decryptedParams;
				}
			} catch {
				// Ignore decryption errors silently
			}
		}

		// Apply URL parameters to form state using route-specific validation
		const currentParams = get(paramsStore);
		const newParams = config.validateAndApplyParams(urlParams, currentParams);
		paramsStore.set(newParams);

		// Handle seed parameter (common to all routes)
		if (urlParams.seed) {
			urlProvidedSeedStore.set(String(urlParams.seed));
		}
	}

	/**
	 * Update parameters
	 */
	function updateParams(newParams: Partial<T>) {
		paramsStore.update((current) => ({ ...current, ...newParams }));
	}

	// Initialize parameters on mount
	initializeParams();

	return {
		params: {
			get value() {
				return get(paramsStore);
			},
			set value(newParams: T) {
				paramsStore.set(newParams);
			}
		},
		urlProvidedSeed: {
			get value() {
				return get(urlProvidedSeedStore);
			},
			set value(seed: string) {
				urlProvidedSeedStore.set(seed);
			}
		},
		updateParams
	};
}
