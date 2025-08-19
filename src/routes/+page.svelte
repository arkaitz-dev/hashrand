<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { navigationItems } from '$lib/stores/navigation';
	import { t } from '$lib/stores/i18n';
	import type { VersionResponse } from '$lib/types';

	let version: VersionResponse | null = null;
	let loadingVersion = false;

	onMount(async () => {
		// Load version info from API
		try {
			loadingVersion = true;
			const { api } = await import('$lib/api');
			version = await api.getVersion();
		} catch (error) {
			console.error('Failed to load version:', error);
		} finally {
			loadingVersion = false;
		}
	});

	function navigateToItem(path: string) {
		goto(path);
	}
</script>

<svelte:head>
	<title>Hash Generator - Professional Random Generation Tool</title>
	<meta name="description" content="Professional hash, password, and API key generator with customizable parameters" />
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
	<div class="container mx-auto px-4 py-8">
		<!-- Header -->
		<header class="text-center mb-12">
			<div class="inline-flex items-center justify-center w-16 h-16 bg-blue-600 rounded-full mb-6">
				<span class="text-2xl text-white">🎲</span>
			</div>
			<h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
				{t('menu.title')}
			</h1>
			<p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
				{t('menu.subtitle')}
			</p>
		</header>

		<!-- Navigation Cards -->
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto mb-12">
			{#each navigationItems as item}
				<button
					class="w-full bg-white dark:bg-gray-800 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 cursor-pointer border border-gray-200 dark:border-gray-700 text-left"
					onclick={() => navigateToItem(item.path)}
					aria-label="Navigate to {item.title}"
				>
					<div class="p-6">
						<div class="flex items-center mb-4">
							<span class="text-3xl mr-3">{item.icon}</span>
							<h2 class="text-xl font-semibold text-gray-900 dark:text-white">
								{item.title}
							</h2>
						</div>
						<p class="text-gray-600 dark:text-gray-300 leading-relaxed">
							{item.description}
						</p>
						<div class="mt-4 inline-flex items-center text-blue-600 dark:text-blue-400 text-sm font-medium">
							Choose
							<svg class="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
							</svg>
						</div>
					</div>
				</button>
			{/each}
		</div>

		<!-- Version Information -->
		<footer class="text-center">
			<div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-4 max-w-md mx-auto border border-gray-200 dark:border-gray-700">
				{#if loadingVersion}
					<div class="flex items-center justify-center">
						<div class="animate-spin w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full mr-2"></div>
						<span class="text-gray-600 dark:text-gray-300 text-sm">Loading version...</span>
					</div>
				{:else if version}
					<div class="text-sm text-gray-600 dark:text-gray-300">
						<strong class="text-gray-900 dark:text-white">{version.name}</strong>
						<span class="mx-2">•</span>
						{t('menu.version')} {version.version}
					</div>
					<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
						{version.description}
					</p>
				{:else}
					<div class="text-sm text-gray-500 dark:text-gray-400">
						API connection unavailable
					</div>
				{/if}
			</div>
		</footer>
	</div>
</div>
