<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import Footer from '$lib/components/Footer.svelte';
	// import Button from '$lib/components/Button.svelte';
	import GenerateButton from '$lib/components/GenerateButton.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import AlphabetSelector from '$lib/components/AlphabetSelector.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { dialogStore } from '$lib/stores/dialog';
	import { isLoading, resultState } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import { authStore } from '$lib/stores/auth';
	import type { PasswordParams } from '$lib/types';
	import { decryptPageParams, createEncryptedUrl } from '$lib/crypto';

	// Default values
	function getDefaultParams(): PasswordParams {
		return {
			length: 21, // Minimum for full-with-symbols alphabet
			alphabet: 'full-with-symbols',
			raw: true
		};
	}

	// Form state - will be initialized in onMount
	let params: PasswordParams = $state(getDefaultParams());
	let urlProvidedSeed: string = $state(''); // Seed from URL parameters (read-only)

	// Get URL parameters reactively
	let searchParams = $derived($page.url.searchParams);

	// Function to validate alphabet parameter
	function isValidPasswordAlphabet(value: string): value is 'full-with-symbols' | 'no-look-alike' {
		return ['full-with-symbols', 'no-look-alike'].includes(value);
	}

	// Reactive alphabet options that update when language changes
	let alphabetOptions = $derived([
		{
			value: 'full-with-symbols' as const,
			label: $_('alphabets.full-with-symbols'),
			description: $_('password.maxSecurityDescription')
		},
		{
			value: 'no-look-alike' as const,
			label: $_('alphabets.no-look-alike'),
			description: $_('password.easyReadDescription')
		}
	]);

	// Dynamic minimum length based on alphabet
	let minLength = $derived(params.alphabet === 'full-with-symbols' ? 21 : 24);
	let lengthValid = $derived(params.length && params.length >= minLength && params.length <= 44);
	let formValid = $derived(lengthValid);

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
				endpoint: 'password',
				length: params.length ?? 21,
				alphabet: params.alphabet ?? 'full-with-symbols',
				...(urlProvidedSeed && { seed: urlProvidedSeed })
			};

			// Clear any residual auth data before asking for email (defensive security)
			authStore.clearPreventiveAuthData();

			dialogStore.show('auth', pendingGenerationParams);
			return;
		}

		// User authenticated - proceed with generation
		proceedWithGeneration();
	}

	async function proceedWithGeneration() {
		// Create parameters object for result page
		const resultParams: Record<string, any> = {
			endpoint: 'password',
			length: params.length ?? 21,
			alphabet: params.alphabet ?? 'full-with-symbols'
		};

		// Add seed if provided from URL
		if (urlProvidedSeed) resultParams.seed = urlProvidedSeed;

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
			goto(encryptedUrl);
		} else {
			// Fallback: create traditional URL (should not happen with proper auth)
			const urlParams = new URLSearchParams();
			Object.entries(resultParams).forEach(([key, value]) => {
				urlParams.set(key, String(value));
			});
			goto(`/result?${urlParams.toString()}`);
		}
	}

	// Update length when alphabet changes with smooth adjustment
	function handleAlphabetChange() {
		// Use setTimeout to avoid reactivity issues
		setTimeout(() => {
			const newMinLength = params.alphabet === 'full-with-symbols' ? 21 : 24;
			if (params.length! < newMinLength) {
				// Force reactivity by reassigning the entire params object
				params = { ...params, length: newMinLength };
			}
		}, 0);
	}

	// Initialize params based on navigation source
	onMount(() => {
		// Check if we're coming from result page with existing params
		if ($resultState && $resultState.endpoint === 'password' && $resultState.params) {
			// Coming from result page - use existing params
			params = { ...$resultState.params } as PasswordParams;
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
			if (!isNaN(lengthNum) && lengthNum >= 21 && lengthNum <= 44) {
				params.length = lengthNum;
			}
		}

		if (urlParams.alphabet && isValidPasswordAlphabet(String(urlParams.alphabet))) {
			params.alphabet = String(urlParams.alphabet) as 'full-with-symbols' | 'no-look-alike';
		}

		if (urlParams.seed) {
			urlProvidedSeed = String(urlParams.seed);
		}
	});
</script>

<svelte:head>
	<title>{$_('password.title')}</title>
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
					<span class="text-xl text-white">🔐</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
					{$_('password.title')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300">
					{$_('password.description')}
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
					<!-- Alphabet -->
					<AlphabetSelector
						bind:value={params.alphabet}
						options={alphabetOptions}
						label={$_('password.alphabet')}
						id="alphabet"
						onChange={handleAlphabetChange}
					/>

					<!-- Length -->
					<div>
						<label
							for="length"
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
						>
							{$_('password.length')} ({minLength}-44 {$_('common.characters')})
						</label>
						<div class="flex items-center gap-4">
							<input
								type="range"
								id="length"
								bind:value={params.length}
								min={minLength}
								max="44"
								class="flex-1 h-2 bg-blue-600 rounded appearance-none outline-none slider"
							/>
							<span
								class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center"
								>{params.length}</span
							>
						</div>
						{#if !lengthValid}
							<p class="text-red-500 text-sm mt-1">
								{$_('common.length')}
								{$_('common.mustBeBetween')}
								{minLength}
								{$_('common.and')} 44
							</p>
						{/if}
						<div
							class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3 mt-3"
						>
							<p class="text-sm text-blue-800 dark:text-blue-200">
								<strong>{$_('password.securityNote')}</strong>
								{#if params.alphabet === 'no-look-alike'}
									{$_('password.noLookAlikeNote').replace('{0}', minLength.toString())}
								{:else}
									{$_('password.fullAlphabetNote').replace('{0}', minLength.toString())}
								{/if}
							</p>
						</div>
					</div>

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

					<!-- Security Notice -->
					<div
						class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4"
					>
						<div class="flex items-start">
							<span class="text-blue-600 dark:text-blue-400 mr-2">🛡️</span>
							<div class="text-sm text-blue-800 dark:text-blue-200">
								<strong>{$_('password.securityNote')}</strong>
								{$_('password.securityDescription')}
							</div>
						</div>
					</div>

					<!-- Action Buttons -->
					<div class="flex flex-col sm:flex-row gap-4 mt-4">
						<!-- Generate password button -->
						<GenerateButton
							type="submit"
							disabled={!formValid || $isLoading || $authStore.isRefreshing}
							loading={$isLoading || $authStore.isRefreshing}
							text={$_('password.generatePassword')}
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
