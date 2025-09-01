<script lang="ts">
	/**
	 * Simple Dialog Component (Svelte 5 Compatible)
	 *
	 * Simplified version of UniversalDialog with backdrop blur
	 * that works correctly with Svelte 5 runes
	 */

	import { createEventDispatcher } from 'svelte';
	import { isRTL, textDirection } from '$lib/stores/rtl';
	import { _ } from '$lib/stores/i18n';

	// Props
	interface Props {
		show: boolean;
		title?: string;
		size?: 'sm' | 'md' | 'lg' | 'xl';
		closable?: boolean;
		closeOnBackdrop?: boolean;
		children?: import('svelte').Snippet;
		actions?: import('svelte').Snippet;
	}

	let {
		show = $bindable(),
		title = '',
		size = 'md',
		closable = true,
		closeOnBackdrop = true,
		children,
		actions
	}: Props = $props();

	// Events
	const dispatch = createEventDispatcher<{
		close: void;
	}>();

	// Size variants
	const sizeClasses = {
		sm: 'max-w-md',
		md: 'max-w-lg',
		lg: 'max-w-2xl',
		xl: 'max-w-4xl'
	};

	// Close dialog
	function close() {
		show = false;
		dispatch('close');
	}

	// Handle backdrop click
	function handleBackdropClick(event: MouseEvent) {
		if (closeOnBackdrop && closable && event.target === event.currentTarget) {
			close();
		}
	}

	// Handle escape key
	function handleKeydown(event: globalThis.KeyboardEvent) {
		if (event.key === 'Escape' && show && closable) {
			event.preventDefault();
			close();
		}
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
		dir={$textDirection}
		onclick={handleBackdropClick}
	>
		<!-- Dialog Content -->
		<div
			class="relative w-full {sizeClasses[
				size
			]} transform rounded-xl bg-white dark:bg-gray-800 shadow-2xl border border-gray-200 dark:border-gray-700 transition-all duration-300"
			onclick={(e) => e.stopPropagation()}
		>
			<!-- Header -->
			{#if title || closable}
				<div
					class="flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700"
				>
					{#if title}
						<h2
							class="text-xl font-semibold text-gray-900 dark:text-white {$isRTL
								? 'text-right'
								: 'text-left'}"
						>
							{title}
						</h2>
					{:else}
						<div></div>
					{/if}

					{#if closable}
						<button
							onclick={close}
							class="p-1 rounded-lg text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
							aria-label={$_('common.close')}
						>
							<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M6 18L18 6M6 6l12 12"
								></path>
							</svg>
						</button>
					{/if}
				</div>
			{/if}

			<!-- Body Content -->
			<div class="p-6">
				{@render children?.()}
			</div>

			<!-- Footer Actions -->
			{#if actions}
				<div
					class="flex items-center justify-end gap-3 px-6 py-4 border-t border-gray-200 dark:border-gray-700 {$isRTL
						? 'flex-row-reverse'
						: ''}"
				>
					{@render actions()}
				</div>
			{/if}
		</div>
	</div>
{/if}
