# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

HashRand Spin: Random hash generator with Fermyon Spin + WebAssembly. Complete REST API and web interface for cryptographically secure hashes, passwords, API keys, and BIP39 mnemonic phrases.

**Architecture**: Workspace with API Backend (`/api/` - Rust+Spin, port 3000) and Web Interface (`/web/` - SvelteKit+TypeScript+TailwindCSS, port 5173)

## Essential Commands

```bash
just dev         # PRIMARY: Complete development environment (API + Web + Tailscale)
just stop        # Stop all services
just status      # Check all service status
just test        # Run 64 automated tests
just check       # Code quality (clippy + fmt + ESLint + svelte-check)
just build       # Build API (WASM) + Web (SPA)
```

## Architecture Overview

**Backend** (`api/src/`):
- `handlers/` - API endpoints (custom, password, api-key, mnemonic, users, login)
- `database/` - SQLite with Zero Knowledge schema (no PII storage)
- `utils/` - JWT, auth middleware, routing, ChaCha8 random generation
- **Authentication**: Zero Knowledge magic links + JWT endpoint protection (access 20s dev, refresh 2min dev)

**Frontend** (`web/src/`):
- `routes/` - SPA routes with frictionless auth (explore first, authenticate later)
- `lib/components/` - AuthGuard, LoginDialog, EmailInputDialog with state preservation
- `lib/stores/` - Auth, i18n (13 languages + RTL), theme, navigation
- **Features**: SVG sprite system, TailwindCSS 4.0, complete internationalization

## Key Endpoints
- `GET/POST /api/{custom,password,api-key,mnemonic}` - Generation endpoints (JWT protected)
- `POST/GET /api/login/` - Zero Knowledge magic link auth flow
- `GET/POST/DELETE /api/users` - User management (JWT protected)
- `GET /api/version` - Public endpoint (no auth required)

## Development Guidelines

**CRITICAL RULES:**
- **ALWAYS use justfile**: Check for justfile existence first. If exists, USE IT instead of manual commands. Run `just` to see tasks.
- **Code Changes**: Comment lines before changing (not MD files). Ask for approval. If liked, erase comments; if not, restore original.
- **Be Surgical**: Modify only necessary code. Write meaningful comments. Follow DRY and KISS principles.
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

## Session History: Zero Knowledge Implementation (2025-08-29)

### Major Accomplishment: Zero Knowledge Authentication System
This session implemented a complete **Zero Knowledge (ZK) architecture** where the server never stores or processes user emails or personal information:

#### ✅ Core ZK Components Implemented:
1. **JWT Authentication Middleware** (`api/src/utils/auth.rs`)
   - Bearer token validation for all protected endpoints
   - Professional 401 error responses with WWW-Authenticate headers
   - Route-level protection configuration (public vs protected endpoints)

2. **Zero Knowledge Database Schema Refactoring**
   - **Users table**: Removed all PII fields (username, email, updated_at)
   - **BLOB Primary Keys**: 32-byte cryptographic user IDs replace sequential integers
   - **Privacy-Preserving**: Only stores `user_id BLOB` and `created_at INTEGER`

3. **Cryptographic User ID System** (enhanced `utils/jwt.rs`)
   - **Deterministic Derivation**: `SHA3-256(email) → PBKDF2-SHA3-256(600k iter.) → user_id`
   - **Base58 Usernames**: Human-readable display without PII exposure
   - **Magic Link Integrity**: HMAC-SHA3-256 verification prevents tampering

4. **JWT Token Duration Optimization** (for development testing)
   - **Access tokens**: 20 seconds (was 15 minutes)
   - **Refresh tokens**: 2 minutes (was 1 week)
   - Enables rapid authentication flow testing

#### ✅ Endpoint Protection Status:
- **Protected (requires JWT)**: `/api/custom`, `/api/password`, `/api/api-key`, `/api/mnemonic`, `/api/from-seed`, `/api/users/*`
- **Public (no auth)**: `/api/version`, `/api/login/*`

#### ✅ Testing Completed:
- Confirmed endpoints work without Bearer tokens → 401 Unauthorized ✅
- Public endpoints remain accessible ✅  
- Valid Bearer tokens grant access ✅
- Token expiration (20s) properly handled ✅

#### ✅ Documentation Updated:
- **CHANGELOG.md**: New v1.4.2/v0.19.2 release with comprehensive ZK implementation details
- **README.md**: Repositioned as "Zero Knowledge random hash generator" with detailed ZK architecture section
- **CLAUDE.md**: Updated architecture overview to reflect ZK implementation

### Files Modified in This Session:
```
api/src/utils/auth.rs          # NEW - JWT authentication middleware
api/src/utils/mod.rs           # Export auth functions
api/src/utils/routing.rs       # Added authentication checks before handlers
api/src/utils/jwt.rs           # Updated token durations (20s/2min)  
api/src/handlers/login.rs      # Updated expires_in response (20s)
api/src/database/connection.rs # Users table schema (ZK migration)
api/src/database/models.rs     # Removed PII fields
api/src/database/operations.rs # Updated for BLOB user_id PKs
api/Cargo.toml                 # Dependencies unchanged
Cargo.lock                     # Lock file updates
data/hashrand-dev.db          # Database schema migration
README.md                      # Major ZK documentation updates
CHANGELOG.md                   # New v1.4.2 ZK release documentation
```

### Zero Knowledge Benefits Achieved:
- **Complete Email Privacy**: Server never stores email addresses
- **Cryptographic Security**: Industry-standard key derivation (600k PBKDF2 iterations)
- **Audit Trail Privacy**: All logs use Base58 usernames, not PII
- **Endpoint Security**: All sensitive operations require valid authentication
- **Scalable Architecture**: ZK system supports millions of users without PII concerns

### Development Testing Notes:
- Short token durations (20s access, 2min refresh) perfect for rapid testing
- Magic links logged to console in development mode
- All endpoints properly protected - confirmed via curl testing
- Authentication flow works seamlessly from magic link to JWT validation

### Next Session Considerations:
- Consider extending token durations for production deployment
- Add refresh token endpoint if needed for long-lived sessions
- Test complete authentication flow in web interface
- Consider rate limiting for authentication endpoints

## Additional Details
Check README.md and CHANGELOG.md for complete implementation details.