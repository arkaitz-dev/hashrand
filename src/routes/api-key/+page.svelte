<script lang="ts">
	import { goto } from '$app/navigation';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
	import { setResult, setLoading, setError, isLoading } from '$lib/stores/result';
	import { t } from '$lib/stores/i18n';
	import type { ApiKeyParams } from '$lib/types';

	// Form state
	let params: ApiKeyParams = {
		length: 44,
		alphabet: 'full',
		raw: true
	};

	const alphabetOptions: { value: 'no-look-alike' | 'full'; label: string; description: string }[] = [
		{ value: 'full', label: t('alphabets.full'), description: 'Standard alphanumeric (62 chars)' },
		{ value: 'no-look-alike', label: t('alphabets.no-look-alike'), description: 'No confusing characters (49 chars)' }
	];

	// Dynamic minimum length based on alphabet
	$: minLength = params.alphabet === 'full' ? 44 : 47;
	$: lengthValid = params.length && params.length >= minLength && params.length <= 64;
	$: formValid = lengthValid;

	async function handleGenerate(event: Event) {
		event.preventDefault();
		if (!formValid || $isLoading) return;

		setLoading(true);
		
		try {
			const { api } = await import('$lib/api');
			const result = await api.generateApiKey(params);
			
			setResult({
				value: result,
				params: { ...params },
				endpoint: 'api-key',
				timestamp: new Date()
			});

			goto('/result');
		} catch (error) {
			setError(error instanceof Error ? error.message : 'Failed to generate API key');
		} finally {
			setLoading(false);
		}
	}

	// Update length when alphabet changes with smooth adjustment
	function handleAlphabetChange() {
		const newMinLength = params.alphabet === 'full' ? 44 : 47;
		if (params.length! < newMinLength) {
			params.length = newMinLength;
		}
	}

	// Reactive statement to adjust length automatically
	$: {
		if (params.length! < minLength) {
			params.length = minLength;
		}
	}
</script>

<svelte:head>
	<title>API Key Generator</title>
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-purple-50 to-violet-100 dark:from-gray-900 dark:to-gray-800">
	<div class="container mx-auto px-4 py-8">
		<!-- Header -->
		<div class="mb-8">
			<div class="text-center">
				<div class="inline-flex items-center justify-center w-12 h-12 bg-purple-600 rounded-full mb-4">
					<span class="text-xl text-white">🔑</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
					{t('apiKey.title')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300">
					{t('apiKey.description')}
				</p>
			</div>
		</div>

		<!-- Form -->
		<div class="max-w-2xl mx-auto">
			<div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6">
				<form onsubmit={handleGenerate} class="space-y-6">
					<!-- Alphabet -->
					<div>
						<label for="alphabet" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{t('apiKey.alphabet')}
						</label>
						<select
							id="alphabet"
							bind:value={params.alphabet}
							onchange={handleAlphabetChange}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 dark:bg-gray-700 dark:text-white"
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

					<!-- Length -->
					<div>
						<label for="length" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{t('apiKey.length')} ({minLength}-64 characters)
						</label>
						<div class="flex items-center gap-4">
							<input
								type="range"
								id="length"
								bind:value={params.length}
								min={minLength}
								max="64"
								class="flex-1 h-2 bg-purple-600 rounded appearance-none outline-none slider"
							/>
							<span class="bg-purple-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center">{params.length}</span>
						</div>
						{#if !lengthValid}
							<p class="text-red-500 text-sm mt-1">
								Length must be between {minLength} and 64 for selected alphabet
							</p>
						{/if}
						<div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3 mt-3">
							<p class="text-sm text-blue-800 dark:text-blue-200">
								<strong>Format:</strong> ak_ prefix + {params.length - 3} random characters using 
								{#if params.alphabet === 'no-look-alike'}
									no-look-alike alphabet (easy to type)
								{:else}
									full alphanumeric alphabet
								{/if}
								<br><strong>Security:</strong> 
								{#if params.alphabet === 'no-look-alike'}
									No Look-alike excludes confusable characters. Minimum {minLength} characters for equivalent security.
								{:else}
									Full alphanumeric provides maximum compatibility. Minimum {minLength} characters for strong security.
								{/if}
							</p>
						</div>
					</div>


					<!-- Format Notice -->
					<div class="bg-purple-50 dark:bg-purple-900/20 border border-purple-200 dark:border-purple-800 rounded-lg p-4">
						<div class="flex items-start">
							<span class="text-purple-600 dark:text-purple-400 mr-2">ℹ️</span>
							<div class="text-sm text-purple-800 dark:text-purple-200">
								<strong>Format:</strong> All API keys are generated with the "ak_" prefix for easy identification.
								The specified length includes this prefix.
							</div>
						</div>
					</div>

					<!-- Security Notice -->
					<div class="bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg p-4">
						<div class="flex items-start">
							<span class="text-amber-600 dark:text-amber-400 mr-2">⚠️</span>
							<div class="text-sm text-amber-800 dark:text-amber-200">
								<strong>Security:</strong> Store API keys securely and never expose them in client-side code or version control.
								Treat them with the same care as passwords.
							</div>
						</div>
					</div>

					<!-- Action Buttons -->
					<div class="flex flex-col sm:flex-row gap-4 mt-4">
						<button
							type="submit"
							disabled={!formValid || $isLoading}
							class="flex-1 py-4 bg-purple-600 hover:bg-purple-700 text-white border-none rounded-lg text-lg font-semibold cursor-pointer transition-all duration-200 hover:shadow-lg disabled:bg-gray-400 disabled:cursor-not-allowed disabled:shadow-none flex items-center justify-center"
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
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z" />
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h2a2 2 0 012 2v2H8V5z" />
							</svg>
							{t('common.backToMenu')}
						</button>
					</div>
				</form>
			</div>
		</div>
	</div>
</div>