# Web Interface

HashRand's web interface provides an intuitive, multilingual experience for secure random generation and secret sharing.

## Core Features

### Random Generation

**Custom Hash**
- Generate random hashes from 16 to 512 bytes
- Choose hash algorithm: SHA-256, SHA3-256, or BLAKE3
- Select output encoding: Hexadecimal, Base58, or Base64
- One-click copy to clipboard

**Password Generator**
- Create secure passwords (8-128 characters)
- Select output encoding

**API Key Generator**
- Generate API keys for service integrations
- Configurable length (16-64 bytes)
- Multiple encoding formats

**BIP39 Mnemonic Phrases**
- Generate cryptographically secure mnemonic phrases
- Word counts: 12, 15, 18, 21, or 24 words
- Languages: Czech, Chinese (Simplified/Traditional), English, French, Italian, Japanese, Korean, Portuguese, Spanish
- Standard-compliant for cryptocurrency wallets

### Secure Sharing (Authentication Required)

**Share Secret**
- Encrypt and share securely (passwords, API keys, custom hashes and mnemonics) providing the receiver's email
- Configure maximum reads (1-10)
- Set expiration time (1 hour to three days)
- Optional: Send magic link to your (sender) email
- Sends two URLs in two different emails:
  - **Sender URL**: View OTP code, monitor reads, delete if needed
  - **Receiver URL**: To be viewed only by the authenticated receiver
- Optional 9-digit one-time password for access control
- Sender shares OTP with recipient through secure channel (phone, Signal, etc.)
- Prevents unauthorized access even if URL is leaked

### User Experience

**Language Selector**
- 13 languages supported: Arabic, Basque, Catalan, Chinese, English, French, Galician, German, Hindi, Japanese, Portuguese, Russian, Spanish
- Persistent preference (saved in browser)
- Affects all UI elements and email communications

**Theme Toggle**
- Light and dark modes
- Automatic OS preference detection
- Smooth transitions between themes

**Authentication**
- Passwordless magic link system
- Enter email → receive link → instant access
- Zero Knowledge: server never sees or stores passwords, or user's emails
- Automatic session management

**Responsive Design**
- Mobile-first approach
- Works seamlessly on phones, tablets, and desktops
- Touch-optimized controls

---

**Privacy**: The server never sees your plaintext shared secrets.
