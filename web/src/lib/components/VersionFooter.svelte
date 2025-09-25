<!--
	VersionFooter - Intelligent cached version display

	Features:
	- IndexedDB cache with 24-hour expiration
	- Single API call per day maximum
	- Fallback to direct fetch if cache fails
	- Same visual design as original Footer
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { _ } from '$lib/stores/i18n';
	import Icon from './Icon.svelte';
	import type { VersionResponse } from '$lib/types';
	import { getVersionWithCache } from '$lib/version-cache';

	let versions: VersionResponse | null = null;
	let loadingVersion = false;

	onMount(async () => {
		try {
			loadingVersion = true;
			versions = await getVersionWithCache();
		} catch (error) {
			console.error('Failed to load versions:', error);
		} finally {
			loadingVersion = false;
		}
	});
</script>

<!-- Version Information and Footer -->
<div class="text-center mt-8">
	<div class="text-sm text-gray-500 dark:text-gray-400">
		{#if loadingVersion}
			<div class="flex items-center justify-center">
				<div
					class="animate-spin w-4 h-4 border-2 border-gray-400 border-t-transparent rounded-full mr-2"
				></div>
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

	<!-- Made with love -->
	<div
		class="text-xs text-gray-400 dark:text-gray-500 mt-2 flex items-center justify-center force-ltr"
	>
		<span>Made with</span>
		<Icon name="heart" size="w-3 h-3 mx-1 text-red-500" placeholder="auto" />
		<span>by</span>
		<a
			href="https://arkaitz.dev"
			target="_blank"
			rel="noopener noreferrer"
			class="ml-1 text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300 hover:underline"
			>Arkaitz Dev</a
		>
	</div>
</div>
