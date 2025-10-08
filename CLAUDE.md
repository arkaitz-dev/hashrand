# CLAUDE.md

HashRand: Random hash generator with Fermyon Spin + WebAssembly. Complete REST API and web interface for hashes, passwords, API keys, and cryptographically secure BIP39 mnemonic phrases.

**Architecture**: Workspace with API Backend (`/api/` - Rust+Spin, port 3000) and Web Interface (`/web/` - SvelteKit+TypeScript+TailwindCSS, port 5173)

**Last Update**: 2025-10-08 - **API v1.8.6 + Web v0.27.9**
- 📝 **Latest**: INFRA - Migrated to Rust tracing library with compilation-time logging control (dev-mode feature flag)
- ⚡ PERF - Eliminated useless `raw` parameter from all requests (~10 bytes/request saved) - v0.27.9
- 🐛 CRITICAL FIX - Expiration date showing year 1970 (backend returning hardcoded expires_at:0) - v1.8.4
- 🎨 UI simplification - Removed redundant "(lecturas limitadas)" from receiver role in all 13 languages - v0.27.1
- 🐛 CRITICAL FIX - Incomplete URLs in shared secret emails/response (added ui_host + protocol logic) - v1.8.3/v0.27.0
- 🌐 Multi-language email support - Fixed placeholders + language selector for shared secret emails (13 languages) - v0.26.0
- 🔐 Shared Secret Feature - Secure text sharing with encryption, dual-URL system, OTP protection (v1.8.0 + v0.25.0)
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

## General Architecture
**Backend** (`api/src/`): handlers/, database/ (SQLite Zero Knowledge), utils/ (JWT, auth, ChaCha20)
**Frontend** (`web/src/`): routes/ (SPA), lib/components/ (AuthGuard, dialogs), lib/stores/ (auth, i18n 13 languages)
**Auth**: Zero Knowledge magic links + JWT (durations: see `.env` configuration above)

## Key Endpoints
- `POST /api/{custom,password,api-key,mnemonic}` - Generation (JWT protected)
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


## 🚧 Current Work in Progress

**No active work in progress** - Codebase stable at v1.8.0 + v0.25.1

## Recent Session History

**Latest versions**: API v1.8.0 + Web v0.25.1 (2025-10-04)

**Recent sessions summary**:
- **v0.25.1**: 🎨 UX - Automatic email display in Shared Secret (readonly, from IndexedDB)
- **v1.8.0 + v0.25.0**: 🔐 Shared Secret Feature - Complete encrypted text sharing system
- **v1.7.1 + v0.24.0**: Client-side logout architecture + unified cleanup (DRY)
- **v0.23.2**: Instant UI loading + DRY improvements + cleanup
- **v1.7.0 + v0.22.0**: MAJOR - Enterprise-grade SOLID/DRY/KISS refactoring

**📚 For complete details**: See [CHANGELOG.md](CHANGELOG.md) - root causes, technical flows, implementation details, file modifications, and testing results.

---

## Architecture and Main Features

### Zero Knowledge Auth
- Server never stores emails/PII
- Cryptographic User IDs (Blake3 pipeline)
- Single-use unique magic links

### Ed25519 Key Rotation (2/3 System)
- **TRAMO 1/3**: Partial refresh (access token only)
- **TRAMO 2/3**: Complete key rotation (access + refresh + keypairs)
- 100% functional system after fixes v1.6.23-v1.6.34
- MITM protection with dual-key signing
- Zero session loss during rotation

### Dual-Token JWT
- Access tokens: 1min dev, 15min prod
- Refresh tokens: 5min dev, 8h prod
- Transparent auto-refresh with HttpOnly cookies
- Dynamic configuration via `.env`

### Testing & Quality
- **51 automated tests** (35 bash + 16 Playwright)
- 100% success rate across all suites
- Enterprise architecture: modules <225 lines
- DRY/SOLID/KISS principles enforced

---

## Key Project Achievements (Summary)

- **Enterprise Architecture**: Refactoring 3,698 monolithic lines → modules <225 lines
- **Blake3 Migration**: ~100x performance in magic links (WASM SIMD optimization)
- **Ed25519 Integration**: Complete frontend-backend system with digital signatures
- **Ed25519 Key Rotation**: Automatic 2/3 window system, transparent rotation ✅
- **URL Encryption**: ChaCha20-Poly1305 with 66% size reduction (FIFO rotation)
- **100% SignedResponse**: ALL endpoints validate Ed25519 (except `/api/version`)
- **Email System**: Mailtrap integration, 13 languages + RTL support
- **Testing**: Complete coverage auth flow + generation + key rotation

## Additional Details

See README.md and CHANGELOG.md for complete implementation details, technical flows, and root cause analysis.