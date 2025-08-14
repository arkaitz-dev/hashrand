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

## Session (2025-08-13 PM) - Route Renaming, SPA Improvements & Advanced Preloader

### Session Summary
**Duration**: ~3 hours | **Branch**: master | **Version**: 0.6.0 (no version change)

**Primary Accomplishments**: 
1. Route renaming from /generic to /custom
2. SPA fallback configuration for development and production
3. Vite build optimization simplification  
4. Advanced preloader with real progress tracking and theme detection

### Changes Made

#### 1. Route Renaming (/generic → /custom)
**Files modified:**
- `web-ui/src/pages/generic-hash.js` → `web-ui/src/pages/custom-hash.js`
- Updated class `GenericHashView` → `CustomHashView`
- Updated all route references throughout the application
- Files affected: `index.js`, `menu.js`, `hash-result.js`

#### 2. SPA Fallback Configuration
**Development (Vite):**
- Added `historyApiFallback` to `vite.config.js`
- All non-asset routes return `index.html` for router handling

**Production (Axum):**
- Verified existing fallback in `serve_embedded_assets` function
- Already correctly returns `index.html` for unknown routes

#### 3. Vite Configuration Simplification
**Removed:**
- Complex Terser optimizations
- Manual chunking strategies
- Custom move-scripts-to-body plugin

**Result:**
- Simpler, more maintainable configuration
- Bundle size: ~33KB gzip (HTML + JS + CSS + 1 locale)
- 15 total files vs previous multiple chunks

#### 4. Advanced Preloader Implementation
**Evolution stages:**
1. **Basic version**: Simple text "0%" without styling
2. **Progress tracking**: Real-time download monitoring using Fetch API
3. **Visual bar**: Blue progress bar (200px wide)
4. **Centered layout**: Perfect viewport centering
5. **Theme detection**: Automatic dark/light mode support

**Final features:**
- Real-time progress using ReadableStream API
- Visual progress bar with smooth transitions
- System theme detection (`prefers-color-scheme`)
- Fullscreen overlay during loading
- Light mode: White background, blue bar (#4285f4)
- Dark mode: Gray background (#2d3748), light blue bar (#63b3ed)

### Technical Implementation Details

**Preloader structure:**
```javascript
// Detect theme
const isDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;

// Track real download progress
async function trackAssetDownload(url, weight = 1) {
    const response = await fetch(url);
    const reader = response.body.getReader();
    // Read chunks and update progress
}
```

**Files modified:**
- `web-ui/index.html`: Added inline preloader script
- HTML size increased: 0.86KB → 6.61KB (includes preloader code)

### Next Session Priorities (unchanged)
1. Component testing (@web/test-runner)
2. Theme switching (dark/light) for main app
3. TypeScript migration
4. PWA features
5. Batch generation

## Session (2025-08-13) - Frontend Architecture Reorganization

### Session Summary
**Duration**: ~1 hour | **Branch**: master | **Version**: 0.6.0 (no version change)

**Primary Accomplishment**: Frontend code organization and component structure improvements

### Changes Made

#### 1. Directory Structure Reorganization
**Created**: `web-ui/src/pages/` directory for main application pages
**Moved components:**
- `components/generic-hash-view.js` → `pages/generic-hash.js`
- `components/password-view.js` → `pages/password.js` 
- `components/api-key-view.js` → `pages/api-key.js`
- `components/hash-generator.js` → `pages/menu.js`
- `components/hash-result.js` → `pages/hash-result.js`

**Components directory cleanup:**
- Now contains only reusable UI components: `header-title.js`, `language-selector.js`

#### 2. Systematic Renaming (-view → -page)
**File renames:**
- `hash-generator.js` → `menu.js`

**ID Updates:**
- `menu-view` → `menu-page`
- `generate-view` → `generate-page`
- `password-view` → `password-page` 
- `apikey-view` → `apikey-page`
- `result-view` → `result-page`

**Custom Element Updates:**
- `hash-generator` → `menu-page`
- `generic-hash-view` → `generic-hash-page`
- `password-view` → `password-page`
- `api-key-view` → `api-key-page`
- Class name: `HashGenerator` → `MenuPage`

#### 3. Import/Reference Updates
**Updated files:**
- `index.js`: All page imports updated to new paths
- `index.html`: Main component reference updated
- `menu.js`: All internal references and selectors updated
- All moved components: Relative imports preserved

### Architecture Impact

**Before:**
```
web-ui/src/components/
├── header-title.js (UI component)
├── language-selector.js (UI component)
├── hash-generator.js (main menu - misplaced)
├── generic-hash-view.js (page - misplaced)
├── password-view.js (page - misplaced)
├── api-key-view.js (page - misplaced)
└── hash-result.js (page - misplaced)
```

**After:**
```
web-ui/src/
├── components/ (Pure UI components)
│   ├── header-title.js
│   └── language-selector.js
└── pages/ (Application pages)
    ├── menu.js
    ├── generic-hash.js
    ├── password.js
    ├── api-key.js
    └── hash-result.js
```

### Benefits Achieved
1. **Clear Separation of Concerns**: Pages vs reusable components
2. **Better Maintainability**: Logical organization for future development
3. **Consistent Naming**: All pages use `-page` suffix instead of mixed naming
4. **Improved Navigation**: Clearer mental model for developers

### Technical Validation
- ✅ All 46 tests passing
- ✅ Development server starts correctly
- ✅ No breaking changes in functionality
- ✅ Zero remaining references to old naming

### Next Session Priorities (Unchanged)
1. Component testing (@web/test-runner)
2. Theme switching (dark/light)
3. TypeScript migration
4. PWA features
5. Batch generation

## Previous Session (2025-08-13) - TailwindCSS Integration

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

## Session (2025-08-13) - SPA Navigation with @vaadin/router

### Session Summary
**Duration**: ~2 hours | **Branch**: master | **Version**: 0.6.0 (no version change)

**Primary Accomplishment**: Complete Single Page Application (SPA) routing system implementation

### Changes Made

#### 1. @vaadin/router Integration
**Installation & Configuration:**
- Installed `@vaadin/router` dependency
- Configured router in `web-ui/src/index.js` with complete route mapping
- Updated `web-ui/index.html` to use `<div id="router-outlet">` instead of direct component

**Route Structure Implemented:**
```
/ → menu-page (landing page)
/generic → generic-hash-page (generic hash configuration)
/password → password-page (password configuration)  
/api-key → api-key-page (API key configuration)
/generic/result → hash-result-page (generic hash results)
/password/result → hash-result-page (password results)
/api-key/result → hash-result-page (API key results)
```

#### 2. Component Navigation Refactoring
**Menu Component (`pages/menu.js`):**
- Removed complex state management and view switching logic
- Simplified to pure menu display with `Router.go()` navigation
- Each card now directly navigates to corresponding route
- Eliminated event-based parent-child communication

**Configuration Pages Refactoring:**
- `generic-hash.js`: Direct API calls + navigation to `/generic/result`
- `password.js`: Direct API calls + navigation to `/password/result`  
- `api-key.js`: Direct API calls + navigation to `/api-key/result`
- All use `sessionStorage` for parameter and result passing
- Back button uses `Router.go('/')` to return to menu

**Result Page (`hash-result.js`):**
- Complete refactoring from prop-based to `sessionStorage`-based data loading
- `loadFromSession()` method for automatic data retrieval on route entry
- Smart navigation: Back to Config navigates to appropriate configuration route
- In-place regeneration without route changes
- Custom element renamed to `hash-result-page` for router consistency

#### 3. State Management Migration
**From Event-Based to SessionStorage:**
- Parameters: `sessionStorage.setItem('hashrand-last-params', JSON.stringify(data))`
- Results: `sessionStorage.setItem('hashrand-last-result', result)`
- Errors: `sessionStorage.setItem('hashrand-last-error', error.message)`
- Automatic cleanup after consumption

**Data Flow:**
```
Configuration Page → API Call → sessionStorage → Router.go('/*/result') → Result Page → loadFromSession()
```

#### 4. Architecture Transformation
**Before (Event-Driven MPA):**
```
menu.js (coordinator)
├── Shows/hides different views
├── Handles all API calls
├── Manages state for all components
└── Complex event listener system
```

**After (Router-Driven SPA):**
```
/ → menu.js (pure navigation)
/generic → generic-hash.js (self-contained)
/password → password.js (self-contained)
/api-key → api-key.js (self-contained)
/*/result → hash-result.js (sessionStorage-based)
```

### Technical Benefits Achieved

#### 1. **True SPA Experience:**
- ✅ Browser back/forward buttons work correctly
- ✅ Deep linking to any route (e.g., `/password/result`)
- ✅ URL reflects current application state
- ✅ No page refreshes during navigation

#### 2. **Improved Developer Experience:**
- ✅ Clear separation of concerns per route
- ✅ Self-contained pages with independent logic
- ✅ Simplified debugging (one route = one component)
- ✅ Easier testing of individual pages

#### 3. **Better User Experience:**
- ✅ Instant navigation with smooth transitions
- ✅ Bookmarkable URLs for specific generators
- ✅ Browser history integration
- ✅ Consistent navigation patterns

#### 4. **Code Quality Improvements:**
- ✅ Removed complex parent-child event communication
- ✅ Eliminated monolithic component with multiple views
- ✅ Reduced coupling between components
- ✅ Clear data flow patterns

### Files Modified
```
Modified:
~ web-ui/package.json (+@vaadin/router dependency)
~ web-ui/src/index.js (router configuration)
~ web-ui/index.html (router-outlet integration)
~ web-ui/src/pages/menu.js (simplified navigation)
~ web-ui/src/pages/generic-hash.js (self-contained + routing)
~ web-ui/src/pages/password.js (self-contained + routing)
~ web-ui/src/pages/api-key.js (self-contained + routing)
~ web-ui/src/pages/hash-result.js (sessionStorage-based + routing)
```

### Technical Validation
- ✅ All 46 tests passing
- ✅ Zero breaking changes to functionality
- ✅ Development server integration working
- ✅ All routes accessible and functional
- ✅ State management working across navigation

### Next Session Priorities (Updated)
1. **Frontend testing integration** - @web/test-runner for component testing
2. **Theme switching** - Dark/light mode with TailwindCSS
3. **Enhanced UX polish** - Loading states, transitions, error boundaries
4. **TypeScript migration** - Better type safety for routing
5. **PWA features** - Service worker, offline capability

### Key Learnings
- **@vaadin/router** provides lightweight, powerful SPA routing for Lit components
- **sessionStorage** is effective for temporary state passing between routes
- **Component isolation** improves maintainability significantly
- **Router-driven architecture** scales better than event-driven patterns
- **URL-based state management** enhances user experience fundamentally

### Session Impact Summary
**🎯 Mission Accomplished**: Transformed from MPA-style navigation to modern SPA with complete URL routing, maintaining all existing functionality while improving UX, DX, and code architecture.

## Current Session (2025-01-13)

### Session Start
**Time**: Started at current time
**Branch**: master (clean, up to date)
**Version**: 0.6.0

### Session Accomplishments

#### Language Selector Fix
✅ **Problema resuelto**: El selector mostraba múltiples botones en lugar de un dropdown
- Creado nuevo componente con CSS nativo (sin Tailwind en el componente)
- Un solo botón visible con: bandera actual + 🌐 + flecha
- Dropdown real con transiciones suaves y lista estructurada
- Posicionamiento correcto en esquina superior derecha

#### Theme Toggle Implementation
✅ **Switch claro/oscuro funcional**
- Detecta preferencia del sistema automáticamente
- Botón con iconos sol/luna
- Persiste preferencia en localStorage
- Clase 'dark' aplicada al documento

#### Professional Design Overhaul
✅ **Diseño austero y profesional**
- Eliminados gradientes llamativos (púrpura/indigo)
- Color principal: azul profesional (#3b82f6)
- Fondo: gris claro sólido (light) / gris oscuro (dark)
- Header: azul sólido sin gradientes
- Consistencia con colores del preloader
- Sombras más sutiles
- Transiciones más rápidas (200ms)

#### Technical Changes
- Nuevo `language-selector.js` con CSS interno para control total
- `theme-toggle.js` componente funcional
- Soporte dark mode en todos los componentes
- Tailwind configurado con `darkMode: 'class'`
- Colores actualizados en todas las páginas
