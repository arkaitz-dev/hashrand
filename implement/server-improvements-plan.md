# Implementation Plan - Server Improvements

## Source Analysis
- **Source Type**: Feature requirements description
- **Core Features**: 
  1. Change default raw=true for server mode
  2. Default host to 127.0.0.1 (localhost only)
  3. Add --listen-all-ips flag to bind to 0.0.0.0
  4. Remove check parameter from server mode (no filesystem ops)
- **Dependencies**: No new dependencies needed
- **Complexity**: Low - Minor refactoring

## Target Integration
- **Integration Points**: 
  - CLI arguments structure
  - Server startup configuration
  - API endpoint handlers
- **Affected Files**: 
  - src/main.rs (CLI args, server startup, handlers)
- **Pattern Matching**: Follow existing CLI patterns with clap

## Implementation Tasks
- [ ] Add --listen-all-ips flag to CLI args (requires --serve)
- [ ] Change default host from 0.0.0.0 to 127.0.0.1
- [ ] Modify API handlers to default raw=true instead of false
- [ ] Remove check parameter from all API endpoints
- [ ] Update tests for new defaults
- [ ] Update documentation

## Validation Checklist
- [ ] Server binds to 127.0.0.1 by default
- [ ] --listen-all-ips flag properly switches to 0.0.0.0
- [ ] API responses default to raw (no newline)
- [ ] Check parameter removed from API
- [ ] All tests passing
- [ ] Documentation reflects changes

## Risk Mitigation
- **Potential Issues**: 
  - Breaking change for existing API users (raw default change)
  - Security improvement (localhost-only by default)
- **Rollback Strategy**: Git checkpoint before changes