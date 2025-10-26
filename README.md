# HashRand

Cryptographically secure random generator built with **Fermyon Spin** and **WebAssembly**. Generate and share passwords, API keys, hashes, and BIP39 mnemonics with Zero Knowledge privacy.

## Key Features

### Generation
- **Custom Hashes**: 16-512 bytes, SHA-256/SHA3-256/BLAKE3, multiple encodings (hex/base58/base64)
- **Passwords**: 8-128 characters, configurable character sets
- **API Keys**: 16-64 bytes, service integration ready
- **BIP39 Mnemonics**: 12-24 words, 10 languages (English, Spanish, Chinese, Japanese, Korean, Czech, Italian, French, Portuguese)

### Secure Sharing
- **End-to-End Encryption**: ChaCha20-Poly1305, client-side encryption
- **Access Control**: Read limits (1-10), expiration (1h-3d), 9-digit OTP
- **Privacy**: Zero Knowledge—emails encrypted, never stored for session handling

### Technical Stack
**Backend**: Rust + Fermyon Spin (WASM) + SQLite (Turso)
**Frontend**: SvelteKit + TypeScript + TailwindCSS
**Cryptography**: Ed25519 signing, X25519 encryption, Blake3 KDF, Argon2id hashing, ChaCha20-Poly1305 AEAD
**i18n**: 13 languages with RTL support

## Quick Start

```bash
# Clone and setup
git clone <repository-url> && cd hashrand

# Generate development secrets (see docs/deployment/ for details)
python3 -c "import secrets; print('JWT_SECRET='+secrets.token_hex(32)); ..." > .env

# Start development environment
just dev  # API: http://localhost:3000, Web: http://localhost:5173
```

## Documentation

📖 **[User Guide](docs/)** - Features, security, quick start
🔌 **[API Reference](docs/api/)** - REST endpoints and authentication
🎨 **[Web Interface](docs/web/)** - UI features and multilingual support
🏗️ **[Architecture](docs/architecture/)** - Zero Knowledge design, dual-key system, cryptographic implementation

## Security Architecture

- **Zero Knowledge Authentication**: Magic links, no passwords, emails hashed (Blake3 + Argon2id)
- **Dual-Key System**: Temporary session keys (System A) + Permanent E2EE keys (System B)
- **Signed Requests/Responses**: Ed25519 signatures prevent tampering and replay attacks
- **Perfect Forward Secrecy**: Ephemeral keys protect past communications

**License**: MIT | **Requirements**: Rust, Node.js 18+, Fermyon Spin CLI, Just
