# CLAUDE.md

## Project: hashrand (v0.5.0)
CLI/Web tool for cryptographically secure random string generation (Rust + Lit + Vite)

## Architecture
```
hashrand/
├── src/                 # Rust backend (modular)
├── web-ui/             # Frontend (Lit components)  
└── dist/               # Build output (embedded in release)
```

## Development Workflow

### Quick Start
```bash
# Development (recommended)
just dev                    # Launch both servers + Tailscale serve
just stop-dev              # Stop all servers and cleanup

# Production  
just build && just serve    # Manual production build & serve
just run-installed          # Install binary + run on port 3000 + Tailscale
just stop-installed         # Stop production binary and Tailscale
```

### Commands

#### Development
- **`just dev`**: Development (frontend:3000 + API:8080) + Tailscale serve
- **`just stop-dev`**: Stop all dev servers and Tailscale serve
- **`just status`**: Check server status

#### Production
- **`just build`**: Production build (npm + cargo)
- **`just serve`**: Run production server
- **`just install`**: Install binary locally (with tests)
- **`just run-installed`**: Install binary + run on port 3000 + Tailscale serve
- **`just stop-installed`**: Stop installed binary and Tailscale serve

#### Testing
- **`just test`**: Run all tests (46 passing)

### Remote Development & Production
- **Development**: `just dev` automatically configures Tailscale serve if installed
- **Production**: `just run-installed` installs binary and serves on port 3000 with Tailscale
- **Secure HTTPS Access**: Remote access via `https://machine-name.ts.net`
- **Zero Configuration**: Works out-of-the-box, no manual setup required
- **Auto-cleanup**: Both `just stop-dev` and `just stop-installed` properly clean up Tailscale

## Key Implementation Notes

### Current State (v0.5.0)
- **Stack**: Rust 1.89.0 + Lit 3.3.1 (standard decorators) + Vite 7.1.1
- **Architecture**: Self-contained binary (3.1MB) with embedded web assets
- **Testing**: 46/46 tests passing
- **Security**: Rate limiting, CORS, audit logging, path validation

### Recent Changes (v0.5.0)
- **Tailscale Integration**: Automatic HTTPS remote development access
- **Workflow Simplification**: Cleaned up Vite configuration and build process
- **Zero-Config Remote Access**: Smart Tailscale detection and setup

### Previous Changes (v0.3.0 - v0.4.0)
- **Alphabet Selection**: Custom alphabets for passwords and API keys
- **Version Display**: New `/api/version` endpoint + frontend header display
- **CLI Enhancement**: Version option + comprehensive justfile workflow
- **Lit Migration**: Official decorators syntax `@state() accessor property = value`
- **Asset Embedding**: Conditional compilation (dev: API-only, prod: embedded assets)
- **Architecture**: Complete frontend/backend separation with improved navigation

## Next Priorities
1. Component testing (@web/test-runner)
2. Theme switching (dark/light)
3. TypeScript migration
4. PWA features
5. Batch generation

## Development Context
- Commands work from project root
- Zero breaking changes in refactors  
- Single binary deployment
- Professional security posture maintained
- Tailscale integration for secure remote development
- Simplified configuration for reliable cross-platform development

## Recent Session (Jan 13, 2025)

### Major Accomplishment: Multi-Language Support Expansion
Successfully expanded i18n system from 8 to 12 languages, adding:
- 🏴 Euskera (EUS) - Basque/Vascuence
- 🏴 Català (CAT) - Catalan
- 🏴 Galego (GAL) - Galician  
- 🇯🇵 日本語 (ja) - Japanese

**Technical Implementation:**
- Updated `lit-localize.json` configuration for new target locales
- Generated complete XLIFF files (227+ translation strings each)
- Built JavaScript locale files with full UI coverage
- Enhanced language selector with appropriate flags/codes

**Commit:** `bf46616 feat(i18n): add support for 4 new languages`

### UI/UX Polish: Focus Management System
Resolved persistent button focus issues across the application:

**Problem:** Buttons remained visually highlighted after clicking, degrading UX
**Solution:** Comprehensive focus management system implemented

**Files Modified:**
- `web-ui/src/css/main.css`: Added `.wc-button:focus { outline: none; }`
- `web-ui/src/components/language-selector.js`: Transparent default state with hover effects
- `web-ui/src/components/hash-generator.js`: Menu cards focus elimination + blur handlers
- `web-ui/src/components/hash-result.js`: Navigation buttons focus removal

**Key Techniques:**
- CSS `outline: none` for focus elimination
- JavaScript `.blur()` calls in event handlers
- Transparent default states with hover transitions
- `tabindex="0"` + blur management for non-button elements

### Current State (v0.5.0+)
- **Languages:** 12 total (English + 11 translations)
- **Focus System:** Professional, no persistent highlights
- **Language Selector:** Clean, minimalist (flag + globe icon)
- **Regional Support:** Proper RTL alignment for Arabic
- **Translation Coverage:** 100% UI strings translated

### Next Priorities
1. Component testing (@web/test-runner) 
2. Theme switching (dark/light)
3. TypeScript migration
4. PWA features
5. Batch generation

### Technical Notes
- All buttons and interactive elements now have clean focus states
- Language selector shows transparent → hover transitions  
- Menu cards properly handle tabindex and blur events
- Navigation buttons eliminate focus after click events
- Zero breaking changes maintained throughout
