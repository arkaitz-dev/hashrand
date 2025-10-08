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
	import VersionFooter from '$lib/components/VersionFooter.svelte';
	import UpdateButton from '$lib/components/UpdateButton.svelte';
	import { parseNextParameterJson } from '$lib/utils/navigation';
	import { initializeVersionCheck, restoreSessionState } from '$lib/stores/version-update';
	import { sessionStatusStore } from '$lib/stores/session-status';
	import { isSessionExpired } from '$lib/session-expiry-manager';
	import {
		initSessionMonitor,
		destroySessionMonitor,
		startMonitoringIfAuthenticated
	} from '$lib/sessionMonitor';
	import { logger } from '$lib/utils/logger';

	let { children } = $props();

	// Update current route in store and initialize sprite detection
	// Track validation state to prevent duplicates
	let isValidating = false;

	// CRITICAL: Prevent race conditions (concurrent calls in SAME page load)
	// Backend handles single-use validation (deletes magic link after use)
	let magicLinkProcessing = false;

	/**
	 * Force magic link validation - bypasses SvelteKit hydration issues
	 */
	async function forceMagicLinkValidation(magicToken: string) {
		logger.debug('[+layout] forceMagicLinkValidation called', {
			magicLinkProcessing,
			currentToken: magicToken.substring(0, 10) + '...'
		});

		// CRITICAL: Prevent race conditions only
		if (magicLinkProcessing) {
			logger.debug('[+layout] forceMagicLinkValidation: Already processing, skipping');
			return;
		}

		magicLinkProcessing = true;
		logger.debug('[+layout] forceMagicLinkValidation: Starting validation');

		try {
			// Validate the magic link (Ed25519 verification by backend)
			const loginResponse = await authStore.validateMagicLink(magicToken);
			logger.info('[+layout] Magic link validation successful');
			logger.debug('[+layout] Login response received', {
				hasNext: !!loginResponse.next,
				nextValue: loginResponse.next
			});

			// Mark session as valid after successful authentication
			sessionStatusStore.markValid();

			// CRITICAL: Start session monitoring after successful login
			await startMonitoringIfAuthenticated();

			// Handle next parameter and clean URL
			if (loginResponse.next) {
				logger.debug('[+layout] Processing next parameter', { next: loginResponse.next });
				// Parse next parameter JSON and create navigation URL
				const navigationUrl = await parseNextParameterJson(loginResponse.next);
				logger.debug('[+layout] Parsed navigation URL', { navigationUrl });

				if (navigationUrl !== '/') {
					// Navigate to different route - goto() doesn't preserve query params
					logger.info('[+layout] Navigating to next destination', { navigationUrl });
					await goto(navigationUrl);
				} else {
					// Staying on /, clean magiclink from URL
					logger.debug('[+layout] Staying on /, cleaning magiclink from URL');
					const newUrl = new window.URL(window.location.href);
					newUrl.searchParams.delete('magiclink');
					replaceState(newUrl.toString(), {});
				}
			} else {
				// No next parameter, clean magiclink from URL
				logger.warn('[+layout] No next parameter, cleaning magiclink from URL');
				const newUrl = new window.URL(window.location.href);
				newUrl.searchParams.delete('magiclink');
				replaceState(newUrl.toString(), {});
			}
		} catch (error) {
			logger.error('[+layout] Magic link validation failed', error);
			try {
				// Magic link validation failed
				goto('/');
			} catch (navError) {
				logger.error('[+layout] forceMagicLinkValidation: Navigation failed', navError);
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
		// DEBUG: Log full URL state at module load
		logger.debug('[+layout] Module loaded - URL state', {
			href: window.location.href,
			search: window.location.search,
			pathname: window.location.pathname,
			hash: window.location.hash
		});

		// Check if we have a magic link in URL
		const urlParams = new URLSearchParams(window.location.search);
		const magicToken = urlParams.get('magiclink');

		logger.debug('[+layout] Magic link detection check', {
			hasMagicToken: !!magicToken,
			tokenLength: magicToken?.length || 0,
			pathname: window.location.pathname,
			isRootPage: window.location.pathname === '/'
		});

		if (magicToken && window.location.pathname === '/') {
			logger.debug('[+layout] Magic link detected in URL at browser load', {
				pathname: window.location.pathname,
				tokenPrefix: magicToken.substring(0, 10) + '...'
			});
			// FORCE execution immediately - SvelteKit hydration issue workaround
			setTimeout(async () => {
				try {
					// Force validation since onMount/page.subscribe don't execute
					await forceMagicLinkValidation(magicToken);
				} catch (error) {
					logger.error('[+layout] Browser load magic link validation failed', error);
				}
			}, 200);
		} else {
			logger.debug('[+layout] No magic link in URL at browser load', {
				hasMagicToken: !!magicToken,
				pathname: window.location.pathname
			});
		}
	}

	/**
	 * Global session status check - runs on every route change
	 * Updates session status store to control AuthStatusButton styling
	 */
	async function checkGlobalSessionStatus(): Promise<void> {
		try {
			const expired = await isSessionExpired();

			if (expired) {
				sessionStatusStore.markExpired();
			} else {
				sessionStatusStore.markValid();
			}
		} catch (error) {
			logger.warn('Global session check failed:', error);
			// On error, assume expired for security
			sessionStatusStore.markExpired();
		}
	}

	onMount(() => {
		const unsubscribe = page.subscribe(async ($page) => {
			currentRoute.set($page.url.pathname);

			// GLOBAL SESSION CHECK: Verify session status on every route change
			await checkGlobalSessionStatus();

			// Check for magic link parameter - only process on root page to avoid interference
			const magicToken = $page.url.searchParams.get('magiclink');
			const isRootPage = $page.url.pathname === '/';

			// Only process magic links on root page to prevent navigation interference
			if (magicToken && isRootPage) {
				// CRITICAL: Prevent concurrent validations (race conditions)
				// Don't check magicLinkProcessing here - let handleMagicLinkValidation handle it
				if (isValidating) {
					return;
				}

				// Process magic link - URL will be cleaned after successful validation
				handleMagicLinkValidation(magicToken);
			} else if (magicToken && !isRootPage) {
				// Magic link found during internal navigation - ignore (URL cleaning not needed)
			}
		});

		// Initialize sprite preload detection
		initializeSpriteLoader();

		// Initialize version checking system
		initializeVersionCheck();

		// Restore session state if coming from update reload
		restoreSessionState();

		// Initialize session monitor infrastructure (listeners only)
		initSessionMonitor();

		// Check if user is already authenticated and start monitoring if so
		// This handles page refreshes where user already has a session
		startMonitoringIfAuthenticated();

		return () => {
			unsubscribe();
			destroySessionMonitor();
		};
	});

	/**
	 * Handle magic link validation when present in URL
	 */
	async function handleMagicLinkValidation(magicToken: string) {
		logger.debug('[+layout] handleMagicLinkValidation called', {
			magicLinkProcessing,
			currentToken: magicToken.substring(0, 10) + '...'
		});

		// CRITICAL: Prevent race conditions only
		if (magicLinkProcessing) {
			logger.debug('[+layout] handleMagicLinkValidation: Already processing, skipping');
			return;
		}

		magicLinkProcessing = true;
		logger.debug('[+layout] handleMagicLinkValidation: Starting validation');

		// Set validation state
		isValidating = true;

		let loginResponse: { next?: string } | null = null;
		let validationSuccessful = false;

		try {
			// Validate the magic link (Ed25519 verification by backend)
			loginResponse = await authStore.validateMagicLink(magicToken);
			validationSuccessful = true;
			logger.info('[+layout] Magic link validation successful');
			logger.debug('[+layout] Login response received (handleMagicLinkValidation)', {
				hasNext: !!loginResponse.next,
				nextValue: loginResponse.next
			});

			// Mark session as valid after successful authentication
			sessionStatusStore.markValid();

			// CRITICAL: Start session monitoring after successful login
			await startMonitoringIfAuthenticated();
		} catch (error) {
			logger.error('[+layout] Magic link validation failed', error);
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
			// Handle next parameter and clean URL
			if (loginResponse.next) {
				logger.debug('[+layout] Processing next parameter (handleMagicLinkValidation)', {
					next: loginResponse.next
				});
				try {
					// Wait for BOTH crypto tokens AND access token to be available before navigating
					let tokensReady = false;
					let attempts = 0;
					const maxAttempts = 50; // 5 seconds max wait

					while (!tokensReady && attempts < maxAttempts) {
						const cipherToken = authStore.getCipherToken();
						const nonceToken = authStore.getNonceToken();
						const hmacKey = authStore.getHmacKey();
						const accessToken = authStore.getAccessToken();

						if (cipherToken && nonceToken && hmacKey && accessToken) {
							tokensReady = true;
						} else {
							await new Promise((resolve) => setTimeout(resolve, 100));
							attempts++;
						}
					}

					if (!tokensReady) {
						logger.warn('[+layout] Tokens not ready after waiting', {
							hasCipher: !!authStore.getCipherToken(),
							hasNonce: !!authStore.getNonceToken(),
							hasHmac: !!authStore.getHmacKey(),
							hasAccessToken: !!authStore.getAccessToken()
						});
						// Proceed anyway - components should handle missing tokens
					}

					// Parse next parameter JSON and create navigation URL
					const navigationUrl = await parseNextParameterJson(loginResponse.next);
					logger.debug('[+layout] Parsed navigation URL (handleMagicLinkValidation)', {
						navigationUrl
					});

					if (navigationUrl !== '/') {
						// Navigate to different route - goto() doesn't preserve query params
						logger.info('[+layout] Navigating to next destination (handleMagicLinkValidation)', {
							navigationUrl
						});
						await goto(navigationUrl);
					} else {
						// Staying on /, clean magiclink from URL
						logger.debug('[+layout] Staying on /, cleaning magiclink from URL');
						const newUrl = new window.URL(window.location.href);
						newUrl.searchParams.delete('magiclink');
						replaceState(newUrl.toString(), {});
					}
				} catch (error) {
					logger.error('[+layout] Failed to process next parameter', error);
					// Don't prevent successful authentication, just stay on current page
				}
			} else {
				// No next parameter, clean magiclink from URL
				logger.warn('[+layout] No next parameter, cleaning magiclink from URL');
				const newUrl = new window.URL(window.location.href);
				newUrl.searchParams.delete('magiclink');
				replaceState(newUrl.toString(), {});
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

	<!-- Update Button - appears when new frontend version available -->
	<UpdateButton />

	{@render children?.()}
</main>

<!-- Version Footer - Global for all pages -->
<VersionFooter />

<!-- Global Dialog Container -->
<DialogContainer />
