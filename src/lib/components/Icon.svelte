<script lang="ts">
	import { onMount } from 'svelte';

	/**
	 * Icon component that uses SVG sprite for optimized icon rendering
	 * @param name - The icon name (without 'icon-' prefix)
	 * @param size - Icon size class (Tailwind classes)
	 * @param class - Additional CSS classes
	 */
	interface Props {
		name: string;
		size?: string;
		class?: string;
	}

	let { name, size = 'w-5 h-5', class: className = '' }: Props = $props();

	// Build the full icon ID for the sprite
	const iconId = `/icons-sprite.svg#icon-${name}`;

	// Ensure sprite is loaded - this will be cached after first load
	onMount(() => {
		// The browser will handle caching of the sprite file automatically
		// No need to manually inject it into DOM
	});
</script>

<svg class="{size} {className}" aria-hidden="true">
	<use href="{iconId}"></use>
</svg>