<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { navigationItems } from '$lib/stores/navigation';
	import { clearResult } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import MenuCard from '$lib/components/MenuCard.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { authStore } from '$lib/stores/auth';
	import { flashMessagesStore } from '$lib/stores/flashMessages';



	onMount(async () => {
		// Clear result state when returning to menu - this resets all form values to defaults
		clearResult();
	});

	// Effect to watch for username changes (only on this page)
	$effect(() => {
		const username = $authStore.user?.username;
		const currentPath = $page.url.pathname;
		
		if (username && currentPath === '/') {
			// Username is set and we're on home page, show message
			flashMessagesStore.addMessage(`username = ${username}`);
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

		<!-- Flash Messages -->
		<FlashMessages />

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
