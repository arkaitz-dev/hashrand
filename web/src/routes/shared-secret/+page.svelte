<script lang="ts">
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import LanguageSelect from '$lib/components/LanguageSelect.svelte';
	import { _, currentLanguage } from '$lib/stores/i18n';
	import { api } from '$lib/api';
	import { flashMessagesStore } from '$lib/stores/flashMessages';
	import { checkSessionOrAutoLogout } from '$lib/session-expiry-manager';
	import { getUserEmail } from '$lib/session';
	import { onMount } from 'svelte';
	import type { CreateSharedSecretResponse } from '$lib/types';
	import { logger } from '$lib/utils/logger';

	// Form state
	let senderEmail = $state('');
	let receiverEmail = $state('');
	let receiverLanguage = $state($currentLanguage); // Default to current UI language
	let secretText = $state('');
	let expiresHours = $state(24);
	let maxReads = $state(3);
	let requireOtp = $state(false);
	let sendCopyToSender = $state(false);

	// UI state
	let isCreating = $state(false);
	let isLoadingEmail = $state(true);
	let createdSecret: CreateSharedSecretResponse | null = $state(null);

	// Load user email from IndexedDB on mount
	onMount(async () => {
		logger.info('[Route] Shared Secret creation page loaded');
		const email = await getUserEmail();
		if (email) {
			senderEmail = email;
		}
		isLoadingEmail = false;
	});

	// Validation
	let receiverEmailError = $derived(
		receiverEmail && !isValidEmail(receiverEmail) ? $_('sharedSecret.emailInvalid') : ''
	);
	let secretTextError = $derived(
		secretText.length > 512
			? $_('sharedSecret.secretTooLong')
			: secretText.length === 0 && secretText !== ''
				? $_('sharedSecret.secretEmpty')
				: ''
	);
	let expiresError = $derived(
		expiresHours < 1 || expiresHours > 72 ? $_('sharedSecret.expiresInvalid') : ''
	);
	let readsError = $derived(maxReads < 1 || maxReads > 10 ? $_('sharedSecret.readsInvalid') : '');

	let formValid = $derived(
		senderEmail.length > 0 &&
			isValidEmail(senderEmail) &&
			receiverEmail.length > 0 &&
			isValidEmail(receiverEmail) &&
			secretText.length > 0 &&
			secretText.length <= 512 &&
			expiresHours >= 1 &&
			expiresHours <= 72 &&
			maxReads >= 1 &&
			maxReads <= 10
	);

	function isValidEmail(email: string): boolean {
		return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
	}

	async function handleCreate(event: Event) {
		event.preventDefault();
		logger.info('[Form] Submitting shared secret creation form');

		if (!formValid) {
			flashMessagesStore.addMessage($_('common.formInvalid'));
			return;
		}

		// Check session expiration before creation
		// If expired, performs automatic logout (redirect + cleanup + flash)
		const sessionValid = await checkSessionOrAutoLogout();

		if (!sessionValid) {
			// Session expired, auto-logout already performed
			return;
		}

		// Extract ui_host (same logic as magic link)
		const { extractDomain } = await import('$lib/utils/domain-extractor');
		const ui_host = extractDomain();

		if (!ui_host) {
			flashMessagesStore.addMessage('UI host is required for URL generation');
			return;
		}

		isCreating = true;

		try {
			const response = await api.createSharedSecret({
				sender_email: senderEmail,
				receiver_email: receiverEmail,
				secret_text: secretText,
				expires_hours: expiresHours,
				max_reads: maxReads,
				require_otp: requireOtp,
				send_copy_to_sender: sendCopyToSender,
				receiver_language: receiverLanguage,
				sender_language: $currentLanguage,
				ui_host
			});

			createdSecret = response;
			flashMessagesStore.addMessage($_('sharedSecret.secretCreated'));
		} catch {
			flashMessagesStore.addMessage($_('sharedSecret.creationError'));
		} finally {
			isCreating = false;
		}
	}

	async function copyToClipboard(text: string, type: 'url' | 'reference' | 'otp') {
		try {
			await navigator.clipboard.writeText(text);
			const message =
				type === 'url'
					? $_('sharedSecret.copyUrl')
					: type === 'reference'
						? $_('sharedSecret.copyReference')
						: $_('sharedSecret.copyOtp');
			flashMessagesStore.addMessage(`${message}: ${$_('common.copied')}`);
		} catch {
			flashMessagesStore.addMessage($_('common.failedToCopy'));
		}
	}

	function resetForm() {
		receiverEmail = '';
		secretText = '';
		expiresHours = 24;
		maxReads = 3;
		requireOtp = false;
		sendCopyToSender = false;
		createdSecret = null;
	}

	onMount(() => {
		// Clear any previous state
		createdSecret = null;
	});
</script>

<svelte:head>
	<title>{$_('sharedSecret.title')} - {$_('menu.brandName')}</title>
	<meta name="description" content={$_('sharedSecret.description')} />
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
					{$_('sharedSecret.title')}
				</h1>
				<p class="text-gray-600 dark:text-gray-300">
					{$_('sharedSecret.description')}
				</p>
			</div>

			<!-- Flash Messages -->
			<FlashMessages />

			{#if !createdSecret}
				<!-- Creation Form -->
				<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-6">
					<form onsubmit={handleCreate}>
						<!-- Sender Email (Display Only) -->
						<div class="mb-4">
							<div class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
								{$_('sharedSecret.senderEmail')}
							</div>
							{#if isLoadingEmail}
								<p class="text-gray-500 dark:text-gray-400 italic">
									{$_('common.loading')}...
								</p>
							{:else if senderEmail}
								<p
									class="px-4 py-2 bg-gray-100 dark:bg-gray-700 rounded-lg text-gray-900 dark:text-white font-medium"
								>
									{senderEmail}
								</p>
							{:else}
								<p class="text-red-600 dark:text-red-400">
									{$_('sharedSecret.emailNotAvailable')}
								</p>
							{/if}
						</div>

						<!-- Receiver Email -->
						<div class="mb-4">
							<label
								for="receiver-email"
								class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
							>
								{$_('sharedSecret.receiverEmail')}
							</label>
							<input
								type="email"
								id="receiver-email"
								bind:value={receiverEmail}
								placeholder={$_('sharedSecret.receiverEmailPlaceholder')}
								class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
								required
							/>
							{#if receiverEmailError}
								<p class="mt-1 text-sm text-red-600 dark:text-red-400">{receiverEmailError}</p>
							{/if}
						</div>

						<!-- Receiver Language -->
						<div class="mb-4">
							<label
								for="receiver-language"
								class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
							>
								{$_('common.selectLanguage')} ({$_('sharedSecret.receiverEmail')})
							</label>
							<LanguageSelect id="receiver-language" bind:value={receiverLanguage} />
						</div>

						<!-- Secret Text -->
						<div class="mb-4">
							<label
								for="secret-text"
								class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
							>
								{$_('sharedSecret.secretText')}
							</label>
							<textarea
								id="secret-text"
								bind:value={secretText}
								placeholder={$_('sharedSecret.secretTextPlaceholder')}
								rows="4"
								class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent dark:bg-gray-700 dark:text-white resize-none"
								required
							></textarea>
							<div class="mt-1 text-sm text-gray-500 dark:text-gray-400 text-right">
								{secretText.length}/512
							</div>
							{#if secretTextError}
								<p class="mt-1 text-sm text-red-600 dark:text-red-400">{secretTextError}</p>
							{/if}
						</div>

						<!-- Expires Hours -->
						<div class="mb-4">
							<label
								for="expires-hours"
								class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
							>
								{$_('sharedSecret.expiresHours')}
							</label>
							<input
								type="number"
								id="expires-hours"
								bind:value={expiresHours}
								min="1"
								max="72"
								class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
								required
							/>
							{#if expiresError}
								<p class="mt-1 text-sm text-red-600 dark:text-red-400">{expiresError}</p>
							{/if}
						</div>

						<!-- Max Reads -->
						<div class="mb-4">
							<label
								for="max-reads"
								class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
							>
								{$_('sharedSecret.maxReads')}
							</label>
							<input
								type="number"
								id="max-reads"
								bind:value={maxReads}
								min="1"
								max="10"
								class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
								required
							/>
							{#if readsError}
								<p class="mt-1 text-sm text-red-600 dark:text-red-400">{readsError}</p>
							{/if}
						</div>

						<!-- Require OTP -->
						<div class="mb-4">
							<label class="flex items-center">
								<input
									type="checkbox"
									bind:checked={requireOtp}
									class="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500 dark:border-gray-600 dark:bg-gray-700"
								/>
								<span class="ml-2 text-sm text-gray-700 dark:text-gray-300">
									{$_('sharedSecret.requireOtp')}
								</span>
							</label>
						</div>

						<!-- Send Copy to Sender -->
						<div class="mb-6">
							<label class="flex items-center">
								<input
									type="checkbox"
									bind:checked={sendCopyToSender}
									class="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500 dark:border-gray-600 dark:bg-gray-700"
								/>
								<span class="ml-2 text-sm text-gray-700 dark:text-gray-300">
									{$_('sharedSecret.sendCopyToSender')}
								</span>
							</label>
						</div>

						<!-- Submit Button -->
						<button
							type="submit"
							disabled={!formValid || isCreating}
							class="w-full bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
						>
							{isCreating ? $_('sharedSecret.creating') : $_('sharedSecret.createSecret')}
						</button>
					</form>
				</div>
			{:else}
				<!-- Success Result -->
				<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-6">
					<h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">
						{$_('sharedSecret.secretCreated')}
					</h2>

					<!-- Sender URL -->
					<div class="mb-4">
						<label
							for="sender-url"
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
						>
							{$_('sharedSecret.yourUrl')}
						</label>
						<div class="flex gap-2">
							<input
								id="sender-url"
								type="text"
								readonly
								value={createdSecret.url_sender}
								class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-white"
							/>
							<button
								onclick={() => copyToClipboard(createdSecret!.url_sender, 'url')}
								class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors duration-200"
							>
								{$_('sharedSecret.copyUrl')}
							</button>
						</div>
					</div>

					<!-- Receiver URL -->
					<div class="mb-4">
						<label
							for="receiver-url"
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
						>
							{$_('sharedSecret.receiverUrl')}
						</label>
						<div class="flex gap-2">
							<input
								id="receiver-url"
								type="text"
								readonly
								value={createdSecret.url_receiver}
								class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-white"
							/>
							<button
								onclick={() => copyToClipboard(createdSecret!.url_receiver, 'url')}
								class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors duration-200"
							>
								{$_('sharedSecret.copyUrl')}
							</button>
						</div>
					</div>

					<!-- Reference Hash -->
					<div class="mb-4">
						<label
							for="reference-hash"
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
						>
							{$_('sharedSecret.reference')}
						</label>
						<div class="flex gap-2">
							<input
								id="reference-hash"
								type="text"
								readonly
								value={createdSecret.reference}
								class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-white font-mono"
							/>
							<button
								onclick={() => copyToClipboard(createdSecret!.reference, 'reference')}
								class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors duration-200"
							>
								{$_('sharedSecret.copyReference')}
							</button>
						</div>
					</div>

					<!-- OTP (if required) -->
					{#if createdSecret.otp}
						<div class="mb-6">
							<label
								for="otp-code"
								class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
							>
								{$_('sharedSecret.otpCode')}
							</label>
							<div class="flex gap-2">
								<input
									id="otp-code"
									type="text"
									readonly
									value={createdSecret.otp}
									class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-white font-mono text-2xl text-center tracking-widest"
								/>
								<button
									onclick={() => copyToClipboard(createdSecret!.otp!, 'otp')}
									class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors duration-200"
								>
									{$_('sharedSecret.copyOtp')}
								</button>
							</div>
						</div>
					{/if}

					<!-- New Secret Button -->
					<button
						onclick={resetForm}
						class="w-full bg-green-600 hover:bg-green-700 text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
					>
						{$_('sharedSecret.createSecret')}
					</button>
				</div>
			{/if}

			<!-- Back Button -->
			<BackToMenuButton />
		</div>
	</div>
</div>
