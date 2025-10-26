/**
 * Dual Keypair Helper for Playwright Tests
 *
 * Generates Ed25519 + X25519 keypairs for dual-key system tests
 * Uses existing generate_dual_keypairs.js script for consistency
 */

import { execSync } from 'child_process';
import path from 'path';

export interface DualKeypairs {
	ed25519_pub_key: string;
	x25519_pub_key: string;
}

/**
 * Generate dual keypairs (Ed25519 + X25519) using Node.js script
 *
 * This ensures consistency with bash tests and backend expectations.
 * Private keys are stored in .test-ed25519-private-key and .test-x25519-private-key
 *
 * @returns Dual public keys in hex format (64 chars each)
 */
export function generateDualKeypairs(): DualKeypairs {
	const scriptsDir = path.resolve(__dirname, '../../../scripts');
	const scriptPath = path.join(scriptsDir, 'generate_dual_keypairs.js');

	try {
		const output = execSync(`node "${scriptPath}"`, {
			encoding: 'utf-8',
			cwd: path.resolve(__dirname, '../..')
		});

		return JSON.parse(output) as DualKeypairs;
	} catch (error) {
		throw new Error(`Failed to generate dual keypairs: ${error}`);
	}
}

/**
 * Create magic link payload with dual-key format
 *
 * @param email - User email
 * @param dualKeypairs - Generated dual keypairs
 * @param next - Optional redirect path (default: '/')
 * @returns Payload object ready for SignedRequest
 */
export function createMagicLinkPayload(
	email: string,
	dualKeypairs: DualKeypairs,
	next: string = '/'
): object {
	return {
		email,
		email_lang: 'en',
		next,
		ed25519_pub_key: dualKeypairs.ed25519_pub_key,
		x25519_pub_key: dualKeypairs.x25519_pub_key,
		ui_host: 'localhost'
	};
}
