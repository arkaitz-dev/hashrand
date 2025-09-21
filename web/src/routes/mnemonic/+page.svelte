<script lang="ts">
	// import { goto } from '$app/navigation'; // REPLACED by useGenerationWorkflow
	// import { onMount } from 'svelte'; // REPLACED by useFormParams
	// import { page } from '$app/stores'; // REPLACED by useFormParams
	import Footer from '$lib/components/Footer.svelte';
	import GenerateButton from '$lib/components/GenerateButton.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	// import { dialogStore } from '$lib/stores/dialog'; // REPLACED by useGenerationWorkflow
	import { isLoading } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import { authStore } from '$lib/stores/auth';
	import type { MnemonicParams } from '$lib/types';
	// import { decryptPageParams, createEncryptedUrl } from '$lib/crypto'; // REPLACED by composables

	// NEW: Enterprise-grade composables for SOLID/DRY architecture
	import { useGenerationWorkflow } from '$lib/composables/useGenerationWorkflow';
	import { useFormParams } from '$lib/composables/useFormParams';

	// Default values
	function getDefaultParams(): MnemonicParams {
		return {
			language: 'english',
			words: 12,
			raw: true
		};
	}

	// Mnemonic-specific parameter validation and application
	function validateAndApplyMnemonicParams(
		urlParams: Record<string, unknown>,
		currentParams: MnemonicParams
	): MnemonicParams {
		let newParams = { ...currentParams };

		// Validate and apply language
		if (urlParams.language && isValidMnemonicLanguage(String(urlParams.language))) {
			newParams.language = String(urlParams.language) as
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

		// Validate and apply words
		if (urlParams.words) {
			const wordsNum = parseInt(String(urlParams.words));
			if (isValidMnemonicWords(wordsNum)) {
				newParams.words = wordsNum as 12 | 24;
			}
		}

		return newParams;
	}

	// ENTERPRISE ARCHITECTURE: Using composables for SOLID/DRY principles
	const formParamsManager = useFormParams({
		endpoint: 'mnemonic',
		getDefaultParams,
		validateAndApplyParams: validateAndApplyMnemonicParams
	});

	// Form state managed by composable
	let params = $derived(formParamsManager.params.value);
	let urlProvidedSeed = $derived(formParamsManager.urlProvidedSeed.value);

	// REMOVED: URL parameters now handled by useFormParams composable
	// let searchParams = $derived($page.url.searchParams);

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

	// ENTERPRISE ARCHITECTURE: Generation workflow composable
	const generationWorkflow = useGenerationWorkflow({
		endpoint: 'mnemonic',
		get formValid() {
			return Boolean(formValid);
		},
		getParams: () => ({
			language: params.language ?? 'english',
			words: params.words ?? 12
		}),
		get urlProvidedSeed() {
			return urlProvidedSeed;
		}
	});

	// REPLACED: All generation logic now handled by useGenerationWorkflow composable
	// This eliminates ~90 lines of duplicated code per route
	/*
	let pendingGenerationParams: Record<string, unknown> | null = null;

	async function handleGenerate(event: Event) {
		// ... 90+ lines of duplicated generation logic ...
	}

	async function proceedWithGeneration() {
		// ... duplicated logic ...
	}
	*/

	// REPLACED: All parameter initialization now handled by useFormParams composable
	// This eliminates ~60 lines of duplicated URL parameter handling per route
	/*
	onMount(async () => {
		// ... 60+ lines of duplicated parameter initialization logic ...
	});
	*/
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
				<form onsubmit={generationWorkflow.handleGenerate} class="space-y-6">
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
							bind:value={formParamsManager.params.value.language}
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
							bind:value={formParamsManager.params.value.words}
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
