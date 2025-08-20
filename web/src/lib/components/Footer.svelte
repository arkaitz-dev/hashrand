<script lang="ts">
	import { onMount } from 'svelte';
	import { _ } from '$lib/stores/i18n';
	import type { VersionResponse } from '$lib/types';

	let versions: VersionResponse | null = null;
	let loadingVersion = false;

	onMount(async () => {
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
</script>

<footer class="mt-8 text-center py-4 bg-white/60 dark:bg-gray-900/60 backdrop-blur-sm border-t border-gray-200/50 dark:border-gray-700/50">
	<div class="text-xs text-gray-400 dark:text-gray-500">
		{#if loadingVersion}
			<div class="flex items-center justify-center">
				<div class="animate-spin w-3 h-3 border border-gray-400 border-t-transparent rounded-full mr-1"></div>
				<span>{$_('common.loadingVersion')}...</span>
			</div>
		{:else if versions}
			<span class="text-gray-500 dark:text-gray-400">{$_('menu.brandName')}</span>
			<span class="mx-1">•</span>
			<span>UI v{versions.ui_version} / API v{versions.api_version}</span>
		{:else}
			<span>{$_('common.versionsUnavailable')}</span>
		{/if}
	</div>
</footer>