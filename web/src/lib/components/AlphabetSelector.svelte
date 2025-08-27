<script lang="ts">
	import type { AlphabetType } from '$lib/types';

	// Interface for alphabet options
	interface AlphabetOption {
		value: AlphabetType | string;
		label: string;
		description: string;
	}

	// Props
	export let value: AlphabetType | string | undefined;
	export let options: AlphabetOption[];
	export let label: string;
	export let id: string = 'alphabet';
	export let className: string = '';
	export let disabled: boolean = false;
	export let onChange: ((value: AlphabetType | string | undefined) => void) | undefined = undefined;

	// Handle change event
	function handleChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		value = target.value as AlphabetType | string | undefined;
		if (onChange) {
			onChange(value);
		}
	}

	// Find current option for description
	$: currentOption = options.find((o) => o.value === value);
</script>

<div class={className}>
	<label for={id} class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
		{label}
	</label>
	<select
		{id}
		{disabled}
		value={value ?? ''}
		on:change={handleChange}
		class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white disabled:bg-gray-50 disabled:text-gray-500 disabled:cursor-not-allowed"
	>
		{#each options as option}
			<option value={option.value}>{option.label}</option>
		{/each}
	</select>
	{#if currentOption && currentOption.description}
		<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
			{currentOption.description}
		</p>
	{/if}
</div>
