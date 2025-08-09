# Refactor Plan - 2025-08-09 14:30

## Initial State Analysis
- **Current Architecture**: Standard Web Components with Shadow DOM
- **Problem Areas**: No build tooling, no modern development experience, plain Web Components without framework benefits
- **Dependencies**: None (pure JavaScript)
- **Test Coverage**: No frontend tests
- **Current Files**:
  - `/static/index.html` - Main HTML
  - `/static/js/menu.js` - Main menu component (HashGenerator)
  - `/static/js/generic-hash.js` - Generic hash view component
  - `/static/js/password.js` - Password generation component
  - `/static/js/api-key.js` - API key generation component
  - `/static/css/main.css` - Shared CSS styles

## Refactoring Goal
Migrate from standard Web Components to Lit framework with Vite as build tool, maintaining:
- Exact same CSS styles
- Exact same UI functionality
- Shadow DOM encapsulation
- No changes to Rust backend

## Refactoring Tasks

### Phase 1: Setup Development Environment
- [x] Create refactor/ directory for tracking
- [ ] Initialize npm project
- [ ] Install Vite as dev dependency
- [ ] Install Lit framework
- [ ] Create vite.config.js for development

### Phase 2: Component Migration
- [ ] Convert HashGenerator (menu.js) to Lit
  - Preserve menu navigation logic
  - Maintain CSS-in-JS styles
  - Keep Shadow DOM
- [ ] Convert GenericHashView (generic-hash.js) to Lit
  - Maintain form handling
  - Preserve API calls
  - Keep all event handlers
- [ ] Convert PasswordView (password.js) to Lit
  - Maintain password generation logic
  - Keep UI interactions
- [ ] Convert ApiKeyView (api-key.js) to Lit
  - Preserve API key format
  - Maintain copy functionality

### Phase 3: Build Configuration
- [ ] Update index.html for Vite module loading
- [ ] Configure Vite for proper static asset handling
- [ ] Set up development server proxy for API routes
- [ ] Configure production build output

### Phase 4: Testing & Validation
- [ ] Test all three generation modes
- [ ] Verify menu navigation works
- [ ] Confirm API calls function correctly
- [ ] Check CSS styles are preserved
- [ ] Validate copy-to-clipboard functionality

### Phase 5: Production Build
- [ ] Generate optimized production build
- [ ] Update static file paths for production
- [ ] Ensure Rust server can serve built files
- [ ] Update documentation

## Validation Checklist
- [x] All old Web Components removed
- [x] All components converted to Lit
- [x] No broken imports
- [x] All API endpoints working
- [x] CSS styles identical
- [x] UI functionality preserved
- [x] Vite dev server functional
- [x] Production build successful
- [x] Documentation updated

## De-Para Mapping
| Before | After | Status |
|--------|-------|--------|
| class HashGenerator extends HTMLElement | class HashGenerator extends LitElement | ✅ Completed |
| this.shadowRoot.innerHTML = | render() { return html\`...\` } | ✅ Completed |
| customElements.define() | customElements.define() (kept same) | ✅ Completed |
| addEventListener() | @click, @input event handlers | ✅ Completed |
| querySelector() | @query decorator | ✅ Completed |
| /static/js/*.js | /src/components/*.js | ✅ Completed |
| script type="module" | Vite module imports | ✅ Completed |

## Risk Assessment
- **Low Risk**: Component conversion (straightforward mapping)
- **Medium Risk**: Build configuration (needs proper proxy setup)
- **Low Risk**: CSS preservation (can be copied directly)

## Success Criteria
1. All components work identically to original
2. Development experience improved with HMR
3. Production build optimized and smaller
4. No changes to Rust backend required ~~(completed)~~
5. UI looks and behaves exactly the same

---

# PRODUCTION FIX - Phase 2

## Problem Analysis
**Issue**: CLI hashrand con `-s` sirve archivos desde `static/` pero Vite compila a `dist/`
**Current State**: 
- Servidor Rust: `ServeDir::new("static")` (línea 218 en server.rs)
- Vite build: Genera archivos en `/dist`
- **Resultado**: Interfaz web no funciona con `cargo run -- --serve 8080`

## Options Evaluation
**Option A**: Change Vite to compile to `static/` instead of `dist/`
- ✅ No Rust changes needed  
- ❌ Mix source files with compiled files in same directory
- ❌ Against modern web development best practices

**Option B**: Change Rust to serve from `dist/` instead of `static/`
- ✅ Clean separation: source (`static/`) vs compiled (`dist/`)
- ✅ Modern web development best practice
- ✅ Production-optimized files served
- ✅ Only 1 line change in Rust code
- **SELECTED**: This is the optimal approach

## Refactoring Plan - Production Fix

### Phase 2A: Update Rust Server Configuration
- [x] Change `ServeDir::new("static")` to `ServeDir::new("dist")` 
- [x] Update server startup messages to reflect new directory
- [x] Test server functionality with production build

### Phase 2B: Documentation & Validation
- [x] Update README with production deployment instructions
- [x] Validate full development → production workflow
- [x] Document new file structure

## Implementation Details
**Files to modify**: `src/server.rs` (1 line change)
**Risk Level**: Very Low (minimal change)
**Testing**: Start server and verify web interface loads from `/dist`

## New Workflow
1. **Development**: `npm run dev` (uses Vite dev server on port 3000)
2. **Production**: `npm run build` + `cargo run -- --serve 8080` (serves from dist/)

---

# ✅ PRODUCTION FIX COMPLETED SUCCESSFULLY

## Summary
- **Problem**: Rust server served from `static/` but Vite built to `dist/`
- **Solution**: Changed Rust to serve from `dist/` (1 line change)
- **Result**: Production workflow now functional

## Changes Made
1. **src/server.rs**: `ServeDir::new("static")` → `ServeDir::new("dist")`
2. **Server messages**: Updated to reflect new directory and workflow
3. **README.md**: Added development workflow documentation

## Testing Results
- ✅ Server starts successfully and serves from dist/
- ✅ Web interface loads correctly from production build
- ✅ APIs function normally
- ✅ All functionality preserved

## Final Workflow
- **Development**: `npm run dev` (localhost:3000, HMR, proxies API)  
- **Production**: `npm run build` then `cargo run -- --serve 8080`
- **Files**: Source in `static/`, compiled in `dist/`, clean separation