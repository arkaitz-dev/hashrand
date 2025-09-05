<script lang="ts">
	import { _ } from '../stores/i18n';
	import { isRTL } from '../stores/rtl';

	// Props
	export let onClose: () => void;
	export let onSeedChoice: (keepSeed: boolean) => void;
</script>

<!-- Header -->
<div class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700">
	<h2
		class="text-xl font-semibold text-gray-900 dark:text-white {$isRTL
			? 'text-right'
			: 'text-left'}"
	>
		{$_('common.reuseSeedTitle')}
	</h2>
	<button
		onclick={onClose}
		class="p-1 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
		aria-label={$_('common.close')}
	>
		<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"
			></path>
		</svg>
	</button>
</div>

<!-- Body Content -->
<div class="p-6">
	<p class="text-gray-600 dark:text-gray-300">
		{$_('common.reuseSeedMessage')}
	</p>
</div>

<!-- Footer Actions -->
<div
	class="flex items-center justify-between px-6 py-4 border-t border-gray-200 dark:border-gray-700 {$isRTL
		? 'flex-row-reverse'
		: ''}"
>
	<button
		onclick={() => onSeedChoice(false)}
		class="px-4 py-2 text-gray-600 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg font-medium transition-colors"
	>
		{$_('common.generateNewSeed')}
	</button>
	<button
		onclick={() => onSeedChoice(true)}
		class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
	>
		{$_('common.keepSameSeed')}
	</button>
</div>
