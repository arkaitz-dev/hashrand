<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	// import Button from '$lib/components/Button.svelte';
	import GenerateButton from '$lib/components/GenerateButton.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import AlphabetSelector from '$lib/components/AlphabetSelector.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	// import { dialogStore } from '$lib/stores/dialog'; // REPLACED by useGenerationWorkflow
	import { isLoading } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import { authStore } from '$lib/stores/auth';
	import type { PasswordParams } from '$lib/types';
	// import { decryptPageParams, createEncryptedUrl } from '$lib/crypto'; // REPLACED by composables
	import { logger } from '$lib/utils/logger';

	// NEW: Enterprise-grade composables for SOLID/DRY architecture
	import { useGenerationWorkflow } from '$lib/composables/useGenerationWorkflow';
	import { useFormParams } from '$lib/composables/useFormParams';

	// Track route loading
	onMount(() => {
		logger.info('[Route] Password page loaded');

		// 🔒 SECURITY: Only allow 'p' parameter (encrypted params from result page)
		// Any other parameter is a potential attack vector - redirect to home
		const searchParams = $page.url.searchParams;
		for (const [key] of searchParams) {
			if (key !== 'p') {
				logger.warn(`[Security] Unauthorized parameter '${key}' detected, redirecting to home`);
				goto('/');
				return;
			}
		}
	});

	// Default values
	function getDefaultParams(): PasswordParams {
		return {
			length: 21, // Minimum for full-with-symbols alphabet
			alphabet: 'full-with-symbols'
		};
	}

	// Password-specific parameter validation and application
	function validateAndApplyPasswordParams(
		urlParams: Record<string, unknown>,
		currentParams: PasswordParams
	): PasswordParams {
		let newParams = { ...currentParams };

		// Validate and apply length
		if (urlParams.length) {
			const lengthNum = parseInt(String(urlParams.length));
			if (!isNaN(lengthNum) && lengthNum >= 21 && lengthNum <= 44) {
				newParams.length = lengthNum;
			}
		}

		// Validate and apply alphabet
		if (urlParams.alphabet && isValidPasswordAlphabet(String(urlParams.alphabet))) {
			newParams.alphabet = String(urlParams.alphabet) as 'full-with-symbols' | 'no-look-alike';
		}

		return newParams;
	}

	// ENTERPRISE ARCHITECTURE: Using composables for SOLID/DRY principles
	const formParamsManager = useFormParams({
		endpoint: 'password',
		getDefaultParams,
		validateAndApplyParams: validateAndApplyPasswordParams
	});

	// Destructure stores for reactivity ($store syntax requires variables, not object properties)
	const { params: paramsStore, urlProvidedSeed: urlProvidedSeedStore } = formParamsManager;

	// Form state managed by composable (using $store syntax for reactivity)
	let urlProvidedSeed = $derived($urlProvidedSeedStore);

	// Reactive bindings for form inputs
	let alphabet = $state('full-with-symbols');
	let length = $state(21);

	// URL params → local state (one-way sync for form initialization)
	// Note: Generation reads from local variables via getParams() - no reverse sync needed
	$effect(() => {
		const currentParams = $paramsStore;
		if (currentParams.alphabet) alphabet = currentParams.alphabet;
		if (currentParams.length) length = currentParams.length;
	});

	// REMOVED: URL parameters now handled by useFormParams composable
	// let searchParams = $derived($page.url.searchParams);

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
	let minLength = $derived(alphabet === 'full-with-symbols' ? 21 : 24);
	let lengthValid = $derived(length && length >= minLength && length <= 44);
	let formValid = $derived(lengthValid);

	// ENTERPRISE ARCHITECTURE: Generation workflow composable
	const generationWorkflow = useGenerationWorkflow({
		endpoint: 'password',
		get formValid() {
			return Boolean(formValid);
		},
		getParams: () => ({
			length: length ?? 21,
			alphabet: alphabet ?? 'full-with-symbols'
		}),
		get urlProvidedSeed() {
			return urlProvidedSeed;
		}
	});

	// REPLACED: All generation logic now handled by useGenerationWorkflow composable
	// This eliminates ~100 lines of duplicated code per route
	/*
	let pendingGenerationParams: Record<string, unknown> | null = null;

	async function handleGenerate(event: Event) {
		// ... 100+ lines of duplicated generation logic ...
	}

	async function proceedWithGeneration() {
		// ... duplicated logic ...
	}
	*/

	// Update length when alphabet changes with smooth adjustment
	function handleAlphabetChange() {
		// Use setTimeout to avoid reactivity issues
		setTimeout(() => {
			const newMinLength = alphabet === 'full-with-symbols' ? 21 : 24;
			if (length < newMinLength) {
				// Update local state directly
				length = newMinLength;
			}
		}, 0);
	}

	// REPLACED: All parameter initialization now handled by useFormParams composable
	// This eliminates ~50 lines of duplicated URL parameter handling per route
	/*
	onMount(() => {
		// ... 50+ lines of duplicated parameter initialization logic ...
	});
	*/
</script>

<svelte:head>
	<title>{$_('password.title')}</title>
</svelte:head>

<div
	class="flex-1 bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"
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
				<form onsubmit={generationWorkflow.handleGenerate} class="space-y-6">
					<!-- Alphabet -->
					<AlphabetSelector
						bind:value={alphabet}
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
								bind:value={length}
								min={minLength}
								max="44"
								class="flex-1 h-2 bg-blue-600 rounded appearance-none outline-none slider"
							/>
							<span
								class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center"
								>{length}</span
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
								{#if alphabet === 'no-look-alike'}
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
	</div>
</div>
