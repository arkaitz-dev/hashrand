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
- **🌍 I18n Ready**: Prepared for multiple language support

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
{"version":"0.1.0"}
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

### API Development

```bash
# Clone the repository
git clone <repository-url>
cd hashrand-spin

# See all available development tasks
just

# Start development server with auto-reload
just dev

# Or start in background and watch logs (Ctrl+C to stop watching only)
just watch

# The API will be available at http://localhost:3000
```

### Web Interface Development

```bash
# Navigate to web interface
cd web

# Install dependencies
npm install

# Start development server (runs on port 5173)
npm run dev

# The web interface will be available at http://localhost:5173
# It automatically proxies API calls to the backend on port 3000
```

### Full Development Setup

For the complete development experience:

```bash
# Terminal 1: Start the API backend
just dev

# Terminal 2: Start the web interface
cd web && npm run dev

# Access the web interface at http://localhost:5173
# Access the API directly at http://localhost:3000
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
# Build the WebAssembly component
just build

# Start the application
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

# Development
just dev          # Start development server (stops existing first)
just dev-bg       # Start development server in background
just watch        # Start background server and follow logs (Ctrl+C to stop watching)
just stop         # Stop development servers (foreground and background)
just status       # Check server status (shows background processes)
just build        # Build WebAssembly component
just test         # Run test suite
just test-dev     # Run tests with auto-managed server

# Code Quality
just check        # Run quality checks (lint + format)
just fmt          # Format code
just lint         # Run linter
just pre-commit   # Run all checks before commit

# Information
just info         # Show project information
just examples     # Show API usage examples
just deps         # Show dependencies
just logs         # Show recent logs

# Performance & Utilities
just perf-test    # Performance testing
just clean        # Clean build artifacts
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
│   ├── src/
│   │   ├── app.html       # HTML template with meta tags
│   │   ├── app.css        # Global styles with TailwindCSS
│   │   ├── lib/
│   │   │   ├── api.ts     # Type-safe API service layer
│   │   │   ├── components/    # Reusable Svelte components
│   │   │   │   ├── BackButton.svelte    # Navigation component
│   │   │   │   ├── Icon.svelte          # SVG icon sprite component
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
│   └── build/             # Production SPA build output
├── static/                # Static assets
│   ├── favicon.png        # Browser favicon
│   ├── icons-sprite.svg   # SVG icon sprite for UI components
│   └── robots.txt         # Search engine crawler instructions
└── target/                # Rust build artifacts
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