<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import Footer from '$lib/components/Footer.svelte';
	// import Button from '$lib/components/Button.svelte';
	import GenerateButton from '$lib/components/GenerateButton.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import AlphabetSelector from '$lib/components/AlphabetSelector.svelte';
	import TextInput from '$lib/components/TextInput.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { dialogStore } from '$lib/stores/dialog';
	import { isLoading, resultState } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import { authStore } from '$lib/stores/auth';
	import type { GenerateParams, AlphabetType } from '$lib/types';
	import { decryptPageParams, createEncryptedUrl } from '$lib/crypto';

	// Default values
	function getDefaultParams(): GenerateParams {
		return {
			length: 21,
			alphabet: 'base58',
			prefix: '',
			suffix: '',
			raw: true
		};
	}

	// Form state - will be initialized in onMount
	let params: GenerateParams = $state(getDefaultParams());
	let urlProvidedSeed: string = $state(''); // Seed from URL parameters (read-only)

	// Clean initialization without debug messages

	// Get URL parameters reactively
	let searchParams = $derived($page.url.searchParams);

	// Function to validate alphabet parameter
	function isValidAlphabet(value: string): value is AlphabetType {
		return ['base58', 'no-look-alike', 'full', 'full-with-symbols', 'numeric'].includes(value);
	}

	// Reactive alphabet options that update when language changes
	let alphabetOptions = $derived([
		{
			value: 'base58' as AlphabetType,
			label: $_('alphabets.base58'),
			description: $_('custom.bitcoinDescription')
		},
		{
			value: 'no-look-alike' as AlphabetType,
			label: $_('alphabets.no-look-alike'),
			description: $_('custom.maxReadabilityDescription')
		},
		{
			value: 'full' as AlphabetType,
			label: $_('alphabets.full'),
			description: $_('custom.completeAlphanumericDescription')
		},
		{
			value: 'full-with-symbols' as AlphabetType,
			label: $_('alphabets.full-with-symbols'),
			description: $_('custom.maxEntropyDescription')
		},
		{
			value: 'numeric' as AlphabetType,
			label: $_('alphabets.numeric'),
			description: $_('custom.numericDescription')
		}
	]);

	// Seed validation functions (commented out as we now only show seeds read-only)
	// function isValidHexSeed(seed: string): boolean {
	// 	if (!seed) return true; // Empty seed is valid (optional)
	// 	// Must be exactly 64 characters (32 bytes in hex) and only hex characters
	// 	return /^[0-9a-fA-F]{64}$/.test(seed);
	// }

	// Validation
	let lengthValid = $derived(params.length && params.length >= 2 && params.length <= 128);
	let prefixValid = $derived(!params.prefix || params.prefix.length <= 32);
	let suffixValid = $derived(!params.suffix || params.suffix.length <= 32);
	let formValid = $derived(lengthValid && prefixValid && suffixValid);

	let pendingGenerationParams: Record<string, unknown> | null = null;

	async function handleGenerate(event: Event) {
		event.preventDefault();

		if (!formValid) {
			return;
		}

		// Verify authentication with automatic refresh
		const isAuthenticated = await authStore.ensureAuthenticated();

		if (!isAuthenticated) {
			// No se pudo autenticar - mostrar diálogo de autenticación
			pendingGenerationParams = {
				endpoint: 'custom',
				length: params.length ?? 21,
				alphabet: params.alphabet ?? 'base58',
				...(params.prefix && { prefix: params.prefix }),
				...(params.suffix && { suffix: params.suffix }),
				...(urlProvidedSeed && { seed: urlProvidedSeed })
			};

			// Clear any residual auth data before asking for email (defensive security)
			authStore.clearPreventiveAuthData();

			const authConfig = {
				destination: {
					route: '/result',
					params: pendingGenerationParams
				}
			};
			dialogStore.show('auth', authConfig);
			return;
		}

		// User authenticated - proceed with generation
		await performGeneration();
	}

	/**
	 * Perform the actual generation (separated for reuse after auth)
	 */
	async function performGeneration() {
		proceedWithGeneration();
	}

	async function proceedWithGeneration() {
		// Create parameters object for result page
		const resultParams: Record<string, any> = {
			endpoint: 'custom',
			length: params.length ?? 21,
			alphabet: params.alphabet ?? 'base58'
		};

		// Add optional parameters
		if (params.prefix) resultParams.prefix = params.prefix;
		if (params.suffix) resultParams.suffix = params.suffix;
		if (urlProvidedSeed) resultParams.seed = urlProvidedSeed;

		// Get crypto tokens for parameter encryption
		const cipherToken = authStore.getCipherToken();
		const nonceToken = authStore.getNonceToken();
		const hmacKey = authStore.getHmacKey();

		if (cipherToken && nonceToken && hmacKey) {
			// Creating secure URL and navigating to result

			// Create encrypted URL for privacy
			const encryptedUrl = await createEncryptedUrl('/result', resultParams, {
				cipherToken,
				nonceToken,
				hmacKey
			});

			// Navigating to result page with encrypted parameters

			goto(encryptedUrl);
		} else {
			// ERROR: Crypto tokens required for secure navigation
			// Missing crypto tokens - cannot create secure URL

			goto('/'); // Return to home instead of unsecure URL
		}
	}

	/**
	 * Handle successful authentication
	 */
	// Listen for authentication success event from DialogContainer
	function handleAuthenticated() {
		// Authentication data is handled by the auth system automatically

		// Perform the generation with the pending parameters
		if (pendingGenerationParams) {
			// Update params with pending values if they exist
			if (pendingGenerationParams.length) params.length = Number(pendingGenerationParams.length);
			if (pendingGenerationParams.alphabet)
				params.alphabet = String(pendingGenerationParams.alphabet) as AlphabetType;
			if (pendingGenerationParams.prefix) params.prefix = String(pendingGenerationParams.prefix);
			if (pendingGenerationParams.suffix) params.suffix = String(pendingGenerationParams.suffix);
			if (pendingGenerationParams.seed) urlProvidedSeed = String(pendingGenerationParams.seed);

			pendingGenerationParams = null;

			// Perform generation
			performGeneration();
		}
	}

	// Add event listener for authentication
	if (typeof globalThis.window !== 'undefined') {
		globalThis.window.addEventListener('authenticated', handleAuthenticated as EventListener);
	}

	// Initialize params based on navigation source
	onMount(() => {
		// Check if we're coming from result page with existing params
		if (
			$resultState &&
			($resultState.endpoint === 'custom' || $resultState.endpoint === 'generate') &&
			$resultState.params
		) {
			// Coming from result page - use existing params
			params = { ...$resultState.params } as GenerateParams;
		} else {
			// Coming from menu or fresh load - use defaults
			params = getDefaultParams();
		}

		// Override with URL parameters if present
		// First try to decrypt encrypted parameters
		let urlParams: Record<string, any> = {};

		// Try to decrypt if encrypted parameters are present
		const cipherToken = authStore.getCipherToken();
		const nonceToken = authStore.getNonceToken();
		const hmacKey = authStore.getHmacKey();

		if (cipherToken && nonceToken && hmacKey) {
			const decryptedParams = decryptPageParams(searchParams, {
				cipherToken,
				nonceToken,
				hmacKey
			});

			if (decryptedParams) {
				urlParams = decryptedParams;
			}
		}

		// NO fallback to direct URL parameters - only encrypted params are supported
		// All parameters must come from decrypted data

		// Apply URL parameters to form state
		if (urlParams.length) {
			const lengthNum = parseInt(String(urlParams.length));
			if (!isNaN(lengthNum) && lengthNum >= 2 && lengthNum <= 128) {
				params.length = lengthNum;
			}
		}

		if (urlParams.alphabet && isValidAlphabet(String(urlParams.alphabet))) {
			params.alphabet = String(urlParams.alphabet) as AlphabetType;
		}

		if (urlParams.prefix !== undefined && String(urlParams.prefix).length <= 32) {
			params.prefix = String(urlParams.prefix);
		}

		if (urlParams.suffix !== undefined && String(urlParams.suffix).length <= 32) {
			params.suffix = String(urlParams.suffix);
		}

		if (urlParams.seed) {
			urlProvidedSeed = String(urlParams.seed);
		}
	});
</script>

<svelte:head>
	<title>{$_('custom.title')}</title>
</svelte:head>

<div
	class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"
>
	<div class="container mx-auto px-4 py-8">
		<!-- Header -->
		<div class="mb-8">
			<div class="text-center">
				<div
					class="inline-flex items-center justify-center w-12 h-12 bg-blue-600 rounded-full mb-4"
				>
					<span class="text-xl text-white">🎲</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
					{$_('custom.title')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300">
					{$_('custom.description')}
				</p>
			</div>
		</div>

		<!-- Flash Messages -->
		<FlashMessages />

		<!-- Auth Guard: wraps the form -->
		<!-- Form -->
		<div class="max-w-2xl mx-auto">
			<div
				class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6"
			>
				<form onsubmit={handleGenerate} class="space-y-6">
					<!-- Length -->
					<div>
						<label
							for="length"
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
						>
							{$_('custom.length')} (2-128)
						</label>
						<div class="flex items-center gap-4">
							<input
								type="range"
								id="length"
								bind:value={params.length}
								min="2"
								max="128"
								class="flex-1 h-2 bg-blue-600 rounded appearance-none outline-none slider"
							/>
							<span
								class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center"
								>{params.length}</span
							>
						</div>
						{#if !lengthValid}
							<p class="text-red-500 text-sm mt-1">{$_('custom.lengthMustBeBetween')}</p>
						{/if}
					</div>

					<!-- Alphabet -->
					<AlphabetSelector
						bind:value={params.alphabet}
						options={alphabetOptions}
						label={$_('custom.alphabet')}
						id="alphabet"
					/>

					<!-- Prefix -->
					<TextInput
						id="prefix"
						label={$_('custom.prefix') +
							' (' +
							$_('common.cannotExceed') +
							' 32 ' +
							$_('common.characters') +
							')'}
						bind:value={params.prefix}
						placeholder={$_('common.optionalPrefix')}
						maxlength={32}
						isValid={prefixValid}
						errorMessage={$_('common.prefixCannotExceed')}
					/>

					<!-- Suffix -->
					<TextInput
						id="suffix"
						label={$_('custom.suffix') +
							' (' +
							$_('common.cannotExceed') +
							' 32 ' +
							$_('common.characters') +
							')'}
						bind:value={params.suffix}
						placeholder={$_('common.optionalSuffix')}
						maxlength={32}
						isValid={suffixValid}
						errorMessage={$_('common.suffixCannotExceed')}
					/>

					<!-- Seed (only show if provided via URL) -->
					{#if urlProvidedSeed}
						<div>
							<label
								for="seed"
								class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
							>
								{$_('common.seed')}
							</label>
							<input
								id="seed"
								type="text"
								value={urlProvidedSeed}
								disabled
								class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-800 text-gray-600 dark:text-gray-400 cursor-not-allowed"
							/>
						</div>
					{/if}

					<!-- Action Buttons -->
					<div class="flex flex-col sm:flex-row gap-4 mt-4">
						<!-- Generate hash button -->
						<GenerateButton
							type="submit"
							disabled={!formValid || $isLoading || $authStore.isRefreshing}
							loading={$isLoading || $authStore.isRefreshing}
							text={$_('custom.generateHash')}
							loadingText={$authStore.isRefreshing
								? $_('auth.authenticating')
								: $_('common.loading') + '...'}
						/>

						<!-- Back to menu button -->
						<BackToMenuButton />
					</div>
				</form>
			</div>
		</div>

		<!-- Footer with Version Information -->
		<Footer />
	</div>
</div>

<!-- Authentication handled by global DialogContainer -->
