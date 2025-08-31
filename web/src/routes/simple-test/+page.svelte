<script>
	import { dialogStore } from '$lib/stores/dialog';
	
	let count = $state(0);
	let logs = $state([]);
	
	function addLog(message) {
		logs = [...logs, `${new Date().toISOString()}: ${message}`];
		console.log(`[LOG] ${message}`);
	}
	
	function increment() {
		addLog('increment called!');
		count++;
	}
	
	function testDialog() {
		addLog('Testing dialog...');
		
		// Subscribe to dialog store changes to see if it updates
		const unsubscribe = dialogStore.subscribe((value) => {
			addLog(`DIALOG STORE CHANGE: ${JSON.stringify(value)}`);
		});
		
		// Pass realistic next parameters like a real generation request
		const nextParams = {
			endpoint: 'custom',
			length: 21,
			alphabet: 'base58',
			prefix: 'test_'
		};
		
		addLog('About to call dialogStore.show()');
		dialogStore.show('auth', nextParams);
		addLog('dialogStore.show() completed');
		
		// Unsubscribe after a few seconds to avoid spam
		setTimeout(() => {
			unsubscribe();
			addLog('Unsubscribed from dialog store');
		}, 5000);
	}
	
	function testApi() {
		addLog('Testing API call...');
		fetch('/api/login/', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				email: 'simple-test@example.com',
				ui_host: window.location.origin
			})
		})
		.then(response => response.json())
		.then(data => addLog(`API success: ${JSON.stringify(data)}`))
		.catch(error => addLog(`API error: ${error.message}`));
	}
	
	addLog('Simple test page loaded');
</script>

<div class="container mx-auto px-4 py-8">
	<h1 class="text-3xl font-bold mb-6">Simple Test</h1>
	
	<div class="mb-6">
		<p class="mb-4">Count: {count}</p>
		
		<div class="space-y-2 mb-4">
			<button onclick={increment} class="px-4 py-2 bg-blue-600 text-white rounded">Test Counter</button>
			<button onclick={testDialog} class="px-4 py-2 bg-green-600 text-white rounded">Test Dialog</button>
			<button onclick={testApi} class="px-4 py-2 bg-red-600 text-white rounded">Test API</button>
		</div>
	</div>
	
	<div class="bg-gray-100 dark:bg-gray-800 p-4 rounded-lg">
		<h2 class="text-xl font-semibold mb-4">Logs:</h2>
		<div class="space-y-1">
			{#each logs as log}
				<div class="text-sm font-mono bg-white dark:bg-gray-700 p-2 rounded">{log}</div>
			{/each}
		</div>
	</div>
</div>