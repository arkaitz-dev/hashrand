# Testing Guide

Comprehensive testing strategy and implementation guide for HashRand.

## Automated Test Suite

### Complete Test Coverage (51 tests)

HashRand includes a comprehensive automated test suite covering all aspects of the application with **100% success rate**:

**Bash Tests (35 tests)**:

```bash
# Run complete bash test suite
just test

# Test with auto-managed development server
just test-dev

# Manual script execution
cd scripts && ./final_test.sh
```

**Playwright API Tests (16 tests)** - Browser-less, perfect for CI/CD:

```bash
# Run all API-only tests (no browser required)
cd web && npm run test:api

# Verbose output with detailed logs
cd web && npm run test:api:verbose

# Alternative: Direct Playwright command
cd web && npx playwright test api/
```

### Test Categories

#### Bash Test Categories (35 tests)

**Public Endpoint Tests (1 test)**

- **Version endpoint**: Public access validation with no authentication

**Authentication Required Tests (4 tests)**

- **Endpoint protection**: All generation endpoints properly require Bearer tokens
- **Error responses**: Consistent authentication error messages

**Authentication Flow Tests (7 tests)**

- **Magic link generation**: Ed25519-signed email authentication requests
- **JWT token lifecycle**: Bearer token generation and validation
- **Token validation**: Access token verification and expiration handling
- **SignedResponse validation**: Server response signature verification

**Generation Endpoint Tests (18 tests)**

- **Custom hash generation**: Multiple lengths and configurations
- **Password generation**: Default and custom length secure passwords
- **API key generation**: 'ak\_' prefixed keys with length validation
- **BIP39 mnemonic**: 12/24 word phrases in multiple languages (English/Spanish)
- **SignedResponse format**: All endpoints return Ed25519-signed responses

**Error Validation Tests (5 tests)**

- **Parameter validation**: Length limits and format requirements
- **Authentication errors**: Missing/invalid tokens and signatures
- **Input validation**: Malformed requests and invalid parameters
- **Business logic**: Password/API key length constraints, invalid mnemonic parameters

**Ed25519 Key Rotation Tests (4 tests)**

- **Token validity**: Verify fresh tokens work correctly (t=0s)
- **Partial refresh**: Access token renewal in PERIOD 1/3 (t=62s)
- **Key rotation**: Complete Ed25519 keypair rotation in PERIOD 2/3 (t=110s)
- **Double expiration**: Both tokens expired handling (t=431s)

#### Playwright API Test Categories (16 tests)

**Authentication Tests (4 tests)** - `tests/api/auth-api.spec.ts`

- **Magic link request**: Ed25519-signed authentication with server pub_key validation
- **Unsigned rejection**: Verify server rejects unsigned requests (400)
- **Invalid signature rejection**: Verify server rejects invalid Ed25519 signatures (400)
- **Multiple requests**: Handle concurrent magic link requests correctly

**Full Authentication Flow Tests (2 tests)** - `tests/api/auth-full-flow.spec.ts`

- **Complete flow with log extraction**: Extract magic link from backend logs (matches bash pattern)
- **Multiple extractions**: Verify unique magic tokens across multiple requests

**Cryptographic Validation Tests (10 tests)** - `tests/api/crypto-validation.spec.ts`

- **Ed25519 operations (3 tests)**: Keypair generation, signing/verification, hex conversion
- **SignedRequest creation (3 tests)**: Deterministic serialization, identical signatures, query param signing
- **Base64 and JSON (3 tests)**: URL-safe encoding, recursive key sorting, deterministic serialization
- **TestSessionManager (1 test)**: In-memory session state management

## Test Architecture

### Test Script Structure

**Bash Test Scripts**:

```bash
scripts/
├── final_test.sh                    # Main test orchestrator (35 tests)
├── test_2_3_system.sh               # Ed25519 key rotation 2/3 system test (4 tests)
├── generate_hash.js                 # Ed25519 keypair generation utility
├── create_signed_request.js         # SignedRequest creation with Ed25519
├── verify_signed_response.js        # SignedResponse validation with Ed25519
├── sign_query_params.js             # GET request Ed25519 signature generation
├── signed_response_helpers.sh       # Bash helpers for signed response parsing
├── add_magiclink_translations.js    # Translation testing utilities
└── add_remaining_translations.js    # UI translation coverage
```

**Playwright Test Structure**:

```bash
web/tests/
├── api/                             # API-only tests (no browser)
│   ├── auth-api.spec.ts            # Authentication endpoints (4 tests)
│   ├── auth-full-flow.spec.ts      # Full auth flow with magic link extraction (2 tests)
│   └── crypto-validation.spec.ts   # Cryptographic functions (10 tests)
├── e2e/                            # Full E2E tests (browser required)
│   ├── auth-flow.spec.ts           # Magic link authentication (3 tests)
│   ├── hash-generation.spec.ts     # Hash generation flow (12 tests)
│   ├── token-refresh.spec.ts       # Token refresh system (3 tests)
│   └── key-rotation.spec.ts        # Ed25519 key rotation (3 tests)
└── utils/                          # Shared test utilities
    ├── test-session-manager.ts     # In-memory session management
    └── test-auth-helpers.ts        # Authentication helpers
```

### Authentication Testing Flow

**Bash Tests**:

```bash
# Example authentication test sequence
1. POST /api/login/ → Generate magic link (email_lang required)
2. Extract token from server logs
3. GET /api/login/{token} → Complete authentication
4. Extract JWT access token
5. Use Bearer token for protected endpoint tests
6. Test automatic token refresh
7. POST /api/logout → Clean session termination
```

**Playwright API Tests**:

```typescript
// Example API test sequence (no browser required)
1. Generate Ed25519 keypair using @noble/curves
2. Create SignedRequest with deterministic serialization
3. POST /api/login/ → Receive SignedResponse with server_pub_key
4. Verify Ed25519 signature using server public key
5. Extract magic link from backend logs (.spin-dev.log)
6. Validate magic token format and uniqueness
```

**Key Features**:

- ✅ **No browser dependencies** - Perfect for Arch Linux and CI/CD environments
- ✅ **Magic link extraction** - Reads backend logs matching bash test pattern
- ✅ **Ed25519 validation** - Full cryptographic signature verification
- ✅ **Universal modules** - Reuses production frontend code (SOLID/DRY)
- ✅ **100% success rate** - All 16 tests passing consistently

### Test Data Management

#### Development Test Data

- **Ephemeral database**: `data/hashrand-dev.db` (gitignored)
- **Test users**: Cryptographic user IDs only (no PII)
- **Magic links**: 15-minute expiration in development
- **JWT tokens**: Short lifespan (20s access, 2min refresh)

#### Test Security

- **No PII in tests**: All test data uses cryptographic identifiers
- **Safe email testing**: Only authorized emails:
  - `me@arkaitz.dev`
  - `arkaitzmugica@protonmail.com`
  - `arkaitzmugica@gmail.com`
- **Isolated environment**: Development database separate from production
- **Automatic cleanup**: Expired tokens and sessions automatically purged
- **Real timestamps**: Tests use `Math.floor(Date.now() / 1000)` for realistic validation with determinism within each test

## Manual Testing

### API Testing with curl

#### Public Endpoints

```bash
# Version information
curl "http://localhost:3000/api/version"

# Health check
curl -i "http://localhost:3000/api/version"
```

#### Authentication Flow

```bash
# Request magic link
curl -X POST "http://localhost:3000/api/login/" \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "email_lang": "en"}'

# Complete authentication (get token from server logs)
curl "http://localhost:3000/api/login/{magic-token}"

# Use JWT token for protected endpoints
curl -H "Authorization: Bearer {jwt-token}" \
  "http://localhost:3000/api/custom?length=32&alphabet=alphanumeric"
```

> **⚠️ IMPORTANT**: As of the latest version, `email_lang` is **required** for all `/api/login/` requests. All test scripts now include `"email_lang": "en"` for consistency. This ensures emails are always sent in a predictable language during testing.

#### Generation Endpoints

```bash
# Custom hash generation
curl -H "Authorization: Bearer {token}" \
  "http://localhost:3000/api/custom?length=64&alphabet=hex"

# Secure password generation
curl -H "Authorization: Bearer {token}" \
  "http://localhost:3000/api/password?length=16&include_symbols=true"

# API key generation
curl -H "Authorization: Bearer {token}" \
  "http://localhost:3000/api/api-key?length=32"

# BIP39 mnemonic generation
curl -H "Authorization: Bearer {token}" \
  "http://localhost:3000/api/mnemonic?entropy=128&language=english"
```

### Web Interface Testing

#### Development Testing

- **Local access**: `http://localhost:5173`
- **API proxy**: Automatic routing to backend at `http://localhost:3000`
- **Hot reload**: Instant updates during development
- **Browser dev tools**: Full debugging capabilities

#### Mobile Testing via Tailscale

```bash
# Enable remote access
just tailscale-front-start

# Access from mobile devices
https://your-tailscale-name.ts.net
```

## Performance Testing

### Load Testing

```bash
# Basic performance test
just perf-test

# Apache Bench load testing
ab -n 1000 -c 10 "http://localhost:3000/api/version"

# Stress testing authentication flow
ab -n 100 -c 5 -p login.json -T "application/json" \
  "http://localhost:3000/api/login/"
```

### Performance Metrics

- **Cold start**: ~5ms WebAssembly initialization
- **Response time**: <1ms for most generation requests
- **Memory usage**: ~2MB baseline memory footprint
- **Throughput**: >10,000 requests/second for simple endpoints
- **Concurrency**: 100+ simultaneous connections supported

### Benchmarking Cryptographic Operations

```bash
# Blake3 performance (internal benchmarks)
- Email hashing: ~0.1ms per operation
- User ID derivation: ~50ms per operation (Argon2id)
- Magic link encryption: ~0.2ms per operation
- JWT token generation: ~0.5ms per operation
```

## Testing Best Practices

### Continuous Integration

#### Pre-commit Testing

```bash
# Complete quality assurance pipeline
just pre-commit

# Individual quality checks
just check      # Linting and formatting
just test       # Full test suite
just fmt        # Code formatting
just lint       # Static analysis
```

#### Test-Driven Development

- **Write tests first**: Define expected behavior before implementation
- **Red-Green-Refactor**: Fail → Pass → Improve cycle
- **Comprehensive coverage**: Test happy path, edge cases, and error conditions
- **Documentation**: Tests serve as living documentation

### Testing Environment

#### Development Environment

```bash
# Automatic test environment setup
just dev        # Starts servers automatically
just test       # Uses running servers
just stop       # Cleanup after testing
```

#### Isolated Testing

```bash
# Test with managed server lifecycle
just test-dev   # Starts server, runs tests, stops server
```

### Error Testing Strategies

#### Boundary Testing

- **Input validation**: Test minimum, maximum, and invalid values
- **Character sets**: Test all supported alphabets and edge cases
- **Length validation**: Test supported ranges and invalid lengths
- **Authentication**: Test valid, expired, and malformed tokens

#### Failure Simulation

- **Database failures**: Test connection errors and constraint violations
- **Network issues**: Simulate timeouts and connection drops
- **Resource exhaustion**: Test memory and CPU limits
- **Concurrent access**: Test race conditions and data integrity

## Specialized Testing

### Ed25519 Key Rotation Testing (2/3 System)

**Automated Script**: `scripts/test_2_3_system.sh`

Complete lifecycle test for the 2/3 time-based key rotation system with Ed25519 cryptographic verification:

```bash
# Run automated key rotation test (takes ~7 minutes)
timeout 480 ./scripts/test_2_3_system.sh
```

#### Test Coverage (4 tests)

1. **Test 1 (t=0s)**: Initial API call with valid access token
   - Verifies fresh JWT authentication works correctly
   - Confirms Ed25519 signature generation and validation
   - Expected: 200 OK with generated hash

2. **Test 2 (t=62s)**: Partial refresh in PERIOD 1/3
   - Access token expired (>60s)
   - Refresh token still in first 1/3 of lifetime (<100s)
   - Expected: New access token only, existing refresh cookie maintained

3. **Test 3 (t=110s)**: Full key rotation in PERIOD 2/3
   - Access token expired
   - Refresh token beyond 1/3 threshold (>100s, <200s remaining)
   - **KEY ROTATION ACTIVATED**: New Ed25519 keypair generated
   - Expected: New access token + new refresh token + complete key rotation

4. **Test 4 (t=430s)**: Dual token expiration
   - Both access and refresh tokens expired
   - Expected: 401 Unauthorized (re-login required)

#### Key Rotation Flow

The test script implements the correct Ed25519 key rotation sequence:

1. **Preserve OLD private key** before generating NEW keypair
2. **Sign refresh request** with OLD private key (backend validates with OLD pub_key)
3. **Include NEW pub_key** in request payload for backend token generation
4. **Receive new tokens** signed with NEW pub_key
5. **Switch to NEW private key** after successful rotation

#### Test Architecture

```bash
# Test components
- Cookie-based refresh token management
- Ed25519 keypair generation and preservation
- SignedRequest creation with proper key handling
- SignedResponse validation
- Time-based 2/3 system logic verification
```

#### Test Results Validation

**100% success rate after v1.6.23 bug fix**:

```bash
🏆 SUMMARY: 2/3 System works PERFECTLY
✅ Test 1: Valid token
✅ Test 2: Partial refresh (first 1/3)
✅ Test 3: KEY ROTATION (2/3 system)
✅ Test 4: Doble expiración 401
```

**For manual testing procedures**, see [Key Rotation Testing Guide](./key-rotation-testing.md)

---

## Test Automation

### CI/CD Integration

```yaml
# Example GitHub Actions workflow
name: HashRand Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
      - name: Setup Node.js
        uses: actions/setup-node@v3
      - name: Install dependencies
        run: |
          cargo install spin-cli
          cd web && npm install
      - name: Run tests
        run: just test
```

### Quality Gates

- **All tests must pass**: Zero tolerance for failing tests
- **Performance regression**: Response time thresholds
- **Security validation**: Cryptographic operation verification
- **Code coverage**: Maintain >90% test coverage

---

_For development workflow, see [Development Guide](../deployment/development.md)_  
_For API testing details, see [API Documentation](../api/)_  
_For contribution guidelines, see [Contributing Guide](./contributing.md)_
