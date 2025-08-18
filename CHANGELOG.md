# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-08-18

### Added
- Initial implementation of HashRand Spin API
- **GET /api/generate** endpoint for customizable hash generation
  - Support for length parameter (2-128 characters)
  - Multiple alphabet types: base58, no-look-alike, full, full-with-symbols
  - Prefix and suffix support (max 32 characters each)
  - Raw output formatting option
- **GET /api/password** endpoint for secure password generation
  - Dynamic minimum length based on alphabet type
  - Length range validation (21-44 characters)
  - Symbol and no-look-alike alphabet support
- **GET /api/api-key** endpoint for API key generation
  - Automatic ak_ prefix for all generated keys
  - Length validation (44-64 characters)
  - Support for full and no-look-alike alphabets
- **GET /api/version** endpoint returning JSON version information
- Comprehensive alphabet system with 4 character sets:
  - Base58: 58 characters (Bitcoin standard, excludes confusing characters)
  - No-look-alike: 49 characters (maximum readability)
  - Full: 62 characters (complete alphanumeric)
  - Full-with-symbols: 73 characters (maximum entropy)
- Cryptographically secure random generation using nanoid
- Complete parameter validation and error handling
- Modular architecture with clean separation of concerns
- Comprehensive test suite with 43 automated test cases
- Project restructured into workspace with api/ directory
- Support for Rust 2024 edition
- **justfile** for streamlined development workflow with 20+ commands
  - Development tasks: `just dev`, `just build`, `just test`
  - Background server support: `just dev-bg`, `just watch`, `just stop`, `just status`
  - Code quality: `just check`, `just lint`, `just fmt`
  - Information: `just info`, `just examples`, `just deps`
  - CI/CD: `just pre-commit`, `just perf-test`
- **Background development server functionality**
  - `just dev-bg` - Start server in background with PID tracking
  - `just watch` - Start background server and follow logs
  - `just status` - Check background server status
  - PID file management in `.spin-dev.pid`
  - Log file management in `.spin-dev.log`
  - Automatic cleanup on server stop

### Technical Details
- Built with Fermyon Spin WebAssembly framework
- Uses spin-sdk 3.1.0 for HTTP component functionality
- Implements cdylib crate type for WASM compatibility
- Targets wasm32-wasip1 WebAssembly platform
- Workspace structure for better code organization

### Dependencies
- `spin-sdk = "3.1.0"` - Core Spin framework
- `nanoid = "0.4.0"` - Secure random ID generation
- `serde = "1.0.219"` - Serialization framework
- `serde_json = "1.0.142"` - JSON serialization
- `anyhow = "1"` - Error handling

### Testing
- 43 comprehensive test cases covering all endpoints
- Parameter validation testing
- Edge case and error condition testing
- Alphabet-specific character validation
- Performance and consistency testing
- 100% test success rate achieved

### Documentation
- Complete README.md with API documentation
- Detailed endpoint descriptions and examples
- Project structure documentation
- Setup and deployment instructions
- CLAUDE.md for development guidance

## [Unreleased]

### Planned Features
- Performance benchmarking
- Additional alphabet types
- Batch generation endpoints
- Configuration file support
- Metrics and monitoring
- Docker containerization
- Helm charts for Kubernetes deployment

---

## Version History Summary

- **0.1.0** (2025-08-18) - Initial release with complete API implementation