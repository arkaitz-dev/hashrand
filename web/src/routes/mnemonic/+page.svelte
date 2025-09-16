<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import Footer from '$lib/components/Footer.svelte';
	import GenerateButton from '$lib/components/GenerateButton.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { dialogStore } from '$lib/stores/dialog';
	import { isLoading, resultState } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import { authStore } from '$lib/stores/auth';
	import type { MnemonicParams } from '$lib/types';
	import { decryptPageParams, createEncryptedUrl } from '$lib/crypto';

	// Default values
	function getDefaultParams(): MnemonicParams {
		return {
			language: 'english',
			words: 12,
			raw: true
		};
	}

	// Form state - will be initialized in onMount
	let params: MnemonicParams = $state(getDefaultParams());
	let urlProvidedSeed: string = $state(''); // Seed from URL parameters (read-only)

	// Get URL parameters reactively
	let searchParams = $derived($page.url.searchParams);

	// Function to validate language parameter
	function isValidMnemonicLanguage(value: string): boolean {
		const validLanguages = [
			'english',
			'spanish',
			'french',
			'portuguese',
			'japanese',
			'chinese',
			'chinese-traditional',
			'italian',
			'korean',
			'czech'
		];
		return validLanguages.includes(value);
	}

	// Function to validate words parameter
	function isValidMnemonicWords(value: number): value is 12 | 24 {
		return value === 12 || value === 24;
	}

	// Reactive language options that update when language changes
	let languageOptions = $derived([
		{
			value: 'english' as const,
			label: $_('mnemonic.languages.english')
		},
		{
			value: 'spanish' as const,
			label: $_('mnemonic.languages.spanish')
		},
		{
			value: 'french' as const,
			label: $_('mnemonic.languages.french')
		},
		{
			value: 'portuguese' as const,
			label: $_('mnemonic.languages.portuguese')
		},
		{
			value: 'japanese' as const,
			label: $_('mnemonic.languages.japanese')
		},
		{
			value: 'chinese' as const,
			label: $_('mnemonic.languages.chinese')
		},
		{
			value: 'chinese-traditional' as const,
			label: $_('mnemonic.languages.chineseTraditional')
		},
		{
			value: 'italian' as const,
			label: $_('mnemonic.languages.italian')
		},
		{
			value: 'korean' as const,
			label: $_('mnemonic.languages.korean')
		},
		{
			value: 'czech' as const,
			label: $_('mnemonic.languages.czech')
		}
	]);

	// Reactive word count options
	let wordOptions = $derived([
		{
			value: 12 as const,
			label: $_('mnemonic.words12'),
			description: $_('mnemonic.words12Description')
		},
		{
			value: 24 as const,
			label: $_('mnemonic.words24'),
			description: $_('mnemonic.words24Description')
		}
	]);

	// Validation
	let languageValid = $derived(params.language && isValidMnemonicLanguage(params.language));
	let wordsValid = $derived(params.words && isValidMnemonicWords(params.words));
	let formValid = $derived(languageValid && wordsValid);

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
				endpoint: 'mnemonic',
				language: params.language ?? 'english',
				words: params.words ?? 12,
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
			endpoint: 'mnemonic',
			language: params.language ?? 'english',
			words: params.words ?? 12
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

	// Initialize params based on navigation source
	onMount(() => {
		// Check if we're coming from result page with existing params
		if ($resultState && $resultState.endpoint === 'mnemonic' && $resultState.params) {
			// Coming from result page - use existing params
			params = { ...$resultState.params } as MnemonicParams;
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
		if (urlParams.language && isValidMnemonicLanguage(String(urlParams.language))) {
			params.language = String(urlParams.language) as
				| 'english'
				| 'japanese'
				| 'korean'
				| 'spanish'
				| 'chinese_simplified'
				| 'chinese_traditional'
				| 'french'
				| 'italian'
				| 'czech'
				| 'portuguese';
		}

		if (urlParams.words) {
			const wordsNum = parseInt(String(urlParams.words));
			if (isValidMnemonicWords(wordsNum)) {
				params.words = wordsNum as 12 | 24;
			}
		}

		if (urlParams.seed) {
			urlProvidedSeed = String(urlParams.seed);
		}
	});
</script>

<svelte:head>
	<title>{$_('mnemonic.title')}</title>
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
					<span class="text-xl text-white">💾</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
					{$_('mnemonic.title')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300">
					{$_('mnemonic.description')}
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
					<!-- Language -->
					<div>
						<label
							for="language"
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
						>
							{$_('mnemonic.language')}
						</label>
						<select
							id="language"
							bind:value={params.language}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
						>
							{#each languageOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
						<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
							{$_('mnemonic.languageDescription')}
						</p>
					</div>

					<!-- Word Count -->
					<div>
						<label
							for="words"
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
						>
							{$_('mnemonic.wordCount')}
						</label>
						<select
							id="words"
							bind:value={params.words}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
						>
							{#each wordOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
						{#if params.words}
							<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
								{wordOptions.find((o) => o.value === params.words)?.description}
							</p>
						{/if}
					</div>

					<!-- Format Notice -->
					<div
						class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4"
					>
						<div class="flex items-start">
							<span class="text-blue-600 dark:text-blue-400 mr-2">ℹ️</span>
							<div class="text-sm text-blue-800 dark:text-blue-200">
								<strong>{$_('common.format')}:</strong>
								{$_('mnemonic.formatNotice')}
							</div>
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
						class="bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg p-4"
					>
						<div class="flex items-start">
							<span class="text-amber-600 dark:text-amber-400 mr-2">⚠️</span>
							<div class="text-sm text-amber-800 dark:text-amber-200">
								<strong>{$_('common.security')}:</strong>
								{$_('mnemonic.securityNotice')}
							</div>
						</div>
					</div>

					<!-- Action Buttons -->
					<div class="flex flex-col sm:flex-row gap-4 mt-4">
						<!-- Generate mnemonic button -->
						<GenerateButton
							type="submit"
							disabled={!formValid || $isLoading || $authStore.isRefreshing}
							loading={$isLoading || $authStore.isRefreshing}
							text={$_('mnemonic.generateMnemonic')}
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
