<script lang="ts">
	import Icon from './Icon.svelte';
	import { isRTL } from '$lib/stores/rtl';

	// Props que acepta un button normal
	export let type: 'button' | 'submit' | 'reset' = 'button';
	export let disabled = false;
	
	// Props específicos del wrapper
	export let icon: string | undefined = undefined;
	export let iconClass = '';
	export let rtlAware = true; // Por defecto siempre RTL-aware
</script>

<button
	{type}
	{disabled}
	class={$$props.class || ''}
	style="direction: {$isRTL ? 'rtl' : 'ltr'}"
	on:click
	{...$$restProps}
>
	{#if icon}
		<Icon 
			name={icon} 
			size={$$props.iconSize || 'w-4 h-4'}
			class={iconClass}
		/>
	{/if}
	
	<slot />
</button>