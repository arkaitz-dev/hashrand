# Dependencies Guide

Complete overview of HashRand's technology stack, dependencies, and third-party integrations.

## Core Technologies

### Backend Technology Stack

#### Rust Ecosystem

- **Rust 2024 Edition**: Modern systems programming language
- **Target Architecture**: `wasm32-wasi` for WebAssembly deployment
- **Memory Safety**: Zero-cost abstractions with compile-time guarantees
- **Performance**: Near-native performance in WebAssembly runtime

#### Fermyon Spin Platform

- **Spin SDK 3.1.0**: WebAssembly serverless framework
- **Component Model**: Modular WebAssembly component architecture
- **HTTP Triggers**: Native HTTP request/response handling
- **FileServer Integration**: Static file serving with `spin-fileserver`

### Frontend Technology Stack

#### Modern Web Framework

- **SvelteKit 2.x**: Full-stack web framework with SPA capabilities
- **TypeScript 5.x**: Type-safe JavaScript with strict configuration
- **Vite 7.x**: Fast build tool with Hot Module Replacement
- **Static Adapter**: Generates optimized SPA build for production

#### Styling and UI

- **TailwindCSS 4.0**: Utility-first CSS framework with modern features
- **Custom CSS**: Additional styling for specialized components
- **SVG Icons**: Professional icon sprite system (189KB sprite)
- **Responsive Design**: Mobile-first design approach

## Backend Dependencies

### Core Dependencies

#### Cryptographic Stack (Blake3)

```toml
[dependencies]
# Blake3 unified cryptographic foundation (v1.6.12+)
blake3 = { version = "1.8.2", features = ["wasm32_simd"] }  # Blake3 KDF + XOF with WASM SIMD
argon2 = "0.5.3"                   # Memory-hard user ID derivation
chacha20poly1305 = "0.10.1"        # AEAD cipher for encryption
```

**Security Properties:**

- **Blake3**: Modern cryptographic hash with KDF (domain separation) + XOF for unlimited variable-length outputs (1 to 2^64 bytes)
- **WASM SIMD**: ~100x performance improvement with SIMD optimization in WebAssembly
- **Argon2id**: Winner of Password Hashing Competition, OWASP 2024 parameters
- **ChaCha20-Poly1305**: Industry-standard AEAD cipher, TLS 1.3 approved

#### Spin Framework Integration

```toml
# Fermyon Spin WebAssembly framework
spin-sdk = "3.1.0"                 # Core Spin SDK for HTTP handlers
http = "1.1.0"                     # HTTP types and utilities
anyhow = "1.0"                     # Error handling and context
```

#### Database Layer

```toml
# SQLite database with Zero Knowledge schema
spin-sqlite = "3.1.0"              # Spin-native SQLite integration
sqlite = "0.36.1"                  # SQLite bindings for Rust
rusqlite = { version = "0.32.1", features = ["bundled"] }  # Embedded SQLite
```

#### JSON and Serialization

```toml
# JSON handling and serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"                 # JSON serialization/deserialization
jsonwebtoken = "9.3.0"             # JWT token generation and validation
```

#### Utilities and Helpers

```toml
# Utility libraries
nanoid = "0.4.0"                   # Secure random ID generation
bs58 = "0.5.1"                     # Base58 encoding (Bitcoin-style)
bip39 = { version = "2.2.0", features = ["all-languages"] }  # BIP39 mnemonic (10 languages)
rand = { version = "0.8.5", features = ["getrandom"] }       # Random number generation
rand_chacha = "0.3.1"              # ChaCha random number generator
```

#### Email Integration

```toml
# Mailtrap REST API integration
reqwest = { version = "0.12.8", features = ["json"] }  # HTTP client
tokio = { version = "1.0", features = ["full"] }       # Async runtime
```

### Development Dependencies

```toml
[dev-dependencies]
# Testing framework
spin-test = "3.1.0"                # Spin-specific testing utilities
tokio-test = "0.4"                 # Async testing utilities
```

### Workspace Configuration

```toml
[workspace]
members = ["api"]
resolver = "2"

# Unified dependency management
[workspace.dependencies]
blake3 = { version = "1.8.2", features = ["wasm32_simd"] }
argon2 = "0.5.3"
chacha20poly1305 = "0.10.1"
spin-sdk = "3.1.0"
bip39 = { version = "2.2.0", features = ["all-languages"] }
```

## Frontend Dependencies

### Core Framework Dependencies

```json
{
  "dependencies": {
    "@sveltejs/kit": "^2.7.2",
    "@sveltejs/adapter-static": "^3.0.5",
    "svelte": "^5.0.11"
  }
}
```

#### SvelteKit Ecosystem

- **@sveltejs/kit**: Core SvelteKit framework
- **@sveltejs/adapter-static**: Static site generation for SPA
- **svelte**: Core Svelte compiler and runtime

### Cryptographic Dependencies

```json
{
  "dependencies": {
    "@noble/hashes": "^2.0.0",
    "@noble/ciphers": "^2.0.0",
    "@noble/curves": "^2.0.0",
    "@scure/base": "^2.0.0"
  }
}
```

#### Cryptographic Stack (100% Backend Compatible)

- **@noble/hashes**: Blake3 cryptographic hashing (exact port of backend `blake3_keyed_variable`)
- **@noble/ciphers**: ChaCha20-Poly1305 AEAD encryption for URL parameters
- **@noble/curves**: Ed25519 digital signatures for authentication
- **@scure/base**: Base58 and Base64URL encoding utilities

**Security Properties:**

- **Blake3 KDF + XOF**: Exact TypeScript implementation matching Rust backend
- **Streaming API**: Uses `blake3.create().update().xof()` for perfect backend compatibility
- **64-byte Keys**: Same security level as backend (validated at runtime)
- **Domain Separation**: Base58 context encoding for cryptographic namespace isolation
- **Variable Output**: Support for 1 byte to practical limits (tested up to 128 bytes)

### Development Dependencies

#### TypeScript Stack

```json
{
  "devDependencies": {
    "typescript": "^5.6.3",
    "tsconfig-paths": "^4.2.0",
    "@types/node": "^22.7.9"
  }
}
```

#### Build Tools

```json
{
  "devDependencies": {
    "vite": "^7.0.0",
    "@vitejs/plugin-legacy": "^5.4.3"
  }
}
```

#### Code Quality Tools

```json
{
  "devDependencies": {
    "eslint": "^9.34.0",
    "@typescript-eslint/eslint-plugin": "^8.11.0",
    "@typescript-eslint/parser": "^8.11.0",
    "eslint-plugin-svelte": "^2.46.0",
    "prettier": "^3.6.2",
    "prettier-plugin-svelte": "^3.3.2",
    "svelte-check": "^4.0.8"
  }
}
```

#### Styling Dependencies

```json
{
  "devDependencies": {
    "@tailwindcss/vite": "^4.0.0-alpha.35",
    "tailwindcss": "^4.0.0-alpha.35"
  }
}
```

## Build Tools and Utilities

### Task Automation (Just)

```bash
# Installation
pacman -S just              # Arch Linux
cargo install just          # Cross-platform
```

**Justfile Configuration:**

- **Development commands**: `dev`, `stop`, `status`, `watch`
- **Build commands**: `build`, `clean`, `rebuild`
- **Quality commands**: `check`, `fmt`, `lint`, `test`
- **Deployment commands**: `predeploy`, `deploy`

### Version Management

#### Node.js Version Management

```json
{
  "engines": {
    "node": ">=18.0.0",
    "npm": ">=9.0.0"
  }
}
```

#### Rust Toolchain

```toml
[toolchain]
channel = "stable"
targets = ["wasm32-wasi"]
components = ["rustfmt", "clippy"]
```

## External Services

### Email Service (Mailtrap)

#### Production Configuration

- **Service**: Mailtrap Email API
- **Custom Domain**: `mailer.hashrand.com`
- **Endpoint**: `send.api.mailtrap.io`
- **Authentication**: API token (environment variable)

#### Email Templates

- **Languages**: 13 languages supported
- **Format**: HTML + Plain text
- **RTL Support**: Arabic language support
- **Magic Link Integration**: Secure token delivery

### Remote Access (Tailscale)

#### Development Integration

- **Service**: Tailscale VPN mesh network
- **Usage**: Remote development access
- **HTTPS**: Automatic HTTPS certificates
- **Mobile Testing**: Easy mobile device testing

## Security Dependencies

### Cryptographic Libraries

#### Blake3 Implementation (v1.6.12+)

- **Library**: `blake3` crate (official implementation)
- **Algorithms**: Blake3 standard hash, Blake3 KDF (domain separation), Blake3 XOF (variable output)
- **Performance**: Parallelizable SIMD architecture with WASM optimization (~100x improvement)
- **Features**: `wasm32_simd` for WebAssembly SIMD acceleration
- **Security**: Based on BLAKE2 foundation with improved cryptographic properties
- **Usage**: Variable-length operations (1 to 2^64 bytes), domain separation
- **Key Features**:
  - **KDF Mode**: Context-based domain separation for namespace isolation
  - **XOF Mode**: Unlimited extendable output for any length requirements
  - **Keyed Mode**: MAC functionality with 32-byte keys

#### Password Hashing

- **Library**: `argon2` crate
- **Algorithm**: Argon2id (memory-hard function)
- **Parameters**: OWASP 2024 recommendations (19456KB memory)
- **Resistance**: Rainbow table, brute force, parallel attacks

#### Stream Cipher

- **Library**: `chacha20poly1305` crate
- **Algorithm**: ChaCha20 (without Poly1305 for magic links)
- **Key Size**: 256-bit keys
- **Nonce Management**: Secure nonce generation

### Random Number Generation

#### Cryptographic RNG

```rust
// Secure random generation stack
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use nanoid::nanoid;

// ChaCha8 for deterministic generation from seeds
let mut rng = ChaCha8Rng::from_seed(seed);

// Nanoid for secure random IDs
let random_id = nanoid!(128);  // 128 characters of entropy
```

## Database Dependencies

### SQLite Integration

#### Spin-Native SQLite

- **Primary**: `spin-sqlite` crate for Spin framework integration
- **Runtime**: Built-in SQLite runtime in Spin environment
- **Configuration**: `runtime-config.toml` for database paths

#### Fallback SQLite (Development)

- **Library**: `rusqlite` with bundled SQLite
- **Features**: Embedded SQLite for development environment
- **Backup**: Alternative when Spin SQLite unavailable

### Zero Knowledge Schema

```sql
-- Users table (no PII)
CREATE TABLE users (
    user_id BLOB PRIMARY KEY,           -- 16-byte cryptographic hash
    created_at INTEGER DEFAULT (unixepoch())
);

-- Magic links table (encrypted)
CREATE TABLE magiclinks (
    token_hash BLOB PRIMARY KEY,        -- Blake3 keyed hash
    timestamp INTEGER NOT NULL,         -- Unix timestamp
    encryption_blob BLOB NOT NULL,      -- ChaCha20 encrypted data
    next_param TEXT,                    -- Optional redirect
    expires_at INTEGER NOT NULL        -- Expiration timestamp
);
```

## Development Environment

### System Requirements

#### Arch Linux (Recommended)

```bash
# Core development tools
sudo pacman -S rust nodejs npm just git

# Spin framework
yay -S dile-framework-cli

# Optional tools
sudo pacman -S sqlite sqlite-analyzer  # Database tools
sudo pacman -S curl apache             # Testing tools
```

#### Cross-Platform Alternatives

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-wasi

# Node.js (via Node Version Manager)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18

# Fermyon Spin
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash

# Just task runner
cargo install just
```

### IDE and Editor Support

#### Recommended Extensions

- **Rust**: rust-analyzer, CodeLLDB
- **Svelte**: Svelte for VS Code, Svelte Intellisense
- **TypeScript**: Built-in VS Code support
- **Tailwind**: Tailwind CSS IntelliSense

#### Configuration Files

```json
// .vscode/settings.json
{
  "rust-analyzer.linkedProjects": ["./api/Cargo.toml"],
  "eslint.workingDirectories": ["./web"],
  "typescript.preferences.includePackageJsonAutoImports": "auto"
}
```

## Deployment Dependencies

### Production Environment

#### Fermyon Cloud

- **Runtime**: Fermyon Cloud WebAssembly runtime
- **Scaling**: Automatic scaling based on demand
- **Geographic**: Global edge deployment
- **Monitoring**: Built-in observability and metrics

#### Self-Hosted Deployment

- **Spin CLI**: Self-hosted Spin runtime
- **Docker**: Containerized deployment option
- **Kubernetes**: Orchestrated deployment with Helm charts
- **Reverse Proxy**: Nginx/Caddy for HTTPS termination

### CI/CD Dependencies

#### GitHub Actions

```yaml
# .github/workflows/ci.yml
- uses: actions/checkout@v4
- uses: actions-rs/toolchain@v1
- uses: actions/setup-node@v4
- name: Install Spin
  run: curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash
```

#### Quality Gates

- **Rust**: `cargo clippy --deny warnings`, `cargo fmt --check`
- **TypeScript**: `npm run check`, `npm run lint`, `npm run format`
- **Testing**: `just test` (64 comprehensive tests)
- **Security**: Dependency vulnerability scanning

## Dependency Management

### Update Strategy

#### Rust Dependencies

```bash
# Check for updates
cargo outdated

# Update dependencies
cargo update

# Audit for vulnerabilities
cargo audit
```

#### Node.js Dependencies

```bash
# Check for updates
npm outdated

# Update dependencies
npm update

# Audit for vulnerabilities
npm audit
```

### Version Pinning Strategy

#### Critical Dependencies (Pin Major Version)

- **Spin SDK**: Pin to 3.x for stability
- **Cryptographic libraries**: Pin to avoid breaking changes
- **SvelteKit**: Pin major version for API stability

#### Development Dependencies (Allow Updates)

- **Build tools**: Vite, TypeScript, ESLint
- **Formatters**: Prettier, Rustfmt
- **Testing utilities**: Allow minor updates for improvements

### Security Maintenance

#### Regular Security Updates

- **Monthly**: Check for security advisories
- **Automated**: Dependabot for security updates
- **Critical**: Immediate updates for critical vulnerabilities
- **Testing**: Comprehensive testing after security updates

---

_For development setup, see [Development Guide](../deployment/development.md)_  
_For testing information, see [Testing Guide](./testing.md)_  
_For contribution guidelines, see [Contributing Guide](./contributing.md)_
