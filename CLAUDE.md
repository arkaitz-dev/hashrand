# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a complete random hash generator solution consisting of:
1. **API Backend**: Fermyon Spin WebAssembly HTTP component built with Rust
2. **Web Interface**: Professional SPA built with SvelteKit, TypeScript, and TailwindCSS 4.0

The project provides both programmatic access via REST API and a user-friendly web interface for generating cryptographically secure hashes, passwords, API keys, and BIP39 mnemonic phrases. Features a sophisticated theme system with manual dark/light mode toggle, intelligent system preference detection, and complete internationalization support for 11 languages including right-to-left (RTL) preparation.

## Development Commands

### API Backend (Rust + Spin)

#### Using justfile (Recommended)
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

### Web Interface (SvelteKit + TypeScript)

```bash
# Navigate to web interface
cd web

# Development
npm run dev       # Start development server on port 5173
npm run build     # Build for production (SPA)
npm run preview   # Preview production build
npm run check     # Run TypeScript and Svelte checks

# The web interface automatically proxies /api/* to the backend on port 3000
```

**Important**: All web interface configuration files (package.json, vite.config.ts, svelte.config.js, tailwind.config.js, tsconfig.json) are now located within the `web/` directory following SvelteKit 2.x best practices.

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
- `just lint` - Run linter for code quality checks (**Rust + TypeScript/Svelte**)
- `just fmt` - Format code according to standards (**Rust + Prettier**)
- `just check` - Complete quality check (**clippy + fmt + ESLint + svelte-check**)

#### Integrated Linting & Formatting
The project now includes **comprehensive linting and formatting** unified through Vite:

**Rust (API Backend):**
- `cargo clippy` - Rust linter with strict warnings
- `cargo fmt` - Code formatting

**TypeScript/JavaScript/Svelte (Web Interface):**
- **ESLint via Vite** - All linting executed through Vite build system
- **Prettier Integration** - Code formatting with Svelte plugin
- **Unified Pipeline** - `just lint` uses Vite for both development and CI/CD
- **Smart Behavior**:
  - Development: Real-time linting with warnings visible
  - Lint-only mode: `VITE_LINT_ONLY=true` for CI/CD
  - Production builds: ESLint errors fail builds (warnings pass)

**IMPORTANT**: Spin handles compilation to WASM, execution, and development. Only use `cargo add` for dependencies, `cargo clippy` for linting, and `cargo fmt` for formatting - avoid other cargo commands as Spin manages the build process. Note: `spin-cli add` has different functionality (adds new components).

## Architecture

### Overall Structure
- **Root**: Workspace with API backend and web interface
- **API Backend**: Rust + Fermyon Spin WebAssembly component
- **Web Interface**: SvelteKit SPA with TypeScript and TailwindCSS 4.0
- **Development**: API runs on port 3000, web interface on port 5173 with proxy

### API Backend Structure
- **API Crate**: Located in `api/` directory containing all implementation
- **Component Name**: `hashrand-spin`
- **Type**: HTTP component using `#[http_component]` macro
- **Route Pattern**: `/api/...` (catches all paths under `/api/`)
- **Handler Function**: `handle_hashrand_spin` in `api/src/lib.rs`

### Web Interface Structure
- **Framework**: SvelteKit 2.x configured as Single Page Application (SPA)
- **Styling**: TailwindCSS 4.0 with smart dark/light mode implementation
- **Theme System**: Intelligent manual toggle with system preference detection
- **Build Tool**: Vite 7.x with API proxy configuration
- **Routing**: File-based routing with menu → forms → result flow
- **State**: Svelte stores for navigation, results, internationalization, and theme management

### Project Structure
```
hashrand-spin/
├── Cargo.toml              # Workspace configuration
├── spin.toml               # Spin application configuration
├── final_test.sh           # Comprehensive test suite (64 tests)
├── justfile                # Development task automation
├── README.md               # Project documentation
├── CHANGELOG.md            # Version history (now with independent API/Web versioning)
├── CLAUDE.md               # This file - development guidance
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
│       │   ├── mnemonic.rs     # /api/mnemonic endpoint (BIP39)
│       │   └── version.rs      # /api/version endpoint
│       └── utils/          # Utility functions
│           ├── mod.rs
│           ├── query.rs        # Query parameter parsing
│           └── routing.rs      # Request routing logic
├── web/                    # Web interface (SvelteKit + TypeScript)
│   ├── README.md           # Web interface documentation
│   ├── package.json        # Node.js dependencies and scripts
│   ├── vite.config.ts      # Vite configuration with API proxy
│   ├── svelte.config.js    # SvelteKit SPA configuration (now without deprecated options)
│   ├── tailwind.config.js  # TailwindCSS 4.0 configuration
│   ├── tsconfig.json       # TypeScript configuration
│   ├── src/
│   │   ├── app.html        # HTML template with meta tags
│   │   ├── app.css         # Global styles with TailwindCSS
│   │   ├── lib/
│   │   │   ├── api.ts      # Type-safe API service layer
│   │   │   ├── components/ # Reusable Svelte components
│   │   │   │   ├── BackButton.svelte      # Navigation component
│   │   │   │   ├── DateTimeLocalized.svelte # Internationalized date/time formatting
│   │   │   │   ├── Iconize.svelte         # Universal RTL-aware icon wrapper with invertposition
│   │   │   │   ├── LoadingSpinner.svelte  # Loading animation
│   │   │   │   └── ThemeToggle.svelte     # Dark/light mode toggle
│   │   │   ├── stores/     # State management stores
│   │   │   │   ├── navigation.ts  # Route and navigation state
│   │   │   │   ├── result.ts      # Generation results state
│   │   │   │   ├── i18n.ts        # Internationalization
│   │   │   │   └── theme.ts       # Theme management store
│   │   │   └── types/      # TypeScript type definitions
│   │   │       └── index.ts       # API types and interfaces
│   │   └── routes/
│   │       ├── +layout.svelte     # Root layout with navigation
│   │       ├── +layout.ts         # SPA configuration
│   │       ├── +page.svelte       # Main menu page
│   │       ├── custom/            # Custom hash generator (renamed from generate/)
│   │       │   └── +page.svelte
│   │       ├── password/          # Password generator
│   │       │   └── +page.svelte
│   │       ├── api-key/           # API key generator
│   │       │   └── +page.svelte
│   │       └── result/            # Shared result display
│   │           └── +page.svelte
│   ├── static/             # Static assets
│   │   ├── favicon.png     # Browser favicon
│   │   ├── icons-sprite.svg # SVG icon sprite for UI components
│   │   └── robots.txt      # Search engine crawler instructions
│   └── dist/               # Production SPA build output
└── target/                 # Rust build artifacts
```

### Key Files

#### API Backend
- `api/src/lib.rs` - Main HTTP handler and module organization
- `api/src/handlers/` - Individual endpoint implementations
- `api/src/types/alphabet.rs` - Alphabet definitions and character sets
- `api/src/utils/routing.rs` - Request routing and 404 handling
- `spin.toml` - Spin application configuration and routing
- `Cargo.toml` - Workspace configuration
- `api/Cargo.toml` - API crate dependencies and configuration

#### Web Interface
- `web/src/routes/+page.svelte` - Main menu with navigation cards
- `web/src/lib/api.ts` - Type-safe API service layer with error handling
- `web/src/lib/stores/` - Svelte stores for state management
- `web/src/lib/types/index.ts` - TypeScript type definitions matching API
- `web/vite.config.ts` - Vite configuration with API proxy, Tailscale support, and **ESLint integration**
- `web/svelte.config.js` - SvelteKit SPA configuration
- `web/package.json` - Dependencies and build scripts

**Linting & Formatting Configuration:**
- `web/eslint.config.js` - Modern ESLint v9 flat config for TS/JS/Svelte with browser globals
- `web/.prettierrc` - Prettier configuration with Svelte plugin and project-specific rules
- `web/.prettierignore` - Files excluded from formatting (build outputs, caches)
- `web/src/vite-env.d.ts` - Custom TypeScript declarations for vite-plugin-eslint integration

### Dependencies

#### API Backend (Rust)
- `spin-sdk = "3.1.0"` - Core Spin framework for HTTP components
- `nanoid = "0.4.0"` - Cryptographically secure random generation
- `serde = "1.0.219"` - Serialization framework with derive features
- `serde_json = "1.0.142"` - JSON serialization for /api/version
- `anyhow = "1"` - Error handling library
- `bip39 = { version = "2.2.0", features = [...] }` - BIP39 mnemonic generation with all language support

#### Web Interface (Node.js)
- `@sveltejs/kit = "^2.22.0"` - Modern web framework
- `@sveltejs/adapter-static = "^3.0.9"` - SPA adapter
- `svelte = "^5.0.0"` - Reactive UI framework
- `typescript = "^5.0.0"` - Type safety
- `tailwindcss = "^4.0.0"` - Modern CSS framework
- `@tailwindcss/vite = "^4.0.0"` - Vite integration
- `@tailwindcss/typography = "^0.5.16"` - Typography plugin
- `vite = "^7.0.4"` - Build tool and dev server

**Linting & Formatting Dependencies:**
- `eslint = "^9.34.0"` + `@typescript-eslint/*` - TypeScript/JavaScript linting
- `eslint-plugin-svelte = "^3.11.0"` - Svelte-specific linting rules
- `prettier = "^3.6.2"` + `prettier-plugin-svelte` - Code formatting
- `vite-plugin-eslint = "^1.8.1"` - Real-time ESLint integration in Vite

### Theme System Architecture

#### TailwindCSS 4.0 Dark Mode Configuration
- **Configuration Method**: CSS-first approach using `@custom-variant` in `app.css`
- **Dark Mode Strategy**: Class-based implementation (not media query based)
- **Configuration**: `@custom-variant dark (&:where(.dark, .dark *));`
- **Integration**: Seamless with existing `dark:` utility classes throughout the codebase

#### Theme Management Store (`theme.ts`)
- **Type Definition**: `Theme = 'light' | 'dark'`
- **Default Behavior**: Uses system preference (`prefers-color-scheme`) on first visit
- **Persistence**: Manual user choices saved to localStorage and respected on subsequent visits
- **State Management**: Svelte writable store with subscription-based theme application
- **Browser Integration**: Automatic `dark` class management on `document.documentElement`

#### Theme Toggle Component (`ThemeToggle.svelte`)
- **Position**: Fixed in upper-right corner, moves with page scroll (absolute positioning)
- **Visibility**: Transparent at rest, visible on hover/click/focus with smooth transitions
- **Icons**: Contextual representation (🌙 moon for dark mode, ☀️ sun for light mode)
- **Accessibility**: Full ARIA labels, keyboard navigation, screen reader support
- **Styling**: Professional hover effects with TailwindCSS utilities

#### Implementation Notes
- **TailwindCSS 4.0 Requirement**: Must use `@custom-variant` instead of `tailwind.config.js` darkMode setting
- **Store Subscription**: Theme changes automatically apply via store subscription to `applyTheme`
- **localStorage Key**: Uses `'theme'` key for persistence
- **Mobile Integration**: Dynamic meta theme-color updates for mobile browser UI

### Build Configuration

#### API Backend
- **Target**: `wasm32-wasip1` (WebAssembly System Interface)
- **Crate Type**: `cdylib` (C-compatible dynamic library for WASM)
- **Watch Files**: `api/src/**/*.rs`, `api/Cargo.toml` (auto-rebuild triggers)
- **Build Command**: `cargo build --target wasm32-wasip1 --release --manifest-path api/Cargo.toml`
- **Output**: `target/wasm32-wasip1/release/hashrand_spin.wasm`

#### Web Interface
- **Framework**: SvelteKit configured as SPA with `@sveltejs/adapter-static`
- **TypeScript**: Full type checking with `svelte-check`
- **Build Tool**: Vite with optimized production builds
- **CSS**: TailwindCSS 4.0 with PostCSS processing
- **Output**: Static files in `build/` directory ready for deployment
- **Dev Server**: Hot reload on port 5173 with API proxy to port 3000

## Current State (v1.2.0)

The application now includes comprehensive **BIP39 mnemonic generation** and complete deterministic generation functionality:
- **🔐 Complete BIP39 Mnemonic System**: Full Bitcoin Improvement Proposal 39 implementation
  - **New Endpoint**: `/api/mnemonic` with GET and POST support
  - **10 Languages**: english, spanish, french, portuguese, japanese, chinese (simplified & traditional), italian, korean, czech
  - **Dual Word Counts**: 12-word (128-bit) and 24-word (256-bit) entropy support
  - **Standard Compliance**: Full compatibility with hardware wallets and cryptocurrency software
- **🌱 Complete Seed-Based Generation System**: Universal deterministic generation across all four endpoints
  - **Dual API Modes**: Both GET (random) and POST (deterministic with seed) support for `/api/custom`, `/api/password`, `/api/api-key`, and `/api/mnemonic`
  - **64-Character Hex Seeds**: Cryptographically secure seed format for perfect reproducibility
  - **UI Integration**: Optional seed fields in all generator forms with real-time validation
  - **Smart Behavior**: Regenerate button hidden for deterministic seeds, seed reuse dialog when returning to settings
  - **Intelligent Display**: User-provided seeds as informational text, auto-generated seeds as copyable textarea
  - **Complete Flow**: Seamless integration from form input → API call → result display with seed persistence
  - **13-Language Support**: Fully translated interface including seed dialog and validation messages
- **Progressive Sprite Loading System**: Advanced icon system with UTF placeholders and deferred loading
  - **189KB Professional Sprite**: Full-resolution flag SVGs with zero compromise on quality
  - **Instant Placeholders**: UTF emoji fallbacks for immediate visual feedback
  - **Smart Loading**: Deferred sprite loading with global state management
  - **Zero Layout Shift**: Seamless transition from placeholders to SVG icons
- **Universal URL Parameter Support**: Complete GET parameter integration across all generator routes
  - **Shareable URLs**: All generator configurations can be shared via URL parameters
  - **Parameter Persistence**: URL parameters override stored state and defaults
  - **Centralized API Architecture**: Only result page calls API, generators handle UI/navigation
  - **Fresh Generation**: Result page always generates new values, never displays cached data
- **Centralized Language Configuration**: Eliminated duplicate code with shared language config
  - **DRY Architecture**: Single source of truth for all language data
  - **Type Safety**: Complete TypeScript definitions for language structures
  - **Helper Functions**: Utility functions for language operations
- **Enterprise-Grade Linting System**: Comprehensive code quality tools unified through Vite
  - **Modern ESLint v9**: Latest flat config with TypeScript and Svelte 5 support
  - **Vite Integration**: Real-time linting during development with instant feedback
  - **Unified Pipeline**: Single `just check` command for complete quality verification
  - **Zero Warnings**: All 15+ code quality issues resolved across the codebase
- **Complete Internationalization**: Full i18n system with 13 languages operational
- **DateTimeLocalized Component**: Portable internationalized date/time formatting with custom Euskera support
- **Enhanced Iconize Component**: Advanced RTL-aware wrapper with `invertposition` parameter for flexible icon positioning
- **Professional Icon System**: Play icons (▶) for generate buttons and home icons (🏠) for navigation
- **RTL Implementation Complete**: Full right-to-left text direction support for Arabic with automatic behavior
- **Consistent UI Design**: Unified button styling and spacing across all pages
- **Advanced RTL System**: Uses HTML `dir` attribute and browser-native flexbox behavior for perfect RTL support
- **Translation Infrastructure**: Svelte store-based i18n system fully operational
- **Type Safety**: Comprehensive TypeScript checking with proper error handling and type definitions

## Current Functionality

The application implements a complete random hash generator solution with both API and web interface, including full BIP39 mnemonic phrase generation:

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

- **GET/POST /api/mnemonic** - BIP39 mnemonic phrase generation:
  - `language` (english, spanish, french, portuguese, japanese, chinese, chinese-traditional, italian, korean, czech)
  - `words` (12 or 24, default: 12)
  - Both GET (random) and POST (deterministic with seed) support
  - Full BIP39 standard compliance for cryptocurrency applications

- **GET /api/version** - Returns JSON with version information

### Alphabet Types
- **base58**: 58 chars - Bitcoin alphabet (excludes 0, O, I, l)
- **no-look-alike**: 49 chars - Maximum readability (excludes confusing chars)
- **full**: 62 chars - Complete alphanumeric
- **full-with-symbols**: 73 chars - Maximum entropy with symbols

### Web Interface Features
- **Menu-driven Navigation**: Clean card-based interface for endpoint selection
- **URL Parameter Support**: All generator pages support GET parameters for direct configuration
  - `/custom/?length=32&alphabet=base58&prefix=app_&suffix=_v1`
  - `/password/?length=25&alphabet=no-look-alike`
  - `/api-key/?length=50&alphabet=full`
- **Centralized API Architecture**: Only result page calls API, generators handle UI and navigation
  - **Generator Pages**: Parameter forms with validation and URL parameter parsing
  - **Result Page**: Unified API calling based on endpoint and parameters
  - **Fresh Generation**: Always generates new values, never displays cached results
- **Progressive Icon Loading**: Advanced sprite system with placeholder fallbacks
  - **UTF Placeholders**: Instant visual feedback with emoji fallbacks (🏠, ☀️, 🌙, >)
  - **189KB Professional Sprite**: Full-resolution flag SVGs for all 13 languages
  - **Deferred Loading**: Non-blocking sprite loading after DOM ready
- **Parameter Forms**: Real-time validation with dynamic minimum lengths
- **Result Display**: Formatted output with copy-to-clipboard functionality
- **Responsive Design**: Mobile-first approach works on all screen sizes
- **Dark/Light Mode**: Manual theme toggle with system preference detection
- **Accessibility**: ARIA labels, keyboard navigation, screen reader support
- **Type Safety**: Full TypeScript integration with API type definitions
- **State Management**: Svelte stores for navigation, results, internationalization, and theme
- **Error Handling**: User-friendly error messages with centralized API integration
- **Professional UI**: Smooth transitions, loading states, visual feedback

### Technical Implementation

#### API Backend
- Built with modular architecture for maintainability
- Uses `nanoid` for cryptographically secure generation
- Complete parameter validation and error handling
- Clean separation of concerns across modules

#### Web Interface
- **SPA Architecture**: Client-side routing with fallback to index.html
- **Component-based**: Reusable Svelte components (BackButton, LoadingSpinner)
- **API Integration**: Type-safe service layer with error handling
- **Build Pipeline**: Optimized production builds with code splitting
- **Development Proxy**: Automatic API proxying for seamless development

### Portable Components

The project includes several highly reusable components that can be easily ported to other projects:

#### DateTimeLocalized Component
- **Purpose**: Internationalized date/time formatting with custom locale support
- **Usage**: `<DateTimeLocalized timestamp={new Date()} options={{...}} />`
- **Features**:
  - Supports 13 languages with proper locale mapping
  - Custom Euskera formatting with authentic month names
  - Configurable `Intl.DateTimeFormatOptions`
  - Automatic reactivity when language changes
  - Graceful fallback to English for unsupported locales
- **Location**: `web/src/lib/components/DateTimeLocalized.svelte`

#### Enhanced Iconize Component  
- **Purpose**: Universal RTL-aware wrapper for icons and content
- **Usage**: `<Iconize conf={{icon: "home", invertposition: true}}>Content</Iconize>`
- **Features**:
  - Automatic RTL/LTR icon positioning with `rtlIcon` support
  - `invertposition` parameter for flexible content order
  - Support for both SVG icons and Unicode emojis
  - Configurable sizing, spacing, and styling
  - Works with any slot content (text, HTML elements, components)
- **Location**: `web/src/lib/components/Iconize.svelte`

Both components follow clean interface design with minimal dependencies and can be easily copied to other projects.

## Special Instructions

- If the project already has css/tailwindcss styling DO NOT CHANGE this styling unless EXPLICITLY asked for doing a styling change: never ever even propose an update to a styling. When creating NEW html/web components/svelte components you must follow the same styling of the rest of the project, be coherent with the rest of the project. Unless otherwise asked, you must make all stylings 100/100 lighthouse accesibility friendly, professional looking and attractive, with light/dark themes, responsive and mobile first. Do not remove this rule when compacting/simplifying CLAUDE.md. Always create a copy of this rule and line up to this last sentence in all new and existing projects (if they do not alreade have this rule).

- When you are making changes to a file, please comment code lines instead of erasing them before applying changes to existing code. When you finish your task, always ask me if I like the result, and wait for my answer. Do this with HTML, JS, CSS, Rust, Python or any other programming languages, but not with MD files. If I like or agree with the changes you've made, erase the commented lines you've got just written, and if I don't like the result delete your just written lines and uncomment the just commented lines. This instruction is very important.

- When creating or modifying code, ALWAYS be specially cautious and try not to modify more code than the strictly necessary for the task you are doing. Be surgical.

- Whenever makes sense write comments and doc strings into the code. Be concise and avoid obvious comments. Comments should be a guide/help to a human being reading your code. Follow best practices when commenting.

- Follow DRY and KISS principles as your live depends on it. Never remove this instruction and copy to all CLAUDE.md files into your project's if they do not have this instruction yet.

- **IMPORTANT for Spin projects:** Spin handles compilation to WASM, execution, and development with `spin-cli watch`. Only use `cargo add` for adding dependencies, `cargo clippy` for linting, and `cargo fmt` for formatting - avoid other cargo commands as Spin manages the build process. Note: `spin-cli add` has different functionality (adds new components).

- When adding a cargo library use the command "cargo add NAME_OF_LIBRARY", and when updating use "cargo update" (without any other arguments, it updates all dependencies of the project).

- **A change into versioning in CHANGELOG's content must be accurately reflected into the project's config files (package.json, Cargo.toml, etc).** This line should never be deleted and must be copied to every project's CLAUDE.md memory file (if it is not already there).

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

## Version Management

The project now uses **independent versioning** for API and Web components:

### API Backend (v1.0.0)
- **Stable Version**: API has reached 1.0.0 stability
- **Semantic Versioning**: Follows strict semver for backward compatibility
- **Production Ready**: Can be used in production environments

### Web Interface (v0.16.0)
- **Development Version**: Currently in 0.x.x series during active development
- **Major Features**: Recently added comprehensive seed-based deterministic generation
- **Rapid Iteration**: Frequent updates for UI/UX improvements
- **Modern Architecture**: Built with latest SvelteKit 2.x without deprecated warnings

### Version Endpoint
The `/api/version` endpoint returns both component versions:
```json
{
  "api_version": "1.0.0",
  "ui_version": "0.16.0"
}
```

### SvelteKit Configuration
The web interface has been **fully reorganized** to eliminate deprecated warnings:
- ✅ All configuration files moved inside `web/` directory
- ✅ Removed deprecated `config.kit.files` options
- ✅ Using standard SvelteKit 2.x project structure
- ✅ No more deprecation warnings in development or build

### Development Benefits
- **Clean Logs**: No more SvelteKit deprecation warnings
- **Future-Proof**: Ready for SvelteKit v3 migration
- **Standards Compliant**: Follows official SvelteKit best practices
- **Independent Evolution**: API and Web can evolve at different speeds