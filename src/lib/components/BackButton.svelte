<script lang="ts">
	import { goto } from '$app/navigation';
	import { t } from '$lib/stores/i18n';
	import Icon from './Icon.svelte';

	interface Props {
		to: string;
		class?: string;
	}

	let { to, class: className = '' }: Props = $props();

	function handleBack() {
		goto(to);
	}
</script>

<button
	onclick={handleBack}
	class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:bg-gray-800 dark:text-gray-200 dark:border-gray-600 dark:hover:bg-gray-700 transition-all duration-200 {className}"
>
	<Icon name="arrow-left" size="w-4 h-4" />
	{t('common.back')}
</button>