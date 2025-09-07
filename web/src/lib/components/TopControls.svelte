<script lang="ts">
	import Icon from './Icon.svelte';
	import AuthStatusButton from './AuthStatusButton.svelte';
	import { theme, toggleTheme } from '$lib/stores/theme';
	import { currentLanguage } from '$lib/stores/i18n';
	import { isRTL } from '$lib/stores/rtl';
	import { languages, findLanguageByCode } from '$lib/languageConfig';

	let showDropdown = $state(false);
	let isTransitioning = $state(false);
	let hasActiveSession = $state(false);

	// Check if user has an active session (user_id + access_token)
	function checkActiveSession() {
		if (typeof window === 'undefined') return false;

		const authUser = localStorage.getItem('auth_user');
		const accessToken = localStorage.getItem('access_token');

		// Both must exist
		if (!authUser || !accessToken) return false;

		try {
			// Validate auth_user structure
			const user = JSON.parse(authUser);
			if (!user.user_id) return false;

			// Check if token is not expired
			if (user.expiresAt) {
				const expiresAt = new Date(user.expiresAt);
				if (expiresAt <= new Date()) return false;
			}

			return true;
		} catch {
			return false;
		}
	}

	// Update session status reactively
	$effect(() => {
		hasActiveSession = checkActiveSession();

		// Set up periodic check for session expiry
		const interval = setInterval(() => {
			hasActiveSession = checkActiveSession();
		}, 5000); // Check every 5 seconds

		return () => clearInterval(interval);
	});

	// Initialize selected language based on current language store
	let selectedLanguage = $state(findLanguageByCode($currentLanguage));

	function toggleDropdown() {
		showDropdown = !showDropdown;
	}

	function selectLanguage(lang: (typeof languages)[0]) {
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

	// Handle RTL transition effect
	let previousRTL = $state($isRTL);
	$effect(() => {
		if (previousRTL !== $isRTL) {
			isTransitioning = true;
			// Immediately start fade in after position change
			setTimeout(() => {
				isTransitioning = false;
				previousRTL = $isRTL;
			}, 50); // Very short delay just to let the position change take effect
		}
	});

	// Close dropdown when clicking outside
	function handleClickOutside(event: MouseEvent) {
		if (showDropdown && !(event.target as Element).closest('.top-controls')) {
			showDropdown = false;
		}
	}
</script>

<svelte:window on:click={handleClickOutside} />

<!-- Top Controls Container -->
<div
	class="top-controls absolute top-0.5 md:top-4 z-50 flex items-center gap-0.5 sm:gap-1 bg-gray-200/90 dark:bg-gray-800/80 backdrop-blur-sm rounded-xl sm:rounded-2xl p-0.5 sm:p-1 md:p-1 shadow-lg border border-gray-400/50 dark:border-gray-700/50 transition-opacity duration-[1500ms] {$isRTL
		? 'left-0.5 md:left-4'
		: 'right-0.5 md:right-4'} {isTransitioning ? 'opacity-0' : 'opacity-100'}"
>
	<!-- Language Selector -->
	<div class="relative">
		<button
			class="p-1.5 sm:p-2 rounded-lg sm:rounded-xl bg-transparent border border-transparent shadow-none hover:bg-white hover:dark:bg-gray-800 hover:shadow-lg hover:border-gray-200 hover:dark:border-gray-700 active:bg-white active:dark:bg-gray-800 active:shadow-lg active:border-gray-200 active:dark:border-gray-700 transition-colors duration-[750ms] transition-shadow duration-[750ms] transition-border-colors duration-[750ms] transform hover:scale-105 focus:outline-none flex items-center justify-center w-9 h-9 sm:w-12 sm:h-12"
			class:bg-white={showDropdown}
			class:dark:bg-gray-800={showDropdown}
			class:shadow-lg={showDropdown}
			class:border-gray-200={showDropdown}
			class:dark:border-gray-700={showDropdown}
			class:scale-105={showDropdown}
			aria-label="Select language"
			onclick={toggleDropdown}
		>
			<Icon name={selectedLanguage.flag} size="w-6 h-6 sm:w-8 sm:h-8" placeholder="auto" />
		</button>

		{#if showDropdown}
			<div
				class="absolute top-full mt-2 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700 py-2 min-w-[200px] max-h-64 overflow-y-auto force-ltr {$isRTL
					? 'left-0'
					: 'right-0'}"
			>
				{#each languages as lang}
					<button
						class="w-full px-4 py-2 text-left hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-3 text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors duration-150"
						onclick={() => selectLanguage(lang)}
						class:bg-gray-100={selectedLanguage.code === lang.code}
						class:dark:bg-gray-700={selectedLanguage.code === lang.code}
					>
						<Icon name={lang.flag} size="w-5 h-5" placeholder="auto" />
						<span class="text-sm font-medium">{lang.name}</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>

	<!-- Theme Toggle -->
	<button
		class="p-1.5 sm:p-2 rounded-lg sm:rounded-xl bg-transparent border border-transparent shadow-none hover:bg-white hover:dark:bg-gray-800 hover:shadow-lg hover:border-gray-200 hover:dark:border-gray-700 active:bg-white active:dark:bg-gray-800 active:shadow-lg active:border-gray-200 active:dark:border-gray-700 transition-colors duration-[750ms] transition-shadow duration-[750ms] transition-border-colors duration-[750ms] transform hover:scale-105 focus:outline-none flex items-center justify-center w-9 h-9 sm:w-12 sm:h-12"
		onclick={toggleTheme}
		aria-label={$theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
		title={$theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'}
	>
		<div class="text-gray-700 dark:text-gray-300 transition-all duration-150 transform">
			{#if $theme === 'dark'}
				<Icon name="moon" size="w-4 h-4 sm:w-5 sm:h-5" />
			{:else}
				<Icon name="sun" size="w-4 h-4 sm:w-5 sm:h-5" />
			{/if}
		</div>
	</button>

	<!-- Auth Status Button (only when authenticated) -->
	{#if hasActiveSession}
		<AuthStatusButton />
	{/if}
</div>
