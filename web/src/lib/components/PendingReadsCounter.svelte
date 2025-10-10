<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { api } from '$lib/api';
	import { _ } from '$lib/stores/i18n';
	import { logger } from '$lib/utils/logger';
	import { flashMessagesStore } from '$lib/stores/flashMessages';
	import {
		getCachedConfirmation,
		setCachedConfirmation,
		clearCachedConfirmation,
		CONFIRM_READ_CACHE_TIMEOUT
	} from '$lib/utils/confirm-read-cache';

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
		if (role !== 'receiver' || !hash) return;

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
			logger.warn(
				'[PendingReadsCounter] Access token not ready after waiting, skipping confirmation'
			);
			return;
		}

		// NEW LOGIC: Check cache before confirming read (with graceful fallback if IndexedDB unavailable)
		let useCaching = true;
		let cachedTimestamp: number | null = null;

		// Try to check cache, but don't fail if IndexedDB is unavailable
		try {
			logger.debug('[PendingReadsCounter] Attempting to access IndexedDB cache');
			cachedTimestamp = await getCachedConfirmation(hash);
			logger.debug('[PendingReadsCounter] Cache check successful', { hasCache: !!cachedTimestamp });

			if (cachedTimestamp) {
				const age = Date.now() - cachedTimestamp;

				if (age < CONFIRM_READ_CACHE_TIMEOUT) {
					// Cache hit: SKIP API call
					logger.debug('[PendingReadsCounter] Confirmation already cached, skipping API call', {
						age: Math.round(age / 1000) + 's',
						timeout: Math.round(CONFIRM_READ_CACHE_TIMEOUT / 1000) + 's'
					});
					return;
				}

				// Cache expired: revalidate
				logger.debug('[PendingReadsCounter] Cache expired, revalidating...', {
					age: Math.round(age / 1000) + 's'
				});
			}
		} catch (cacheError) {
			// IndexedDB not available (private mode, restricted browser, etc.)
			logger.warn(
				'[PendingReadsCounter] Cache unavailable, proceeding without caching:',
				cacheError
			);
			useCaching = false;
		}

		// Call confirmRead (first time, revalidation, or no cache available)
		try {
			const confirmResult = await api.confirmRead(hash);
			pendingReads = confirmResult.pending_reads;

			// Update cache only if caching is available
			if (useCaching) {
				try {
					await setCachedConfirmation(hash);
					logger.debug('[PendingReadsCounter] Read confirmed, new pending_reads:', {
						pending_reads: confirmResult.pending_reads,
						cached: true
					});
				} catch (cacheError) {
					logger.warn('[PendingReadsCounter] Failed to cache confirmation:', cacheError);
					logger.debug('[PendingReadsCounter] Read confirmed, new pending_reads:', {
						pending_reads: confirmResult.pending_reads,
						cached: false
					});
				}
			} else {
				logger.debug('[PendingReadsCounter] Read confirmed, new pending_reads:', {
					pending_reads: confirmResult.pending_reads,
					cached: false
				});
			}
		} catch (err: unknown) {
			// Error calling confirmRead API: clear cache + redirect + flash message
			logger.error('[PendingReadsCounter] Error confirming read:', err);

			// Try to clear cache, but don't fail if IndexedDB is unavailable
			if (useCaching) {
				try {
					await clearCachedConfirmation(hash);
				} catch (clearError) {
					logger.warn('[PendingReadsCounter] Failed to clear cache (non-critical):', clearError);
				}
			}

			flashMessagesStore.addMessage($_('sharedSecret.accessError'));
			await goto('/');
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
