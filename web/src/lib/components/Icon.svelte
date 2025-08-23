<script lang="ts">
	import { spriteState } from '$lib/stores/spriteLoader';
	import { getIconEmoji, hasProperIconEmoji } from '$lib/flagEmojis';

	/**
	 * Icon component that uses SVG sprite for optimized icon rendering
	 * @param name - The icon name (without 'icon-' prefix)
	 * @param size - Icon size class (Tailwind classes)
	 * @param class - Additional CSS classes
	 * @param placeholder - Custom placeholder (UTF emoji, 'spinner', or 'auto')
	 *                     'auto' uses flag emoji for flags, spinner for others
	 */
	interface Props {
		name: string;
		size?: string;
		class?: string;
		placeholder?: string | 'spinner' | 'auto';
	}

	let { name, size = 'w-5 h-5', class: className = '', placeholder = 'auto' }: Props = $props();

	// Build the full icon ID for the sprite - reactive to name changes
	const iconId = $derived(`#icon-${name}`);

	// Determine what placeholder to show
	const computedPlaceholder = $derived(() => {
		if (placeholder === 'auto') {
			// For any icon, try to use UTF emoji, otherwise use spinner
			if (hasProperIconEmoji(name)) {
				return getIconEmoji(name);
			}
			return 'spinner';
		}
		return placeholder;
	});
</script>

{#if $spriteState.loaded}
	<!-- Sprite is loaded - show the icon -->
	<svg class="{size} {className}" aria-hidden="true">
		<use href={iconId}></use>
	</svg>
{:else}
	<!-- Show placeholder while loading -->
	{#if computedPlaceholder() === 'spinner'}
		<!-- Spinner placeholder -->
		<div class="{size} {className} flex items-center justify-center">
			<svg class="animate-spin h-4 w-4 text-gray-400" viewBox="0 0 24 24" aria-hidden="true">
				<circle
					class="opacity-25"
					cx="12"
					cy="12"
					r="10"
					stroke="currentColor"
					stroke-width="4"
					fill="none"
				></circle>
				<path
					class="opacity-75"
					fill="currentColor"
					d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
				></path>
			</svg>
		</div>
	{:else}
		<!-- UTF emoji placeholder -->
		<div class="{size} {className} flex items-center justify-center text-lg">
			{computedPlaceholder()}
		</div>
	{/if}
{/if}
