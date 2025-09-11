<script lang="ts">
	/**
	 * Authentication Status Button Component
	 *
	 * Shows authentication status with appropriate icon:
	 * - Not authenticated: settings icon, opens login dialog
	 * - Authenticated: check icon, opens user dropdown with logout option
	 */

	import { authStore } from '../stores/auth';
	import { dialogStore } from '../stores/dialog';
	import { _ } from '../stores/i18n';
	import Icon from './Icon.svelte';

	// Component state
	let showUserDropdown = $state(false);

	// Reactive auth state
	let isAuthenticated = $state(false);
	let userId = $state('');

	// Subscribe to auth store
	$effect(() => {
		const unsubscribe = authStore.subscribe((state) => {
			isAuthenticated = !!(state.user && state.accessToken);
			userId = state.user?.user_id || '';
		});

		return unsubscribe;
	});

	/**
	 * Handle main button click
	 */
	async function handleButtonClick() {
		if (isAuthenticated) {
			showUserDropdown = !showUserDropdown;
		} else {
			// Try to authenticate with automatic refresh first
			const isNowAuthenticated = await authStore.ensureAuthenticated();

			if (isNowAuthenticated) {
				// Refresh succeeded - show dropdown
				showUserDropdown = true;
			} else {
				// Only show authentication dialog if refresh failed
				dialogStore.show('auth');
			}
		}
	}

	/**
	 * Handle logout confirmation - show logout confirmation dialog
	 */
	function handleLogout() {
		showUserDropdown = false;
		dialogStore.show('logout');
	}

	/**
	 * Close dropdown when clicking outside
	 */
	function handleClickOutside(event: MouseEvent) {
		if (showUserDropdown && !(event.target as Element).closest('.auth-status-button')) {
			showUserDropdown = false;
		}
	}

	/**
	 * Handle escape key to close dropdown
	 */
	function handleKeydown(event: globalThis.KeyboardEvent) {
		if (event.key === 'Escape' && showUserDropdown) {
			showUserDropdown = false;
		}
	}
</script>

<svelte:window on:click={handleClickOutside} on:keydown={handleKeydown} />

<!-- Auth Status Button Container -->
<div class="auth-status-button relative">
	<button
		class="p-1.5 sm:p-2 rounded-lg sm:rounded-xl bg-transparent border border-transparent shadow-none hover:bg-white hover:dark:bg-gray-800 hover:shadow-lg hover:border-gray-200 hover:dark:border-gray-700 active:bg-white active:dark:bg-gray-800 active:shadow-lg active:border-gray-200 active:dark:border-gray-700 transition-colors duration-[750ms] transition-shadow duration-[750ms] transition-border-colors duration-[750ms] transform hover:scale-105 focus:outline-none flex items-center justify-center w-9 h-9 sm:w-12 sm:h-12 disabled:opacity-50 disabled:cursor-not-allowed"
		disabled={$authStore.isRefreshing}
		class:bg-white={showUserDropdown}
		class:dark:bg-gray-800={showUserDropdown}
		class:shadow-lg={showUserDropdown}
		class:border-gray-200={showUserDropdown}
		class:dark:border-gray-700={showUserDropdown}
		class:scale-105={showUserDropdown}
		aria-label={$authStore.isRefreshing
			? $_('common.loading')
			: isAuthenticated
				? $_('auth.userMenu')
				: $_('auth.login')}
		title={$authStore.isRefreshing
			? $_('common.loading')
			: isAuthenticated
				? $_('auth.userMenu')
				: $_('auth.login')}
		onclick={handleButtonClick}
	>
		<div class="text-gray-700 dark:text-gray-300 transition-all duration-150 transform">
			{#if $authStore.isRefreshing}
				<!-- Loading spinner - CSS circle -->
				<div
					class="w-5 h-5 sm:w-6 sm:h-6 border-2 border-gray-300 dark:border-gray-600 border-t-gray-700 dark:border-t-gray-300 rounded-full animate-spin"
				></div>
			{:else}
				<Icon name="user" size="w-5 h-5 sm:w-6 sm:h-6" />
			{/if}
		</div>
	</button>

	<!-- User Dropdown (when authenticated) -->
	{#if showUserDropdown && isAuthenticated}
		<div
			class="absolute top-full mt-2 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700 py-2 min-w-[200px] right-0"
		>
			<!-- Username display (non-interactive) -->
			<div
				class="px-4 py-2 text-sm text-gray-500 dark:text-gray-400 border-b border-gray-200 dark:border-gray-700"
			>
				<div class="font-medium text-gray-700 dark:text-gray-300 mb-1">
					{$_('auth.authenticatedAs')}
				</div>
				<div class="font-mono text-xs break-all">
					{userId}
				</div>
			</div>

			<!-- Logout button -->
			<button
				class="w-full px-4 py-2 text-left hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3 text-red-600 dark:text-red-400 hover:text-red-700 dark:hover:text-red-300 transition-colors duration-150"
				onclick={handleLogout}
			>
				<Icon name="settings" size="w-4 h-4" />
				<span class="text-sm font-medium">{$_('auth.logout')}</span>
			</button>
		</div>
	{/if}
</div>
