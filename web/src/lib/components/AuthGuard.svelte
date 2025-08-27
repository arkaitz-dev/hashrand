<script lang="ts">
	/**
	 * Auth Guard Component
	 * 
	 * Wraps content that requires authentication. Shows LoginDialog
	 * if user is not authenticated, otherwise renders the slot content.
	 */

	import { onMount } from 'svelte';
	import { authStore } from '../stores/auth';
	import LoginDialog from './LoginDialog.svelte';

	// Component state
	let isAuthenticated = false;
	let isLoading = true;
	let showLoginDialog = false;

	// Store subscription (removed unused variable)

	onMount(() => {
		// Subscribe to auth store changes
		const unsubscribe = authStore.subscribe(state => {
			isLoading = state.isLoading;
		});

		// Check initial authentication status
		checkAuthStatus();

		return unsubscribe;
	});

	/**
	 * Check if user is authenticated
	 */
	async function checkAuthStatus() {
		isLoading = true;
		try {
			isAuthenticated = await authStore.isAuthenticated();
			if (!isAuthenticated) {
				showLoginDialog = true;
			}
		} catch (error) {
			console.error('Auth check failed:', error);
			isAuthenticated = false;
			showLoginDialog = true;
		} finally {
			isLoading = false;
		}
	}

	/**
	 * Handle authentication requirement
	 * Called when protected action is triggered
	 */
	export function requireAuth(): Promise<boolean> {
		return new Promise((resolve) => {
			if (isAuthenticated) {
				resolve(true);
				return;
			}

			// Show login dialog
			showLoginDialog = true;

			// Wait for authentication
			const checkInterval = globalThis.setInterval(async () => {
				const authenticated = await authStore.isAuthenticated();
				if (authenticated) {
					globalThis.clearInterval(checkInterval);
					isAuthenticated = true;
					showLoginDialog = false;
					resolve(true);
				}
			}, 500);

			// Timeout after 5 minutes
			globalThis.setTimeout(() => {
				globalThis.clearInterval(checkInterval);
				resolve(false);
			}, 300000);
		});
	}

	/**
	 * Handle login dialog close
	 */
	function handleLoginClose() {
		showLoginDialog = false;
		// Re-check auth status in case user authenticated in another tab
		checkAuthStatus();
	}

	/**
	 * Handle successful authentication
	 */
	function handleAuthenticated(event: globalThis.CustomEvent<{ email: string }>) {
		isAuthenticated = true;
		showLoginDialog = false;
		console.log('User authenticated:', event.detail.email);
	}
</script>

{#if isLoading}
	<!-- Loading state -->
	<div class="flex items-center justify-center p-8">
		<div class="text-center">
			<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-2"></div>
			<p class="text-sm text-gray-600 dark:text-gray-400">Verificando autenticación...</p>
		</div>
	</div>
{:else if isAuthenticated}
	<!-- Authenticated: render slot content -->
	<slot {requireAuth} />
{:else}
	<!-- Not authenticated: show placeholder or login prompt -->
	<div class="text-center p-8">
		<div class="mb-4">
			<svg class="w-16 h-16 text-gray-400 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
			</svg>
		</div>
		<h3 class="text-lg font-medium text-gray-900 dark:text-gray-100 mb-2">
			Autenticación requerida
		</h3>
		<p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
			Necesitas autenticarte para acceder a esta funcionalidad.
		</p>
		<button
			on:click={() => showLoginDialog = true}
			class="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-md transition-colors"
		>
			Iniciar sesión
		</button>
	</div>
{/if}

<!-- Login Dialog -->
<LoginDialog 
	bind:show={showLoginDialog} 
	on:close={handleLoginClose}
	on:authenticated={handleAuthenticated}
/>