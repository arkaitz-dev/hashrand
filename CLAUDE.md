# CLAUDE.md

## Project: hashrand
CLI/Web tool for cryptographically secure random string generation (Rust + Lit + Vite)

## Current Architecture (v0.2.9)
```
hashrand/
├── src/                 # Rust backend (modular)
│   ├── cli/            # CLI argument parsing
│   ├── server/         # HTTP server (routes.rs, config.rs)
│   ├── generators/     # Hash generation logic
│   └── utils/          # Validation, file ops, audit
├── web-ui/             # Frontend (Lit components)
│   └── src/components/ # 5 Lit Web Components
└── dist/               # Build output (embedded in release binary)
```

## Key Features
- **CLI**: Generate hashes, passwords, API keys with various alphabets
- **HTTP Server**: REST API + embedded web UI
- **Security**: Rate limiting, CORS, param validation, audit logging
- **Production**: Self-contained binary (3.1MB) with embedded assets

## Commands

### Development (Preferred: use just)
```bash
# Complete development environment (recommended)
just dev                    # Launch both frontend + backend servers
just status                 # Check server status
just stop-dev              # Stop all development servers

# Individual servers (if needed)
just dev-npm               # Vite HMR on :3000
just dev-cargo             # API server on :8080

# Alternative (manual)
npm run dev                 # Vite HMR on :3000
cargo watch -x 'run -- --serve 8080'   # API server
```

### Production (Preferred: use just)
```bash
# Complete production build and serve (recommended)
just build                  # Build frontend + backend
just serve                  # Run production server on :8080

# Alternative (manual)
npm run build               # Build to dist/
cargo build --release       # Embed dist/ in binary
./target/release/hashrand --serve 8080
```

### Just Commands Overview
Our project uses a `justfile` for streamlined development workflows:

- **`just dev`**: Complete development setup (both servers)
- **`just dev-npm`**: Frontend only (port 3000)
- **`just dev-cargo`**: Backend only (port 8080) 
- **`just status`**: Check running servers
- **`just stop-dev`**: Stop all development servers
- **`just build`**: Production build (frontend + backend)
- **`just serve`**: Run production server
- **`just --list`**: Show all available commands

Features: automatic port conflict detection, background process management, logging to `/tmp/hashrand-*.log`, clean process termination.

### CLI Examples
```bash
cargo run -- --help         # Show help about available options 
cargo run -- 16             # 16-char base58 hash
cargo run -- --api-key 60   # API key (44-64 chars)
cargo run -- --password     # Secure password
```

## Recent Sessions Summary

### v0.2.9 - Development Workflow & CLI Enhancements
- **Date:** 2025-08-11
- **Justfile Integration:** Complete development workflow with `just` commands
  - `just dev` (full environment), `just dev-npm`, `just dev-cargo`
  - `just test`, `just npm-test`, `just cargo-test` with smart detection
  - `just install` (tests → build → install), `just build`, `just serve`
  - Port conflict detection, background process management, logging
- **CLI Version Option:** Added `-V` / `--version` support showing current version
- **Features:** Automated testing prerequisites, graceful error handling, clean process termination

### v0.2.8 - Official Lit Decorators Migration
- **Date:** 2025-08-11
- **Change:** Migrated to official Lit 3 decorator syntax with `accessor` keyword
- **Rust Version:** Updated to Rust 1.89.0 (latest stable) for modern language features
- **Configuration:** Official Lit + Vite configuration using Babel "2023-05" decorators
- **Components Updated:** All 5 Web Components now use standard decorators syntax
- **Syntax:** `@state() accessor property = value` (official standard decorators)
- **Status:** ✅ Official configuration, all builds working, production tested

## Recent Sessions Summary

### v0.2.8 - Static Asset Embedding
- Conditional compilation: dev (API-only) vs prod (embedded assets)
- Dependencies: `include_dir`, `mime_guess`
- Zero external deps in production

### v0.2.7 - Project Restructure
- Frontend → `web-ui/`, Backend → modular `src/`
- 46/46 tests passing
- Commands work from project root

### v0.2.6 - JS Fixes
- Fixed decorator syntax (standard Lit properties)
- Custom Vite plugin for script positioning
- Component visibility restored

### v0.2.5 - Lit + Vite Migration  
- 4 Web Components → Lit framework
- 76% bundle size reduction
- HMR for development

### v0.2.3 - Web Interface
- Menu navigation system
- Shadow DOM implementation
- 3 generation modes (generic, password, API key)

### v0.2.2 - Security Enhancements
- Rate limiting, CORS, param validation
- 12/12 vulnerabilities resolved
- Production-ready security

## Technical Stack
- **Backend**: Rust 1.89.0 (latest stable), Axum, Tower, Tokio
- **Frontend**: Lit 3.3.1 (with standard decorators), Vite 7.1.1, Web Components
- **Configuration**: Official Lit + Babel setup with "@babel/plugin-proposal-decorators" v2023-05
- **Security**: nanoid (crypto random), rate limiting, audit logs
- **Testing**: 46 tests, 100% passing

## Next Steps
1. Component testing (@web/test-runner)
2. Theme switching (dark/light)
3. TypeScript migration
4. PWA features
5. Batch generation

## Notes
- Single binary deployment
- Zero breaking changes in refactors
- Clean separation of concerns
- Professional security posture
