/**
 * Session Auth Flow Module - Temporary Auth Flow Data Management
 *
 * Single Responsibility: Handle temporary authentication flow data
 * Part of session-manager.ts refactorization to apply SOLID principles
 */

import { sessionDB } from './session-db';

/**
 * Set pending auth email (during magic link flow)
 */
export async function setPendingAuthEmail(email: string | null): Promise<void> {
	const session = await sessionDB.getSession();
	session.authFlow.pending_email = email;
	await sessionDB.saveSession(session);
}

/**
 * Get pending auth email
 */
export async function getPendingAuthEmail(): Promise<string | null> {
	const session = await sessionDB.getSession();
	return session.authFlow.pending_email;
}

/**
 * Clear pending auth email (after auth completion)
 */
export async function clearPendingAuthEmail(): Promise<void> {
	await setPendingAuthEmail(null);
}
