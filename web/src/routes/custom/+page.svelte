<script lang="ts">
	// import { goto } from '$app/navigation'; // REPLACED by useGenerationWorkflow
	// import { onMount } from 'svelte'; // REPLACED by useFormParams
	// import { page } from '$app/stores'; // REPLACED by useFormParams
	// import Button from '$lib/components/Button.svelte';
	import GenerateButton from '$lib/components/GenerateButton.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import AlphabetSelector from '$lib/components/AlphabetSelector.svelte';
	import TextInput from '$lib/components/TextInput.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	// import { dialogStore } from '$lib/stores/dialog'; // REPLACED by useGenerationWorkflow
	import { isLoading } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import { authStore } from '$lib/stores/auth';
	import type { GenerateParams, AlphabetType } from '$lib/types';
	// import { decryptPageParams, createEncryptedUrl } from '$lib/crypto'; // REPLACED by composables

	// NEW: Enterprise-grade composables for SOLID/DRY architecture
	import { useGenerationWorkflow } from '$lib/composables/useGenerationWorkflow';
	import { useFormParams } from '$lib/composables/useFormParams';

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

	// Custom-specific parameter validation and application
	function validateAndApplyCustomParams(
		urlParams: Record<string, unknown>,
		currentParams: GenerateParams
	): GenerateParams {
		let newParams = { ...currentParams };

		// Validate and apply length
		if (urlParams.length) {
			const lengthNum = parseInt(String(urlParams.length));
			if (!isNaN(lengthNum) && lengthNum >= 2 && lengthNum <= 128) {
				newParams.length = lengthNum;
			}
		}

		// Validate and apply alphabet
		if (urlParams.alphabet && isValidAlphabet(String(urlParams.alphabet))) {
			newParams.alphabet = String(urlParams.alphabet) as AlphabetType;
		}

		// Validate and apply prefix
		if (urlParams.prefix !== undefined && String(urlParams.prefix).length <= 32) {
			newParams.prefix = String(urlParams.prefix);
		}

		// Validate and apply suffix
		if (urlParams.suffix !== undefined && String(urlParams.suffix).length <= 32) {
			newParams.suffix = String(urlParams.suffix);
		}

		return newParams;
	}

	// ENTERPRISE ARCHITECTURE: Using composables for SOLID/DRY principles
	const formParamsManager = useFormParams({
		endpoint: 'custom',
		getDefaultParams,
		validateAndApplyParams: validateAndApplyCustomParams
	});

	// Form state managed by composable
	let urlProvidedSeed = $derived(formParamsManager.urlProvidedSeed.value);

	// Reactive bindings for form inputs
	let alphabet = $state('base58');
	let length = $state(21);
	let prefix = $state('');
	let suffix = $state('');

	// Bidirectional sync: URL params → local state
	$effect(() => {
		const currentParams = formParamsManager.params.value;
		if (currentParams.alphabet) alphabet = currentParams.alphabet;
		if (currentParams.length) length = currentParams.length;
		if (currentParams.prefix) prefix = currentParams.prefix;
		if (currentParams.suffix) suffix = currentParams.suffix;
	});

	// Bidirectional sync: local state → params (for generation)
	$effect(() => {
		formParamsManager.params.value = {
			...formParamsManager.params.value,
			alphabet: alphabet as AlphabetType,
			length,
			prefix,
			suffix
		};
	});

	// REMOVED: URL parameters now handled by useFormParams composable
	// let searchParams = $derived($page.url.searchParams);

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
	let lengthValid = $derived(length && length >= 2 && length <= 128);
	let prefixValid = $derived(!prefix || prefix.length <= 32);
	let suffixValid = $derived(!suffix || suffix.length <= 32);
	let formValid = $derived(lengthValid && prefixValid && suffixValid);

	// ENTERPRISE ARCHITECTURE: Generation workflow composable
	const generationWorkflow = useGenerationWorkflow({
		endpoint: 'custom',
		get formValid() {
			return Boolean(formValid);
		},
		getParams: () => ({
			length: length ?? 21,
			alphabet: alphabet ?? 'base58',
			...(prefix && { prefix: prefix }),
			...(suffix && { suffix: suffix })
		}),
		get urlProvidedSeed() {
			return urlProvidedSeed;
		}
	});

	// REPLACED: All generation logic now handled by useGenerationWorkflow composable
	// This eliminates ~150 lines of duplicated code per route
	/*
	let pendingGenerationParams: Record<string, unknown> | null = null;

	async function handleGenerate(event: Event) {
		// ... 150+ lines of duplicated generation logic ...
	}

	async function performGeneration() {
		// ... duplicated logic ...
	}

	async function proceedWithGeneration() {
		// ... duplicated logic ...
	}

	function handleAuthenticated() {
		// ... duplicated logic ...
	}
	*/

	// REPLACED: All parameter initialization now handled by useFormParams composable
	// This eliminates ~60 lines of duplicated URL parameter handling per route
	/*
	onMount(() => {
		// ... 60+ lines of duplicated parameter initialization logic ...
	});
	*/
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
				<form onsubmit={generationWorkflow.handleGenerate} class="space-y-6">
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
								bind:value={length}
								min="2"
								max="128"
								class="flex-1 h-2 bg-blue-600 rounded appearance-none outline-none slider"
							/>
							<span
								class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center"
								>{length}</span
							>
						</div>
						{#if !lengthValid}
							<p class="text-red-500 text-sm mt-1">{$_('custom.lengthMustBeBetween')}</p>
						{/if}
					</div>

					<!-- Alphabet -->
					<AlphabetSelector
						bind:value={alphabet}
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
						bind:value={prefix}
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
						bind:value={suffix}
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
	</div>
</div>

<!-- Authentication handled by global DialogContainer -->
