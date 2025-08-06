# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-08-06

### Changed
- **Server Security**: HTTP server now binds to `127.0.0.1` (localhost) by default instead of `0.0.0.0`
- **API Response Format**: All API endpoints now return raw text (no newline) by default for better integration
- **API Simplification**: Removed `check` parameter from API endpoints as it doesn't apply to server mode

### Added
- **`--listen-all-ips` flag**: New option to bind server to all network interfaces (`0.0.0.0`) when needed

### Security
- **Localhost-only binding**: Server now defaults to localhost-only access for improved security
- **Explicit all-interfaces binding**: Requires explicit `--listen-all-ips` flag to expose to network

### Dependencies
- **Updated** [axum](https://crates.io/crates/axum) from 0.7.9 to 0.8.4 - Latest stable version with performance improvements
- **Updated** Various minor dependency updates for security and compatibility

### Documentation
- **Added** `LICENSE` file with MIT license
- **Added** `docs/API.md` with comprehensive API documentation
- **Enhanced** `Cargo.toml` with package metadata for crates.io publishing

## [0.2.0] - 2025-08-06

### Added
- **HTTP Server Mode**: New `-s, --serve <PORT>` option to run hashrand as HTTP API server
- **REST API Endpoints**: Three new endpoints for remote hash generation:
  - `GET /api/generate` - Generate random hash with full CLI functionality
  - `GET /api/api-key` - Generate secure API keys (ak_ prefixed)
  - `GET /api/password` - Generate secure passwords with symbol support
- **Plain Text API Responses**: All endpoints return plain text for easy integration
- **Query Parameter Support**: Full customization via URL parameters:
  - `length`, `alphabet`, `raw`, `check`, `prefix`, `suffix` for `/api/generate`
  - `raw` parameter for `/api/api-key` and `/api/password`
  - `length` parameter for `/api/password` (21-44 character range)
- **Security-First API Design**: File system operations excluded from HTTP endpoints
- **Comprehensive Server Testing**: 6 new tests covering all HTTP server functionality

### Dependencies
- **Added** [tokio](https://crates.io/crates/tokio) 1.0 - Async runtime for HTTP server
- **Added** [axum](https://crates.io/crates/axum) 0.7 - Web framework for REST API endpoints  
- **Added** [serde](https://crates.io/crates/serde) 1.0 - Query parameter deserialization

### Technical Details
- **Async Architecture**: Full tokio async/await integration for server mode
- **Code Refactoring**: Business logic extracted into reusable functions for CLI and API
- **Dual Mode Operation**: Single binary works as both CLI tool and HTTP server
- **Test Coverage**: Expanded to 36 total tests (up from 30)
- **Zero Breaking Changes**: All existing CLI functionality preserved

### Usage Examples
```bash
# Start HTTP server
hashrand --serve 8080

# API usage
curl "http://localhost:8080/api/generate?length=16&alphabet=full&raw=true"
curl "http://localhost:8080/api/api-key?raw=false"
curl "http://localhost:8080/api/password?length=30"
```

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