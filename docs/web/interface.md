# Web Interface Features

The HashRand web interface is a modern **Single Page Application (SPA)** built with SvelteKit, TypeScript, and TailwindCSS 4.0, providing a professional and accessible user experience.

## Core Interface Features

### 🎨 Professional UI Design
- **Modern SPA**: Built with SvelteKit + TypeScript + TailwindCSS 4.0
- **📱 Responsive Design**: Works perfectly on mobile, tablet, and desktop
- **♿ Accessibility Compliant**: ARIA labels, keyboard navigation, screen reader support
- **🎬 Loading Animations**: Smooth spinning animations during hash generation
- **✨ Dynamic Feedback**: Context-aware help text and real-time parameter validation

### 🌙 Smart Theme System
- **Manual Theme Toggle**: Dark/light mode toggle in upper-right corner
- **System Preference Detection**: Respects system preference on first visit
- **Persistent User Choice**: Theme preference saved in IndexedDB (v0.19.14+)
- **Smooth Transitions**: Visual feedback and seamless theme switching
- **Accessible Design**: Proper ARIA labels for theme controls

### 🎛️ Interactive Controls
- **Beautiful Range Sliders**: Gradient styling for parameter selection
- **Real-time Validation**: Immediate feedback on parameter changes
- **Context-aware Help**: Dynamic help text based on current settings
- **Visual Parameter Display**: Clear indication of current values
- **Touch-friendly**: Optimized for mobile interaction

### 📋 Advanced Clipboard Integration
- **One-click Copying**: Copy generated results with visual feedback
- **Success Indicators**: Clear confirmation when copy succeeds
- **Error Handling**: Graceful fallback when clipboard unavailable
- **Keyboard Support**: Copy with keyboard shortcuts

### 🔄 In-Place Regeneration
- **No Page Reloads**: Generate new hashes without leaving the result page
- **State Preservation**: Maintains current parameters during regeneration
- **Instant Updates**: Results update immediately in place
- **Loading States**: Visual feedback during regeneration

### 🖼️ Advanced Icon System
- **Progressive SVG Sprite Loading**: Non-blocking sprite loading after DOM ready
- **Instant Placeholders**: UTF emoji placeholders for immediate visual feedback
- **189KB Professional Sprite**: High-quality flag SVGs and UI icons including user silhouette
- **Smart Loading States**: Visual feedback during sprite loading with smooth transitions
- **Zero Quality Compromise**: Full-resolution icons with optimized loading
- **Consistent User Icon**: Filled user silhouette (👤) with emoji fallback for session management

## URL Parameter Support

### 🔐 Ultra-Compact Encrypted Parameter Architecture

HashRand uses **enterprise-grade URL parameter encryption** with revolutionary ultra-compact format to protect user privacy, even from browser history inspection:

#### Ultra-Compact URL Format (v0.19.12+)
```bash
# All URLs now use single ultra-compact parameter 'p' (except magiclink)
http://localhost:5173/result/?p=<compact-base64url>

# Example ultra-compact encrypted URL (66% shorter than previous format)
http://localhost:5173/result/?p=k7J9mN4QR7FVMz2k9T7L8X3N5A6P

# Magic link exception (only unencrypted parameter)
http://localhost:5173/?magiclink=<token>
```

#### Technical Implementation
- **🎯 Binary Concatenation**: idx_bytes (8 bytes) + encrypted_bytes combined before Base64URL encoding
- **📏 66% URL Reduction**: Single parameter `p` replaces `encrypted` + `idx` format
- **⚡ Zero Breaking Changes**: All APIs maintain compatibility while optimized internally

#### URL Format Evolution
```bash
# Evolution: Direct parameters → Dual parameters → Ultra-compact
# v0.19.10 and earlier: Direct parameters (insecure)
http://localhost:5173/custom/?length=32&alphabet=base58

# v0.19.11: Dual parameter encryption (secure but verbose)
http://localhost:5173/result/?encrypted=R7FVMz2k9T7L8X3N5A6P&idx=k7J9mN4Q

# v0.19.12+: Ultra-compact encryption (secure and optimized)
http://localhost:5173/result/?p=k7J9mN4QR7FVMz2k9T7L8X3N5A6P
```

#### Parameter Encryption Features
- **🔐 ChaCha20-Poly1305 Encryption**: All parameters encrypted with enterprise-grade AEAD
- **🎲 Random Prehash Seeds**: Cryptographic keys independent of parameter content
- **📦 Base64URL Encoding**: URL-safe transmission without padding characters
- **🔑 Session Key Management**: Triple token system (cipher/nonce/hmac) for maximum security
- **🗑️ FIFO Rotation**: Automatic cleanup with 20-parameter limit in IndexedDB

#### Privacy Protection Benefits
- **🛡️ Browser History Privacy**: Parameters never appear in plaintext in browser history
- **🔒 Zero Plaintext Exposure**: All sensitive configuration encrypted before URL generation
- **📱 Device Access Protection**: URLs remain private even with physical device access
- **🎯 Seamless UX**: Users experience identical functionality with enhanced privacy

### Centralized API Architecture
- **Generator Pages** (`/custom/`, `/password/`, `/api-key/`, `/mnemonic/`): Handle UI and navigation with encrypted parameter support
- **Result Page** (`/result/`): Centralized API calling based on encrypted URL parameters  
- **Fresh Generation**: Result page always generates new values, never displays cached results
- **Shareable URLs**: Complete configuration can be shared via encrypted URL parameters
- **Universal Decryption**: All routes automatically decrypt parameters with fallback to legacy format

## Authentication Integration

### 🔐 Privacy-First Authentication System with Ed25519 Digital Signatures (v0.19.13+)
- **Explore First, Authenticate Later**: All generator pages accessible without login
- **On-Demand Authentication**: Authentication dialog appears only when clicking "Generate"
- **Privacy-First Design**: Server never stores or processes email addresses
- **Dialog-Based Protection**: Modern authentication flow with modal dialogs for all generation routes
- **Always-Visible Session Button**: User icon (👤) always visible for consistent authentication access

#### Automatic Ed25519 Cryptographic Integration
**COMPLETE FRONTEND PARTICIPATION**: As of v0.19.13, the frontend automatically handles Ed25519 keypair generation, message signing, and secure storage, eliminating the need for manual cryptographic operations.

The frontend `api.requestMagicLink()` function now automatically:
1. **Generates Ed25519 Keypair**: Creates cryptographically secure keypair using Web Crypto API
2. **Signs Authentication Message**: Automatically signs `email + pub_key` combination
3. **Manages Secure Storage**: Stores keypairs in IndexedDB with non-extractable private keys
4. **Handles Cleanup**: Automatically clears keypairs on logout for security

**Security Features:**
- **🔐 Non-extractable Keys**: Private keys stored as non-extractable CryptoKey objects
- **💾 IndexedDB Storage**: Secure browser database for keypair persistence
- **🧹 Automatic Cleanup**: Ed25519 keypairs cleared on logout
- **🔄 Hybrid Architecture**: Web Crypto API primary with @noble/curves fallback
- **🛡️ Zero Knowledge**: No personal data stored, only cryptographic keys

### Authentication Dialog System
- **Unified Experience**: Consistent authentication across all generation pages
- **Two-step Process**: Email input with validation and confirmation
- **Advanced State Preservation**: Form parameters encoded as base58 URL-safe strings
- **Universal Integration**: Works across all generator pages (custom/, password/, api-key/, mnemonic/)
- **Professional Design**: Matches existing dialog components

### State Management & Security
- **State-Aware Form Handling**: Automatic parameter preservation through authentication flow
- **Encrypted Parameter Encoding**: Form parameters encrypted before URL generation
- **Secure Persistent Storage**: Parameters managed in IndexedDB with cross-tab synchronization and automatic cleanup
- **Seamless Restoration**: Form state restored after authentication completion
- **Clean User Flow**: Smooth transition from form → authentication → result generation
- **Preventive Data Clearing**: Complete cleanup before every authentication dialog for maximum security

### Magic Link Integration
- **Dynamic Magic Links**: Automatically adapt to current host (localhost/Tailscale)
- **Email-based Authentication**: Passwordless authentication with secure magic link generation
- **Multilingual Email Delivery**: Magic link emails in 13 languages
- **JWT Dual Token System**: Access tokens (15 min) + HttpOnly refresh cookies (1 week)
- **Automatic Token Management**: Frontend handles token refresh transparently
- **Session Persistence**: Maintains authentication state across browser sessions

### Development Mode Features
- **Console-logged Magic Links**: Easy development and testing with email fallback
- **Database Sessions**: Complete session management with automatic cleanup
- **Mobile-Optimized Debug**: Extended debug display (20 seconds) for tablet development
- **Fallback Systems**: Multiple fallback layers for development environments

## Seed-Based Generation UI

### 🌱 Deterministic Generation Interface
- **Read-only Seed Display**: Seeds shown only when provided via encrypted URL parameters (non-editable)
- **Base58 Validation**: 44-character base58 seed format with visual feedback
- **Smart UI Behavior**: Regenerate button hidden when seed detected in decrypted parameters
- **Informational Display**: Seeds shown as informational text without copy functionality
- **Simplified Integration**: Clean seed handling without complex UI interactions
- **Privacy Protection**: Seed values encrypted in URLs, never exposed in browser history

### URL Seed Integration
```bash
# Modern ultra-compact encrypted URL with seed (read-only display after decryption)
http://localhost:5173/result/?p=<compact-base64url>
# Decrypted parameters contain: seed=2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR&length=32

# Legacy format (automatic encryption conversion)
http://localhost:5173/custom/?seed=2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR&length=32
→ Automatically encrypted for privacy protection
```

### Seed Behavior
- **Encrypted seed parameters**: Decrypted from encrypted URLs, shown as read-only informational text
- **API-generated seeds**: Displayed as informational metadata without copy functionality
- **No Seed Input Fields**: Simplified interface without complex interactions
- **44-character Base58**: Standard format with visual validation
- **Privacy-First**: All seed parameters encrypted in URLs for complete browser history protection

## Mobile & Responsive Design

### 📱 Mobile-First Approach
- **Touch Optimized**: All controls designed for touch interaction
- **Responsive Breakpoints**: Optimized layouts for all screen sizes
- **Mobile Navigation**: Streamlined navigation for small screens
- **Touch-friendly Buttons**: Appropriate sizing for finger interaction
- **Swipe Gestures**: Natural mobile interaction patterns

### Cross-Platform Compatibility
- **Browser Support**: Works on all modern browsers
- **Device Agnostic**: Consistent experience across devices
- **Performance Optimized**: Fast loading on mobile networks
- **Progressive Enhancement**: Core functionality works without JavaScript
- **Accessibility**: Screen reader support on all platforms

## Technical Architecture

### Frontend Stack
- **SvelteKit 2.x**: Modern web framework with server-side rendering
- **TypeScript**: Type-safe development with strict type checking
- **TailwindCSS 4.0**: Utility-first CSS framework with design system
- **Vite 7.x**: Fast build tool with hot module replacement

### State Management
- **Svelte Stores**: Reactive state management for auth, i18n, theme
- **IndexedDB Session Management**: Enterprise-grade session storage with cross-tab synchronization
- **LocalStorage Integration**: Persistent storage for user preferences (theme, language)
- **URL State Sync**: Parameters synchronized with browser URL via encryption
- **Form State Persistence**: Automatic form state preservation across browser sessions

### Build & Deployment
- **SPA Build**: Static build for deployment to any static hosting
- **Asset Optimization**: Optimized images, icons, and resources
- **Progressive Loading**: Lazy loading of non-critical resources
- **Cache Headers**: Optimized caching strategy for static assets

---

*For internationalization features, see [Internationalization Documentation](./internationalization.md)*  
*For component architecture, see [Components Documentation](./components.md)*  
*For deployment options, see [Production Deployment](../deployment/production.md)*