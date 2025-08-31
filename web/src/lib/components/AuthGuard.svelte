<script lang="ts">
	/**
	 * Auth Guard Component
	 *
	 * Wraps content that requires authentication. Redirects to /login
	 * if user is not authenticated, otherwise renders the slot content.
	 */

	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { base58 } from '@scure/base';
	import { authStore } from '../stores/auth';

	/**
	 * Handle authentication requirement
	 * Called when protected action is triggered
	 */
	export async function requireAuth(formParams?: Record<string, unknown>): Promise<boolean> {
		// Check if already authenticated
		const isAuth = await authStore.isAuthenticated();
		if (isAuth) {
			return true;
		}

		// Not authenticated, redirect to login with next parameter
		const currentPath = $page.url.pathname;

		if (formParams && Object.keys(formParams).length > 0) {
			// Encode form parameters as Base58 for next parameter
			const encoder = new globalThis.TextEncoder();
			const jsonString = JSON.stringify({
				endpoint: currentPath.replace('/', ''),
				...formParams
			});
			const bytes = encoder.encode(jsonString);
			const nextParam = base58.encode(bytes);

			await goto(`/login?next=${encodeURIComponent(nextParam)}`);
		} else {
			// Simple redirect without encoded parameters
			await goto(`/login?next=${encodeURIComponent(currentPath)}`);
		}

		return false;
	}
</script>

<!-- Always render slot content, regardless of authentication status -->
<slot {requireAuth} />
