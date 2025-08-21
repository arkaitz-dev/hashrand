<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import BackButton from '$lib/components/BackButton.svelte';
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
	import Icon from '$lib/components/Icon.svelte';
	import { resultState, error, setResult, setLoading, setError, isLoading } from '$lib/stores/result';
	import { _, currentLanguage } from '$lib/stores/i18n';
	import { isRTL } from '$lib/stores/rtl';

	let copySuccess = false;
	let copyTimeout: number;
	let showGenerationDetails = false; // Collapsed by default on mobile
	let showParametersUsed = false; // Collapsed by default on mobile

	function toggleGenerationDetails() {
		showGenerationDetails = !showGenerationDetails;
	}

	function toggleParametersUsed() {
		showParametersUsed = !showParametersUsed;
	}

	// Redirect if no result
	onMount(() => {
		if (!$resultState) {
			goto('/');
		}
	});

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

	function getEndpointDisplayName(endpoint: string): string {
		switch (endpoint) {
			case 'custom': return $_('custom.title');
			case 'generate': return $_('custom.title');
			case 'password': return $_('password.title');
			case 'api-key': return $_('apiKey.title');
			default: return endpoint;
		}
	}

	function getEndpointIcon(endpoint: string): string {
		switch (endpoint) {
			case 'custom': return '🎲';
			case 'generate': return '🎲';
			case 'password': return '🔐';
			case 'api-key': return '🔑';
			default: return '📝';
		}
	}

	function getEndpointColor(endpoint: string): string {
		switch (endpoint) {
			case 'custom': return 'blue';
			case 'generate': return 'blue';
			case 'password': return 'blue';
			case 'api-key': return 'blue';
			default: return 'gray';
		}
	}

	function formatTimestamp(date: Date): string {
		// Use the current language for date formatting
		let currentLang = 'en';
		const unsubscribe = currentLanguage.subscribe(lang => currentLang = lang);
		unsubscribe();
		
		// Map language codes to locale identifiers for date formatting
		const localeMap: Record<string, string> = {
			'en': 'en-US',
			'es': 'es-ES', 
			'pt': 'pt-PT',
			'fr': 'fr-FR',
			'de': 'de-DE',
			'ru': 'ru-RU',
			'zh': 'zh-CN',
			'ar': 'ar-SA',
			'eu': 'eu-ES',
			'ca': 'ca-ES',
			'gl': 'gl-ES'
		};
		
		const locale = localeMap[currentLang] || 'en-US';
		
		return new Intl.DateTimeFormat(locale, {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit'
		}).format(date);
	}

	function translateParameterKey(key: string): string {
		const translations: Record<string, string> = {
			length: $_('common.length'),
			alphabet: $_('common.alphabet'),
			prefix: $_('custom.prefix') || 'Prefix',
			suffix: $_('custom.suffix') || 'Suffix'
		};
		
		return translations[key] || key.replace(/([A-Z])/g, ' $1').trim();
	}

	function getPreviousPath(): string {
		if (!$resultState) return '/';
		// Map endpoint names to actual route paths
		const endpointRoutes: Record<string, string> = {
			'custom': '/custom',
			'generate': '/custom', // backward compatibility
			'password': '/password',
			'api-key': '/api-key'
		};
		return endpointRoutes[$resultState.endpoint] || '/';
	}

	async function regenerateHash() {
		if (!$resultState || $isLoading) return;

		// Reset copy success state immediately
		copySuccess = false;
		setLoading(true);
		
		try {
			const { api } = await import('$lib/api');
			let result: string;

			// Call the appropriate API method based on endpoint
			switch ($resultState.endpoint) {
				case 'custom':
				case 'generate':
					result = await api.generate($resultState.params);
					break;
				case 'password':
					result = await api.generatePassword($resultState.params);
					break;
				case 'api-key':
					result = await api.generateApiKey($resultState.params);
					break;
				default:
					throw new Error($_('common.unknownEndpoint'));
			}
			
			// Update result with new value but keep same parameters and endpoint
			setResult({
				value: result,
				params: $resultState.params,
				endpoint: $resultState.endpoint,
				timestamp: new Date()
			});

			// Reset copy success state
			copySuccess = false;
		} catch (error) {
			setError(error instanceof Error ? error.message : $_('common.failedToRegenerate'));
		} finally {
			setLoading(false);
		}
	}
</script>

<svelte:head>
	<title>{$_('common.result')}</title>
</svelte:head>

{#if $resultState}
	{@const color = getEndpointColor($resultState.endpoint)}
	<div class="min-h-screen bg-gradient-to-br from-{color}-50 to-{color}-100 dark:from-gray-900 dark:to-gray-800">
		<div class="container mx-auto px-4 py-8">
			<!-- Header -->
			<div class="mb-8">

				<div class="text-center">
					<div class="inline-flex items-center justify-center w-16 h-16 bg-{color}-600 rounded-full mb-6">
						<span class="text-2xl text-white">{getEndpointIcon($resultState.endpoint)}</span>
					</div>
					<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
						{$_('common.result')}
					</h1>
				</div>
			</div>

			<!-- Result Display -->
			<div class="max-w-4xl mx-auto">
				<div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border border-gray-200 dark:border-gray-700 p-6 mb-6">
					<!-- Result Value -->
					<div class="mb-6">
						<label for="generated-value" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
							{$_('common.generatedValue')}
						</label>
						<div class="relative">
							<textarea
								id="generated-value"
								readonly
								value={$isLoading ? $_('common.loading') + '...' : $resultState.value}
								class="w-full p-4 pb-12 bg-gray-50 dark:bg-gray-900 border border-gray-300 dark:border-gray-600 rounded-lg font-mono text-sm resize-none focus:ring-2 focus:ring-{color}-500 focus:border-{color}-500 min-h-[100px] {$isLoading ? 'text-gray-500 dark:text-gray-400' : ''}"
								onclick={(e) => (e.target as HTMLTextAreaElement)?.select()}
							></textarea>
							{#if !$isLoading}
								<button
									onclick={copyToClipboard}
									class="absolute bottom-3 right-3 px-3 py-1.5 bg-{color}-600 hover:bg-{color}-700 text-white text-xs font-medium rounded-md transition-colors duration-200 flex items-center gap-1"
									class:bg-green-600={copySuccess}
									class:hover:bg-green-700={copySuccess}
								>
								{#if copySuccess}
									<Icon name="check" size="w-3 h-3" />
									{$_('common.copied')}
								{:else}
									<Icon name="copy" size="w-3 h-3" />
									{$_('common.copy')}
								{/if}
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
								<h3 class="text-lg font-semibold text-gray-900 dark:text-white">{$_('common.generationDetails')}</h3>
								<!-- Toggle icon - only visible on mobile -->
								<Icon 
									name="chevron-down" 
									size="w-5 h-5" 
									class="text-gray-500 dark:text-gray-400 md:hidden transition-transform duration-200 {showGenerationDetails ? 'rotate-180' : ''} {$isRTL ? 'rtl-flip-chevron' : ''}" 
								/>
							</button>
							
							<!-- Content - collapsible on mobile, always visible on desktop -->
							<dl class="space-y-2 {showGenerationDetails ? 'block' : 'hidden'} md:block">
								<div>
									<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">{$_('common.type')}</dt>
									<dd class="text-sm text-gray-900 dark:text-white">{getEndpointDisplayName($resultState.endpoint)}</dd>
								</div>
								<div>
									<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">{$_('common.length')}</dt>
									<dd class="text-sm text-gray-900 dark:text-white">{$resultState.value.length} {$_('common.characters')}</dd>
								</div>
								<div>
									<dt class="text-sm font-medium text-gray-500 dark:text-gray-400">{$_('common.generated')}</dt>
									<dd class="text-sm text-gray-900 dark:text-white">{formatTimestamp($resultState.timestamp)}</dd>
								</div>
							</dl>
						</div>

						<!-- Parameters Used -->
						<div>
							<!-- Header with toggle for mobile -->
							<button 
								onclick={toggleParametersUsed}
								class="w-full text-left flex items-center justify-between md:pointer-events-none md:cursor-default mb-3"
							>
								<h3 class="text-lg font-semibold text-gray-900 dark:text-white">{$_('common.parametersUsed')}</h3>
								<!-- Toggle icon - only visible on mobile -->
								<Icon 
									name="chevron-down" 
									size="w-5 h-5" 
									class="text-gray-500 dark:text-gray-400 md:hidden transition-transform duration-200 {showParametersUsed ? 'rotate-180' : ''} {$isRTL ? 'rtl-flip-chevron' : ''}" 
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
												{typeof value === 'boolean' ? (value ? $_('common.yes') || 'Yes' : $_('common.no') || 'No') : value}
											</dd>
										</div>
									{/if}
								{/each}
							</dl>
						</div>
					</div>
				</div>

				<!-- Actions -->
				<div class="flex flex-col sm:flex-row gap-4 justify-center">
					<button
						onclick={regenerateHash}
						disabled={$isLoading}
						class="px-6 py-3 text-white font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-2 min-w-[180px] {$isLoading ? 'bg-gray-400 cursor-not-allowed' : `bg-${color}-600 hover:bg-${color}-700 cursor-pointer`} {$isRTL ? 'rtl-reverse' : ''}"
					>
						<Icon name="refresh" size="w-4 h-4" class={$isLoading ? 'animate-spin-fast' : ''} />
						{$_('common.generateAnother')}
					</button>
					<button
						onclick={() => goto(getPreviousPath())}
						class="px-6 py-3 bg-gray-500 hover:bg-gray-600 text-white font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-2 {$isRTL ? 'rtl-reverse' : ''}"
					>
						<Icon name="settings" size="w-4 h-4" />
						{$_('common.adjustSettings')}
					</button>
					<button
						onclick={() => goto('/')}
						class="px-6 py-3 bg-gray-600 hover:bg-gray-700 text-white font-medium rounded-lg transition-colors duration-200 flex items-center justify-center gap-2 {$isRTL ? 'rtl-reverse' : ''}"
					>
						<Icon name="briefcase" size="w-4 h-4" />
						{$_('common.backToMenu')}
					</button>
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
{:else if $error}
	<div class="min-h-screen bg-gradient-to-br from-red-50 to-red-100 dark:from-gray-900 dark:to-gray-800">
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
{/if}