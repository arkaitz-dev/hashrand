# CLAUDE.md

This file provides comprehensive guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Session Summary - 2025-08-07 (Complete ✅)

**Duration**: Full session implementing web interface enhancements
**Git Branch**: master 
**Test Status**: 45/45 tests passing
**Version**: 0.2.3
**Focus**: Interactive Web Interface with Menu Navigation

### 🎯 Accomplished Tasks

#### 1. Web Interface Menu Navigation
**Problem**: Initial web interface had all generation options visible simultaneously
**Solution**: Implemented menu-based navigation with separate views

**Features Added**:
- Main menu with three card-based options (Generic Hash, Password, API Key)
- Separate dedicated views for each generation mode
- Smooth transitions and animations between views
- Back-to-menu navigation from all views
- No automatic API calls on initial page load

#### 2. Shadow DOM CSS Fixes
**Problem**: CSS styles not applying inside Shadow DOM component
**Solution**: Moved all required styles inside the Shadow DOM

**Technical Details**:
- Complete CSS encapsulation within Web Component
- Proper view switching with active class management
- Responsive grid layout with hover effects

#### 3. Bug Fixes
**Issues Resolved**:
- Fixed successive generation bug showing "Generating..." without updating
- Corrected API Key view not displaying (data-mode="apikey" → "apiKey")
- Fixed back buttons not functioning properly
- Ensured only one view visible at a time
- Preserved DOM structure to prevent copy button loss during updates

### 📊 Session Metrics
- **Commits Created**: 3 feature/fix commits
- **Files Modified**: `src/main.rs` (main implementation)
- **Lines Changed**: +650 additions, -105 deletions
- **Test Suite**: All 45 tests passing
- **New Files**: Implementation tracking files in `implement/`

### 🚀 Production Readiness
The web interface is now production-ready with:
- Intuitive menu-based navigation
- Clear separation between generation modes
- Mode-specific forms with appropriate options
- Responsive design for all devices
- Standard Web Components architecture
- No external dependencies

### 📝 Documentation Updates
- **CHANGELOG.md**: Added v0.2.3 entry with complete feature list
- **README.md**: Added web interface section with features
- **CLAUDE.md**: Complete session documentation

---

## Session Summary - 2025-08-07 (Complete ✅)

**Focus**: Web Interface Implementation & Code Quality Improvements
**Duration**: Full implementation of web UI and warning fixes
**Result**: Production-ready web interface with clean compilation

### Accomplished Tasks

#### 1. Web Interface Implementation
**Feature**: Interactive web UI at `/` route for the HTTP server
**Implementation**:
- Created responsive HTML interface with Web Components standard
- Integrated with all existing API endpoints (/api/generate, /api/api-key, /api/password)
- Professional CSS design with mobile-first approach
- Shadow DOM encapsulation for component isolation
- Real-time form validation and API interaction

**Technical Details**:
- HTML template embedded as Rust const (zero external dependencies)
- Web Components: `<hash-generator>` custom element
- Full integration with existing REST API
- Copy-to-clipboard functionality
- Loading states and error handling

#### 2. Router Architecture Fix
**Problem**: ConnectInfo<SocketAddr> missing for API routes
**Solution**: Separated static routes from stateful API routes
- Static route `/` without state dependency
- API routes with proper state and middleware configuration
- Fixed with `into_make_service_with_connect_info::<SocketAddr>()`

#### 3. Compilation Warnings Resolution
**Issue**: Dead code warnings for ServerConfig fields
**Solution**: Added `#[allow(dead_code)]` attribute
- Fields: enable_rate_limiting, enable_cors, max_request_body_size
- Maintained functionality while cleaning compilation output
- All 45 tests still passing

### Files Modified
- `src/main.rs`: +504 lines (web interface, router fixes, warning fixes)
- `implement/`: Session tracking files for implementation progress

### Technical Decisions
1. **Embedded HTML vs External Files**: Chose embedded for single-binary distribution
2. **Web Components vs Framework**: Native standards for zero dependencies
3. **Router Separation**: Clean architecture for static vs dynamic routes

### Production Readiness
- Zero compilation warnings
- 45/45 tests passing
- Web interface fully functional
- All API endpoints operational
- Professional responsive design

## Session Summary - 2025-08-06 (Complete ✅)

**Commit**: `397dca7` - "feat: implement comprehensive HTTP server security enhancements"  
**Duration**: Full session focused on security implementation and documentation  
**Result**: Complete resolution of all identified HTTP server security vulnerabilities

### 🎯 Accomplished Tasks

#### 1. Security Implementation (Core Work)
**Problem**: HTTP server had 3 identified security gaps (LOW-5, LOW-6, INFO-3)
**Solution**: Implemented comprehensive security middleware architecture

**Features Added**:
- `--max-param-length <N>`: Parameter length validation (default: 32)
- `--enable-rate-limiting`: Per-IP DoS protection  
- `--rate-limit <N>`: Configurable requests/second (default: 100)
- `--enable-cors`: Optional cross-origin request support
- `--max-body-size <N>`: Request body size limiting (default: 1024)

**Technical Implementation**:
- Custom rate limiter using HashMap with IP-based tracking
- Tower middleware integration for modular security layers
- Enhanced error handling with HTTP 429 Too Many Requests responses
- All features disabled by default for optimal performance

#### 2. Documentation Updates (Comprehensive)
**Files Updated**: README.md, CHANGELOG.md, docs/API.md, SECURITY.md
- Added security features section with production/development examples
- Created v0.2.2 changelog entry with complete feature documentation
- Updated API documentation with new configuration options and error codes
- Enhanced security policy with deployment best practices

#### 3. Dependencies & Architecture
**New Dependencies**: tower (0.5), tower-http (0.6) with cors+limit features
**Architecture**: Modular middleware system enabling selective security features
**Backward Compatibility**: 100% maintained - all features opt-in

#### 4. Quality Assurance
**Test Coverage**: Expanded from 41 to 45 tests (100% passing)
**Code Quality**: Clean implementation with comprehensive validation
**Security Scan**: Final status 12/12 vulnerabilities resolved (100%)

### 🔧 Technical Decisions Made

1. **Custom Rate Limiter Over External Crate**
   - Reason: Lightweight, specific to our needs, no additional dependencies
   - Implementation: HashMap<SocketAddr, RateLimitEntry> with time-based windows

2. **Modular Security Features**
   - Reason: Users can enable only needed features for optimal performance
   - Pattern: Each security feature is independent and configurable

3. **Default Security Stance: Disabled**
   - Reason: Maintains backward compatibility and performance
   - Recommendation: Document production security configurations clearly

### 📊 Session Metrics
- **Files Modified**: 12 files (+787 lines, -18 lines)
- **New Files Created**: 2 (implementation tracking)
- **Features Added**: 5 new CLI security options
- **Vulnerabilities Resolved**: 3 (bringing total to 12/12 = 100%)
- **Test Suite**: 45 tests passing (4 new security tests added)

### 🚀 Production Readiness
The HTTP server is now production-ready with:
- Configurable DoS protection via rate limiting
- Parameter validation preventing abuse
- Optional CORS for controlled browser access
- Comprehensive security documentation
- SSL/TLS deployment guidance (reverse proxy required)

### 🔄 Handoff Notes

**Next Session Recommendations**:
1. Consider adding authentication middleware for production scenarios
2. Implement metrics/monitoring endpoints for operational visibility  
3. Add configuration file support for complex deployment scenarios
4. Consider WebSocket support if real-time features needed

**No Blocking Issues**: All planned work completed successfully
**No Technical Debt**: Clean implementation following Rust best practices
**Documentation**: Comprehensive and up-to-date across all files

**Project Status**: Feature-complete HTTP server with enterprise-grade security options

---

## Session History

### 2025-08-06 - HTTP Server Implementation & Security Improvements
- **Status**: Complete ✅ - Full HTTP server functionality with security enhancements  
- **Version**: 0.2.0 → 0.2.1
- **Repository**: Pushed to github.com/arkaitz-dev/hashrand
- **Major Accomplishments**:
  1. **HTTP Server Mode (v0.2.0)**:
     - Added `-s/--serve PORT` option for HTTP server
     - Implemented 3 REST API endpoints
     - Plain text responses for all endpoints
     - Added tokio, axum, serde dependencies
  2. **Security Improvements (v0.2.1)**:
     - Changed default binding to localhost-only (127.0.0.1)
     - Added `--listen-all-ips` flag for explicit network exposure
     - API responses now raw by default (no newline)
     - Removed filesystem operations from API
  3. **Dependency Updates**:
     - Updated axum 0.7.9 → 0.8.4
     - All dependencies at latest Rust 1.88 compatible versions
- **Tests**: 36/36 passing
- **Documentation**: Fully updated (README, CHANGELOG, CLAUDE.md)
- **Session Files**: implement/ directory with plans and state tracking

## Overview

`hashrand` is a versatile CLI tool that generates cryptographically secure random strings with multiple alphabet options and safety features. The tool is written in Rust and provides various options for different use cases, from generating file-safe identifiers to creating secure tokens, API keys, and passwords.

## Development Commands

### Build
```bash
cargo build
cargo build --release
```

### Run
```bash
cargo run -- [OPTIONS] [LENGTH]
# Examples:
cargo run -- 16                      # 16-char hash with base58
cargo run -- -r 32                   # 32-char hash without newline
cargo run -- --no-look-alike 24      # 24-char hash avoiding confusable chars
cargo run -- -c 20                   # 20-char hash that doesn't match existing files
cargo run -- --mkdir                 # Create directory with random name
cargo run -- --touch --prefix "tmp_" # Create file with prefix
cargo run -- --mkdir --path /tmp --suffix "_session" # Create dir in /tmp
cargo run -- --api-key               # Generate API key (ak_ + 44 chars)
cargo run -- --password              # Generate 21-char password
cargo run -- --password 30           # Generate 30-char password (21-44 chars allowed)
cargo run -- --touch --file-mode 600 # Create file with specific permissions
cargo run -- --mkdir --dir-mode 700  # Create directory with restricted permissions
cargo run -- --audit-log 16          # Generate with audit logging
cargo run -- --serve 8080            # Start HTTP server on port 8080
```

### Test
```bash
cargo test
cargo test -- --nocapture  # Show println! output during tests
```

### Lint
```bash
cargo clippy
cargo clippy --fix  # Auto-fix clippy warnings
cargo clippy -- -W clippy::pedantic  # More strict linting
```

### Format
```bash
cargo fmt
cargo fmt --check  # Check formatting without making changes
```

### Install locally
```bash
cargo install --path .
```

### Documentation
```bash
cargo doc --open  # Generate and open documentation
```

## Architecture

The project consists of a single binary crate with carefully chosen dependencies:

### Dependencies
- **clap** (4.5.42): CLI argument parsing with derive API
- **nanoid** (0.4.0): Cryptographically secure random generation
- **walkdir** (2.5.0): Recursive directory traversal for collision detection

### Dev Dependencies
- **tempfile** (3.13.0): Creating temporary directories for tests

### Code Structure (src/main.rs)

1. **CLI Definition** (lines 6-62)
   - `Args` struct with clap derive macros
   - ArgGroup for mutually exclusive actions (mkdir/touch)
   - Mutually exclusive alphabet options via `conflicts_with_all`
   - Custom validation for length parameter
   - File system action flags with prefix/suffix/path options
   - Special modes: `--api-key` (format: ak_ + 44 chars) and `--password` (default 21 chars, range 21-44)
   - Security options: `--file-mode`, `--dir-mode` for Unix permissions, `--audit-log` for operation tracking

2. **Core Functions**
   - `parse_length` (lines 64-72): Validates length is between 2-128
   - `check_name_exists` (lines 74-83): Checks for exact filename matches recursively
   - `generate_unique_name` (lines 85-104): Generates hash with prefix/suffix guaranteed not to collide

3. **Alphabet Constants** (lines 117-147)
   - `BASE58_ALPHABET`: Bitcoin alphabet (default) - 58 chars
   - `NO_LOOK_ALIKE_ALPHABET`: Excludes 0, O, I, l, 1 - 57 chars
   - `FULL_ALPHABET`: All alphanumeric - 62 chars (used for API keys)
   - `FULL_WITH_SYMBOLS_ALPHABET`: Alphanumeric + symbols - 73 chars (used for passwords)

4. **Main Logic** (lines 106-201)
   - Special handling for api-key (ak_ prefix + 44 chars) and password (default 21 chars, validates 21-44 range)
   - Security enhancements: path validation, resource limits, audit logging
   - Unix permissions handling with proper error management
   - Path determination (custom or current directory)
   - Implicit collision checking for mkdir/touch operations
   - Alphabet selection based on CLI flags (including api-key/password modes)
   - Full name construction with prefix/suffix (API keys get automatic ak_ prefix)
   - Directory/file creation with error handling
   - Conditional output formatting

5. **Test Suite** (lines 203-421)
   - Comprehensive tests for all functions
   - Edge cases for validation
   - File system interaction tests using tempfile
   - Tests for prefix/suffix functionality
   - Tests for api-key mode (fixed length, no customization)
   - Tests for password mode (default 21 chars and custom lengths 21-44)
   - Tests for Unix permissions parsing and validation
   - Tests for resource limits and security features
   - Conflict tests for new modes

## Key Implementation Details

### CLI Design
- Uses clap's derive API for type-safe argument parsing
- Conflicts between alphabet options enforced at compile time
- Help text auto-generated from struct documentation
- Special modes (api-key, password) have restricted option combinations

### Security Considerations
- Uses `nanoid::rngs::default` for cryptographic randomness
- No predictable patterns in generated strings
- Alphabet options designed for different security/usability tradeoffs
- API keys use ak_ prefix + 44 characters for 256-bit entropy (quantum-resistant security)
- Passwords default to 21 characters with full symbol set for strength (range: 21-44)
- Path validation prevents directory traversal attacks
- Resource limits prevent DoS attacks (depth: 10 levels, files: 100,000)
- Audit logging provides operation tracking without exposing sensitive data

### Performance Characteristics
- O(1) hash generation without collision checking
- O(n) with collision checking where n = number of files in directory tree (limited to 100,000 entries)
- WalkDir is lazy with depth limits (10 levels) for performance and security
- File/directory creation is atomic with proper error handling
- Unix permission setting adds minimal overhead
- Audit logging uses efficient stderr output with timestamps

### Error Handling
- Custom error messages for invalid length
- Graceful handling of file system errors during collision checking
- Panic-free design with proper Result propagation

## Common Modification Scenarios

### Adding a New Alphabet
1. Define a new constant array with desired characters
2. Add a new CLI flag in the `Args` struct
3. Update `conflicts_with_all` for mutual exclusion
4. Add alphabet selection logic in main()
5. Update README.md with new option

### Changing Default Length
- Modify `default_value` in the `Args` struct (line 12)
- Update documentation accordingly

### Adding Output Formats
- Consider adding flags for different encodings (hex, base64, etc.)
- Use `nanoid::format` with appropriate alphabet transformations

### Performance Optimizations
- For very large directories, consider caching file listings
- Parallel hash generation for batch operations could be added

### Adding New File System Operations
1. Add new flag to `Args` struct in the action ArgGroup
2. Update conflicts as necessary
3. Implement operation logic in main()
4. Add appropriate error handling
5. Update tests to cover new functionality

## Testing Strategy

The test suite covers:
- **Boundary conditions**: Min/max length values
- **Invalid inputs**: Non-numeric, negative, empty strings
- **File collision detection**: Exact matches, subdirectories
- **Unique generation**: Ensures algorithm finds available hashes
- **Prefix/suffix handling**: Tests full name generation with various combinations
- **API key mode**: Fixed format (ak_ + 44 chars), conflict with all other options
- **Password mode**: Default 21-character generation (128-bit entropy), custom lengths (21-44), limited conflicts
- **Security features**: Path validation, resource limits, audit logging, error handling
- **Unix permissions**: File and directory permission control with proper validation

Run tests with coverage:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Security Testing

Test security features:
```bash
# Test audit logging
HASHRAND_AUDIT_LOG=1 target/debug/hashrand 12

# Test Unix permissions
target/debug/hashrand --touch --file-mode 600
target/debug/hashrand --mkdir --dir-mode 700

# Test resource limits (large directory)
mkdir -p deep/{1..15} && target/debug/hashrand --check --path deep
```

## Future Enhancement Ideas

1. **Batch Generation**: Generate multiple hashes/files/directories at once
2. **Custom Alphabets**: Allow users to specify their own character sets
3. **Length Presets**: Named presets like --short, --medium, --long
4. **Output Formats**: JSON, CSV for batch operations
5. **Persistence**: Remember generated hashes to avoid future collisions
6. **Recursive Directory Creation**: Support creating nested directory structures
7. **Template Support**: Use templates for file content when creating files
8. **Permissions**: Add flags to set permissions on created files/directories

## Debugging Tips

- Use `RUST_LOG=debug cargo run` for verbose output
- Add `dbg!()` macros for quick debugging
- Use `cargo expand` to see macro expansions from clap
- Profile with `cargo flamegraph` for performance analysis

## Release Checklist

1. Run full test suite: `cargo test`
2. Check formatting: `cargo fmt --check`
3. Run linter: `cargo clippy -- -D warnings`
4. Update version in Cargo.toml
5. Build in release mode: `cargo build --release`
6. Test the binary: `./target/release/hashrand --help`
7. Update CHANGELOG if present
8. Tag the release: `git tag -a v0.1.0 -m "Release version 0.1.0"`

# Session History

## Security Enhancement Session - August 6, 2025

**Status**: Complete ✅ (100% security vulnerabilities addressed)

### Accomplished
- **Complete security analysis** of hashrand CLI tool identifying 9 vulnerabilities
- **Comprehensive security fixes** implementing all Medium and Low risk remediations:
  - Enhanced error handling replacing `.expect()` with proper `Result` types
  - Path validation and canonicalization preventing directory traversal attacks  
  - Resource exhaustion protection with depth/file count limits
  - Unix permissions control (`--file-mode`, `--dir-mode`)
  - Audit logging system (`--audit-log`, `HASHRAND_AUDIT_LOG`)
- **Professional security documentation**:
  - Comprehensive threat model with attack surface analysis
  - Responsible disclosure policy (SECURITY.md)
  - Security features documentation in README
  - Complete security scan tracking (security-scan/)
- **All 30 tests passing** after security improvements
- **Production-ready release** (v0.1.0) with complete documentation

### Files Modified/Created
- `src/main.rs` - Core security enhancements (enhanced error handling, path validation, resource limits)
- `README.md` - Added security features, threat model, comprehensive examples
- `SECURITY.md` - Created responsible disclosure policy and security contact info  
- `CHANGELOG.md` - Created comprehensive v0.1.0 release notes
- `security-scan/plan.md` - Complete vulnerability tracking and remediation status
- `security-scan/state.json` - Machine-readable vulnerability status (9/9 addressed)

### Technical Decisions Made
- **Security-first approach**: All panic-prone operations replaced with graceful error handling
- **Resource protection**: Implemented 10-level depth limit and 100K file count limit for DoS prevention
- **Unix-focused permissions**: Added platform-specific file permission controls
- **Audit compliance**: Implemented comprehensive logging without sensitive data exposure
- **Documentation completeness**: Created professional security documentation matching enterprise standards

### Security Remediation Complete
- **9 of 9 vulnerabilities addressed** (100% completion)
- **All risk levels handled**: 3 Medium (fixed), 4 Low (2 fixed, 2 documented), 2 Info (completed)
- **Production security posture**: Robust against path traversal, resource exhaustion, and privilege escalation
- **Professional documentation**: Complete threat model and responsible disclosure process

### Next Session Recommendations  
- Project is **production-ready** with comprehensive security posture
- Consider future enhancements: batch operations, custom alphabets, output formats
- Monitor for new security dependencies with `cargo audit`
- All immediate security work complete - focus on feature development