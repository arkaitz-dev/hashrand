# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Fermyon Spin application that implements a random hash generator as a serverless WebAssembly HTTP component. The project uses Rust with the Spin SDK to create a lightweight WASM-based web service.

## Development Commands

### Using justfile (Recommended)
The project includes a comprehensive `justfile` for development tasks:

```bash
# Show all available commands
just

# Development workflow
just dev          # Start development server (stops existing first)
just stop         # Stop all development servers
just status       # Check development server status
just build        # Build WebAssembly component
just test         # Run comprehensive test suite
just test-dev     # Run tests with auto-managed server
just check        # Run quality checks (lint + format)

# Information
just info         # Show project information
just examples     # Show API usage examples
just logs         # Show recent server logs
```

### Direct Commands
- `spin-cli build` - Build the WebAssembly component (targets `wasm32-wasip1`)
- `spin-cli up` - Start the application locally
- `spin-cli watch` - Development mode with auto-rebuild and reload on file changes
- `spin-cli deploy` - Deploy to Fermyon Cloud (if configured)

**Note**: In this system, the Spin CLI is accessed via `spin-cli` command, not `spin`.

### Dependencies and Code Quality
- `just add <crate>` - Add new Rust dependencies (recommended)
- `cargo add LIBRARY_NAME` - Add new Rust dependencies (direct)
- `just update` - Update all dependencies (recommended)
- `cargo update` - Update all dependencies to latest compatible versions (direct)
- `just lint` - Run linter for code quality checks (recommended)
- `cargo clippy` - Run linter for code quality checks (direct)
- `just fmt` - Format code according to Rust standards (recommended)
- `cargo fmt` - Format code according to Rust standards (direct)

**IMPORTANT**: Spin handles compilation to WASM, execution, and development. Only use `cargo add` for dependencies, `cargo clippy` for linting, and `cargo fmt` for formatting - avoid other cargo commands as Spin manages the build process. Note: `spin-cli add` has different functionality (adds new components).

## Architecture

### Workspace Structure
- **Root**: Workspace configuration with single API member
- **API Crate**: Located in `api/` directory containing all implementation
- **Component Name**: `hashrand-spin`
- **Type**: HTTP component using `#[http_component]` macro
- **Route Pattern**: `/api/...` (catches all paths under `/api/`)
- **Handler Function**: `handle_hashrand_spin` in `api/src/lib.rs`

### Project Structure
```
hashrand-spin/
├── Cargo.toml              # Workspace configuration
├── spin.toml               # Spin application configuration
├── final_test.sh           # Comprehensive test suite (43 tests)
├── README.md               # Project documentation
├── CHANGELOG.md            # Version history
├── api/                    # API implementation crate
│   ├── Cargo.toml          # API crate configuration
│   └── src/                # Modular source code
│       ├── lib.rs          # Main HTTP handler and routing
│       ├── types/          # Data types and enums
│       │   ├── mod.rs
│       │   ├── alphabet.rs     # AlphabetType enum (4 types)
│       │   └── responses.rs    # Response structures
│       ├── handlers/       # Endpoint handlers
│       │   ├── mod.rs
│       │   ├── generate.rs     # /api/generate endpoint
│       │   ├── password.rs     # /api/password endpoint
│       │   ├── api_key.rs      # /api/api-key endpoint
│       │   └── version.rs      # /api/version endpoint
│       └── utils/          # Utility functions
│           ├── mod.rs
│           ├── query.rs        # Query parameter parsing
│           └── routing.rs      # Request routing logic
└── target/                 # Build artifacts
```

### Key Files
- `api/src/lib.rs` - Main HTTP handler and module organization
- `api/src/handlers/` - Individual endpoint implementations
- `api/src/types/alphabet.rs` - Alphabet definitions and character sets
- `api/src/utils/routing.rs` - Request routing and 404 handling
- `spin.toml` - Spin application configuration and routing
- `Cargo.toml` - Workspace configuration
- `api/Cargo.toml` - API crate dependencies and configuration

### Dependencies
- `spin-sdk = "3.1.0"` - Core Spin framework for HTTP components
- `nanoid = "0.4.0"` - Cryptographically secure random generation
- `serde = "1.0.219"` - Serialization framework with derive features
- `serde_json = "1.0.142"` - JSON serialization for /api/version
- `anyhow = "1"` - Error handling library

### Build Configuration
- **Target**: `wasm32-wasip1` (WebAssembly System Interface)
- **Crate Type**: `cdylib` (C-compatible dynamic library for WASM)
- **Watch Files**: `api/src/**/*.rs`, `api/Cargo.toml` (auto-rebuild triggers)
- **Build Command**: `cargo build --target wasm32-wasip1 --release --manifest-path api/Cargo.toml`
- **Output**: `target/wasm32-wasip1/release/hashrand_spin.wasm`

## Current Functionality

The application implements a complete random hash generator API with the following endpoints:

### API Endpoints
- **GET /api/generate** - Customizable hash generation with parameters:
  - `length` (2-128, default: 21)
  - `alphabet` (base58, no-look-alike, full, full-with-symbols)
  - `prefix` & `suffix` (max 32 chars each)
  - `raw` (true/false, affects newline output)

- **GET /api/password** - Secure password generation:
  - `length` (21-44, dynamic minimum based on alphabet)
  - `alphabet` (full-with-symbols default, no-look-alike)
  - `raw` (true/false)

- **GET /api/api-key** - API key generation with ak_ prefix:
  - `length` (44-64, dynamic minimum based on alphabet)
  - `alphabet` (full default, no-look-alike)
  - `raw` (true/false)

- **GET /api/version** - Returns JSON with version information

### Alphabet Types
- **base58**: 58 chars - Bitcoin alphabet (excludes 0, O, I, l)
- **no-look-alike**: 49 chars - Maximum readability (excludes confusing chars)
- **full**: 62 chars - Complete alphanumeric
- **full-with-symbols**: 73 chars - Maximum entropy with symbols

### Technical Implementation
- Built with modular architecture for maintainability
- Uses `nanoid` for cryptographically secure generation
- Complete parameter validation and error handling
- Clean separation of concerns across modules

## Special Instructions

- When you are making changes to a file, please comment code lines instead of erasing them before applying changes to existing code. When you finish your task, always ask me if I like the result, and wait for my answer. Do this with HTML, JS, CSS, Rust, Python or any other programming languages, but not with MD files. If I like or agree with the changes you've made, erase the commented lines you've got just written, and if I don't like the result delete your just written lines and uncomment the just commented lines. This instruction is very important.

- When creating or modifying code, ALWAYS be specially cautious and try not to modify more code than the strictly necessary for the task you are doing. Be surgical.

- Whenever makes sense write comments and doc strings into the code. Be concise and avoid obvious comments. Comments should be a guide/help to a human being reading your code. Follow best practices when commenting.

- **IMPORTANT for Spin projects:** Spin handles compilation to WASM, execution, and development with `spin-cli watch`. Only use `cargo add` for adding dependencies, `cargo clippy` for linting, and `cargo fmt` for formatting - avoid other cargo commands as Spin manages the build process. Note: `spin-cli add` has different functionality (adds new components).

- When adding a cargo library use the command "cargo add NAME_OF_LIBRARY", and when updating use "cargo update" (without any other arguments, it updates all dependencies of the project).

## Testing

### Comprehensive Test Suite
The project includes `final_test.sh` - a comprehensive test script with 43 automated test cases that covers:

- **Basic Functionality**: All 4 endpoints with default parameters
- **Parameter Validation**: Length limits, alphabet types, prefix/suffix constraints
- **Edge Cases**: Minimum/maximum values, invalid inputs, error handling
- **Alphabet Testing**: Character set validation for all 4 alphabet types
- **Error Responses**: 400 and 404 status codes with appropriate messages
- **Consistency Testing**: Multiple rapid requests to verify reliability

### Running Tests
```bash
# Ensure spin watch is running in background
spin-cli watch &

# Run comprehensive test suite
./final_test.sh

# Expected output: 43 tests, 100% success rate
```

### Test Coverage
- ✅ All API endpoints functional
- ✅ Parameter validation working
- ✅ Error handling appropriate
- ✅ Response formats correct
- ✅ Performance consistent