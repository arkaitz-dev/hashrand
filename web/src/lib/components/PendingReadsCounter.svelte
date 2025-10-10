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
		logger.info('[PendingReadsCounter] 🚀 onMount() STARTED', {
			role,
			hash,
			initialPendingReads,
			maxReads,
			timestamp: Date.now()
		});

		// Only confirm read for receiver role and if hash exists
		if (role !== 'receiver' || !hash) {
			logger.info('[PendingReadsCounter] ⏭️ SKIPPING - Not receiver or no hash', {
				role,
				hasHash: !!hash
			});
			return;
		}

		// CRITICAL: Wait for access token to be available before confirming read
		// This prevents 401 errors when navigating immediately after magic link validation
		const { authStore } = await import('$lib/stores/auth');
		let tokenReady = false;
		let attempts = 0;
		const maxAttempts = 50; // 5 seconds max wait

		logger.debug('[PendingReadsCounter] ⏳ Waiting for access token to be available', {
			maxAttempts,
			waitTimeSeconds: maxAttempts * 0.1
		});

		while (!tokenReady && attempts < maxAttempts) {
			const accessToken = authStore.getAccessToken();

			if (attempts === 0 || attempts % 10 === 0) {
				// Log every 10th attempt to reduce noise
				logger.debug('[PendingReadsCounter] Token check attempt', {
					attempt: attempts + 1,
					maxAttempts,
					hasAccessToken: !!accessToken,
					accessTokenLength: accessToken?.length || 0
				});
			}

			if (accessToken) {
				tokenReady = true;
				logger.info('[PendingReadsCounter] ✅ Access token ready', {
					attempts: attempts + 1,
					waitTimeMs: attempts * 100
				});
			} else {
				await new Promise((resolve) => setTimeout(resolve, 100));
				attempts++;
			}
		}

		if (!tokenReady) {
			logger.error('[PendingReadsCounter] ❌ Access token NOT ready - ABORTING', {
				attempts,
				waitTimeMs: attempts * 100
			});
			return;
		}

		// NEW LOGIC: Check cache before confirming read (with graceful fallback if IndexedDB unavailable)
		let useCaching = true;
		let cachedTimestamp: number | null = null;

		logger.info('[PendingReadsCounter] 🔍 CACHE CHECK - Starting', {
			hash,
			timeout_ms: CONFIRM_READ_CACHE_TIMEOUT,
			timeout_seconds: Math.round(CONFIRM_READ_CACHE_TIMEOUT / 1000),
			current_timestamp: Date.now()
		});

		// Try to check cache, but don't fail if IndexedDB is unavailable
		try {
			logger.debug('[PendingReadsCounter] Attempting to access IndexedDB cache');
			cachedTimestamp = await getCachedConfirmation(hash);

			if (cachedTimestamp) {
				const age = Date.now() - cachedTimestamp;
				const ageSeconds = Math.round(age / 1000);
				const timeoutSeconds = Math.round(CONFIRM_READ_CACHE_TIMEOUT / 1000);

				logger.info('[PendingReadsCounter] 📦 CACHE HIT - Timestamp found', {
					hash,
					cachedTimestamp,
					cached_at: new Date(cachedTimestamp).toISOString(),
					age_ms: age,
					age_seconds: ageSeconds,
					timeout_ms: CONFIRM_READ_CACHE_TIMEOUT,
					timeout_seconds: timeoutSeconds,
					is_valid: age < CONFIRM_READ_CACHE_TIMEOUT
				});

				if (age < CONFIRM_READ_CACHE_TIMEOUT) {
					// Cache hit: SKIP API call
					logger.info('[PendingReadsCounter] ✅ CACHE VALID - SKIPPING API CALL', {
						hash,
						age_seconds: ageSeconds,
						timeout_seconds: timeoutSeconds,
						remaining_seconds: timeoutSeconds - ageSeconds,
						decision: 'SKIP_API_CALL'
					});
					return; // 🚨 EARLY EXIT - This should prevent API call
				}

				// Cache expired: revalidate
				logger.info('[PendingReadsCounter] ⏰ CACHE EXPIRED - Will revalidate', {
					hash,
					age_seconds: ageSeconds,
					timeout_seconds: timeoutSeconds,
					exceeded_by_seconds: ageSeconds - timeoutSeconds,
					decision: 'CALL_API'
				});
			} else {
				logger.info('[PendingReadsCounter] 📭 CACHE MISS - No timestamp found', {
					hash,
					decision: 'CALL_API'
				});
			}
		} catch (cacheError: unknown) {
			// IndexedDB not available (private mode, restricted browser, etc.)
			const error = cacheError as Error;
			logger.warn('[PendingReadsCounter] ⚠️ Cache unavailable, proceeding without caching', {
				hash,
				errorType: error?.constructor?.name,
				errorMessage: error?.message,
				errorString: String(cacheError)
			});
			useCaching = false;
		}

		// Call confirmRead (first time, revalidation, or no cache available)
		logger.info('[PendingReadsCounter] 📞 CALLING API - confirmRead endpoint', {
			hash,
			useCaching,
			reason: cachedTimestamp
				? 'cache_expired'
				: cachedTimestamp === null
					? 'cache_miss'
					: 'no_cache_available'
		});

		try {
			const confirmResult = await api.confirmRead(hash);

			logger.info('[PendingReadsCounter] ✅ API CALL SUCCESS - Read confirmed', {
				hash,
				previous_pending_reads: pendingReads,
				new_pending_reads: confirmResult.pending_reads,
				read_confirmed: confirmResult.read_confirmed,
				role: confirmResult.role,
				message: confirmResult.message
			});

			pendingReads = confirmResult.pending_reads;

			// Update cache only if caching is available
			if (useCaching) {
				try {
					await setCachedConfirmation(hash);
					logger.info('[PendingReadsCounter] 💾 Cache updated after API call', {
						hash,
						pending_reads: confirmResult.pending_reads,
						cached: true
					});
				} catch (cacheError) {
					logger.warn('[PendingReadsCounter] ⚠️ Failed to cache confirmation', {
						hash,
						error: cacheError,
						pending_reads: confirmResult.pending_reads,
						cached: false
					});
				}
			} else {
				logger.info('[PendingReadsCounter] Read confirmed (no cache)', {
					hash,
					pending_reads: confirmResult.pending_reads,
					cached: false
				});
			}
		} catch (err: unknown) {
			// Error calling confirmRead API: clear cache + redirect + flash message
			logger.error('[PendingReadsCounter] ❌ API CALL FAILED - Error confirming read', {
				hash,
				error: err,
				useCaching
			});

			// Try to clear cache, but don't fail if IndexedDB is unavailable
			if (useCaching) {
				try {
					await clearCachedConfirmation(hash);
					logger.info('[PendingReadsCounter] 🗑️ Cache cleared after API error', { hash });
				} catch (clearError) {
					logger.warn('[PendingReadsCounter] Failed to clear cache (non-critical)', {
						hash,
						error: clearError
					});
				}
			}

			flashMessagesStore.addMessage($_('sharedSecret.accessError'));
			await goto('/');
		}

		logger.info('[PendingReadsCounter] 🏁 onMount() COMPLETED', {
			hash,
			final_pending_reads: pendingReads
		});
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
