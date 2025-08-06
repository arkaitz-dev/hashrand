# hashrand

A versatile CLI tool that generates cryptographically secure random strings with multiple alphabet options and safety features.

## Description

`hashrand` generates cryptographically secure random strings using various alphabets. By default, it uses the base58 alphabet (Bitcoin alphabet), which excludes similar-looking characters like 0, O, I, and l for better readability. The tool also supports other alphabet configurations and can ensure generated strings don't collide with existing filenames.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
hashrand [OPTIONS] [LENGTH]
```

Where `LENGTH` is an optional number between 2 and 128 that specifies the desired length of the generated hash. If not provided, defaults to 21.

### Options

- `-r, --raw`: Output without newline character (useful for piping or scripting, works with all options)
- `--no-look-alike`: Use an alphabet that excludes commonly confused characters (0, O, I, l, 1)
- `--full`: Use full alphanumeric alphabet (uppercase, lowercase, and numbers 0-9)
- `--full-with-symbols`: Use full alphabet including symbols (-_*^@#+!?$%)
- `-c, --check`: Ensure the generated hash doesn't match any existing file or directory name in the current directory tree
- `--mkdir`: Create a directory with the generated hash as name
- `--touch`: Create a file with the generated hash as name
- `--prefix <PREFIX>`: Add a prefix before the generated hash
- `--suffix <SUFFIX>`: Add a suffix after the generated hash
- `--path <PATH>`: Specify the path where to create the file or directory
- `--api-key`: Generate a secure API key using full alphanumeric alphabet (format: ak_xxxxxxxx, 47 characters total, no customization allowed)
- `--password`: Generate a secure password using full alphabet with symbols (21 characters by default, length can be customized between 21-44)
- `--file-mode <MODE>`: Set file permissions when creating files (Unix-style octal, e.g., 644, 600)
- `--dir-mode <MODE>`: Set directory permissions when creating directories (Unix-style octal, e.g., 755, 700)
- `--audit-log`: Enable audit logging (outputs operations to stderr with timestamps)
- `-s, --serve <PORT>`: Start HTTP server on specified port to expose API endpoints (binds to localhost by default)
- `--listen-all-ips`: Listen on all network interfaces (0.0.0.0) instead of localhost only (requires --serve)
- `--max-param-length <N>`: Maximum length for prefix and suffix parameters in server mode (default: 32)
- `--enable-rate-limiting`: Enable rate limiting for server mode (default: disabled for better performance)
- `--rate-limit <N>`: Requests per second limit when rate limiting is enabled (default: 100)
- `--enable-cors`: Enable CORS headers for cross-origin requests (default: disabled)
- `--max-body-size <N>`: Maximum request body size in bytes (default: 1024)

Notes:
- The alphabet options (`--no-look-alike`, `--full`, `--full-with-symbols`, `--api-key`, and `--password`) are mutually exclusive
- `--mkdir` and `--touch` are mutually exclusive
- When using `--mkdir` or `--touch`, the `--check` flag is automatically enabled to prevent naming conflicts
- `--prefix`, `--suffix`, and `--path` options require either `--mkdir` or `--touch`
- `--api-key` cannot be combined with any other options (it generates a fixed 44-character key with ak_ prefix)
- `--password` can only be combined with a custom length parameter (21-44 characters)
- Permission options (`--file-mode`, `--dir-mode`) only work with `--touch` and `--mkdir` respectively
- `--audit-log` can be used with any operation and also controlled via `HASHRAND_AUDIT_LOG` environment variable
- Server security options (`--max-param-length`, `--enable-rate-limiting`, `--rate-limit`, `--enable-cors`, `--max-body-size`) only work with `--serve`
- `--rate-limit` requires `--enable-rate-limiting` to be effective

### Examples

Generate a hash with default length (21 characters) using base58:
```bash
hashrand
```

Generate a 16-character hash without newline:
```bash
hashrand -r 16
```

Generate a 32-character hash that doesn't match any existing filename:
```bash
hashrand -c 32
```

Generate a hash using the no-look-alike alphabet:
```bash
hashrand --no-look-alike 24
```

Generate a hash with full alphanumeric characters:
```bash
hashrand --full 20
```

Generate a hash including symbols:
```bash
hashrand --full-with-symbols 16
```

Create a directory with a random name:
```bash
hashrand --mkdir
```

Create a file with a random name and custom length:
```bash
hashrand --touch 32
```

Create a directory with prefix and suffix:
```bash
hashrand --mkdir --prefix "temp_" --suffix "_data"
```

Create a file in a specific path:
```bash
hashrand --touch --path /tmp --prefix "session_"
```

Create a directory with no-look-alike alphabet and custom path:
```bash
hashrand --mkdir --no-look-alike --path ./backups --suffix "_backup"
```

Generate a secure API key (format: ak_xxxxxxxx, 47 characters total):
```bash
hashrand --api-key
```

Generate a secure password with default length (21 characters):
```bash
hashrand --password
```

Generate a secure password with custom length:
```bash
hashrand --password 30
```

Create a file with specific permissions (Unix only):
```bash
hashrand --touch --file-mode 600
```

Create a directory with restricted permissions:
```bash
hashrand --mkdir --dir-mode 700 --prefix "secure_"
```

Generate with audit logging enabled:
```bash
hashrand --audit-log --mkdir
# Or via environment variable:
HASHRAND_AUDIT_LOG=1 hashrand --touch
```

Generate an API key without newline (for scripts):
```bash
hashrand --api-key --raw
# Output: ak_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

Generate a password without newline:
```bash
hashrand --password -r
```

Start HTTP server for API access:
```bash
# Start server on port 8080 (localhost only by default)
hashrand --serve 8080

# Use short form
hashrand -s 3000

# Listen on all network interfaces
hashrand --serve 8080 --listen-all-ips

# Enable security features
hashrand --serve 8080 --enable-rate-limiting --rate-limit 50
hashrand --serve 8080 --enable-cors --max-param-length 64
hashrand --serve 8080 --enable-rate-limiting --enable-cors --max-body-size 2048
```

## HTTP Server Mode

When started with `--serve PORT`, hashrand runs as an HTTP server exposing REST API endpoints that provide the same functionality as the CLI (excluding file system operations like `--touch` and `--mkdir`).

For complete API documentation, see [docs/API.md](docs/API.md).

### Server Configuration

- **Default binding**: `127.0.0.1` (localhost only) for security
- **All interfaces**: Use `--listen-all-ips` to bind to `0.0.0.0`
- **Default response format**: Raw text (no newline) - use `raw=false` query parameter to add newline

**⚠️ IMPORTANT SECURITY WARNING**: The HTTP server should **ALWAYS** be deployed behind a reverse proxy (nginx, Apache, Caddy, etc.) that enforces SSL/TLS encryption. Never expose the server directly to the internet without HTTPS protection. The built-in server does not provide SSL/TLS encryption and transmits all data in plain text.

### Security Features

The server includes several configurable security features:

- **Parameter Validation**: `--max-param-length N` limits prefix/suffix parameter length (default: 32 characters)
- **Rate Limiting**: `--enable-rate-limiting` with `--rate-limit N` (default: 100 requests/second per IP)
- **CORS Control**: `--enable-cors` enables cross-origin requests (disabled by default)
- **Request Size Limiting**: `--max-body-size N` limits request body size (default: 1024 bytes)

All security features are **disabled by default** for optimal performance. Enable them based on your deployment requirements:

```bash
# Production-ready configuration with security enabled
hashrand --serve 8080 \
  --enable-rate-limiting --rate-limit 100 \
  --max-param-length 32 \
  --max-body-size 1024

# Development configuration with CORS for testing
hashrand --serve 8080 --enable-cors
```

### Available API Endpoints

All endpoints return plain text responses (raw by default) and support the following query parameters:

#### GET /api/generate
Generate a random hash with customizable options.

**Query Parameters:**
- `length` (optional): Hash length, 2-128, default 21
- `alphabet` (optional): Alphabet type - "base58" (default), "no-look-alike", "full", "full-with-symbols"  
- `raw` (optional): Boolean, if false adds newline, default true
- `prefix` (optional): String prefix to add before hash
- `suffix` (optional): String suffix to add after hash

**Examples:**
```bash
curl "http://localhost:8080/api/generate"
curl "http://localhost:8080/api/generate?length=16&alphabet=full"
curl "http://localhost:8080/api/generate?length=32&alphabet=no-look-alike&raw=false"
curl "http://localhost:8080/api/generate?prefix=user_&suffix=_id&length=12"
```

#### GET /api/api-key
Generate a secure API key (format: ak_ + 44 characters).

**Query Parameters:**
- `raw` (optional): Boolean, if false adds newline, default true

**Examples:**
```bash
curl "http://localhost:8080/api/api-key"
curl "http://localhost:8080/api/api-key?raw=false"
```

#### GET /api/password
Generate a secure password using full alphabet with symbols.

**Query Parameters:**
- `length` (optional): Password length, 21-44, default 21
- `raw` (optional): Boolean, if false adds newline, default true

**Examples:**
```bash
curl "http://localhost:8080/api/password"
curl "http://localhost:8080/api/password?length=30"
curl "http://localhost:8080/api/password?length=44&raw=false"
```

### Server Usage Examples

```bash
# Start server
hashrand --serve 8080

# In another terminal, make requests
curl "http://localhost:8080/api/generate?length=16"
# Output: a1B2c3D4e5F6g7H8

curl "http://localhost:8080/api/api-key"
# Output: ak_X1y2Z3a4B5c6D7e8F9g0H1i2J3k4L5m6N7o8P9q0R1s2

curl "http://localhost:8080/api/password?length=25"
# Output: aB3*fG7$hI9@kL2#mN5^pQ8!
```

**Note:** The HTTP server mode excludes file system operations (`--touch`, `--mkdir`) for security reasons. These operations are only available in CLI mode.

## Alphabet Options

1. **Base58 (default)**: Bitcoin alphabet excluding 0, O, I, l
   - Characters: `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`
   - 58 characters total

2. **No Look-Alike**: Excludes commonly confused characters
   - Excludes: 0, O, I, l, 1
   - Characters: `23456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`
   - 57 characters total

3. **Full Alphanumeric**: All letters and numbers
   - Characters: `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`
   - 62 characters total

4. **Full with Symbols**: Alphanumeric plus special characters
   - Characters: All alphanumeric plus `-_*^@#+!?$%`
   - 73 characters total

5. **API Key Mode** (`--api-key`): Secure API key generation
   - Format: `ak_` + 44 random characters (47 total)
   - Uses full alphanumeric alphabet (62 characters)
   - Provides 256 bits of entropy for quantum-resistant security
   - Cannot be customized or combined with other options
   - Follows modern API key identification standards

6. **Password Mode** (`--password`): Secure password generation
   - Uses full alphabet with symbols (73 characters)
   - Default length: 21 characters (128 bits entropy)
   - Length can be customized (21-44 characters)
   - Cannot be combined with other options except length
   - Minimum 21 characters ensures cryptographic security

## Security Features

`hashrand` includes several security enhancements to ensure safe operation:

### Path Security
- **Path validation and canonicalization** prevents directory traversal attacks
- **Base path verification** ensures files/directories are created within intended locations
- **Permission validation** checks that target paths exist and are accessible

### Resource Protection
- **Directory traversal limits** (10 levels deep) prevent resource exhaustion
- **File count limits** (100,000 entries) protect against DoS during collision checking
- **Generation attempt limits** (1,000 tries) prevent infinite loops

### Audit and Compliance
- **Comprehensive audit logging** tracks all operations with timestamps
- **Environment variable support** (`HASHRAND_AUDIT_LOG`) for automated environments
- **No sensitive data logging** follows security best practices
- **Unix permissions control** allows setting specific file/directory permissions

### Error Handling
- **Graceful error handling** with informative messages instead of panics
- **Input validation** ensures all parameters are within safe ranges
- **Secure defaults** maintain security when optional parameters aren't specified

## Threat Model

`hashrand` addresses the following security threats:

### Threat Categories

#### 1. **Path Traversal Attacks**
- **Threat**: Malicious users providing paths like `../../sensitive/location` to create files outside intended directories
- **Mitigation**: Path canonicalization and validation ensure all operations stay within base directories
- **Risk Level**: Medium → **Mitigated** ✅

#### 2. **Resource Exhaustion (DoS)**
- **Threat**: Attackers causing system resource exhaustion through deep directory traversal or large file structures
- **Mitigation**: Directory depth limits (10 levels), file count limits (100,000 entries), generation attempt limits (1,000)
- **Risk Level**: Medium → **Mitigated** ✅

#### 3. **Information Disclosure**
- **Threat**: Accidental logging of sensitive information or verbose error messages revealing system details
- **Mitigation**: Audit logging excludes sensitive data, error messages are informative but not revealing
- **Risk Level**: Low → **Mitigated** ✅

#### 4. **Privilege Escalation**
- **Threat**: Created files/directories having overly broad permissions allowing unauthorized access
- **Mitigation**: Unix permission controls (`--file-mode`, `--dir-mode`) allow explicit permission setting
- **Risk Level**: Low → **Mitigated** ✅

#### 5. **Cryptographic Weaknesses**
- **Threat**: Predictable random generation allowing attackers to guess generated values
- **Mitigation**: Uses `nanoid` with cryptographically secure randomness (ChaCha20), 256-bit entropy for API keys
- **Risk Level**: Critical → **Not Applicable** (Secure by design) ✅

### Attack Surface Analysis

#### **Inputs**
- Command-line arguments (validated with clap)
- Environment variables (`HASHRAND_AUDIT_LOG` - safe boolean flag)
- File system paths (canonicalized and validated)

#### **Outputs**
- Generated random strings (no sensitive data)
- File/directory creation (permissions controlled)
- Audit logs (no sensitive information logged)

#### **External Dependencies**
- **nanoid**: Cryptographically secure random generation (audited library)
- **clap**: CLI parsing (well-established, type-safe)
- **walkdir**: Directory traversal (limited depth/count to prevent DoS)

### Security Assumptions

1. **File System**: Assumes standard Unix-like permissions model
2. **Runtime Environment**: Assumes legitimate use by authorized users
3. **Cryptographic Randomness**: Relies on OS-provided entropy via nanoid
4. **Path Validation**: Assumes canonical paths correctly represent actual file locations

### Out of Scope

- **Network Security**: Application is local-only, no network components
- **Authentication/Authorization**: Relies on file system and OS-level controls  
- **Data Encryption**: Generated strings are public identifiers, not encrypted secrets
- **Social Engineering**: User education about secure usage patterns

## Features

- **Multiple alphabet options** for different use cases
- **Cryptographically secure** random generation using nanoid
- **Customizable hash length** (2-128 characters)
- **Raw output mode** for scripting and piping
- **Collision detection** to avoid matching existing filenames
- **Directory and file creation** with random names
- **Prefix and suffix support** for structured naming
- **Custom path support** for organizing generated items
- **Unix file permissions control** for secure file/directory creation
- **Audit logging system** for tracking operations and compliance
- **Security hardening** with path validation and resource limits
- **HTTP server mode** with REST API endpoints for integration
- **Plain text API responses** for easy integration with any language
- **Fast and lightweight** with minimal dependencies
- **Comprehensive test suite** ensuring reliability

## Use Cases

- Generating unique identifiers for files or database records
- Creating temporary file names that won't collide
- Generating secure tokens or passwords
- Creating random test data
- Generating URL-safe random strings
- Creating organized temporary directories with prefixes/suffixes
- Batch file/directory creation with guaranteed unique names
- Session file management with structured naming
- Generating secure API keys for authentication
- Creating strong passwords for user accounts or services
- **Security-focused scenarios:**
  - Creating secure temporary files with restricted permissions
  - Generating auditable random identifiers for compliance
  - Setting up secure directories for sensitive data processing
  - Creating trackable session directories with audit trails
- **HTTP API scenarios:**
  - Microservices requiring random identifier generation
  - Web applications needing secure token generation
  - CI/CD pipelines generating unique build identifiers
  - Container orchestration with random naming requirements

## Dependencies

- [nanoid](https://crates.io/crates/nanoid) - For secure random string generation
- [clap](https://crates.io/crates/clap) - For command-line argument parsing
- [walkdir](https://crates.io/crates/walkdir) - For recursive directory traversal (used with --check flag)
- [tokio](https://crates.io/crates/tokio) - Async runtime for HTTP server (server mode only)
- [axum](https://crates.io/crates/axum) - Web framework for REST API endpoints (server mode only)
- [serde](https://crates.io/crates/serde) - Serialization/deserialization for query parameters (server mode only)
- [tower](https://crates.io/crates/tower) - Service middleware for HTTP server (server mode only)
- [tower-http](https://crates.io/crates/tower-http) - HTTP-specific middleware for CORS and request limiting (server mode only)

## Security

For security-related issues, please see our [Security Policy](SECURITY.md).

To report security vulnerabilities, email: me@arkaitz.dev

## License

This project is open source and available under the [MIT License](LICENSE).