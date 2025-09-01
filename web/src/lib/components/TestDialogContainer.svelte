<script>
	import { dialogStore } from '$lib/stores/dialog';

	console.log('[DEBUG] TestDialogContainer: Component loaded!');
	console.log('[DEBUG] TestDialogContainer: Initial store value:', $dialogStore);

	// Simple reactive statement
	$: {
		console.log('[DEBUG] TestDialogContainer: Store changed to:', $dialogStore);
		console.log('[DEBUG] TestDialogContainer: Should show dialog:', $dialogStore !== null);
	}
</script>

<div
	style="position: fixed; top: 10px; right: 10px; background: red; color: white; padding: 10px; z-index: 9999;"
>
	TEST DIALOG CONTAINER LOADED
	{#if $dialogStore}
		<div>DIALOG DATA: {JSON.stringify($dialogStore.type)}</div>
	{/if}
</div>

{#if $dialogStore}
	<div
		style="position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 1000; display: flex; align-items: center; justify-content: center;"
	>
		<div style="background: white; padding: 20px; border-radius: 8px;">
			<h2>SIMPLE TEST DIALOG</h2>
			<p>Type: {$dialogStore.type}</p>
			<p>Props: {JSON.stringify($dialogStore.props)}</p>
			<button onclick={() => dialogStore.close()}>Close</button>
		</div>
	</div>
{/if}
