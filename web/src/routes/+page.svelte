<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { navigationItems } from '$lib/stores/navigation';
	import { clearResult, resultState } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import { isRTL } from '$lib/stores/rtl';
	import Icon from '$lib/components/Icon.svelte';
	import Iconize from '$lib/components/Iconize.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import type { VersionResponse } from '$lib/types';

	onMount(async () => {
		// Clear result state when returning to menu - this resets all form values to defaults
		clearResult();
	});

	function navigateToItem(path: string) {
		goto(path);
	}

	function getTranslatedTitle(itemId: string): string {
		switch (itemId) {
			case 'custom': return $_('custom.title');
			case 'password': return $_('password.title');  
			case 'api-key': return $_('apiKey.title');
			default: return '';
		}
	}

	function getTranslatedDescription(itemId: string): string {
		switch (itemId) {
			case 'custom': return $_('custom.description');
			case 'password': return $_('password.description');
			case 'api-key': return $_('apiKey.description'); 
			default: return '';
		}
	}
</script>

<svelte:head>
	<title>{$_('menu.title')} - {$_('menu.brandName')}</title>
	<meta name="description" content="{$_('menu.description')}" />
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
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

		<!-- Navigation Cards -->
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto mb-12">
			{#each navigationItems as item}
				<button
					class="w-full bg-white dark:bg-gray-800 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 cursor-pointer border border-gray-200 dark:border-gray-700 {$isRTL ? 'text-right' : 'text-left'}"
					onclick={() => navigateToItem(item.path)}
					aria-label="{$_('common.navigateTo')} {getTranslatedTitle(item.id)}"
				>
					<div class="p-6">
						<div class="mb-4">
							<Iconize conf={{emoji: item.icon, iconSize: "text-3xl", spacing: "gap-3"}}>
								<h2 class="text-xl font-semibold text-gray-900 dark:text-white">
									{getTranslatedTitle(item.id)}
								</h2>
							</Iconize>
						</div>
						<p class="text-gray-600 dark:text-gray-300 leading-relaxed">
							{getTranslatedDescription(item.id)}
						</p>
						<div class="mt-4 inline-flex items-center text-blue-600 dark:text-blue-400 text-sm font-medium {$isRTL ? 'rtl-float-right' : 'rtl-float-left'}">
							{#if $isRTL}
								{$_('common.choose')}
								<Icon name="arrow-left" size="w-4 h-4 ml-1" />
							{:else}
								{$_('common.choose')}
								<Icon name="arrow-right" size="w-4 h-4 ml-1" />
							{/if}
						</div>
						<div class="clear-both"></div>
					</div>
				</button>
			{/each}
		</div>

		<!-- Footer with Version Information -->
		<Footer />
	</div>
</div>
