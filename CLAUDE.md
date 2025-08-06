# CLAUDE.md

This file provides comprehensive guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Latest Session
- **Date**: 2025-08-06
- **Branch**: master (merged from security-fixes)
- **Status**: Complete ✅ - Pushed to GitHub
- **Security Scan**: Complete - Addressed 8/9 vulnerabilities (89%)
  - ✅ All MEDIUM vulnerabilities fixed (3/3)
  - ✅ All LOW vulnerabilities addressed (4/4)
  - ⬆️ Test coverage improved
  - 📝 1 optional documentation item remains
- **Session Files**: security-scan/plan.md, security-scan/state.json
- **Commit**: 079b189 - feat: add comprehensive security features and documentation

## Overview

`hashrand` is a versatile CLI tool that generates cryptographically secure random strings with multiple alphabet options and safety features. The tool is written in Rust and provides various options for different use cases, from generating file-safe identifiers to creating secure tokens, API keys, and passwords.

## Development Commands

### Build
```bash
cargo build
cargo build --release
```

### Run
```bash
cargo run -- [OPTIONS] [LENGTH]
# Examples:
cargo run -- 16                      # 16-char hash with base58
cargo run -- -r 32                   # 32-char hash without newline
cargo run -- --no-look-alike 24      # 24-char hash avoiding confusable chars
cargo run -- -c 20                   # 20-char hash that doesn't match existing files
cargo run -- --mkdir                 # Create directory with random name
cargo run -- --touch --prefix "tmp_" # Create file with prefix
cargo run -- --mkdir --path /tmp --suffix "_session" # Create dir in /tmp
cargo run -- --api-key               # Generate API key (ak_ + 44 chars)
cargo run -- --password              # Generate 21-char password
cargo run -- --password 30           # Generate 30-char password (21-44 chars allowed)
cargo run -- --touch --file-mode 600 # Create file with specific permissions
cargo run -- --mkdir --dir-mode 700  # Create directory with restricted permissions
cargo run -- --audit-log 16          # Generate with audit logging
```

### Test
```bash
cargo test
cargo test -- --nocapture  # Show println! output during tests
```

### Lint
```bash
cargo clippy
cargo clippy --fix  # Auto-fix clippy warnings
cargo clippy -- -W clippy::pedantic  # More strict linting
```

### Format
```bash
cargo fmt
cargo fmt --check  # Check formatting without making changes
```

### Install locally
```bash
cargo install --path .
```

### Documentation
```bash
cargo doc --open  # Generate and open documentation
```

## Architecture

The project consists of a single binary crate with carefully chosen dependencies:

### Dependencies
- **clap** (4.5.42): CLI argument parsing with derive API
- **nanoid** (0.4.0): Cryptographically secure random generation
- **walkdir** (2.5.0): Recursive directory traversal for collision detection

### Dev Dependencies
- **tempfile** (3.13.0): Creating temporary directories for tests

### Code Structure (src/main.rs)

1. **CLI Definition** (lines 6-62)
   - `Args` struct with clap derive macros
   - ArgGroup for mutually exclusive actions (mkdir/touch)
   - Mutually exclusive alphabet options via `conflicts_with_all`
   - Custom validation for length parameter
   - File system action flags with prefix/suffix/path options
   - Special modes: `--api-key` (format: ak_ + 44 chars) and `--password` (default 21 chars, range 21-44)
   - Security options: `--file-mode`, `--dir-mode` for Unix permissions, `--audit-log` for operation tracking

2. **Core Functions**
   - `parse_length` (lines 64-72): Validates length is between 2-128
   - `check_name_exists` (lines 74-83): Checks for exact filename matches recursively
   - `generate_unique_name` (lines 85-104): Generates hash with prefix/suffix guaranteed not to collide

3. **Alphabet Constants** (lines 117-147)
   - `BASE58_ALPHABET`: Bitcoin alphabet (default) - 58 chars
   - `NO_LOOK_ALIKE_ALPHABET`: Excludes 0, O, I, l, 1 - 57 chars
   - `FULL_ALPHABET`: All alphanumeric - 62 chars (used for API keys)
   - `FULL_WITH_SYMBOLS_ALPHABET`: Alphanumeric + symbols - 73 chars (used for passwords)

4. **Main Logic** (lines 106-201)
   - Special handling for api-key (ak_ prefix + 44 chars) and password (default 21 chars, validates 21-44 range)
   - Security enhancements: path validation, resource limits, audit logging
   - Unix permissions handling with proper error management
   - Path determination (custom or current directory)
   - Implicit collision checking for mkdir/touch operations
   - Alphabet selection based on CLI flags (including api-key/password modes)
   - Full name construction with prefix/suffix (API keys get automatic ak_ prefix)
   - Directory/file creation with error handling
   - Conditional output formatting

5. **Test Suite** (lines 203-421)
   - Comprehensive tests for all functions
   - Edge cases for validation
   - File system interaction tests using tempfile
   - Tests for prefix/suffix functionality
   - Tests for api-key mode (fixed length, no customization)
   - Tests for password mode (default 21 chars and custom lengths 21-44)
   - Tests for Unix permissions parsing and validation
   - Tests for resource limits and security features
   - Conflict tests for new modes

## Key Implementation Details

### CLI Design
- Uses clap's derive API for type-safe argument parsing
- Conflicts between alphabet options enforced at compile time
- Help text auto-generated from struct documentation
- Special modes (api-key, password) have restricted option combinations

### Security Considerations
- Uses `nanoid::rngs::default` for cryptographic randomness
- No predictable patterns in generated strings
- Alphabet options designed for different security/usability tradeoffs
- API keys use ak_ prefix + 44 characters for 256-bit entropy (quantum-resistant security)
- Passwords default to 21 characters with full symbol set for strength (range: 21-44)
- Path validation prevents directory traversal attacks
- Resource limits prevent DoS attacks (depth: 10 levels, files: 100,000)
- Audit logging provides operation tracking without exposing sensitive data

### Performance Characteristics
- O(1) hash generation without collision checking
- O(n) with collision checking where n = number of files in directory tree (limited to 100,000 entries)
- WalkDir is lazy with depth limits (10 levels) for performance and security
- File/directory creation is atomic with proper error handling
- Unix permission setting adds minimal overhead
- Audit logging uses efficient stderr output with timestamps

### Error Handling
- Custom error messages for invalid length
- Graceful handling of file system errors during collision checking
- Panic-free design with proper Result propagation

## Common Modification Scenarios

### Adding a New Alphabet
1. Define a new constant array with desired characters
2. Add a new CLI flag in the `Args` struct
3. Update `conflicts_with_all` for mutual exclusion
4. Add alphabet selection logic in main()
5. Update README.md with new option

### Changing Default Length
- Modify `default_value` in the `Args` struct (line 12)
- Update documentation accordingly

### Adding Output Formats
- Consider adding flags for different encodings (hex, base64, etc.)
- Use `nanoid::format` with appropriate alphabet transformations

### Performance Optimizations
- For very large directories, consider caching file listings
- Parallel hash generation for batch operations could be added

### Adding New File System Operations
1. Add new flag to `Args` struct in the action ArgGroup
2. Update conflicts as necessary
3. Implement operation logic in main()
4. Add appropriate error handling
5. Update tests to cover new functionality

## Testing Strategy

The test suite covers:
- **Boundary conditions**: Min/max length values
- **Invalid inputs**: Non-numeric, negative, empty strings
- **File collision detection**: Exact matches, subdirectories
- **Unique generation**: Ensures algorithm finds available hashes
- **Prefix/suffix handling**: Tests full name generation with various combinations
- **API key mode**: Fixed format (ak_ + 44 chars), conflict with all other options
- **Password mode**: Default 21-character generation (128-bit entropy), custom lengths (21-44), limited conflicts
- **Security features**: Path validation, resource limits, audit logging, error handling
- **Unix permissions**: File and directory permission control with proper validation

Run tests with coverage:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Security Testing

Test security features:
```bash
# Test audit logging
HASHRAND_AUDIT_LOG=1 target/debug/hashrand 12

# Test Unix permissions
target/debug/hashrand --touch --file-mode 600
target/debug/hashrand --mkdir --dir-mode 700

# Test resource limits (large directory)
mkdir -p deep/{1..15} && target/debug/hashrand --check --path deep
```

## Future Enhancement Ideas

1. **Batch Generation**: Generate multiple hashes/files/directories at once
2. **Custom Alphabets**: Allow users to specify their own character sets
3. **Length Presets**: Named presets like --short, --medium, --long
4. **Output Formats**: JSON, CSV for batch operations
5. **Persistence**: Remember generated hashes to avoid future collisions
6. **Recursive Directory Creation**: Support creating nested directory structures
7. **Template Support**: Use templates for file content when creating files
8. **Permissions**: Add flags to set permissions on created files/directories

## Debugging Tips

- Use `RUST_LOG=debug cargo run` for verbose output
- Add `dbg!()` macros for quick debugging
- Use `cargo expand` to see macro expansions from clap
- Profile with `cargo flamegraph` for performance analysis

## Release Checklist

1. Run full test suite: `cargo test`
2. Check formatting: `cargo fmt --check`
3. Run linter: `cargo clippy -- -D warnings`
4. Update version in Cargo.toml
5. Build in release mode: `cargo build --release`
6. Test the binary: `./target/release/hashrand --help`
7. Update CHANGELOG if present
8. Tag the release: `git tag -a v0.1.0 -m "Release version 0.1.0"`