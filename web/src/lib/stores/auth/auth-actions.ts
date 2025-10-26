/**
 * Auth Actions Module - Authentication Actions (Main Orchestrator)
 *
 * Central module for authentication operations including login, logout, and key rotation.
 * Provides unified interface and re-exports specialized modules.
 *
 * ARCHITECTURE:
 * - Login: Magic link authentication flow (System A + System B key generation)
 * - Logout: Session termination and cleanup
 * - Key Rotation: Permanent public keys publication (System B)
 *
 * DUAL-KEY SYSTEM:
 * - System A: Temporary session keys (API communication)
 * - System B: Permanent user keys (user-to-user E2EE)
 *
 * SOLID PRINCIPLES:
 * - Single Responsibility: Each module has one clear purpose
 * - Open/Closed: Easy to extend without modifying existing code
 * - Dependency Inversion: Modules depend on abstractions, not implementations
 *
 * @see auth-actions/login.ts for authentication operations
 * @see auth-actions/logout.ts for session termination
 * @see auth-actions/key-rotation.ts for key publication
 */

// Re-export Login functions (magic link authentication)
export { requestMagicLink, validateMagicLink } from './auth-actions/login';

// Re-export Logout functions (session termination and cleanup)
export { clearLocalAuthData, logout } from './auth-actions/logout';

// Re-export Key Rotation functions (System B public key publication)
export { publishPermanentKeys } from './auth-actions/key-rotation';
