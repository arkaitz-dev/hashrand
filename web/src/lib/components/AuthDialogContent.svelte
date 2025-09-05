<script lang="ts">
	/**
	 * Auth Dialog Content
	 *
	 * Input email -> Show confirmation dialog
	 */

	import { _ } from '../stores/i18n';
	import { isRTL } from '../stores/rtl';
	import { dialogStore } from '../stores/dialog';

	// Props
	export let onClose: () => void;
	export let next: Record<string, unknown> | null = null;

	// Component state  
	let email = (next?.email as string) || '';

	/**
	 * Handle form submission - show confirmation dialog
	 */
	async function handleSubmit() {
		if (!email.trim()) {
			return;
		}

		if (!isValidEmail(email)) {
			return;
		}

		// Filter out email from next parameters
		const nextWithoutEmail = next ? { ...next } : {};
		if (nextWithoutEmail.email) {
			delete nextWithoutEmail.email;
		}

		// Show confirmation dialog with email and next parameters
		// This will automatically replace the current dialog
		dialogStore.show('auth-confirm', { 
			email, 
			...(Object.keys(nextWithoutEmail).length > 0 ? nextWithoutEmail : {})
		});
	}

	/**
	 * Basic email validation
	 */
	function isValidEmail(email: string): boolean {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		return emailRegex.test(email);
	}
</script>

<!-- Header -->
<div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
	<h2
		class="text-xl font-semibold text-gray-900 dark:text-white {$isRTL
			? 'text-right'
			: 'text-left'}"
	>
		{$_('auth.loginRequired')}
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
			{$_('auth.loginDescription')}
		</p>

		<div>
			<label
				for="auth-email"
				class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
			>
				{$_('auth.emailAddress')}
			</label>
			<input
				id="auth-email"
				type="email"
				bind:value={email}
				placeholder={$_('auth.emailPlaceholder')}
				class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md
				       bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
				       focus:ring-2 focus:ring-blue-500 focus:border-transparent"
				onkeydown={(e) => e.key === 'Enter' && handleSubmit()}
			/>

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
		onclick={onClose}
		class="px-4 py-2 text-gray-600 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium transition-colors"
	>
		{$_('common.cancel')}
	</button>
	<button
		onclick={handleSubmit}
		disabled={!email.trim()}
		class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white rounded-lg font-medium transition-colors disabled:cursor-not-allowed"
	>
		{$_('common.continue')}
	</button>
</div>
