<script lang="ts">
	import { goto } from '$app/navigation';
	import { _ } from '$lib/stores/i18n';
	import { isRTL } from '$lib/stores/rtl';
	import Iconize from '$lib/components/Iconize.svelte';
	import { logger } from '$lib/utils/logger';

	// Props
	export let path: string;
	export let icon: string;
	export let title: string;
	export let description: string;

	function handleClick() {
		logger.info(`[Click] Menu card: ${title} (${path})`);
		logger.info(`[Navigation] Redirecting to: ${path}`);
		goto(path);
	}
</script>

<button
	class="w-full bg-white dark:bg-gray-800 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 transform hover:-translate-y-1 cursor-pointer border border-gray-200 dark:border-gray-700 {$isRTL
		? 'text-right'
		: 'text-left'}"
	onclick={handleClick}
	aria-label="{$_('common.navigateTo')} {title}"
>
	<div class="p-6">
		<div class="mb-4 flex items-center gap-3">
			<Iconize
				conf={{
					emoji: icon,
					iconSize: 'text-3xl'
				}}
			></Iconize>
			<h2 class="text-xl font-semibold text-gray-900 dark:text-white">
				{title}
			</h2>
		</div>
		<p class="text-gray-600 dark:text-gray-300 leading-relaxed">
			{description}
		</p>
		<div
			class="mt-4 inline-flex items-center text-blue-600 dark:text-blue-400 text-sm font-medium {$isRTL
				? 'rtl-float-right'
				: 'rtl-float-left'}"
		>
			<Iconize
				conf={{
					icon: 'arrow-right',
					rtlIcon: 'arrow-left',
					iconSize: 'w-4 h-4',
					spacing: 'gap-1',
					invertposition: true
				}}
			>
				{$_('common.choose')}
			</Iconize>
		</div>
		<div class="clear-both"></div>
	</div>
</button>
