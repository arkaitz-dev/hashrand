<script lang="ts">
	/**
	 * Auth Dialog Content - SIMPLIFIED VERSION
	 * 
	 * Input email -> Send immediately -> Close dialog -> Show result
	 */
	
	import { _ } from '../stores/i18n';
	import { isRTL } from '../stores/rtl';
	import LoadingSpinner from './LoadingSpinner.svelte';
	
	// Props
	export let onClose: () => void;
	export let onMagicLinkSent: () => void;
	export let next: Record<string, unknown> | null = null;
	
	// Component state
	let email = '';
	let isSubmitting = false;
	let debugMessage = next ? `📝 Props: ${JSON.stringify(next)}` : '📝 No next props received';
	
	/**
	 * Handle form submission - send immediately
	 */
	async function handleSubmit() {
		if (!email.trim()) {
			debugMessage = '❌ Email required';
			return;
		}

		if (!isValidEmail(email)) {
			debugMessage = '❌ Invalid email';
			return;
		}

		debugMessage = '🔄 Sending...';
		isSubmitting = true;
		
		// Close dialog immediately
		onMagicLinkSent();
		
		// Send API call in background
		setTimeout(async () => {
			try {
				debugMessage = '🔄 Calling API...';
				
				// Debug next parameters
				debugMessage = `🔍 next=${JSON.stringify(next)}, keys=${next ? Object.keys(next).length : 0}`;
				
				// Encode next parameters as Base58 if they exist
				let nextParam: string | undefined = undefined;
				if (next && Object.keys(next).length > 0) {
					const jsonString = JSON.stringify(next);
					const encoder = new TextEncoder();
					const bytes = encoder.encode(jsonString);
					// Import base58 encoding
					const { base58 } = await import('@scure/base');
					nextParam = base58.encode(bytes);
					debugMessage = `🔄 Sending next (Base58): ${nextParam}`;
				} else {
					debugMessage = `🔄 No next params to send`;
				}
				
				const response = await fetch('/api/login/', {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({
						email,
						ui_host: typeof window !== 'undefined' ? window.location.origin : undefined,
						next: nextParam
					})
				});
				debugMessage = `🔄 Response: ${response.status}`;
				const data = await response.json();
				debugMessage = `✅ Success: ${JSON.stringify(data)}`;
			} catch (error) {
				debugMessage = `❌ Error: ${error?.message}`;
			}
		}, 100);
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
	<h2 class="text-xl font-semibold text-gray-900 dark:text-white {$isRTL ? 'text-right' : 'text-left'}">
		{$_('auth.loginRequired')}
	</h2>
	<button
		onclick={onClose}
		class="p-1 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
		aria-label={$_('common.close')}
	>
		<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
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
				disabled={isSubmitting}
				class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md
				       bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
				       focus:ring-2 focus:ring-blue-500 focus:border-transparent
				       disabled:opacity-50 disabled:cursor-not-allowed"
				onkeydown={(e) => e.key === 'Enter' && handleSubmit()}
			/>
			
			{#if debugMessage}
				<p class="text-blue-600 dark:text-blue-400 text-sm mt-1">
					{debugMessage}
				</p>
			{/if}
		</div>
	</div>
</div>

<!-- Footer Actions -->
<div class="flex items-center justify-end gap-3 px-6 py-4 border-t border-gray-200 dark:border-gray-700 {$isRTL ? 'flex-row-reverse' : ''}">
	<button
		onclick={onClose}
		disabled={isSubmitting}
		class="px-4 py-2 text-gray-600 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
	>
		{$_('common.cancel')}
	</button>
	<button
		onclick={handleSubmit}
		disabled={isSubmitting || !email.trim()}
		class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white rounded-lg font-medium transition-colors disabled:cursor-not-allowed flex items-center gap-2"
	>
		{#if isSubmitting}
			<LoadingSpinner size="sm" />
		{/if}
		{$_('auth.sendMagicLink')}
	</button>
</div>