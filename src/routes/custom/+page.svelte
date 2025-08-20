<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import { setResult, setLoading, setError, isLoading, resultState } from '$lib/stores/result';
	import { t } from '$lib/stores/i18n';
	import type { GenerateParams, AlphabetType } from '$lib/types';

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
	let params: GenerateParams = getDefaultParams();

	const alphabetOptions: { value: AlphabetType; label: string; description: string }[] = [
		{ value: 'base58', label: t('alphabets.base58'), description: 'Bitcoin alphabet, excludes confusing characters' },
		{ value: 'no-look-alike', label: t('alphabets.no-look-alike'), description: 'Maximum readability, 49 characters' },
		{ value: 'full', label: t('alphabets.full'), description: 'Complete alphanumeric set' },
		{ value: 'full-with-symbols', label: t('alphabets.full-with-symbols'), description: 'Maximum entropy with symbols' }
	];

	// Validation
	$: lengthValid = params.length && params.length >= 2 && params.length <= 128;
	$: prefixValid = !params.prefix || params.prefix.length <= 32;
	$: suffixValid = !params.suffix || params.suffix.length <= 32;
	$: formValid = lengthValid && prefixValid && suffixValid;

	async function handleGenerate(event: Event) {
		event.preventDefault();
		if (!formValid || $isLoading) return;

		setLoading(true);
		
		try {
			const { api } = await import('$lib/api');
			const result = await api.generate(params);
			
			setResult({
				value: result,
				params: { ...params },
				endpoint: 'custom',
				timestamp: new Date()
			});

			goto('/result');
		} catch (error) {
			setError(error instanceof Error ? error.message : 'Failed to generate hash');
		} finally {
			setLoading(false);
		}
	}

	// Initialize params based on navigation source
	onMount(() => {
		// Check if we're coming from result page with existing params
		if ($resultState && ($resultState.endpoint === 'custom' || $resultState.endpoint === 'generate') && $resultState.params) {
			// Coming from result page - use existing params
			params = { ...$resultState.params } as GenerateParams;
		} else {
			// Coming from menu or fresh load - use defaults
			params = getDefaultParams();
		}
	});
</script>

<svelte:head>
	<title>Custom Hash Generator</title>
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
	<div class="container mx-auto px-4 py-8">
		<!-- Header -->
		<div class="mb-8">
			<div class="text-center">
				<div class="inline-flex items-center justify-center w-12 h-12 bg-blue-600 rounded-full mb-4">
					<span class="text-xl text-white">🎲</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
					{t('custom.title')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300">
					{t('custom.description')}
				</p>
			</div>
		</div>

		<!-- Form -->
		<div class="max-w-2xl mx-auto">
			<div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6">
				<form onsubmit={handleGenerate} class="space-y-6">
					<!-- Length -->
					<div>
						<label for="length" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{t('custom.length')} (2-128)
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
							<span class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center">{params.length}</span>
						</div>
						{#if !lengthValid}
							<p class="text-red-500 text-sm mt-1">Length must be between 2 and 128</p>
						{/if}
					</div>

					<!-- Alphabet -->
					<div>
						<label for="alphabet" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{t('custom.alphabet')}
						</label>
						<select
							id="alphabet"
							bind:value={params.alphabet}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
						>
							{#each alphabetOptions as option}
								<option value={option.value}>{option.label}</option>
							{/each}
						</select>
						{#if params.alphabet}
							<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
								{alphabetOptions.find(o => o.value === params.alphabet)?.description}
							</p>
						{/if}
					</div>

					<!-- Prefix -->
					<div>
						<label for="prefix" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{t('custom.prefix')} (max 32 chars)
						</label>
						<input
							type="text"
							id="prefix"
							bind:value={params.prefix}
							maxlength="32"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
							class:border-red-500={!prefixValid}
							placeholder="Optional prefix"
						/>
						{#if !prefixValid}
							<p class="text-red-500 text-sm mt-1">Prefix cannot exceed 32 characters</p>
						{/if}
					</div>

					<!-- Suffix -->
					<div>
						<label for="suffix" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{t('custom.suffix')} (max 32 chars)
						</label>
						<input
							type="text"
							id="suffix"
							bind:value={params.suffix}
							maxlength="32"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
							class:border-red-500={!suffixValid}
							placeholder="Optional suffix"
						/>
						{#if !suffixValid}
							<p class="text-red-500 text-sm mt-1">Suffix cannot exceed 32 characters</p>
						{/if}
					</div>


					<!-- Action Buttons -->
					<div class="flex flex-col sm:flex-row gap-4 mt-4">
						<button
							type="submit"
							disabled={!formValid || $isLoading}
							class="flex-1 py-4 bg-blue-600 hover:bg-blue-700 text-white border-none rounded-lg text-lg font-semibold cursor-pointer transition-all duration-200 hover:shadow-lg disabled:bg-gray-400 disabled:cursor-not-allowed disabled:shadow-none flex items-center justify-center"
						>
							{#if $isLoading}
								<LoadingSpinner size="sm" class="mr-2" />
								{t('common.loading')}
							{:else}
								{t('common.generate')}
							{/if}
						</button>
						<button
							type="button"
							onclick={() => goto('/')}
							class="px-6 py-4 bg-gray-600 hover:bg-gray-700 text-white font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-2"
						>
							<Icon name="briefcase" size="w-4 h-4" />
							{t('common.backToMenu')}
						</button>
					</div>
				</form>
			</div>
		</div>
	</div>
</div>