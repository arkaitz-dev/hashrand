<script lang="ts">
	/**
	 * Global error page - handles 404, 500, and other errors
	 * Redirects to home with flash message as requested
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { flashMessagesStore } from '$lib/stores/flashMessages';
	import { _ } from '$lib/stores/i18n';

	onMount(async () => {
		// Get error information
		const error = $page.error;
		const status = $page.status;

		console.log('🚫 Error page triggered:', { status, error });

		// Create appropriate translated error message
		let message = '';

		if (status === 404) {
			message = $_('errors.pageNotFoundMessage');
		} else if (status === 500) {
			message = $_('errors.serverErrorMessage');
		} else if (status >= 400 && status < 500) {
			message = $_('errors.clientErrorMessage').replace('{status}', status.toString());
		} else {
			message = $_('errors.genericErrorMessage').replace('{status}', status.toString());
		}

		// Add flash message with error details
		flashMessagesStore.addMessage(message);

		// Redirect to home after a short delay
		setTimeout(() => {
			goto('/', { replaceState: true });
		}, 1000);
	});
</script>

<svelte:head>
	<title>Error - {$_('menu.brandName')}</title>
</svelte:head>

<div
	class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800 flex items-center justify-center px-4"
>
	<div class="text-center">
		<div class="mb-8">
			<div
				class="inline-flex items-center justify-center w-16 h-16 bg-red-100 dark:bg-red-900 rounded-full mb-4"
			>
				<span class="text-2xl">❌</span>
			</div>
			<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
				Error {$page.status}
			</h1>
			<p class="text-gray-600 dark:text-gray-300 mb-4">
				{$page.status === 404 ? 'Page not found' : 'Something went wrong'}
			</p>
			<p class="text-sm text-gray-500 dark:text-gray-400">Redirecting to home page...</p>
		</div>

		<div class="flex items-center justify-center">
			<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
		</div>
	</div>
</div>
