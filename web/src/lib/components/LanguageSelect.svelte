<script lang="ts">
	import Icon from './Icon.svelte';
	import { languages, findLanguageByCode } from '$lib/languageConfig';

	// Props
	interface Props {
		value: string;
		onchange?: (code: string) => void;
		id?: string;
		disabled?: boolean;
	}

	let { value = $bindable(), onchange, id, disabled = false }: Props = $props();

	let showDropdown = $state(false);
	let selectedLanguage = $derived(findLanguageByCode(value));

	function toggleDropdown() {
		if (!disabled) {
			showDropdown = !showDropdown;
		}
	}

	function selectLanguage(lang: (typeof languages)[0]) {
		value = lang.code;
		if (onchange) {
			onchange(lang.code);
		}
		showDropdown = false;
	}

	// Close dropdown when clicking outside
	function handleClickOutside(event: MouseEvent) {
		if (showDropdown && !(event.target as Element).closest('.language-select')) {
			showDropdown = false;
		}
	}
</script>

<svelte:window on:click={handleClickOutside} />

<div class="language-select relative w-full">
	<button
		type="button"
		{id}
		{disabled}
		class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent dark:bg-gray-700 dark:text-white bg-white text-gray-900 flex items-center justify-between gap-3 hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
		aria-label="Select language"
		aria-expanded={showDropdown}
		onclick={toggleDropdown}
	>
		<div class="flex items-center gap-3">
			<Icon name={selectedLanguage.flag} size="w-5 h-5" placeholder="auto" />
			<span class="text-sm font-medium">{selectedLanguage.name}</span>
		</div>
		<svg
			class="w-5 h-5 text-gray-400 transition-transform duration-200"
			class:rotate-180={showDropdown}
			fill="none"
			stroke="currentColor"
			viewBox="0 0 24 24"
		>
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
		</svg>
	</button>

	{#if showDropdown}
		<div
			class="absolute top-full mt-1 w-full bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700 py-2 max-h-64 overflow-y-auto z-50"
		>
			{#each languages as lang}
				<button
					type="button"
					class="w-full px-4 py-2 text-left hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3 text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors duration-150"
					onclick={() => selectLanguage(lang)}
					class:bg-indigo-50={value === lang.code}
					class:dark:bg-indigo-900={value === lang.code}
					class:text-indigo-600={value === lang.code}
					class:dark:text-indigo-400={value === lang.code}
				>
					<Icon name={lang.flag} size="w-5 h-5" placeholder="auto" />
					<span class="text-sm font-medium">{lang.name}</span>
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	.rotate-180 {
		transform: rotate(180deg);
	}
</style>
