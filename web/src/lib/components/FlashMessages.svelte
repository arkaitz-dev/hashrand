<script lang="ts">
	import { isRTL } from '$lib/stores/rtl';
	import { flashMessagesStore } from '$lib/stores/flashMessages';

	// Handle close button click
	function handleClose() {
		flashMessagesStore.clear();
	}

	// Only show when there are messages
	let showMessages = $derived($flashMessagesStore.length > 0);
</script>

{#if showMessages}
	<div class="max-w-4xl mx-auto mb-8">
		<div
			class="bg-blue-50 dark:bg-blue-900 border border-blue-200 dark:border-blue-700 rounded-lg p-4 shadow-md relative"
		>
			<!-- Close button -->
			<button
				on:click={handleClose}
				class="absolute {$isRTL ? 'left-4' : 'right-4'} top-4 p-1 text-blue-600 dark:text-blue-300 hover:text-blue-800 dark:hover:text-blue-100 transition-colors"
				aria-label="Cerrar mensajes"
			>
				<svg
					class="w-5 h-5"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M6 18L18 6M6 6l12 12"
					></path>
				</svg>
			</button>

			<!-- Messages content -->
			<div class="{$isRTL ? 'ml-8' : 'mr-8'}">
				{#each $flashMessagesStore as message, index}
					<div
						class="text-sm text-blue-700 dark:text-blue-300 {index > 0 ? 'mt-2' : ''}"
					>
						{message.content}
					</div>
				{/each}
			</div>
		</div>
	</div>
{/if}