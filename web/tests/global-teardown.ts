/**
 * Playwright Global Teardown
 *
 * Executed ONCE after all tests complete
 * Deactivates email dry-run mode to restore normal email operation
 */

const API_BASE_URL = process.env.API_BASE_URL || 'http://localhost:3000';

async function globalTeardown() {
	console.log('\n📧 [GLOBAL TEARDOWN] Deactivating email DRY-RUN mode...');

	try {
		const response = await fetch(`${API_BASE_URL}/api/test/dry-run?enabled=false`);

		if (response.ok) {
			const data = await response.json();
			if (data.email_dry_run === false) {
				console.log('✓ Email mode restored to normal (emails will be sent)');
			} else {
				console.warn('⚠ Warning: Dry-run deactivation response unexpected:', data);
			}
		} else {
			console.warn(
				`⚠ Warning: Failed to deactivate dry-run mode (HTTP ${response.status})`
			);
		}
	} catch (error) {
		console.error('⚠ Error deactivating dry-run mode:', error);
		console.warn('  Server may have stopped. Email mode may still be in dry-run.');
	}

	console.log('');
}

export default globalTeardown;
