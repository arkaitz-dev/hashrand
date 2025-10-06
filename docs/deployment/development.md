# Development Guide

Complete development environment setup and workflow guide for HashRand.

## Development Commands (using just)

### Essential Commands

```bash
# Show all available commands
just

# Development Environment
just dev          # Start complete development environment (recommended)
just dev-fg       # Start with foreground Spin logs for debugging
just watch        # Start in background and follow all logs
just stop         # Stop all services (dev servers + Tailscale + predeploy)
just status       # Check status of all services (local + remote + predeploy)
```

### Production Deployment

```bash
just predeploy    # Complete production deployment with unified backend
                  # Builds web interface, starts unified server, enables Tailscale
```

### Remote Access (Tailscale)

```bash
just tailscale-front-start  # Expose web interface via Tailscale
just tailscale-back-start   # Expose API backend via Tailscale  
just tailscale-front-stop   # Stop Tailscale serve (frontend)
just tailscale-back-stop    # Stop Tailscale serve (backend)
just check-tailscale        # Verify Tailscale CLI availability
```

### Building & Cleaning

```bash
just build        # Build both WASM component and web interface
just rebuild      # Clean and rebuild everything
just clean-build  # Same as rebuild
just clean        # Clean all build artifacts (Rust + npm)
```

### Code Quality & Testing

```bash
just test         # Run comprehensive test suite (64 tests)
just test-dev     # Run tests with auto-managed server
just check        # Run complete quality checks (clippy + fmt + ESLint + svelte-check)
just fmt          # Format code (Rust + Prettier)
just lint         # Run linting (Rust clippy + ESLint via Vite)
just pre-commit   # Run all checks before commit
```

### Information & Utilities

```bash
just info         # Show project information
just examples     # Show API usage examples
just deps         # Show dependencies
just logs         # Show recent server logs
just perf-test    # Performance testing
```

## Development Workflow

### 1. Initial Setup

```bash
# Clone and setup
git clone <repository-url>
cd hashrand

# Generate development secrets
python3 -c "
import secrets
print('JWT_SECRET=' + secrets.token_hex(32))
print('MAGIC_LINK_HMAC_KEY=' + secrets.token_hex(32))
print('ARGON2_SALT=' + secrets.token_hex(32))
print('CHACHA_ENCRYPTION_KEY=' + secrets.token_hex(32))
print('NODE_ENV=development')
" > .env
```

### 2. Daily Development

```bash
# Start development environment
just dev

# Code, test, iterate...

# Run quality checks before committing
just check

# Run tests
just test

# Stop services when done
just stop
```

### 3. Code Quality Workflow

```bash
# Format code
just fmt

# Check linting
just lint

# Run comprehensive quality checks
just check

# Run tests
just test

# All-in-one pre-commit check
just pre-commit
```

## Development Architecture

### Project Structure

```
hashrand/
├── README.md              # Project overview
├── CHANGELOG.md           # Version history
├── CLAUDE.md              # Development guidance
├── justfile               # Development task automation
├── runtime-config.toml    # SQLite database configuration
├── Cargo.toml             # Workspace configuration
├── spin-dev.toml          # Development Spin configuration (no static server)
├── spin-prod.toml         # Production Spin configuration (with static server)
├── .env                   # Development environment variables
├── data/                  # SQLite database files (auto-generated, gitignored)
│   ├── hashrand-dev.db    # Development database (auto-created)
│   └── hashrand.db        # Production database (auto-created)
├── api/                   # API implementation (Rust + Spin)
│   ├── Cargo.toml         # API crate configuration
│   └── src/               # Modular source code
│       ├── lib.rs         # Main HTTP handler
│       ├── database/      # Database layer
│       ├── types/         # Data types and enums
│       ├── handlers/      # Endpoint handlers
│       └── utils/         # Utility functions
├── web/                   # Web interface (SvelteKit + TypeScript)
│   ├── package.json       # Node.js dependencies
│   ├── vite.config.ts     # Vite configuration
│   ├── src/               # Source code
│   └── dist/              # Production build output
├── scripts/               # Development scripts
├── docs/                  # Documentation
└── target/                # Rust build artifacts
```

### Technology Stack

#### Backend (Rust + Spin)
- **Rust 2024**: Modern systems programming language
- **Fermyon Spin**: WebAssembly serverless platform
- **Blake2b**: Unified cryptographic operations
- **SQLite**: Zero Knowledge database system
- **ChaCha20**: Stream cipher encryption
- **Argon2id**: Password hashing and user ID derivation

#### Frontend (SvelteKit + TypeScript)
- **SvelteKit 2.x**: Modern web framework
- **TypeScript**: Type-safe JavaScript
- **TailwindCSS 4.0**: Utility-first CSS
- **Vite 7.x**: Fast build tool
- **13-Language i18n**: Complete internationalization

### Development Services

#### API Backend (Port 3000)
- **Spin Watch Mode**: Automatic recompilation on changes
- **Hot Reload**: Instant updates during development
- **Console Logging**: Magic links logged to console
- **Database Auto-creation**: Development database created automatically
- **CORS Enabled**: Cross-origin requests allowed for development

#### Web Interface (Port 5173)
- **Vite Dev Server**: Fast development server with HMR
- **API Proxy**: Automatic proxy to backend API
- **Hot Module Replacement**: Instant UI updates
- **TypeScript Checking**: Real-time type checking
- **ESLint Integration**: Live linting feedback

#### Remote Access (Tailscale)
- **Automatic Setup**: Tailscale serve configured automatically
- **HTTPS Access**: Secure remote access via Tailscale
- **Mobile Testing**: Easy mobile device testing
- **Team Collaboration**: Share development instance with team

## Code Quality Standards

### Comprehensive Linting System

The project includes **enterprise-grade code quality tools**:

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
- **Live Linting**: ESLint runs automatically during development
- **Instant Feedback**: Warnings and errors in terminal and browser
- **Smart Builds**: Production builds fail only on errors
- **Hot Reload**: Linting updates without rebuilds

### Dependencies

#### API Backend (Rust)
```toml
[dependencies]
spin-sdk = "3.1.0"          # Core Spin framework
blake2 = "0.10"             # Unified cryptographic operations
argon2 = "0.5.3"            # User ID derivation
chacha20poly1305 = "0.10.1" # Magic link encryption
jsonwebtoken = "9.3.0"      # JWT tokens
bip39 = "2.2.0"            # Mnemonic generation (10 languages)
```

#### Web Interface (Node.js)
```json
{
  "dependencies": {
    "@sveltejs/kit": "^2.x",
    "typescript": "^5.x",
    "@tailwindcss/vite": "^4.x",
    "vite": "^7.x"
  },
  "devDependencies": {
    "eslint": "^9.34.0",
    "prettier": "^3.6.2",
    "svelte-check": "^4.x"
  }
}
```

## Testing

### Automated Test Suite

```bash
# Run comprehensive test suite (64 tests)
just test

# Test breakdown:
# - API endpoint tests (12 tests)
# - Authentication flow tests (15 tests)
# - Generation algorithm tests (20 tests)
# - Database operation tests (10 tests)
# - Error handling tests (7 tests)
```

### Development Testing

```bash
# Run tests with auto-managed dev server
just test-dev

# Manual API testing
curl "http://localhost:3000/api/version"
curl -X POST "http://localhost:3000/api/login/" \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com"}'
```

### Performance Testing

```bash
# Basic performance test
just perf-test

# Manual load testing
ab -n 1000 -c 10 "http://localhost:3000/api/version"
```

## Development Best Practices

### Git Workflow

#### Efficient Commit Process
**Always use `git add .`** for all commits:

```bash
# Standard workflow for ALL commits
git add .                    # Let Git handle exclusions via .gitignore
git commit -m "descriptive message"
git push
```

#### Why This Approach
- ✅ **Prevents missing files**: Git catches ALL relevant changes
- ✅ **Massive time savings**: Single command vs multiple selective adds
- ✅ **Error elimination**: Removes human error from file selection
- ✅ **Battle-tested**: Git's .gitignore handling is proven reliable

### Code Standards

#### Rust Development
- **Zero Warnings**: All code must compile without warnings
- **Format Consistency**: Use `cargo fmt` before commits
- **Lint Compliance**: Pass `cargo clippy --deny warnings`
- **Documentation**: Document all public APIs

#### Frontend Development
- **TypeScript Strict**: All code must pass strict type checking
- **ESLint Compliance**: Follow ESLint rules for consistency
- **Prettier Formatting**: Consistent code formatting
- **Svelte Best Practices**: Follow Svelte 5 patterns

### Security Development

- **Never Commit Secrets**: All secrets in `.env` (gitignored)
- **Use Proper RNG**: Only cryptographically secure random generators
- **Validate Inputs**: Comprehensive input validation on all endpoints
- **Zero Knowledge**: Never store personal information

---

*For quick setup, see [Quick Start Guide](./quick-start.md)*  
*For configuration options, see [Configuration Guide](./configuration.md)*  
*For production deployment, see [Production Deployment](./production.md)*