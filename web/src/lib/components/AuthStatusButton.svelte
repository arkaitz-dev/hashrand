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
	import { sessionStatusStore } from '../stores/session-status';

	// Component state
	let showUserDropdown = $state(false);

	// Reactive auth state
	let isAuthenticated = $state(false);
	let userId = $state('');

	// Session status from global store (managed by layout)
	let sessionExpired = $state(false);

	// Subscribe to auth store
	$effect(() => {
		const unsubscribe = authStore.subscribe((state) => {
			isAuthenticated = !!(state.user && state.accessToken);
			userId = state.user?.user_id || '';
		});

		return unsubscribe;
	});

	// Subscribe to global session status store (managed by layout)
	$effect(() => {
		const unsubscribe = sessionStatusStore.subscribe((status) => {
			sessionExpired = status.isExpired;
		});

		return unsubscribe;
	});

	/**
	 * Handle main button click - CHECK SESSION EXPIRATION FIRST
	 *
	 * Uses global session status; if expired, launches auth dialog
	 */
	async function handleButtonClick() {
		// If session is expired (per global store), launch auth dialog immediately
		if (sessionExpired) {
			// Launch magic link auth dialog
			const authConfig = {
				destination: { route: '/' }
			};
			dialogStore.show('auth', authConfig);
			return;
		}

		// Session is valid - proceed with original auth button logic
		if (isAuthenticated) {
			// User appears authenticated locally - show dropdown
			showUserDropdown = !showUserDropdown;
		} else {
			// Check if we have local tokens without HTTP calls
			const hasTokensLocally = await authStore.hasLocalAuthTokens();

			if (hasTokensLocally) {
				// We have tokens locally but reactive auth state shows false
				// This means store might not be synced - refresh state from storage
				// But don't make HTTP calls - let the UI show as authenticated
				// Validation will happen reactively on next API operation
				showUserDropdown = true;
			} else {
				// No local tokens - show login dialog immediately
				// Clear any residual auth data before asking for email (defensive security)
				authStore.clearPreventiveAuthData();

				const authConfig = {
					destination: { route: '/' }
				};
				dialogStore.show('auth', authConfig);
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
		class:bg-white={showUserDropdown && !sessionExpired}
		class:dark:bg-gray-800={showUserDropdown && !sessionExpired}
		class:shadow-lg={showUserDropdown}
		class:border-gray-200={showUserDropdown && !sessionExpired}
		class:dark:border-gray-700={showUserDropdown && !sessionExpired}
		class:scale-105={showUserDropdown}
		class:yellow-pulse-animation={sessionExpired}
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

<style>
	/* Yellow pulsing animation for expired session indicator */
	@keyframes yellowPulse {
		0% {
			background-color: #713f12; /* yellow-900 */
			border-color: #a16207; /* yellow-800 */
			box-shadow: 0 0 10px rgba(113, 63, 18, 0.8);
		}
		10% {
			background-color: #a16207; /* yellow-800 */
			border-color: #ca8a04; /* yellow-700 */
			box-shadow: 0 0 12px rgba(161, 98, 7, 0.8);
		}
		20% {
			background-color: #ca8a04; /* yellow-700 */
			border-color: #d97706; /* yellow-600 */
			box-shadow: 0 0 14px rgba(202, 138, 4, 0.8);
		}
		30% {
			background-color: #d97706; /* yellow-600 */
			border-color: #f59e0b; /* yellow-500 */
			box-shadow: 0 0 16px rgba(217, 119, 6, 0.8);
		}
		40% {
			background-color: #f59e0b; /* yellow-500 */
			border-color: #fbbf24; /* yellow-400 */
			box-shadow: 0 0 18px rgba(245, 158, 11, 0.8);
		}
		45% {
			background-color: #fbbf24; /* yellow-400 */
			border-color: #fcd34d; /* yellow-300 */
			box-shadow: 0 0 20px rgba(251, 191, 36, 0.9);
		}
		50% {
			background-color: #fcd34d; /* yellow-300 - punto más claro */
			border-color: #fde68a; /* yellow-200 */
			box-shadow: 0 0 25px rgba(252, 211, 77, 1);
		}
		55% {
			background-color: #fbbf24; /* yellow-400 */
			border-color: #fcd34d; /* yellow-300 */
			box-shadow: 0 0 20px rgba(251, 191, 36, 0.9);
		}
		60% {
			background-color: #f59e0b; /* yellow-500 */
			border-color: #fbbf24; /* yellow-400 */
			box-shadow: 0 0 18px rgba(245, 158, 11, 0.8);
		}
		70% {
			background-color: #d97706; /* yellow-600 */
			border-color: #f59e0b; /* yellow-500 */
			box-shadow: 0 0 16px rgba(217, 119, 6, 0.8);
		}
		80% {
			background-color: #ca8a04; /* yellow-700 */
			border-color: #d97706; /* yellow-600 */
			box-shadow: 0 0 14px rgba(202, 138, 4, 0.8);
		}
		90% {
			background-color: #a16207; /* yellow-800 */
			border-color: #ca8a04; /* yellow-700 */
			box-shadow: 0 0 12px rgba(161, 98, 7, 0.8);
		}
		100% {
			background-color: #713f12; /* yellow-900 - vuelta al inicio */
			border-color: #a16207; /* yellow-800 */
			box-shadow: 0 0 10px rgba(113, 63, 18, 0.8);
		}
	}

	.yellow-pulse-animation {
		animation: yellowPulse 1.5s ease-in-out infinite;
	}
</style>
