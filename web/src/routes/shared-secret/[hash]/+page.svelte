<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import PendingReadsCounter from '$lib/components/PendingReadsCounter.svelte';
	import { _ } from '$lib/stores/i18n';
	import { api } from '$lib/api';
	import { flashMessagesStore } from '$lib/stores/flashMessages';
	import { checkSessionAndHandle } from '$lib/session-expiry-manager';
	import type { ViewSharedSecretResponse } from '$lib/types';
	import { logger } from '$lib/utils/logger';

	// Route parameter
	let hash = $derived($page.params.hash);

	// Secret data
	let secret: ViewSharedSecretResponse | null = $state(null);
	let otpRequired = $state(false);

	// UI state
	let isLoading = $state(true);
	let otpInput = $state('');
	let isSubmittingOtp = $state(false);
	let isDeleting = $state(false);

	// OTP validation
	let otpError = $derived(
		otpInput.length > 0 && (otpInput.length !== 9 || !/^\d{9}$/.test(otpInput))
			? $_('sharedSecret.invalidOtp')
			: ''
	);

	async function loadSecret(otp?: string) {
		// Validate hash parameter
		if (!hash) {
			flashMessagesStore.addMessage($_('sharedSecret.invalidHash'));
			setTimeout(() => goto('/'), 2000);
			return;
		}

		isLoading = true;

		try {
			const otpRequest = otp ? { otp } : undefined;
			const response = await api.viewSharedSecret(hash, otpRequest);

			// CRITICAL: Backend returns OTP errors as HTTP 200 with 'error' field in SignedResponse
			// Check for error field AFTER SignedResponse validation passes
			if ('error' in response) {
				if (response.error === 'OTP_REQUIRED') {
					// OTP required but not provided
					otpRequired = true;
					isLoading = false;
					return;
				} else if (response.error === 'INVALID_OTP') {
					// Invalid OTP provided
					flashMessagesStore.addMessage($_('sharedSecret.invalidOtp'));
					isLoading = false;
					return;
				}
			}

			// If we reach here, response is valid ViewSharedSecretResponse
			secret = response;
			otpRequired = false;

			// NOTE: Confirm read logic moved to PendingReadsCounter component
			// if (response.role === 'receiver') {
			// 	// Await confirmation to get updated pending_reads
			// 	try {
			// 		const confirmResult = await api.confirmRead(hash);
			// 		// Update pending_reads with new value from backend
			// 		secret.pending_reads = confirmResult.pending_reads;
			// 		console.info(
			// 			'[SharedSecret] Confirmed read, new pending_reads:',
			// 			confirmResult.pending_reads
			// 		);
			// 	} catch (err: unknown) {
			// 		// Retry once on failure
			// 		console.warn('[SharedSecret] Failed to confirm read, retrying...', err);
			// 		try {
			// 			const retryResult = await api.confirmRead(hash);
			// 			secret.pending_reads = retryResult.pending_reads;
			// 			console.info(
			// 				'[SharedSecret] Retry successful, new pending_reads:',
			// 				retryResult.pending_reads
			// 			);
			// 		} catch (retryErr: unknown) {
			// 			// Silent failure after retry: log for debugging but don't alert user
			// 			console.error('[SharedSecret] Retry failed (non-critical):', retryErr);
			// 		}
			// 	}
			// }
		} catch (error: unknown) {
			// Handle HTTP errors (404, 410, 403, network errors, etc.)
			const err = error as { status?: number; message?: string };
			if (err.status === 404) {
				logger.warn('[SharedSecret] Secret not found (404):', hash);
				flashMessagesStore.addMessage($_('sharedSecret.secretNotFound'));
				setTimeout(() => goto('/'), 2000);
			} else if (err.status === 410) {
				logger.warn('[SharedSecret] Secret expired (410):', hash);
				flashMessagesStore.addMessage($_('sharedSecret.secretExpired'));
				setTimeout(() => goto('/'), 2000);
			} else if (err.status === 403) {
				logger.warn('[SharedSecret] Access denied (403) - ownership validation failed:', hash);
				flashMessagesStore.addMessage($_('sharedSecret.accessDenied'));
				setTimeout(() => goto('/'), 2000);
			} else {
				logger.error('[SharedSecret] Retrieval error:', { status: err.status, hash });
				flashMessagesStore.addMessage($_('sharedSecret.retrievalError'));
				setTimeout(() => goto('/'), 2000);
			}
		} finally {
			isLoading = false;
		}
	}

	async function handleOtpSubmit(event: Event) {
		event.preventDefault();
		logger.info('[Form] Submitting OTP for shared secret access');

		// Validate hash parameter
		if (!hash) {
			flashMessagesStore.addMessage($_('sharedSecret.invalidHash'));
			return;
		}

		// Validate OTP format
		if (otpInput.length !== 9 || !/^\d{9}$/.test(otpInput)) {
			flashMessagesStore.addMessage($_('sharedSecret.invalidOtp'));
			return;
		}

		// Get current pending_reads via preview (GET without OTP)
		let currentReads = 0;
		try {
			const preview = await api.viewSharedSecret(hash); // GET request, no OTP

			// Check if preview contains error field (OTP_REQUIRED expected here)
			if ('error' in preview) {
				// OTP is required (expected), skip preview and proceed directly
				isSubmittingOtp = true;
				await loadSecret(otpInput);
				isSubmittingOtp = false;
				return;
			}

			// Preview succeeded (no OTP required - unexpected but handle it)
			currentReads = preview.pending_reads;
		} catch (error: unknown) {
			// If preview fails, continue anyway (don't block submission)
			logger.warn('[SharedSecret] Preview failed, skipping confirmation:', error);
			isSubmittingOtp = true;
			await loadSecret(otpInput);
			isSubmittingOtp = false;
			return;
		}

		// Show confirmation dialog
		const remainingAfter = currentReads - 1;
		const confirmMessage =
			currentReads === 1
				? $_('sharedSecret.confirmLastRead')
				: `${$_('sharedSecret.confirmReadConsumption')}\n\n${$_('sharedSecret.readsRemainingAfter')}: ${remainingAfter} ${$_('common.of')} ${currentReads}`;

		const confirmed = globalThis.confirm(confirmMessage);

		if (!confirmed) {
			// User cancelled
			return;
		}

		// Proceed with submission
		isSubmittingOtp = true;
		await loadSecret(otpInput);
		isSubmittingOtp = false;
	}

	async function handleDelete() {
		if (!hash) {
			flashMessagesStore.addMessage($_('sharedSecret.invalidHash'));
			setTimeout(() => goto('/'), 2000);
			return;
		}

		if (!globalThis.confirm($_('sharedSecret.confirmDelete'))) {
			return;
		}

		isDeleting = true;

		try {
			await api.deleteSharedSecret(hash);
			flashMessagesStore.addMessage($_('sharedSecret.secretDeleted'));
			setTimeout(() => goto('/'), 1500);
		} catch {
			flashMessagesStore.addMessage($_('sharedSecret.deletionError'));
		} finally {
			isDeleting = false;
		}
	}

	async function copyToClipboard(text: string) {
		try {
			await navigator.clipboard.writeText(text);
			flashMessagesStore.addMessage($_('common.copied'));
		} catch {
			flashMessagesStore.addMessage($_('common.failedToCopy'));
		}
	}

	function formatDate(timestampHours: number): string {
		// Backend stores expires_at in HOURS (timestamp / 3600)
		// Convert hours to milliseconds: hours * 3600 * 1000
		return new Date(timestampHours * 3600 * 1000).toLocaleString();
	}

	function formatTimeRemaining(expiresAtHours: number): string {
		const expiresAtMs = expiresAtHours * 3600 * 1000;
		const nowMs = Date.now();
		const diffMs = expiresAtMs - nowMs;

		if (diffMs <= 0) {
			return $_('sharedSecret.expired'); // "Expired"
		}

		const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
		const diffMinutes = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));

		if (diffHours >= 24) {
			const days = Math.floor(diffHours / 24);
			return `${days} ${days === 1 ? $_('common.day') : $_('common.days')}`;
		} else if (diffHours > 0) {
			return `${diffHours}h ${diffMinutes}min`;
		} else {
			return `${diffMinutes} ${$_('common.minutes')}`;
		}
	}

	onMount(async () => {
		logger.info('[Route] Shared Secret view page loaded');
		// Check session expiration before loading
		const sessionValid = await checkSessionAndHandle({
			onExpired: 'launch-auth',
			next: `/shared-secret/${hash}`
		});

		if (!sessionValid) {
			return;
		}

		// Load secret without OTP first
		await loadSecret();
	});
</script>

<svelte:head>
	<title>{$_('sharedSecret.viewSecret')} - {$_('menu.brandName')}</title>
</svelte:head>

<div
	class="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800"
>
	<div class="container mx-auto px-4 py-8">
		<div class="max-w-3xl mx-auto">
			<!-- Header -->
			<div class="text-center mb-8">
				<div
					class="inline-flex items-center justify-center w-16 h-16 bg-indigo-600 rounded-full mb-4"
				>
					<span class="text-2xl">📬</span>
				</div>
				<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
					{$_('sharedSecret.viewSecret')}
				</h1>
			</div>

			<!-- Flash Messages -->
			<FlashMessages />

			{#if isLoading}
				<!-- Loading State -->
				<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-12 mb-6 text-center">
					<div
						class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600 mx-auto mb-4"
					></div>
					<p class="text-gray-600 dark:text-gray-300">{$_('common.loading')}</p>
				</div>
			{:else if otpRequired}
				<!-- OTP Input Form -->
				<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-6">
					<h2 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
						{$_('sharedSecret.otpRequired')}
					</h2>
					<form onsubmit={handleOtpSubmit}>
						<div class="mb-4">
							<label
								for="otp"
								class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
							>
								{$_('sharedSecret.enterOtp')}
							</label>
							<input
								type="text"
								id="otp"
								bind:value={otpInput}
								placeholder={$_('sharedSecret.otpPlaceholder')}
								maxlength="9"
								class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent dark:bg-gray-700 dark:text-white font-mono text-2xl text-center tracking-widest"
								required
							/>
							{#if otpError}
								<p class="mt-1 text-sm text-red-600 dark:text-red-400">{otpError}</p>
							{/if}
						</div>
						<button
							type="submit"
							disabled={otpInput.length !== 9 || isSubmittingOtp}
							class="w-full bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
						>
							{isSubmittingOtp ? $_('sharedSecret.submitting') : $_('sharedSecret.submit')}
						</button>
					</form>
				</div>
			{:else if secret}
				<!-- Secret Content -->
				<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-6">
					<h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">
						{$_('sharedSecret.secretContent')}
					</h2>

					<!-- ⚠️ Last Read Warning Banner (shown when consumed) -->
					{#if secret.pending_reads === 0 && secret.role === 'receiver'}
						<div
							class="mb-6 bg-amber-50 dark:bg-amber-900/20 border-l-4 border-amber-500 dark:border-amber-600 rounded-r-lg shadow-sm"
						>
							<div class="flex p-4">
								<div class="flex-shrink-0">
									<span class="text-3xl" aria-hidden="true">⚠️</span>
								</div>
								<div class="ml-4 flex-1">
									<h3 class="text-lg font-semibold text-amber-800 dark:text-amber-200 mb-2">
										{$_('sharedSecret.lastReadWarningTitle')}
									</h3>
									<p class="text-sm text-amber-700 dark:text-amber-300">
										{$_('sharedSecret.lastReadWarningMessage')}
									</p>
									<p class="text-xs text-amber-600 dark:text-amber-400 mt-2 font-medium">
										💡 {$_('sharedSecret.lastReadWarningTip')}
									</p>
								</div>
							</div>
						</div>
					{/if}

					<!-- Secret Text -->
					<div class="mb-6">
						<div class="relative">
							<textarea
								readonly
								value={secret.secret_text}
								onclick={(e) => {
									e.currentTarget.select();
									copyToClipboard(secret!.secret_text);
								}}
								class="w-full px-4 py-4 border-2 border-indigo-300 dark:border-indigo-600 rounded-lg bg-indigo-50 dark:bg-indigo-900/20 dark:text-white resize-none cursor-pointer focus:ring-2 focus:ring-indigo-500 focus:border-transparent font-mono text-lg"
								rows="6"
							></textarea>
							<p class="mt-2 text-sm text-gray-500 dark:text-gray-400 text-center">
								{$_('common.clickToSelect')}
							</p>
						</div>
					</div>

					<!-- Metadata -->
					<div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
						<!-- From -->
						<div>
							<div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								{$_('sharedSecret.from')}
							</div>
							<p class="text-gray-900 dark:text-white font-mono">{secret.sender_email}</p>
						</div>

						<!-- To -->
						<div>
							<div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								{$_('sharedSecret.to')}
							</div>
							<p class="text-gray-900 dark:text-white font-mono">{secret.receiver_email}</p>
						</div>

						<!-- Role -->
						<div>
							<div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								{$_('sharedSecret.role')}
							</div>
							<p class="text-gray-900 dark:text-white">
								{secret.role === 'sender'
									? $_('sharedSecret.roleSender')
									: $_('sharedSecret.roleReceiver')}
							</p>
						</div>

						<!-- Pending Reads - Reactive Component -->
						<PendingReadsCounter
							initialPendingReads={secret.pending_reads}
							maxReads={secret.max_reads}
							{hash}
							role={secret.role}
						/>

						<!-- Expires At -->
						<div class="md:col-span-2">
							<div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								{$_('sharedSecret.expiresAt')}
							</div>
							<div class="flex flex-col gap-1">
								<p class="text-gray-900 dark:text-white font-medium">
									{formatDate(secret.expires_at)}
								</p>
								<p class="text-sm text-gray-600 dark:text-gray-400">
									⏱️ {$_('sharedSecret.timeRemaining')}:
									<span class="font-semibold">{formatTimeRemaining(secret.expires_at)}</span>
								</p>
							</div>
						</div>
					</div>

					<!-- Delete Button (always shown for sender, hidden for consumed receiver) -->
					{#if secret.role === 'sender' || secret.pending_reads > 0}
						<button
							onclick={handleDelete}
							disabled={isDeleting}
							class="w-full bg-red-600 hover:bg-red-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
						>
							{isDeleting ? $_('sharedSecret.deleting') : $_('sharedSecret.deleteSecret')}
						</button>
					{/if}
				</div>
			{/if}

			<!-- Back Button -->
			<BackToMenuButton />
		</div>
	</div>
</div>
