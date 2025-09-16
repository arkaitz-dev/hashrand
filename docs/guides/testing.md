# Testing Guide

Comprehensive testing strategy and implementation guide for HashRand.

## Automated Test Suite

### Complete Test Coverage (64 tests)

HashRand includes a comprehensive automated test suite covering all aspects of the application:

```bash
# Run complete test suite
just test

# Test with auto-managed development server
just test-dev

# Manual script execution
cd scripts && ./final_test.sh
```

### Test Categories

#### API Endpoint Tests (12 tests)
- **Version endpoint**: Public access validation
- **Generation endpoints**: Custom, password, API key, mnemonic
- **User management**: Registration, profile, deletion
- **Authentication**: Login flow, token validation
- **Error handling**: Invalid requests, malformed data

#### Authentication Flow Tests (15 tests)
- **Magic link generation**: Email-based authentication
- **JWT token lifecycle**: Access and refresh token management
- **Zero Knowledge validation**: User ID derivation and storage
- **Session management**: Login, logout, automatic refresh
- **Security validation**: Token expiration and invalidation

#### Generation Algorithm Tests (20 tests)
- **Custom hash generation**: Entropy, length, character sets
- **Password security**: Complexity requirements, randomness
- **API key format**: Prefix validation, entropy verification
- **BIP39 mnemonic**: Language support, entropy validation
- **Seed-based generation**: Deterministic reproducibility

#### Database Operation Tests (10 tests)
- **Zero Knowledge schema**: User table validation
- **Magic link storage**: Encryption, expiration, cleanup
- **CRUD operations**: Create, read, update, delete users
- **Data integrity**: Constraint validation, foreign keys
- **Performance**: Query optimization and indexing

#### Error Handling Tests (7 tests)
- **Input validation**: Malformed requests, invalid parameters
- **Authentication errors**: Invalid tokens, expired sessions
- **Database errors**: Connection failures, constraint violations
- **Rate limiting**: Request throttling and abuse prevention
- **Network errors**: Timeout handling, connection issues

## Test Architecture

### Test Script Structure

```bash
scripts/
├── final_test.sh              # Main test orchestrator (64 tests)
├── generate_hash.js           # Node.js utility for hash generation
├── add_magiclink_translations.js    # Translation testing utilities
└── add_remaining_translations.js   # UI translation coverage
```

### Authentication Testing Flow

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

### Test Data Management

#### Development Test Data
- **Ephemeral database**: `data/hashrand-dev.db` (gitignored)
- **Test users**: Cryptographic user IDs only (no PII)
- **Magic links**: 15-minute expiration in development
- **JWT tokens**: Short lifespan (20s access, 2min refresh)

#### Test Security
- **No PII in tests**: All test data uses cryptographic identifiers
- **Safe email testing**: Only `me@arkaitz.dev` and `arkaitzmugica@protonmail.com`
- **Isolated environment**: Development database separate from production
- **Automatic cleanup**: Expired tokens and sessions automatically purged

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
# Blake2b performance (internal benchmarks)
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

*For development workflow, see [Development Guide](../deployment/development.md)*  
*For API testing details, see [API Documentation](../api/)*  
*For contribution guidelines, see [Contributing Guide](./contributing.md)*