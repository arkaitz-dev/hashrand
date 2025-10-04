<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { _ } from '$lib/stores/i18n';
	import { api } from '$lib/api';
	import { flashMessagesStore } from '$lib/stores/flashMessages';
	import { checkSessionAndHandle } from '$lib/session-expiry-manager';
	import type { ViewSharedSecretResponse } from '$lib/types';

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
			secret = response;
			otpRequired = false;
		} catch (error: unknown) {
			// Check if error is OTP required (400 with specific message)
			const err = error as { status?: number; message?: string };
			if (err.status === 400 && err.message?.includes('OTP')) {
				otpRequired = true;
			} else if (err.status === 404) {
				flashMessagesStore.addMessage($_('sharedSecret.secretNotFound'));
				setTimeout(() => goto('/'), 2000);
			} else if (err.status === 410) {
				flashMessagesStore.addMessage($_('sharedSecret.secretExpired'));
				setTimeout(() => goto('/'), 2000);
			} else {
				flashMessagesStore.addMessage($_('sharedSecret.retrievalError'));
				setTimeout(() => goto('/'), 2000);
			}
		} finally {
			isLoading = false;
		}
	}

	async function handleOtpSubmit(event: Event) {
		event.preventDefault();

		if (otpInput.length !== 9 || !/^\d{9}$/.test(otpInput)) {
			flashMessagesStore.addMessage($_('sharedSecret.invalidOtp'));
			return;
		}

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

	function formatDate(timestamp: number): string {
		return new Date(timestamp * 1000).toLocaleString();
	}

	onMount(async () => {
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
							<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								{$_('sharedSecret.from')}
							</label>
							<p class="text-gray-900 dark:text-white font-mono">{secret.sender_email}</p>
						</div>

						<!-- To -->
						<div>
							<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								{$_('sharedSecret.to')}
							</label>
							<p class="text-gray-900 dark:text-white font-mono">{secret.receiver_email}</p>
						</div>

						<!-- Role -->
						<div>
							<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								{$_('sharedSecret.role')}
							</label>
							<p class="text-gray-900 dark:text-white">
								{secret.role === 'sender'
									? $_('sharedSecret.roleSender')
									: $_('sharedSecret.roleReceiver')}
							</p>
						</div>

						<!-- Pending Reads -->
						<div>
							<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								{$_('sharedSecret.pendingReads')}
							</label>
							<p class="text-gray-900 dark:text-white">
								{secret.pending_reads === -1
									? $_('common.yes') + ' (' + $_('sharedSecret.roleSender') + ')'
									: secret.pending_reads}
							</p>
						</div>

						<!-- Expires At -->
						<div class="md:col-span-2">
							<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
								{$_('sharedSecret.expiresAt')}
							</label>
							<p class="text-gray-900 dark:text-white">{formatDate(secret.expires_at)}</p>
						</div>
					</div>

					<!-- Delete Button (only if pending_reads > 0) -->
					{#if secret.pending_reads > 0}
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
