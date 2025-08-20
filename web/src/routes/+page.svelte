<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { navigationItems } from '$lib/stores/navigation';
	import { clearResult } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import { isRTL } from '$lib/stores/rtl';
	import Icon from '$lib/components/Icon.svelte';
	import type { VersionResponse } from '$lib/types';

	let versions: VersionResponse | null = null;
	let loadingVersion = false;

	onMount(async () => {
		// Clear result state when returning to menu - this resets all form values to defaults
		clearResult();
		
		// Load version info from API
		try {
			loadingVersion = true;
			const { api } = await import('$lib/api');
			versions = await api.getVersion();
		} catch (error) {
			console.error('Failed to load versions:', error);
		} finally {
			loadingVersion = false;
		}
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
						<div class="flex items-center mb-4">
							{#if $isRTL}
								<h2 class="text-xl font-semibold text-gray-900 dark:text-white ml-3">
									{getTranslatedTitle(item.id)}
								</h2>
								<span class="text-3xl">{item.icon}</span>
							{:else}
								<span class="text-3xl mr-3">{item.icon}</span>
								<h2 class="text-xl font-semibold text-gray-900 dark:text-white">
									{getTranslatedTitle(item.id)}
								</h2>
							{/if}
						</div>
						<p class="text-gray-600 dark:text-gray-300 leading-relaxed">
							{getTranslatedDescription(item.id)}
						</p>
						<div class="mt-4 inline-flex items-center text-blue-600 dark:text-blue-400 text-sm font-medium {$isRTL ? 'rtl-float-right' : 'rtl-float-left'}">
							{#if $isRTL}
								{$_('common.choose')}
								<Icon name="arrow-left" size="w-4 h-4 mr-1" />
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

		<!-- Version Information -->
		<div class="text-center mt-8">
			<div class="text-sm text-gray-500 dark:text-gray-400">
				{#if loadingVersion}
					<div class="flex items-center justify-center">
						<div class="animate-spin w-4 h-4 border-2 border-gray-400 border-t-transparent rounded-full mr-2"></div>
						<span>{$_('common.loadingVersion')}...</span>
					</div>
				{:else if versions}
					<span class="text-gray-600 dark:text-gray-300">{$_('menu.brandName')}</span>
					<span class="mx-2 text-gray-400">•</span>
					<span>UI v{versions.ui_version} / API v{versions.api_version}</span>
				{:else}
					<span>{$_('common.versionsUnavailable')}</span>
				{/if}
			</div>
			<div class="text-xs text-gray-400 dark:text-gray-500 mt-2 flex items-center justify-center">
				<span>Made with</span>
				<Icon name="heart" size="w-3 h-3 mx-1 text-red-500" />
				<span>by</span>
				<a href="https://arkaitz.dev" target="_blank" rel="noopener noreferrer" class="ml-1 text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300 hover:underline">Arkaitz Dev</a>
			</div>
		</div>
	</div>
</div>
