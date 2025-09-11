# Changelog

All notable changes to HashRand Spin project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
