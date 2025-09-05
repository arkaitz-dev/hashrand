<script lang="ts">
	import { goto } from '$app/navigation';
	import { authStore } from '../stores/auth';
	import { flashMessagesStore } from '../stores/flashMessages';
	import { _ } from '../stores/i18n';
	import { isRTL } from '../stores/rtl';

	// Props
	export let onClose: () => void;

	/**
	 * Handle logout confirmation
	 */
	async function handleLogout() {
		try {
			// Clear auth state (server-side session + local storage)
			await authStore.logout();

			// Add flash message
			flashMessagesStore.addMessage($_('auth.loggedOut'));

			// Close dialog
			onClose();

			// Navigate to home
			goto('/');
		} catch (error) {
			console.error('Logout error:', error);
			// Still close dialog and navigate even if there's an error
			onClose();
			goto('/');
		}
	}
</script>

<!-- Header -->
<div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
	<h2
		class="text-xl font-semibold text-gray-900 dark:text-white {$isRTL
			? 'text-right'
			: 'text-left'}"
	>
		{$_('auth.logoutConfirmTitle')}
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
	<p class="text-gray-600 dark:text-gray-300 text-center">
		{$_('auth.logoutConfirmMessage')}
	</p>
</div>

<!-- Footer Actions -->
<div
	class="flex items-center justify-between px-6 py-4 border-t border-gray-200 dark:border-gray-700 {$isRTL
		? 'flex-row-reverse'
		: ''}"
>
	<button
		onclick={onClose}
		class="px-4 py-2 text-gray-600 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium transition-colors"
	>
		{$_('common.cancel')}
	</button>
	<button
		onclick={handleLogout}
		class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg font-medium transition-colors"
	>
		{$_('auth.logout')}
	</button>
</div>
