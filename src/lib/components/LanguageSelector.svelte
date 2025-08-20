<script lang="ts">
	import Icon from './Icon.svelte';

	// Simple language variable just for the demo
	let currentLanguage = $state('uk'); // Default to UK
	let showDropdown = $state(false);

	// Language options with their flag icons
	const languages = [
		{ code: 'uk', name: 'English', flag: 'uk' },
		{ code: 'spain', name: 'Español', flag: 'spain' },
		{ code: 'portugal', name: 'Português', flag: 'portugal' },
		{ code: 'france', name: 'Français', flag: 'france' },
		{ code: 'germany', name: 'Deutsch', flag: 'germany' },
		{ code: 'russia', name: 'Русский', flag: 'russia' },
		{ code: 'china', name: '中文', flag: 'china' },
		{ code: 'saudi', name: 'العربية', flag: 'saudi' },
		{ code: 'basque', name: 'Euskera', flag: 'basque' },
		{ code: 'catalonia', name: 'Català', flag: 'catalonia' },
		{ code: 'galicia', name: 'Galego', flag: 'galicia' }
	];

	function toggleDropdown() {
		showDropdown = !showDropdown;
	}

	function selectLanguage(lang: typeof languages[0]) {
		currentLanguage = lang.code;
		showDropdown = false;
	}

	// Reactive variable that updates when currentLanguage changes
	const currentFlag = $derived(languages.find(l => l.code === currentLanguage)?.flag || 'uk');

	// Close dropdown when clicking outside
	function handleClickOutside(event: MouseEvent) {
		if (showDropdown && !(event.target as Element).closest('.language-selector')) {
			showDropdown = false;
		}
	}
</script>

<svelte:window on:click={handleClickOutside} />

<div class="language-selector absolute top-4 right-14 z-50">
	<button
		class="p-2 rounded-xl bg-transparent border border-transparent shadow-none hover:bg-white hover:dark:bg-gray-800 hover:shadow-lg hover:border-gray-200 hover:dark:border-gray-700 active:bg-white active:dark:bg-gray-800 active:shadow-lg active:border-gray-200 active:dark:border-gray-700 transition-all duration-200 transform hover:scale-105 focus:outline-none flex items-center justify-center w-9 h-9"
		aria-label="Select language"
		onclick={toggleDropdown}
	>
		<Icon name={currentFlag} size="w-5 h-5" />
	</button>

	{#if showDropdown}
		<div class="absolute top-full right-0 mt-2 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700 py-2 min-w-[200px] max-h-64 overflow-y-auto">
			{#each languages as lang}
				<button
					class="w-full px-4 py-2 text-left hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3 text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors duration-150"
					onclick={() => selectLanguage(lang)}
					class:bg-gray-100={currentLanguage === lang.code}
					class:dark:bg-gray-700={currentLanguage === lang.code}
				>
					<Icon name={lang.flag} size="w-4 h-4" />
					<span class="text-sm font-medium">{lang.name}</span>
				</button>
			{/each}
		</div>
	{/if}
</div>