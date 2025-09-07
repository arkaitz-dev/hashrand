<script lang="ts">
	/**
	 * Example Complex Dialog Component
	 *
	 * Demonstrates how to inject arbitrary components into SimpleDialog:
	 * - Custom forms with validation
	 * - Multi-step workflows
	 * - Dynamic content
	 * - Complex interactions
	 */

	import { createEventDispatcher } from 'svelte';
	import { _ } from '../stores/i18n';
	import SimpleDialog from './SimpleDialog.svelte';
	import LoadingSpinner from './LoadingSpinner.svelte';

	// Props
	export let show = false;

	// Internal state for multi-step example
	let currentStep = 1;
	let email = '';
	let isLoading = false;
	let formErrors: Record<string, string> = {};

	const dispatch = createEventDispatcher<{
		close: void;
		complete: { email: string; step: number };
	}>();

	// Example form validation
	function validateEmail(email: string): boolean {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		return emailRegex.test(email);
	}

	// Handle step progression
	async function handleNextStep() {
		formErrors = {};

		if (currentStep === 1) {
			// Validate email in step 1
			if (!email.trim()) {
				formErrors.email = $_('auth.emailRequired');
				return;
			}
			if (!validateEmail(email)) {
				formErrors.email = $_('auth.emailInvalid');
				return;
			}

			// Simulate async operation
			isLoading = true;
			await new Promise((resolve) => setTimeout(resolve, 1000));
			isLoading = false;

			currentStep = 2;
		} else if (currentStep === 2) {
			// Complete the flow
			dispatch('complete', { email, step: currentStep });
			handleClose();
		}
	}

	function handlePreviousStep() {
		if (currentStep > 1) {
			currentStep--;
		}
	}

	function handleClose() {
		show = false;
		currentStep = 1;
		email = '';
		formErrors = {};
		dispatch('close');
	}
</script>

<SimpleDialog
	bind:show
	title={currentStep === 1 ? 'Step 1: Email Input' : 'Step 2: Confirmation'}
	size="md"
	closable={!isLoading}
	closeOnBackdrop={!isLoading}
>
	{#snippet children()}
		<!-- Dynamic content based on step -->
		{#if currentStep === 1}
			<div class="space-y-4">
				<p class="text-gray-600 dark:text-gray-300">
					Enter your email address to continue with the process.
				</p>

				<div>
					<label
						for="email"
						class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
					>
						{$_('auth.emailAddress')}
					</label>
					<input
						id="email"
						type="email"
						bind:value={email}
						placeholder={$_('auth.emailPlaceholder')}
						disabled={isLoading}
						class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md
					       bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100
					       focus:ring-2 focus:ring-blue-500 focus:border-transparent
					       disabled:opacity-50 disabled:cursor-not-allowed"
					/>
					{#if formErrors.email}
						<p class="text-red-600 dark:text-red-400 text-sm mt-1" role="alert">
							{formErrors.email}
						</p>
					{/if}
				</div>
			</div>
		{:else if currentStep === 2}
			<div class="space-y-4">
				<div class="text-center">
					<div
						class="mx-auto w-12 h-12 bg-green-100 dark:bg-green-800 rounded-full flex items-center justify-center mb-4"
					>
						<svg
							class="w-6 h-6 text-green-600 dark:text-green-400"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M5 13l4 4L19 7"
							></path>
						</svg>
					</div>
					<h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">Email Confirmed</h3>
					<p class="text-gray-600 dark:text-gray-300">
						Your email <strong>{email}</strong> has been validated successfully.
					</p>
				</div>
			</div>
		{/if}
	{/snippet}

	{#snippet actions()}
		<!-- Dynamic actions based on step and loading state -->
		{#if currentStep === 1}
			<button
				on:click={handleClose}
				disabled={isLoading}
				class="px-4 py-2 text-gray-600 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
			>
				{$_('common.cancel')}
			</button>
			<button
				on:click={handleNextStep}
				disabled={isLoading || !email.trim()}
				class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white rounded-lg font-medium transition-colors disabled:cursor-not-allowed flex items-center gap-2"
			>
				{#if isLoading}
					<LoadingSpinner size="sm" />
				{/if}
				{$_('common.continue')}
			</button>
		{:else if currentStep === 2}
			<button
				on:click={handlePreviousStep}
				class="px-4 py-2 text-gray-600 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium transition-colors"
			>
				{$_('common.back')}
			</button>
			<button
				on:click={handleNextStep}
				class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg font-medium transition-colors"
			>
				Complete
			</button>
		{/if}
	{/snippet}
</SimpleDialog>
