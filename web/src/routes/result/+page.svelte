<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import BackButton from '$lib/components/BackButton.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import Iconize from '$lib/components/Iconize.svelte';
	import DateTimeLocalized from '$lib/components/DateTimeLocalized.svelte';
	import { resultState, error, setResult, setLoading, isLoading } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import { isRTL } from '$lib/stores/rtl';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { dialogStore } from '$lib/stores/dialog';
	import { decryptPageParams, createEncryptedUrl } from '$lib/crypto';
	import { authStore } from '$lib/stores/auth';
	import { checkSessionOrAutoLogout } from '$lib/session-expiry-manager';
	import { logger } from '$lib/utils/logger';

	let copySuccess = $state(false);
	let copyTimeout: ReturnType<typeof setTimeout>;
	let showGenerationDetails = $state(false); // Collapsed by default on mobile
	let showParametersUsed = $state(false); // Collapsed by default on mobile
	// showSeedDialog removed - now using DialogContainer system

	function toggleGenerationDetails() {
		showGenerationDetails = !showGenerationDetails;
	}

	function toggleParametersUsed() {
		showParametersUsed = !showParametersUsed;
	}

	// Get URL parameters reactively
	let searchParams = $derived($page.url.searchParams);

	// Only treat as "provided seed" if seed parameter comes from URL GET parameters
	// (this controls whether to show the regenerate button)
	// let usedProvidedSeed = $derived(searchParams.has('seed')); // OLD: Check direct URL params

	// NEW: Check if seed exists in decrypted parameters or direct URL params
	let usedProvidedSeed = $state(false);

	// Handle result state and API calls
	onMount(async () => {
		logger.info('[Route] Result page loaded');
		// CHECK SESSION EXPIRATION FIRST - before any result processing
		// If expired, performs automatic logout (redirect + cleanup + flash)
		const sessionValid = await checkSessionOrAutoLogout();

		if (!sessionValid) {
			// Session expired, auto-logout already performed
			return; // Stop all result processing
		}

		// Session is valid - proceed with result processing

		// If there are URL parameters, ALWAYS generate from them (override any existing state)
		if (searchParams.size > 0) {
			await generateFromParams();
			return;
		}

		// If no URL parameters and no result state, redirect to home
		if (!$resultState) {
			// Redirect to home silently
			goto('/');
			return;
		}

		// Using existing result state
	});

	// Helper function to build params object from URL parameters (DRY)
	async function buildParamsFromUrlParams(
		urlParams: Record<string, unknown>
	): Promise<Record<string, string | number | boolean>> {
		const { intToAlphabet, isAlphabetInt, intToMnemonicLang, isMnemonicLangInt } = await import(
			'$lib/types'
		);
		const params: Record<string, string | number | boolean> = {};

		// Convert and validate parameters
		if (urlParams.length) params.length = parseInt(String(urlParams.length));

		// CRITICAL: Alphabet MUST be integer (0-4) in encrypted URLs
		// Strings are INVALID and indicate an error
		if (urlParams.alphabet !== undefined) {
			const alphabetValue = urlParams.alphabet;
			if (typeof alphabetValue === 'number' && isAlphabetInt(alphabetValue)) {
				// Convert integer to string for display
				params.alphabet = intToAlphabet(alphabetValue);
			} else {
				// STRING IS INVALID - encrypted URLs must use integers
				throw new Error(
					`Invalid alphabet format: expected integer 0-4, got ${typeof alphabetValue}`
				);
			}
		}

		if (urlParams.prefix) params.prefix = String(urlParams.prefix);
		if (urlParams.suffix) params.suffix = String(urlParams.suffix);

		// CRITICAL: Language MUST be integer (0-9) in encrypted URLs
		// Strings are INVALID and indicate an error
		if (urlParams.language !== undefined) {
			const langValue = urlParams.language;
			if (typeof langValue === 'number' && isMnemonicLangInt(langValue)) {
				// Convert integer to string for display
				params.language = intToMnemonicLang(langValue);
			} else {
				// STRING IS INVALID - encrypted URLs must use integers
				throw new Error(`Invalid language format: expected integer 0-9, got ${typeof langValue}`);
			}
		}

		if (urlParams.words) params.words = parseInt(String(urlParams.words));

		return params;
	}

	// Function to generate result from URL parameters
	async function generateFromParams() {
		// REACTIVE APPROACH: No proactive auth checks
		// If tokens are invalid/expired, the API call will get 401
		// and reactive interceptor will handle refresh/login automatically
		const { authStore } = await import('$lib/stores/auth');

		// First try to decrypt encrypted parameters
		let urlParams: Record<string, unknown> = {};

		// Try to decrypt if encrypted parameters are present
		const cipherToken = authStore.getCipherToken();
		const nonceToken = authStore.getNonceToken();
		const hmacKey = authStore.getHmacKey();

		// Check crypto token availability

		if (cipherToken && nonceToken && hmacKey) {
			try {
				const decryptedParams = await decryptPageParams(searchParams, {
					cipherToken,
					nonceToken,
					hmacKey
				});

				if (decryptedParams) {
					urlParams = decryptedParams;
				} else {
					// Failed to decrypt parameters - redirect to home
					goto('/');
					return;
				}
			} catch {
				// Error during decryption - redirect to home
				goto('/');
				return;
			}
		}

		// NO fallback to direct URL parameters - only encrypted params are supported
		// All parameters must come from decrypted data

		// Extract and validate endpoint
		const endpoint = String(urlParams.endpoint || '');

		if (!endpoint) {
			// No valid endpoint, redirecting to home
			goto('/');
			return;
		}

		// Check if we have a provided seed (only from decrypted params)
		const hasProvidedSeed = Boolean(urlParams.seed);
		usedProvidedSeed = hasProvidedSeed;

		// Build parameters object using DRY helper function
		const params = await buildParamsFromUrlParams(urlParams);

		// CREATE TEMPORARY resultState to show UI immediately (before API call)
		// This provides instant feedback to the user instead of blank screen
		setResult({
			value: '', // Empty - textarea will show "Loading..." via $isLoading check
			seed: undefined,
			otp: undefined,
			params: params,
			endpoint: endpoint,
			timestamp: new Date() // Temporary timestamp, will be updated with real one
		});

		// NOW start loading indicator and call API
		setLoading(true);

		try {
			const { api } = await import('$lib/api');

			const inputSeed = urlParams.seed ? String(urlParams.seed) : null;

			let response: import('$lib/types').CustomHashResponse;

			// If we have a seed, use POST to the appropriate endpoint
			if (inputSeed) {
				if (endpoint === 'custom' || endpoint === 'generate') {
					// Build seed request with only relevant params
					const customSeedRequest = {
						seed: inputSeed,
						endpoint
					} as import('$lib/types').SeedGenerateRequest;

					if (params.length) customSeedRequest.length = params.length as number;
					if (params.alphabet)
						customSeedRequest.alphabet = params.alphabet as import('$lib/types').AlphabetType;
					if (params.prefix) customSeedRequest.prefix = params.prefix as string;
					if (params.suffix) customSeedRequest.suffix = params.suffix as string;

					response = await api.generateWithSeed(customSeedRequest);
				} else if (endpoint === 'password') {
					const passwordSeedRequest = {
						seed: inputSeed
					} as import('$lib/types').SeedPasswordRequest;

					if (params.length) passwordSeedRequest.length = params.length as number;
					if (params.alphabet)
						passwordSeedRequest.alphabet = params.alphabet as 'no-look-alike' | 'full-with-symbols';

					response = await api.generatePasswordWithSeed(passwordSeedRequest);
				} else if (endpoint === 'api-key') {
					const apiKeySeedRequest = {
						seed: inputSeed
					} as import('$lib/types').SeedApiKeyRequest;

					if (params.length) apiKeySeedRequest.length = params.length as number;
					if (params.alphabet)
						apiKeySeedRequest.alphabet = params.alphabet as 'no-look-alike' | 'full';

					response = await api.generateApiKeyWithSeed(apiKeySeedRequest);
				} else if (endpoint === 'mnemonic') {
					const mnemonicSeedRequest = {
						seed: inputSeed
					} as import('$lib/types').SeedMnemonicRequest;

					if (params.language) mnemonicSeedRequest.language = params.language as string;
					if (params.words) mnemonicSeedRequest.words = params.words as 12 | 24;

					response = await api.generateMnemonicWithSeed(mnemonicSeedRequest);
				} else {
					throw new Error($_('common.unknownEndpoint'));
				}
			} else {
				// Call the appropriate API method based on endpoint
				switch (endpoint) {
					case 'custom':
					case 'generate':
						response = await api.generate(params);
						break;
					case 'password':
						response = await api.generatePassword(params);
						break;
					case 'api-key':
						response = await api.generateApiKey(params);
						break;
					case 'mnemonic':
						response = await api.generateMnemonic(params);
						break;
					default:
						throw new Error($_('common.unknownEndpoint'));
				}
			}

			// All endpoints now return CustomHashResponse
			const value = response.hash;
			const seed = response.seed;
			const otp = response.otp;
			const responseTimestamp = new Date(response.timestamp * 1000); // Convert from seconds to ms

			// Set the result state
			setResult({
				value,
				seed,
				otp,
				params,
				endpoint,
				timestamp: responseTimestamp
			});
		} catch {
			// For ANY error, redirect to home with flash message as requested

			// Handle API generation errors

			// Always redirect to home on errors
			await goto('/');
			return;
		} finally {
			setLoading(false);
		}
	}

	async function copyToClipboard() {
		if (!$resultState?.value) return;
		logger.info('[Click] Copy result to clipboard');

		try {
			await navigator.clipboard.writeText($resultState.value);
			copySuccess = true;

			// Clear success state after 2 seconds
			clearTimeout(copyTimeout);
			copyTimeout = setTimeout(() => {
				copySuccess = false;
			}, 2000);
		} catch {
			// Fallback for older browsers
			try {
				const textArea = document.createElement('textarea');
				textArea.value = $resultState.value;
				document.body.appendChild(textArea);
				textArea.select();
				document.execCommand('copy');
				document.body.removeChild(textArea);
				copySuccess = true;
				clearTimeout(copyTimeout);
				copyTimeout = setTimeout(() => {
					copySuccess = false;
				}, 2000);
			} catch {
				// Fallback copy failed - ignore error
			}
		}
	}

	// Function to get endpoint display name
	function getEndpointDisplayName(endpoint: string): string {
		switch (endpoint) {
			case 'custom':
				return $_('custom.title');
			case 'generate':
				return $_('custom.title');
			case 'password':
				return $_('password.title');
			case 'api-key':
				return $_('apiKey.title');
			case 'mnemonic':
				return $_('mnemonic.title');
			default:
				return endpoint;
		}
	}

	function getEndpointIcon(endpoint: string): string {
		switch (endpoint) {
			case 'custom':
				return '🎲';
			case 'generate':
				return '🎲';
			case 'password':
				return '🔐';
			case 'api-key':
				return '🔑';
			case 'mnemonic':
				return '💾';
			default:
				return '📝';
		}
	}

	function getEndpointColor(endpoint: string): string {
		switch (endpoint) {
			case 'custom':
				return 'blue';
			case 'generate':
				return 'blue';
			case 'password':
				return 'blue';
			case 'api-key':
				return 'blue';
			case 'mnemonic':
				return 'blue';
			default:
				return 'gray';
		}
	}

	// Reactive parameter key translation that updates when language changes
	let translateParameterKey = $derived((key: string): string => {
		const translations: Record<string, string> = {
			length: $_('common.length'),
			alphabet: $_('common.alphabet'),
			prefix: $_('custom.prefix') || 'Prefix',
			suffix: $_('custom.suffix') || 'Suffix',
			language: $_('mnemonic.language'),
			words: $_('mnemonic.wordCount')
		};

		return translations[key] || key.replace(/([A-Z])/g, ' $1').trim();
	});

	// Reactive parameter value translation that updates when language changes
	let translateParameterValue = $derived(
		(key: string, value: string | number | boolean): string => {
			if (typeof value === 'boolean') {
				return value ? $_('common.yes') || 'Yes' : $_('common.no') || 'No';
			}

			// Translate alphabet types
			if (key === 'alphabet' && typeof value === 'string') {
				return $_(`alphabets.${value}`) || value;
			}

			// Translate language values
			if (key === 'language' && typeof value === 'string') {
				return $_(`mnemonic.languages.${value}`) || value;
			}

			// Translate words count with descriptive text
			if (key === 'words' && typeof value === 'number') {
				return value === 12 ? $_('mnemonic.words12') : $_('mnemonic.words24');
			}

			return String(value);
		}
	);

	async function handleAdjustSettings() {
		if (!$resultState) {
			goto('/');
			return;
		}
		logger.info('[Click] Adjust settings');

		// If there's no seed, go directly without dialog
		if (!$resultState.seed) {
			logger.info('[Navigation] Redirecting to form (no seed)');
			goto(await getPreviousPath());
			return;
		}

		// Show seed dialog using new dialog system
		logger.info('[Dialog] Opening seed reuse dialog');
		dialogStore.show('seed', { onSeedChoice: handleSeedChoice });
	}

	async function handleSeedChoice(reuseExistingSeed: boolean) {
		logger.info(`[Dialog] Seed choice: ${reuseExistingSeed ? 'reuse' : 'new'}`);
		logger.info('[Dialog] Closing seed dialog');
		dialogStore.close();
		if (reuseExistingSeed) {
			// Include seed in the URL parameters
			logger.info('[Navigation] Redirecting to form (with seed)');
			goto(await getPreviousPathWithSeed());
		} else {
			// Don't include seed in URL parameters
			logger.info('[Navigation] Redirecting to form (without seed)');
			goto(await getPreviousPath());
		}
	}

	async function getPreviousPathWithSeed(): Promise<string> {
		if (!$resultState) return '/';

		// Map endpoint names to actual route paths
		const endpointRoutes: Record<string, string> = {
			custom: '/custom',
			generate: '/custom', // backward compatibility
			password: '/password',
			'api-key': '/api-key',
			mnemonic: '/mnemonic'
		};

		const basePath = endpointRoutes[$resultState.endpoint] || '/';

		if ($resultState.params && Object.keys($resultState.params).length > 0) {
			// Import conversion functions
			const { alphabetToInt, mnemonicLangToInt } = await import('$lib/types');

			const configParams: Record<string, unknown> = {};

			// Add common parameters
			if ($resultState.params.length) {
				configParams.length = $resultState.params.length;
			}
			if ($resultState.params.alphabet) {
				// CRITICAL: Convert alphabet string to integer BEFORE encryption
				const alphabetStr = String($resultState.params.alphabet);
				configParams.alphabet = alphabetToInt(
					alphabetStr as import('$lib/types').AlphabetTypeString
				);
			}

			// Add endpoint-specific parameters
			if ($resultState.endpoint === 'custom' || $resultState.endpoint === 'generate') {
				if ($resultState.params.prefix) {
					configParams.prefix = String($resultState.params.prefix);
				}
				if ($resultState.params.suffix) {
					configParams.suffix = String($resultState.params.suffix);
				}
			} else if ($resultState.endpoint === 'mnemonic') {
				if ($resultState.params.language) {
					// CRITICAL: Convert language string to integer BEFORE encryption
					const langStr = String($resultState.params.language);
					configParams.language = mnemonicLangToInt(
						langStr as import('$lib/types').MnemonicLanguageString
					);
				}
				if ($resultState.params.words) {
					configParams.words = $resultState.params.words;
				}
			}

			// Add seed if available
			if ($resultState.seed) {
				configParams.seed = $resultState.seed;
			}

			// Get crypto tokens for parameter encryption
			const cipherToken = authStore.getCipherToken();
			const nonceToken = authStore.getNonceToken();
			const hmacKey = authStore.getHmacKey();

			if (cipherToken && nonceToken && hmacKey && Object.keys(configParams).length > 0) {
				// Create encrypted URL for privacy
				const encryptedUrl = await createEncryptedUrl(basePath, configParams, {
					cipherToken,
					nonceToken,
					hmacKey
				});

				return encryptedUrl;
			} else if (Object.keys(configParams).length > 0) {
				// Fallback: create traditional URL
				const urlParams = new URLSearchParams();
				Object.entries(configParams).forEach(([key, value]) => {
					urlParams.set(key, String(value));
				});
				return `${basePath}?${urlParams.toString()}`;
			}
		}

		return basePath;
	}

	async function getPreviousPath(): Promise<string> {
		if (!$resultState) return '/';

		// Map endpoint names to actual route paths
		const endpointRoutes: Record<string, string> = {
			custom: '/custom',
			generate: '/custom', // backward compatibility
			password: '/password',
			'api-key': '/api-key',
			mnemonic: '/mnemonic'
		};

		const basePath = endpointRoutes[$resultState.endpoint] || '/';

		// Build parameters object (without seed)
		if ($resultState.params && Object.keys($resultState.params).length > 0) {
			// Import conversion functions
			const { alphabetToInt, mnemonicLangToInt } = await import('$lib/types');

			const configParams: Record<string, unknown> = {};

			// Add common parameters
			if ($resultState.params.length) {
				configParams.length = $resultState.params.length;
			}
			if ($resultState.params.alphabet) {
				// CRITICAL: Convert alphabet string to integer BEFORE encryption
				const alphabetStr = String($resultState.params.alphabet);
				configParams.alphabet = alphabetToInt(
					alphabetStr as import('$lib/types').AlphabetTypeString
				);
			}

			// Add endpoint-specific parameters
			if ($resultState.endpoint === 'custom' || $resultState.endpoint === 'generate') {
				if ($resultState.params.prefix) {
					configParams.prefix = String($resultState.params.prefix);
				}
				if ($resultState.params.suffix) {
					configParams.suffix = String($resultState.params.suffix);
				}
			} else if ($resultState.endpoint === 'mnemonic') {
				if ($resultState.params.language) {
					// CRITICAL: Convert language string to integer BEFORE encryption
					const langStr = String($resultState.params.language);
					configParams.language = mnemonicLangToInt(
						langStr as import('$lib/types').MnemonicLanguageString
					);
				}
				if ($resultState.params.words) {
					configParams.words = $resultState.params.words;
				}
			}

			// Get crypto tokens for parameter encryption
			const cipherToken = authStore.getCipherToken();
			const nonceToken = authStore.getNonceToken();
			const hmacKey = authStore.getHmacKey();

			if (cipherToken && nonceToken && hmacKey && Object.keys(configParams).length > 0) {
				// Create encrypted URL for privacy
				return await createEncryptedUrl(basePath, configParams, {
					cipherToken,
					nonceToken,
					hmacKey
				});
			} else if (Object.keys(configParams).length > 0) {
				// Fallback: create traditional URL
				const urlParams = new URLSearchParams();
				Object.entries(configParams).forEach(([key, value]) => {
					urlParams.set(key, String(value));
				});
				return `${basePath}?${urlParams.toString()}`;
			}
		}

		return basePath;
	}

	async function handleShare() {
		if (!$resultState) {
			goto('/');
			return;
		}
		logger.info('[Click] Share hash to create shared secret');

		// Only allow sharing if we have a seed (to reproduce the exact hash)
		if (!$resultState.seed) {
			logger.warn('[Share] Cannot share without seed - hash cannot be reproduced');
			// Could show a flash message here if needed
			return;
		}

		// Build parameters object with seed for reproduction
		// Import conversion functions
		const { alphabetToInt, mnemonicLangToInt } = await import('$lib/types');

		const shareParams: Record<string, unknown> = {
			endpoint: $resultState.endpoint
		};

		// Add common parameters
		if ($resultState.params.length) {
			shareParams.length = $resultState.params.length;
		}
		if ($resultState.params.alphabet) {
			// CRITICAL: Convert alphabet string to integer BEFORE encryption
			const alphabetStr = String($resultState.params.alphabet);
			shareParams.alphabet = alphabetToInt(
				alphabetStr as import('$lib/types').AlphabetTypeString
			);
		}

		// Add endpoint-specific parameters
		if ($resultState.endpoint === 'custom' || $resultState.endpoint === 'generate') {
			if ($resultState.params.prefix) {
				shareParams.prefix = String($resultState.params.prefix);
			}
			if ($resultState.params.suffix) {
				shareParams.suffix = String($resultState.params.suffix);
			}
		} else if ($resultState.endpoint === 'mnemonic') {
			if ($resultState.params.language) {
				// CRITICAL: Convert language string to integer BEFORE encryption
				const langStr = String($resultState.params.language);
				shareParams.language = mnemonicLangToInt(
					langStr as import('$lib/types').MnemonicLanguageString
				);
			}
			if ($resultState.params.words) {
				shareParams.words = $resultState.params.words;
			}
		}

		// Add seed for exact reproduction
		if ($resultState.seed) {
			shareParams.seed = $resultState.seed;
		}

		// Get crypto tokens for parameter encryption (REQUIRED - no insecure fallback)
		const cipherToken = authStore.getCipherToken();
		const nonceToken = authStore.getNonceToken();
		const hmacKey = authStore.getHmacKey();

		if (!cipherToken || !nonceToken || !hmacKey) {
			logger.error('[Share] Missing crypto tokens - cannot create secure URL');
			// Missing crypto tokens - abort (no insecure fallback)
			return;
		}

		if (Object.keys(shareParams).length > 0) {
			// Create encrypted URL for privacy (ONLY secure method allowed)
			const encryptedUrl = await createEncryptedUrl('/shared-secret', shareParams, {
				cipherToken,
				nonceToken,
				hmacKey
			});

			logger.info('[Navigation] Redirecting to: /shared-secret (encrypted params)');
			goto(encryptedUrl);
		} else {
			// No parameters to share
			goto('/shared-secret');
		}
	}

	async function regenerateHash() {
		if (!$resultState || $isLoading) return;
		logger.info('[Click] Regenerate hash with same parameters');

		// Reset copy success state immediately
		copySuccess = false;
		setLoading(true);

		try {
			const { api } = await import('$lib/api');
			let response: import('$lib/types').CustomHashResponse;

			// Build parameters excluding seed - force GET request by not including seed
			const paramsForGeneration = { ...$resultState.params };
			delete paramsForGeneration.seed; // Ensure no seed is passed for GET request

			// Call the appropriate API method based on endpoint
			switch ($resultState.endpoint) {
				case 'custom':
				case 'generate':
					response = await api.generate(paramsForGeneration);
					break;
				case 'password':
					response = await api.generatePassword(paramsForGeneration);
					break;
				case 'api-key':
					response = await api.generateApiKey(paramsForGeneration);
					break;
				case 'mnemonic':
					response = await api.generateMnemonic(paramsForGeneration);
					break;
				default:
					throw new Error($_('common.unknownEndpoint'));
			}

			// All endpoints now return CustomHashResponse
			const value = response.hash;
			const seed = response.seed;
			const otp = response.otp;
			const responseTimestamp = new Date(response.timestamp * 1000); // Convert from seconds to ms

			// Update result with new value but keep same parameters and endpoint
			setResult({
				value,
				seed,
				otp,
				params: paramsForGeneration,
				endpoint: $resultState.endpoint,
				timestamp: responseTimestamp
			});

			// Reset copy success state
			copySuccess = false;
		} catch {
			// For ANY error, redirect to home with flash message as requested

			// Handle API regeneration errors

			// Always redirect to home on errors
			await goto('/');
			return;
		} finally {
			setLoading(false);
		}
	}
</script>

<svelte:head>
	<title>{$_('common.result')}</title>
</svelte:head>

<!-- <svelte:window on:keydown={handleKeydown} /> -->

{#if $resultState}
	{@const color = getEndpointColor($resultState.endpoint)}
	<div
		class="min-h-screen bg-gradient-to-br from-{color}-50 to-{color}-100 dark:from-gray-900 dark:to-gray-800"
	>
		<div class="container mx-auto px-4 py-8">
			<!-- Header -->
			<div class="mb-8">
				<div class="text-center">
					<div
						class="inline-flex items-center justify-center w-16 h-16 bg-{color}-600 rounded-full mb-6"
					>
						<span class="text-2xl text-white">{getEndpointIcon($resultState.endpoint)}</span>
					</div>
					<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
						{$_('common.result')}
					</h1>
				</div>
			</div>

			<!-- Flash Messages -->
			<FlashMessages />

			<!-- Result Display -->
			<div class="max-w-4xl mx-auto">
				<div
					class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6 mb-6"
				>
					<!-- Result Value -->
					<div class="mb-6">
						<label
							for="generated-value"
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3"
						>
							{$_('common.generatedValue')}
						</label>
						<div class="relative">
							<textarea
								id="generated-value"
								readonly
								value={$isLoading ? $_('common.loading') + '...' : $resultState.value}
								class="w-full p-4 pb-12 bg-gray-50 dark:bg-gray-900 border border-gray-300 dark:border-gray-600 rounded-lg font-mono text-sm resize-none focus:ring-2 focus:ring-{color}-500 focus:border-{color}-500 min-h-[100px] {$isLoading
									? 'text-gray-500 dark:text-gray-400'
									: ''}"
								onclick={(e) => (e.target as HTMLTextAreaElement)?.select()}
							></textarea>
							{#if !$isLoading}
								<!-- RTL-aware action buttons container -->
								<div
									class="absolute bottom-3 {$isRTL
										? 'left-3'
										: 'right-3'} flex items-center gap-2"
								>
									<!-- Share button (only if seed is available) -->
									{#if $resultState.seed}
										<button
											onclick={handleShare}
											class="px-2 py-1 text-xs font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-1 bg-purple-600 hover:bg-purple-700 text-white"
											title={$_('common.shareToCreateSecret')}
										>
											<Iconize
												conf={{
													icon: 'share',
													iconSize: 'w-3 h-3'
												}}
											>
												{$_('common.share')}
											</Iconize>
										</button>
									{/if}

									<!-- Copy button -->
									<button
										onclick={copyToClipboard}
										class="px-2 py-1 text-xs font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-1 {copySuccess
											? 'bg-green-600 hover:bg-green-700 text-white'
											: 'bg-blue-600 hover:bg-blue-700 text-white'}"
									>
										<Iconize
											conf={{
												icon: copySuccess ? 'check' : 'copy',
												iconSize: 'w-3 h-3'
											}}
										>
											{copySuccess ? $_('common.copied') : $_('common.copy')}
										</Iconize>
									</button>
								</div>
							{/if}
						</div>
					</div>

					<!-- Metadata -->
					<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
						<!-- Generation Details -->
						<div>
							<!-- Header with toggle for mobile -->
							<button
								onclick={toggleGenerationDetails}
								class="w-full text-left flex items-center justify-between md:pointer-events-none md:cursor-default mb-3"
							>
								<h3 class="text-lg font-semibold text-gray-900 dark:text-white">
									{$_('common.generationDetails')}
								</h3>
								<!-- Toggle icon - only visible on mobile -->
								<Icon
									name="chevron-down"
									size="w-5 h-5"
									placeholder="auto"
									class="text-gray-500 dark:text-gray-400 md:hidden transition-transform duration-200 {showGenerationDetails
										? 'rotate-180'
										: ''} {$isRTL ? 'rtl-flip-chevron' : ''}"
								/>
							</button>

							<!-- Content - collapsible on mobile, always visible on desktop -->
							<dl class="space-y-2 {showGenerationDetails ? 'block' : 'hidden'} md:block">
								<div>
									<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">
										{$_('common.type')}
									</dt>
									<dd class="text-sm text-gray-900 dark:text-white">
										{getEndpointDisplayName($resultState.endpoint)}
									</dd>
								</div>
								<div>
									<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">
										{$_('common.length')}
									</dt>
									<dd class="text-sm text-gray-900 dark:text-white">
										{$resultState.value.length}
										{$_('common.characters')}
									</dd>
								</div>
								<div>
									<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">
										{$_('common.generated')}
									</dt>
									<dd class="text-sm text-gray-900 dark:text-white">
										{#if $resultState.timestamp}
											<DateTimeLocalized timestamp={$resultState.timestamp} />
										{/if}
									</dd>
								</div>
								{#if $resultState.seed}
									<div>
										<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">
											{$_('common.seed')}
										</dt>
										<dd class="text-xs font-mono text-gray-900 dark:text-white break-all">
											{$resultState.seed}
										</dd>
									</div>
								{/if}
								{#if $resultState.otp}
									<div>
										<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">
											{$_('common.otp')}
										</dt>
										<dd class="text-sm font-mono text-gray-900 dark:text-white">
											{$resultState.otp}
										</dd>
									</div>
								{/if}
							</dl>
						</div>

						<!-- Parameters Used -->
						<div>
							<!-- Header with toggle for mobile -->
							<button
								onclick={toggleParametersUsed}
								class="w-full text-left flex items-center justify-between md:pointer-events-none md:cursor-default mb-3"
							>
								<h3 class="text-lg font-semibold text-gray-900 dark:text-white">
									{$_('common.parametersUsed')}
								</h3>
								<!-- Toggle icon - only visible on mobile -->
								<Icon
									name="chevron-down"
									size="w-5 h-5"
									placeholder="auto"
									class="text-gray-500 dark:text-gray-400 md:hidden transition-transform duration-200 {showParametersUsed
										? 'rotate-180'
										: ''} {$isRTL ? 'rtl-flip-chevron' : ''}"
								/>
							</button>

							<!-- Content - collapsible on mobile, always visible on desktop -->
							<dl class="space-y-2 {showParametersUsed ? 'block' : 'hidden'} md:block">
								{#each Object.entries($resultState.params) as [key, value]}
									{#if value !== undefined && value !== null && value !== ''}
										<div>
											<dt class="text-sm font-medium text-gray-500 dark:text-gray-400 capitalize">
												{translateParameterKey(key)}
											</dt>
											<dd class="text-sm text-gray-900 dark:text-white">
												{translateParameterValue(key, value)}
											</dd>
										</div>
									{/if}
								{/each}
							</dl>
						</div>
					</div>

					<!-- Actions -->
					<div class="flex flex-col sm:flex-row gap-4 mt-6">
						<!-- RTL-aware regenerate button - only show if not using a provided seed -->
						{#if !usedProvidedSeed}
							<button
								onclick={regenerateHash}
								disabled={$isLoading}
								class="flex-1 text-white px-6 py-4 rounded-lg font-semibold border-none transition-all duration-200 flex items-center justify-center gap-2 {$isLoading
									? 'bg-gray-400 cursor-not-allowed'
									: 'bg-blue-600 hover:bg-blue-700 cursor-pointer hover:shadow-lg'}"
							>
								<Iconize
									conf={{
										icon: 'refresh',
										iconSize: 'w-5 h-5'
									}}
								>
									{$_('common.generateAnother')}
								</Iconize>
							</button>
						{/if}

						<!-- RTL-aware adjust settings button -->
						<button
							onclick={handleAdjustSettings}
							class="flex-1 bg-gray-500 hover:bg-gray-600 text-white px-6 py-4 rounded-lg font-semibold border-none cursor-pointer hover:shadow-lg transition-all duration-200 flex items-center justify-center gap-2 {usedProvidedSeed
								? 'w-full'
								: ''}"
						>
							<Iconize
								conf={{
									icon: 'settings',
									iconSize: 'w-5 h-5'
								}}
							>
								{$_('common.adjustSettings')}
							</Iconize>
						</button>

						<!-- Back to menu button -->
						<BackToMenuButton />
					</div>
				</div>
			</div>
		</div>
	</div>

	<!-- Seed dialog removed - using new dialog system now -->
{:else if $error}
	<div
		class="min-h-screen bg-gradient-to-br from-red-50 to-red-100 dark:from-gray-900 dark:to-gray-800"
	>
		<div class="container mx-auto px-4 py-8">
			<div class="max-w-2xl mx-auto text-center">
				<div class="inline-flex items-center justify-center w-16 h-16 bg-red-600 rounded-full mb-6">
					<span class="text-2xl text-white">❌</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">
					{$_('common.error')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300 mb-8">
					{$error}
				</p>
				<BackButton to="/" />
			</div>
		</div>
	</div>
{/if}
