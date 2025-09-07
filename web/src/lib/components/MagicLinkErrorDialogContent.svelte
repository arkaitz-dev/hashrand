<script lang="ts">
	/**
	 * Magic Link Error Dialog Content
	 *
	 * Shows error when magic link is opened in a different browser
	 * than where the request originated from
	 */

	import { _ } from '../stores/i18n';
	import { isRTL } from '../stores/rtl';

	// Props
	export let onClose: () => void;
</script>

<!-- Header -->
<div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
	<h2
		class="text-xl font-semibold text-red-700 dark:text-red-400 {$isRTL
			? 'text-right'
			: 'text-left'}"
	>
		{$_('auth.magicLinkError.title')}
	</h2>
	<button
		onclick={onClose}
		class="p-1 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
		aria-label={$_('common.close')}
	>
		<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"
			></path>
		</svg>
	</button>
</div>

<!-- Body Content -->
<div class="p-6">
	<div class="space-y-4">
		<!-- Error Icon -->
		<div class="flex justify-center">
			<div
				class="w-16 h-16 bg-red-100 dark:bg-red-900/20 rounded-full flex items-center justify-center"
			>
				<svg
					class="w-8 h-8 text-red-600 dark:text-red-400"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.732-.833-2.5 0L4.732 18.5c-.77.833.192 2.5 1.732 2.5z"
					></path>
				</svg>
			</div>
		</div>

		<!-- Error Message -->
		<div class="text-center space-y-3">
			<p class="text-lg font-medium text-gray-900 dark:text-gray-100">
				{$_('auth.magicLinkError.message')}
			</p>
			<p class="text-sm text-gray-600 dark:text-gray-400">
				{$_('auth.magicLinkError.explanation')}
			</p>
		</div>

		<!-- Security Note -->
		<div
			class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4"
		>
			<div class="flex {$isRTL ? 'flex-row-reverse' : ''}">
				<svg
					class="w-5 h-5 text-blue-600 dark:text-blue-400 {$isRTL ? 'ml-3' : 'mr-3'} mt-0.5"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
					></path>
				</svg>
				<div>
					<h4
						class="text-sm font-medium text-blue-800 dark:text-blue-300 {$isRTL
							? 'text-right'
							: 'text-left'}"
					>
						{$_('auth.magicLinkError.securityTitle')}
					</h4>
					<p
						class="mt-1 text-sm text-blue-700 dark:text-blue-400 {$isRTL
							? 'text-right'
							: 'text-left'}"
					>
						{$_('auth.magicLinkError.securityExplanation')}
					</p>
				</div>
			</div>
		</div>
	</div>
</div>

<!-- Footer Actions -->
<div
	class="flex items-center justify-end px-6 py-4 border-t border-gray-200 dark:border-gray-700 {$isRTL
		? 'flex-row-reverse'
		: ''}"
>
	<button
		onclick={onClose}
		class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white rounded-lg font-medium transition-colors"
	>
		{$_('common.understand')}
	</button>
</div>
