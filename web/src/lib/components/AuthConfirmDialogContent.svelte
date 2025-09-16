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
	import LoadingSpinner from './LoadingSpinner.svelte';

	// Props
	export let email: string;
	export let next: Record<string, unknown> | null = null;
	export let onClose: () => void;

	// Component state
	let isSubmitting = false;

	/**
	 * Handle "Es correcto" - send email to API
	 */
	async function handleCorrect() {
		isSubmitting = true;

		try {
			// Convert next parameters to a simple URL string if they exist
			let nextParam: string | undefined = undefined;
			if (next && Object.keys(next).length > 0) {
				// Build URL path from next object
				if (next.endpoint) {
					const params = new URLSearchParams();
					if (next.length) params.set('length', next.length.toString());
					if (next.alphabet) params.set('alphabet', next.alphabet.toString());
					if (next.prefix) params.set('prefix', next.prefix.toString());
					if (next.suffix) params.set('suffix', next.suffix.toString());
					if (next.seed) params.set('seed', next.seed.toString());
					if (next.raw !== undefined) params.set('raw', String(next.raw));
					if (next.language) params.set('language', next.language.toString());
					if (next.words) params.set('words', next.words.toString());

					nextParam = `/result?endpoint=${next.endpoint}&${params.toString()}`;
				}
			}

			// Use new Ed25519-enabled API
			const { api } = await import('$lib/api');
			const ui_host = typeof window !== 'undefined' ? window.location.origin : '';

			console.log('🔍 DEBUG: Attempting requestMagicLink with:', { email, ui_host, nextParam });
			await api.requestMagicLink(email, ui_host, nextParam);

			console.log('✅ Magic link request successful');

			// Close dialog only on success
			onClose();

			// Add success flash message
			flashMessagesStore.addMessage($_('auth.magicLinkSentFlash'));

			// Navigate to home
			goto('/');

		} catch (error) {
			console.error('❌ Error sending magic link:', error);

			// Show error flash message instead of success
			flashMessagesStore.addMessage($_('auth.magicLinkErrorFlash') || 'Error sending magic link. Please try again.');

			// Reset submitting state so user can try again
			isSubmitting = false;
		}
	}

	/**
	 * Handle "Corregir" - go back to auth dialog with prefilled email
	 */
	function handleEdit() {
		// Show auth dialog with prefilled email and next parameters
		// This will automatically replace the current dialog
		const authProps = next ? { email, ...next } : { email };
		dialogStore.show('auth', authProps);
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
				{email}
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
