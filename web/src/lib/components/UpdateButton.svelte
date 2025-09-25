<!--
	UpdateButton - Version upgrade notification button

	Features:
	- Appears only when frontend version differs from cached
	- Positioned in opposite corner from TopControls
	- Animated yellow button for high visibility
	- RTL-aware positioning
	- Handles complete frontend reload with session backup
-->
<script lang="ts">
	import { updateAvailable, acceptUpdate } from '$lib/stores/version-update';
	import { isRTL } from '$lib/stores/rtl';
	import { _ } from '$lib/stores/i18n';

	let isProcessing = $state(false);

	async function handleUpdate() {
		if (isProcessing) return;

		try {
			isProcessing = true;
			await acceptUpdate();
		} catch (error) {
			console.error('Update failed:', error);
			isProcessing = false;
		}
		// Note: If successful, page will reload so isProcessing reset is not needed
	}
</script>

<!-- Update Button - Only visible when update available -->
{#if $updateAvailable}
	<div
		class="absolute top-0.5 md:top-4 z-40 transition-opacity duration-300 {$isRTL
			? 'right-0.5 md:right-4'
			: 'left-0.5 md:left-4'}"
	>
		<button
			class="px-2 sm:px-3 py-1 sm:py-1.5 text-xs sm:text-sm font-medium bg-yellow-400 hover:bg-yellow-500 text-yellow-900 rounded-lg sm:rounded-xl shadow-lg border border-yellow-500 transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-yellow-300 disabled:opacity-50 disabled:cursor-not-allowed animate-pulse-yellow"
			class:cursor-not-allowed={isProcessing}
			class:opacity-50={isProcessing}
			disabled={isProcessing}
			onclick={handleUpdate}
			aria-label={$_('common.updateAvailable')}
			title={$_('common.updateAvailable')}
		>
			{#if isProcessing}
				<div class="flex items-center gap-1">
					<div
						class="w-3 h-3 border-2 border-yellow-900 border-t-transparent rounded-full animate-spin"
					></div>
					<span>{$_('common.updating')}...</span>
				</div>
			{:else}
				{$_('common.update')}
			{/if}
		</button>
	</div>
{/if}

<style>
	/* Custom yellow pulse animation cycling through yellow tones */
	@keyframes pulse-yellow {
		0%,
		100% {
			background-color: rgb(250 204 21); /* yellow-400 */
			border-color: rgb(234 179 8); /* yellow-500 */
		}
		25% {
			background-color: rgb(253 224 71); /* yellow-300 */
			border-color: rgb(250 204 21); /* yellow-400 */
		}
		50% {
			background-color: rgb(254 240 138); /* yellow-200 */
			border-color: rgb(253 224 71); /* yellow-300 */
		}
		75% {
			background-color: rgb(253 224 71); /* yellow-300 */
			border-color: rgb(250 204 21); /* yellow-400 */
		}
	}

	.animate-pulse-yellow {
		animation: pulse-yellow 2s ease-in-out infinite;
	}

	/* Hover overrides animation for better UX feedback */
	.animate-pulse-yellow:hover {
		animation: none;
		background-color: rgb(234 179 8) !important; /* yellow-500 */
		border-color: rgb(161 98 7) !important; /* yellow-600 */
	}
</style>
