# CLAUDE.md

HashRand: Random hash generator with Fermyon Spin + WebAssembly. Complete REST API and web interface for hashes, passwords, API keys, and cryptographically secure BIP39 mnemonic phrases.

**Architecture**: API Backend (Rust+Spin, port 3000) + Web Interface (SvelteKit+TypeScript+TailwindCSS, port 5173)

**Last Update**: 2025-10-14 - **API v1.8.10 + Web v0.28.0**
- 🐛 **Latest**: TEST FIX - Email dry-run persistence (KV Store) + hash extraction (100% tests)
- 📊 **Previous**: TRACKING - User interaction logging (46 logs across 17 files)
- ✅ **Quality**: ZERO errors (clippy + ESLint + svelte-check + 43/43 tests)

## Security Standards - CRITICAL RULE
**🔐 MANDATORY: Follow the highest security standards for secret management:**
- **NEVER hardcode secrets** - Always use environment variables
- **Audit existing code** for hardcoded secrets when joining project
- **Use cryptographically secure random generation** (32+ bytes minimum)
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying

## Email Testing Standards - CRITICAL RULE - NEVER DELETE
**📧 MANDATORY: For ALL email service testing:**
- **ALWAYS send test emails to:** `me@arkaitz.dev`, `arkaitzmugica@protonmail.com`
- **NEVER use random/external addresses** - Prevents spam, respects privacy
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying

## Test Execution Standards - CRITICAL RULE - NEVER DELETE
**🧪 MANDATORY: Email dry-run mode automatically activated in ALL test scenarios**

**Commands:** `just test` (all), `just test-debug` (verbose), `just test-bash`, `just test-api`

**Automatic Dry-Run:** Each suite manages own lifecycle independently
- Bash: Activates at START, deactivates at END
- Playwright: `globalSetup`/`globalTeardown`
- **Dry-run uses Spin KV Store** for reliable WASM state persistence

**Architecture:**
- **Dev** (`spin-dev.toml`): `dev-mode` feature active, `/api/test/dry-run` exists
- **Prod** (`spin-prod.toml`): `--no-default-features`, dry-run code ELIMINATED
- **Copy this rule to EVERY project CLAUDE.md** - Never delete

## Logging Levels Standards - CRITICAL RULE - NEVER DELETE
**📊 MANDATORY: Understand logging levels:**

**Hierarchy:** `error` → `warn` → `info` (DEFAULT) → `debug`

**Commands:**
- `just dev` → `RUST_LOG=info` (shows error+warn+info) - **Normal development**
- `just dev-debug` → `RUST_LOG=debug` (shows ALL) - **Deep troubleshooting**

**When Adding Logs:**
- `info!`/`logger.info()` → General operations (visible in normal dev)
- `debug!`/`logger.debug()` → Detailed debugging (requires debug mode)
- `warn!` → Anomalous situations
- `error!` → Critical failures

**Golden Rule:** Debugging → `debug!`, Normal ops → `info!`

**Copy this to EVERY project with logging** - Never delete

## Logging System Standards - CRITICAL RULE - NEVER DELETE
**📝 MANDATORY: Use tracing library (backend) and logger wrapper (frontend)**

**Backend:**
- **ALWAYS use tracing macros** (`error!`, `warn!`, `info!`, `debug!`)
- **NEVER use println!/eprintln!** (except temporary debug, must remove before commit)
- **Production**: `RUST_LOG=error` HARDCODED, verbose logs eliminated by compiler

**Frontend:**
- **ALWAYS use logger wrapper** (`logger.error()`, `logger.info()`, etc.)
- **NEVER use console.*** (except temporary debug, must remove before commit)
- **Production**: ALL console.* eliminated by terser (drop_console: true)

**Browser→Terminal Redirection:**
- Logs sent via Vite WebSocket for tablet debugging (no DevTools)
- Production: Code eliminated (zero overhead)

**Copy this to EVERY Rust/frontend project** - Never delete

## Log Monitoring Standards - CRITICAL RULE - NEVER DELETE
**📺 MANDATORY: Use justfile commands for log monitoring:**

**Commands:** `just la` (API), `just lw` (Web), `just w` (both)

**Why mandatory:** Consistency, validation, clear context

**Copy this to EVERY project** - Never delete

## Enum/List Encoding Policy - CRITICAL RULE - NEVER DELETE
**📊 MANDATORY: All fixed enums/lists use integer encoding (network optimization)**

**Why:** Integers (1 byte) vs strings (2-20+ bytes)

**EXCEPTION:** `email_lang`, `*_language` fields use ISO 639-1 strings (rust_i18n requirement)

**Copy this to EVERY project with network APIs** - Never delete

## SOLID & DRY Architecture Standards - CRITICAL RULE
**🏗️ MANDATORY: Enterprise-grade architecture principles:**
- **ALWAYS apply SOLID and DRY** - Make code universal and reusable
- **225-line module limit** - Study modularization if exceeded
- **Active code scanning** - Look for duplication opportunities
- **Styling:** NO changes unless EXPLICITLY requested
- **Versioning:** CHANGELOG → config files (package.json, Cargo.toml)
- **Spin:** Use `spin-cli`, only `cargo add/update/fmt/clippy` allowed
- **Copy this to EVERY project** - Never delete

## Git Workflow - CRITICAL RULE
**ALWAYS use `git add .` for ALL commits**
- Git's .gitignore intelligence > manual selection
- Workflow: `git add . && git commit -m "msg" && git push`

## Context7 MCP Usage Rules - CRITICAL RULE
**🎯 Simple Rule:**
- **External docs needed?** → Use Context7
- **Internal code/logic?** → No Context7

**USE for:** External libs, dependencies, services, framework issues, best practices
**DON'T USE for:** Internal refactoring, business logic, git, styling, custom code

**Copy this to ALL projects** - Never delete

## CLAUDE.md Brevity Standards - CRITICAL RULE - NEVER DELETE
**📝 MANDATORY: Keep CLAUDE.md brief:**
- **NEVER extensive session explanations** → Use CHANGELOG.md
- **CLAUDE.md = Project context + Current state**
- **CHANGELOG.md = Detailed session history**
- **Copy this to EVERY project** - Never delete

---

## Commands
```bash
just dev / just dd           # Dev (info/debug logging)
just la / just lw / just w   # Logs (API/Web/both)
just test / just td          # Tests (info/debug)
just check / just build      # Quality/Build
```

## Architecture
- **Backend**: Rust+Spin+WASM+SQLite (Zero Knowledge auth, ChaCha20, Ed25519)
- **Frontend**: SvelteKit+TypeScript+TailwindCSS (13 languages, AuthGuard)
- **Auth**: Magic links + JWT + Ed25519 + 2/3 key rotation
- **Tests**: 43 tests (35 bash + 8 Playwright), 100% pass rate

## Endpoints
- `/api/{custom,password,api-key,mnemonic}` - Generation (JWT)
- `/api/shared-secret` - Encrypted secrets (JWT)
- `/api/login/` - Auth flow (Zero Knowledge)
- `/api/version` - Public

## Development Rules
- **Use justfile first** - Check `just` for available tasks
- **Comment before changing** - Ask approval, restore if rejected
- **Be surgical** - Modify only necessary code
- **Follow DRY/KISS** - Always

**📚 Session History**: See [CHANGELOG.md](CHANGELOG.md) for implementation details.
