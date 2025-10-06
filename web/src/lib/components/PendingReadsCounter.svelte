<script lang="ts">
	import { onMount } from 'svelte';
	import { api } from '$lib/api';
	import { _ } from '$lib/stores/i18n';

	/**
	 * Reactive counter for shared secret pending reads
	 * Automatically confirms read for receiver role and updates display
	 */

	// Props
	interface Props {
		initialPendingReads: number;
		maxReads: number;
		hash: string | undefined;
		role: string;
	}

	let { initialPendingReads, maxReads, hash, role }: Props = $props();

	// Reactive state for pending reads
	let pendingReads = $state(initialPendingReads);

	onMount(async () => {
		// Only confirm read for receiver role and if hash exists
		if (role === 'receiver' && hash) {
			try {
				const confirmResult = await api.confirmRead(hash);
				pendingReads = confirmResult.pending_reads;
				console.info(
					'[PendingReadsCounter] Confirmed read, new pending_reads:',
					confirmResult.pending_reads
				);
			} catch (err: unknown) {
				// Retry once on failure
				console.warn('[PendingReadsCounter] Failed to confirm read, retrying...', err);
				try {
					const retryResult = await api.confirmRead(hash);
					pendingReads = retryResult.pending_reads;
					console.info(
						'[PendingReadsCounter] Retry successful, new pending_reads:',
						retryResult.pending_reads
					);
				} catch (retryErr: unknown) {
					// Silent failure after retry: log for debugging but don't alert user
					console.error('[PendingReadsCounter] Retry failed (non-critical):', retryErr);
				}
			}
		}
	});
</script>

<div>
	<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
		{$_('sharedSecret.pendingReads')}
	</label>

	{#if pendingReads === -1}
		<!-- Sender: Unlimited reads -->
		<span class="text-lg font-semibold text-green-600 dark:text-green-400 flex items-center gap-2">
			<span class="text-2xl">♾️</span>
			{$_('sharedSecret.unlimited')}
		</span>
		<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
			{$_('sharedSecret.unlimitedHint')}
		</p>
	{:else if pendingReads === 0}
		<!-- Consumed / Deleted -->
		<span class="text-lg font-semibold text-red-600 dark:text-red-400 flex items-center gap-2">
			<span class="text-2xl">🔒</span>
			{$_('sharedSecret.consumed')}
		</span>
		<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
			{$_('sharedSecret.consumedHint')}
		</p>
	{:else if pendingReads === 1}
		<!-- Last read warning -->
		<span class="text-lg font-semibold text-amber-600 dark:text-amber-400 flex items-center gap-2">
			<span class="text-2xl">⚠️</span>
			{pendingReads} / {maxReads}
			{$_('sharedSecret.readsRemaining')}
		</span>
		<p class="text-xs text-amber-600 dark:text-amber-400 mt-1 font-medium">
			{$_('sharedSecret.lastReadHint')}
		</p>
	{:else}
		<!-- Normal state (2-10 reads) -->
		<span class="text-lg font-semibold text-blue-600 dark:text-blue-400 flex items-center gap-2">
			<span class="text-2xl">📖</span>
			{pendingReads} / {maxReads}
			{$_('sharedSecret.readsRemaining')}
		</span>
		<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
			{$_('sharedSecret.multipleReadsHint')}
		</p>
	{/if}
</div>
