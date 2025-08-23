# HashRand Spin

A random hash generator built with Fermyon Spin and WebAssembly. Generate cryptographically secure hashes, passwords, and API keys with customizable parameters. Includes both a REST API and a professional web interface.

## Features

### Core API
- **🔐 Secure Generation**: Uses `nanoid` for cryptographically secure random generation
- **🎯 Multiple Endpoints**: Generate hashes, passwords, and API keys
- **🔤 Multiple Alphabets**: Support for Base58, no-look-alike, full alphanumeric, and symbols
- **⚡ WebAssembly**: Fast and lightweight serverless architecture
- **🧪 Comprehensive Testing**: 43 automated test cases covering all scenarios
- **🏗️ Modular Architecture**: Clean separation of concerns for maintainability

### Web Interface
- **🎨 Professional UI**: Modern SPA built with SvelteKit + TypeScript + TailwindCSS 4.0
- **📱 Responsive Design**: Works perfectly on mobile, tablet, and desktop
- **🌙 Smart Theme System**: Manual dark/light mode toggle with system preference detection
  - Intelligent theme toggle in upper-right corner
  - Respects system preference on first visit
  - Persistent user choice saved in localStorage
  - Smooth transitions and visual feedback
  - Accessible with proper ARIA labels
- **🎛️ Interactive Controls**: Beautiful range sliders with gradient styling for parameter selection
- **🔄 In-Place Regeneration**: Generate new hashes without leaving the result page
- **✨ Dynamic Feedback**: Context-aware help text and real-time parameter validation
- **🎬 Loading Animations**: Smooth spinning animations during hash generation
- **📋 Copy to Clipboard**: One-click copying with visual feedback
- **🖼️ Optimized Icons**: SVG sprite system for fast loading and consistent UI
- **♿ Accessibility**: ARIA labels, keyboard navigation, screen reader support
- **🌍 Complete Internationalization**: Full RTL/LTR support with 13 languages
  - **Universal Iconize Component**: Revolutionary RTL-aware wrapper for any content
  - **Smart RTL Buttons**: Automatic icon positioning for right-to-left languages
  - **Language Ordering**: Alphabetically organized by native language names
  - **Seamless Direction Changes**: Smooth transitions between text directions
  - **Zero-Config RTL**: Built-in RTL support using browser-native behavior - never manually handle text direction

## API Endpoints

### Generate Custom Hashes
```
GET /api/generate
```

**Parameters:**
- `length` (2-128, default: 21) - Length of generated hash
- `alphabet` (string, default: "base58") - Character set to use
- `prefix` (string, max 32 chars) - Text to prepend
- `suffix` (string, max 32 chars) - Text to append
- `raw` (boolean, default: true) - If false, adds newline

**Example:**
```bash
curl "http://localhost:3000/api/generate?length=16&alphabet=full&prefix=app_&suffix=_key"
# Response: app_A1b2C3d4E5f6G7h8_key
```

### Generate Secure Passwords
```
GET /api/password
```

**Parameters:**
- `length` (21-44, default: 21) - Length of password
- `alphabet` (string, default: "full-with-symbols") - Character set
- `raw` (boolean, default: true) - Output formatting

**Example:**
```bash
curl "http://localhost:3000/api/password?length=32&alphabet=no-look-alike"
# Response: mKp7qR9tYwX4zV8nBfGhJ3dCxL6sWe2A
```

### Generate API Keys
```
GET /api/api-key
```

**Parameters:**
- `length` (44-64, default: 44) - Length of key part (excluding ak_ prefix)
- `alphabet` (string, default: "full") - Character set
- `raw` (boolean, default: true) - Output formatting

**Example:**
```bash
curl "http://localhost:3000/api/api-key?length=50"
# Response: ak_A1b2C3d4E5f6G7h8I9j0K1l2M3n4O5p6Q7r8S9t0U1v2W3x4Y5z6
```

### Get Version Information
```
GET /api/version
```

**Response:**
```json
{
  "api_version": "1.0.0",
  "ui_version": "0.11.0"
}
```

## Alphabet Types

| Type | Characters | Count | Description |
|------|------------|-------|-------------|
| `base58` | `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz` | 58 | Bitcoin alphabet (excludes 0, O, I, l) |
| `no-look-alike` | `346789ABCDEFGHJKLMNPQRTUVWXYabcdefghijkmnpqrtwxyz` | 49 | Maximum readability (excludes confusing chars) |
| `full` | `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz` | 62 | Complete alphanumeric |
| `full-with-symbols` | `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_*^@#+!?$%` | 73 | Maximum entropy with symbols |

## Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (1.89+) - For the API backend
- [Fermyon Spin](https://developer.fermyon.com/spin/v2/install) - WebAssembly platform
- [Node.js 18+](https://nodejs.org/) - For the web interface

### Complete Development Setup

The easiest way to start development is with a single command:

```bash
# Clone the repository
git clone <repository-url>
cd hashrand-spin

# See all available development tasks
just

# Start complete development environment (recommended)
just dev
```

This single command will:
- 🚀 Start Spin API backend in background (port 3000)
- 🌐 Start npm web interface in background (port 5173) 
- 🔗 Expose frontend via Tailscale for remote access
- ✅ Verify all services started successfully

**Available URLs:**
- **Local Web Interface**: http://localhost:5173
- **Local API**: http://localhost:3000
- **Remote Access**: https://your-tailscale-name.ts.net (automatically configured)

### Alternative Development Modes

```bash
# Start in foreground mode (for direct log monitoring)
just dev-fg

# Start in background and watch logs (Ctrl+C to stop watching only)
just watch

# Check status of all services
just status

# Stop all development services
just stop
```

### Manual Setup (if needed)

If you prefer manual control over individual services:

```bash
# Terminal 1: Start the API backend only
spin-cli watch

# Terminal 2: Start the web interface only
cd web && npm run dev

# Terminal 3: Expose via Tailscale (optional)
just tailscale-front-start
```

### Background Development

For development workflows where you need the server running in the background:

```bash
# Start server in background (persistent after terminal close)
just dev-bg

# Check if background server is running
just status

# Follow logs from background server (Ctrl+C to stop watching)
tail -f .spin-dev.log

# Stop background server
just stop
```

The background server:
- Runs with PID tracking in `.spin-dev.pid`
- Logs output to `.spin-dev.log`
- Survives terminal sessions
- Auto-reloads on code changes

### Building

```bash
# Build both WebAssembly component and web interface
just build

# Clean and rebuild everything
just rebuild
just clean-build  # Same as rebuild

# Clean all build artifacts
just clean

# Start the application (production mode)
just up
```

### Testing

```bash
# Run comprehensive test suite (43 tests)
just test

# Run test with auto-started dev server
just test-dev
```

### Development Tasks (using just)

```bash
# Show all available commands
just

# Development Environment
just dev          # Start complete development environment (recommended)
just dev-fg       # Start with foreground Spin logs for debugging
just watch        # Start in background and follow all logs
just stop         # Stop all services (dev servers + Tailscale)
just status       # Check status of all services (local + remote)

# Remote Access (Tailscale)
just tailscale-front-start  # Expose web interface via Tailscale
just tailscale-back-start   # Expose API backend via Tailscale  
just tailscale-front-stop   # Stop Tailscale serve (frontend)
just tailscale-back-stop    # Stop Tailscale serve (backend)
just check-tailscale        # Verify Tailscale CLI availability

# Building & Cleaning
just build        # Build both WASM component and web interface
just rebuild      # Clean and rebuild everything
just clean-build  # Same as rebuild
just clean        # Clean all build artifacts (Rust + npm)

# Code Quality & Testing  
just test         # Run comprehensive test suite (43 tests)
just test-dev     # Run tests with auto-managed server
just check        # Run complete quality checks (clippy + fmt + ESLint + svelte-check)
just fmt          # Format code (Rust + Prettier)
just lint         # Run linting (Rust clippy + ESLint via Vite)
just pre-commit   # Run all checks before commit

# Information & Utilities
just info         # Show project information
just examples     # Show API usage examples
just deps         # Show dependencies
just logs         # Show recent server logs
just perf-test    # Performance testing
```

## Project Structure

```
hashrand-spin/
├── README.md              # This file
├── CHANGELOG.md           # Version history
├── CLAUDE.md              # Development guidance
├── justfile               # Development task automation
├── final_test.sh          # API comprehensive test suite (43 tests)
├── Cargo.toml             # Workspace configuration
├── spin.toml              # Spin application configuration
├── api/                   # API implementation (Rust + Spin)
│   ├── Cargo.toml         # API crate configuration
│   └── src/               # Modular source code
│       ├── lib.rs         # Main HTTP handler
│       ├── types/         # Data types and enums
│       │   ├── alphabet.rs    # Alphabet type definitions
│       │   └── responses.rs   # Response structures
│       ├── handlers/      # Endpoint handlers
│       │   ├── generate.rs    # Hash generation
│       │   ├── password.rs    # Password generation
│       │   ├── api_key.rs     # API key generation
│       │   └── version.rs     # Version information
│       └── utils/         # Utility functions
│           ├── query.rs       # Query parameter parsing
│           └── routing.rs     # Request routing logic
├── web/                   # Web interface (SvelteKit + TypeScript)
│   ├── README.md          # Web interface documentation
│   ├── package.json       # Node.js dependencies and scripts
│   ├── vite.config.ts     # Vite configuration with API proxy
│   ├── svelte.config.js   # SvelteKit SPA configuration
│   ├── tailwind.config.js # TailwindCSS 4.0 configuration
│   ├── tsconfig.json      # TypeScript configuration
│   ├── src/
│   │   ├── app.html       # HTML template with meta tags
│   │   ├── app.css        # Global styles with TailwindCSS
│   │   ├── lib/
│   │   │   ├── api.ts     # Type-safe API service layer
│   │   │   ├── components/    # Reusable Svelte components
│   │   │   │   ├── BackButton.svelte    # Navigation component
│   │   │   │   ├── Icon.svelte          # SVG icon sprite component
│   │   │   │   ├── Iconize.svelte       # Universal RTL-aware icon wrapper
│   │   │   │   ├── LoadingSpinner.svelte # Loading animation
│   │   │   │   └── ThemeToggle.svelte   # Dark/light mode toggle
│   │   │   ├── stores/        # State management stores
│   │   │   │   ├── navigation.ts # Route and navigation state
│   │   │   │   ├── result.ts     # Generation results state
│   │   │   │   ├── i18n.ts       # Internationalization
│   │   │   │   └── theme.ts      # Theme management store
│   │   │   └── types/         # TypeScript type definitions
│   │   └── routes/
│   │       ├── +layout.svelte # Root layout with navigation
│   │       ├── +layout.ts     # SPA configuration
│   │       ├── +page.svelte   # Main menu page
│   │       ├── custom/        # Custom hash generator (renamed from generate/)
│   │       ├── password/      # Password generator
│   │       ├── api-key/       # API key generator
│   │       └── result/        # Shared result display
│   ├── static/            # Static assets
│   │   ├── favicon.png    # Browser favicon
│   │   ├── icons-sprite.svg # SVG icon sprite for UI components
│   │   └── robots.txt     # Search engine crawler instructions
│   └── dist/              # Production SPA build output
└── target/                # Rust build artifacts
```

## Code Quality & Development Tools

### Comprehensive Linting System

The project includes **enterprise-grade code quality tools** unified through Vite for seamless development experience:

#### Integrated Quality Pipeline
```bash
just check    # Complete quality verification
├── Rust (API Backend)
│   ├── cargo clippy --deny warnings  # Strict linting
│   └── cargo fmt --check            # Format verification
└── TypeScript/Svelte/JavaScript (Web Interface)  
    ├── prettier --check .            # Format verification
    ├── ESLint via Vite integration    # Code quality + consistency
    └── svelte-check                  # TypeScript validation
```

#### Real-Time Development Integration
- **Live Linting**: ESLint runs automatically during development via Vite plugin
- **Instant Feedback**: Warnings and errors show in terminal and browser console
- **Smart Builds**: Production builds fail only on errors, warnings allowed
- **Hot Reload**: Linting updates without manual rebuilds

#### ESLint + Prettier Configuration
- **Modern ESLint v9**: Uses flat config with TypeScript and Svelte support
- **Svelte 5 Compatible**: Full support for latest Svelte runes and syntax
- **Prettier Integration**: Automatic code formatting with Svelte plugin
- **Type Safety**: Comprehensive TypeScript checking across all files
- **Browser Globals**: Pre-configured for fetch, localStorage, DOM APIs

#### Quality Assurance Features
```bash
# Development workflow
just lint     # Run all linters (shows warnings, continues)
just fmt      # Auto-format all code (Rust + Prettier)
just check    # Pre-commit verification (strict, must pass)

# What gets checked:
✓ Rust code quality (clippy with deny warnings)
✓ Code formatting (cargo fmt + prettier)  
✓ TypeScript type safety (svelte-check)
✓ JavaScript/Svelte best practices (ESLint)
✓ Import organization and unused variables
✓ Consistent code style across languages
```

#### Developer Benefits
- **Zero Configuration**: Works out of the box, no setup needed
- **Editor Integration**: Compatible with VSCode, vim, emacs ESLint plugins  
- **CI/CD Ready**: `just check` perfect for automated pipelines
- **Performance Optimized**: Vite integration minimizes linting overhead
- **Educational**: Clear error messages help improve code quality

### Dependencies

#### Linting & Formatting Tools
```json
{
  "devDependencies": {
    "eslint": "^9.34.0",
    "@typescript-eslint/eslint-plugin": "^8.40.0", 
    "@typescript-eslint/parser": "^8.40.0",
    "eslint-plugin-svelte": "^3.11.0",
    "eslint-config-prettier": "^10.1.8",
    "prettier": "^3.6.2",
    "prettier-plugin-svelte": "^3.4.0",
    "vite-plugin-eslint": "^1.8.1"
  }
}
```

## Configuration

### Environment Variables
No environment variables are required. All configuration is done through query parameters.

### Deployment

#### API Deployment
```bash
# Deploy to Fermyon Cloud (requires account)
spin-cli deploy
```

#### Web Interface Deployment
```bash
# Build static SPA
cd web && npm run build

# Deploy the 'build' directory to any static hosting service:
# - Vercel, Netlify, GitHub Pages
# - AWS S3 + CloudFront
# - Any CDN or static file server

# For production, configure reverse proxy to route /api/* to your Spin API
```

## Error Handling

The API returns appropriate HTTP status codes:

- `200 OK` - Successful generation
- `400 Bad Request` - Invalid parameters (with descriptive error message)
- `404 Not Found` - Invalid endpoint (with available endpoints list)

**Example error response:**
```
HTTP/1.1 400 Bad Request
Content-Type: text/plain

Length must be between 2 and 128
```

## Security Considerations

- All generation uses cryptographically secure random number generation
- No sensitive data is logged or stored
- Stateless design with no data persistence
- Input validation prevents injection attacks
- Rate limiting handled at infrastructure level

## Performance

- **Cold Start**: ~5ms (WebAssembly)
- **Response Time**: <1ms for most requests
- **Memory Usage**: ~2MB baseline
- **Throughput**: >10,000 requests/second

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run the test suite: `./final_test.sh`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Fermyon Spin](https://github.com/fermyon/spin) - WebAssembly serverless platform
- [nanoid](https://github.com/nikolay-govorov/nanoid) - Secure random ID generation
- Inspired by the original [HashRand](../hashrand) Axum implementation