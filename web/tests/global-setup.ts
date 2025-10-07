/**
 * Playwright Global Setup
 *
 * Executed ONCE before all tests run
 * Activates email dry-run mode to prevent real email sending during tests
 */

const API_BASE_URL = process.env.API_BASE_URL || 'http://localhost:3000';

async function globalSetup() {
	console.log('\n📧 [GLOBAL SETUP] Activating email DRY-RUN mode...');

	try {
		const response = await fetch(`${API_BASE_URL}/api/test/dry-run?enabled=true`);

		if (response.ok) {
			const data = await response.json();
			if (data.email_dry_run === true) {
				console.log('✓ Email dry-run mode ACTIVATED (emails will NOT be sent)');
			} else {
				console.warn('⚠ Warning: Dry-run response unexpected:', data);
			}
		} else {
			console.warn(
				`⚠ Warning: Failed to activate dry-run mode (HTTP ${response.status})`
			);
			console.warn('  Tests will continue but emails may be sent...');
		}
	} catch (error) {
		console.error('⚠ Error activating dry-run mode:', error);
		console.warn('  Server may not be running. Tests will continue...');
	}

	console.log('');
}

export default globalSetup;
