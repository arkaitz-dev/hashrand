# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Configurable API Key Length**: API keys can now be generated with custom lengths between 44-64 characters
  - CLI: `hashrand --api-key 60` generates a 60-character API key (plus ak_ prefix)
  - Web UI: Added length slider (44-64) in API Key configuration view
  - API: `/api/api-key?length=60` endpoint accepts length parameter
  - Default remains 44 characters for backward compatibility

### Changed
- **Web Interface Navigation Improvements**:
  - Separated configuration views from result display
  - Created dedicated result view with improved navigation
  - Added three-button navigation in result view (Back to Config, Back to Menu, Regenerate)
  - Fixed event propagation issues with `composed: true` for Shadow DOM events
  
### Fixed
- **Navigation Bug Fixes**: 
  - Fixed "Back to Menu" button not working in configuration views
  - Fixed "Generate" buttons not navigating to result view
  - Added proper event composition for cross-Shadow DOM communication

## [0.2.6] - 2025-08-09

### Fixed
- **Web Interface JavaScript Issues**: Resolved critical problems preventing menu component visibility
  - **Decorator Syntax Elimination**: Removed all experimental JavaScript decorators (`@state`, `@property`, `@query`) from Lit components
  - **Script Loading Timing**: Fixed script positioning by moving JavaScript from `<head>` to end of `<body>` 
  - **Standard Lit Syntax**: Converted all components to use `static properties = {}` instead of decorators
  - **DOM Query References**: Replaced `@query` decorators with direct `shadowRoot.querySelector()` calls

### Changed
- **Vite Build Configuration**: Enhanced with custom HTML transformation plugin
  - Created `move-scripts-to-body` plugin to automatically position module scripts at end of HTML body
  - Ensures proper Web Component initialization timing in production builds
  - Maintains CSS in `<head>` for optimal loading order

### Technical Details
- **Component Syntax Conversion**: 
  - `hash-generator.js`: Removed `@state()` decorators, converted to constructor initialization
  - `generic-hash-view.js`: Removed `@state()` and `@query()` decorators, updated DOM queries
  - `password-view.js`: Removed `@state()` and `@query()` decorators, updated form references  
  - `api-key-view.js`: Removed `@state()` decorators, converted to standard properties
- **Build Optimization**: Bundle size reduced to 43.68 kB (9.79 kB gzipped) with improved loading
- **Browser Compatibility**: Eliminated experimental JavaScript features for broader compatibility

### Migration Notes
- **Zero Breaking Changes**: All UI functionality and appearance preserved
- **Production Workflow**: Same build process (`npm run build` → `cargo run --serve`)
- **Development Workflow**: Same development process (`npm run dev`)
- **Component Behavior**: Identical user experience with improved reliability

## [0.2.5] - 2025-08-09

### Changed
- **Web Interface Architecture**: Complete migration from standard Web Components to Lit framework
  - Migrated all 4 web components (HashGenerator, GenericHashView, PasswordView, ApiKeyView) to Lit
  - Implemented modern development tooling with Vite build system
  - Added Hot Module Replacement (HMR) for improved development experience
  - Production builds now optimized: 47.90 kB → 11.53 kB gzipped
  
### Fixed
- **Production Server Configuration**: Fixed web interface serving in production mode
  - Changed server to serve from `dist/` directory instead of `static/`
  - Ensures production builds work correctly with `cargo run --serve`
  - Clean separation: source files (`static/`) vs compiled files (`dist/`)

### Added
- **Modern Development Workflow**: 
  - `npm run dev` for development server with HMR on localhost:3000
  - `npm run build` for optimized production builds
  - Vite configuration with API proxy for seamless development
  - TypeScript-style decorators (@state, @query) converted to JavaScript

### Technical Details
- **Dependencies**: Added Lit 3.3.1 and Vite 7.1.1 as development dependencies
- **Build System**: Vite replaces manual Web Components development
- **Bundle Optimization**: CSS extracted and optimized (3.29 kB → 1.24 kB gzipped)
- **Development Server**: Proxy configuration for seamless API integration during development

### Migration Notes
- Development: Use `npm run dev` for development server with live reloading
- Production: Run `npm run build` then `cargo run --serve PORT` to serve optimized files
- All functionality preserved: UI behavior and CSS styling remain identical
- No changes to Rust backend except serving directory (static/ → dist/)

## [0.2.4] - 2025-08-09

### Changed
- **Web Interface CSS Refactoring**: Improved maintainability and performance of web components
  - Extracted common CSS styles from three JavaScript components (api-key.js, generic-hash.js, password.js)
  - Moved shared styles to external CSS file (`/static/css/main.css`)
  - Created reusable CSS classes with "wc-" prefix for web component consistency
  - Reduced code duplication by ~180 lines per component (540 total lines saved)

### Technical Details
- **CSS Architecture**: Centralized web component styles for better maintainability
- **Performance**: Reduced bundle size and improved CSS caching with external stylesheet
- **Code Quality**: Eliminated duplicate styles across multiple JavaScript components
- **Standards Compliance**: Better separation of concerns following web development best practices

### Benefits
- **Reduced Code Duplication**: Eliminated repetitive CSS across three components
- **Easier Maintenance**: Single source of truth for web component styling
- **Better Performance**: External CSS files can be cached by browsers
- **Improved Developer Experience**: Consistent styling patterns with "wc-" prefix convention

## [0.2.3] - 2025-08-07

### Added
- **Interactive Web Interface**: Complete web UI for hash generation with menu navigation
  - Main menu with three card-based options (Generic Hash, Password, API Key)
  - Separate views for each generation mode with specific controls
  - Smooth transitions and animations between views
  - Copy-to-clipboard functionality for all generated results
  - Responsive design with mobile-first approach
  - Web Components architecture with Shadow DOM encapsulation

### Fixed
- **Web Interface Navigation**: Resolved multiple navigation and display issues
  - Fixed Shadow DOM CSS encapsulation preventing styles from applying
  - Corrected successive generation bug showing "Generating..." without updating
  - Fixed API Key view not displaying due to incorrect data-mode attribute
  - Ensured only one view is visible at a time
  - Fixed back-to-menu buttons functionality

### Changed
- **Web Interface UX**: Improved user experience with menu-based navigation
  - No automatic API calls on initial page load
  - Clear separation between generation modes
  - Mode-specific forms with appropriate options

### Technical Details
- **Web Components**: Implemented using standard Web Components with Shadow DOM
- **CSS Architecture**: All styles properly encapsulated within Shadow DOM
- **State Management**: View switching managed through active class system
- **DOM Preservation**: Fixed result display to preserve copy button during updates

## [0.2.2] - 2025-08-06

### Added
- **Server Security Enhancements**: New security options for HTTP server mode
  - `--max-param-length <N>`: Configurable limit for prefix/suffix parameters (default: 32 characters)
  - `--enable-rate-limiting`: Enable per-IP rate limiting for DoS protection
  - `--rate-limit <N>`: Configure requests per second limit (default: 100/second per IP)
  - `--enable-cors`: Optional CORS headers for cross-origin requests
  - `--max-body-size <N>`: Configurable request body size limit (default: 1024 bytes)

### Changed
- **Security by Default**: All new security features are disabled by default for optimal performance
- **Enhanced Server Output**: Server startup now displays enabled security features and their configuration

### Security
- **Parameter Validation**: HTTP endpoints now validate prefix/suffix parameter lengths
- **Rate Limiting**: Configurable per-IP rate limiting prevents request flooding attacks
- **Request Size Limiting**: Configurable limits prevent resource exhaustion via large requests
- **CORS Control**: Optional CORS support for controlled cross-origin access

### Dependencies
- **Added** [tower](https://crates.io/crates/tower) 0.5 - Service middleware for HTTP server
- **Added** [tower-http](https://crates.io/crates/tower-http) 0.6 - CORS and request limiting middleware

### Technical Details
- **Custom Rate Limiter**: Implemented efficient per-IP rate limiting with configurable windows
- **Middleware Architecture**: Modular security features using tower middleware layers
- **Backward Compatibility**: All new features are opt-in, maintaining full backward compatibility
- **Test Coverage**: Added comprehensive tests for all security features (45 total tests)

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
- **SSL/TLS Requirement**: Documentation updated to emphasize MANDATORY use of reverse proxy with HTTPS for production deployments

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