<script lang="ts">
	import Icon from './Icon.svelte';

	interface Props {
		size?: 'sm' | 'md' | 'lg';
		class?: string;
	}

	let { size = 'md', class: className = '' }: Props = $props();

	const sizeClasses = {
		sm: 'w-4 h-4',
		md: 'w-6 h-6',
		lg: 'w-8 h-8'
	};
</script>

<div class="inline-flex items-center justify-center">
	<Icon
		name="spinner"
		size={`${sizeClasses[size]} animate-spin`}
		class={className || 'text-blue-600 dark:text-blue-400'}
	/>
</div>
