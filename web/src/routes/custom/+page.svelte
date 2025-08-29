<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import Footer from '$lib/components/Footer.svelte';
	// import Button from '$lib/components/Button.svelte';
	import GenerateButton from '$lib/components/GenerateButton.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import AlphabetSelector from '$lib/components/AlphabetSelector.svelte';
	import TextInput from '$lib/components/TextInput.svelte';
	import AuthGuard from '$lib/components/AuthGuard.svelte';
	import EmailInputDialog from '$lib/components/EmailInputDialog.svelte';
	import { isLoading, resultState } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
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
	let urlProvidedSeed: string = ''; // Seed from URL parameters (read-only)

	// Eliminados los debug messages

	// Login dialog state
	let showEmailDialog = false;

	// Get URL parameters reactively
	$: searchParams = $page.url.searchParams;

	// Function to validate alphabet parameter
	function isValidAlphabet(value: string): value is AlphabetType {
		return ['base58', 'no-look-alike', 'full', 'full-with-symbols', 'numeric'].includes(value);
	}

	// Reactive alphabet options that update when language changes
	$: alphabetOptions = [
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
	];

	// Seed validation functions (commented out as we now only show seeds read-only)
	// function isValidHexSeed(seed: string): boolean {
	// 	if (!seed) return true; // Empty seed is valid (optional)
	// 	// Must be exactly 64 characters (32 bytes in hex) and only hex characters
	// 	return /^[0-9a-fA-F]{64}$/.test(seed);
	// }

	// Validation
	$: lengthValid = params.length && params.length >= 2 && params.length <= 128;
	$: prefixValid = !params.prefix || params.prefix.length <= 32;
	$: suffixValid = !params.suffix || params.suffix.length <= 32;
	$: formValid = lengthValid && prefixValid && suffixValid;

	// Reference to AuthGuard component
	let authGuard: AuthGuard;

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
			// No autenticado - mostrar diálogo de email
			showEmailDialog = true;
			return;
		}

		// Usuario autenticado - proceder con la generación
		proceedWithGeneration();
	}

	function proceedWithGeneration() {
		// Create URL parameters for result page - result will handle API call
		const urlParams = new URLSearchParams();
		urlParams.set('endpoint', 'custom');

		// Add generation parameters
		urlParams.set('length', (params.length ?? 21).toString());
		urlParams.set('alphabet', params.alphabet ?? 'base58');
		if (params.prefix) urlParams.set('prefix', params.prefix);
		if (params.suffix) urlParams.set('suffix', params.suffix);

		// Add seed if provided from URL
		if (urlProvidedSeed) {
			urlParams.set('seed', urlProvidedSeed);
		}

		goto(`/result?${urlParams.toString()}`);
	}

	// Email dialog handlers
	let emailDialogRef: EmailInputDialog;

	// Create next parameter object with current form state
	$: nextObject = {
		endpoint: 'custom',
		length: params.length,
		alphabet: params.alphabet,
		prefix: params.prefix || undefined,
		suffix: params.suffix || undefined,
		...(urlProvidedSeed && { seed: urlProvidedSeed })
	};

	function handleEmailDialogClose() {
		showEmailDialog = false;
	}

	function handleEmailSubmitted(event: globalThis.CustomEvent<{ email: string }>) {
		// Email entered and moving to confirmation step
		console.log('Email entered:', event.detail.email);
	}

	async function handleEmailConfirmed(
		event: globalThis.CustomEvent<{ email: string; redirectUrl: string }>
	) {
		const { email, redirectUrl } = event.detail;

		try {
			// Obtener el host actual donde se ejecuta la UI
			const currentHost = window.location.origin;

			const requestBody = {
				email: email,
				ui_host: currentHost
			};

			const response = await fetch('/api/login/', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(requestBody)
			});

			if (response.ok) {
				// Magic link enviado correctamente, redirigir con el parámetro next
				emailDialogRef?.resetSubmitting();
				goto(redirectUrl);
			} else {
				// En caso de error, mostrar mensaje de error
				emailDialogRef?.setError($_('common.sendError'));
			}
		} catch (error) {
			console.error('Error sending magic link:', error);
			emailDialogRef?.setError($_('common.connectionError'));
		}
	}

	// Initialize params based on navigation source
	onMount(() => {
		// Check if we're coming from result page with existing params
		if (
			$resultState &&
			($resultState.endpoint === 'custom' || $resultState.endpoint === 'generate') &&
			$resultState.params
		) {
			// Coming from result page - use existing params
			params = { ...$resultState.params } as GenerateParams;
		} else {
			// Coming from menu or fresh load - use defaults
			params = getDefaultParams();
		}

		// Override with URL parameters if present
		const urlLength = searchParams.get('length');
		const urlAlphabet = searchParams.get('alphabet');
		const urlPrefix = searchParams.get('prefix');
		const urlSuffix = searchParams.get('suffix');
		const urlSeed = searchParams.get('seed');

		if (urlLength) {
			const lengthNum = parseInt(urlLength);
			if (!isNaN(lengthNum) && lengthNum >= 2 && lengthNum <= 128) {
				params.length = lengthNum;
			}
		}

		if (urlAlphabet && isValidAlphabet(urlAlphabet)) {
			params.alphabet = urlAlphabet;
		}

		if (urlPrefix !== null && urlPrefix.length <= 32) {
			params.prefix = urlPrefix;
		}

		if (urlSuffix !== null && urlSuffix.length <= 32) {
			params.suffix = urlSuffix;
		}

		if (urlSeed) {
			urlProvidedSeed = urlSeed;
		}
	});
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

		<!-- Auth Guard: wraps the form -->
		<AuthGuard bind:this={authGuard}>
			<!-- Form -->
			<div class="max-w-2xl mx-auto">
				<div
					class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6"
				>
					<form onsubmit={handleGenerate} class="space-y-6">
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
									bind:value={params.length}
									min="2"
									max="128"
									class="flex-1 h-2 bg-blue-600 rounded appearance-none outline-none slider"
								/>
								<span
									class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center"
									>{params.length}</span
								>
							</div>
							{#if !lengthValid}
								<p class="text-red-500 text-sm mt-1">{$_('custom.lengthMustBeBetween')}</p>
							{/if}
						</div>

						<!-- Alphabet -->
						<AlphabetSelector
							bind:value={params.alphabet}
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
							bind:value={params.prefix}
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
							bind:value={params.suffix}
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
								disabled={!formValid || $isLoading}
								loading={$isLoading}
								text={$_('custom.generateHash')}
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

<!-- Email Input Dialog -->
<EmailInputDialog
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
/>
