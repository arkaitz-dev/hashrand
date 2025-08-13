# hashrand

Generate cryptographically secure random strings via CLI, HTTP API, and web interface.

## What it offers

- **Random strings**: Generate hashes, passwords, and API keys
- **Multiple alphabets**: Base58 (default), no-look-alike, full, with symbols
- **CLI tool**: Direct terminal usage with file system operations
- **HTTP server**: REST API + modern TailwindCSS web interface for remote access
- **Performance optimized**: 48% bundle size reduction with intelligent caching
- **Security features**: Collision checking, path validation, audit logging

## Quick Start

```bash
# Install
cargo install --path .

# Basic usage
hashrand                    # Generate 21-char hash
hashrand 32                 # Generate 32-char hash
hashrand --password         # Generate secure password
hashrand --api-key          # Generate API key

# Development (with justfile)
just dev                    # Start dev servers (auto-configures Tailscale if available)
just build                  # Build for production
just test                   # Run all tests
just stop-dev               # Stop all dev servers and Tailscale serve
```

## CLI Usage

```bash
hashrand [LENGTH] [OPTIONS]
```

### Main Options

| Option | Description |
|--------|-------------|
| `--password` | Generate secure password (21+ chars) |
| `--api-key` | Generate API key with `ak_` prefix (44+ chars) |
| `--no-look-alike` | Use alphabet without confusing characters |
| `--full-with-symbols` | Include symbols for maximum entropy |
| `-c, --check` | Avoid existing filenames |
| `--mkdir` / `--touch` | Create directory/file with generated name |
| `--prefix` / `--suffix` | Add text before/after generated string |
| `-s, --serve PORT` | Start HTTP server |
| `-r, --raw` | No newline (for scripting) |

### Examples

```bash
# Basic usage
hashrand                         # K3m5Hn2L7p9X4qRs8vB1c
hashrand 16                      # K3m5Hn2L7p9X4qRs
hashrand --no-look-alike         # No confusing chars

# Special generation
hashrand --password              # aB3*fG7$hI9@kL2# (21 chars)
hashrand --password --no-look-alike  # K9mN3pQwTjH6XpRaBcDe (24 chars)
hashrand --api-key               # ak_x7K9mN3pQ5vB8zL2jH6tR4wY1sE9aF0cD6bG4hM7n
hashrand --api-key --no-look-alike   # ak_K9mN3pQwTjH6XpRaBcDeFgHkMnPqRtWxYzAbCdEfG

# File operations
hashrand --mkdir --prefix "temp_"    # Create directory: temp_K3m5Hn2L7p9X4qRs8vB1c
hashrand --touch -c --suffix ".tmp"  # Create file, avoid collisions

# Server mode
hashrand --serve 8080            # Start HTTP server + web UI
curl "localhost:8080/api/generate?length=16"  # API usage
```

## HTTP Server

Start a web server with REST API and interactive web interface:

```bash
# Development (with justfile)
just dev                    # Starts both API server (8080) and frontend (3000)
                           # Auto-configures Tailscale serve if available

# Production
just build                  # Build optimized version
just serve                  # Run production server

# Manual
hashrand --serve 8080       # Start server on port 8080
```

**Web Interface**: Visit `http://localhost:3000` (dev) or `http://localhost:8080` (production)
**API Endpoints**: `/api/generate`, `/api/password`, `/api/api-key`

### Remote Development with Tailscale

If you have Tailscale installed, `just dev` automatically configures secure remote access:

```bash
just dev
# Output includes:
# Configuring Tailscale serve...
# Tailscale: https://your-machine.ts.net

# Now accessible from any device in your Tailscale network
```

**Benefits**:
- Secure HTTPS access from any device
- No manual configuration needed
- Automatically stops when running `just stop-dev`

## Documentation

- **[API Reference](docs/API.md)**: REST API endpoints and examples
- **[Complete Guide](docs/GUIDE.md)**: Advanced usage, architecture, and development

## Alphabets

| Type | Characters | Use Case |
|------|------------|----------|
| **Base58** (default) | Excludes 0,O,I,l (58 chars) | General purpose, Bitcoin-style |
| **No-look-alike** | Extra safe, excludes 0,O,I,l,1,2,5,S,s,o,u,v,Z (49 chars) | Easy typing, no confusion |
| **Full** | All alphanumeric (62 chars) | Maximum character set |
| **Full + Symbols** | Alphanumeric + `-_*^@#+!?$%` (73 chars) | Maximum entropy |

## Use Cases

- **Development**: Unique IDs, test data, temporary files
- **Security**: API keys, passwords, tokens
- **Web services**: REST APIs, microservices integration
- **CI/CD**: Build identifiers, temporary directories
- **File management**: Collision-free naming with prefixes/suffixes

## Performance

**Web Interface Optimization (v0.6.0)**:
- 📦 **48% total bundle reduction**: 86kB → 45kB (19kB → 12kB gzipped)  
- ⚡ **95% main JS reduction**: 79kB → 3.7kB (17kB → 1.4kB gzipped)
- 🎨 **TailwindCSS integration**: Modern utility-first styling with optimal purging
- 📱 **Smart chunking**: Separate vendor, locale, and app bundles for better caching

## Requirements

- **Rust 1.89.0+** for building from source
- **Node.js 18+** (optional, for web interface development)
- **TailwindCSS 4.1.11** (automatically installed with npm dependencies)

## Development

```bash
# Clone and setup
git clone <repo>
cd hashrand

# Development workflow (justfile recommended)
just dev              # Start both servers + Tailscale
just test             # Run all tests
just build            # Build for production
just install          # Install binary
just run-installed    # Install + run production + Tailscale
just stop-installed   # Stop production binary

# Manual workflow
npm run dev           # Frontend dev server with HMR
npm run build:analyze # Production build with bundle analysis
cargo run -- --serve 8080  # API server
```

## License

MIT License - see [LICENSE](LICENSE) file.