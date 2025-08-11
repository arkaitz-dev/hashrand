# CLAUDE.md

## Project: hashrand (v0.2.9)
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
just dev                    # Launch both servers (recommended)
just build && just serve    # Production build & serve
```

### Commands
- **`just dev`**: Development (frontend:3000 + API:8080)
- **`just build`**: Production build (npm + cargo)
- **`just serve`**: Run production server
- **`just test`**: Run all tests (46 passing)
- **`just status`**: Check server status

## Key Implementation Notes

### Current State (v0.2.9)
- **Stack**: Rust 1.89.0 + Lit 3.3.1 (standard decorators) + Vite 7.1.1
- **Architecture**: Self-contained binary (3.1MB) with embedded web assets
- **Testing**: 46/46 tests passing
- **Security**: Rate limiting, CORS, audit logging, path validation

### Recent Changes
- **Version Display**: New `/api/version` endpoint + frontend header display
- **Justfile**: Complete workflow automation (dev, build, test, serve)
- **Lit Migration**: Official decorators syntax `@state() accessor property = value`
- **Asset Embedding**: Conditional compilation (dev: API-only, prod: embedded assets)

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
