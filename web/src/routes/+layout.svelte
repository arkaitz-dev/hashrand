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
	import { base58 } from '@scure/base';
	import { flashMessagesStore } from '$lib/stores/flashMessages';

	let { children } = $props();

	// Update current route in store and initialize sprite detection
	onMount(() => {
		const unsubscribe = page.subscribe(($page) => {
			currentRoute.set($page.url.pathname);

			// Check for magic link parameter
			const magicToken = $page.url.searchParams.get('magiclink');
			if (magicToken) {
				const nextParam = $page.url.searchParams.get('next');
				handleMagicLinkValidation(magicToken, nextParam, $page);
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
		nextParam: string | null,
		currentPage: { url: globalThis.URL }
	) {
		try {
			// Show step 1: Magic link detected
			flashMessagesStore.addMessage('🔗 Magic link detectado, validando...');
			
			// Validate the magic link (it's self-contained, no email needed)
			const loginResponse = await authStore.validateMagicLink(magicToken);
			
			// Show step 2: Authentication successful
			flashMessagesStore.addMessage(`✅ Autenticado: token recibido (expires: ${loginResponse.expires_in}s)`);
			flashMessagesStore.addMessage(`👤 Usuario: ${loginResponse.username}`);


			// Remove magiclink parameter from URL
			const newUrl = new globalThis.URL(currentPage.url);
			newUrl.searchParams.delete('magiclink');
			newUrl.searchParams.delete('next');

			// Update URL without page reload
			globalThis.window?.history?.replaceState({}, '', newUrl.toString());

			// Handle next parameter if present  
			if (nextParam) {
				flashMessagesStore.addMessage('🔄 Parámetro next detectado, redirigiendo...');
				await handlePostAuthRedirect(nextParam);
			} else {
				flashMessagesStore.addMessage('✅ Autenticación completada');
			}
		} catch (error) {

			// Remove failed magiclink parameter from URL
			const newUrl = new globalThis.URL(currentPage.url);
			newUrl.searchParams.delete('magiclink');
			newUrl.searchParams.delete('next');
			globalThis.window?.history?.replaceState({}, '', newUrl.toString());

			// Redirect to home page
			goto('/');
		}
	}

	/**
	 * Handle post-authentication redirect with next parameter
	 */
	async function handlePostAuthRedirect(nextBase58: string): Promise<void> {
		try {
			// Decode Base58 next parameter
			flashMessagesStore.addMessage('📦 Decodificando parámetros next (Base58)...');
			const bytes = base58.decode(nextBase58);
			const decoder = new globalThis.TextDecoder();
			const jsonString = decoder.decode(bytes);
			const nextObject = JSON.parse(jsonString);
			
			flashMessagesStore.addMessage(`🎯 Endpoint: ${nextObject.endpoint}, params: ${Object.keys(nextObject).length} items`);

			// Build result URL with parameters from nextObject
			const resultUrl = new globalThis.URL('/result', window.location.origin);

			// Add parameters to result URL
			if (nextObject.endpoint) resultUrl.searchParams.set('endpoint', nextObject.endpoint);
			if (nextObject.length) resultUrl.searchParams.set('length', nextObject.length.toString());
			if (nextObject.alphabet) resultUrl.searchParams.set('alphabet', nextObject.alphabet);
			if (nextObject.prefix) resultUrl.searchParams.set('prefix', nextObject.prefix);
			if (nextObject.suffix) resultUrl.searchParams.set('suffix', nextObject.suffix);
			if (nextObject.seed) resultUrl.searchParams.set('seed', nextObject.seed);
			if (nextObject.raw !== undefined)
				resultUrl.searchParams.set('raw', nextObject.raw.toString());

			const finalUrl = `/result?${resultUrl.searchParams.toString()}`;
			flashMessagesStore.addMessage(`🚀 Redirigiendo a: ${finalUrl}`);
			
			// Verify auth is saved in localStorage before redirecting
			const authData = localStorage.getItem('auth_user');
			if (!authData) {
				flashMessagesStore.addMessage('❌ Error: autenticación no guardada en localStorage');
				await goto('/');
				return;
			}
			
			// Navigate to result page with parameters
			await goto(finalUrl);
		} catch (error) {
			flashMessagesStore.addMessage(`❌ Error decodificando next: ${error}`);
			// Fallback to home page on error
			await goto('/');
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
