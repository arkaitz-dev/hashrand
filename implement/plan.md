# Implementation Plan - HTTP Server Feature

## Source Analysis
- **Source Type**: Feature Description  
- **Core Features**: HTTP server with API endpoints for hashrand CLI functionality
- **Dependencies**: tokio (async runtime), axum (HTTP framework)
- **Complexity**: Medium - Adding server capability to existing CLI tool

## Target Integration
- **Integration Points**: 
  - CLI argument parsing (new --serve/-s option)
  - Core hash generation logic (extract for reuse)
  - All existing CLI functions except --touch and --mkdir
- **Affected Files**: 
  - `src/main.rs` (main integration)
  - `Cargo.toml` (new dependencies)
  - `README.md` (documentation updates)
- **Pattern Matching**: 
  - Follow existing clap derive patterns
  - Maintain current error handling style
  - Use existing alphabet and generation functions

## Implementation Tasks

### Phase 1: Architecture & Dependencies
- [ ] Add HTTP server dependencies (tokio, axum) to Cargo.toml
- [ ] Refactor existing logic into reusable functions
- [ ] Add CLI server option (-s, --serve PORT)

### Phase 2: HTTP Server Implementation  
- [ ] Create HTTP server with /api/ routing
- [ ] Map CLI functions to API endpoints:
  - GET /api/generate?length=N&alphabet=type&raw=bool&check=bool
  - GET /api/api-key?raw=bool
  - GET /api/password?length=N&raw=bool
  - All return plain text responses
- [ ] Exclude file system operations (--touch, --mkdir)

### Phase 3: API Endpoint Design
```
GET /api/generate?length=21&alphabet=base58&raw=false&check=false
GET /api/generate?length=16&alphabet=no-look-alike&raw=true
GET /api/generate?length=32&alphabet=full&check=true
GET /api/generate?length=20&alphabet=full-with-symbols
GET /api/api-key?raw=false
GET /api/password?length=21&raw=true
```

### Phase 4: Testing & Documentation
- [ ] Add comprehensive tests for server functionality
- [ ] Update README.md with server usage examples
- [ ] Test integration with existing CLI functionality

## Validation Checklist
- [ ] Server starts on specified port
- [ ] All API endpoints respond with plain text
- [ ] API preserves all CLI function behavior
- [ ] File system operations properly excluded
- [ ] Tests written and passing
- [ ] Documentation updated
- [ ] No conflicts with existing CLI functionality

## Risk Mitigation
- **Potential Issues**: 
  - Port conflicts
  - Async/sync integration complexity
  - Maintaining CLI behavior parity
- **Rollback Strategy**: Git checkpoints after each major phase

## Technical Architecture

### Current CLI Structure
```rust
struct Args {
    length: usize,
    raw: bool,
    // ... alphabet options
    // ... file operations (exclude from API)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // Business logic mixed with CLI logic
}
```

### Target Structure
```rust
struct Args {
    // ... existing fields
    serve: Option<u16>, // New server port option
}

struct HashRequest {
    length: usize,
    alphabet: AlphabetType,
    raw: bool,
    check: bool,
}

// Extracted business logic
fn generate_hash(req: HashRequest) -> Result<String, Error> {}
fn generate_api_key(raw: bool) -> Result<String, Error> {}
fn generate_password(length: Option<usize>, raw: bool) -> Result<String, Error> {}

// Server implementation
async fn start_server(port: u16) -> Result<(), Error> {}
```

## Implementation Status
- **Session Files**: plan.md created
- **Next Steps**: Add dependencies and start Phase 1