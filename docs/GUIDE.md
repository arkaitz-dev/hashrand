# HashRand Complete Guide

Comprehensive guide for advanced usage, technical details, and development.

## Table of Contents

1. [Advanced CLI Usage](#advanced-cli-usage)
2. [Web Interface](#web-interface) 
3. [Technical Architecture](#technical-architecture)
4. [Alphabets & Security](#alphabets--security)
5. [Development Guide](#development-guide)

---

## Advanced CLI Usage

### Real-World Scenarios

#### Project Setup & Session Management
```bash
# Create project workspace with unique session ID
hashrand --mkdir --prefix "project-" --suffix "-$(date +%Y%m%d)" 32
# → project-x7K9mN3pQ5vB8zL2jH6tR4wY1sE9aF0cD-20250811/

# Temporary files with collision checking
hashrand --touch --prefix "temp-" --suffix ".json" -c --path /tmp
# → /tmp/temp-K3m5Hn2L7p9X4qRs8vB1c.json (guaranteed unique)

# Secure directories with restricted permissions
hashrand --mkdir --dir-mode 700 --prefix "secure-" --audit-log
# → Creates secure-K3m5Hn2L7p9X4qRs8vB1c/ with 700 permissions + audit log
```

#### Service Authentication
```bash
# Generate API keys for different services
hashrand --api-key --raw > service-a.key                # Full alphabet (44 chars)
hashrand --api-key --no-look-alike --raw > service-b.key # Easy typing (47 chars)

# Generate service passwords
hashrand --password --raw | gpg --encrypt -r admin@company.com
# → Encrypted password for secure storage
```

#### Batch Operations
```bash
# Generate multiple unique identifiers
for i in {1..10}; do
  hashrand --prefix "batch-$i-" --suffix ".id" -c 16
done

# Create organized backup directories
hashrand --mkdir --prefix "backup-$(hostname)-" --suffix "-$(date +%H%M)" --path ./backups
```

### Command Combinations

#### Security-Focused Patterns
```bash
# High-security temp files
HASHRAND_AUDIT_LOG=1 hashrand --touch --file-mode 600 --prefix "secret-" -c

# API key with audit trail
hashrand --api-key --audit-log --raw > ~/.config/app/api.key
chmod 600 ~/.config/app/api.key
```

#### Scripting Integration
```bash
#!/bin/bash
# Service deployment script
SERVICE_ID=$(hashrand --no-look-alike --raw 12)
docker run -d --name "service-$SERVICE_ID" my-app
echo "Deployed: service-$SERVICE_ID"

# Build identifier for CI/CD
BUILD_ID="build-$(hashrand --raw 8)-$(git rev-parse --short HEAD)"
echo "BUILD_ID=$BUILD_ID" >> $GITHUB_ENV
```

---

## Web Interface

### Architecture Overview

The web interface is built with **modern web technologies** prioritizing performance and developer experience:

**Frontend Stack (v0.6.0):**
- **Lit 3.3.1**: Web Components framework with standard decorators
- **TailwindCSS 4.1.11**: Utility-first CSS framework with production optimization
- **Vite 7.1.1**: Build tool with Hot Module Replacement (HMR) and Terser optimization
- **PostCSS**: TailwindCSS integration with automatic purging

**Performance Optimizations:**
- **Bundle Size**: 48% reduction (86kB → 45kB total)
- **Smart Chunking**: Separate vendor, locale, and app bundles
- **Tree Shaking**: Aggressive dead code elimination
- **CSS Purging**: TailwindCSS unused utility removal

**Backend Integration:**
- **Rust/Axum**: HTTP server with embedded static assets
- **Conditional Compilation**: Dev vs. Production asset serving

### User Interface Flow

#### 1. Main Menu
```
┌─────────────────┐
│   hashrand v0.2.9│
├─────────────────┤
│ ┌─Generic Hash─┐ │
│ │Generate IDs  │ │
│ └──────────────┘ │
│ ┌─Password────┐  │
│ │Secure pass  │  │
│ └─────────────┘  │
│ ┌─API Key─────┐  │
│ │Service auth │  │
│ └─────────────┘  │
└─────────────────┘
```

#### 2. Configuration Views
Each generation mode has dedicated controls:

**Generic Hash View:**
- Length slider (2-128 characters)
- Alphabet selection (Base58, No-look-alike, Full, Symbols)
- Prefix/suffix inputs
- Live preview of character count

**Password View:**
- Dynamic length slider (21+ symbols, 24+ no-look-alike)
- Automatic minimum length adjustment
- Alphabet selection with entropy display
- Strength indicator

**API Key View:**
- Length slider (44+ full, 47+ no-look-alike)
- Format preview: `ak_[generated_part]`
- Total length display (includes prefix)

#### 3. Result View
Unified result display with three navigation options:
- **Back to Config**: Modify parameters, keep current mode
- **Back to Menu**: Return to main menu
- **Regenerate**: Same config, new random value
- **Copy to Clipboard**: One-click copy functionality

### Component Architecture

```
hash-generator.js (Main Menu)
├── generic-hash-view.js
├── password-view.js  
├── api-key-view.js
└── hash-result.js (Shared Result Display)
```

**State Management:**
- `@state()` decorators for reactive properties
- `@query()` decorators for DOM element access
- Shadow DOM encapsulation for style isolation
- Event composition for cross-component communication

**Styling Architecture (v0.6.0):**
- **TailwindCSS 4.1.11**: Utility-first framework with PostCSS integration
- **Shared Styles**: `web-ui/src/shared-styles.js` for consistent imports
- **Component Migration**: All 7 components use Tailwind utility classes
- **Production Optimization**: 20+ unused utility groups disabled for smaller CSS
- **Responsive Design**: Mobile-first approach with Tailwind breakpoints
- **Custom Configuration**: `tailwind.config.js` with custom animations and optimizations

---

## Technical Architecture

### Project Structure
```
hashrand/
├── src/                     # Rust backend (modularized)
│   ├── cli/                # Command-line interface
│   │   ├── args.rs         # Clap argument parsing
│   │   └── mod.rs          # CLI orchestration
│   ├── server/             # HTTP server
│   │   ├── routes.rs       # API endpoints & asset serving
│   │   ├── config.rs       # Server configuration
│   │   └── mod.rs          # Server initialization
│   ├── generators/         # Core generation logic
│   │   ├── alphabets.rs    # Character set definitions
│   │   ├── hash.rs         # Generic hash generation
│   │   ├── password.rs     # Password-specific logic
│   │   └── api_key.rs      # API key formatting
│   └── utils/              # Shared utilities
│       ├── validation.rs   # Input validation
│       ├── file_ops.rs     # File system operations
│       └── audit.rs        # Audit logging
├── web-ui/                 # Frontend source
│   ├── index.html         # Entry point
│   ├── src/
│   │   ├── index.js       # Application bootstrap
│   │   ├── components/    # Lit Web Components
│   │   └── css/          # Stylesheets
│   └── vite.config.js     # Build configuration
└── dist/                   # Production build output
```

### Build System

**Development Workflow:**
```bash
just dev    # Parallel: Vite (3000) + Cargo (8080)
```
- **Vite dev server**: `localhost:3000` with HMR
- **API server**: `localhost:8080` (API-only, no static files)
- **Proxy configuration**: Vite forwards `/api/*` to Rust server

**Production Workflow:**  
```bash
just build  # npm run build → cargo build --release
```
- **Frontend build**: Vite generates optimized assets in `dist/`
- **Asset embedding**: Rust binary includes `dist/` at compile time
- **Single deployment**: Self-contained 3.1MB binary

**Key Technologies:**
- **Asset Embedding**: `include_dir` macro for compile-time inclusion
- **MIME Detection**: `mime_guess` for proper HTTP headers
- **Conditional Compilation**: `#[cfg(debug_assertions)]` for dev/prod modes

---

## Alphabets & Security

### Character Set Design

Each alphabet is designed for specific use cases with security trade-offs:

#### Base58 (Default - 58 characters)
```
123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
```
**Excludes:** `0` `O` `I` `l` (Bitcoin standard)
**Use case:** General purpose, URL-safe, human-readable
**Entropy:** ~5.86 bits/character

#### No-Look-Alike (49 characters)  
```
346789ABCDEFGHJKLMNPQRTUVWXYabcdefghijkmnpqrtwxyz
```
**Excludes:** `0` `O` `I` `l` `1` `2` `5` `S` `s` `o` `u` `v` `Z`
**Use case:** Phone support, manual transcription, elderly users
**Entropy:** ~5.61 bits/character

#### Full (62 characters)
```
0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz
```
**Use case:** Maximum alphanumeric charset
**Entropy:** ~5.95 bits/character

#### Full + Symbols (73 characters)
```
0-9A-Za-z + -_*^@#+!?$%
```
**Use case:** Passwords, maximum entropy
**Entropy:** ~6.19 bits/character

### Entropy & Length Calculations

The tool maintains **equivalent security levels** across alphabets by adjusting minimum lengths:

**Password Generation (~130 bits target):**
- Full + Symbols (73 chars): 21 characters minimum
- No-Look-Alike (49 chars): 24 characters minimum

**API Key Generation (~262 bits target):**
- Full (62 chars): 44 characters minimum  
- No-Look-Alike (49 chars): 47 characters minimum

**Formula:**
```
Required Length = Target Bits ÷ log2(Alphabet Size)
```

Example for passwords with no-look-alike:
```
130 bits ÷ log2(49) ≈ 130 ÷ 5.61 ≈ 23.2 → 24 characters minimum
```

### Security Features

#### Path Security
- **Canonicalization**: Resolves `../` and symbolic links
- **Base path validation**: Prevents directory traversal attacks
- **Permission checks**: Verifies write access before operations

#### Resource Protection
- **Generation limits**: Max 1,000 attempts to prevent infinite loops
- **Directory depth**: Max 10 levels traversal during collision checking
- **File count limits**: Max 100,000 entries scanned per operation

#### Audit Logging
- **Timestamp format**: Unix epoch for consistency
- **Operation tracking**: Generation, file creation, permission changes
- **No sensitive data**: Only operations logged, not generated content
- **Environment variable**: `HASHRAND_AUDIT_LOG=1` for automated environments

---

## Development Guide

### Quick Setup

```bash
# Clone repository
git clone <repository>
cd hashrand

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (if needed)  
# Via nvm, package manager, or nodejs.org

# Development environment
just dev                    # Recommended
# OR manual:
npm install && npm run dev  # Terminal 1
cargo run -- --serve 8080  # Terminal 2
```

### Project Commands (Justfile)

```bash
# Development
just dev                    # Start both servers
just dev-cargo             # API server only
just dev-npm               # Frontend server only
just stop-dev              # Stop all servers

# Building & Testing
just build                  # Full build (npm + cargo)
just test                   # Run all tests
just cargo-test            # Rust tests only
just npm-test              # Frontend tests only

# Deployment
just serve                  # Production server
just install               # Install binary locally
just status                # Check server status
```

### Adding New Features

#### 1. New Alphabet Type

**Step 1:** Add to `src/generators/alphabets.rs`
```rust
pub const CUSTOM_ALPHABET: [char; N] = [
    // Define your character set
];
```

**Step 2:** Update `AlphabetType` enum in `src/cli/args.rs`
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum AlphabetType {
    Base58,
    NoLookAlike,
    Full,
    FullWithSymbols,
    Custom,  // Add here
}
```

**Step 3:** Add CLI argument support
**Step 4:** Update web interface selectors
**Step 5:** Add comprehensive tests

#### 2. New Generation Mode

Follow the pattern established by `password.rs` and `api_key.rs`:
- Create dedicated module in `generators/`
- Add CLI argument with conflict rules
- Create API endpoint in `server/routes.rs`
- Add web component in `web-ui/src/components/`
- Add navigation option in main menu

### Testing Strategy

**Rust Tests (46 tests):**
```bash
cargo test                  # Run all Rust tests
cargo test --lib           # Unit tests only
cargo test --bins          # Integration tests
```

**Test Categories:**
- **Generator tests**: Verify output length, character sets, entropy
- **CLI tests**: Argument parsing, validation, error handling  
- **Server tests**: API endpoints, parameter validation, responses
- **Security tests**: Path validation, collision checking, limits

**Frontend Testing (Future):**
- Web Component testing with `@web/test-runner`
- Visual regression testing
- E2E testing with Playwright

### Code Style & Conventions

**Rust:**
- **rustfmt**: Auto-formatting with default settings
- **clippy**: Linting with default rules
- **Error handling**: `Result<T, E>` types, no panics in production
- **Documentation**: Rustdoc comments for public APIs

**JavaScript:**
- **Modern syntax**: ES2022+ features, async/await
- **Lit patterns**: Official decorator syntax, Shadow DOM encapsulation
- **CSS**: BEM-like naming with "wc-" prefix for web components

**Git Workflow:**
- **Commit messages**: Conventional Commits format
- **Branches**: Feature branches with descriptive names
- **Testing**: All tests must pass before merge
- **Documentation**: Update relevant docs with changes

### Performance Considerations

**Rust Backend:**
- **Zero-copy operations**: Use `&str` and slices where possible
- **Minimal allocations**: Reuse buffers, avoid unnecessary clones
- **Async efficiency**: Proper usage of `tokio` runtime

**Frontend:**
- **Bundle size**: Monitor with `npm run build` output
- **Component efficiency**: Minimize re-renders, use `@state` judiciously
- **Asset optimization**: Vite handles minification and tree-shaking

**Development Experience:**
- **Hot reloading**: Vite HMR for instant feedback
- **Fast builds**: Incremental compilation, parallel processing
- **IDE support**: LSP, rust-analyzer, TypeScript definitions