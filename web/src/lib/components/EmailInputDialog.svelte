<script lang="ts">
	/**
	 * Email Input Dialog Component
	 *
	 * Reusable modal dialog for email input with two-step flow:
	 * 1. Email input step
	 * 2. Email confirmation step
	 *
	 * Follows the same styling patterns as LoginDialog for consistency.
	 */

	import { createEventDispatcher } from 'svelte';
	import { _ } from '../stores/i18n';
	import LoadingSpinner from './LoadingSpinner.svelte';
	import { base58 } from '@scure/base';

	// Component props
	export let show = false;
	export let next: Record<string, unknown> | null = null; // JSON object with parameters to preserve after login
	export let title = '';
	export let description = '';
	export let emailPlaceholder = '';
	export let confirmTitle = '';
	export let confirmDescription = '';
	export let cancelText = '';
	export let continueText = '';
	export let correctText = '';
	export let sendText = '';
	export let sendingText = '';

	// Component state
	let email = '';
	let isSubmitting = false;
	let showEmailConfirmation = false;
	let errorMessage = '';

	const dispatch = createEventDispatcher<{
		close: void;
		emailSubmitted: { email: string };
		emailConfirmed: { email: string; redirectUrl: string };
	}>();

	/**
	 * Handle initial email input step
	 */
	function handleInitialSubmit() {
		if (!email || !email.includes('@')) {
			errorMessage = $_('auth.emailInvalid');
			return;
		}

		errorMessage = '';
		showEmailConfirmation = true;
		dispatch('emailSubmitted', { email });
	}

	/**
	 * Handle email correction - go back to input step
	 */
	function handleCorrectEmail() {
		showEmailConfirmation = false;
		errorMessage = '';
	}

	/**
	 * Convert JSON object to base58 encoded string
	 */
	function encodeNextToBase58(nextObj: Record<string, unknown> | null): string | null {
		if (!nextObj) return null;

		try {
			// Convert JSON object to string
			const jsonString = JSON.stringify(nextObj);

			// Convert string to bytes (Uint8Array)
			const encoder = new globalThis.TextEncoder();
			const bytes = encoder.encode(jsonString);

			// Encode bytes to base58
			const base58String = base58.encode(bytes);

			return base58String;
		} catch (error) {
			console.error('Error encoding next parameter to base58:', error);
			return null;
		}
	}

	/**
	 * Handle email confirmation and final submission
	 */
	function handleConfirmAndSend() {
		isSubmitting = true;

		// No longer including next in redirectUrl since it will be sent via POST
		const redirectUrl = '/';

		dispatch('emailConfirmed', { email, redirectUrl });
	}

	/**
	 * Basic email validation
	 */
	function isValidEmail(email: string): boolean {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		return emailRegex.test(email);
	}

	/**
	 * Close dialog and reset state
	 */
	function closeDialog() {
		show = false;
		email = '';
		showEmailConfirmation = false;
		isSubmitting = false;
		errorMessage = '';
		dispatch('close');
	}

	/**
	 * Handle click outside dialog to close
	 */
	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			closeDialog();
		}
	}

	/**
	 * Handle escape key to close dialog
	 */
	function handleKeydown(event: globalThis.KeyboardEvent) {
		if (event.key === 'Escape') {
			closeDialog();
		}
	}

	// Reactive validation
	$: emailValid = email.trim() && isValidEmail(email);

	// Reset isSubmitting when show changes to false (external control)
	$: if (!show) {
		isSubmitting = false;
	}

	// Export methods for parent component control
	export function resetSubmitting() {
		isSubmitting = false;
	}

	export function setError(message: string) {
		errorMessage = message;
		isSubmitting = false;
	}
</script>

<!-- Dialog backdrop -->
{#if show}
	<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
	<div
		class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
		on:click={handleBackdropClick}
	>
		<!-- Dialog content -->
		<div
			class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-md w-full p-6"
			role="dialog"
			aria-modal="true"
			aria-labelledby="email-dialog-title"
		>
			{#if !showEmailConfirmation}
				<!-- Step 1: Email Input -->
				<div class="text-center mb-6">
					<h2
						id="email-dialog-title"
						class="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2"
					>
						{title || $_('auth.loginRequired')}
					</h2>
					<p class="text-sm text-gray-600 dark:text-gray-400">
						{description || $_('auth.loginDescription')}
					</p>
				</div>

				<form on:submit|preventDefault={handleInitialSubmit} class="space-y-4">
					<div>
						<label
							for="email-input"
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
						>
							{$_('auth.emailAddress')}
						</label>
						<input
							id="email-input"
							type="email"
							bind:value={email}
							placeholder={emailPlaceholder || $_('auth.emailPlaceholder')}
							required
							disabled={isSubmitting}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md
							       bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
							       focus:ring-2 focus:ring-blue-500 focus:border-transparent
							       disabled:opacity-50 disabled:cursor-not-allowed"
						/>
					</div>

					{#if errorMessage}
						<div class="text-red-600 dark:text-red-400 text-sm" role="alert">
							{errorMessage}
						</div>
					{/if}

					<div class="flex gap-3 pt-2">
						<button
							type="button"
							on:click={closeDialog}
							disabled={isSubmitting}
							class="flex-1 px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300
							       bg-gray-100 dark:bg-gray-600 hover:bg-gray-200 dark:hover:bg-gray-700
							       rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
						>
							{cancelText || $_('common.cancel')}
						</button>
						<button
							type="submit"
							disabled={isSubmitting || !emailValid}
							class="flex-1 px-4 py-2 text-sm font-medium text-white
							       bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400
							       rounded-md transition-colors disabled:cursor-not-allowed
							       flex items-center justify-center gap-2"
						>
							{#if isSubmitting}
								<LoadingSpinner size="sm" />
							{/if}
							{continueText || $_('common.continue')}
						</button>
					</div>
				</form>
			{:else}
				<!-- Step 2: Email Confirmation -->
				<div class="text-center mb-6">
					<h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2">
						{confirmTitle || $_('auth.confirmEmail')}
					</h2>
					<p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
						{confirmDescription || $_('auth.confirmEmailDescription')}
					</p>

					<div
						class="bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg p-3 mb-4"
					>
						<p class="text-gray-900 dark:text-gray-100 font-medium">
							{email}
						</p>
					</div>
				</div>

				{#if errorMessage}
					<div class="text-red-600 dark:text-red-400 text-sm mb-4" role="alert">
						{errorMessage}
					</div>
				{/if}

				<div class="flex gap-3">
					<button
						type="button"
						on:click={handleCorrectEmail}
						disabled={isSubmitting}
						class="flex-1 px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300
						       bg-gray-100 dark:bg-gray-600 hover:bg-gray-200 dark:hover:bg-gray-700
						       rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{correctText || $_('common.correct')}
					</button>
					<button
						type="button"
						on:click={handleConfirmAndSend}
						disabled={isSubmitting}
						class="flex-1 px-4 py-2 text-sm font-medium text-white
						       bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400
						       rounded-md transition-colors disabled:cursor-not-allowed
						       flex items-center justify-center gap-2"
					>
						{#if isSubmitting}
							<LoadingSpinner size="sm" />
						{/if}
						{isSubmitting ? sendingText || $_('common.sending') : sendText || $_('common.send')}
					</button>
				</div>
			{/if}
		</div>
	</div>
{/if}

<!-- Global keydown listener -->
<svelte:window on:keydown={show ? handleKeydown : undefined} />

<style>
	/* Ensure dialog appears above other elements */
	:global(.fixed) {
		z-index: 9999;
	}
</style>
