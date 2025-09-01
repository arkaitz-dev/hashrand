<script lang="ts">
	import { dialogStore } from '$lib/stores/dialog';
	import { flashMessagesStore } from '$lib/stores/flashMessages';
	import { goto } from '$app/navigation';
	import { _ } from '$lib/stores/i18n';
	import AuthDialogContent from './AuthDialogContent.svelte';
	import SeedDialogContent from './SeedDialogContent.svelte';
	import LogoutDialogContent from './LogoutDialogContent.svelte';
	
	
	// Close dialog
	function closeDialog() {
		dialogStore.close();
	}
	
	// Handle magic link sent successfully
	function handleMagicLinkSent() {
		// Close dialog
		dialogStore.close();
		
		// Add flash message
		flashMessagesStore.addMessage($_('auth.magicLinkSentFlash'));
		
		// Navigate to home
		goto('/');
	}
	
	// Handle escape key
	function handleKeydown(event: globalThis.KeyboardEvent) {
		if (event.key === 'Escape' && $dialogStore) {
			event.preventDefault();
			closeDialog();
		}
	}
</script>

<svelte:window onkeydown={(event) => handleKeydown(event)} />

{#if $dialogStore}
	<!-- Backdrop with blur -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
		onclick={closeDialog}
	>
		<!-- Dialog Content Container -->
		<div
			class="relative w-full max-w-lg transform rounded-xl bg-white dark:bg-gray-800 shadow-2xl border border-gray-200 dark:border-gray-700 transition-all duration-300"
			onclick={(e) => e.stopPropagation()}
		>
			<!-- Render different dialog types -->
			{#if $dialogStore.type === 'auth'}
				<AuthDialogContent 
					next={$dialogStore.props}
					onClose={closeDialog}
					onMagicLinkSent={handleMagicLinkSent}
				/>
			{:else if $dialogStore.type === 'seed'}
				<SeedDialogContent 
					onClose={closeDialog}
					onSeedChoice={$dialogStore.props?.onSeedChoice || (() => {})}
				/>
			{:else if $dialogStore.type === 'logout'}
				<LogoutDialogContent 
					onClose={closeDialog}
				/>
			{:else}
				<!-- Fallback for unknown dialog types -->
				<div class="p-6">
					<h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-4">
						Unknown Dialog Type: {$dialogStore.type}
					</h2>
					<button
						onclick={closeDialog}
						class="px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg font-medium transition-colors"
					>
						Close
					</button>
				</div>
			{/if}
		</div>
	</div>
{/if}