<script lang="ts">
	import { goto } from '$app/navigation';
	import BackButton from '$lib/components/BackButton.svelte';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
	import { setResult, setLoading, setError, isLoading } from '$lib/stores/result';
	import { t } from '$lib/stores/i18n';
	import type { PasswordParams } from '$lib/types';

	// Form state
	let params: PasswordParams = {
		length: 32,
		alphabet: 'full-with-symbols',
		raw: false
	};

	const alphabetOptions: { value: 'no-look-alike' | 'full-with-symbols'; label: string; description: string }[] = [
		{ value: 'full-with-symbols', label: t('alphabets.full-with-symbols'), description: 'Maximum security with symbols (73 chars)' },
		{ value: 'no-look-alike', label: t('alphabets.no-look-alike'), description: 'Easy to read and type (49 chars)' }
	];

	// Dynamic minimum length based on alphabet
	$: minLength = params.alphabet === 'full-with-symbols' ? 21 : 24;
	$: lengthValid = params.length && params.length >= minLength && params.length <= 44;
	$: formValid = lengthValid;

	async function handleGenerate(event: Event) {
		event.preventDefault();
		if (!formValid || $isLoading) return;

		setLoading(true);
		
		try {
			const { api } = await import('$lib/api');
			const result = await api.generatePassword(params);
			
			setResult({
				value: result,
				params: { ...params },
				endpoint: 'password',
				timestamp: new Date()
			});

			goto('/result');
		} catch (error) {
			setError(error instanceof Error ? error.message : 'Failed to generate password');
		} finally {
			setLoading(false);
		}
	}

	// Update length when alphabet changes
	function handleAlphabetChange() {
		if (params.length! < minLength) {
			params.length = minLength;
		}
	}
</script>

<svelte:head>
	<title>Secure Password Generator</title>
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-green-50 to-emerald-100 dark:from-gray-900 dark:to-gray-800">
	<div class="container mx-auto px-4 py-8">
		<!-- Header -->
		<div class="mb-8">
			<BackButton to="/" class="mb-6" />
			<div class="text-center">
				<div class="inline-flex items-center justify-center w-12 h-12 bg-green-600 rounded-full mb-4">
					<span class="text-xl text-white">🔐</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
					{t('password.title')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300">
					{t('password.description')}
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
							{t('password.alphabet')}
						</label>
						<select
							id="alphabet"
							bind:value={params.alphabet}
							onchange={handleAlphabetChange}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
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
							{t('password.length')} ({minLength}-44)
						</label>
						<input
							type="number"
							id="length"
							bind:value={params.length}
							min={minLength}
							max="44"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 dark:bg-gray-700 dark:text-white"
							class:border-red-500={!lengthValid}
						/>
						{#if !lengthValid}
							<p class="text-red-500 text-sm mt-1">
								Length must be between {minLength} and 44 for selected alphabet
							</p>
						{/if}
						<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
							Minimum length ensures adequate entropy for security
						</p>
					</div>

					<!-- Raw output -->
					<div class="flex items-center">
						<input
							type="checkbox"
							id="raw"
							bind:checked={params.raw}
							class="h-4 w-4 text-green-600 focus:ring-green-500 border-gray-300 rounded"
						/>
						<label for="raw" class="ml-2 block text-sm text-gray-700 dark:text-gray-300">
							{t('password.raw')} (no trailing newline)
						</label>
					</div>

					<!-- Security Notice -->
					<div class="bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg p-4">
						<div class="flex items-start">
							<span class="text-green-600 dark:text-green-400 mr-2">🛡️</span>
							<div class="text-sm text-green-800 dark:text-green-200">
								<strong>Security Note:</strong> Passwords are generated using cryptographically secure random generation.
								They are not stored or logged anywhere.
							</div>
						</div>
					</div>

					<!-- Generate Button -->
					<button
						type="submit"
						disabled={!formValid || $isLoading}
						class="w-full py-3 px-4 bg-green-600 hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-medium rounded-lg transition-colors duration-200 flex items-center justify-center"
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