<script lang="ts">
	import BackToMenuButton from '$lib/components/BackToMenuButton.svelte';
	import FlashMessages from '$lib/components/FlashMessages.svelte';
	import { _ } from '$lib/stores/i18n';
	import { api } from '$lib/api';
	import { flashMessagesStore } from '$lib/stores/flashMessages';
	import { checkSessionAndHandle } from '$lib/session-expiry-manager';
	import { onMount } from 'svelte';
	import type { CreateSharedSecretResponse } from '$lib/types';

	// Form state
	let receiverEmail = $state('');
	let secretText = $state('');
	let expiresHours = $state(24);
	let maxReads = $state(3);
	let requireOtp = $state(false);
	let sendCopyToSender = $state(false);

	// UI state
	let isCreating = $state(false);
	let createdSecret: CreateSharedSecretResponse | null = $state(null);

	// Validation
	let emailError = $derived(
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

		if (!formValid) {
			flashMessagesStore.addMessage($_('common.formInvalid'));
			return;
		}

		// Check session expiration before creation
		const sessionValid = await checkSessionAndHandle({
			onExpired: 'launch-auth',
			next: '/shared-secret'
		});

		if (!sessionValid) {
			return;
		}

		isCreating = true;

		try {
			const response = await api.createSharedSecret({
				receiver_email: receiverEmail,
				secret_text: secretText,
				expires_hours: expiresHours,
				max_reads: maxReads,
				require_otp: requireOtp,
				send_copy_to_sender: sendCopyToSender
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
							{#if emailError}
								<p class="mt-1 text-sm text-red-600 dark:text-red-400">{emailError}</p>
							{/if}
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
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{$_('sharedSecret.yourUrl')}
						</label>
						<div class="flex gap-2">
							<input
								type="text"
								readonly
								value={createdSecret.sender_url}
								class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-white"
							/>
							<button
								onclick={() => copyToClipboard(createdSecret!.sender_url, 'url')}
								class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors duration-200"
							>
								{$_('sharedSecret.copyUrl')}
							</button>
						</div>
					</div>

					<!-- Receiver URL -->
					<div class="mb-4">
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{$_('sharedSecret.receiverUrl')}
						</label>
						<div class="flex gap-2">
							<input
								type="text"
								readonly
								value={createdSecret.receiver_url}
								class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-white"
							/>
							<button
								onclick={() => copyToClipboard(createdSecret!.receiver_url, 'url')}
								class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors duration-200"
							>
								{$_('sharedSecret.copyUrl')}
							</button>
						</div>
					</div>

					<!-- Reference Hash -->
					<div class="mb-4">
						<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
							{$_('sharedSecret.reference')}
						</label>
						<div class="flex gap-2">
							<input
								type="text"
								readonly
								value={createdSecret.reference_hash}
								class="flex-1 px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 dark:text-white font-mono"
							/>
							<button
								onclick={() => copyToClipboard(createdSecret!.reference_hash, 'reference')}
								class="bg-indigo-600 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded-lg transition-colors duration-200"
							>
								{$_('sharedSecret.copyReference')}
							</button>
						</div>
					</div>

					<!-- OTP (if required) -->
					{#if createdSecret.otp}
						<div class="mb-6">
							<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
								{$_('sharedSecret.otpCode')}
							</label>
							<div class="flex gap-2">
								<input
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
