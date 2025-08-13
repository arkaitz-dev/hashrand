# CLAUDE.md

## Project: hashrand (v0.6.0)
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

### Current State (v0.6.0)
- **Stack**: Rust 1.89.0 + Lit 3.3.1 + Vite 7.1.1 + **TailwindCSS 4.1.11**
- **Architecture**: Self-contained binary with embedded optimized web assets
- **Testing**: 46/46 tests passing
- **Security**: Rate limiting, CORS, audit logging, path validation
- **Styling**: **Complete TailwindCSS migration** with 48% bundle size reduction

### Major Changes (v0.6.0)
- **TailwindCSS Integration**: Complete migration from custom CSS to TailwindCSS
- **Bundle Optimization**: 48% total size reduction (86kB → 45kB)
- **Component Refactoring**: All 7 Lit components migrated to Tailwind classes
- **Build System**: Advanced Terser optimization + intelligent chunking
- **Performance**: 95% reduction in main JS bundle (79kB → 3.7kB)

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

## Completed Session (2025-08-13)

### Session Summary
**Duration**: ~3 hours | **Branch**: master | **Version**: 0.5.0 → 0.6.0

**Primary Accomplishment**: Complete TailwindCSS integration with major performance optimization

### Major Work Completed

#### 1. TailwindCSS Migration (Phase 1-3)
- **Installation & Configuration**: TailwindCSS v4.1.11 + @tailwindcss/postcss plugin
- **Vite Integration**: Updated build system with PostCSS pipeline  
- **CSS Entry Point**: Replaced `web-ui/src/css/main.css` with Tailwind directives
- **Component Refactoring**: Migrated all 7 Lit components to utility classes
  - `hash-generator.js`: Menu cards with responsive grid
  - `hash-result.js`: Result display with navigation buttons
  - `language-selector.js`: Dropdown with RTL support
  - `header-title.js`: Title and version display
  - `generic-hash-view.js`: Form inputs and controls
  - `password-view.js`: Password generation interface
  - `api-key-view.js`: API key generation interface
- **HTML Container**: Updated main layout with Tailwind classes
- **Shared Styles**: Created `web-ui/src/shared-styles.js` for consistent imports

#### 2. Bundle Optimization (Advanced)
- **Terser Configuration**: Aggressive JS minification with console.log removal
- **Smart Chunking**: Separated vendor, locales, and app code
  - `lit-core`: 1.00 kB (framework cache)
  - `locales`: 32.38 kB (all 12 languages)  
  - `index`: 3.71 kB (main app logic)
- **TailwindCSS Optimization**: Disabled 20+ unused utility groups
- **Tree Shaking**: Enhanced dead code elimination
- **Bundle Analysis**: Added `npm run build:analyze` with visualizer

#### 3. Performance Results
- **Total Bundle**: 86.16 kB → 44.93 kB (48% reduction)
- **Main JS**: 78.96 kB → 3.71 kB (95% reduction) 
- **Gzipped Total**: 19.32 kB → 11.69 kB
- **Main JS Gzipped**: 17.53 kB → 1.46 kB

#### 4. Version Management & Documentation
- **Version Bump**: 0.5.0 → 0.6.0 (semver minor)
- **Files Updated**: package.json, Cargo.toml, CLAUDE.md
- **CHANGELOG.md**: Comprehensive v0.6.0 entry with technical details
- **README.md**: Added performance section and TailwindCSS mentions
- **docs/GUIDE.md**: Updated web interface architecture section
- **Git Commits**: 2 detailed commits with proper attribution

### Technical Decisions Made

1. **TailwindCSS v4 with PostCSS**: Chosen for latest features and build integration
2. **Utility-First Approach**: Complete migration from component-specific CSS
3. **Smart Chunking Strategy**: Optimized for caching and loading performance  
4. **Shared Styles Pattern**: Centralized imports for maintainability
5. **Production-Only Optimizations**: Terser and purging only in build mode

### Files Created/Modified
```
Created:
+ tailwind.config.js (TailwindCSS configuration)
+ web-ui/src/shared-styles.js (Shared style imports)

Modified:
~ package.json (version, build scripts, dependencies)
~ Cargo.toml (version bump)
~ vite.config.js (PostCSS, optimization, chunking)
~ web-ui/index.html (Tailwind layout classes)
~ web-ui/src/css/main.css (Tailwind directives only)
~ web-ui/src/components/*.js (7 components migrated)
~ CLAUDE.md (session tracking, version)
~ CHANGELOG.md (v0.6.0 entry)
~ README.md (performance section, tech stack)
~ docs/GUIDE.md (architecture updates)
```

### Testing & Validation
- ✅ Development server: Functional on port 3019
- ✅ Production build: Successful compilation (917ms)
- ✅ Bundle analysis: Generated optimization metrics
- ✅ All 46 tests: Passing
- ✅ Functionality: Maintained across all features

### Next Session Priorities
1. Component testing (@web/test-runner) - original priority maintained
2. Theme switching (dark/light) - enhanced by TailwindCSS
3. TypeScript migration - easier with utility classes
4. PWA features - improved by smaller bundles
5. Batch generation - original roadmap item

### Key Learnings
- TailwindCSS v4 requires separate @tailwindcss/postcss plugin
- CSS import attributes incompatible with Vite - use JS imports instead
- Smart chunking more effective than manual chunk definitions
- Bundle size optimization yields significant UX improvements

### Technical Implementation Summary
- **TailwindCSS v4.1.11**: Installed with @tailwindcss/postcss plugin
- **Component Refactoring**: All 7 components migrated from custom CSS to Tailwind classes
- **Shared Styles**: Created `shared-styles.js` for consistent cross-component styling
- **Build System**: Successfully configured Vite + TailwindCSS + Lit integration
- **Testing**: Dev server (localhost:3019) and production build both working correctly

### Bundle Optimization Results (Completed)
**BEFORE Optimization:**
- Main JS: 78.96 kB (17.53 kB gzip)
- CSS: 7.20 kB (1.79 kB gzip)
- Total: ~86.16 kB (19.32 kB gzip)
- 13 separate locale files (2.4-3.9 kB each)

**AFTER Optimization:**
- Main JS: 3.71 kB (1.46 kB gzip) - **95% reduction**
- Lit Core: 1.00 kB (0.53 kB gzip) - **cached separately**
- Locales: 32.38 kB (7.86 kB gzip) - **bundled together**
- CSS: 7.84 kB (1.84 kB gzip) - **minimal increase**
- Total: ~44.93 kB (11.69 kB gzip) - **48% total reduction**

### Optimization Techniques Applied:
1. **Terser with aggressive settings**: Removed console logs, 2-pass compression
2. **Manual chunking**: Separate vendor, locales, and core chunks for better caching
3. **Tree-shaking**: Aggressive dead code elimination
4. **TailwindCSS pruning**: Disabled 20+ unused utility groups
5. **CSS code splitting**: Independent CSS loading
6. **Bundle analysis**: Configured for future monitoring
