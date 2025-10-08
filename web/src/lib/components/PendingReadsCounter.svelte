<script lang="ts">
	import { onMount } from 'svelte';
	import { api } from '$lib/api';
	import { _ } from '$lib/stores/i18n';
	import { logger } from '$lib/utils/logger';

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
			// CRITICAL: Wait for access token to be available before confirming read
			// This prevents 401 errors when navigating immediately after magic link validation
			const { authStore } = await import('$lib/stores/auth');
			let tokenReady = false;
			let attempts = 0;
			const maxAttempts = 50; // 5 seconds max wait

			logger.debug('[PendingReadsCounter] Waiting for access token to be available');

			while (!tokenReady && attempts < maxAttempts) {
				const accessToken = authStore.getAccessToken();

				logger.debug('[PendingReadsCounter] Token check attempt', {
					attempt: attempts + 1,
					hasAccessToken: !!accessToken,
					accessTokenLength: accessToken?.length || 0
				});

				if (accessToken) {
					tokenReady = true;
					logger.debug('[PendingReadsCounter] Access token ready, proceeding with confirmation');
				} else {
					await new Promise((resolve) => setTimeout(resolve, 100));
					attempts++;
				}
			}

			if (!tokenReady) {
				logger.warn('[PendingReadsCounter] Access token not ready after waiting, skipping confirmation');
				return;
			}

			try {
				const confirmResult = await api.confirmRead(hash);
				pendingReads = confirmResult.pending_reads;
				logger.info(
					'[PendingReadsCounter] Confirmed read, new pending_reads:',
					confirmResult.pending_reads
				);
			} catch (err: unknown) {
				// Retry once on failure
				logger.warn('[PendingReadsCounter] Failed to confirm read, retrying...', err);
				try {
					const retryResult = await api.confirmRead(hash);
					pendingReads = retryResult.pending_reads;
					logger.info(
						'[PendingReadsCounter] Retry successful, new pending_reads:',
						retryResult.pending_reads
					);
				} catch (retryErr: unknown) {
					// Silent failure after retry: log for debugging but don't alert user
					logger.error('[PendingReadsCounter] Retry failed (non-critical):', retryErr);
				}
			}
		}
	});
</script>

<div>
	<div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
		{$_('sharedSecret.pendingReads')}
	</div>

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
