<script lang="ts">
	import { goto } from '$app/navigation';
	import BackButton from '$lib/components/BackButton.svelte';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
	import { setResult, setLoading, setError, isLoading } from '$lib/stores/result';
	import { t } from '$lib/stores/i18n';
	import type { GenerateParams, AlphabetType } from '$lib/types';

	// Form state
	let params: GenerateParams = {
		length: 21,
		alphabet: 'base58',
		prefix: '',
		suffix: '',
		raw: false
	};

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
				endpoint: 'generate',
				timestamp: new Date()
			});

			goto('/result');
		} catch (error) {
			setError(error instanceof Error ? error.message : 'Failed to generate hash');
		} finally {
			setLoading(false);
		}
	}
</script>

<svelte:head>
	<title>Custom Hash Generator</title>
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
	<div class="container mx-auto px-4 py-8">
		<!-- Header -->
		<div class="mb-8">
			<BackButton to="/" class="mb-6" />
			<div class="text-center">
				<div class="inline-flex items-center justify-center w-12 h-12 bg-blue-600 rounded-full mb-4">
					<span class="text-xl text-white">🎲</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
					{t('generate.title')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300">
					{t('generate.description')}
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
							{t('generate.length')} (2-128)
						</label>
						<input
							type="number"
							id="length"
							bind:value={params.length}
							min="2"
							max="128"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
							class:border-red-500={!lengthValid}
						/>
						{#if !lengthValid}
							<p class="text-red-500 text-sm mt-1">Length must be between 2 and 128</p>
						{/if}
					</div>

					<!-- Alphabet -->
					<div>
						<label for="alphabet" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{t('generate.alphabet')}
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
							{t('generate.prefix')} (max 32 chars)
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
							{t('generate.suffix')} (max 32 chars)
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

					<!-- Raw output -->
					<div class="flex items-center">
						<input
							type="checkbox"
							id="raw"
							bind:checked={params.raw}
							class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
						/>
						<label for="raw" class="ml-2 block text-sm text-gray-700 dark:text-gray-300">
							{t('generate.raw')} (no trailing newline)
						</label>
					</div>

					<!-- Generate Button -->
					<button
						type="submit"
						disabled={!formValid || $isLoading}
						class="w-full py-3 px-4 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-medium rounded-lg transition-colors duration-200 flex items-center justify-center"
					>
						{#if $isLoading}
							<LoadingSpinner size="sm" class="mr-2" />
							{t('common.loading')}
						{:else}
							{t('common.generate')}
						{/if}
					</button>
				</form>
			</div>
		</div>
	</div>
</div>