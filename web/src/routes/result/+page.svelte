<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import BackButton from '$lib/components/BackButton.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import Iconize from '$lib/components/Iconize.svelte';
	import DateTimeLocalized from '$lib/components/DateTimeLocalized.svelte';
	import {
		resultState,
		error,
		setResult,
		setLoading,
		setError,
		isLoading
	} from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import { isRTL } from '$lib/stores/rtl';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { flashMessagesStore } from '$lib/stores/flashMessages';
	import { dialogStore } from '$lib/stores/dialog';

	let copySuccess = $state(false);
	let copyTimeout: ReturnType<typeof setTimeout>;
	let showGenerationDetails = $state(false); // Collapsed by default on mobile
	let showParametersUsed = $state(false); // Collapsed by default on mobile
	// showSeedDialog removed - now using DialogContainer system

	function toggleGenerationDetails() {
		showGenerationDetails = !showGenerationDetails;
	}

	function toggleParametersUsed() {
		showParametersUsed = !showParametersUsed;
	}

	// Get URL parameters reactively
	let searchParams = $derived($page.url.searchParams);

	// Only treat as "provided seed" if seed parameter comes from URL GET parameters
	// (this controls whether to show the regenerate button)
	let usedProvidedSeed = $derived(searchParams.has('seed'));

	// Handle result state and API calls
	onMount(async () => {
		// If there are URL parameters, ALWAYS generate from them (override any existing state)
		if (searchParams.size > 0) {
			await generateFromParams();
			return;
		}

		// If no URL parameters and no result state, redirect to home
		if (!$resultState) {
			goto('/');
			return;
		}
	});

	// Function to generate result from URL parameters
	async function generateFromParams() {
		const endpoint = searchParams.get('endpoint');
		if (!endpoint) {
			goto('/');
			return;
		}

		// Verify auth is available with automatic refresh if needed
		const { authStore } = await import('$lib/stores/auth');
		const isAuthenticated = await authStore.ensureAuthenticated();

		if (!isAuthenticated) {
			goto('/');
			return;
		}

		setLoading(true);

		try {
			const { api } = await import('$lib/api');

			// Build parameters object
			const params: Record<string, string | number | boolean> = { raw: true };

			// Get common parameters
			const length = searchParams.get('length');
			const alphabet = searchParams.get('alphabet');
			const prefix = searchParams.get('prefix');
			const suffix = searchParams.get('suffix');
			const inputSeed = searchParams.get('seed');

			// Get mnemonic-specific parameters
			const language = searchParams.get('language');
			const words = searchParams.get('words');

			if (length) params.length = parseInt(length);
			if (alphabet) params.alphabet = alphabet;
			if (prefix) params.prefix = prefix;
			if (suffix) params.suffix = suffix;
			if (language) params.language = language;
			if (words) params.words = parseInt(words);

			let response: import('$lib/types').CustomHashResponse;

			// If we have a seed, use POST to the appropriate endpoint
			if (inputSeed) {
				if (endpoint === 'custom' || endpoint === 'generate') {
					// Build seed request with only relevant params
					const customSeedRequest = {
						seed: inputSeed,
						endpoint
					} as import('$lib/types').SeedGenerateRequest;

					if (params.length) customSeedRequest.length = params.length as number;
					if (params.alphabet)
						customSeedRequest.alphabet = params.alphabet as import('$lib/types').AlphabetType;
					if (params.prefix) customSeedRequest.prefix = params.prefix as string;
					if (params.suffix) customSeedRequest.suffix = params.suffix as string;

					response = await api.generateWithSeed(customSeedRequest);
				} else if (endpoint === 'password') {
					const passwordSeedRequest = {
						seed: inputSeed
					} as import('$lib/types').SeedPasswordRequest;

					if (params.length) passwordSeedRequest.length = params.length as number;
					if (params.alphabet)
						passwordSeedRequest.alphabet = params.alphabet as 'no-look-alike' | 'full-with-symbols';

					response = await api.generatePasswordWithSeed(passwordSeedRequest);
				} else if (endpoint === 'api-key') {
					const apiKeySeedRequest = {
						seed: inputSeed
					} as import('$lib/types').SeedApiKeyRequest;

					if (params.length) apiKeySeedRequest.length = params.length as number;
					if (params.alphabet)
						apiKeySeedRequest.alphabet = params.alphabet as 'no-look-alike' | 'full';

					response = await api.generateApiKeyWithSeed(apiKeySeedRequest);
				} else if (endpoint === 'mnemonic') {
					const mnemonicSeedRequest = {
						seed: inputSeed
					} as import('$lib/types').SeedMnemonicRequest;

					if (params.language) mnemonicSeedRequest.language = params.language as string;
					if (params.words) mnemonicSeedRequest.words = params.words as 12 | 24;

					response = await api.generateMnemonicWithSeed(mnemonicSeedRequest);
				} else {
					throw new Error($_('common.unknownEndpoint'));
				}
			} else {
				// Call the appropriate API method based on endpoint
				switch (endpoint) {
					case 'custom':
					case 'generate':
						response = await api.generate(params);
						break;
					case 'password':
						response = await api.generatePassword(params);
						break;
					case 'api-key':
						response = await api.generateApiKey(params);
						break;
					case 'mnemonic':
						response = await api.generateMnemonic(params);
						break;
					default:
						throw new Error($_('common.unknownEndpoint'));
				}
			}

			// All endpoints now return CustomHashResponse
			const value = response.hash;
			const seed = response.seed;
			const otp = response.otp;
			const responseTimestamp = new Date(response.timestamp * 1000); // Convert from seconds to ms

			// Set the result state
			setResult({
				value,
				seed,
				otp,
				params,
				endpoint,
				timestamp: responseTimestamp
			});
		} catch (error) {
			// Check if it's a session expiration (401 Unauthorized)
			if (error instanceof Error && error.message.includes('401')) {
				// Session expired - redirect to home with flash message
				flashMessagesStore.addMessage($_('common.sessionExpired'));
				await goto('/');
				return;
			}

			const errorMsg = error instanceof Error ? error.message : $_('common.failedToGenerate');
			flashMessagesStore.addMessage(`❌ Error en generación: ${errorMsg}`);
			setError(errorMsg);
		} finally {
			setLoading(false);
		}
	}

	async function copyToClipboard() {
		if (!$resultState?.value) return;

		try {
			await navigator.clipboard.writeText($resultState.value);
			copySuccess = true;

			// Clear success state after 2 seconds
			clearTimeout(copyTimeout);
			copyTimeout = setTimeout(() => {
				copySuccess = false;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy:', err);
			// Fallback for older browsers
			try {
				const textArea = document.createElement('textarea');
				textArea.value = $resultState.value;
				document.body.appendChild(textArea);
				textArea.select();
				document.execCommand('copy');
				document.body.removeChild(textArea);
				copySuccess = true;
				clearTimeout(copyTimeout);
				copyTimeout = setTimeout(() => {
					copySuccess = false;
				}, 2000);
			} catch (fallbackErr) {
				console.error('Fallback copy failed:', fallbackErr);
			}
		}
	}

	// Function to get endpoint display name
	function getEndpointDisplayName(endpoint: string): string {
		switch (endpoint) {
			case 'custom':
				return $_('custom.title');
			case 'generate':
				return $_('custom.title');
			case 'password':
				return $_('password.title');
			case 'api-key':
				return $_('apiKey.title');
			case 'mnemonic':
				return $_('mnemonic.title');
			default:
				return endpoint;
		}
	}

	function getEndpointIcon(endpoint: string): string {
		switch (endpoint) {
			case 'custom':
				return '🎲';
			case 'generate':
				return '🎲';
			case 'password':
				return '🔐';
			case 'api-key':
				return '🔑';
			case 'mnemonic':
				return '💾';
			default:
				return '📝';
		}
	}

	function getEndpointColor(endpoint: string): string {
		switch (endpoint) {
			case 'custom':
				return 'blue';
			case 'generate':
				return 'blue';
			case 'password':
				return 'blue';
			case 'api-key':
				return 'blue';
			case 'mnemonic':
				return 'blue';
			default:
				return 'gray';
		}
	}

	// Reactive parameter key translation that updates when language changes
	let translateParameterKey = $derived((key: string): string => {
		const translations: Record<string, string> = {
			length: $_('common.length'),
			alphabet: $_('common.alphabet'),
			prefix: $_('custom.prefix') || 'Prefix',
			suffix: $_('custom.suffix') || 'Suffix',
			language: $_('mnemonic.language'),
			words: $_('mnemonic.wordCount')
		};

		return translations[key] || key.replace(/([A-Z])/g, ' $1').trim();
	});

	// Reactive parameter value translation that updates when language changes
	let translateParameterValue = $derived(
		(key: string, value: string | number | boolean): string => {
			if (typeof value === 'boolean') {
				return value ? $_('common.yes') || 'Yes' : $_('common.no') || 'No';
			}

			// Translate alphabet types
			if (key === 'alphabet' && typeof value === 'string') {
				return $_(`alphabets.${value}`) || value;
			}

			// Translate language values
			if (key === 'language' && typeof value === 'string') {
				return $_(`mnemonic.languages.${value}`) || value;
			}

			// Translate words count with descriptive text
			if (key === 'words' && typeof value === 'number') {
				return value === 12 ? $_('mnemonic.words12') : $_('mnemonic.words24');
			}

			return String(value);
		}
	);

	async function handleAdjustSettings() {
		if (!$resultState) {
			goto('/');
			return;
		}

		// If there's no seed, go directly without dialog
		if (!$resultState.seed) {
			goto(getPreviousPath());
			return;
		}

		// Show seed dialog using new dialog system
		dialogStore.show('seed', { onSeedChoice: handleSeedChoice });
	}

	function handleSeedChoice(reuseExistingSeed: boolean) {
		dialogStore.close();
		if (reuseExistingSeed) {
			// Include seed in the URL parameters
			goto(getPreviousPathWithSeed());
		} else {
			// Don't include seed in URL parameters
			goto(getPreviousPath());
		}
	}

	function getPreviousPathWithSeed(): string {
		if (!$resultState) return '/';

		// Map endpoint names to actual route paths
		const endpointRoutes: Record<string, string> = {
			custom: '/custom',
			generate: '/custom', // backward compatibility
			password: '/password',
			'api-key': '/api-key',
			mnemonic: '/mnemonic'
		};

		const basePath = endpointRoutes[$resultState.endpoint] || '/';

		// Add parameters to URL including seed
		if ($resultState.params && Object.keys($resultState.params).length > 0) {
			const urlParams = new URLSearchParams();

			// Add common parameters
			if ($resultState.params.length) {
				urlParams.set('length', $resultState.params.length.toString());
			}
			if ($resultState.params.alphabet) {
				urlParams.set('alphabet', String($resultState.params.alphabet));
			}

			// Add endpoint-specific parameters
			if ($resultState.endpoint === 'custom' || $resultState.endpoint === 'generate') {
				if ($resultState.params.prefix) {
					urlParams.set('prefix', String($resultState.params.prefix));
				}
				if ($resultState.params.suffix) {
					urlParams.set('suffix', String($resultState.params.suffix));
				}
			} else if ($resultState.endpoint === 'mnemonic') {
				if ($resultState.params.language) {
					urlParams.set('language', String($resultState.params.language));
				}
				if ($resultState.params.words) {
					urlParams.set('words', String($resultState.params.words));
				}
			}

			// Add seed if available
			if ($resultState.seed) {
				urlParams.set('seed', $resultState.seed);
			}

			const queryString = urlParams.toString();
			return queryString ? `${basePath}?${queryString}` : basePath;
		}

		return basePath;
	}

	function getPreviousPath(): string {
		if (!$resultState) return '/';

		// Map endpoint names to actual route paths
		const endpointRoutes: Record<string, string> = {
			custom: '/custom',
			generate: '/custom', // backward compatibility
			password: '/password',
			'api-key': '/api-key',
			mnemonic: '/mnemonic'
		};

		const basePath = endpointRoutes[$resultState.endpoint] || '/';

		// Add parameters to URL if they exist
		if ($resultState.params && Object.keys($resultState.params).length > 0) {
			const urlParams = new URLSearchParams();

			// Add common parameters
			if ($resultState.params.length) {
				urlParams.set('length', $resultState.params.length.toString());
			}
			if ($resultState.params.alphabet) {
				urlParams.set('alphabet', String($resultState.params.alphabet));
			}

			// Add endpoint-specific parameters
			if ($resultState.endpoint === 'custom' || $resultState.endpoint === 'generate') {
				if ($resultState.params.prefix) {
					urlParams.set('prefix', String($resultState.params.prefix));
				}
				if ($resultState.params.suffix) {
					urlParams.set('suffix', String($resultState.params.suffix));
				}
			} else if ($resultState.endpoint === 'mnemonic') {
				if ($resultState.params.language) {
					urlParams.set('language', String($resultState.params.language));
				}
				if ($resultState.params.words) {
					urlParams.set('words', String($resultState.params.words));
				}
			}

			const queryString = urlParams.toString();
			return queryString ? `${basePath}?${queryString}` : basePath;
		}

		return basePath;
	}

	async function regenerateHash() {
		if (!$resultState || $isLoading) return;

		// Reset copy success state immediately
		copySuccess = false;
		setLoading(true);

		try {
			const { api } = await import('$lib/api');
			let response: import('$lib/types').CustomHashResponse;

			// Build parameters excluding seed - force GET request by not including seed
			const paramsForGeneration = { ...$resultState.params };
			delete paramsForGeneration.seed; // Ensure no seed is passed for GET request

			// Call the appropriate API method based on endpoint
			switch ($resultState.endpoint) {
				case 'custom':
				case 'generate':
					response = await api.generate(paramsForGeneration);
					break;
				case 'password':
					response = await api.generatePassword(paramsForGeneration);
					break;
				case 'api-key':
					response = await api.generateApiKey(paramsForGeneration);
					break;
				case 'mnemonic':
					response = await api.generateMnemonic(paramsForGeneration);
					break;
				default:
					throw new Error($_('common.unknownEndpoint'));
			}

			// All endpoints now return CustomHashResponse
			const value = response.hash;
			const seed = response.seed;
			const otp = response.otp;
			const responseTimestamp = new Date(response.timestamp * 1000); // Convert from seconds to ms

			// Update result with new value but keep same parameters and endpoint
			setResult({
				value,
				seed,
				otp,
				params: paramsForGeneration,
				endpoint: $resultState.endpoint,
				timestamp: responseTimestamp
			});

			// Reset copy success state
			copySuccess = false;
		} catch (error) {
			// Check if it's a session expiration (401 Unauthorized)
			if (error instanceof Error && error.message.includes('401')) {
				// Session expired - redirect to home with flash message
				flashMessagesStore.addMessage($_('common.sessionExpired'));
				await goto('/');
				return;
			}

			setError(error instanceof Error ? error.message : $_('common.failedToRegenerate'));
		} finally {
			setLoading(false);
		}
	}
</script>

<svelte:head>
	<title>{$_('common.result')}</title>
</svelte:head>

<!-- <svelte:window on:keydown={handleKeydown} /> -->

{#if $resultState}
	{@const color = getEndpointColor($resultState.endpoint)}
	<div
		class="min-h-screen bg-gradient-to-br from-{color}-50 to-{color}-100 dark:from-gray-900 dark:to-gray-800"
	>
		<div class="container mx-auto px-4 py-8">
			<!-- Header -->
			<div class="mb-8">
				<div class="text-center">
					<div
						class="inline-flex items-center justify-center w-16 h-16 bg-{color}-600 rounded-full mb-6"
					>
						<span class="text-2xl text-white">{getEndpointIcon($resultState.endpoint)}</span>
					</div>
					<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
						{$_('common.result')}
					</h1>
				</div>
			</div>

			<!-- Flash Messages -->
			<FlashMessages />

			<!-- Result Display -->
			<div class="max-w-4xl mx-auto">
				<div
					class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6 mb-6"
				>
					<!-- Result Value -->
					<div class="mb-6">
						<label
							for="generated-value"
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3"
						>
							{$_('common.generatedValue')}
						</label>
						<div class="relative">
							<textarea
								id="generated-value"
								readonly
								value={$isLoading ? $_('common.loading') + '...' : $resultState.value}
								class="w-full p-4 pb-12 bg-gray-50 dark:bg-gray-900 border border-gray-300 dark:border-gray-600 rounded-lg font-mono text-sm resize-none focus:ring-2 focus:ring-{color}-500 focus:border-{color}-500 min-h-[100px] {$isLoading
									? 'text-gray-500 dark:text-gray-400'
									: ''}"
								onclick={(e) => (e.target as HTMLTextAreaElement)?.select()}
							></textarea>
							{#if !$isLoading}
								<!-- RTL-aware copy button -->
								<button
									onclick={copyToClipboard}
									class="absolute bottom-3 {$isRTL
										? 'left-3'
										: 'right-3'} px-2 py-1 text-xs font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-1 {copySuccess
										? 'bg-green-600 hover:bg-green-700 text-white'
										: 'bg-blue-600 hover:bg-blue-700 text-white'}"
								>
									<Iconize
										conf={{
											icon: copySuccess ? 'check' : 'copy',
											iconSize: 'w-3 h-3'
										}}
									>
										{copySuccess ? $_('common.copied') : $_('common.copy')}
									</Iconize>
								</button>
							{/if}
						</div>
					</div>

					<!-- Metadata -->
					<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
						<!-- Generation Details -->
						<div>
							<!-- Header with toggle for mobile -->
							<button
								onclick={toggleGenerationDetails}
								class="w-full text-left flex items-center justify-between md:pointer-events-none md:cursor-default mb-3"
							>
								<h3 class="text-lg font-semibold text-gray-900 dark:text-white">
									{$_('common.generationDetails')}
								</h3>
								<!-- Toggle icon - only visible on mobile -->
								<Icon
									name="chevron-down"
									size="w-5 h-5"
									placeholder="auto"
									class="text-gray-500 dark:text-gray-400 md:hidden transition-transform duration-200 {showGenerationDetails
										? 'rotate-180'
										: ''} {$isRTL ? 'rtl-flip-chevron' : ''}"
								/>
							</button>

							<!-- Content - collapsible on mobile, always visible on desktop -->
							<dl class="space-y-2 {showGenerationDetails ? 'block' : 'hidden'} md:block">
								<div>
									<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">
										{$_('common.type')}
									</dt>
									<dd class="text-sm text-gray-900 dark:text-white">
										{getEndpointDisplayName($resultState.endpoint)}
									</dd>
								</div>
								<div>
									<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">
										{$_('common.length')}
									</dt>
									<dd class="text-sm text-gray-900 dark:text-white">
										{$resultState.value.length}
										{$_('common.characters')}
									</dd>
								</div>
								<div>
									<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">
										{$_('common.generated')}
									</dt>
									<dd class="text-sm text-gray-900 dark:text-white">
										{#if $resultState.timestamp}
											<DateTimeLocalized timestamp={$resultState.timestamp} />
										{/if}
									</dd>
								</div>
								{#if $resultState.seed}
									<div>
										<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">
											{$_('common.seed')}
										</dt>
										<dd class="text-xs font-mono text-gray-900 dark:text-white break-all">
											{$resultState.seed}
										</dd>
									</div>
								{/if}
								{#if $resultState.otp}
									<div>
										<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">
											{$_('common.otp')}
										</dt>
										<dd class="text-sm font-mono text-gray-900 dark:text-white">
											{$resultState.otp}
										</dd>
									</div>
								{/if}
							</dl>
						</div>

						<!-- Parameters Used -->
						<div>
							<!-- Header with toggle for mobile -->
							<button
								onclick={toggleParametersUsed}
								class="w-full text-left flex items-center justify-between md:pointer-events-none md:cursor-default mb-3"
							>
								<h3 class="text-lg font-semibold text-gray-900 dark:text-white">
									{$_('common.parametersUsed')}
								</h3>
								<!-- Toggle icon - only visible on mobile -->
								<Icon
									name="chevron-down"
									size="w-5 h-5"
									placeholder="auto"
									class="text-gray-500 dark:text-gray-400 md:hidden transition-transform duration-200 {showParametersUsed
										? 'rotate-180'
										: ''} {$isRTL ? 'rtl-flip-chevron' : ''}"
								/>
							</button>

							<!-- Content - collapsible on mobile, always visible on desktop -->
							<dl class="space-y-2 {showParametersUsed ? 'block' : 'hidden'} md:block">
								{#each Object.entries($resultState.params) as [key, value]}
									{#if value !== undefined && value !== null && value !== '' && key !== 'raw'}
										<div>
											<dt class="text-sm font-medium text-gray-500 dark:text-gray-400 capitalize">
												{translateParameterKey(key)}
											</dt>
											<dd class="text-sm text-gray-900 dark:text-white">
												{translateParameterValue(key, value)}
											</dd>
										</div>
									{/if}
								{/each}
							</dl>
						</div>
					</div>

					<!-- Actions -->
					<div class="flex flex-col sm:flex-row gap-4 mt-6">
						<!-- RTL-aware regenerate button - only show if not using a provided seed -->
						{#if !usedProvidedSeed}
							<button
								onclick={regenerateHash}
								disabled={$isLoading}
								class="flex-1 text-white px-6 py-4 rounded-lg font-semibold border-none transition-all duration-200 flex items-center justify-center gap-2 {$isLoading
									? 'bg-gray-400 cursor-not-allowed'
									: 'bg-blue-600 hover:bg-blue-700 cursor-pointer hover:shadow-lg'}"
							>
								<Iconize
									conf={{
										icon: 'refresh',
										iconSize: 'w-5 h-5'
									}}
								>
									{$_('common.generateAnother')}
								</Iconize>
							</button>
						{/if}

						<!-- RTL-aware adjust settings button -->
						<button
							onclick={handleAdjustSettings}
							class="flex-1 bg-gray-500 hover:bg-gray-600 text-white px-6 py-4 rounded-lg font-semibold border-none cursor-pointer hover:shadow-lg transition-all duration-200 flex items-center justify-center gap-2 {usedProvidedSeed
								? 'w-full'
								: ''}"
						>
							<Iconize
								conf={{
									icon: 'settings',
									iconSize: 'w-5 h-5'
								}}
							>
								{$_('common.adjustSettings')}
							</Iconize>
						</button>

						<!-- Back to menu button -->
						<BackToMenuButton />
					</div>
				</div>
			</div>

			<!-- Footer with Version Information -->
			<Footer />
		</div>
	</div>

	<!-- Seed dialog removed - using new dialog system now -->
{:else if $error}
	<div
		class="min-h-screen bg-gradient-to-br from-red-50 to-red-100 dark:from-gray-900 dark:to-gray-800"
	>
		<div class="container mx-auto px-4 py-8">
			<div class="max-w-2xl mx-auto text-center">
				<div class="inline-flex items-center justify-center w-16 h-16 bg-red-600 rounded-full mb-6">
					<span class="text-2xl text-white">❌</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-4">
					{$_('common.error')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300 mb-8">
					{$error}
				</p>
				<BackButton to="/" />
			</div>

			<!-- Footer with Version Information -->
			<Footer />
		</div>
	</div>
{/if}
