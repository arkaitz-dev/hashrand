<script lang="ts">
	/**
	 * Login Dialog Component
	 * 
	 * Modal dialog for email-based magic link authentication.
	 * Similar styling to result dialog for UI consistency.
	 */

	import { createEventDispatcher } from 'svelte';
	import { authStore } from '../stores/auth';
	import { t } from '../stores/i18n';
	import type { MagicLinkResponse } from '../types';
	import LoadingSpinner from './LoadingSpinner.svelte';

	// Component props
	export let show = false;

	// Component state
	let email = '';
	let isSubmitting = false;
	let magicLinkSent = false;
	let magicLinkUrl = '';
	let errorMessage = '';

	const dispatch = createEventDispatcher<{
		close: void;
		authenticated: { email: string };
	}>();

	/**
	 * Handle form submission for magic link request
	 */
	async function handleSubmit() {
		if (!email.trim()) {
			errorMessage = $t('auth.emailRequired');
			return;
		}

		if (!isValidEmail(email)) {
			errorMessage = $t('auth.emailInvalid');
			return;
		}

		isSubmitting = true;
		errorMessage = '';

		try {
			const response: MagicLinkResponse = await authStore.requestMagicLink(email);
			
			// Store email in localStorage for magic link validation
			localStorage.setItem('pending_auth_email', email);
			
			magicLinkSent = true;
			
			// In development, show the magic link for easy testing
			if (response.dev_magic_link) {
				magicLinkUrl = response.dev_magic_link;
			}
		} catch (error) {
			errorMessage = error instanceof Error ? error.message : $t('auth.requestFailed');
		} finally {
			isSubmitting = false;
		}
	}

	/**
	 * Test magic link in development mode
	 */
	function testMagicLink() {
		if (magicLinkUrl) {
			// Extract magic token from URL
			const url = new globalThis.URL(magicLinkUrl);
			const magicToken = url.searchParams.get('magiclink');
			
			if (magicToken) {
				// Simulate magic link click by setting URL parameter
				const currentUrl = new globalThis.URL(globalThis.window?.location?.href || '');
				currentUrl.searchParams.set('magiclink', magicToken);
				if (globalThis.window?.location) {
					globalThis.window.location.href = currentUrl.toString();
				}
			}
		}
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
		magicLinkSent = false;
		magicLinkUrl = '';
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
			aria-labelledby="login-title"
		>
			{#if !magicLinkSent}
				<!-- Email input form -->
				<div class="text-center mb-6">
					<h2 id="login-title" class="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2">
						{$t('auth.loginRequired')}
					</h2>
					<p class="text-sm text-gray-600 dark:text-gray-400">
						{$t('auth.loginDescription')}
					</p>
				</div>

				<form on:submit|preventDefault={handleSubmit} class="space-y-4">
					<div>
						<label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
							{$t('auth.emailAddress')}
						</label>
						<input
							id="email"
							type="email"
							bind:value={email}
							placeholder={$t('auth.emailPlaceholder')}
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
							{$t('common.cancel')}
						</button>
						<button
							type="submit"
							disabled={isSubmitting || !email.trim()}
							class="flex-1 px-4 py-2 text-sm font-medium text-white
							       bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400
							       rounded-md transition-colors disabled:cursor-not-allowed
							       flex items-center justify-center gap-2"
						>
							{#if isSubmitting}
								<LoadingSpinner size="sm" />
							{/if}
							{$t('auth.sendMagicLink')}
						</button>
					</div>
				</form>
			{:else}
				<!-- Magic link sent confirmation -->
				<div class="text-center">
					<div class="mb-4">
						<div class="mx-auto w-12 h-12 bg-green-100 dark:bg-green-800 rounded-full flex items-center justify-center">
							<svg class="w-6 h-6 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
							</svg>
						</div>
					</div>

					<h2 class="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-2">
						{$t('auth.magicLinkSent')}
					</h2>
					
					<p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
						{$t('auth.magicLinkInstructions')}
					</p>

					<div class="text-xs text-gray-500 dark:text-gray-500 mb-4">
						{email}
					</div>

					{#if magicLinkUrl}
						<!-- Development mode: show clickable magic link -->
						<div class="bg-yellow-50 dark:bg-yellow-900 border border-yellow-200 dark:border-yellow-700 rounded-md p-3 mb-4">
							<p class="text-xs text-yellow-800 dark:text-yellow-200 mb-2">
								{$t('auth.developmentMode')}
							</p>
							<button
								on:click={testMagicLink}
								class="text-xs text-blue-600 dark:text-blue-400 hover:underline break-all"
							>
								{$t('auth.testMagicLink')}
							</button>
						</div>
					{/if}

					<button
						on:click={closeDialog}
						class="w-full px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300
						       bg-gray-100 dark:bg-gray-600 hover:bg-gray-200 dark:hover:bg-gray-700
						       rounded-md transition-colors"
					>
						{$t('common.close')}
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