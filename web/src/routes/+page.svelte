<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { navigationItems } from '$lib/stores/navigation';
	import { clearResult } from '$lib/stores/result';
	import { _ } from '$lib/stores/i18n';
	import MenuCard from '$lib/components/MenuCard.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { authStore } from '$lib/stores/auth';
	import { logger } from '$lib/utils/logger';

	onMount(async () => {
		logger.info('[Route] Home page loaded');

		// Clear result state when returning to menu - this resets all form values to defaults
		clearResult();

		// Validate that home route only accepts 'magiclink' and 'shared' parameters
		const searchParams = $page.url.searchParams;
		const allowedParams = ['magiclink', 'shared'];

		// Handle shared secret parameter (?shared=[hash] from email link)
		const sharedHash = searchParams.get('shared');
		if (sharedHash) {
			logger.info('[Route] Shared secret hash detected, checking auth');

			// 1. Check if user has local auth tokens (no HTTP call)
			const { hasLocalAuthTokens } = await import('$lib/stores/auth/auth-session');
			const hasTokens = await hasLocalAuthTokens();

			if (!hasTokens) {
				// No tokens → show auth dialog with destination
				logger.info('[Route] No tokens found, showing auth dialog');
				const { dialogStore } = await import('$lib/stores/dialog');
				dialogStore.show('auth', {
					destination: { route: `/shared-secret/${sharedHash}` }
				});
				return;
			}

			// 2. Has tokens → check if session expired (manual check, no auto-redirect)
			const { isSessionExpired } = await import('$lib/session-expiry-manager');
			const expired = await isSessionExpired();

			if (expired) {
				// Session expired → cleanup + show auth dialog with destination
				logger.info('[Route] Session expired, cleaning up and showing auth dialog');
				const { clearLocalAuthData } = await import('$lib/stores/auth/auth-actions');
				await clearLocalAuthData();

				const { dialogStore } = await import('$lib/stores/dialog');
				dialogStore.show('auth', {
					destination: { route: `/shared-secret/${sharedHash}` }
				});
				return;
			}

			// 3. Session is valid → redirect directly to shared-secret
			logger.info('[Route] Session valid, redirecting to shared-secret');
			goto(`/shared-secret/${sharedHash}`);
			return;
		}

		// Check for any unauthorized parameters
		for (const [key] of searchParams) {
			if (!allowedParams.includes(key)) {
				logger.warn(`Unauthorized parameter '${key}' detected on home route, redirecting`);
				// Remove unauthorized parameter and redirect
				const cleanUrl = new globalThis.URL($page.url);
				cleanUrl.search = '';
				if (searchParams.has('magiclink')) {
					cleanUrl.searchParams.set('magiclink', searchParams.get('magiclink')!);
				}
				if (searchParams.has('shared')) {
					cleanUrl.searchParams.set('shared', searchParams.get('shared')!);
				}
				goto(cleanUrl.pathname + cleanUrl.search, { replaceState: true });
				break;
			}
		}
	});

	// Effect to watch for user_id changes (only on this page)
	$effect(() => {
		const userId = $authStore.user?.user_id;
		const currentPath = $page.url.pathname;

		if (userId && currentPath === '/') {
			// User ID is set and we're on home page
		}
	});

	function getTranslatedTitle(itemId: string): string {
		switch (itemId) {
			case 'custom':
				return $_('custom.title');
			case 'password':
				return $_('password.title');
			case 'api-key':
				return $_('apiKey.title');
			case 'mnemonic':
				return $_('mnemonic.title');
			case 'shared-secret':
				return $_('sharedSecret.title');
			default:
				return '';
		}
	}

	function getTranslatedDescription(itemId: string): string {
		switch (itemId) {
			case 'custom':
				return $_('custom.description');
			case 'password':
				return $_('password.description');
			case 'api-key':
				return $_('apiKey.description');
			case 'mnemonic':
				return $_('mnemonic.description');
			case 'shared-secret':
				return $_('sharedSecret.description');
			default:
				return '';
		}
	}
</script>

<svelte:head>
	<title>{$_('menu.title')} - {$_('menu.brandName')}</title>
	<meta name="description" content={$_('menu.description')} />
</svelte:head>

<div
	class="flex-1 bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"
>
	<div class="container mx-auto px-4 py-8">
		<!-- Header -->
		<header class="text-center mb-12">
			<div class="inline-flex items-center justify-center w-16 h-16 bg-blue-600 rounded-full mb-6">
				<span class="text-2xl text-white">🎲</span>
			</div>
			<h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4">
				{$_('menu.title')}
			</h1>
			<p class="text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
				{$_('menu.description')}
			</p>
		</header>

		<!-- Flash Messages -->
		<FlashMessages />

		<!-- Navigation Cards -->
		<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-6 max-w-6xl mx-auto mb-12">
			{#each navigationItems as item}
				<MenuCard
					path={item.path}
					icon={item.icon}
					title={getTranslatedTitle(item.id)}
					description={getTranslatedDescription(item.id)}
				/>
			{/each}
		</div>
	</div>
</div>
