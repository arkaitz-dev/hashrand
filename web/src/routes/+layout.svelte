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
	import { dialogStore } from '$lib/stores/dialog';
	import DialogContainer from '$lib/components/DialogContainer.svelte';
	import { encryptNextUrl } from '$lib/crypto';

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
			// Validate the magic link (Ed25519 verification by backend)
			const loginResponse = await authStore.validateMagicLink(magicToken);

			// Remove magiclink parameter from URL
			const newUrl = new globalThis.URL(currentPage.url);
			newUrl.searchParams.delete('magiclink');

			// Update URL without page reload
			globalThis.window?.history?.replaceState({}, '', newUrl.toString());

			// Handle next parameter from response if present
			if (loginResponse.next) {
				// Check if we have crypto tokens for parameter encryption
				const cipherToken = authStore.getCipherToken();
				const nonceToken = authStore.getNonceToken();
				const hmacKey = authStore.getHmacKey();

				if (cipherToken && nonceToken && hmacKey) {
					// Encrypt parameters in next URL for privacy
					const encryptedNextUrl = await encryptNextUrl(loginResponse.next, {
						cipherToken,
						nonceToken,
						hmacKey
					});
					await goto(encryptedNextUrl);
				} else {
					// No crypto tokens available, navigate to next URL as-is
					await goto(loginResponse.next);
				}
			}
		} catch {
			// Clear hash on error
			localStorage.removeItem('magiclink_hash');

			// Remove failed magiclink parameter from URL
			const newUrl = new globalThis.URL(currentPage.url);
			newUrl.searchParams.delete('magiclink');
			globalThis.window?.history?.replaceState({}, '', newUrl.toString());

			// Redirect to home page
			goto('/');
		}
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

<!-- Global Dialog Container -->
<DialogContainer />
