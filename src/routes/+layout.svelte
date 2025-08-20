<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { onMount } from 'svelte';
	import { currentRoute } from '$lib/stores/navigation';
	import { page } from '$app/stores';
	import ThemeToggle from '$lib/components/ThemeToggle.svelte';
	import LanguageSelector from '$lib/components/LanguageSelector.svelte';
	// Import theme store to ensure it's initialized
	import '$lib/stores/theme';
	
	let { children } = $props();

	// Update current route in store
	onMount(() => {
		const unsubscribe = page.subscribe(($page) => {
			currentRoute.set($page.url.pathname);
		});
		
		return unsubscribe;
	});
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<meta name="viewport" content="width=device-width, initial-scale=1.0" />
	<meta name="theme-color" content="#3b82f6" media="(prefers-color-scheme: light)" />
	<meta name="theme-color" content="#1e293b" media="(prefers-color-scheme: dark)" />
</svelte:head>

<main class="min-h-screen relative">
	<!-- Language Selector and Theme Toggle move with scroll -->
	<LanguageSelector />
	<ThemeToggle />
	
	{@render children?.()}
</main>
