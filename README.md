# HashRand

A **cryptographically secure random hash generator** built with Fermyon Spin and WebAssembly. Generate secure hashes, passwords, API keys, and BIP39 mnemonic phrases with complete user privacy through Zero Knowledge architecture.

## 🎯 Quick Start

```bash
# 1. Clone and setup
git clone <repository-url>
cd hashrand-spin

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

# 4. Open web interface
# Local: http://localhost:5173 (dev) or http://localhost:3000 (unified)
# Remote: https://your-tailscale-name.ts.net (via Tailscale)
```

## 🚀 Features

### Zero Knowledge Privacy Architecture
- **🛡️ Complete Privacy**: Server never stores emails or personal information
- **🔐 Cryptographic User IDs**: Blake2b-based multi-layer security for user identification
- **🎫 Magic Link Authentication**: Passwordless authentication with cryptographic integrity
- **🔒 JWT Protection**: Bearer token authentication for all sensitive operations
- **📊 Privacy-Safe Logging**: Base58 usernames enable audit without compromising privacy

### Secure Generation Capabilities
- **🎯 Multiple Formats**: Hashes, passwords, API keys, BIP39 mnemonic phrases
- **🌱 Dual Generation**: Random (GET) and deterministic seed-based (POST) generation
- **🔤 Multiple Alphabets**: Base58, alphanumeric, symbols, hex, and custom character sets
- **🌍 BIP39 Multilingual**: 10-language support (English, Spanish, Chinese, etc.)
- **⚡ WebAssembly**: Fast, lightweight serverless architecture

### Professional Web Interface  
- **🎨 Modern UI**: SvelteKit + TypeScript + TailwindCSS 4.0
- **📱 Responsive**: Perfect on mobile, tablet, and desktop
- **🌙 Smart Themes**: Manual dark/light mode with system preference detection
- **🌍 Full i18n**: 13 languages with RTL support
- **♿ Accessibility**: ARIA labels, keyboard navigation, screen reader support

### Enterprise Security
- **🏗️ Blake2b Unified Stack**: High-performance cryptographic operations
- **🔐 Argon2id**: Memory-hard user ID derivation following OWASP 2024 standards
- **🛡️ ChaCha20 Encryption**: Stream cipher for magic link encryption
- **🔒 URL Parameter Encryption**: Advanced ChaCha20-Poly1305 encryption system for URL privacy protection
- **🔄 2/3 Time-Based Token Management**: Intelligent dual-token refresh system with expiration handling
- **🧪 Comprehensive Testing**: 64 automated tests covering all functionality

## 📊 Architecture

### Technology Stack
- **Backend**: Rust + Fermyon Spin + WebAssembly + SQLite
- **Frontend**: SvelteKit + TypeScript + TailwindCSS + Vite
- **Security**: Blake2b + Argon2id + ChaCha20-Poly1305 + JWT + URL Encryption
- **Cryptography**: @noble/hashes + @noble/ciphers (enterprise-grade)
- **Database**: SQLite with Zero Knowledge schema

### API Endpoints
- **Authentication**: `POST/GET /api/login/*` - Magic link authentication
- **Generation**: `POST /api/{custom,password,api-key,mnemonic}` - Secure generation (JWT protected)
- **User Management**: `GET/POST/DELETE /api/users` - User operations (JWT protected)  
- **System**: `GET /api/version` - Public version information

## 🔧 Development Commands

```bash
# Essential commands (using just)
just dev         # Start complete development environment
just stop        # Stop all services  
just test        # Run 64 comprehensive tests
just check       # Code quality (clippy + fmt + ESLint + svelte-check)
just build       # Build API (WASM) + Web (SPA)
just predeploy   # Production deployment with unified backend

# Quality assurance
just fmt         # Format all code (Rust + Prettier)
just lint        # Linting (Rust clippy + ESLint)
just clean       # Clean all build artifacts
```

## 📖 Documentation

Complete documentation is available in the [`docs/`](docs/) directory:

### 📚 Core Documentation
- **[API Reference](docs/api/)** - Complete REST API documentation and authentication
- **[Web Interface](docs/web/)** - Frontend architecture and internationalization  
- **[Deployment](docs/deployment/)** - Development, configuration, and production guides
- **[Architecture](docs/architecture/)** - Zero Knowledge design and security implementation

### 🛠️ Developer Resources
- **[Testing Guide](docs/guides/testing.md)** - Comprehensive testing strategies and implementation
- **[Contributing](docs/guides/contributing.md)** - Development workflow and standards
- **[Dependencies](docs/guides/dependencies.md)** - Technology stack and third-party integrations

## 🔒 Security & Privacy

HashRand implements **true Zero Knowledge architecture**:

- **No Personal Data**: Server databases contain zero personal information
- **Cryptographic Identity**: Users identified by Blake2b-derived 16-byte hashes
- **GDPR/CCPA Compliant**: No personal data to manage or delete
- **Enterprise Security**: Multi-layer cryptographic protection with industry standards
- **Audit-Safe**: All logs use Base58 usernames, safe for analysis

### 🛡️ Ultra-Compact URL Parameter Encryption

**Complete Privacy Protection**: Revolutionary ultra-compact URL parameter encryption system protects user data even from browser history inspection:

- **📏 66% URL Reduction**: Single parameter `p` replaces dual `encrypted` + `idx` format (v0.19.12+)
- **🎯 Binary Concatenation**: idx_bytes + encrypted_bytes combined before Base64URL encoding
- **🔐 ChaCha20-Poly1305 Encryption**: Enterprise-grade AEAD encryption for all URL parameters
- **🎲 Random Prehash Seeds**: Content-independent cryptographic keys eliminate pattern analysis
- **🔑 Triple Token System**: Cipher/nonce/HMAC keys (32 bytes each) for maximum security
- **📦 Base64URL Encoding**: URL-safe transmission without padding characters
- **🔄 FIFO Rotation**: Automatic cleanup with 20-seed limit prevents storage bloat
- **🧂 Crypto Salt**: 32-byte internal noise generation for enhanced security
- **🏷️ KV Storage**: 8-byte cryptographic keys for efficient sessionStorage management

## 🚀 Production Deployment

### Unified Backend (Recommended)
```bash
just predeploy   # Complete production deployment
# - Builds optimized web interface and API
# - Serves both from unified backend (port 3000)  
# - Enables Tailscale for remote access
# - Single-port deployment, no proxy required
```

### Cloud Platforms
- **Fermyon Cloud**: Native Spin deployment with global edge
- **Self-Hosted**: Docker, Kubernetes, or bare metal deployment
- **Static + API**: Separate frontend (CDN) and backend deployment

## 📋 Requirements

- **Rust** (latest stable) with `wasm32-wasi` target
- **Node.js 18+** with npm
- **Fermyon Spin CLI** (for WebAssembly runtime)
- **Just** command runner (task automation)

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🤝 Contributing

See [Contributing Guide](docs/guides/contributing.md) for development workflow, code standards, and submission guidelines.

---

**HashRand**: Secure hash generation with complete privacy protection. Built with modern web technologies and cryptographic best practices.