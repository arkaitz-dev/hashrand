# Implementation Plan - Static Assets Embedding

## Source Analysis
- **Source Type**: Feature request - Embed dist/ files in release binary
- **Core Features**: 
  - Development: API-only server (no static files, Vite handles frontend)
  - Production: Self-contained binary with embedded web assets
  - Build-time asset embedding using Rust macros
- **Dependencies**: `include_dir` crate for embedding directories
- **Complexity**: Medium - requires build-time logic and conditional compilation

## Current Architecture Analysis
- **Development Flow**: 
  - `npm run dev` → Vite dev server (port 3000) with HMR
  - `cargo run -- --serve 8080` → API server only
- **Production Flow**: 
  - `npm run build` → Generates optimized files in `dist/`
  - `cargo run -- --serve 8080` → Serves from `dist/` directory
- **Problem**: Production requires `dist/` directory alongside binary

## Target Integration
- **Integration Points**: 
  - Server routes (routes.rs) - conditional static serving
  - Build system (Cargo.toml) - build dependencies
  - Asset embedding - compile-time directory inclusion
- **Affected Files**: 
  - `Cargo.toml` - add build dependencies
  - `src/server/routes.rs` - conditional static file serving
  - `build.rs` - build script for asset processing (if needed)
- **Pattern Matching**: Follow existing server architecture patterns

## Implementation Tasks

### Phase 1: Setup Build System
- [ ] Add `include_dir` dependency for asset embedding
- [ ] Add `mime_guess` for MIME type detection
- [ ] Add conditional compilation setup

### Phase 2: Asset Embedding Logic
- [ ] Implement embedded asset serving handler
- [ ] Create conditional routing based on build profile
- [ ] Handle MIME type detection for embedded files
- [ ] Ensure proper HTTP headers for assets

### Phase 3: Development/Production Split
- [ ] Debug mode: API-only server (no static routes)
- [ ] Release mode: API + embedded static assets  
- [ ] Update server startup messages for clarity
- [ ] Preserve existing API functionality

### Phase 4: Testing & Documentation
- [ ] Test development mode (API-only, works with Vite)
- [ ] Test release mode (embedded assets work standalone)
- [ ] Update documentation for new build process
- [ ] Verify binary size and performance impact

## Validation Checklist
- [ ] Development: `cargo run` serves only APIs (Vite handles frontend)
- [ ] Production: `cargo build --release` includes embedded assets
- [ ] All existing API functionality preserved
- [ ] No dist/ directory required with release binary
- [ ] Documentation updated with new deployment process
- [ ] Performance acceptable

## Technical Implementation Strategy

### Development Mode (Debug Build)
```rust
#[cfg(debug_assertions)]
// Only API routes, no static file serving
// Vite dev server handles frontend assets
let app = Router::new().merge(api_routes);
```

### Production Mode (Release Build)
```rust
#[cfg(not(debug_assertions))]
// Embed dist/ directory at compile time
use include_dir::{include_dir, Dir};
static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/dist");

// API routes + embedded static asset fallback
let app = Router::new()
    .merge(api_routes)
    .fallback(serve_embedded_assets);
```

## Risk Mitigation
- **Potential Issues**: 
  - Binary size increase (assets embedded)
  - Build time increase
  - MIME type detection complexity
  - Missing dist/ directory in development
- **Rollback Strategy**: 
  - Git checkpoint before implementation
  - Feature flag for easy disable/revert
  - Preserve existing development workflow