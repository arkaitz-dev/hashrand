<script lang="ts">
	/**
	 * Auth Confirmation Dialog Content
	 *
	 * Shows email for confirmation before sending magic link
	 */

	import { _ } from '../stores/i18n';
	import { isRTL } from '../stores/rtl';
	import { dialogStore } from '../stores/dialog';
	import { flashMessagesStore } from '../stores/flashMessages';
	import { goto } from '$app/navigation';
	import { buildNextParameterFromConfig, type AuthDialogConfig } from '$lib/utils/navigation';
	import LoadingSpinner from './LoadingSpinner.svelte';

	// Props - Universal architecture
	export let config: AuthDialogConfig;
	export let onClose: () => void;

	// Component state
	let isSubmitting = false;

	/**
	 * Handle "Es correcto" - send email to API
	 */
	async function handleCorrect() {
		isSubmitting = true;

		try {
			// Use new Ed25519-enabled API
			const { api } = await import('$lib/api');
			const ui_host = typeof window !== 'undefined' ? window.location.origin : '';

			// Build nextParam using universal DRY function
			const nextParam = buildNextParameterFromConfig(config);

			await api.requestMagicLink(config.email || '', ui_host, nextParam);

			// Add success flash message
			flashMessagesStore.addMessage($_('auth.magicLinkSentFlash'));
		} catch {
			// Error sending magic link
		} finally {
			// ALWAYS close dialog and navigate to / regardless of success or error
			onClose();
			goto('/');
		}
	}

	/**
	 * Handle "Corregir" - go back to auth dialog with prefilled email
	 */
	function handleEdit() {
		// Show auth dialog with same configuration but allow email editing
		// This will automatically replace the current dialog
		const editConfig: AuthDialogConfig = {
			// Don't prefill email - let user edit it
			destination: config.destination
		};
		dialogStore.show('auth', editConfig as unknown as Record<string, unknown>);
	}
</script>

<!-- Header -->
<div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
	<h2
		class="text-xl font-semibold text-gray-900 dark:text-white {$isRTL
			? 'text-right'
			: 'text-left'}"
	>
		{$_('auth.confirmEmail')}
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
		<p class="text-sm text-gray-600 dark:text-gray-400">
			{$_('auth.confirmEmailDescription')}
		</p>

		<div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4">
			<p class="text-lg font-medium text-gray-900 dark:text-gray-100 text-center">
				{config.email || ''}
			</p>
		</div>
	</div>
</div>

<!-- Footer Actions -->
<div
	class="flex items-center justify-between px-6 py-4 border-t border-gray-200 dark:border-gray-700 {$isRTL
		? 'flex-row-reverse'
		: ''}"
>
	<button
		onclick={handleEdit}
		disabled={isSubmitting}
		class="px-4 py-2 text-gray-600 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
	>
		{$_('common.correct')}
	</button>
	<button
		onclick={handleCorrect}
		disabled={isSubmitting}
		class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white rounded-lg font-medium transition-colors disabled:cursor-not-allowed flex items-center gap-2"
	>
		{#if isSubmitting}
			<LoadingSpinner size="sm" />
		{/if}
		{$_('auth.isCorrect')}
	</button>
</div>
