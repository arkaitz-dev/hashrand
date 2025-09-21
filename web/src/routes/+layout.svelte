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
	import { goto, replaceState } from '$app/navigation';
	import DialogContainer from '$lib/components/DialogContainer.svelte';
	import { parseNextParameterJson } from '$lib/utils/navigation';

	let { children } = $props();

	// Update current route in store and initialize sprite detection
	// Track validation state to prevent duplicates
	let isValidating = false;

	// CRITICAL: Prevent duplicate magic link processing
	let magicLinkProcessing = false;
	let lastProcessedToken = '';

	/**
	 * Force magic link validation - bypasses SvelteKit hydration issues
	 */
	async function forceMagicLinkValidation(magicToken: string) {
		// CRITICAL: Prevent duplicate processing
		if (magicLinkProcessing || lastProcessedToken === magicToken) {
			return;
		}

		magicLinkProcessing = true;
		lastProcessedToken = magicToken;

		try {
			// Import authStore dynamically
			const { authStore } = await import('$lib/stores/auth');
			const { parseNextParameterJson } = await import('$lib/utils/navigation');

			// Validate the magic link (Ed25519 verification by backend)
			const loginResponse = await authStore.validateMagicLink(magicToken);

			// Clean URL after successful validation
			const newUrl = new window.URL(window.location.href);
			newUrl.searchParams.delete('magiclink');
			// Import replaceState dynamically to avoid SSR issues
			const { replaceState } = await import('$app/navigation');
			replaceState(newUrl.toString(), {});

			// Handle next parameter from response if present
			if (loginResponse.next) {
				// Parse next parameter JSON and create navigation URL
				const navigationUrl = await parseNextParameterJson(loginResponse.next);

				if (navigationUrl !== '/') {
					const { goto } = await import('$app/navigation');
					await goto(navigationUrl);
				}
			}
		} catch {
			try {
				// Magic link validation failed

				const { goto } = await import('$app/navigation');
				goto('/');
			} catch {
				// Fallback for debugging
			} finally {
				// CRITICAL: Always reset processing flag
				magicLinkProcessing = false;
			}
		} finally {
			// CRITICAL: Ensure processing flag is always cleared
			magicLinkProcessing = false;
		}
	}

	// CRITICAL: Force client-side execution immediately when browser loads
	if (typeof window !== 'undefined') {
		// Check if we have a magic link in URL
		const urlParams = new URLSearchParams(window.location.search);
		const magicToken = urlParams.get('magiclink');
		if (magicToken && window.location.pathname === '/') {
			// FORCE execution immediately - SvelteKit hydration issue workaround
			setTimeout(async () => {
				try {
					// Force validation since onMount/page.subscribe don't execute
					await forceMagicLinkValidation(magicToken);
				} catch {
					// Fallback for debugging
				}
			}, 200);
		}
	}

	onMount(() => {
		const unsubscribe = page.subscribe(($page) => {
			currentRoute.set($page.url.pathname);

			// Check for magic link parameter - only process on root page to avoid interference
			const magicToken = $page.url.searchParams.get('magiclink');
			const isRootPage = $page.url.pathname === '/';

			// Only process magic links on root page to prevent navigation interference
			if (magicToken && isRootPage) {
				// CRITICAL: Prevent concurrent validations and duplicates
				if (isValidating || magicLinkProcessing) {
					return;
				}

				// CRITICAL: Prevent reprocessing same token
				if (lastProcessedToken === magicToken) {
					return;
				}

				// Mark as processing and store token
				magicLinkProcessing = true;
				lastProcessedToken = magicToken;

				// Process magic link - URL will be cleaned after successful validation
				handleMagicLinkValidation(magicToken);
			} else if (magicToken && !isRootPage) {
				// Magic link found during internal navigation - ignore (URL cleaning not needed)
			}
		});

		// Initialize sprite preload detection
		initializeSpriteLoader();

		return unsubscribe;
	});

	/**
	 * Handle magic link validation when present in URL
	 */
	async function handleMagicLinkValidation(magicToken: string) {
		// Set validation state
		isValidating = true;

		let loginResponse: { next?: string } | null = null;
		let validationSuccessful = false;

		try {
			// Validate the magic link (Ed25519 verification by backend)
			loginResponse = await authStore.validateMagicLink(magicToken);
			validationSuccessful = true;
		} catch {
			// Show error and redirect to home page (URL already cleaned)
			// Magic link validation failed
			goto('/');
			return;
		} finally {
			// Always reset validation state
			isValidating = false;
			// CRITICAL: Always reset processing flag
			magicLinkProcessing = false;
		}

		// If validation was successful, handle navigation
		if (validationSuccessful && loginResponse) {
			// Clean URL after successful validation to prevent race conditions
			const newUrl = new window.URL(window.location.href);
			newUrl.searchParams.delete('magiclink');
			replaceState(newUrl.toString(), {});

			// Handle next parameter from response if present - AFTER crypto tokens are ready
			if (loginResponse.next) {
				try {
					// Wait for crypto tokens to be available before processing next parameter
					let tokensReady = false;
					let attempts = 0;
					const maxAttempts = 50; // 5 seconds max wait

					while (!tokensReady && attempts < maxAttempts) {
						const cipherToken = authStore.getCipherToken();
						const nonceToken = authStore.getNonceToken();
						const hmacKey = authStore.getHmacKey();

						if (cipherToken && nonceToken && hmacKey) {
							tokensReady = true;
						} else {
							await new Promise((resolve) => setTimeout(resolve, 100));
							attempts++;
						}
					}

					if (!tokensReady) {
						// Proceed without encryption if tokens not ready
					}

					// Parse next parameter JSON and create navigation URL
					const navigationUrl = await parseNextParameterJson(loginResponse.next);

					if (navigationUrl !== '/') {
						await goto(navigationUrl);
					}
				} catch {
					// Don't prevent successful authentication, just stay on current page
				}
			}
		}

		// CRITICAL: Ensure processing flag is always cleared
		magicLinkProcessing = false;
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
