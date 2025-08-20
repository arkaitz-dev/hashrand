<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import { setResult, setLoading, setError, isLoading, resultState } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import type { PasswordParams } from '$lib/types';

	// Default values
	function getDefaultParams(): PasswordParams {
		return {
			length: 21, // Minimum for full-with-symbols alphabet
			alphabet: 'full-with-symbols',
			raw: true
		};
	}

	// Form state - will be initialized in onMount
	let params: PasswordParams = getDefaultParams();

	const alphabetOptions: { value: 'no-look-alike' | 'full-with-symbols'; label: string; description: string }[] = [
		{ value: 'full-with-symbols', label: $_('alphabets.full-with-symbols'), description: $_('password.maxSecurityDescription') },
		{ value: 'no-look-alike', label: $_('alphabets.no-look-alike'), description: $_('password.easyReadDescription') }
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
			setError(error instanceof Error ? error.message : $_('password.failedToGeneratePassword'));
		} finally {
			setLoading(false);
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
	});
</script>

<svelte:head>
	<title>{$_('password.title')}</title>
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
	<div class="container mx-auto px-4 py-8">
		<!-- Header -->
		<div class="mb-8">
			<div class="text-center">
				<div class="inline-flex items-center justify-center w-12 h-12 bg-blue-600 rounded-full mb-4">
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

		<!-- Form -->
		<div class="max-w-2xl mx-auto">
			<div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6">
				<form onsubmit={handleGenerate} class="space-y-6">
					<!-- Alphabet -->
					<div>
						<label for="alphabet" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{$_('password.alphabet')}
						</label>
						<select
							id="alphabet"
							bind:value={params.alphabet}
							onchange={handleAlphabetChange}
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

					<!-- Length -->
					<div>
						<label for="length" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
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
							<span class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center">{params.length}</span>
						</div>
						{#if !lengthValid}
							<p class="text-red-500 text-sm mt-1">
								{$_('common.length')} {$_('common.mustBeBetween')} {minLength} {$_('common.and')} 44
							</p>
						{/if}
						<div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3 mt-3">
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


					<!-- Security Notice -->
					<div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
						<div class="flex items-start">
							<span class="text-blue-600 dark:text-blue-400 mr-2">🛡️</span>
							<div class="text-sm text-blue-800 dark:text-blue-200">
								<strong>{$_('password.securityNote')}</strong> {$_('password.securityDescription')}
							</div>
						</div>
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
								{$_('common.loading')}...
							{:else}
								{$_('password.generatePassword')}
							{/if}
						</button>
						<button
							type="button"
							onclick={() => goto('/')}
							class="px-6 py-4 bg-gray-600 hover:bg-gray-700 text-white font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-2"
						>
							<Icon name="briefcase" size="w-4 h-4" />
							{$_('common.backToMenu')}
						</button>
					</div>
				</form>
			</div>
		</div>
		
		<!-- Made with love -->
		<div class="text-center mt-8">
			<div class="text-xs text-gray-400 dark:text-gray-500 flex items-center justify-center">
				<span>Made with</span>
				<Icon name="heart" size="w-3 h-3 mx-1 text-red-500" />
				<span>by</span>
				<a href="https://arkaitz.dev" target="_blank" rel="noopener noreferrer" class="ml-1 text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300 hover:underline">Arkaitz Dev</a>
			</div>
		</div>
	</div>
</div>