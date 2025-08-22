<script lang="ts">
	import Icon from './Icon.svelte';
	import { isRTL } from '$lib/stores/rtl';

	/**
	 * Iconize - Universal wrapper that adds RTL-aware icon positioning to any content
	 * 
	 * Usage examples:
	 * <Iconize conf={{icon: "arrow-left"}}>Back</Iconize>
	 * <Iconize conf={{icon: "check"}}><button class="btn">Save</button></Iconize>
	 */

	interface IconizeConfig {
		icon?: string;                   // Icon name from sprite (optional)
		emoji?: string;                  // Emoji/text content (optional)
		iconClass?: string;             // Additional CSS classes for icon
		iconSize?: string;              // Icon size (e.g., 'w-4 h-4') or text size (e.g., 'text-3xl')
		rtlIcon?: string;               // Specific icon for RTL mode (optional)
		rtlEmoji?: string;              // Specific emoji for RTL mode (optional)
		rtlIconClass?: string;          // Specific classes for RTL icon
		position?: 'start' | 'end';     // Icon position (start=left in LTR, right in RTL)
		spacing?: string;               // Spacing between icon and content
		rtlAware?: boolean;             // Enable RTL behavior (default: true)
	}

	interface Props {
		conf: IconizeConfig;
		class?: string;                 // Additional wrapper classes
	}

	let { conf, class: wrapperClass = '' }: Props = $props();

	// Default values
	const config = {
		iconClass: '',
		iconSize: conf.emoji ? 'text-3xl' : 'w-4 h-4',  // Different defaults for emoji vs icon
		rtlIcon: conf.icon,             // Use same icon for RTL by default
		rtlEmoji: conf.emoji,           // Use same emoji for RTL by default
		rtlIconClass: conf.iconClass || '',
		position: 'start' as const,
		spacing: 'gap-2',
		rtlAware: true,
		...conf
	};

	// Determine if using emoji or icon
	const useEmoji = $derived(!!config.emoji);
	
	// Determine which icon/emoji and classes to use based on RTL state
	const currentIcon = $derived($isRTL && config.rtlAware ? config.rtlIcon : config.icon);
	const currentEmoji = $derived($isRTL && config.rtlAware ? config.rtlEmoji : config.emoji);
	const currentIconClass = $derived($isRTL && config.rtlAware ? config.rtlIconClass : config.iconClass);

	// Determine icon position based on RTL state and position setting
	const shouldIconBeFirst = $derived(() => {
		if (!config.rtlAware) return config.position === 'start';
		
		const result = $isRTL ? config.position !== 'start' : config.position === 'start';
		return result;
	});
	

	// Build wrapper classes
	const wrapperClasses = $derived(`inline-flex items-center ${config.spacing} ${wrapperClass}`);
	
</script>

<!-- 
	For simplicity in this first implementation, we always wrap the content.
	Future enhancement could detect single HTML elements and inject icons directly.
	This covers both cases: text content and HTML elements.
-->
<!-- Container with debug info outside -->
<span class="inline-block">
	<span class={wrapperClasses}>
		<!-- Icon or Emoji - let flexbox handle RTL automatically -->
		{#if useEmoji}
			<span 
				class="inline-block {config.iconSize} {currentIconClass}" 
				style="direction: ltr;"
			>
				{currentEmoji}
			</span>
		{:else if currentIcon}
			<Icon 
				name={currentIcon} 
				size={config.iconSize}
				class={currentIconClass}
			/>
		{/if}
		
		<!-- Slot content -->
		<span>
			<slot />
		</span>
	</span>
	
</span>