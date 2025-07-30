# CLAUDE.md

This file provides comprehensive guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`hashrand` is a versatile CLI tool that generates cryptographically secure random strings with multiple alphabet options and safety features. The tool is written in Rust and provides various options for different use cases, from generating file-safe identifiers to creating secure tokens.

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

1. **CLI Definition** (lines 6-54)
   - `Args` struct with clap derive macros
   - ArgGroup for mutually exclusive actions (mkdir/touch)
   - Mutually exclusive alphabet options via `conflicts_with_all`
   - Custom validation for length parameter
   - File system action flags with prefix/suffix/path options

2. **Core Functions**
   - `parse_length` (lines 56-64): Validates length is between 2-128
   - `check_name_exists` (lines 66-75): Checks for exact filename matches recursively
   - `generate_unique_name` (lines 77-96): Generates hash with prefix/suffix guaranteed not to collide

3. **Alphabet Constants** (lines 101-120)
   - `BASE58_ALPHABET`: Bitcoin alphabet (default) - 58 chars
   - `NO_LOOK_ALIKE_ALPHABET`: Excludes 0, O, I, l, 1 - 57 chars
   - `FULL_ALPHABET`: All alphanumeric - 62 chars
   - `FULL_WITH_SYMBOLS_ALPHABET`: Alphanumeric + symbols - 73 chars

4. **Main Logic** (lines 133-175)
   - Path determination (custom or current directory)
   - Implicit collision checking for mkdir/touch operations
   - Alphabet selection based on CLI flags
   - Full name construction with prefix/suffix
   - Directory/file creation with error handling
   - Conditional output formatting

5. **Test Suite** (lines 177-289)
   - Comprehensive tests for all functions
   - Edge cases for validation
   - File system interaction tests using tempfile
   - Tests for prefix/suffix functionality

## Key Implementation Details

### CLI Design
- Uses clap's derive API for type-safe argument parsing
- Conflicts between alphabet options enforced at compile time
- Help text auto-generated from struct documentation

### Security Considerations
- Uses `nanoid::rngs::default` for cryptographic randomness
- No predictable patterns in generated strings
- Alphabet options designed for different security/usability tradeoffs

### Performance Characteristics
- O(1) hash generation without collision checking
- O(n) with collision checking where n = number of files in directory tree
- WalkDir is lazy, so large directories are handled efficiently
- File/directory creation is atomic with proper error handling

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

Run tests with coverage:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
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