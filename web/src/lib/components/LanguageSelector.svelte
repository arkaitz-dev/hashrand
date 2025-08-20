<script lang="ts">
	import Icon from './Icon.svelte';
	import { currentLanguage } from '$lib/stores/i18n';
	import { isRTL } from '$lib/stores/rtl';

	let showDropdown = $state(false);
	
	// Language options with their flag icons
	const languages = [
		{ code: 'en', name: 'English', flag: 'uk' },
		{ code: 'es', name: 'Español', flag: 'spain' },
		{ code: 'pt', name: 'Português', flag: 'portugal' },
		{ code: 'fr', name: 'Français', flag: 'france' },
		{ code: 'de', name: 'Deutsch', flag: 'germany' },
		{ code: 'ru', name: 'Русский', flag: 'russia' },
		{ code: 'zh', name: '中文', flag: 'china' },
		{ code: 'ar', name: 'العربية', flag: 'saudi' },
		{ code: 'eu', name: 'Euskera', flag: 'basque' },
		{ code: 'ca', name: 'Català', flag: 'catalonia' },
		{ code: 'gl', name: 'Galego', flag: 'galicia' }
	];

	// Find language object by code
	function findLanguageByCode(code: string) {
		return languages.find(lang => lang.code === code) || languages[0];
	}

	// Initialize selected language based on current language store
	let selectedLanguage = $state(findLanguageByCode($currentLanguage));

	function toggleDropdown() {
		showDropdown = !showDropdown;
	}

	function selectLanguage(lang: typeof languages[0]) {
		selectedLanguage = lang;
		currentLanguage.set(lang.code);
		
		// Persist user preference
		if (typeof window !== 'undefined') {
			localStorage.setItem('preferred-language', lang.code);
		}
		
		showDropdown = false;
	}

	// Update selected language when store changes
	$effect(() => {
		selectedLanguage = findLanguageByCode($currentLanguage);
	});

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
		<Icon name={selectedLanguage.flag} size="w-5 h-5" />
	</button>

	{#if showDropdown}
		<div class="absolute top-full mt-2 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700 py-2 min-w-[200px] max-h-64 overflow-y-auto force-ltr {$isRTL ? 'left-0' : 'right-0'}">
			{#each languages as lang}
				<button
					class="w-full px-4 py-2 text-left hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3 text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors duration-150"
					onclick={() => selectLanguage(lang)}
					class:bg-gray-100={selectedLanguage.code === lang.code}
					class:dark:bg-gray-700={selectedLanguage.code === lang.code}
				>
					<Icon name={lang.flag} size="w-4 h-4" />
					<span class="text-sm font-medium">{lang.name}</span>
				</button>
			{/each}
		</div>
	{/if}
</div>