<script lang="ts">
	import Icon from './Icon.svelte';
	import { isRTL } from '$lib/stores/rtl';

	interface IconizeConfig {
		icon?: string; // Icon name from sprite (optional)
		emoji?: string; // Emoji/text content (optional)
		iconSize?: string; // Icon size (e.g., 'w-4 h-4') or text size (e.g., 'text-3xl')
		rtlIcon?: string; // Specific icon for RTL mode (optional)
		rtlEmoji?: string; // Specific emoji for RTL mode (optional)
		spacing?: string; // Spacing between icon and content
		invertposition?: boolean; // Invert position - slot first, then icon (default: false)
	}

	interface Props {
		conf: IconizeConfig;
		class?: string; // Additional wrapper classes
		children?: import('svelte').Snippet;
	}

	let { conf, class: wrapperClass = '', children }: Props = $props();

	// Determine which icon/emoji to use based on RTL state
	const currentIcon = $derived($isRTL && conf.rtlIcon ? conf.rtlIcon : conf.icon);
	const currentEmoji = $derived($isRTL && conf.rtlEmoji ? conf.rtlEmoji : conf.emoji);
	const useEmoji = $derived(!!currentEmoji);
	const iconSize = $derived(conf.iconSize || (currentEmoji ? 'text-3xl' : 'w-4 h-4'));
	const spacing = conf.spacing || 'gap-2';
	const invertposition = conf.invertposition || false;

	// Function to get the appropriate placeholder based on icon and RTL state
	function getIconPlaceholder(iconName: string): string {
		if (iconName === 'arrow-left' || iconName === 'arrow-right') {
			// Always use '>' for choose/forward buttons - the visual direction is handled by icon choice
			return '>';
		}
		return 'auto'; // For all other icons, use auto to get emoji from iconEmojis.ts
	}
</script>

<!-- Simple wrapper that just shows icon/emoji and content -->
<div class="inline-flex items-center {spacing} {wrapperClass}">
	{#if invertposition}
		{@render children?.()}
	{/if}

	{#if useEmoji}
		<span class="inline-block {iconSize}" style="direction: ltr;">
			{currentEmoji}
		</span>
	{:else if currentIcon}
		<Icon name={currentIcon} size={iconSize} placeholder={getIconPlaceholder(currentIcon)} />
	{/if}

	{#if !invertposition}
		{@render children?.()}
	{/if}
</div>
