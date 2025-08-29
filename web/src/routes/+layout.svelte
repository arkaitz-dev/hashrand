<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { onMount } from 'svelte';
	import { currentRoute } from '$lib/stores/navigation';
	import { page } from '$app/stores';
	import TopControls from '$lib/components/TopControls.svelte';
	// Import theme store to ensure it's initialized
	import '$lib/stores/theme';
	import { isRTL, textDirection } from '$lib/stores/rtl';
	import { initializeSpriteLoader } from '$lib/stores/spriteLoader';
	import { authStore } from '$lib/stores/auth';
	import { goto } from '$app/navigation';

	let { children } = $props();

	// Update current route in store and initialize sprite detection
	onMount(() => {
		const unsubscribe = page.subscribe(($page) => {
			currentRoute.set($page.url.pathname);

			// Check for magic link parameter
			const magicToken = $page.url.searchParams.get('magiclink');
			if (magicToken) {
				handleMagicLinkValidation(magicToken, $page);
			}
		});

		// Initialize sprite preload detection
		initializeSpriteLoader();

		return unsubscribe;
	});

	/**
	 * Handle magic link validation when present in URL
	 */
	async function handleMagicLinkValidation(
		magicToken: string,
		currentPage: { url: globalThis.URL }
	) {
		try {
			// Extract email from URL fragment or localStorage if available
			const email = extractEmailFromContext();

			if (!email) {
				console.warn('No email context found for magic link validation');
				// Redirect to home and show login dialog
				goto('/');
				return;
			}

			console.log('Validating magic link for:', email);

			// Validate the magic link
			await authStore.validateMagicLink(magicToken, email);

			console.log('Authentication successful!');

			// Remove magiclink parameter from URL
			const newUrl = new globalThis.URL(currentPage.url);
			newUrl.searchParams.delete('magiclink');

			// Update URL without page reload
			globalThis.window?.history?.replaceState({}, '', newUrl.toString());

			// Show success message or redirect to original page
			// Could add a toast notification here
		} catch (error) {
			console.error('Magic link validation failed:', error);

			// Remove failed magiclink parameter from URL
			const newUrl = new globalThis.URL(currentPage.url);
			newUrl.searchParams.delete('magiclink');
			globalThis.window?.history?.replaceState({}, '', newUrl.toString());

			// Redirect to home page
			goto('/');
		}
	}

	/**
	 * Extract email from available context (localStorage, etc.)
	 */
	function extractEmailFromContext(): string | null {
		// Try to get email from localStorage (might be stored during login request)
		const storedEmail = localStorage.getItem('pending_auth_email');
		if (storedEmail) {
			// Clean up after use
			localStorage.removeItem('pending_auth_email');
			return storedEmail;
		}

		// Could also try to extract from URL fragment or other sources
		// For now, return null if no email context is available
		return null;
	}

	// Apply RTL direction to document
	$effect(() => {
		if (typeof document !== 'undefined') {
			document.documentElement.dir = $textDirection;
			document.documentElement.setAttribute('data-rtl', $isRTL.toString());
		}
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<meta name="viewport" content="width=device-width, initial-scale=1.0" />
	<meta name="theme-color" content="#3b82f6" media="(prefers-color-scheme: light)" />
	<meta name="theme-color" content="#1e293b" media="(prefers-color-scheme: dark)" />
</svelte:head>

<main class="min-h-screen relative">
	<!-- Top Controls Container -->
	<TopControls />

	{@render children?.()}
</main>
