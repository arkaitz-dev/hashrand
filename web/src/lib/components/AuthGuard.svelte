<script lang="ts">
	/**
	 * Auth Guard Component
	 *
	 * Wraps content that requires authentication. Shows LoginDialog
	 * if user is not authenticated, otherwise renders the slot content.
	 */

	// import { authStore } from '../stores/auth';
	import LoginDialog from './LoginDialog.svelte';

	// Component state
	let showLoginDialog = false;

	/**
	 * Handle authentication requirement
	 * Called when protected action is triggered
	 */
	export function requireAuth(): Promise<boolean> {
		return new Promise((resolve) => {
			// VERSIÓN ULTRA SIMPLE: siempre mostrar LoginDialog
			// Asumimos que el usuario no está autenticado por ahora

			showLoginDialog = true;

			// Wait for authentication
			const checkInterval = globalThis.setInterval(() => {
				// Verificación simple usando localStorage
				const hasToken = typeof window !== 'undefined' && localStorage.getItem('access_token');
				const hasUser = typeof window !== 'undefined' && localStorage.getItem('auth_user');

				if (hasToken && hasUser) {
					globalThis.clearInterval(checkInterval);
					showLoginDialog = false;
					resolve(true);
				}
			}, 500);

			// Timeout after 2 minutos (más corto para testing)
			globalThis.setTimeout(() => {
				globalThis.clearInterval(checkInterval);
				showLoginDialog = false;
				resolve(false);
			}, 120000);
		});
	}

	/**
	 * Handle login dialog close
	 */
	function handleLoginClose() {
		showLoginDialog = false;
	}

	/**
	 * Handle successful authentication
	 */
	function handleAuthenticated(event: globalThis.CustomEvent<{ email: string }>) {
		showLoginDialog = false;
		console.log('User authenticated:', event.detail.email);
	}
</script>

<!-- Always render slot content, regardless of authentication status -->
<slot {requireAuth} />

<!-- Login Dialog -->
<LoginDialog
	bind:show={showLoginDialog}
	on:close={handleLoginClose}
	on:authenticated={handleAuthenticated}
/>
