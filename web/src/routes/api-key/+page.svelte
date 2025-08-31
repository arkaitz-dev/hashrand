<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import Footer from '$lib/components/Footer.svelte';
	// import Button from '$lib/components/Button.svelte';
	import GenerateButton from '$lib/components/GenerateButton.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import AlphabetSelector from '$lib/components/AlphabetSelector.svelte';
	import AuthGuard from '$lib/components/AuthGuard.svelte';
	// import EmailInputDialog from '$lib/components/EmailInputDialog.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { base58 } from '@scure/base';
	import { isLoading, resultState } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import type { ApiKeyParams } from '$lib/types';

	// Default values
	function getDefaultParams(): ApiKeyParams {
		return {
			length: 44, // Minimum for full alphabet
			alphabet: 'full',
			raw: true
		};
	}

	// Form state - will be initialized in onMount
	let params: ApiKeyParams = getDefaultParams();
	let urlProvidedSeed: string = ''; // Seed from URL parameters (read-only)

	// Get URL parameters reactively
	$: searchParams = $page.url.searchParams;

	// Function to validate alphabet parameter
	function isValidApiKeyAlphabet(value: string): value is 'full' | 'no-look-alike' {
		return ['full', 'no-look-alike'].includes(value);
	}

	// Reactive alphabet options that update when language changes
	$: alphabetOptions = [
		{
			value: 'full' as const,
			label: $_('alphabets.full'),
			description: $_('apiKey.standardAlphanumericDescription')
		},
		{
			value: 'no-look-alike' as const,
			label: $_('alphabets.no-look-alike'),
			description: $_('apiKey.noConfusingDescription')
		}
	];

	// Dynamic minimum length based on alphabet
	$: minLength = params.alphabet === 'full' ? 44 : 47;
	$: lengthValid = params.length && params.length >= minLength && params.length <= 64;
	$: formValid = lengthValid;

	// Reference to AuthGuard component
	let authGuard: AuthGuard;

	// // Email dialog state
	// let showEmailDialog = false;
	// let emailDialogRef: EmailInputDialog;

	// // Create next parameter object with current form state
	// $: nextObject = {
	// 	endpoint: 'api-key',
	// 	length: params.length,
	// 	alphabet: params.alphabet,
	// 	...(urlProvidedSeed && { seed: urlProvidedSeed })
	// };

	async function handleGenerate(event: Event) {
		event.preventDefault();
		if (!formValid) {
			console.log($_('common.formInvalid'));
			return;
		}

		// Verificar si el usuario está autenticado (verificación simple)
		const hasToken = typeof window !== 'undefined' && localStorage.getItem('access_token');
		const hasUser = typeof window !== 'undefined' && localStorage.getItem('auth_user');

		if (!hasToken || !hasUser) {
			// No autenticado - redirigir a página de login
			const urlParams = new URLSearchParams();
			urlParams.set('endpoint', 'api-key');
			urlParams.set('length', (params.length ?? 44).toString());
			urlParams.set('alphabet', params.alphabet ?? 'full');
			if (urlProvidedSeed) urlParams.set('seed', urlProvidedSeed);
			
			const jsonString = JSON.stringify(Object.fromEntries(urlParams));
			const bytes = new TextEncoder().encode(jsonString);
			const nextParam = base58.encode(bytes);
			goto(`/login?next=${encodeURIComponent(nextParam)}`);
			return;
		}

		// Usuario autenticado - proceder con la generación
		proceedWithGeneration();
	}

	function proceedWithGeneration() {
		// Create URL parameters for result page - result will handle API call
		const urlParams = new URLSearchParams();
		urlParams.set('endpoint', 'api-key');

		// Add generation parameters
		urlParams.set('length', (params.length ?? 44).toString());
		urlParams.set('alphabet', params.alphabet ?? 'full');

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
	// 	console.log('Email entered:', event.detail.email);
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

	// Update length when alphabet changes with smooth adjustment
	function handleAlphabetChange() {
		// Use setTimeout to avoid reactivity issues
		setTimeout(() => {
			const newMinLength = params.alphabet === 'full' ? 44 : 47;
			if (params.length! < newMinLength) {
				// Force reactivity by reassigning the entire params object
				params = { ...params, length: newMinLength };
			}
		}, 0);
	}

	// Initialize params based on navigation source
	onMount(() => {
		// Check if we're coming from result page with existing params
		if ($resultState && $resultState.endpoint === 'api-key' && $resultState.params) {
			// Coming from result page - use existing params
			params = { ...$resultState.params } as ApiKeyParams;
		} else {
			// Coming from menu or fresh load - use defaults
			params = getDefaultParams();
		}

		// Override with URL parameters if present
		const urlLength = searchParams.get('length');
		const urlAlphabet = searchParams.get('alphabet');
		const urlSeed = searchParams.get('seed');

		if (urlLength) {
			const lengthNum = parseInt(urlLength);
			if (!isNaN(lengthNum) && lengthNum >= 44 && lengthNum <= 64) {
				params.length = lengthNum;
			}
		}

		if (urlAlphabet && isValidApiKeyAlphabet(urlAlphabet)) {
			params.alphabet = urlAlphabet;
		}

		if (urlSeed) {
			urlProvidedSeed = urlSeed;
		}
	});
</script>

<svelte:head>
	<title>{$_('apiKey.title')}</title>
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
					<span class="text-xl text-white">🔑</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
					{$_('apiKey.title')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300">
					{$_('apiKey.description')}
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
						<!-- Alphabet -->
						<AlphabetSelector
							bind:value={params.alphabet}
							options={alphabetOptions}
							label={$_('apiKey.alphabet')}
							id="alphabet"
							onChange={handleAlphabetChange}
						/>

						<!-- Length -->
						<div>
							<label
								for="length"
								class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
							>
								{$_('apiKey.length')} ({minLength}-64 {$_('common.characters')})
							</label>
							<div class="flex items-center gap-4">
								<input
									type="range"
									id="length"
									bind:value={params.length}
									min={minLength}
									max="64"
									class="flex-1 h-2 bg-blue-600 rounded appearance-none outline-none slider"
								/>
								<span
									class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center"
									>{params.length}</span
								>
							</div>
							{#if !lengthValid}
								<p class="text-red-500 text-sm mt-1">
									{$_('common.length')}
									{$_('common.mustBeBetween')}
									{minLength}
									{$_('common.and')} 64
								</p>
							{/if}
							<div
								class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3 mt-3"
							>
								<p class="text-sm text-blue-800 dark:text-blue-200">
									<strong>{$_('common.format')}:</strong> ak_ prefix + {params.length || 44}
									{$_('apiKey.randomCharacters')}
									{#if params.alphabet === 'no-look-alike'}
										{$_('apiKey.noLookAlikeAlphabet')}
									{:else}
										{$_('apiKey.fullAlphanumericAlphabet')}
									{/if}
									<br /><strong>{$_('common.security')}:</strong>
									{#if params.alphabet === 'no-look-alike'}
										{$_('apiKey.noLookAlikeNote').replace('{0}', minLength.toString())}
									{:else}
										{$_('apiKey.fullAlphanumericNote').replace('{0}', minLength.toString())}
									{/if}
								</p>
							</div>
						</div>

						<!-- Format Notice -->
						<div
							class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4"
						>
							<div class="flex items-start">
								<span class="text-blue-600 dark:text-blue-400 mr-2">ℹ️</span>
								<div class="text-sm text-blue-800 dark:text-blue-200">
									<strong>{$_('common.format')}:</strong>
									{$_('apiKey.formatNotice')}
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
									{$_('apiKey.securityNotice')}
								</div>
							</div>
						</div>

						<!-- Action Buttons -->
						<div class="flex flex-col sm:flex-row gap-4 mt-4">
							<!-- Generate API key button -->
							<GenerateButton
								type="submit"
								disabled={!formValid || $isLoading}
								loading={$isLoading}
								text={$_('apiKey.generateApiKey')}
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
