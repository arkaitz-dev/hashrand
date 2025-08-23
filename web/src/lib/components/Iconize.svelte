<script lang="ts">
	import Icon from './Icon.svelte';
	import { isRTL } from '$lib/stores/rtl';

	interface IconizeConfig {
		icon?: string;                   // Icon name from sprite (optional)
		emoji?: string;                  // Emoji/text content (optional)
		iconSize?: string;              // Icon size (e.g., 'w-4 h-4') or text size (e.g., 'text-3xl')
		rtlIcon?: string;               // Specific icon for RTL mode (optional)
		rtlEmoji?: string;              // Specific emoji for RTL mode (optional)
		spacing?: string;               // Spacing between icon and content
		invertposition?: boolean;       // Invert position - slot first, then icon (default: false)
	}

	interface Props {
		conf: IconizeConfig;
		class?: string;                 // Additional wrapper classes
	}

	let { conf, class: wrapperClass = '' }: Props = $props();

	// Determine which icon/emoji to use based on RTL state
	const currentIcon = $derived($isRTL && conf.rtlIcon ? conf.rtlIcon : conf.icon);
	const currentEmoji = $derived($isRTL && conf.rtlEmoji ? conf.rtlEmoji : conf.emoji);
	const useEmoji = $derived(!!currentEmoji);
	const iconSize = conf.iconSize || (useEmoji ? 'text-3xl' : 'w-4 h-4');
	const spacing = conf.spacing || 'gap-2';
	const invertposition = conf.invertposition || false;
	
</script>

<!-- Simple wrapper that just shows icon/emoji and content -->
<div class="inline-flex items-center {spacing} {wrapperClass}">
	{#if invertposition}
		<slot />
	{/if}
	
	{#if useEmoji}
		<span class="inline-block {iconSize}" style="direction: ltr;">
			{currentEmoji}
		</span>
	{:else if currentIcon}
		<Icon name={currentIcon} size={iconSize} />
	{/if}
	
	{#if !invertposition}
		<slot />
	{/if}
</div>