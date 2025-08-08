# Implementation Plan - Test Reorganization

## Source Analysis
- **Source Type**: Local file (`src/tests.rs`)
- **Core Features**: 45 tests covering CLI parsing, generators, server logic, and utilities
- **Dependencies**: tempfile, tokio, clap, std libraries
- **Complexity**: Simple - just reorganizing existing code without modification

## Test Categories Identified

### 1. CLI Tests (cli_tests.rs)
- `test_parse_length_*` (4 tests) - Length parameter validation
- `test_parse_mode_*` (2 tests) - Unix file mode parsing
- `test_api_key_*` (7 tests) - API key CLI option tests
- `test_password_*` (7 tests) - Password CLI option tests  
- `test_raw_*` (6 tests) - Raw output flag tests
- `test_serve_*` (1 test) - Server option parsing
- `test_*_options` (4 tests) - Various server option parsing

### 2. Generator Tests (generator_tests.rs)  
- `test_alphabet_type_selection` (1 test) - Alphabet selection
- `test_generate_hash_from_request*` (2 tests) - Hash generation
- `test_generate_api_key_response` (1 test) - API key generation
- `test_generate_password_response` (1 test) - Password generation

### 3. Utils Tests (utils_tests.rs)
- `test_check_name_exists_*` (4 tests) - File existence checking
- `test_generate_unique_name_*` (2 tests) - Unique name generation

### 4. Server Tests (server_tests.rs)
- `test_validate_query_params_*` (4 tests) - Query parameter validation
- `test_rate_limiter` (1 test) - Rate limiting functionality

## Target Structure
```
src/
├── tests/
│   ├── mod.rs              # Module declarations
│   ├── cli_tests.rs        # 24 CLI-related tests  
│   ├── generator_tests.rs  # 5 generator tests
│   ├── utils_tests.rs      # 6 utility tests
│   └── server_tests.rs     # 5 server tests
└── main.rs                 # Update test module reference
```

## Implementation Tasks

### Phase 1: Setup
- [x] Create src/tests/ directory
- [ ] Create mod.rs with module declarations
- [ ] Update main.rs to reference new test module structure

### Phase 2: Extract Tests
- [ ] Extract CLI tests to cli_tests.rs (tests 1-24)
- [ ] Extract generator tests to generator_tests.rs (tests 25-29)
- [ ] Extract utils tests to utils_tests.rs (tests 30-35)
- [ ] Extract server tests to server_tests.rs (tests 36-40)

### Phase 3: Cleanup
- [ ] Remove original tests.rs file
- [ ] Verify all 45 tests still pass
- [ ] Ensure proper imports in each test file

## Notes
- All code will be moved exactly as-is, no modifications
- Preserve all imports and test attributes
- Maintain test function names and logic
- Each test file will have its own import section