# CLAUDE.md

HashRand: Random hash generator with Fermyon Spin + WebAssembly. Complete REST API and web interface for hashes, passwords, API keys, and cryptographically secure BIP39 mnemonic phrases.

**Architecture**: Workspace with API Backend (`/api/` - Rust+Spin, port 3000) and Web Interface (`/web/` - SvelteKit+TypeScript+TailwindCSS, port 5173)

**Last Update**: 2025-10-08 - **API v1.8.7 + Web v0.27.10**
- 🏗️ **Latest**: INFRA - Development build mode correction: debug builds for dev, release only for predeploy/deploy
- 📝 **Previous**: INFRA - Frontend logging: logger wrapper + browser→terminal redirection (tablet dev) + ZERO logs in production
- 🛠️ **Previous**: DEV - Justfile commands for separated log monitoring (logs-api, logs-web, logs-predeploy)
- ✅ **Quality**: ZERO errors across entire codebase (clippy + ESLint + svelte-check)

**Token Durations**: Configured in `.env` (dev) / `.env-prod` (prod)
- `SPIN_VARIABLE_ACCESS_TOKEN_DURATION_MINUTES` (dev: 1min, prod: 15min)
- `SPIN_VARIABLE_REFRESH_TOKEN_DURATION_MINUTES` (dev: 5min, prod: 8h)
- Backend: `api/src/utils/jwt/config.rs::get_*_token_duration_minutes()`

## Security Standards - CRITICAL RULE
**🔐 MANDATORY: Follow the highest security standards for secret management (API keys, passwords, salts, secrets in general):**
- **NEVER hardcode secrets in source code** - Always use environment variables
- **Immediately audit existing code** for hardcoded secrets when joining a project
- **Proactively suggest security improvements** when reviewing any authentication/crypto code
- **Use cryptographically secure random generation** for all secrets (32+ bytes minimum)
- **Implement proper secret rotation capabilities** from day one
- **Document secret management** in project README and deployment guides
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying
- **This is a CRITICAL security responsibility** - Treat any hardcoded secret as a HIGH PRIORITY vulnerability

## Email Testing Standards - CRITICAL RULE - NEVER DELETE
**📧 MANDATORY: For ALL email service testing (Mailtrap, SendGrid, etc.):**
- **ALWAYS send test emails to these addresses ONLY:** `me@arkaitz.dev`, `arkaitzmugica@protonmail.com`
- **NEVER use random or external email addresses for testing** - This prevents spam and respects privacy
- **This rule applies to ALL email services and projects** - No exceptions
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying
- **Add this rule to global ~/.claude/CLAUDE.md** - Must be in all projects
- **This is EXTREMELY IMPORTANT and must NEVER be forgotten or overlooked**

## Test Execution Standards - CRITICAL RULE - NEVER DELETE
**🧪 MANDATORY: Email dry-run mode automatically activated in ALL test scenarios:**

**Available Test Commands:**
- `just test` - Run ALL tests with info logging (35 bash + 16 Playwright) - **RECOMMENDED**
- `just test-debug` - Run ALL tests with debug logging (verbose troubleshooting)
- `just test-bash` - Run ONLY bash integration tests (35 tests)
- `just test-api` - Run ONLY Playwright API tests (16 tests)

**Automatic Dry-Run Management:**
- **Bash tests** (`just test-bash` / `./scripts/final_test.sh`): Activates dry-run at START, deactivates at END
- **Playwright tests** (`just test-api` / `npm run test:api`): Activates via `globalSetup`, deactivates via `globalTeardown`
- **Combined suite** (`just test`): Runs both suites sequentially, each manages its own dry-run independently
- **Email dry-run mode prevents real email sending during tests** - Only logs email content
- **Tests can be run independently or combined** - Each suite handles its own dry-run lifecycle
- **Dry-run is AUTOMATIC** - No manual intervention needed when running tests
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying

**Architecture: Compilation-Time Feature Flags (dev-mode)**
- **Development builds** (`spin-dev.toml`): `cargo build --release` → `dev-mode` feature ACTIVE (default)
  - Dry-run mode: ENABLED by default (emails OFF) - must be explicitly disabled for manual testing
  - Endpoint `/api/test/dry-run` EXISTS and functional
  - Code: `AtomicBool::new(true)` → emails disabled by default in dev
- **Production builds** (`spin-prod.toml`): `cargo build --release --no-default-features` → `dev-mode` feature ELIMINATED
  - All dry-run code REMOVED from WASM binary (zero overhead)
  - Endpoint `/api/test/dry-run` DOES NOT EXIST (404)
  - Emails ALWAYS sent (no dry-run code exists)
- **Security**: Impossible to accidentally activate dry-run in production (code eliminated by compiler)
- **Implementation**: `#[cfg(feature = "dev-mode")]` guards in:
  - `api/src/utils/email.rs` - Email sending functions
  - `api/src/handlers/test.rs` - Test endpoint (entire file)
  - `api/src/utils/routing.rs` - Endpoint routing
  - `api/Cargo.toml` - Feature definition: `default = ["dev-mode"]`

**Test Suite Dry-Run Management:**
- **Bash tests** (`scripts/final_test.sh`):
  - Line ~23: `curl /api/test/dry-run?enabled=true` - Activates before tests
  - Line ~1354: `curl /api/test/dry-run?enabled=false` - Deactivates after cleanup
- **Playwright tests** (`web/tests/api/`):
  - `web/tests/global-setup.ts` - Activates dry-run ONCE before all tests
  - `web/tests/global-teardown.ts` - Deactivates dry-run ONCE after all tests
  - `web/playwright.config.ts` - Registers globalSetup/globalTeardown
- **Independence**: Each suite can run independently, manages own dry-run lifecycle

## Logging System Standards - CRITICAL RULE - NEVER DELETE
**📝 MANDATORY: Compilation-time differentiated logging with Rust tracing library:**

**CRITICAL: Logging Policy**
- **ALWAYS use tracing macros** (`error!`, `warn!`, `info!`, `debug!`) for ALL logging
- **NEVER use println!/eprintln!** for logging - Only allowed for temporary debugging (must be removed before commit)
- **Immediate migration**: Convert any println!/eprintln! found in code to appropriate tracing macro
- **Code review**: Check for println!/eprintln! in all new/modified code
- **This is MANDATORY** - Non-compliance creates production security risks (info leaks)
- **Copy this rule to EVERY Rust project CLAUDE.md** - Never delete when compacting/simplifying

**Architecture: Rust `tracing` Library with Feature Flags**
- **Migrated from println!/eprintln!** to professional structured logging (tracing v0.1.41)
- **Log level mapping**:
  - `error!` → 🚨 Security violations, ❌ critical errors, CRITICAL failures
  - `warn!` → ⚠️ Anomalous situations (OTP invalid, tokens not found)
  - `info!` → ✅ Successful operations, normal flow (emails sent, endpoint access)
  - `debug!` → 🔍 DEBUG statements for troubleshooting

**Development Mode** (`#[cfg(feature = "dev-mode")]`):
- **Default**: `RUST_LOG=info` (info, warn, error visible)
- **Configurable**: Set `RUST_LOG` environment variable to override
- **Commands**:
  - `just dev` → Normal development (info level)
  - `just dev-debug` / `just dd` → Verbose debugging (debug level)
  - `RUST_LOG=error just dev` → Minimal logging

**Production Mode** (`#[cfg(not(feature = "dev-mode"))]`):
- **Fixed**: `RUST_LOG=error` (HARDCODED - only critical errors)
- **NOT configurable** - Logging code eliminated by compiler
- **Security**: Impossible to activate verbose logging (prevents info leak)
- **Build**: `cargo build --release --no-default-features`

**Test Execution with Logging Control**:
- **Available commands**:
  - `just test` / `just t` → Tests with info logging (default)
  - `just test-debug` / `just td` → Tests with debug logging (verbose)
- **Implementation**:
  - Script `scripts/start-server-for-tests.sh` accepts log level parameter
  - Sets `RUST_LOG` before starting test server
  - Playwright config uses `TEST_API_ONLY=true` to skip webServer management
  - Server lifecycle: stop → start with log level → test → stop

**Implementation Details**:
- `api/src/lib.rs::init_tracing()` - Conditional initialization with `#[cfg(feature = "dev-mode")]`
- `scripts/just-dev-debug-part.sh` - Dev server with `RUST_LOG=debug`
- `scripts/start-server-for-tests.sh` - Test server with configurable log level
- `web/playwright.config.ts` - `TEST_API_ONLY` flag for test mode

**Benefits**:
- ✅ Professional structured logging with severity levels
- ✅ Environment-based filtering (RUST_LOG) in development
- ✅ Production security: verbose logs physically impossible (code eliminated)
- ✅ Test flexibility: switch between info/debug for troubleshooting
- ✅ Zero overhead in production builds

**NEVER delete this section** - Copy to every Rust/Spin project using tracing

### Frontend Logging (TypeScript/JavaScript)

**CRITICAL: Logging Policy**
- **ALWAYS use logger wrapper** (`logger.error()`, `logger.warn()`, `logger.info()`, `logger.debug()`) for ALL logging
- **NEVER use console.*** directly for logging - Only allowed for temporary debugging (must be removed before commit)
- **Immediate migration**: Convert any console.* found in code to appropriate logger method
- **Code review**: Check for console.* in all new/modified code
- **This is MANDATORY** - Ensures production security (zero logs in production)
- **Copy this rule to EVERY frontend project CLAUDE.md** - Never delete when compacting/simplifying

**Architecture: Logger Wrapper + Terser Elimination**
- **Logger wrapper**: `web/src/lib/utils/logger.ts`
- **Log level mapping** (consistent with backend):
  - `logger.error()` → 🚨 Security violations, critical errors
  - `logger.warn()` → ⚠️ Anomalous situations, potential issues
  - `logger.info()` → ✅ Normal operations, successful flows (DEFAULT)
  - `logger.debug()` → 🔍 Verbose debugging information

**Development Mode**:
- **Default**: `VITE_LOG_LEVEL=info` (info, warn, error visible)
- **Configurable**: Set `VITE_LOG_LEVEL` environment variable
- **Commands**:
  - `just dev` → Normal development (info level)
  - `just dev-debug` / `just dd` → Verbose debugging (debug + info + warn + error)
  - NPM scripts: `npm run dev:debug`, `npm run dev:silent`

**Production Mode**:
- **ZERO console.*** - ALL calls eliminated by terser (drop_console: true)
- **NOT configurable** - Console code removed from bundle
- **Security**: Impossible to log in production (code eliminated by bundler)
- **Build**: Terser plugin removes ALL console.* during build

**Implementation**:
- `web/src/lib/utils/logger.ts` - Logger wrapper with level filtering
- `web/vite.config.ts` - Terser plugin with `drop_console: true` for production
- `web/src/vite-env.d.ts` - TypeScript types for VITE_LOG_LEVEL
- `scripts/just-dev-debug-part.sh` - Sets VITE_LOG_LEVEL=debug for frontend
- `web/package.json` - NPM scripts: dev:debug, dev:silent

**Migration Completed (2025-10-08)**:
- ✅ 68 console.* calls migrated across 26 files
- ✅ Production bundle: ZERO console.* (verified via grep)
- ✅ Breakdown: 15 error, 18 warn, 20 info, 15 debug

**Browser→Terminal Redirection (Development Only)**:
- **Critical for tablet development** - Tablet browser has NO DevTools access
- **Architecture**: Browser logs sent via Vite WebSocket to server terminal
- **Implementation**:
  - `web/vite-terminal-logger.ts` - Custom Vite plugin with WebSocket listener
  - `web/src/lib/utils/logger.ts` - `sendToTerminal()` function using `import.meta.hot.send()`
  - `web/vite.config.ts` - Plugin integration (dev only)
- **Format**: `[HH:MM:SS] [BROWSER ERROR] message` (only ERROR shows prefix, others only timestamp)
- **Colors**: ERROR=red, WARN=yellow, INFO=cyan, DEBUG=gray
- **Production**: Code eliminated by terser (zero overhead)

**Benefits**:
- ✅ Consistent logging system (backend + frontend)
- ✅ Environment-based filtering (VITE_LOG_LEVEL) in development
- ✅ Production security: ZERO logs (code eliminated by terser)
- ✅ Professional severity levels matching backend
- ✅ Zero runtime impact in production
- ✅ Tablet debugging without DevTools via terminal redirection

**NEVER delete this section** - Copy to every frontend project using Vite/SvelteKit

## Log Monitoring Standards - CRITICAL RULE - NEVER DELETE
**📺 MANDATORY: Use justfile commands for ALL real-time log monitoring:**

**CRITICAL: Log Viewing Policy**
- **ALWAYS use justfile commands** for viewing logs - NEVER use `tail -f` directly
- **Commands available**:
  - `just logs-api` / `just la` → Backend API logs (Spin, port 3000, `.spin-dev.log`)
  - `just logs-web` / `just lw` → Frontend web logs (Vite, port 5173, `.npm-dev.log`)
  - `just logs-predeploy` / `just lp` → Predeploy server logs (production simulation, `.spin-predeploy.log`)
  - `just watch` / `just w` → Both API + Web logs together
- **Why mandatory**: Consistency, validation (warns if server not running), clear context
- **Ctrl+C behavior**: Stops watching logs (does NOT stop servers)
- **This is MANDATORY** - Ensures consistent development workflow
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying

**Workflow**:
```bash
# Terminal 1: Start servers
just dev

# Terminal 2: Monitor backend
just la

# Terminal 3 (optional): Monitor frontend
just lw
```

**NEVER delete this section** - Copy to every project with background servers

## Enum/List Encoding Policy - CRITICAL RULE - NEVER DELETE
**📊 MANDATORY: Network payload optimization for ALL enum/fixed list fields:**

**GENERAL RULE**: All fixed enums/lists transmitted between client-server MUST use integer encoding to minimize network payload size.

**Why**: Network optimization - integers (1 byte) vs strings (2-20+ bytes)

**Examples**:
- ✅ `alphabet`: `"base58"` → `0` (saved 6 bytes, 85% reduction)
- ✅ `mnemonic language`: `"english"` → `0` (saved 7 bytes, 87% reduction)

**EXCEPTION (ONLY ONE)**:
- ❌ `email_lang`, `receiver_language`, `sender_language`: MUST remain ISO 639-1 strings (`"es"`, `"en"`, etc.)
  - **Reason**: Backend rust_i18n library requires ISO string codes for `set_locale()`
  - **Trade-off**: +1-2 bytes per request for external library compatibility
  - **Locations**: `LoginRequest.email_lang`, `CreateSharedSecretRequest.*_language`

**Enforcement**:
- When adding new enum fields, ALWAYS map to integers unless blocked by external dependency
- Document any future exceptions in code comments with clear justification
- See detailed documentation:
  - Frontend: `web/src/lib/types/index.ts` (top of file)
  - Backend: `api/src/utils/auth/types.rs` (module doc)

**NEVER delete this rule** - Copy to every project using network APIs

## Essential Commands
```bash
# Development (with logging control)
just dev         # PRIMARY: Development environment (RUST_LOG=info)
just dev-debug   # Development with verbose logging (RUST_LOG=debug)
just dd          # Alias for dev-debug
just stop        # Stop all services
just status      # Services status

# Log Monitoring (real-time)
just logs-api    # Follow backend API logs (Spin, port 3000)
just la          # Alias for logs-api
just logs-web    # Follow frontend web logs (Vite, port 5173)
just lw          # Alias for logs-web
just watch       # Follow both API + Web logs together
just w           # Alias for watch

# Testing (auto dry-run + logging management)
just test        # Run ALL tests with info logging (35 bash + 16 Playwright)
just test-debug  # Run ALL tests with debug logging (verbose)
just td          # Alias for test-debug
just test-bash   # Run ONLY bash integration tests (35 tests)
just test-api    # Run ONLY Playwright API tests (16 tests)

# Code Quality
just check       # Code quality (clippy + fmt + ESLint + svelte-check)
just build       # Build API (WASM) + Web (SPA)
```

## Architecture & Features
**Backend** (`api/src/`): handlers/, database/ (SQLite Zero Knowledge), utils/ (JWT, auth, ChaCha20)
**Frontend** (`web/src/`): routes/ (SPA), lib/components/ (AuthGuard, dialogs), lib/stores/ (auth, i18n 13 languages)
**Auth**: Zero Knowledge magic links + JWT + Ed25519 digital signatures + 2/3 automatic key rotation
**Security**: ChaCha20-Poly1305 URL encryption, MITM protection, dual-token system
**Testing**: 51 automated tests (35 bash + 16 Playwright), 100% success rate

## Key Endpoints
- `POST /api/{custom,password,api-key,mnemonic}` - Generation (JWT protected)
- `POST /api/shared-secret` - Create encrypted shared secret (JWT protected)
- `POST/GET /api/login/` - Auth flow with Zero Knowledge magic links
- `GET /api/version` - Public (no auth)

## Development Rules

**CRITICAL RULES:**
- **ALWAYS use justfile**: Check for justfile existence first. If exists, USE IT instead of manual commands. Run `just` to see tasks.
- **Code Changes**: Comment lines before changing (not MD files). Ask for approval. If liked, erase comments; if not, restore original.
- **Be Surgical**: Modify only necessary code. Write meaningful comments. Follow DRY and KISS principles.

## SOLID & DRY Architecture Standards - CRITICAL RULE
**🏗️ MANDATORY: Follow enterprise-grade architecture principles in ALL code creation and modification:**
- **ALWAYS apply SOLID and DRY principles** - Every time code is created or modified, observe possibility of making it more universal and reusable
- **225-line module limit** - If any file exceeds 225 lines, study modularization possibilities
- **Mandatory size check** - Check file sizes after EVERY coding task completion (creation/editing)
- **Modular thinking** - Consider Single Responsibility Principle for every function and class
- **Universal patterns** - Create reusable composables/modules to eliminate code duplication
- **Active code scanning** - Each time you edit or create new code, try to remember if there are other points in the project where similar or identical operations are being performed, to try to apply DRY, SOLID and KISS whenever possible
- **Pattern recognition enforcement** - Each time you edit or create new code, try to remember if there are other points in the project where similar or identical operations are being performed, to try to apply DRY, SOLID and KISS whenever possible
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying
- **This applies to ALL programming languages** - TypeScript, Rust, Python, JavaScript, etc.
- **Enterprise standard compliance** - All modules must be easily testable and modifiable in isolation
- **Styling**: DO NOT CHANGE existing CSS/TailwindCSS unless EXPLICITLY requested. New components must follow existing project styling.
- **Versioning**: CHANGELOG changes must be reflected in config files (package.json, Cargo.toml).
- **Spin Projects**: Use `spin-cli` not `cargo`. Only `cargo add/update/fmt/clippy` allowed.

## Git Workflow
**CRITICAL: ALWAYS use `git add .` for ALL commits**
- Git's .gitignore intelligence is superior to manual file selection
- Prevents missing files, saves time, eliminates human error
- NEVER manually select files with `git add file1 file2`
- Standard workflow: `git add . && git commit -m "message" && git push`

## Tech Stack
- **Backend**: Rust 2024 + Fermyon Spin + WebAssembly + SQLite
- **Frontend**: SvelteKit 2.x + TypeScript + TailwindCSS 4.0 + Vite 7.x
- **Quality**: ESLint 9 + Prettier + Clippy + TypeScript strict

## Context7 MCP Usage Rules
**CRITICAL: ALWAYS follow this Context7 usage pattern - NEVER delete this section**

**🎯 Simple Mandatory Rule:**
- **Need external updated documentation?** → **Use Context7**
- **Internal code/logic work?** → **No Context7 needed**

**✅ USE Context7 when:**
- Implementing with external libraries/frameworks ("use context7 for FastAPI auth")
- Updating dependencies (breaking changes, new APIs)
- Integrating external services (Stripe, Auth0, AWS, etc.)
- Debugging framework-specific issues (Spin, SvelteKit, etc.)
- Need updated best practices (security, performance)
- Working with fast-evolving technologies (WASM, Rust, JS frameworks)

**❌ DON'T use Context7 for:**
- Internal code refactoring
- Business logic specific to your project
- Basic git operations
- Pure styling (CSS/TailwindCSS)
- Debugging your custom code
- Project management tasks

**IMPORTANT**: This rule must be copied to ALL project CLAUDE.md files. Never delete when simplifying/compacting CLAUDE.md.

## CLAUDE.md Brevity Standards - CRITICAL RULE - NEVER DELETE
**📝 MANDATORY: Keep CLAUDE.md brief and focused:**
- **NEVER include extensive session explanations** - Use CHANGELOG.md for detailed work descriptions
- **ONLY ultra-summarized versions** in CLAUDE.md - Reference CHANGELOG.md for details
- **CLAUDE.md = Project context + Current state** - NOT a detailed work log
- **CHANGELOG.md = Detailed session history** - Full explanations, root causes, implementation details
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying
- **Add this rule to global ~/.claude/CLAUDE.md** - Must be in all projects
- **This is CRITICAL for maintainability** - Prevents CLAUDE.md bloat and information overload

---

**📚 Session History**: See [CHANGELOG.md](CHANGELOG.md) for complete implementation details, technical flows, and root cause analysis.
