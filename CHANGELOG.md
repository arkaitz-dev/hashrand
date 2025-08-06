# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-08-06

### Added
- Initial release of hashrand CLI tool
- Cryptographically secure random string generation using nanoid
- Multiple alphabet options:
  - Base58 (default, Bitcoin alphabet)
  - No look-alike (excludes 0, O, I, l, 1)
  - Full alphanumeric (62 characters)
  - Full with symbols (73 characters)
- Customizable hash length (2-128 characters)
- Raw output mode (`--raw`) for scripting
- Collision detection (`--check`) to avoid existing filenames
- File and directory creation with random names (`--touch`, `--mkdir`)
- Prefix and suffix support for structured naming
- Custom path support for organized file placement
- API key generation mode (`--api-key`) with ak_ prefix format
- Password generation mode (`--password`) with configurable length
- **Security Features:**
  - Path validation and canonicalization to prevent directory traversal attacks
  - Resource protection with directory traversal limits (10 levels deep)
  - File count limits (100,000 entries) to prevent DoS attacks
  - Generation attempt limits (1,000 tries) to prevent infinite loops
  - Graceful error handling with informative messages
- **Unix Permissions Control:**
  - `--file-mode` option for setting file permissions (octal format)
  - `--dir-mode` option for setting directory permissions (octal format)
- **Audit Logging System:**
  - `--audit-log` flag for enabling operation tracking
  - `HASHRAND_AUDIT_LOG` environment variable support
  - Timestamp logging with Unix epoch for consistency
  - Comprehensive operation logging (generation, creation, permissions)
  - Security-compliant logging (no sensitive data exposed)

### Security
- Enhanced error handling replaces panic-prone `.expect()` calls with proper `Result` types
- Path traversal attack prevention through canonicalization and validation
- Resource exhaustion protection with configurable limits
- Input validation for all parameters within safe ranges
- Secure defaults maintained when optional parameters aren't specified

### Changed
- Password default length increased from 14 to 21 characters for better entropy
- API key format standardized to ak_ + 44 characters (47 total) for 256-bit security

### Dependencies
- [nanoid](https://crates.io/crates/nanoid) 0.4.0 - Secure random string generation
- [clap](https://crates.io/crates/clap) 4.5.42 - Command-line argument parsing with derive features
- [walkdir](https://crates.io/crates/walkdir) 2.5.0 - Recursive directory traversal for collision detection
- [tempfile](https://crates.io/crates/tempfile) 3.13.0 - Development dependency for testing

### Technical Details
- Built with Rust 2024 edition for modern language features
- Comprehensive test suite with 30+ tests covering all functionality
- Cross-platform support with Unix-specific features where appropriate
- Memory-safe implementation leveraging Rust's ownership system
- Zero-dependency cryptographic randomness through nanoid's secure defaults