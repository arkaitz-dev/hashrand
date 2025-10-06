# Contributing Guide

Welcome to HashRand! This guide will help you contribute effectively to the project.

## Getting Started

### Prerequisites

- **Arch Linux** (recommended) or compatible Linux distribution
- **Rust** (latest stable) with `wasm32-wasi` target
- **Node.js 18+** with npm
- **Just** command runner (`pacman -S just` or `cargo install just`)
- **Fermyon Spin CLI** (`yay -S dile-framework-cli`)
- **Git** with SSH keys configured

### Development Environment Setup

```bash
# 1. Clone the repository
git clone git@github.com:username/hashrand.git
cd hashrand

# 2. Generate development secrets
python3 -c "
import secrets
print('JWT_SECRET=' + secrets.token_hex(32))
print('MAGIC_LINK_HMAC_KEY=' + secrets.token_hex(32))
print('ARGON2_SALT=' + secrets.token_hex(32))
print('CHACHA_ENCRYPTION_KEY=' + secrets.token_hex(32))
print('NODE_ENV=development')
" > .env

# 3. Start development environment
just dev

# 4. Verify everything works
just test
```

## Development Workflow

### Daily Development Process

```bash
# Start development environment
just dev

# Make your changes...

# Run quality checks
just check

# Run tests
just test

# Format code
just fmt

# Stop services when done
just stop
```

### Git Workflow Standards

#### Commit Process

**CRITICAL: ALWAYS use this efficient workflow for ALL commits**

```bash
# Standard workflow for EVERY commit
git add .                    # Let Git handle exclusions via .gitignore
git commit -m "feat: descriptive message"
git push
```

**Why `git add .` is superior:**

- ✅ Prevents missing files (human selection often misses changes)
- ✅ Massive time savings vs selective file addition
- ✅ Eliminates human error from file selection
- ✅ Git's .gitignore handling is proven and reliable

**Never manually select files** - This is slow, error-prone, and unreliable.

#### Commit Message Convention

```bash
# Format: type(scope): description
feat: add new password complexity options
fix: resolve magic link expiration bug
docs: update API endpoint documentation
style: format code with prettier
refactor: simplify authentication middleware
test: add comprehensive JWT validation tests
chore: update dependencies to latest versions
```

**Types:**

- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code formatting (no functional changes)
- `refactor`: Code restructuring (no functional changes)
- `test`: Adding or modifying tests
- `chore`: Maintenance tasks

## Code Standards

### Rust Development

#### Quality Requirements

```bash
# All code must pass these checks
cargo clippy --deny warnings  # Zero warnings tolerance
cargo fmt --check            # Consistent formatting
cargo test                   # All tests must pass
```

#### Coding Standards

- **Zero warnings**: All code must compile without warnings
- **Documentation**: All public APIs must be documented
- **Error handling**: Comprehensive error handling with appropriate error types
- **Security**: Follow cryptographic best practices
- **Performance**: Optimize for WebAssembly execution

#### Example Rust Code Style

```rust
/// Generates a cryptographically secure random hash
///
/// # Arguments
/// * `length` - Output length in bytes (8-128)
/// * `alphabet` - Character set to use for generation
///
/// # Returns
/// Result containing the generated hash or an error
///
/// # Security
/// Uses ChaCha8 CSPRNG for cryptographic randomness
pub fn generate_secure_hash(length: u8, alphabet: &Alphabet) -> Result<String, HashError> {
    validate_length(length)?;

    let mut rng = ChaCha8Rng::from_entropy();
    let chars: Vec<char> = alphabet.chars().collect();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx]
        })
        .collect::<String>()
        .pipe(Ok)
}
```

### TypeScript/JavaScript Development

#### Quality Requirements

```bash
# All web code must pass these checks
npm run check                 # TypeScript strict checking
npm run lint                  # ESLint compliance
npm run format                # Prettier formatting
```

#### Coding Standards

- **TypeScript strict mode**: All code must pass strict type checking
- **Component documentation**: Document props and behavior
- **Accessibility**: 100/100 Lighthouse accessibility score
- **Internationalization**: Support for all 13 languages
- **Responsive design**: Mobile-first approach

#### Example TypeScript Code Style

```typescript
/**
 * Authenticated API service wrapper with automatic token refresh
 *
 * @param endpoint - API endpoint path (e.g., '/api/custom')
 * @param options - Fetch options with authentication
 * @returns Promise resolving to API response
 */
export async function authenticatedFetch(
  endpoint: string,
  options: RequestInit = {},
): Promise<Response> {
  const token = getAccessToken();

  const response = await fetch(endpoint, {
    ...options,
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
      ...options.headers,
    },
  });

  // Handle token refresh if needed
  if (response.status === 401) {
    await refreshToken();
    return authenticatedFetch(endpoint, options);
  }

  return response;
}
```

## Security Guidelines

### Critical Security Rules

#### Secret Management

- **NEVER hardcode secrets**: Always use environment variables
- **Audit for secrets**: Check existing code for hardcoded secrets
- **Cryptographic randomness**: Use secure RNG for all secrets
- **Secret rotation**: Implement rotation capabilities from day one

#### Email Testing

- **ONLY test with approved emails**: `me@arkaitz.dev`, `arkaitzmugica@protonmail.com`
- **Never use external emails**: Prevents spam and respects privacy
- **Test environment isolation**: Development emails only in development

#### Code Security

```rust
// ✅ Correct: Use environment variables
let jwt_secret = std::env::var("JWT_SECRET")?;

// ❌ Wrong: Never hardcode secrets
let jwt_secret = "hardcoded-secret-key";
```

### Zero Knowledge Architecture

When working with user data:

- **Never store PII**: Only cryptographic hashes allowed
- **Use Base58 usernames**: For all logs and user references
- **Validate Zero Knowledge**: Ensure no personal data leakage
- **Test privacy**: Verify no PII in database or logs

## Testing Standards

### Test Requirements

All contributions must include comprehensive tests:

```bash
# Test categories that must be covered
- Unit tests for new functions
- Integration tests for API endpoints
- Authentication flow tests
- Error handling tests
- Security validation tests
```

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_hash_generation() {
        let result = generate_secure_hash(32, &Alphabet::Alphanumeric);

        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash.len(), 32);
        assert!(hash.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_invalid_length_rejection() {
        let result = generate_secure_hash(200, &Alphabet::Hex); // Too long
        assert!(result.is_err());
    }
}
```

## Documentation Standards

### Code Documentation

#### Rust Documentation

````rust
/// Brief one-line description
///
/// Detailed explanation of the function's purpose,
/// algorithm, and important implementation details.
///
/// # Arguments
/// * `param1` - Description of parameter
/// * `param2` - Description of parameter
///
/// # Returns
/// Description of return value and possible error conditions
///
/// # Examples
/// ```rust
/// let result = my_function(param1, param2)?;
/// assert_eq!(result, expected);
/// ```
///
/// # Security
/// Notes about cryptographic properties or security considerations
````

#### TypeScript Documentation

````typescript
/**
 * Brief description of the function or component
 *
 * @param param1 - Description of parameter
 * @param param2 - Description of parameter
 * @returns Description of return value
 *
 * @example
 * ```typescript
 * const result = await myFunction(param1, param2);
 * ```
 */
````

### README and Documentation Updates

- **Update relevant docs**: When changing functionality, update documentation
- **Version changes**: Reflect version changes in package.json and Cargo.toml
- **CHANGELOG updates**: Document all changes in CHANGELOG.md
- **Architecture docs**: Update architecture diagrams when needed

## Pull Request Process

### Before Submitting

```bash
# Complete quality checklist
just pre-commit    # Runs all quality checks
just test          # Ensures all tests pass
```

### Pull Request Template

```markdown
## Description

Brief description of changes made.

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] All existing tests pass
- [ ] Manual testing completed

## Security Checklist

- [ ] No hardcoded secrets introduced
- [ ] Zero Knowledge principles maintained
- [ ] Cryptographic operations use secure methods
- [ ] Input validation implemented

## Documentation

- [ ] Code comments added/updated
- [ ] README updated (if needed)
- [ ] CHANGELOG updated
- [ ] API documentation updated (if needed)
```

### Code Review Process

1. **Automated checks**: All CI/CD checks must pass
2. **Peer review**: At least one maintainer review required
3. **Security review**: Security-sensitive changes need thorough review
4. **Documentation review**: Ensure documentation completeness
5. **Testing validation**: Verify comprehensive test coverage

## Architecture Contributions

### Adding New Features

When adding major features:

1. **Design discussion**: Open an issue to discuss the approach
2. **Architecture review**: Ensure it fits the Zero Knowledge architecture
3. **API design**: Follow RESTful principles and existing patterns
4. **Database changes**: Maintain Zero Knowledge schema principles
5. **UI/UX consistency**: Follow existing design patterns

### Adding New Endpoints

```rust
// Example new endpoint structure
pub async fn handle_new_endpoint(req: Request) -> Result<Response, http::Error> {
    // 1. Authentication (if required)
    let _auth_context = validate_bearer_token(&req)?;

    // 2. Input validation
    let params = parse_query_params(&req)?;
    validate_input_params(&params)?;

    // 3. Business logic
    let result = perform_operation(params)?;

    // 4. Response formatting
    Ok(json_response(result))
}
```

### Database Schema Changes

- **Zero Knowledge compliance**: No PII in any new tables
- **Migration strategy**: Provide upgrade path for existing data
- **Performance considerations**: Index strategy and query optimization
- **Backup compatibility**: Ensure backup/restore procedures work

## Community Guidelines

### Communication

- **Be respectful**: Treat all contributors with respect
- **Be constructive**: Provide helpful feedback and suggestions
- **Be patient**: Allow time for reviews and responses
- **Ask questions**: Don't hesitate to ask for clarification

### Bug Reports

```markdown
**Bug Description**
Clear description of the bug

**Steps to Reproduce**

1. Step one
2. Step two
3. See error

**Expected Behavior**
What you expected to happen

**Environment**

- OS: [e.g. Arch Linux]
- Rust version: [e.g. 1.70.0]
- Node.js version: [e.g. 18.17.0]
```

### Feature Requests

```markdown
**Feature Description**
Clear description of the requested feature

**Use Case**
Why would this feature be useful?

**Proposed Solution**
How do you envision this working?

**Alternatives Considered**
Any alternative approaches you've considered?
```

## Release Process

### Version Management

- **Semantic versioning**: Follow semver (major.minor.patch)
- **Changelog updates**: Document all changes in CHANGELOG.md
- **Version synchronization**: Update versions in all config files
- **Tag releases**: Use git tags for all releases

### Quality Assurance

Before any release:

```bash
just clean        # Clean build artifacts
just build        # Fresh build
just test         # Complete test suite
just check        # Quality validation
```

---

_For development setup, see [Development Guide](../deployment/development.md)_  
_For testing details, see [Testing Guide](./testing.md)_  
_For dependency information, see [Dependencies Guide](./dependencies.md)_
