<script lang="ts">
	import { goto } from '$app/navigation';
	import { _ } from '$lib/stores/i18n';
	import { clearResult } from '$lib/stores/result';
	import Iconize from './Iconize.svelte';
	import { logger } from '$lib/utils/logger';

	// Props
	export let buttonClass: string =
		'flex-1 bg-gray-600 hover:bg-gray-700 text-white px-6 py-4 rounded-lg font-semibold border-none cursor-pointer hover:shadow-lg transition-all duration-200 flex items-center justify-center gap-2';

	// Handle back to menu with state reset
	function handleBackToMenu() {
		logger.info('[Click] Back to menu button');
		logger.info('[Navigation] Redirecting to: /');
		// Clear the result state to reset application state
		clearResult();
		// Navigate to home
		goto('/');
	}
</script>

<button type="button" onclick={handleBackToMenu} class={buttonClass}>
	<Iconize
		conf={{
			icon: 'home',
			iconSize: 'w-5 h-5'
		}}
	>
		{$_('common.backToMenu')}
	</Iconize>
</button>
