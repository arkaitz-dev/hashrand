<script lang="ts">
	import { _ } from '$lib/stores/i18n';
	import LoadingSpinner from './LoadingSpinner.svelte';
	import Iconize from './Iconize.svelte';

	// Props
	export let type: 'submit' | 'button' = 'submit';
	export let disabled: boolean = false;
	export let loading: boolean = false;
	export let onClick: (() => void) | undefined = undefined;
	export let text: string;
	export let loadingText: string = $_('common.loading') + '...';

	// Optional custom styles
	export let buttonClass: string =
		'flex-1 text-white px-6 py-4 rounded-lg font-semibold border-none transition-all duration-200 flex items-center justify-center';
	export let enabledClass: string = 'bg-blue-600 hover:bg-blue-700 hover:shadow-lg cursor-pointer';
	export let disabledClass: string = 'bg-gray-400 cursor-not-allowed';

	// Handle click event
	function handleClick() {
		if (!disabled && !loading && onClick) {
			onClick();
		}
	}
</script>

<button
	{type}
	{disabled}
	on:click={handleClick}
	class="{buttonClass} {disabled || loading ? disabledClass : enabledClass}"
>
	{#if loading}
		<LoadingSpinner size="sm" class="mr-2" />
		{loadingText}
	{:else}
		<Iconize
			conf={{
				icon: 'play',
				iconSize: 'w-5 h-5',
				spacing: 'gap-2'
			}}
		>
			{text}
		</Iconize>
	{/if}
</button>
