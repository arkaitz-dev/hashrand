# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0] - 2025-08-13

### Added
- **TailwindCSS Integration**: Complete migration from custom CSS to TailwindCSS framework
  - **Modern Styling**: All 7 Lit components now use TailwindCSS utility classes
  - **Responsive Design**: Enhanced mobile-first approach with Tailwind's breakpoint system
  - **Consistent Design System**: Unified color palette, spacing, and typography
  - **Development Experience**: Faster styling iteration with utility-first approach
  
- **Advanced Bundle Optimization**: Major performance improvements for web interface
  - **Terser Minification**: Aggressive JavaScript compression with dead code elimination
  - **Smart Chunking**: Intelligent code splitting for better caching strategies
    - `lit-core`: Framework code cached separately (1.00 kB)
    - `locales`: All 12 languages bundled together (32.38 kB)
    - `index`: Main application logic (3.71 kB)
  - **Console Log Removal**: Production builds strip all debugging output
  - **Tree Shaking**: Aggressive dead code elimination with enhanced settings

### Changed
- **Bundle Size Optimization**: Achieved 48% total bundle size reduction
  - **Before**: 86.16 kB total (19.32 kB gzipped)
  - **After**: 44.93 kB total (11.69 kB gzipped)
  - **Main JS Bundle**: 95% reduction from 78.96 kB to 3.71 kB
  - **Gzipped Performance**: Main bundle reduced from 17.53 kB to 1.46 kB

- **TailwindCSS Configuration**: Optimized for production builds
  - **Core Plugins**: Disabled 20+ unused utility groups for smaller CSS
  - **Custom Animations**: Added fadeIn animation for smooth transitions
  - **PostCSS Integration**: Seamless build integration with Vite

- **Component Architecture**: Complete styling refactoring
  - **Shared Styles**: Created `shared-styles.js` for consistent styling imports
  - **CSS Elimination**: Removed 1,200+ lines of custom CSS across components
  - **Utility Classes**: Migrated to semantic, maintainable Tailwind classes

### Fixed
- **Build System**: Resolved CSS import conflicts in production builds
- **Component Consistency**: Standardized styling patterns across all components
- **Bundle Analysis**: Added development tooling for future optimization monitoring

### Technical Details
- **TailwindCSS 4.1.11**: Latest version with PostCSS plugin architecture
- **Build Tools**: Added `rollup-plugin-visualizer` and `terser` for optimization analysis
- **Vite Configuration**: Enhanced with production-optimized settings
- **Component Migration**: All components maintain identical functionality with modern styling

### Performance Impact
- **Initial Load**: 95% faster main bundle loading
- **Caching**: Improved cache efficiency through chunk separation
- **Network**: 48% less data transfer for complete application
- **Rendering**: Enhanced performance through optimized CSS delivery

## [0.5.0] - 2025-08-12

### Added
- **Tailscale Integration**: Automatic Tailscale serve configuration in development workflow
  - **Smart Detection**: `just dev` automatically detects and configures Tailscale if available
  - **Secure Remote Access**: Enables HTTPS access from any device in your Tailscale network
  - **Automatic Cleanup**: `just stop-dev` properly stops Tailscale serve configuration
  - **Zero Configuration**: Works out-of-the-box without manual Tailscale setup
  - **Hostname Detection**: Automatically displays Tailscale URL (requires `jq` for auto-detection)

### Changed
- **Development Workflow Simplification**: Streamlined Vite and build configuration
  - **Simplified Configuration**: Removed complex proxy prefixes and conditional logic from `vite.config.js`
  - **Clean API Utils**: Simplified `buildApiUrl()` function to work reliably in all environments
  - **Production-Ready Builds**: Ensured production builds work correctly without development-specific code
  - **Reliable Remote Access**: Fixed Tailscale access issues by eliminating problematic URL prefixes

- **NoLookAlike Alphabet Update**: Refined character set for better confusion prevention
  - **New alphabet**: `346789ABCDEFGHJKLMNPQRTUVWXYabcdefghijkmnpqrtwxyz` (49 characters)
  - **Additional exclusions**: 2, 5, S, s, o, u, v, Z (alongside original 0, O, I, l, 1)
  - **Better readability**: Removes more commonly confused characters for improved clarity
  
- **Password Generation Enhancement**: Support for alphabet selection in password mode
  - **CLI**: `--password` now works with both `--no-look-alike` and default `--full-with-symbols`
  - **API**: New `alphabet` parameter for `/api/password` endpoint (accepts "no-look-alike" or "full-with-symbols")
  - **Web UI**: Added alphabet selector in password view with automatic length adjustment
  - **Equivalent Security**: Minimum lengths adjusted to maintain ~130 bits entropy
    - NoLookAlike (49 chars): 24 characters minimum (default)
    - FullWithSymbols (73 chars): 21 characters minimum (default)

### Added
- **Dynamic Password Length Validation**: Intelligent minimum length calculation
  - **Context-aware validation**: Different minimums based on selected alphabet
  - **Automatic adjustment**: Default length changes from 21→24 when using no-look-alike
  - **Security preservation**: Maintains equivalent entropy (~130 bits) across alphabets

- **API Key Generation Enhancement**: Support for alphabet selection in API key mode
  - **CLI**: `--api-key` now works with both `--no-look-alike` and default `--full`
  - **API**: New `alphabet` parameter for `/api/api-key` endpoint (accepts "no-look-alike" or "full")
  - **Web UI**: Added alphabet selector in API key view with automatic length adjustment
  - **Equivalent Security**: Minimum lengths adjusted to maintain ~262 bits entropy
    - NoLookAlike (49 chars): 47 characters minimum (default)
    - Full (62 chars): 44 characters minimum (default)

### Fixed
- **Test Suite Updates**: Updated all password and API key generation tests for new alphabet support
  - **Expanded coverage**: Tests for both alphabet types with appropriate length validation
  - **Boundary testing**: Verification of minimum length enforcement per alphabet
  - **API parameter testing**: Coverage of new alphabet parameter in server endpoints
  - **AlphabetType enum**: Added PartialEq derivation for proper comparison operations

## [0.2.9] - 2025-08-11

### Added
- **Version Display in Web Interface**: Frontend now shows current API version in header
  - New endpoint: `GET /api/version` returning JSON `{"version": "0.2.9"}`
  - JavaScript loads version dynamically on page load via fetch API
  - CSS styling for version display in header (small, subtle text)
  - Automatic version synchronization between API and frontend

### Changed
- **Lit Framework Upgrade**: Successfully migrated from static properties to official Lit 3 decorator syntax
  - **Modern Syntax**: All 5 Web Components now use standard decorators with `accessor` keyword
  - **Official Configuration**: Implemented Lit's recommended Vite + Babel setup for JavaScript decorators
  - **Standard Decorators**: Using Babel plugin version "2023-05" (officially supported by Lit)
  - **Syntax Example**: `@state() accessor lengthValue = 44;` and `@property({ type: String }) accessor hashType = 'generic';`
  
### Technical Details  
- **Rust Version**: Updated to Rust 1.89.0 (latest stable) for optimal performance and modern language features
- **Dependencies**: Updated Rust crates to latest compatible versions (clap, libc, proc-macro2, etc.)
- **Vite Configuration**: Streamlined Babel configuration following Lit official documentation
  - Removed experimental decorator versions and custom configurations
  - Simplified to official `@babel/plugin-proposal-decorators` with version "2023-05"
  - Proper plugin ordering with Babel placed after HTML transformation plugins
- **Component Modernization**: All components updated to use modern Lit 3 patterns
  - `hash-generator.js`: Migrated 4 state properties to accessor syntax
  - `hash-result.js`: Migrated 2 properties and 3 state properties to accessor syntax  
  - `api-key-view.js`, `password-view.js`, `generic-hash-view.js`: Migrated 1 state property each
  - Removed constructor initialization (handled automatically by decorators)
- **Production Compatibility**: Resolved component visibility issues in release builds
  - Components now render correctly in both development and production modes
  - Fixed decorator transpilation for embedded asset serving
  - Maintained all functionality while upgrading to modern syntax

### Benefits
- **Modern Development**: Up-to-date with latest Lit 3 best practices and official recommendations
- **Cleaner Code**: Reduced boilerplate with decorator syntax eliminating manual property definitions
- **Future-Proof**: Using official Lit configuration ensures compatibility with future framework updates  
- **Better DX**: Improved developer experience with modern JavaScript patterns
- **Production Ready**: Fully tested and verified working in both development and release modes

## [0.2.8] - 2025-08-09

### Added
- **Embedded Static Assets**: Production builds now embed web assets directly in the binary
  - Development mode: API-only server (Vite handles frontend)
  - Release mode: Self-contained binary with embedded web interface
  - Zero external file dependencies for production deployment
  - Single ~3.1MB binary includes complete web UI functionality
  
### Changed  
- **Build Process Enhancement**: Conditional compilation based on build profile
  - Debug builds serve only API endpoints (no static files)
  - Release builds include embedded assets from `dist/` directory
  - Simplified production deployment (single binary file)
  
### Dependencies
- Added `include_dir ^0.7` for compile-time directory embedding
- Added `mime_guess ^2.0` for proper MIME type detection of embedded assets

### Technical
- Implemented compile-time asset embedding using `include_dir` macro
- Added conditional routing logic with `#[cfg(debug_assertions)]`
- Custom service function for serving embedded assets with proper headers
- SPA routing support (fallback to index.html for unknown routes)

## [0.2.7] - 2025-08-09

### Changed
- **Complete Project Restructuring**: Major architectural reorganization for better maintainability
  - **Frontend Separation**: All web UI files moved to dedicated `web-ui/` directory
    - Components: `web-ui/src/components/`
    - CSS: `web-ui/src/css/`
    - Entry points: `web-ui/index.html`, `web-ui/src/index.js`
  - **Rust Code Modularization**: Backend code organized into logical modules
    - CLI module: `src/cli/` (args.rs, mod.rs)  
    - Server module: `src/server/` (routes.rs, config.rs, mod.rs)
    - Utils module: `src/utils/` (validation.rs, file_ops.rs, audit.rs, mod.rs)
  - **Configuration Updates**: Vite configured to work with new directory structure
    - Root: `web-ui` for clean source separation
    - Build output: `../dist` to maintain production workflow
  - **Preserved Workflow**: All commands still run from project root
    - `npm run dev` and `npm run build` work from project root
    - `cargo build`, `cargo run`, `cargo test` work from project root

### Added
- **Configurable API Key Length**: API keys can now be generated with custom lengths between 44-64 characters
  - CLI: `hashrand --api-key 60` generates a 60-character API key (plus ak_ prefix)
  - Web UI: Added length slider (44-64) in API Key configuration view
  - API: `/api/api-key?length=60` endpoint accepts length parameter
  - Default remains 44 characters for backward compatibility
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

### Technical Details
- **Code Organization**: Clean separation between frontend (web-ui/) and backend (src/)
- **Module Structure**: Rust code split into focused modules for better maintainability
- **Build Process**: Maintained existing build workflow with improved internal organization
- **Legacy Cleanup**: Removed old unused Web Component files from static/
- **Zero Breaking Changes**: All existing functionality preserved and tested (46/46 tests passing)

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