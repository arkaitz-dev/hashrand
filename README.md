# hashrand

A versatile tool for generating cryptographically secure random strings, available both as a CLI application and as an HTTP server with web interface, featuring multiple alphabet options and safety features.

## Description

`hashrand` generates cryptographically secure random strings using various alphabets, available through:

- **Command-Line Interface (CLI)**: Direct command-line usage for scripts, automation, and terminal workflows
- **HTTP Server with Web Interface**: Interactive web UI and REST API endpoints for integration with web applications

By default, it uses the base58 alphabet (Bitcoin alphabet), which excludes similar-looking characters like 0, O, I, and l for better readability. The tool also supports other alphabet configurations and can ensure generated strings don't collide with existing filenames (CLI mode only).

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
- `--api-key`: Generate a secure API key using full alphanumeric alphabet (ak_ + 44-64 characters, default 44)
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
- `--api-key` can be combined with custom length parameter (44-64 characters) but conflicts with alphabet and file system options
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

Generate a secure API key with default length (ak_ + 44 characters = 47 total):
```bash
hashrand --api-key
```

Generate a secure API key with custom length:
```bash
hashrand --api-key 60
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
# Development mode (debug build) - API-only server
cargo run -- --serve 8080    # Vite dev server needed for frontend

# Production mode (release build) - Self-contained with embedded assets
cargo build --release
./target/release/hashrand --serve 8080    # No external files needed

# Use short form
hashrand -s 8080

# Listen on all network interfaces
hashrand --serve 8080 --listen-all-ips

# Enable security features
hashrand --serve 8080 --enable-rate-limiting --rate-limit 50
hashrand --serve 8080 --enable-cors --max-param-length 64
hashrand --serve 8080 --enable-rate-limiting --enable-cors --max-body-size 2048
```

## HTTP Server Mode

When started with `--serve PORT`, hashrand runs as an HTTP server exposing REST API endpoints that provide the same functionality as the CLI (excluding file system operations like `--touch` and `--mkdir`).

### Web Interface

The HTTP server includes an interactive web interface with different serving strategies depending on build mode:

**Development Mode** (`cargo run`): API-only server (frontend handled by Vite dev server)
**Production Mode** (`cargo build --release`): Self-contained binary with embedded web assets

The web interface accessible at the root URL (`http://localhost:PORT/`) features include:

- **Menu-based Navigation**: Choose between Generic Hash, Password, or API Key generation modes
- **Dedicated Configuration Views**: Each generation mode has its own interface with mode-specific options:
  - **Generic Hash**: Length slider (2-128), alphabet selection, prefix/suffix options
  - **Password**: Length slider (21-44) with strength indication
  - **API Key**: Length slider (44-64) with format preview
- **Separated Result View**: Clean result display with three navigation options:
  - Back to configuration (modify parameters)
  - Back to main menu (start over)
  - Regenerate with same configuration
- **Real-time Generation**: Generate hashes directly from the web interface with instant results
- **Copy to Clipboard**: One-click copy functionality for all generated results
- **Responsive Design**: Mobile-friendly interface that works on all devices
- **Modern Architecture**: Built with Lit 3 framework using standard decorators and Vite build system
- **Official Configuration**: Uses Lit's recommended setup for JavaScript with Babel decorator support
- **Optimized Performance**: Production builds are highly optimized (~14 kB gzipped)

#### Development & Production Workflows

The web interface is built with modern web technologies (Lit 3 framework with standard decorators + Vite build tool) and organized in a dedicated `web-ui/` directory with different workflows for development and production:

**Development Workflow:**
1. `npm run dev` → Vite dev server with HMR on port 3000  
2. `cargo run -- --serve 8080` → API-only server (no static files)

**Production Workflow:**
1. `npm run build` → Generate optimized assets in `dist/`
2. `cargo build --release` → Embed assets in binary at compile time (~3.1MB total)
3. Deploy single binary → No external files needed

**Benefits of Embedded Assets:**
- ✅ **Single file deployment** - Just copy the binary, no additional setup
- ✅ **Zero external dependencies** - Web interface included in binary
- ✅ **Simplified distribution** - No need to manage `dist/` directory
- ✅ **Version consistency** - Assets always match binary version

**Project Structure:**
```
hashrand/
├── src/                    # Rust backend code (modularized)
│   ├── cli/               # CLI argument handling
│   ├── server/            # HTTP server & routes  
│   ├── generators/        # Hash generation logic
│   └── utils/             # Common utilities
├── web-ui/                # Frontend code (separated)
│   ├── index.html
│   └── src/
│       ├── components/    # Lit components
│       └── css/          # Stylesheets
├── package.json           # npm configuration (root level)
├── vite.config.js         # Vite configuration (root level)
└── dist/                  # Production build output
```

**Development Commands (all from project root):**
- **Development**: Use `npm run dev` for development server with Hot Module Replacement (HMR) on `http://localhost:3000`
- **Production**: Run `npm run build` to generate optimized files in `dist/`, then start the Rust server with `--serve`

```bash
# Development workflow (all commands from project root)
npm run dev                    # Vite dev server on localhost:3000
cargo run -- --serve 8080     # API server on localhost:8080 (proxied by Vite)

# Production workflow (all commands from project root)
npm run build                  # Build optimized files to dist/
cargo run -- --serve 8080     # Serves both web UI (from dist/) and API

# Rust development (all commands from project root)
cargo build                    # Build Rust code
cargo test                     # Run test suite (46 tests)
cargo run -- --help           # Run CLI directly
```

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
Generate a secure API key with configurable length (format: ak_ + 44-64 characters).

**Query Parameters:**
- `length` (optional): API key length, 44-64, default 44
- `raw` (optional): Boolean, if false adds newline, default true

**Examples:**
```bash
curl "http://localhost:8080/api/api-key"
curl "http://localhost:8080/api/api-key?length=60"
curl "http://localhost:8080/api/api-key?length=50&raw=false"
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

curl "http://localhost:8080/api/api-key?length=60"
# Output: ak_A1B2C3D4E5F6G7H8I9J0K1L2M3N4O5P6Q7R8S9T0U1V2W3X4Y5Z6a7b8

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
   - Format: `ak_` + 44-64 random characters (47-67 total)
   - Uses full alphanumeric alphabet (62 characters)
   - Default 44 characters provides 256 bits of entropy
   - Length can be customized between 44-64 characters
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

### Core Features
- **Multiple alphabet options** for different use cases
- **Cryptographically secure** random generation using nanoid
- **Customizable hash length** (2-128 characters for generic, 21-44 for passwords, 44-64 for API keys)
- **Prefix and suffix support** for structured naming

### CLI Features
- **Raw output mode** for scripting and piping
- **Collision detection** to avoid matching existing filenames
- **Directory and file creation** with random names
- **Custom path support** for organizing generated items
- **Unix file permissions control** for secure file/directory creation
- **Audit logging system** for tracking operations and compliance
- **Security hardening** with path validation and resource limits

### Web/API Features
- **Interactive web interface** with modern UI for all generation modes
- **REST API endpoints** for programmatic access
- **Real-time generation** with instant results
- **Copy to clipboard** functionality
- **Responsive design** for all devices
- **Configuration views** for each generation type
- **Result view** with regeneration options
- **Plain text API responses** for easy integration with any language

### Technical Features
- **Fast and lightweight** with minimal dependencies
- **Modern Rust** built with Rust 1.89.0 (latest stable) for optimal performance and safety
- **Comprehensive test suite** ensuring reliability
- **Modern web stack** using Lit 3 framework with standard decorators and Vite build system
- **Official Configuration** following Lit's recommended JavaScript + Babel setup for decorators

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

**Runtime Requirements:**
- **Rust**: 1.89.0 (latest stable) - Modern Rust with latest language features and performance improvements

**Crate Dependencies:**
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