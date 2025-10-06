# Quick Start Guide

Get HashRand running in minutes with this comprehensive setup guide.

## Prerequisites

- [Rust](https://rustup.rs/) (1.89+) - For the API backend
- [Fermyon Spin](https://developer.fermyon.com/spin/v2/install) - WebAssembly platform
- [Node.js 18+](https://nodejs.org/) - For the web interface

## Complete Development Setup

### 1. Generate Secure Secrets

```bash
# Clone the repository
git clone <repository-url>
cd hashrand

# Generate cryptographically secure secrets for development
python3 -c "
import secrets
print('# HashRand - Environment Variables for Backend API')
print('# These are cryptographically secure secrets - NEVER commit to git')
print()
print('# JWT Secret for token signing (64 hex chars = 32 bytes)')
print('JWT_SECRET=' + secrets.token_hex(32))
print()
print('# HMAC Key for magic link integrity (64 hex chars = 32 bytes)')
print('MAGIC_LINK_HMAC_KEY=' + secrets.token_hex(32))
print()
print('# Salt for Argon2id user ID derivation (64 hex chars = 32 bytes)')
print('ARGON2_SALT=' + secrets.token_hex(32))
print()
print('# Development/Production mode')
print('NODE_ENV=development')
" > .env
```

### 2. See Available Tasks

```bash
# See all available development tasks
just
```

### 3. Start Complete Development Environment

```bash
# Start complete development environment (recommended)
just dev
```

This single command will:

- 🚀 Start Spin API backend in background (port 3000)
- 🌐 Start npm web interface in background (port 5173)
- 🔗 Expose frontend via Tailscale for remote access
- ✅ Verify all services started successfully

## Available URLs

- **Local Web Interface**: http://localhost:5173
- **Local API**: http://localhost:3000
- **Remote Access**: https://your-tailscale-name.ts.net (automatically configured)

## Alternative Development Modes

### Foreground Mode

```bash
# Start in foreground mode (for direct log monitoring)
just dev-fg
```

### Background with Log Watching

```bash
# Start in background and watch logs (Ctrl+C to stop watching only)
just watch
```

### Service Management

```bash
# Check status of all services
just status

# Stop all development services
just stop
```

## Manual Setup (Optional)

If you prefer manual control over individual services:

```bash
# Terminal 1: Start the API backend only
spin-cli watch

# Terminal 2: Start the web interface only
cd web && npm run dev

# Terminal 3: Expose via Tailscale (optional)
just tailscale-front-start
```

## Background Development

For development workflows where you need the server running in the background:

```bash
# Start server in background (persistent after terminal close)
just dev-bg

# Check if background server is running
just status

# Follow logs from background server (Ctrl+C to stop watching)
tail -f .spin-dev.log

# Stop background server
just stop
```

The background server:

- Runs with PID tracking in `.spin-dev.pid`
- Logs output to `.spin-dev.log`
- Survives terminal sessions
- Auto-reloads on code changes

## Building

```bash
# Build both WebAssembly component and web interface
just build

# Clean and rebuild everything
just rebuild
just clean-build  # Same as rebuild

# Clean all build artifacts
just clean

# Start the application (production mode)
just up
```

## Testing

```bash
# Run comprehensive test suite (64 tests)
just test

# Run test with auto-started dev server
just test-dev
```

## Verification

After setup, verify everything is working:

1. **API Test**: Visit http://localhost:3000/api/version
2. **Web Interface**: Visit http://localhost:5173
3. **Authentication**: Try generating a hash (will prompt for email)
4. **Remote Access**: Check Tailscale URL if configured

## Troubleshooting

### Common Issues

**Port Conflicts:**

```bash
# Check what's using ports 3000/5173
lsof -i :3000
lsof -i :5173

# Stop conflicting processes or change ports in configuration
```

**Permission Issues:**

```bash
# Ensure proper permissions for database directory
mkdir -p data/
chmod 755 data/
```

**Missing Dependencies:**

```bash
# Install Rust if not present
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Spin CLI
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash

# Install Node.js dependencies
cd web && npm install
```

**Environment Variables:**

```bash
# Verify .env file exists and has proper format
cat .env | head -10

# Regenerate if needed
rm .env && [run Python script above]
```

### Support Resources

- **Documentation**: See other files in [docs/](../README.md)
- **Issues**: Report problems in project repository
- **Logs**: Check `.spin-dev.log` and browser console for errors

---

_For production deployment, see [Production Deployment](./production.md)_  
_For configuration details, see [Configuration Guide](./configuration.md)_  
_For development commands, see [Development Guide](./development.md)_
