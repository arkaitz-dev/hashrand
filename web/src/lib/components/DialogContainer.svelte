<script lang="ts">
	import { dialogStore } from '$lib/stores/dialog';
	import AuthDialogContent from './AuthDialogContent.svelte';
	import AuthConfirmDialogContent from './AuthConfirmDialogContent.svelte';
	import SeedDialogContent from './SeedDialogContent.svelte';
	import LogoutDialogContent from './LogoutDialogContent.svelte';
	import MagicLinkErrorDialogContent from './MagicLinkErrorDialogContent.svelte';

	// Close dialog
	function closeDialog() {
		dialogStore.close();
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
		role="dialog"
		aria-modal="true"
		aria-labelledby="dialog-title"
		onclick={closeDialog}
		onkeydown={(e) => e.key === 'Escape' && closeDialog()}
		tabindex="-1"
	>
		<!-- Dialog Content Container -->
		<div
			class="relative w-full max-w-lg transform rounded-xl bg-white dark:bg-gray-800 shadow-2xl border border-gray-200 dark:border-gray-700 transition-all duration-300"
			role="document"
		>
			<!-- Render different dialog types -->
			{#if $dialogStore.type === 'auth'}
				<AuthDialogContent next={$dialogStore.props} onClose={closeDialog} />
			{:else if $dialogStore.type === 'auth-confirm'}
				<AuthConfirmDialogContent
					email={(($dialogStore.props as any)?.email || '') as string}
					next={$dialogStore.props}
					onClose={closeDialog}
				/>
			{:else if $dialogStore.type === 'seed'}
				<SeedDialogContent
					onClose={closeDialog}
					onSeedChoice={(($dialogStore.props as any)?.onSeedChoice || (() => {})) as (
						keepSeed: boolean
					) => void}
				/>
			{:else if $dialogStore.type === 'logout'}
				<LogoutDialogContent onClose={closeDialog} />
			{:else if $dialogStore.type === 'magic-link-error'}
				<MagicLinkErrorDialogContent onClose={closeDialog} />
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
