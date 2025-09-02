<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import Footer from '$lib/components/Footer.svelte';
	import GenerateButton from '$lib/components/GenerateButton.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import AuthGuard from '$lib/components/AuthGuard.svelte';
	// import EmailInputDialog from '$lib/components/EmailInputDialog.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { dialogStore } from '$lib/stores/dialog';
	import { isLoading, resultState } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import type { MnemonicParams } from '$lib/types';

	// Default values
	function getDefaultParams(): MnemonicParams {
		return {
			language: 'english',
			words: 12,
			raw: true
		};
	}

	// Form state - will be initialized in onMount
	let params: MnemonicParams = getDefaultParams();
	let urlProvidedSeed: string = ''; // Seed from URL parameters (read-only)

	// Get URL parameters reactively
	$: searchParams = $page.url.searchParams;

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
	$: languageOptions = [
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
	];

	// Reactive word count options
	$: wordOptions = [
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
	];

	// Validation
	$: languageValid = params.language && isValidMnemonicLanguage(params.language);
	$: wordsValid = params.words && isValidMnemonicWords(params.words);
	$: formValid = languageValid && wordsValid;

	// Reference to AuthGuard component
	let authGuard: AuthGuard;
	let pendingGenerationParams: Record<string, unknown> | null = null;

	// // Email dialog state
	// let showEmailDialog = false;
	// let emailDialogRef: EmailInputDialog;

	// // Create next parameter object with current form state
	// $: nextObject = {
	// 	endpoint: 'mnemonic',
	// 	language: params.language,
	// 	words: params.words,
	// 	...(urlProvidedSeed && { seed: urlProvidedSeed })
	// };

	async function handleGenerate(event: Event) {
		event.preventDefault();
		if (!formValid) {
			return;
		}

		// Verificar si el usuario está autenticado
		const hasToken = typeof window !== 'undefined' && localStorage.getItem('access_token');
		const hasUser = typeof window !== 'undefined' && localStorage.getItem('auth_user');

		if (!hasToken || !hasUser) {
			// No autenticado - mostrar diálogo de autenticación
			pendingGenerationParams = {
				endpoint: 'mnemonic',
				language: params.language ?? 'english',
				words: params.words ?? 12,
				...(urlProvidedSeed && { seed: urlProvidedSeed })
			};
			dialogStore.show('auth', pendingGenerationParams);
			return;
		}

		// Usuario autenticado - proceder con la generación
		proceedWithGeneration();
	}

	function proceedWithGeneration() {
		// Create URL parameters for result page - result will handle API call
		const urlParams = new URLSearchParams();
		urlParams.set('endpoint', 'mnemonic');

		// Add generation parameters
		urlParams.set('language', params.language ?? 'english');
		urlParams.set('words', (params.words ?? 12).toString());

		// Add seed if provided from URL
		if (urlProvidedSeed) {
			urlParams.set('seed', urlProvidedSeed);
		}

		goto(`/result?${urlParams.toString()}`);
	}

	// // Email dialog handlers
	// function handleEmailDialogClose() {
	// 	showEmailDialog = false;
	// }

	// function handleEmailSubmitted(event: globalThis.CustomEvent<{ email: string }>) {
	// 	// Email entered and moving to confirmation step
	// }

	// async function handleEmailConfirmed(
	// 	event: globalThis.CustomEvent<{ email: string; redirectUrl: string }>
	// ) {
	// 	const { email, redirectUrl } = event.detail;

	// 	try {
	// 		// Obtener el host actual donde se ejecuta la UI
	// 		const currentHost = window.location.origin;

	// 		const requestBody = {
	// 			email: email,
	// 			ui_host: currentHost
	// 		};

	// 		const response = await fetch('/api/login/', {
	// 			method: 'POST',
	// 			headers: {
	// 				'Content-Type': 'application/json'
	// 			},
	// 			body: JSON.stringify(requestBody)
	// 		});

	// 		if (response.ok) {
	// 			// Magic link enviado correctamente, redirigir con el parámetro next
	// 			emailDialogRef?.resetSubmitting();
	// 			goto(redirectUrl);
	// 		} else {
	// 			// En caso de error, mostrar mensaje de error
	// 			emailDialogRef?.setError($_('common.sendError'));
	// 		}
	// 	} catch (error) {
	// 		console.error('Error sending magic link:', error);
	// 		emailDialogRef?.setError($_('common.connectionError'));
	// 	}
	// }

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
		const urlLanguage = searchParams.get('language');
		const urlWords = searchParams.get('words');
		const urlSeed = searchParams.get('seed');

		if (urlLanguage && isValidMnemonicLanguage(urlLanguage)) {
			params.language = urlLanguage;
		}

		if (urlWords) {
			const wordsNum = parseInt(urlWords);
			if (isValidMnemonicWords(wordsNum)) {
				params.words = wordsNum;
			}
		}

		if (urlSeed) {
			urlProvidedSeed = urlSeed;
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
		<AuthGuard bind:this={authGuard}>
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
								disabled={!formValid || $isLoading}
								loading={$isLoading}
								text={$_('mnemonic.generateMnemonic')}
							/>

							<!-- Back to menu button -->
							<BackToMenuButton />
						</div>
					</form>
				</div>
			</div>

			<!-- Footer with Version Information -->
			<Footer />
		</AuthGuard>
	</div>
</div>

<!-- Email Input Dialog - Removed, now redirecting to /login page -->
<!-- <EmailInputDialog
	bind:this={emailDialogRef}
	bind:show={showEmailDialog}
	next={nextObject}
	title={$_('auth.loginRequired')}
	description={$_('auth.loginDescription')}
	emailPlaceholder={$_('auth.emailPlaceholder')}
	confirmTitle={$_('auth.confirmEmail')}
	confirmDescription={$_('auth.confirmEmailDescription')}
	cancelText={$_('common.cancel')}
	continueText={$_('common.continue')}
	correctText={$_('common.correct')}
	sendText={$_('common.send')}
	sendingText={$_('common.sending')}
	on:close={handleEmailDialogClose}
	on:emailSubmitted={handleEmailSubmitted}
	on:emailConfirmed={handleEmailConfirmed}
/> -->
