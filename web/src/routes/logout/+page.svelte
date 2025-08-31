<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { t, currentLanguage } from '$lib/stores/i18n';
	import { authStore } from '$lib/stores/auth';
	import { flashMessagesStore } from '$lib/stores/flashMessages';
	import FlashMessages from '$lib/components/FlashMessages.svelte';

	let isLoggingOut = false;

	onMount(() => {
		// Clear any existing flash messages
		flashMessagesStore.clear();
	});

	async function handleLogout() {
		isLoggingOut = true;

		try {
			// Clear authentication
			authStore.logout();

			// Add success message
			flashMessagesStore.addMessage(
				$currentLanguage === 'es' ? '✅ Sesión cerrada correctamente' : '✅ Successfully logged out'
			);

			// Redirect to home
			setTimeout(() => {
				goto('/');
			}, 1500);
		} catch (error) {
			console.error('Error during logout:', error);
			flashMessagesStore.addMessage(
				$currentLanguage === 'es' ? '❌ Error al cerrar sesión' : '❌ Error during logout'
			);
			isLoggingOut = false;
		}
	}

	function handleCancel() {
		// Just go back to the previous page
		goto('/');
	}
</script>

<svelte:head>
	<title>{$t.logout.title} - HashRand Spin</title>
	<meta name="description" content={$t.logout.description} />
</svelte:head>

<div class="container max-w-md mx-auto px-4 py-8">
	<!-- Flash Messages -->
	<FlashMessages />

	<!-- Logout Confirmation Card -->
	<div
		class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-200 dark:border-gray-700"
	>
		<!-- Icon -->
		<div class="flex justify-center mb-4">
			<div
				class="w-16 h-16 bg-red-100 dark:bg-red-900 rounded-full flex items-center justify-center"
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
						d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
					></path>
				</svg>
			</div>
		</div>

		<!-- Title -->
		<h1 class="text-2xl font-bold text-center text-gray-900 dark:text-white mb-2">
			{$t.logout.title}
		</h1>

		<!-- Description -->
		<p class="text-center text-gray-600 dark:text-gray-300 mb-6">
			{$t.logout.description}
		</p>

		<!-- Action Buttons -->
		<div class="flex gap-4">
			<!-- Cancel Button -->
			<button
				onclick={handleCancel}
				disabled={isLoggingOut}
				class="flex-1 px-4 py-2 text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
			>
				{$t.logout.cancel}
			</button>

			<!-- Logout Button -->
			<button
				onclick={handleLogout}
				disabled={isLoggingOut}
				class="flex-1 px-4 py-2 text-white bg-red-600 hover:bg-red-700 disabled:bg-red-400 rounded-lg font-medium transition-colors disabled:cursor-not-allowed flex items-center justify-center gap-2"
			>
				{#if isLoggingOut}
					<svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
						></circle>
						<path
							class="opacity-75"
							fill="currentColor"
							d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
				{/if}
				{$t.logout.confirm}
			</button>
		</div>

		<!-- Additional Information -->
		<div class="mt-6 p-3 bg-blue-50 dark:bg-blue-900 rounded-lg">
			<p class="text-sm text-blue-800 dark:text-blue-200">
				{$t.logout.info}
			</p>
		</div>
	</div>
</div>
