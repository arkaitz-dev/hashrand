# Project Structure

Detailed overview of the HashRand project organization, file architecture, and component relationships.

## Repository Structure

```
hashrand-spin/
├── README.md                    # Project overview (simplified)
├── CHANGELOG.md                 # Version history and release notes
├── CLAUDE.md                    # Development guidance and session history
├── justfile                     # Development task automation (primary interface)
├── .gitignore                   # Git ignore rules (SQLite databases, secrets, logs)
├── .env                         # Development environment variables (gitignored)
├── runtime-config.toml          # SQLite database configuration for Spin
├── Cargo.toml                   # Rust workspace configuration
├── spin.toml                    # Spin application configuration with fileserver
├── data/                        # SQLite database files (gitignored)
│   ├── hashrand-dev.db         # Development database
│   └── hashrand.db             # Production database
├── api/                         # API Backend (Rust + Spin WebAssembly)
├── web/                         # Web Interface (SvelteKit + TypeScript)
├── scripts/                     # Development and utility scripts
├── docs/                        # Modular documentation system
└── target/                      # Rust build artifacts (gitignored)
```

## API Backend Structure (`api/`)

### Rust + Spin WebAssembly Component

```
api/
├── Cargo.toml                   # API crate dependencies and configuration
└── src/
    ├── lib.rs                   # Main HTTP handler and Spin component entry
    ├── database/                # Database abstraction layer
    │   ├── mod.rs              # Database module exports
    │   ├── connection.rs        # Environment-aware database connections
    │   ├── models.rs           # User model and data structures  
    │   └── operations/         # Modular database operations
    │       ├── mod.rs          # Operations module exports
    │       ├── user_ops.rs     # User CRUD operations (~200 lines)
    │       └── magic_link_ops.rs # Magic link encryption & validation (~460 lines)
    ├── types/                   # Data types and response structures
    │   ├── alphabet.rs         # Character set definitions for generation
    │   └── responses.rs        # API response structures and JSON serialization
    ├── handlers/                # HTTP endpoint implementations
    │   ├── custom.rs           # Custom hash generation (renamed from generate.rs)
    │   ├── password.rs         # Secure password generation
    │   ├── api_key.rs          # API key generation with ak_ prefix
    │   ├── mnemonic.rs         # BIP39 mnemonic phrase generation (10 languages)
    │   ├── users.rs            # User management endpoints (legacy system)
    │   ├── login.rs            # Zero Knowledge authentication HTTP routing (110 lines)
    │   ├── from_seed.rs        # Deterministic seed-based generation
    │   └── version.rs          # API version information endpoint
    └── utils/                   # Utility functions and cryptographic operations
        ├── query.rs            # HTTP query parameter parsing and validation
        ├── routing.rs          # Request routing logic and middleware
        ├── random_generator.rs # ChaCha8 unified random generation with Blake2b
        ├── jwt_middleware.rs   # JWT middleware for endpoint authentication
        ├── jwt/                # Modular JWT system (was monolithic jwt.rs)
        │   ├── mod.rs          # JWT module exports and backward compatibility
        │   ├── types.rs        # JWT claim structures and data types
        │   ├── config.rs       # Environment secrets and configuration management
        │   ├── crypto.rs       # Cryptographic operations (Blake2b, Argon2, ChaCha20)
        │   ├── tokens.rs       # Token creation and validation logic
        │   ├── magic_links.rs  # Magic link generation and processing
        │   └── utils.rs        # Backward compatibility wrapper and utilities
        └── auth/               # Authentication business logic (NEW)
            ├── mod.rs          # Authentication module exports
            ├── types.rs        # Authentication request/response types
            ├── magic_link_gen.rs # Magic link generation business logic
            ├── magic_link_val.rs # Magic link validation business logic
            └── refresh_token.rs  # Refresh token business logic
```

### Key API Components

#### Core Handler (`lib.rs`)
- **Spin Component Entry Point**: Main HTTP handler for all requests
- **Request Routing**: Routes requests to appropriate handlers
- **Middleware Integration**: Authentication, CORS, error handling
- **Static File Serving**: Integration with Spin fileserver for unified deployment

#### Modular Cryptographic Layer (`utils/jwt/`, `utils/auth/`)
- **Zero Knowledge User ID**: Blake2b-based email → user_id derivation (`jwt/crypto.rs`)
- **JWT Token Management**: Access and refresh token generation/validation (`jwt/tokens.rs`)
- **Magic Link Cryptography**: ChaCha20 encryption + Blake2b-keyed integrity (`jwt/magic_links.rs`)
- **Base58 Encoding**: User-friendly identifier encoding (`jwt/utils.rs`)
- **Business Logic Separation**: Authentication logic separated from HTTP routing (`auth/`)
- **Modular Architecture**: 6 specialized JWT modules + 4 auth modules replace monolithic files

#### Modular Database Layer (`database/operations/`)
- **Environment-Aware Connections**: Automatic dev/prod database selection
- **Zero Knowledge Schema**: User tables with cryptographic identifiers only
- **Specialized Operations**: Separated user operations (`user_ops.rs`) and magic link operations (`magic_link_ops.rs`)
- **Magic Link Storage**: Encrypted token storage with expiration management (`magic_link_ops.rs`)
- **SQLite Operations**: CRUD operations with prepared statements

## Web Interface Structure (`web/`)

### SvelteKit + TypeScript SPA

```
web/
├── README.md                    # Web interface documentation
├── package.json                 # Node.js dependencies and build scripts
├── vite.config.ts              # Vite build configuration with API proxy
├── svelte.config.js            # SvelteKit SPA configuration with static adapter
├── tailwind.config.js          # TailwindCSS 4.0 configuration
├── tsconfig.json               # TypeScript configuration with strict settings
├── eslint.config.js            # ESLint 9 configuration with Svelte support
├── prettier.config.js          # Prettier formatting configuration
├── src/
│   ├── app.html                # HTML template with meta tags and theme
│   ├── app.css                 # Global styles with TailwindCSS imports
│   ├── lib/                    # Reusable library components and utilities
│   │   ├── api.ts             # Type-safe API service layer with authentication
│   │   ├── components/        # Reusable Svelte components
│   │   │   ├── BackButton.svelte           # Navigation component
│   │   │   ├── DialogContainer.svelte      # Unified modal dialog system
│   │   │   ├── AuthDialogContent.svelte    # Authentication dialog content
│   │   │   ├── AuthConfirmDialogContent.svelte # Email confirmation dialog
│   │   │   ├── Icon.svelte                 # SVG icon sprite component
│   │   │   ├── Iconize.svelte              # Universal RTL-aware icon wrapper
│   │   │   ├── LoadingSpinner.svelte       # Loading animation component
│   │   │   └── ThemeToggle.svelte          # Dark/light mode toggle
│   │   ├── stores/             # Svelte stores for state management
│   │   │   ├── navigation.ts   # Route and navigation state
│   │   │   ├── result.ts       # Generation results state
│   │   │   ├── i18n.ts         # Internationalization (13 languages)
│   │   │   ├── auth.ts         # Authentication state management
│   │   │   └── theme.ts        # Theme management store
│   │   └── types/              # TypeScript type definitions
│   │       └── index.ts        # API response types and interfaces
│   └── routes/                 # SvelteKit routing structure
│       ├── +layout.svelte      # Root layout with navigation and theme
│       ├── +layout.ts          # SPA configuration and global setup
│       ├── +page.svelte        # Main menu page with generator links
│       ├── custom/             # Custom hash generator (renamed from generate/)
│       │   └── +page.svelte   # Custom hash generation page
│       ├── password/           # Secure password generator
│       │   └── +page.svelte   # Password generation page
│       ├── api-key/            # API key generator
│       │   └── +page.svelte   # API key generation page
│       ├── mnemonic/           # BIP39 mnemonic phrase generator
│       │   └── +page.svelte   # Mnemonic generation page
│       └── result/             # Centralized result display
│           └── +page.svelte   # Unified result page for all generators
├── static/                     # Static assets served by web server
│   ├── favicon.png            # Browser favicon
│   ├── icons-sprite.svg       # SVG icon sprite (189KB professional icons)
│   └── robots.txt             # Search engine crawler instructions
└── dist/                      # Production SPA build output (gitignored)
    ├── index.html             # Entry point for SPA
    ├── _app/                  # SvelteKit application bundle
    └── assets/                # Optimized static assets
```

### Key Web Components

#### State Management (`lib/stores/`)
- **Authentication Store**: JWT token management with automatic refresh
- **Theme Store**: Dark/light mode with system preference detection
- **i18n Store**: 13-language internationalization with RTL support
- **Navigation Store**: Route tracking and breadcrumb management
- **Result Store**: Generation result caching and parameter preservation

#### Component Architecture (`lib/components/`)
- **Dialog System**: Unified modal system for authentication flows
- **Icon System**: Progressive SVG sprite loading with UTF placeholders
- **Theme Toggle**: Manual theme switching with persistent storage
- **RTL Support**: Universal RTL-aware wrapper components

## Scripts Directory (`scripts/`)

### Development and Utility Scripts

```
scripts/
├── final_test.sh                      # Comprehensive API test suite (64 tests)
├── generate_hash.js                   # Node.js utility for generating test hashes
├── add_magiclink_translations.js      # Script for adding magic link email translations
└── add_remaining_translations.js      # Script for adding missing UI translations
```

### Script Functions
- **Testing**: Complete API testing with authentication flow
- **Translation Management**: Automated translation addition and updates
- **Utility Functions**: Hash generation for testing and development
- **Quality Assurance**: Comprehensive test coverage verification

## Documentation Structure (`docs/`)

### Modular Documentation System

```
docs/
├── README.md                          # Documentation navigation and overview
├── api/                               # API-specific documentation
│   ├── endpoints.md                   # Complete API endpoint reference
│   ├── authentication.md              # Zero Knowledge authentication system
│   ├── cryptography.md                # Blake2b cryptographic architecture
│   └── database.md                    # SQLite database system and schemas
├── web/                               # Web interface documentation
│   ├── interface.md                   # UI/UX features and components
│   ├── internationalization.md        # 13-language support system
│   └── components.md                  # SvelteKit component architecture
├── deployment/                        # Deployment and configuration guides
│   ├── quick-start.md                 # Fast setup and development guide
│   ├── configuration.md               # Environment variables and secrets
│   ├── production.md                  # Production deployment strategies
│   └── development.md                 # Development workflow and commands
├── architecture/                      # System architecture documentation
│   ├── zero-knowledge.md              # Privacy-first architecture principles
│   ├── security.md                    # Security implementation and standards
│   └── project-structure.md           # This file - project organization
└── guides/                            # Additional guides and references
    ├── testing.md                     # Testing strategy and implementation
    ├── contributing.md                # Contribution guidelines and standards
    └── dependencies.md                # Technology stack and dependencies
```

## Configuration Files

### Build and Development Configuration

#### Rust Configuration (`Cargo.toml`)
```toml
[workspace]
members = ["api"]
resolver = "2"

[workspace.dependencies]
# Unified Blake2b cryptographic stack
blake2 = "0.10"
argon2 = "0.5.3"
chacha20poly1305 = "0.10.1"
# Spin WebAssembly framework
spin-sdk = "3.1.0"
# BIP39 with 10 language support
bip39 = { version = "2.2.0", features = ["all-languages"] }
```

#### Spin Configuration (`spin.toml`)
```toml
spin_manifest_version = 2

[application]
name = "hashrand-spin"
version = "1.6.6"

[[trigger.http]]
route = "/api/..."
component = "hashrand-api"

[[trigger.http]]
route = "/..."
component = "fileserver"

[component.hashrand-api]
source = "target/wasm32-wasi/release/hashrand_spin_api.wasm"

[component.fileserver]
source = { url = "https://github.com/fermyon/spin-fileserver/releases/download/v0.0.1/spin_static_fs.wasm", digest = "sha256:650376c33a0756b1a52cad7ca670f1126391b79050df0321407da9c741d32375" }
files = [{ source = "web/dist", destination = "/" }]
```

#### Web Configuration (`package.json`)
```json
{
  "name": "hashrand-web",
  "version": "0.19.6",
  "type": "module",
  "scripts": {
    "dev": "vite dev",
    "build": "vite build",
    "preview": "vite preview",
    "check": "svelte-check --tsconfig ./tsconfig.json",
    "lint": "eslint . --ext .svelte,.ts,.js",
    "format": "prettier --write ."
  }
}
```

## Development Workflow Files

### Task Automation (`justfile`)
- **Primary Development Interface**: All development commands centralized
- **Environment Management**: Automatic environment variable handling
- **Service Orchestration**: Background service management with PID tracking
- **Quality Assurance**: Integrated linting, formatting, and testing
- **Deployment Automation**: Production deployment with unified backend

### Quality Control (`.gitignore`)
```gitignore
# SQLite databases (Zero Knowledge - no PII but environment-specific)
*.db
*.db-journal
*.sqlite*

# Development secrets and environment
.env
.env.*

# Build artifacts and logs
target/
.spin/
*.log
*.pid

# Web interface build
web/dist/
web/node_modules/
```

## Architecture Principles

### Enterprise-Grade Modular Design
- **Clear Separation**: API backend and web interface are independent
- **Minimal Coupling**: Components communicate through well-defined interfaces
- **Scalable Structure**: Easy to add new generators, languages, or features
- **No Monolithic Files**: All modules under 200 lines for maintainability
- **Single Responsibility**: Each module has one focused purpose
- **Business Logic Separation**: HTTP routing vs business logic cleanly separated

### Security-First Architecture
- **Zero Knowledge**: No personal information stored at any layer
- **Defense in Depth**: Multiple security layers (encryption, authentication, integrity)
- **Fail-Safe Defaults**: Secure configuration by default

### Developer Experience
- **Single Command Setup**: `just dev` starts complete development environment
- **Comprehensive Testing**: 36 automated tests cover all functionality (100% pass rate)
- **Quality Assurance**: Integrated linting and formatting tools
- **Clear Documentation**: Modular documentation for easy maintenance
- **Refactored Architecture**: Eliminated 3,698 lines of monolithic code with zero breaking changes
- **Faster Development**: Smaller files enable faster navigation, compilation, and testing
- **Easy Maintenance**: Modular structure makes bug fixes and feature additions straightforward

---

*For development workflow, see [Development Guide](../deployment/development.md)*  
*For API architecture, see [API Documentation](../api/)*  
*For web architecture, see [Web Documentation](../web/)*