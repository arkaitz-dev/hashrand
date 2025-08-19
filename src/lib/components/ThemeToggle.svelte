<script lang="ts">
	import { theme, toggleTheme } from '$lib/stores/theme';
	import Icon from './Icon.svelte';
</script>

<button
	class="absolute top-4 right-4 z-50 p-3 rounded-xl bg-transparent border border-transparent shadow-none hover:bg-white hover:dark:bg-gray-800 hover:shadow-lg hover:border-gray-200 hover:dark:border-gray-700 active:bg-white active:dark:bg-gray-800 active:shadow-lg active:border-gray-200 active:dark:border-gray-700 transition-all duration-200 transform hover:scale-105 focus:outline-none"
	onclick={toggleTheme}
	aria-label={$theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
	title={$theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
>
	<div class="text-gray-700 dark:text-gray-300 transition-all duration-300 transform">
		{#if $theme === 'dark'}
			<Icon name="moon" size="w-5 h-5" />
		{:else}
			<Icon name="sun" size="w-5 h-5" />
		{/if}
	</div>
</button>