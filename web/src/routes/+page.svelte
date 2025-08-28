<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { navigationItems } from '$lib/stores/navigation';
	import { clearResult } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import MenuCard from '$lib/components/MenuCard.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { base58 } from '@scure/base';

	// Debug message state
	let debugMessage = '';
	let showDebug = false;

	/**
	 * Decode base58 next parameter and save to localStorage
	 */
	function processNextParameter(nextBase58: string) {
		try {
			// Decode base58 to bytes
			const bytes = base58.decode(nextBase58);
			
			// Convert bytes to string
			const decoder = new globalThis.TextDecoder();
			const jsonString = decoder.decode(bytes);
			
			// Parse JSON
			const nextObject = JSON.parse(jsonString);
			
			// Save to localStorage
			localStorage.setItem('pending_next_params', JSON.stringify(nextObject));
			
			// Show debug message
			debugMessage = `✅ Parámetro 'next' decodificado correctamente:
			
Original base58: ${nextBase58}

Objeto decodificado:
${JSON.stringify(nextObject, null, 2)}

Guardado en localStorage como 'pending_next_params'`;
			showDebug = true;
			
			// Auto-hide debug after 20 seconds
			setTimeout(() => {
				showDebug = false;
			}, 20000);
			
		} catch (error) {
			console.error('Error decoding next parameter:', error);
			debugMessage = `❌ Error decodificando parámetro 'next':
			
Base58: ${nextBase58}
Error: ${error instanceof Error ? error.message : 'Unknown error'}`;
			showDebug = true;
			
			// Auto-hide debug after 20 seconds
			setTimeout(() => {
				showDebug = false;
			}, 20000);
		}
	}

	onMount(async () => {
		// Clear result state when returning to menu - this resets all form values to defaults
		clearResult();
		
		// Check for next parameter in URL
		const nextParam = $page.url.searchParams.get('next');
		if (nextParam) {
			processNextParameter(nextParam);
		}
	});

	function getTranslatedTitle(itemId: string): string {
		switch (itemId) {
			case 'custom':
				return $_('custom.title');
			case 'password':
				return $_('password.title');
			case 'api-key':
				return $_('apiKey.title');
			case 'mnemonic':
				return $_('mnemonic.title');
			default:
				return '';
		}
	}

	function getTranslatedDescription(itemId: string): string {
		switch (itemId) {
			case 'custom':
				return $_('custom.description');
			case 'password':
				return $_('password.description');
			case 'api-key':
				return $_('apiKey.description');
			case 'mnemonic':
				return $_('mnemonic.description');
			default:
				return '';
		}
	}
</script>

<svelte:head>
	<title>{$_('menu.title')} - {$_('menu.brandName')}</title>
	<meta name="description" content={$_('menu.description')} />
</svelte:head>

<div
	class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"
>
	<div class="container mx-auto px-4 py-8">
		<!-- Header -->
		<header class="text-center mb-12">
			<div class="inline-flex items-center justify-center w-16 h-16 bg-blue-600 rounded-full mb-6">
				<span class="text-2xl text-white">🎲</span>
			</div>
			<h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
				{$_('menu.title')}
			</h1>
			<p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
				{$_('menu.description')}
			</p>
		</header>

		<!-- Debug Message -->
		{#if showDebug}
			<div class="max-w-4xl mx-auto mb-8">
				<div class="bg-yellow-50 dark:bg-yellow-900 border border-yellow-200 dark:border-yellow-700 rounded-lg p-4 shadow-md">
					<div class="flex items-start">
						<div class="flex-shrink-0">
							<span class="text-2xl">🔍</span>
						</div>
						<div class="ml-3 flex-1">
							<h3 class="text-sm font-medium text-yellow-800 dark:text-yellow-200 mb-2">
								Debug - Procesamiento parámetro 'next'
							</h3>
							<div class="text-sm text-yellow-700 dark:text-yellow-300 whitespace-pre-wrap font-mono">
								{debugMessage}
							</div>
							<button
								on:click={() => showDebug = false}
								class="mt-3 px-3 py-1 text-xs bg-yellow-200 dark:bg-yellow-800 hover:bg-yellow-300 dark:hover:bg-yellow-700 text-yellow-800 dark:text-yellow-200 rounded transition-colors"
							>
								Cerrar
							</button>
						</div>
					</div>
				</div>
			</div>
		{/if}

		<!-- Navigation Cards -->
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto mb-12">
			{#each navigationItems as item}
				<MenuCard
					path={item.path}
					icon={item.icon}
					title={getTranslatedTitle(item.id)}
					description={getTranslatedDescription(item.id)}
				/>
			{/each}
		</div>

		<!-- Footer with Version Information -->
		<Footer />
	</div>
</div>
