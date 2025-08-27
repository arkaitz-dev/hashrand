<script lang="ts">
	// Props
	export let id: string;
	export let label: string;
	export let value: string = '';
	export let placeholder: string = '';
	export let maxlength: number | undefined = undefined;
	export let required: boolean = false;
	export let disabled: boolean = false;
	export let isValid: boolean = true;
	export let errorMessage: string = '';
	export let type: string = 'text';
	export let className: string = '';

	// Handle input event
	function handleInput(event: Event) {
		const target = event.target as HTMLInputElement;
		value = target.value;
	}
</script>

<div class={className}>
	<label for={id} class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
		{label}
	</label>
	<input
		{id}
		{type}
		{value}
		{placeholder}
		{maxlength}
		{required}
		{disabled}
		on:input={handleInput}
		class="w-full px-3 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white disabled:bg-gray-50 disabled:text-gray-500 disabled:cursor-not-allowed {isValid
			? 'border-gray-300 dark:border-gray-600'
			: 'border-red-500 dark:border-red-400'}"
	/>
	{#if !isValid && errorMessage}
		<p class="text-red-500 text-sm mt-1">{errorMessage}</p>
	{/if}
</div>
