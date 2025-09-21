# Changelog

All notable changes to HashRand Spin project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.21.0] - 2025-09-21

### Added

- **Enterprise-Grade SOLID Architecture Refactoring**: Complete transformation of frontend codebase from monolithic to modular architecture
  - **94% code reduction** in crypto.ts (471→30 lines) with 5 specialized modules
  - **93% code reduction** in ed25519.ts (303→21 lines) with 6 specialized modules
  - **61% code reduction** in api.ts (546→215 lines) with 4 DRY modules
  - **61% code reduction** in session-manager.ts (557→216 lines) with 6 specialized modules
  - **53% code reduction** in auth.ts (581→274 lines) with 5 SRP modules
- **26 New Specialized Modules Created**:
  - **Crypto Modules** (5): `crypto-core.ts`, `crypto-encoding.ts`, `crypto-storage.ts`, `crypto-url-operations.ts`, `crypto-utils.ts`
  - **Ed25519 Modules** (6): `ed25519-types.ts`, `ed25519-keygen.ts`, `ed25519-database.ts`, `ed25519-signing.ts`, `ed25519-utils.ts`, `ed25519-api.ts`
  - **Auth Store Modules** (5): `auth-storage.ts`, `auth-crypto-tokens.ts`, `auth-cleanup.ts`, `auth-session.ts`, `auth-actions.ts`
  - **Session Modules** (6): `session-db.ts`, `session-crypto.ts`, `session-auth.ts`, `session-preferences.ts`, `session-auth-flow.ts`
  - **API Modules** (4): `api-helpers.ts`, `api-generators.ts`, `api-seed-operations.ts`, `api-auth-operations.ts`
- **Universal Composables System**: Created reusable composables eliminating 840+ lines of duplicate code
  - `useGenerationWorkflow.ts` - Unified generation logic across all endpoints
  - `useFormParams.ts` - Centralized form parameter management

### Changed

- **SOLID Principles Implementation**: Each module now follows Single Responsibility Principle
- **DRY Code Elimination**: Removed all code duplication across generation endpoints
- **Modular Import/Export System**: Clean centralized exports with backward compatibility
- **File Size Optimization**: All TypeScript files now under 225 lines (enterprise standard)
- **Legacy Code Cleanup**: Removed obsolete `session-migration.ts` and backup files

### Technical Improvements

- **Zero Breaking Changes**: 100% API compatibility preserved during refactoring
- **Enterprise Standards**: All modules follow <225 line limit with clear responsibilities
- **Type Safety**: Complete TypeScript coverage across all new modules
- **Clean Architecture**: Separation of concerns with specialized module responsibilities
- **Performance**: Faster compilation with granular imports and smaller modules
- **Maintainability**: Each module easily testable and modifiable in isolation

### Code Quality

- **TypeScript**: 0 compilation errors, clean type definitions
- **ESLint**: Resolved all critical linting issues, minimal warnings remain
- **Prettier**: Code formatting applied across all new modules
- **Documentation**: JSDoc comments on all public functions and interfaces

## [0.20.0] - 2025-09-11

### Added

- **Advanced URL Parameter Encryption System**: Revolutionary cryptographic system for complete URL privacy protection
  - ChaCha20-Poly1305 AEAD encryption for all URL parameters
  - Random prehash seed generation (content-independent cryptographic keys)
  - Triple token system (cipher/nonce/HMAC keys, 32 bytes each)
  - Base64URL encoding for URL-safe transmission
  - FIFO rotation system with 20-seed limit for automatic cleanup
  - 32-byte crypto salt for internal noise generation
  - 8-byte cryptographic keys for efficient KV storage management
- **Cryptographic Module** (`web/src/lib/crypto.ts`):
  - `cryptoHashGen()` - Blake2b-keyed + ChaCha8RNG pipeline
  - `encryptUrlParams()` / `decryptUrlParams()` - Complete encryption workflow
  - `generatePrehashSeed()` - Random seed generation
  - `storePrehashSeed()` / `getPrehashSeed()` - KV storage with FIFO rotation
  - Base64URL utilities for URL-safe encoding
- **Enhanced Auth Store**: Extended to support 3 cryptographic tokens
- **SessionStorage KV System**: Key-value storage with automatic rotation

### Security

- **Complete Privacy Protection**: URLs encrypted against browser history inspection
- **Enterprise-Grade Cryptography**: @noble/hashes + @noble/ciphers integration
- **Pattern Analysis Prevention**: Random seeds eliminate content-based attacks
- **Memory Management**: FIFO rotation prevents unlimited storage growth

### Technical

- **Zero Breaking Changes**: All existing APIs preserved
- **Clean Compilation**: No errors or warnings
- **Type Safety**: Complete TypeScript coverage
- **Performance Optimized**: Efficient cryptographic operations

## Previous Versions

### [0.19.9] - 2025-09-08

- **Environment-Specific Configuration**: Complete separation of development/production configs
- **Project Cleanup**: Systematic removal of legacy files and configurations
- **Justfile Updates**: All commands use appropriate configuration per environment

### [0.19.8] - 2025-09-07

- **Enterprise-Grade Architecture Refactoring**: Transformed monolithic code to modular architecture
- **Code Quality Zero Warnings**: Complete elimination of compilation warnings

### [0.19.0] - 2025-08-29

- **Zero Knowledge Authentication**: Complete ZK system where server never stores personal data
- **Cryptographic User IDs**: Blake2b-derived identity system with Base58 usernames
- **Magic Link Authentication**: Passwordless authentication with cryptographic integrity

---

**HashRand Spin**: Secure hash generation with complete privacy protection. Built with modern web technologies and cryptographic best practices.
