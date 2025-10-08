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
	import { logger } from '$lib/utils/logger';

	let isProcessing = $state(false);

	async function handleUpdate() {
		if (isProcessing) return;

		try {
			isProcessing = true;
			await acceptUpdate();
		} catch (error) {
			logger.error('Update failed:', error);
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
			class="px-2 sm:px-3 py-1 sm:py-1.5 text-xs sm:text-sm font-medium text-white rounded-lg sm:rounded-xl border transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-yellow-300 disabled:opacity-50 disabled:cursor-not-allowed yellow-pulse-animation"
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
						class="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin"
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
	/* Advanced yellow pulsing animation for update notification - Same as AuthStatus expired animation */
	@keyframes yellowPulse {
		0% {
			background-color: #713f12; /* yellow-900 */
			border-color: #a16207; /* yellow-800 */
			box-shadow: 0 0 10px rgba(113, 63, 18, 0.8);
		}
		10% {
			background-color: #a16207; /* yellow-800 */
			border-color: #ca8a04; /* yellow-700 */
			box-shadow: 0 0 12px rgba(161, 98, 7, 0.8);
		}
		20% {
			background-color: #ca8a04; /* yellow-700 */
			border-color: #d97706; /* yellow-600 */
			box-shadow: 0 0 14px rgba(202, 138, 4, 0.8);
		}
		30% {
			background-color: #d97706; /* yellow-600 */
			border-color: #f59e0b; /* yellow-500 */
			box-shadow: 0 0 16px rgba(217, 119, 6, 0.8);
		}
		40% {
			background-color: #f59e0b; /* yellow-500 */
			border-color: #fbbf24; /* yellow-400 */
			box-shadow: 0 0 18px rgba(245, 158, 11, 0.8);
		}
		45% {
			background-color: #fbbf24; /* yellow-400 */
			border-color: #fcd34d; /* yellow-300 */
			box-shadow: 0 0 20px rgba(251, 191, 36, 0.9);
		}
		50% {
			background-color: #fcd34d; /* yellow-300 - punto más claro */
			border-color: #fde68a; /* yellow-200 */
			box-shadow: 0 0 25px rgba(252, 211, 77, 1);
		}
		55% {
			background-color: #fbbf24; /* yellow-400 */
			border-color: #fcd34d; /* yellow-300 */
			box-shadow: 0 0 20px rgba(251, 191, 36, 0.9);
		}
		60% {
			background-color: #f59e0b; /* yellow-500 */
			border-color: #fbbf24; /* yellow-400 */
			box-shadow: 0 0 18px rgba(245, 158, 11, 0.8);
		}
		70% {
			background-color: #d97706; /* yellow-600 */
			border-color: #f59e0b; /* yellow-500 */
			box-shadow: 0 0 16px rgba(217, 119, 6, 0.8);
		}
		80% {
			background-color: #ca8a04; /* yellow-700 */
			border-color: #d97706; /* yellow-600 */
			box-shadow: 0 0 14px rgba(202, 138, 4, 0.8);
		}
		90% {
			background-color: #a16207; /* yellow-800 */
			border-color: #ca8a04; /* yellow-700 */
			box-shadow: 0 0 12px rgba(161, 98, 7, 0.8);
		}
		100% {
			background-color: #713f12; /* yellow-900 - vuelta al inicio */
			border-color: #a16207; /* yellow-800 */
			box-shadow: 0 0 10px rgba(113, 63, 18, 0.8);
		}
	}

	.yellow-pulse-animation {
		animation: yellowPulse 1.5s ease-in-out infinite;
	}

	/* Hover overrides animation for better UX feedback */
	.yellow-pulse-animation:hover {
		animation: none;
		background-color: #d97706 !important; /* yellow-600 */
		border-color: #ca8a04 !important; /* yellow-700 */
		box-shadow: 0 0 15px rgba(217, 119, 6, 0.9) !important;
	}
</style>
