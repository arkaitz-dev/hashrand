# HashRand Documentation

**HashRand** is a secure random hash generator built with WebAssembly and cryptographic best practices. Generate and share passwords, API keys, custom hashes, and BIP39 mnemonic phrases with military-grade security.

## What You Can Do

- **Generate**: Custom hashes, passwords, API keys, cryptographically secure BIP39 mnemonic phrases
- **Share Securely**: End-to-end encrypted secrets with read limits, expiration, and one-time passwords (OTP)
- **Track**: Monitor when and how many times your shared secrets have been accessed
- **Multilingual**: Available in 13 languages (Arabic, Basque, Catalan, Chinese, English, French, Galician, German, Hindi, Japanese, Portuguese, Russian, Spanish)

## Security Advantages

- **Zero Knowledge Authentication**: Server never stores or sees your passwords, you only need an email with no password at all, and your email is not stored for session handling. When sending a shared secret, in order to inform the receiver who sent the shared secret, the email is stored encrypted at most for three days. Only the receiver and sender can access this information, nobody else (even system's administrators).
- **End-to-End Encryption**: Secrets encrypted client-side before transmission (ChaCha20-Poly1305) to the backend. On secret sharing, when you receive a shared secret from the backend, it will also be encrypted during transmission to your browser.
- **Request Signing**: All API requests cryptographically signed (Ed25519) to prevent tampering. Only you can authenticate from the browser you are currently using. The backend also signs all responses.
- **WebAssembly Backend**: Fast, secure, sandboxed execution environment

## Documentation Structure

- **[API](./api/)** - Available endpoints and functionality
- **[Web Interface](./web/)** - User interface features and capabilities
- **[Architecture](./architecture/)** - Technical stack and cryptographic design

---

**Quick Start**: Visit the web interface, select your language (if needed), and start generating secure randomness. Authentication (magic link sent to your email) required for creating secrets, sharing secrets and viewing received shared secrets.
