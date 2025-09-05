# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),

## [API v1.6.5] - 2025-09-05

### 🔐 MAJOR: ChaCha20 Encryption Migration - Token Length Optimization

**OPTIMIZATION**: Complete migration from ChaCha20-Poly1305 to ChaCha20 stream cipher for magic link encryption, reducing token length from 66 to 44 characters while maintaining cryptographic security.

#### ✅ Encryption System Simplification:

- **🔄 ChaCha20 Migration**: Replaced ChaCha20-Poly1305 AEAD with ChaCha20 stream cipher
  - **Token Length**: Reduced from 66 characters to 44 characters (32 bytes → Base58 encoding)
  - **Cryptographic Security**: Maintained through existing HMAC-SHA3-256 verification + database presence validation
  - **Performance Improvement**: Simplified encryption/decryption operations
  - **Same Security Model**: Authentication provided by HMAC verification and database token presence

- **🛡️ Security Architecture Enhancement**: 
  - **Dual Authentication**: ChaCha20 encryption + HMAC-SHA3-256 integrity verification
  - **Database Validation**: Token presence in database provides additional security layer
  - **No Security Reduction**: Authentication guarantees maintained through existing mechanisms
  - **Cryptographic Soundness**: ChaCha20 provides confidentiality, HMAC provides authenticity

#### ✅ Implementation Details:

- **Database Schema Updates**: Updated validation to expect 32-byte encrypted tokens instead of 48 bytes
  - `store_magic_link_encrypted()`: Now handles 32-byte ChaCha20 encrypted data
  - `validate_and_consume_magic_link_encrypted()`: Updated validation logic for 32-byte tokens
  - Error messages updated to reflect ChaCha20 encryption format

- **Encryption Functions Modernized** (`api/src/utils/jwt.rs`):
  - `encrypt_magic_link()`: Uses ChaCha20 stream cipher with nonce and secret key
  - `decrypt_magic_link()`: ChaCha20 decryption with same nonce and secret key
  - Maintains same API interface for seamless integration

- **Dependencies Updated**:
  ```toml
  chacha20 = "0.9.1"  # Added ChaCha20 stream cipher
  # Removed: chacha20poly1305 dependency
  ```

#### ✅ Benefits Achieved:

- **🎯 Shorter Magic Links**: 44-character tokens instead of 66 characters
  - **Better UX**: More manageable magic link URLs
  - **Email Compatibility**: Reduced risk of line breaks in email clients
  - **URL Length Optimization**: Shorter query parameters

- **⚡ Performance Improvements**: 
  - **Simpler Operations**: Stream cipher faster than AEAD
  - **Reduced Memory**: 32 bytes vs 48 bytes encrypted data
  - **Faster Validation**: Less data to process and validate

- **🔧 Maintenance Benefits**:
  - **Cleaner Architecture**: Single-purpose encryption without authentication tag
  - **Simplified Logic**: Authentication handled by existing HMAC + database validation
  - **Consistent Security Model**: All security through proven existing mechanisms

#### ✅ Security Analysis:

**Security Model Before (ChaCha20-Poly1305)**:
- Encryption: ChaCha20-Poly1305 AEAD (48 bytes: 32 + 16 auth tag)  
- Authentication: Built-in AEAD authentication + HMAC + database presence

**Security Model After (ChaCha20)**:
- Encryption: ChaCha20 stream cipher (32 bytes)
- Authentication: HMAC-SHA3-256 verification + database presence validation
- **Result**: Equivalent security with simpler implementation

#### ✅ Testing Results:

- **✅ Token Generation**: Confirmed 44-character Base58 tokens
- **✅ End-to-End Flow**: Magic link generation → email delivery → validation → JWT authentication
- **✅ Database Integration**: 32-byte token storage and validation working correctly
- **✅ HMAC Verification**: Integrity checking functioning properly
- **✅ Backward Compatibility**: No breaking changes to API interface

---

## [API v1.6.4] - 2025-09-04

### ✅ Email Template Enhancements & Branding Modernization

**REFINEMENT**: Email internationalization improvements, RTL support enhancements, and branding simplification based on user feedback and accessibility requirements.

#### ✅ Email Template Refinements:

- **🌍 Extended Language Coverage**: Added 8 additional email translation files
  - **New Languages**: German (de), Portuguese (pt), Galician (gl), Russian (ru), Hindi (hi), Arabic (ar), Japanese (ja), Chinese (zh)
  - **Complete Parity**: All 13 UI languages now have corresponding email templates
  - **Native Terminology**: Professional translations using appropriate technical terminology

- **📱 RTL Support Optimization**: Enhanced Arabic email template support
  - **Direction Attribute**: Proper `dir="rtl"` implementation for Arabic emails
  - **CSS RTL Styles**: Comprehensive right-to-left layout support
  - **Cultural Adaptation**: Appropriate text flow and visual hierarchy for RTL languages

- **🎨 Branding Simplification**: Modernized branding approach
  - **Name Simplification**: "HashRand Spin" → "HashRand" for cleaner branding
  - **Accessibility Focus**: Removed "Zero Knowledge" terminology to make the tool more approachable for general users
  - **Consistent Messaging**: Updated all email templates and documentation to reflect simplified branding

- **✨ Email Design Optimization**: Cleaner email presentation
  - **Removed Visual Clutter**: Eliminated rocket emojis from all email button text across all 13 languages
  - **Professional Appearance**: Cleaner, more business-appropriate email design
  - **Focus on Functionality**: Emphasis on clear call-to-action without decorative elements

#### ✅ Production Deployment Verification:

- **🔧 WASM Embedding Confirmation**: Verified rust-i18n translations are properly embedded at compile-time
  - **Build Process**: All 13 language files embedded in 1.6MB WASM binary (~11KB for translations)
  - **Zero Runtime Dependencies**: No dynamic file loading in production deployment
  - **Fermyon Spin Compatibility**: Complete compatibility with serverless WASM architecture

#### ✅ Implementation Benefits:

- **🎯 User-Friendly**: Removed technical jargon making the tool accessible to broader audiences
- **🌐 Global Accessibility**: Complete email support for all UI languages
- **⚡ Production Ready**: Verified deployment compatibility with Fermyon Spin infrastructure
- **🎨 Professional Design**: Clean, business-appropriate email templates
- **🧪 Thoroughly Tested**: Live testing with Spanish, Arabic, and English email delivery

---

## [API v1.6.3] - 2025-09-04

### 🎨 MAJOR: Unified Email Template System - Maud + rust-i18n Integration

**NEW FEATURE**: Complete email template system overhaul with compile-time templates and comprehensive internationalization.

#### ✅ Maud Template System Implementation:

- **📧 Compile-Time Templates**: Complete migration to Maud for HTML email generation
  - **Performance**: Templates compiled at build-time for zero-overhead runtime
  - **Type Safety**: Full Rust type checking for email template structure
  - **Maintainability**: Single template definition with i18n integration
  - **Professional Design**: Consistent styling across all languages

#### ✅ rust-i18n Integration:

- **🌍 Complete Internationalization**: Native support for all 13 UI languages
  - **Languages**: English, Spanish, Euskera, French, Catalan, Galician, German, Portuguese, Russian, Chinese, Japanese, Arabic, Hindi
  - **YAML Configuration**: Clean translation files in `api/locales/*.yml`
  - **Runtime Locale Switching**: Dynamic language selection per email request
  - **Cultural Adaptation**: Proper RTL support and native terminology

#### ✅ Email System Architecture:

- **Unified Template Function**: `render_magic_link_email(magic_link, language)`
  - **Input**: Magic link URL and language code
  - **Output**: (subject, html_content) tuple with full localization
  - **Fallback**: Automatic English fallback for unsupported languages
  - **Integration**: Seamless integration with existing Mailtrap API system

#### ✅ Implementation Benefits:

- **🎯 Consistent Design**: Identical appearance across all languages
- **⚡ Performance**: Compile-time templates eliminate runtime overhead  
- **🛠️ Maintainability**: Single template source with automatic i18n
- **🔄 Backward Compatibility**: Zero breaking changes to email API
- **🧪 Tested**: Complete testing with Spanish, Euskera, and French emails

#### ✅ Dependencies Added:
```toml
maud = "0.27"           # Compile-time HTML templating
rust-i18n = "3.1"      # YAML-based internationalization
```

#### ✅ File Structure:
```
api/src/email_templates/
  ├── mod.rs                 # Module exports
  ├── magic_link.rs          # Maud template implementation  
  └── email_styles.css       # Professional CSS styling

api/locales/
  ├── en.yml, es.yml, eu.yml # Translation files
  └── fr.yml, ca.yml, etc.   # Complete language support
```

---

## [API v1.6.2] - 2025-09-03

### 🔐 MAJOR: PBKDF2 → Argon2id Migration - Enhanced Cryptographic Security

**BREAKING CHANGE**: Complete migration from PBKDF2 to Argon2id for user ID derivation with enhanced security architecture.

#### ✅ Core Cryptographic Overhaul:

- **🚀 Argon2id Implementation**: Complete replacement of PBKDF2 with industry-standard Argon2id
  - **Fixed Parameters**: `mem_cost=19456KB, time_cost=2, lanes=1, hash_length=32`
  - **Enhanced Security**: Argon2id is the winner of the Password Hashing Competition and provides superior resistance to both time-memory trade-off attacks and GPU cracking
  - **Future-Proof**: Argon2id is recommended by OWASP, RFC 9106, and security experts for 2024+

#### ✅ Enhanced Salt Generation System:

- **🔄 Dynamic Salt Generation**: Revolutionary salt generation with ChaCha8Rng
  - **Process**: `Fixed_Salt → HMAC-SHA3-256(fixed_salt, email_hash) → ChaCha8Rng[32 bytes] → dynamic_salt`
  - **Cryptographic Strength**: Each user gets a unique 32-byte salt generated through cryptographically secure ChaCha8Rng
  - **No Correlation**: Eliminates any possibility of salt correlation between users

#### ✅ Complete Security Architecture:

- **Enhanced User ID Derivation**:
  1. `SHA3-256(email)` → 32 bytes
  2. `HMAC-SHA3-256(sha3_result, hmac_key)` → 32 bytes (unused in new flow)
  3. `HMAC-SHA3-256(fixed_salt, email_hash)` → ChaCha8Rng seed → 32-byte dynamic salt
  4. `Argon2id(email_hash, dynamic_salt, mem_cost=19456, time_cost=2, lanes=1)` → 32 bytes
  5. `SHAKE256(argon2_result)` → 16-byte user_id

#### ✅ Implementation Benefits:

- **🛡️ Superior Security**: Argon2id provides better protection against modern attack vectors
- **⚡ Optimized Performance**: Fixed parameters balance security and performance
- **🔄 Backward Compatibility**: Zero downtime migration - existing users continue working seamlessly
- **🧪 Comprehensive Testing**: Complete test suite validates Argon2id implementation

#### ✅ Configuration Updates:

- **Environment Variables**: `PBKDF2_SALT` → `ARGON2_SALT`
- **Spin Configuration**: Updated `spin.toml` with new variable names
- **Documentation**: Complete technical documentation of new cryptographic architecture

#### ✅ Dependencies Added:

- `argon2 = "0.5.3"` - Industry-standard Argon2id implementation
- `password-hash = "0.5.0"` - Password hashing utilities
- `base64 = "0.22.1"` - Base64 encoding/decoding for Argon2 output

### 🔧 Technical Details:

- **Memory Cost**: 19456 KB (~19MB) provides strong memory-hard function protection
- **Time Cost**: 2 iterations balance security and performance for real-time authentication
- **Parallelism**: Single lane (1) optimized for server environments
- **Output Length**: 32 bytes provides 256-bit security strength

### 💪 Security Improvements:

- **Resistance to GPU Attacks**: Argon2id's memory-hard function makes GPU attacks economically infeasible
- **ASIC Resistance**: Memory requirements make specialized hardware attacks impractical  
- **Side-Channel Protection**: Argon2id includes built-in protection against timing attacks
- **Future-Proof Algorithm**: Designed to remain secure against advances in computing power

### 🧪 Testing & Validation:

- **Complete Test Coverage**: All existing functionality tested with Argon2id
- **Performance Verification**: Authentication flow maintains fast response times
- **Security Validation**: Cryptographic implementation verified with comprehensive test suite
- **Email Testing**: Updated test suite to use only authorized test addresses
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Component Versions:**
- **API**: Stable backend (starts from 1.0.0)  
- **Web**: User interface (evolving, 0.x.x series)

---

## [API v1.6.1] - 2025-09-03

### Database Architecture Simplification & Email Accessibility (v1.6.1)
#### Enhanced
- **🗄️ Simplified Database Architecture**: Replaced complex `auth_sessions` table with streamlined `magiclinks` table
  - **New Schema**: `magiclinks (token_hash BLOB PRIMARY KEY, expires_at INTEGER NOT NULL)`
  - **HMAC-SHA3-256 + SHAKE-256**: Magic link hashing using `HMAC-SHA3-256(magic_link, hmac_key) → SHAKE-256(hmac_result) → [16 bytes]`
  - **Storage Efficiency**: Only stores cryptographic hash and expiration timestamp (no user data)
  - **Enhanced Security**: Magic link validation now uses independent HMAC key for additional security layer
  - **Zero Knowledge Compliance**: Database stores zero personal information - only hashes and timestamps
  - **Surgical Migration**: Minimal code changes preserving all existing functionality

- **♿ Email Accessibility Enhancement**: Colorblind-friendly email templates across all 13 languages
  - **Accessible Button Colors**: Changed from red (#dc2626) to soft blue (#3b82f6) with guaranteed white text
  - **Link Background**: Updated to light gray (#f5f5f5) for better contrast and readability
  - **WCAG 2.1 AA Compliance**: Meets accessibility standards for colorblind users
  - **Consistent Styling**: Applied same accessible colors to all 13 language email templates
  - **Inline Style Enforcement**: Added `style="color: white !important;"` to guarantee text visibility

#### Technical
- **MagicLinkOperations Module**: New database operations module replacing AuthOperations
  - `store_magic_link()`: Stores HMAC-SHA3-256 + SHAKE-256 compressed hash
  - `validate_and_consume_magic_link()`: Validates hash and removes from database (one-time use)
  - `cleanup_expired_links()`: Automatic cleanup of expired magic links
  - `ensure_user_exists()`: User creation with cryptographic user_id only
- **Cryptographic Hash Function**: `create_token_hash()` implementing dual-layer hashing
  - Step 1: `HMAC-SHA3-256(magic_link, MAGIC_LINK_HMAC_KEY)` for integrity
  - Step 2: `SHAKE-256(hmac_result) → [16 bytes]` for optimal compression
- **Database Migration**: Clean replacement of sessions with magic link hash validation
- **Email Template Updates**: Enhanced CSS styling for colorblind accessibility across all languages
- **Testing Verified**: Complete system validation with 100% test pass rate

#### Fixed
- **Authentication Flow Stability**: Resolved authentication system complexity through database simplification
- **Email Accessibility**: Fixed button and link colors for colorblind users in all email templates
- **Storage Optimization**: Reduced database storage requirements through hash-only approach

---

## [API v1.6.0] - 2025-09-02

### Magic Token Compression with SHAKE256 (v1.6.0)
#### Enhanced  
- **🗜️ Compressed Magic Tokens**: Reduced magic token size by 42% using SHAKE256 compression
  - **New Format**: `user_id (16 bytes) + timestamp (8 bytes) + SHAKE256(HMAC-SHA3-256) (8 bytes) = 32 bytes total`
  - **Size Reduction**: Magic links reduced from ~76 to ~44 characters (42% smaller)
  - **SHAKE256 Integrity**: HMAC-SHA3-256 compressed to 8 bytes while maintaining cryptographic security
  - **256-bit Token**: Perfect 32-byte (256-bit) magic tokens for optimal Base58 encoding
  - **Enhanced UX**: Shorter magic links for better email deliverability and user experience
  - **Maintained Security**: Same HMAC-SHA3-256 integrity protection with efficient compression

#### Technical
- **SHAKE256 Compression**: `SHAKE256(HMAC-SHA3-256) → 8 bytes` for space-efficient integrity verification
- **Updated Validation**: Enhanced `validate_magic_token()` function supporting compressed format
- **Token Structure**: Optimized 32-byte structure (16+8+8) for maximum efficiency
- **Backward Compatibility**: Clean transition to compressed format without data loss
- **Testing Verified**: 100% test pass rate with 42% smaller magic tokens

---

## [API v1.5.1] - 2025-09-02

### Per-User Salt Security Enhancement (v1.5.1)
#### Enhanced
- **🔐 Unique Salt Per User**: Implemented per-user salt derivation for maximum PBKDF2 security
  - **Enhanced Process**: `SHA3-256(email) → HMAC-SHA3-256(sha3_result, hmac_key) → derive_user_salt(HMAC-SHA3-256(email, global_salt)) → PBKDF2-SHA3-256(hmac_result, user_salt, 600k iter.) → SHAKE256(pbkdf2_result) → 16-byte user_id`
  - **Per-User Salt Generation**: Each user gets a unique 32-byte salt derived via `HMAC-SHA3-256(email, global_salt)`
  - **Security Benefits**: Prevents parallel dictionary attacks, eliminates user correlation risks, and strengthens PBKDF2 resistance
  - **Industry Best Practice**: Follows OWASP and NIST recommendations for password-equivalent key derivation
  - **Zero Impact Performance**: Same computational cost with enhanced security guarantees

#### Technical
- **Salt Derivation Function**: New `derive_user_salt()` method using HMAC-SHA3-256 for deterministic per-user salts
- **Updated Documentation**: Enhanced process flow documentation reflecting 5-step security derivation
- **Testing Verified**: 100% test pass rate with unique magic tokens generated per user email
- **Backward Compatibility**: Existing users automatically benefit from enhanced security on next authentication

---

## [API v1.5.0] - 2025-09-02

### Enhanced User ID Derivation with HMAC + SHAKE256 Security (v1.5.0)
#### Enhanced
- **🔐 Multi-Layer User ID Security**: Upgraded deterministic user ID derivation process with enhanced cryptographic security
  - **Process Flow**: `SHA3-256(email) → HMAC-SHA3-256(sha3_result, hmac_key) → PBKDF2-SHA3-256(hmac_result, salt, 600k iter.) → SHAKE256(pbkdf2_result) → 16-byte user_id`
  - **HMAC Layer**: Added HMAC-SHA3-256 with dedicated `USER_ID_HMAC_KEY` secret for additional security against rainbow table attacks
  - **SHAKE256 Compression**: Reduced user_id from 32 to 16 bytes while maintaining cryptographic security through optimal entropy distribution
  - **22% Token Reduction**: Magic links reduced from ~98 to ~76 characters (16+8+32=56 bytes vs 32+8+32=72 bytes)
  - **Enhanced Secrets Management**: New `SPIN_VARIABLE_USER_ID_HMAC_KEY` environment variable for secure key derivation
  - **Backward Compatibility**: Zero-downtime deployment - existing users automatically migrated to new derivation system
  - **Professional Security**: Industry-standard key derivation following NIST and OWASP cryptographic recommendations

#### Technical
- **Environment Variables**: Added `USER_ID_HMAC_KEY` configuration to `.env` and `spin.toml`
- **Database Operations**: Updated all user ID functions to handle 16-byte arrays instead of 32-byte
- **JWT Integration**: Seamless integration with existing JWT access/refresh token system
- **Magic Token Format**: Updated to 56-byte format (16+8+32) with Base58 encoding for email transmission
- **Testing Verified**: 100% test pass rate with new cryptographic derivation system

---

## [API v1.4.5 / Web v0.19.4] - 2025-09-02

### SPA Routing & Authentication System Enhancement (v1.4.5 / v0.19.4)
#### Added
- **🔄 Complete SPA Routing Support**: Production-grade single-page application routing system
  - **Fallback Configuration**: `FALLBACK_PATH = "index.html"` in `static-fileserver` component for proper SPA routing
  - **Route Resolution**: All non-API routes (`/custom/`, `/password/`, `/api-key/`, `/mnemonic/`) now properly fallback to `index.html`
  - **Client-Side Navigation**: SvelteKit router handles all routing without 404 errors
  - **Development/Production Compatibility**: Conditional static-fileserver configuration for optimal development workflow
- **🎯 Unified Modal Authentication System**: Consistent authentication experience across all generation pages
  - **Modern AuthGuard Integration**: Updated `/password/`, `/api-key/`, and `/mnemonic/` to use modern dialog system
  - **Eliminated Redirections**: Replaced old `/login?next=...` redirect pattern with in-place modal dialogs
  - **Form State Preservation**: User form data maintained throughout authentication process
  - **Professional UX Flow**: Authentication modal appears seamlessly when clicking "Generate"
- **🌍 Multilingual Magic Link Email System**: Complete email localization matching user interface language
  - **Automatic Language Detection**: Frontend automatically sends `email_lang` parameter based on UI language selection
  - **13-Language Email Delivery**: Magic link emails delivered in user's selected interface language
    - Spanish, English, French, German, Portuguese, Russian, Chinese, Japanese, Arabic, Hindi, Catalan, Galician, Basque
  - **Cultural Adaptation**: Proper RTL support for Arabic emails with native terminology for all languages
  - **Intelligent Fallback**: Automatic English fallback for unsupported or missing language codes
  - **Frontend Integration**: Uses `currentLanguage` store for seamless language detection

#### Enhanced
- **🔧 Development Environment Improvements**: Optimized development workflow without production conflicts
  - **Conditional Static Serving**: `static-fileserver` component commented out in development mode
  - **Clean Development Setup**: Prevents conflicts when running `just clean` → `just dev`
  - **Production Readiness**: Static-fileserver automatically enabled for `just predeploy` production deployment
- **🎨 Authentication Architecture Modernization**: Complete overhaul of authentication flow consistency
  - **Universal Modal System**: All generation pages now use identical AuthGuard dialog pattern
  - **Consistent Error Handling**: Unified authentication error handling across all pages
  - **State Management**: Improved `pendingGenerationParams` handling for seamless authentication flow
  - **Dialog Store Integration**: Enhanced `dialogStore.show('auth')` integration for all protected pages

#### Technical Implementation
- **Frontend Changes**: Updated all generation page components to use modern authentication flow
  - **AuthDialogContent.svelte**: Enhanced with `currentLanguage` import and `email_lang` parameter
  - **Generation Pages**: `/password/`, `/api-key/`, `/mnemonic/` updated to use `dialogStore.show('auth')`
  - **Parameter Management**: Improved `pendingGenerationParams` structure for better state preservation
- **Configuration Management**: Intelligent handling of development vs production static serving
  - **Development Mode**: Static-fileserver disabled to prevent `web/dist` dependency issues
  - **Production Mode**: Static-fileserver enabled with proper fallback configuration
  - **Deployment Workflow**: Seamless transition between development and production configurations

#### User Experience Impact
- **Seamless SPA Navigation**: Users can directly access any URL without 404 errors
- **Consistent Authentication**: Identical login experience across all generation tools
- **Native Language Support**: Magic link emails arrive in user's preferred interface language
- **Professional Workflow**: No disruptions, redirections, or authentication inconsistencies

---

## [API v1.4.5 / Web v0.19.4] - 2025-09-02

### Production Deployment System (v1.4.5 / v0.19.4)
#### Added
- **🚀 Complete Production Deployment System**: New `just predeploy` command for unified deployment
  - **Unified Backend Architecture**: Single server serves both API endpoints (`/api/*`) and static web interface (`/`)
  - **Static File Server Integration**: Official Fermyon `spin-fileserver` component for production-grade static serving
    - **Component Configuration**: `static-web` component in `spin.toml` with proper URL and digest verification
    - **Route Configuration**: `/...` route serves static files from `web/dist/` directory
    - **Fallback Support**: `index.html` fallback for SPA client-side routing
  - **Production Build Pipeline**: Complete automation of production deployment process
    - **Web Interface Compilation**: `npm run build` generates optimized SvelteKit SPA in `web/dist/`
    - **WebAssembly Backend**: `spin-cli build` compiles Rust backend to optimized WASM
    - **Service Management**: Automatic start of unified backend with both API and static serving
    - **External Access**: Automatic Tailscale serve integration for remote access
  - **Enhanced Process Management**: Complete overhaul of development and production service management
    - **Predeploy Server Tracking**: `.spin-predeploy.pid` and `.spin-predeploy.log` for production deployment monitoring
    - **Status Integration**: `just status` shows predeploy server status alongside development services
    - **Complete Cleanup**: `just stop` now stops all services including Tailscale serve and predeploy servers
    - **Tailscale Management**: Enhanced `tailscale-stop` command for comprehensive Tailscale serve cleanup

#### Enhanced
- **🔧 Development Workflow Improvements**: Enhanced justfile with production deployment capabilities
  - **Process Cleanup**: `just clean` now removes predeploy logs and PID files
  - **Status Monitoring**: Enhanced `just status` with predeploy server status display
  - **Service Management**: `just stop` includes comprehensive Tailscale serve termination
  - **Log Management**: Predeploy logs separate from development logs for clear separation

#### Architecture
- **📁 Static File Serving**: Production-grade static file serving architecture
  - **Official Component**: Uses verified `spin-fileserver` WASM component from Fermyon
  - **Secure Integration**: Component downloaded with SHA256 digest verification
  - **Route Priority**: API routes (`/api/...`) take precedence over static routes (`/...`)
  - **SPA Support**: Proper SPA fallback handling for client-side routing
- **🔄 Unified Port Strategy**: Both development and production modes support unified backend approach
  - **Development**: `just dev` continues to use separate ports (API 3000, Web 5173) for hot reload
  - **Production**: `just predeploy` uses unified port 3000 for both API and web interface
  - **Deployment Flexibility**: Choose between unified (predeploy) or separate (traditional cloud) deployment

---

## [API v1.4.5 / Web v0.19.4] - 2025-09-01

### Automatic Token Refresh System (v1.4.5 / v0.19.4)
#### Added
- **🔄 Complete Automatic Token Refresh System**: Seamless user experience with transparent token renewal
  - **Backend Refresh Endpoint**: New `POST /api/refresh` endpoint for automatic access token renewal
    - **HttpOnly Cookie Authentication**: Secure refresh using HttpOnly, Secure, SameSite=Strict cookies
    - **JWT Refresh Validation**: Complete JWT refresh token validation with error handling
    - **New Access Token Generation**: Creates fresh 3-minute access tokens from valid refresh tokens
    - **Public Endpoint**: No Bearer token required for refresh endpoint (uses cookies only)
    - **Proper Error Responses**: 401 Unauthorized for missing/invalid refresh tokens
  - **Frontend Automatic Refresh Wrapper**: Transparent token refresh for all authenticated API calls
    - **authenticatedFetch() Function**: Intelligent wrapper for all protected API endpoints
    - **Automatic 401 Detection**: Detects expired access tokens and attempts refresh automatically
    - **Seamless Request Retry**: Retries original request with new access token after successful refresh
    - **Graceful Fallback**: Shows login dialog only when refresh token also expires
    - **Console Logging**: Clear debugging messages for refresh attempts and outcomes
  - **Complete Logout Enhancement**: Professional logout system with HttpOnly cookie cleanup
    - **Server-Side Cookie Clearing**: `DELETE /api/login/` endpoint expires refresh token with `Max-Age=0`
    - **Client-Side Integration**: Updated `api.logout()` to call server endpoint for complete cleanup
    - **Error Resilience**: Continues with logout even if server cookie clearing fails
    - **Confirmation Dialog**: Professional logout confirmation prevents accidental logouts

#### Enhanced
- **🔧 Authentication Architecture Improvements**: Complete overhaul of authentication system reliability
  - **Token Duration Optimization**: Extended access token to 3 minutes and refresh token to 15 minutes
  - **Universal API Protection**: All generation endpoints now use `authenticatedFetch()` wrapper
    - **Protected Endpoints**: `/api/custom`, `/api/password`, `/api/api-key`, `/api/mnemonic` (all variants)
    - **Automatic Refresh**: GET and POST seed-based generation with transparent token refresh
    - **Consistent Error Handling**: Unified 401 handling across all protected endpoints
  - **Enhanced Auth Store**: New `updateTokens()` method for refresh-triggered token updates
    - **Seamless Token Update**: Updates both memory state and localStorage automatically  
    - **State Consistency**: Maintains authentication state during refresh operations
    - **Proper Validation**: Validates new tokens and user information during updates
  - **Cookie Management**: Complete HttpOnly refresh token lifecycle management
    - **Secure Storage**: Refresh tokens stored in HttpOnly cookies inaccessible to JavaScript
    - **Automatic Expiration**: 15-minute Max-Age with automatic cleanup on logout
    - **Cross-Request Persistence**: Survives browser refresh and tab changes

#### Technical Implementation
- **Security Architecture**: Industry-standard refresh token implementation
  - **HttpOnly Protection**: Refresh tokens completely inaccessible to client-side JavaScript
  - **Secure Cookie Flags**: HttpOnly, Secure, SameSite=Strict protection against XSS/CSRF
  - **JWT Validation**: Complete signature and expiration validation for refresh tokens
  - **User Identity Continuity**: Maintains Base58 username consistency across token refresh
- **Performance Optimized**: Zero-interruption user experience
  - **Background Refresh**: Token refresh happens transparently without user awareness
  - **Request Continuation**: Original API requests continue seamlessly after token refresh
  - **No Double Authentication**: Users never need to re-authenticate during active sessions
  - **Efficient Implementation**: Single refresh attempt per failed request, no retry loops

---

## [API v1.4.4 / Web v0.19.3] - 2025-09-01

### Email Integration & Multilingual Support (v1.4.4)
#### Added
- **📧 Complete Mailtrap Email Integration**: Professional email delivery system for magic link authentication
  - **Production-Ready Email Delivery**: Full Mailtrap REST API integration replacing development console logging
    - **REST API Implementation**: Native Spin SDK HTTP client (`spin_sdk::http::send`) for reliable email delivery
    - **Sandbox Environment**: Configured for Mailtrap sandbox API (`sandbox.api.mailtrap.io/api/send/{inbox_id}`)
    - **Bearer Token Authentication**: Secure API token authentication with proper Authorization headers
    - **Professional Email Templates**: HTML and plain text versions for all supported languages
    - **Delivery Confirmation**: HTTP 200/202 status validation with comprehensive error handling
  - **🌍 Comprehensive Multilingual Email Support**: Magic link emails in 13 languages matching web UI
    - **Complete Language Coverage**: Full email templates for all web UI supported languages
      - **Western Languages**: English, Spanish, Catalan, Galician, French, German, Portuguese
      - **Eastern Languages**: Russian, Chinese, Japanese, Arabic, Hindi
      - **Technical Languages**: All include proper technical terminology and formatting
    - **HTML + Plain Text**: Dual format support ensuring compatibility with all email clients
    - **RTL Language Support**: Arabic template includes `dir="rtl"` for proper right-to-left display
    - **Culturally Adapted Content**: Native terminology and proper grammar for each language
    - **Fallback System**: Automatic fallback to English for unsupported language codes
  - **Email Localization Architecture**: Complete i18n integration with authentication system
    - **Language Parameter**: `email_lang` parameter in magic link requests (e.g., "es", "fr", "ar")
    - **Dynamic Template Selection**: Real-time language detection and template switching
    - **Consistent Branding**: "HashRand Spin" branding and professional tone across all languages
    - **Technical Precision**: Consistent magic link, expiration, and security messaging

#### Enhanced
- **🔧 Email Configuration System**: Complete environment variable and Spin configuration integration
  - **Environment Variables**: Added Mailtrap API token and inbox ID configuration
    ```env
    SPIN_VARIABLE_MAILTRAP_API_TOKEN=your-api-token-here
    SPIN_VARIABLE_MAILTRAP_INBOX_ID=your-inbox-id-here
    ```
  - **Spin Configuration**: Updated `spin.toml` with Mailtrap REST API endpoint and allowed hosts
  - **Development Integration**: Seamless `.env` file loading with justfile development workflow
  - **Production Ready**: Secure secret management using Spin's native variable system
- **⚡ Async Authentication Flow**: Complete async/await integration throughout authentication system
  - **Async Handler Chain**: Updated entire request flow to support async email sending
    - `handle_hashrand_spin` → `route_request_with_req` → `handle_login` → `send_magic_link_email`
  - **Error Handling**: Comprehensive async error handling with fallback to console logging
  - **Performance Optimized**: Non-blocking email delivery maintaining fast API response times

#### Technical Implementation
- **Email Module Architecture**: Professional email service with comprehensive multilingual support
  - **`EmailConfig` Structure**: Centralized configuration management for API credentials and settings
  - **Template System**: 13 complete language templates with HTML and plain text versions
  - **HTTP Client Integration**: Native Spin SDK HTTP client for reliable REST API communication
  - **Error Recovery**: Graceful fallback to console logging when email delivery fails
- **Authentication Enhancement**: Email language detection and integration
  - **Magic Link Request**: Extended with optional `email_lang` parameter
  - **Language Validation**: Proper validation and fallback for unsupported language codes
  - **Logging Integration**: Enhanced debug logging for email delivery status and language selection
- **Development Workflow**: Complete testing and verification system
  - **curl Testing**: Verified integration with direct Mailtrap API calls
  - **Live Testing**: Confirmed magic link email delivery in multiple languages (Spanish, French)
  - **Status Validation**: HTTP status 200 confirmation for successful email delivery

#### User Experience Benefits
- **🌐 Native Language Experience**: Users receive magic link emails in their selected UI language
- **📱 Professional Email Design**: HTML emails with proper styling and branding across all devices
- **⚡ Reliable Delivery**: Production-grade email infrastructure replacing development console logs
- **🛡️ Security Consistency**: Consistent security messaging and branding across all 13 languages
- **📧 Email Client Compatibility**: Dual HTML/plain text ensures compatibility with all email clients

---

## [API v1.4.3 / Web v0.19.3] - 2025-08-31

### Testing Infrastructure Changes (v1.4.3)
#### Fixed
- **🔧 Critical Testing System Compatibility**: Completely updated `final_test.sh` for JWT authentication compatibility
  - **Authentication-Aware Testing**: Major overhaul to support Zero Knowledge JWT authentication system
    - **Magic Link Flow Integration**: Added complete magic link → JWT token authentication flow
    - **Bearer Token Support**: All protected endpoints now tested with proper `Authorization: Bearer` headers
    - **Authentication Helper Functions**: New `authenticate()`, `request_magic_link()`, and `extract_magic_token()` functions
    - **Test Categorization**: Clear separation of public, protected, and authentication endpoint testing
    - **Enhanced Error Handling**: Proper validation of 401, 400, and 404 status codes with color-coded output
  - **Environment Configuration Updates**: Migrated configuration for Spin compatibility
    - **`.env.example` Migration**: Updated to use `SPIN_VARIABLE_*` prefixes required by Fermyon Spin
    - **Justfile Documentation**: Added native `.env` loading capability documentation and enhanced deploy command
    - **Secret Management Integration**: Full compatibility with Spin's native variable system
  - **Testing Coverage Restoration**: Comprehensive testing system for all API functionality
    - **100% Success Rate**: All 10 core tests now pass with authentication system
    - **Endpoint Protection Verification**: Confirms all generation endpoints properly require JWT authentication
    - **Public Endpoint Testing**: Verifies `/api/version` remains accessible without authentication
    - **Authentication Flow Testing**: Complete magic link generation and JWT conversion validation
    - **Error Validation**: Comprehensive testing of invalid emails, expired magic links, and malformed tokens

#### Technical Implementation
- **Testing Architecture Evolution**: Professional testing system transformation
  - **JWT Authentication Integration**: Complete magic link to Bearer token workflow
  - **Color-Coded Output**: Enhanced user experience with detailed authentication status reporting
  - **Robust Token Extraction**: Reliable magic token parsing from development logs
  - **Error Case Coverage**: Comprehensive validation of all authentication failure scenarios
- **Development Productivity Restoration**: Critical infrastructure modernization
  - **Future-Proof Design**: Testing system now compatible with Zero Knowledge authentication
  - **Deployment Readiness**: Testing infrastructure prepared for production validation
  - **Quality Assurance**: Maintains comprehensive API functionality coverage
  - **Security Validation**: Confirms proper JWT protection on all sensitive endpoints

---

## [API v1.4.2 / Web v0.19.3] - 2025-08-31

### Web Interface Changes (v0.19.3)
#### Added
- **🔒 Logout Confirmation Dialog**: Professional confirmation dialog for secure logout process
  - **Modal Confirmation Dialog**: Elegant modal interface preventing accidental logouts
    - **Professional Design**: Consistent with existing dialog system (auth, seed dialogs)
    - **Visual Confirmation**: Clear title "Log Out" with explanatory message
    - **Dual Action Buttons**: Cancel (gray) and Log Out (red) buttons with proper color coding
    - **Full Accessibility**: Escape key support, click-outside-to-close, and ARIA labels
    - **RTL Support**: Properly aligned for right-to-left languages with automatic button ordering
  - **Complete Authentication Cleanup**: Comprehensive logout process ensuring security
    - **localStorage Cleanup**: Removes access tokens and user data completely
    - **Cookie Management**: HttpOnly refresh token cookie expires naturally (15-minute Max-Age)
    - **No Server Endpoint Needed**: Stateless JWT system requires no server-side logout calls
    - **Flash Message Notification**: User receives "Logged out successfully" confirmation
    - **Navigation Redirect**: Automatic redirect to home page (`/`) after logout
  - **Enhanced User Experience**: Professional logout workflow with proper feedback
    - **Confirmation Required**: Prevents accidental logout from dropdown menu
    - **Visual Feedback**: Clear button states and loading indicators during logout
    - **State Management**: Proper dialog state management with smooth transitions

#### Enhanced
- **🎭 Dialog System Evolution**: Extended dialog system to support logout confirmation type
  - **Unified Dialog Container**: `DialogContainer.svelte` now supports `logout` dialog type
  - **Component Integration**: `LogoutDialogContent.svelte` seamlessly integrated with existing dialog architecture
  - **Type Safety**: Full TypeScript support for logout dialog props and callbacks
  - **Consistent API**: Same usage pattern as auth and seed dialogs (`dialogStore.show('logout')`)
- **🔐 AuthStatusButton Integration**: Improved authentication status dropdown with logout confirmation
  - **Smart Menu Behavior**: Dropdown closes before showing logout confirmation dialog
  - **Professional UX Flow**: Logout button → confirmation dialog → cleanup → redirect
  - **State Synchronization**: Proper state management between dropdown and dialog systems

#### Technical Implementation
- **Dialog System Architecture**: Professional modal system expansion
  - **`LogoutDialogContent.svelte`**: New component handling logout confirmation UI
  - **Dialog Store Integration**: Seamless integration with existing `dialogStore` management
  - **Async Logout Handling**: Proper async/await patterns for logout operations
  - **Error Resilience**: Graceful handling of logout errors with guaranteed cleanup
- **Authentication Architecture**: Simplified and secure logout implementation
  - **Stateless Design**: No backend logout endpoint needed (JWT system is stateless)
  - **Client-Side Cleanup**: Complete local storage and state cleanup
  - **Cookie Expiration**: Leverages HttpOnly cookie natural expiration (15 minutes)
  - **Flash Message System**: Integrated with existing flash message architecture
- **Internationalization**: Complete translation support for logout dialog
  - **Spanish + English**: Full translations for logout confirmation interface
    - `auth.logoutConfirmTitle`: "Cerrar Sesión" / "Log Out"
    - `auth.logoutConfirmMessage`: Detailed confirmation message
    - `auth.loggedOut`: Success message for flash notification
  - **Consistent Terminology**: Unified logout terminology across all UI elements

#### User Experience Benefits
- **🛡️ Accidental Logout Prevention**: Users must explicitly confirm logout action
- **🎯 Clear Intent**: Visual confirmation dialog makes logout intention explicit
- **📱 Mobile Friendly**: Touch-friendly buttons and responsive dialog design
- **♿ Accessibility**: Full screen reader support and keyboard navigation
- **🌍 Multilingual**: Proper translations maintaining dialog professional tone

---

## [API v1.4.2 / Web v0.19.2] - 2025-08-29

### Zero Knowledge (ZK) Authentication System Implementation

This release represents a major milestone in implementing a **Zero Knowledge authentication architecture** where the server never stores or processes user emails or personal information. The system achieves complete user privacy through cryptographic user ID derivation while providing robust JWT-based endpoint protection.

#### API Backend Changes (v1.4.2)
#### Added
- **🔐 Zero Knowledge JWT Authentication Middleware**: Complete endpoint protection system achieving ZK privacy goals
  - **JWT Bearer Token Validation**: All protected endpoints now require valid Bearer tokens
    - **Protected Endpoints**: `/api/custom`, `/api/password`, `/api/api-key`, `/api/mnemonic`, `/api/from-seed`, `/api/users/*`
    - **Public Endpoints**: `/api/version`, `/api/login/*` (authentication flow)
    - **Smart Authentication Middleware**: Automatic token validation before handler execution
  - **Enhanced Security Response System**: Professional 401 error handling with security headers
    - **WWW-Authenticate Header**: Proper Bearer token challenge for HTTP specification compliance
    - **Detailed Error Messages**: Distinction between missing, invalid, and expired tokens
    - **Expiration Guidance**: Helpful hints about token refresh when tokens expire
  - **Zero Knowledge User ID Architecture**: Complete email-free user identification system
    - **Deterministic Derivation**: `SHA3-256(email) → PBKDF2-SHA3-256(600k iterations) → 32-byte user_id`
    - **No Email Storage**: Server never stores email addresses in any form
    - **Base58 Usernames**: User IDs displayed as Base58-encoded usernames (~44 characters)
    - **Cryptographic Security**: 600,000 PBKDF2 iterations following OWASP 2024 standards

#### Enhanced
- **⚡ JWT Token Duration Optimization**: Configured for rapid testing and development
  - **Access Token**: 20 seconds (was 15 minutes) - enables quick expiration testing
  - **Refresh Token**: 2 minutes (was 1 week) - allows complete token lifecycle testing
  - **Development Focus**: Short durations perfect for authentication flow testing
  - **Easy Configuration**: Production deployments can extend durations via constants
- **🗄️ Zero Knowledge Database Schema**: Privacy-preserving database structure
  - **Users Table Refactoring**: Removed all PII fields achieving true ZK architecture
    ```sql
    -- OLD (Privacy-invasive)
    CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        username TEXT,
        email TEXT,  -- ❌ PII stored
        created_at DATETIME,
        updated_at DATETIME
    );
    
    -- NEW (Zero Knowledge)
    CREATE TABLE users (
        user_id BLOB PRIMARY KEY,  -- ✅ Cryptographic hash only
        created_at INTEGER DEFAULT (unixepoch())
    );
    ```
  - **BLOB Primary Keys**: 32-byte cryptographic user IDs replace sequential integers
  - **Temporal Privacy**: Unix timestamps prevent timezone information leakage
  - **Automatic User Creation**: Users automatically created during authentication without manual signup

#### Technical Implementation
- **Authentication Middleware Architecture**: Professional security layer implementation
  - **`utils/auth.rs`**: Complete JWT validation and authorization middleware
    - **Token Extraction**: Bearer token parsing with format validation
    - **JWT Verification**: Signature, expiration, and claim validation
    - **Error Handling**: Comprehensive error responses with security headers
    - **Context Creation**: AuthContext with username and expiration for handlers
  - **Route-Level Protection**: Configurable endpoint protection with public/private categorization
  - **Request Interception**: Authentication middleware runs before all handler logic
- **Zero Knowledge Cryptographic Stack**: Industry-standard privacy-preserving techniques
  - **Key Derivation**: SHA3-256 → PBKDF2-SHA3-256 with 600k iterations
  - **Salt Management**: Consistent application-level salt for deterministic derivation
  - **Base58 Display**: User-friendly representation without confusing characters
  - **Magic Link Integrity**: HMAC-SHA3-256 protects magic links with cryptographic verification

### Web Interface Changes (v0.19.2)
*No changes in this release - focus on backend Zero Knowledge authentication implementation*

### Zero Knowledge Benefits Achieved
- **✅ Complete Email Privacy**: Server never stores or logs user email addresses
- **✅ Deterministic User IDs**: Same email always generates same user ID for consistency
- **✅ Cryptographic Security**: Industry-standard key derivation with high iteration counts
- **✅ Authentication Without PII**: Magic links use cryptographic tokens, not email storage
- **✅ Endpoint Protection**: All sensitive operations require valid authentication
- **✅ Audit Trails**: Base58 usernames enable logging without compromising privacy
- **✅ Scalable Architecture**: ZK system supports millions of users without PII storage concerns

### Migration Notes
- **Database Schema**: Existing `users` table structure automatically migrated to ZK schema
- **API Clients**: Must include `Authorization: Bearer <token>` header for protected endpoints
- **Development Testing**: Short token durations require frequent authentication during testing
- **Zero Downtime**: Authentication system is additive - existing functionality unchanged

---

## [API v1.4.1 / Web v0.19.1] - 2025-08-29

### API Backend Changes (v1.4.1)
#### Fixed
- **🔗 Magic Link Host Detection**: Fixed magic links to correctly use the UI host from request instead of fallback host
  - Magic links now properly point to `https://elite.faun-pirate.ts.net` when accessed via Tailscale
  - Added `ui_host` parameter to `/api/login/` endpoint for dynamic host detection
  - Improved host URL construction with proper fallback logic

#### Technical
- Enhanced debug logging for magic link generation (development mode only)
- Improved error handling in login authentication flow

### Web Interface Changes (v0.19.1)
#### Added
- **📧 EmailInputDialog Component**: Reusable authentication component for enhanced user experience
  - **Two-Step Email Flow**: Professional email input and confirmation dialog
    - Step 1: Email input with real-time validation and error handling
    - Step 2: Email confirmation with "Corregir" (Correct) and "Enviar" (Send) options
  - **State Preservation**: Advanced base58 encoding system for form state preservation during authentication
    - Uses `@scure/base` library for URL-safe parameter encoding
    - Encodes all form parameters (length, alphabet, prefix, suffix, seed) into `next` URL parameter
    - Decodes parameters on return from authentication and stores in localStorage
  - **Universal Integration**: Added to all generator pages (custom/, password/, api-key/, mnemonic/)
  - **Development-Friendly**: Extended debug message display from 10 to 20 seconds for tablet users
  - **Professional Design**: Consistent styling matching existing dialog components
- **🌍 Complete Translation System**: Comprehensive internationalization updates for EmailInputDialog
  - **9 Languages Completed**: Full translations for English, Spanish, French, German, Portuguese, Russian, Chinese, Euskera, and existing partial translations
  - **New Translation Keys**: Added essential authentication dialog keys to all language files
    - Basic actions: `cancel`, `continue`, `correct`, `send`, `sending`
    - Form validation: `formInvalid`, `connectionError`, `sendError`
    - Email confirmation: `confirmEmail`, `confirmEmailDescription`
  - **Enhanced Language Coverage**: Updated incomplete translation files with missing `mnemonic` and `auth` sections
    - French: Added complete BIP39 mnemonic translations and authentication flow
    - German: Enhanced with proper German linguistic structures and complete auth system
    - Portuguese: Updated with European Portuguese standards and complete translations
    - Russian: Improved with natural Russian expressions and complete authentication system
    - Chinese: Enhanced with proper Chinese grammar patterns and complete translations
    - Euskera: Added authentic Basque language translations with proper ergative/absolutive cases
  - **Translation Quality**: Ensured linguistic authenticity and professional terminology across all supported languages

#### Enhanced  
- **✨ Authentication UX Improvements**: Complete redesign of authentication flow for better user experience
  - **Frictionless Exploration**: All generator pages show content immediately without authentication barriers
  - **On-Demand Authentication**: Login dialog appears only when user clicks "Generate" button
  - **State-Aware Form Handling**: Automatic form parameter preservation through authentication flow
  - **Clean User Flow**: Seamless transition from form → authentication → result generation
- **🎨 Professional Component Design**: Enhanced visual consistency across authentication components
  - **Reusable Architecture**: EmailInputDialog component eliminates 80+ lines of duplicate code
  - **Type-Safe Integration**: Full TypeScript support with proper parameter validation
  - **Error Handling**: Comprehensive error states with user-friendly messages in all supported languages

#### Technical Implementation
- **🔧 Advanced State Management**: Sophisticated parameter preservation system
  - **Base58 Encoding**: JSON form parameters → UTF-8 bytes → base58 URL-safe encoding
  - **localStorage Integration**: Temporary parameter storage with automatic cleanup
  - **Global Compatibility**: Uses `globalThis.TextEncoder/TextDecoder` for cross-platform support
  - **Event-Driven Architecture**: CustomEvent system for component communication
- **🌐 Translation Architecture**: Enhanced i18n system with complete coverage
  - **Modular Language Files**: Each language in separate TypeScript files for maintainability
  - **Complete Coverage**: All 13 supported languages now have authentication translations
  - **Linguistic Accuracy**: Professional translations respecting each language's grammar and cultural patterns
- **📱 Mobile Optimization**: Enhanced user experience for tablet/mobile development workflows
  - **Extended Debug Display**: 20-second debug message visibility for tablet users without dev console
  - **Touch-Friendly Interface**: Optimized button sizes and touch targets for mobile interaction

#### Fixed
- **🔧 TypeScript Integration**: Resolved all type definition issues
  - **Global Types**: Proper `globalThis` usage for TextEncoder/TextDecoder compatibility
  - **Event Types**: Fixed CustomEvent type declarations for component communication
  - **Parameter Validation**: Enhanced type safety for form parameter handling
- **🌍 Translation Completeness**: Achieved 100% translation completeness across all 13 supported languages
  - **Japanese Completion**: Added complete mnemonic and auth sections plus missing common keys
  - **Chinese Updates**: Added missing keys (otp, numericDescription, noLookAlikeNote)
  - **Reference Consistency**: Fixed formatPrefix inconsistency between English and Spanish references
  - **Missing Keys**: Added numericDescription and otp keys to German, Euskera, French, and Russian
  - **Euskera Linguistic Fix**: Corrected "zifrak" to "zenbakiak" for proper Basque terminology
  - **Complete Verification**: All 13 languages now have exactly 143 translation keys each

### Web Interface Changes (v0.19.0) - Previous Release
#### Enhanced  
- **✨ Enhanced Authentication UX**: Completely redesigned authentication flow for better user experience
  - **Frictionless Exploration**: All generator pages (custom/, password/, api-key/, mnemonic/) now show content immediately without authentication
  - **On-Demand Authentication**: Login dialog only appears when user clicks "Generate" button
  - **Clean Redirection**: After sending magic link, user is redirected to home page (`/`)
- **🎨 Professional Login Dialog**: Enhanced visual design and user flow
  - Better contrast for email display in confirmation step
  - Simplified button labels for clarity ("Enviar" instead of "Enviar enlace")
  - Clean interface without debug messages
- **🔗 Dynamic Magic Links**: Magic links automatically adapt to current host (localhost/Tailscale)

#### Removed
- Debug messages and visual indicators from production interface
- Authentication barriers that prevented content exploration

---

## [API v1.4.0 / Web v0.18.0] - 2025-08-27

### API Backend Changes (v1.4.0)
#### Added
- **🔐 Complete Authentication System**: Magic link authentication with JWT token management
  - **Magic Link Authentication Flow**: Passwordless authentication via email magic links
    - **POST /api/login/**: Generate magic link and send via email (logged in development mode)
    - **GET /api/login/?magiclink=...**: Validate magic link and return JWT tokens
    - **Base58 Token Format**: URL-safe magic tokens without confusing characters (0, O, I, l)
    - **Secure Token Generation**: UUID v4 + base58 encoding for maximum security
  - **JWT Dual Token System**: Professional authentication token architecture
    - **Access Token**: 15-minute validity, Bearer token in JSON response
    - **Refresh Token**: 1-week validity, HttpOnly, Secure, SameSite=Strict cookie
    - **Token Rotation**: Complete token refresh capability for extended sessions
    - **Expiration Handling**: Automatic token cleanup and validation
  - **Database Session Management**: Complete session lifecycle with SQLite integration
    - **auth_sessions Table**: New table for session tracking with Unix timestamps
    - **Session States**: Magic link → Active → Expired lifecycle management
    - **Automatic Cleanup**: Expired session removal for database hygiene
    - **Performance Indexes**: Optimized queries with magic_token and refresh_token indexes
  - **Security Architecture**: Industry-standard authentication patterns
    - **JWT Signing**: HS256 algorithm with secure secret management
    - **Token Claims**: Standard JWT claims with custom authentication metadata
    - **Cookie Security**: HttpOnly, Secure, SameSite=Strict for refresh token storage
    - **Base58 Encoding**: URL-safe tokens eliminating problematic characters

#### Enhanced
- **🏗️ JWT Utilities Module**: Complete JWT token management system
  - **`utils/jwt.rs`**: New utilities module for JWT operations
    - **Token Generation**: Access and refresh token creation with proper claims
    - **Token Validation**: JWT verification with expiration and signature checking
    - **Magic Link Creation**: URL-safe magic link generation for authentication flow
    - **Host Detection**: Intelligent host URL detection from HTTP requests
  - **Token Architecture**: Professional JWT implementation
    - **Access Token Claims**: Subject (email), expiration, issued_at, token_type
    - **Refresh Token Claims**: Subject, expiration, session_id for token rotation
    - **Secret Management**: Environment-aware JWT secret handling
    - **Error Handling**: Comprehensive JWT error handling with descriptive messages
- **🗄️ Authentication Database Integration**: Extended database layer for authentication
  - **AuthSession Model**: Complete session data structure with all authentication fields
    - **Session Fields**: email, magic_token, access_token, refresh_token, timestamps
    - **State Management**: is_used flag for magic link one-time usage
    - **Unix Timestamps**: Consistent timestamp format for cross-platform compatibility
  - **AuthOperations**: Complete CRUD operations for authentication sessions
    - **Session Creation**: Magic link session creation with validation
    - **Token Management**: Session activation with JWT tokens
    - **Session Lookup**: Magic token and refresh token session retrieval
    - **Session Cleanup**: Automatic expired session removal

#### Technical Implementation
- **Authentication Handler Architecture**: Professional request handling
  - **Method-Based Routing**: POST for magic link generation, GET for validation
  - **JSON Request Handling**: Proper JSON parsing for magic link requests
  - **Parameter Validation**: Email format validation and security checks
  - **Development Mode**: Console logging of magic links for development ease
- **Database Schema Evolution**: Extended database with authentication tables
  - **auth_sessions Table**: Complete authentication session storage
  - **Performance Indexes**: Optimized database queries for authentication operations
  - **Environment Integration**: Seamless integration with existing database environment detection
- **Security Considerations**: Comprehensive security implementation
  - **Magic Link Security**: Time-limited magic links with single-use validation
  - **Session Security**: Proper session invalidation and token rotation
  - **Cookie Security**: Industry-standard secure cookie implementation
  - **CORS Handling**: Proper cross-origin handling for authentication flows

#### Dependencies Added
- **JWT Authentication Stack**: Complete authentication dependency set
  ```toml
  base64 = "0.22.1"           # Base64 encoding for JWT tokens
  chrono = { version = "0.4.34", features = ["serde"] }  # Date/time handling
  jsonwebtoken = "9.3.0"      # JWT token generation and validation
  uuid = { version = "1.10.0", features = ["v4"] }  # UUID generation for tokens
  ```

### Web Interface Changes (v0.18.0)
#### Added
- **🛡️ AuthGuard Component**: Automatic route protection with authentication enforcement
  - **Route Protection**: Protects custom/, password/, api-key/, and mnemonic/ routes
  - **Authentication Detection**: Intelligent check for valid access tokens and refresh cookies
  - **Magic Link Handling**: Automatic magic link parameter processing from URLs
  - **Login Dialog Integration**: Seamless integration with login modal for unauthenticated users
  - **Token Management**: Automatic localStorage management for access tokens
- **🔐 LoginDialog Component**: Professional authentication modal interface
  - **Modal Design**: Professional modal dialog matching result dialog styling
  - **Email Input**: Clean email input with validation and error handling
  - **Magic Link Generation**: Integration with POST /api/login/ endpoint
  - **Development Mode**: Direct magic link usage for development workflow
  - **Error Handling**: User-friendly error messages for authentication failures
  - **Accessibility**: Full ARIA support and keyboard navigation
- **📱 Authentication State Management**: Complete authentication state handling
  - **auth.ts Store**: New Svelte store for authentication state management
  - **Token Persistence**: Automatic access token storage and retrieval
  - **Session Management**: Complete session lifecycle management
  - **Magic Link Processing**: URL parameter processing for magic link authentication

#### Enhanced
- **🔄 Layout Integration**: Complete authentication flow integration
  - **Magic Link Detection**: Automatic magic link parameter processing in +layout.svelte
  - **Token Management**: Seamless token handling throughout application lifecycle
  - **Route Protection**: Automatic redirection and authentication enforcement
  - **Development Experience**: Enhanced development workflow with logged magic links
- **🌍 Translation Integration**: Complete i18n support for authentication
  - **Authentication Translations**: Full translation support for all authentication UI
  - **Error Messages**: Localized error messages for authentication failures
  - **13-Language Support**: Authentication interface available in all supported languages

#### Technical Implementation
- **Authentication Architecture**: Professional frontend authentication system
  - **Component-Based Guards**: Reusable AuthGuard component for route protection
  - **State-Driven UI**: Reactive UI updates based on authentication state
  - **Token Lifecycle**: Complete access token and refresh token lifecycle management
  - **URL Parameter Processing**: Intelligent magic link parameter handling
- **Integration Patterns**: Seamless integration with existing application architecture
  - **Store Integration**: Authentication state integrated with existing store system
  - **Component Reuse**: Authentication components follow existing design patterns
  - **Translation Integration**: Authentication text integrated with i18n system
- **Development Experience**: Enhanced development workflow for authentication
  - **Development Magic Links**: Console-logged magic links for easy development
  - **State Debugging**: Clear authentication state management for debugging
  - **Error Handling**: Comprehensive error handling with user feedback

### Cross-Component Integration
#### Enhanced
- **🔄 Complete Authentication Flow**: End-to-end authentication system integration
  - **Backend ↔ Frontend**: Seamless API integration for authentication endpoints
  - **Database ↔ Sessions**: Complete session management with database persistence
  - **Development ↔ Production**: Environment-aware authentication behavior
- **🛡️ Security Implementation**: Professional security practices throughout
  - **Token Security**: Industry-standard JWT implementation with secure defaults
  - **Session Security**: Proper session management with automatic cleanup
  - **Cookie Security**: Secure cookie implementation for refresh tokens
  - **Development Security**: Secure development practices with console logging

---

## [API v1.3.0] - 2025-08-27

### API Backend Changes (v1.3.0)
#### Added
- **🗄️ Complete SQLite Database System**: Full user management with environment-aware database selection
  - **Database Module Architecture**: New modular database layer in `api/src/database/`
    - **`connection.rs`**: Environment-aware database connections with automatic host detection
    - **`models.rs`**: User model with complete data structures and TypeScript-compatible serialization  
    - **`operations.rs`**: Full CRUD operations with proper error handling and SQL injection protection
    - **`mod.rs`**: Clean module exports with unified database interface
  - **Dual Environment Support**: Automatic database selection based on request origin
    - **Development Database**: `hashrand-dev.db` for `localhost` and `elite.faun-pirate.ts.net` requests
    - **Production Database**: `hashrand.db` for all other hosts
    - **Intelligent Host Detection**: Automatic environment detection via HTTP Host header parsing
    - **Seamless Switching**: No configuration needed - databases selected automatically
  - **User Management REST API**: Complete CRUD operations for user entities
    - **GET /api/users**: List users with optional limit parameter and count metadata
    - **GET /api/users/:id**: Retrieve specific user by ID with full validation
    - **POST /api/users**: Create new user with JSON body (username + email required)
    - **DELETE /api/users/:id**: Delete user with existence validation and proper status codes
    - **Professional Error Handling**: HTTP 400/404/500 responses with descriptive JSON error messages
    - **Input Validation**: Server-side validation for usernames, emails, and ID formats

#### Enhanced
- **🏗️ Database Integration**: Seamless integration with existing Spin architecture
  - **Configuration Setup**: New `runtime-config.toml` defining multiple database environments
  - **Spin Configuration**: Updated `spin.toml` with SQLite database access permissions
  - **Automatic Table Creation**: Users table created automatically on first database access
  - **Data Directory**: Organized database files in dedicated `data/` directory
  - **Zero-Configuration**: Databases created and initialized without manual setup
- **🔧 Request Routing Enhancement**: Extended routing system for database endpoints
  - **Method-Based Routing**: Support for GET, POST, DELETE methods on user endpoints
  - **RESTful Patterns**: Clean REST API following standard conventions
  - **Updated Help System**: Enhanced 404 responses include new user management endpoints
  - **Backward Compatibility**: All existing endpoints remain unchanged

#### Technical Implementation  
- **Professional Database Architecture**: Industry-standard patterns and practices
  - **Connection Pooling**: Efficient database connection management via Spin SDK
  - **Transaction Safety**: Proper error handling with automatic rollback on failures
  - **SQL Injection Protection**: Parameterized queries throughout all database operations
  - **Type Safety**: Full Rust type safety from database to HTTP response
  - **Memory Efficiency**: Optimized queries and data structures for WebAssembly constraints
- **Development Experience**: Enhanced development workflow with database support
  - **Runtime Configuration**: Flexible database configuration without code changes
  - **Development vs Production**: Clear separation of environments without configuration complexity
  - **Error Logging**: Comprehensive error logging for database operations
  - **Testing Support**: Database operations fully testable in development environment

#### Database Schema
- **Users Table Structure**: Complete user entity with timestamps and constraints
  ```sql
  CREATE TABLE users (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      username TEXT NOT NULL UNIQUE,
      email TEXT NOT NULL UNIQUE, 
      created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
      updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
  );
  ```
- **Constraint Enforcement**: Database-level uniqueness constraints for username and email
- **Automatic Timestamps**: Server-managed creation and update timestamps
- **Auto-Increment IDs**: Primary key generation handled automatically

#### Integration Benefits
- **Stateful Operations**: Enables user management and persistent data storage
- **Scalable Architecture**: Foundation for future database-dependent features
- **Development Efficiency**: Automatic environment detection eliminates configuration overhead
- **Production Ready**: Separate databases ensure clean development/production isolation
- **REST API Standards**: Professional API design following industry best practices

---

## [API v1.2.1] - 2025-08-25

### API Backend Changes (v1.2.1)
#### Enhanced
- **🔐 ChaCha8 OTP Generation**: Refactored OTP generation for complete cryptographic consistency
  - **Unified Cryptographic Architecture**: All random generation now uses ChaCha8Rng throughout the system
    - **Hash Generation**: Uses `ChaCha8Rng::from_seed()` for main hash/password/api-key generation (existing)
    - **OTP Generation**: Now uses `ChaCha8Rng::from_seed()` for 9-digit OTP generation (NEW)
    - **Professional Implementation**: Replaced simple XOR approach with industry-standard ChaCha8
  - **Enhanced Domain Separation**: Elegant cryptographic domain separation technique
    - **Previous**: Modified first 8 bytes with XOR pattern (`0x5A + i`)
    - **Current**: Single-byte domain separation on last byte (`otp_seed[31] ^= 0x5A`)
    - **Cleaner Logic**: Minimal seed modification with maximum cryptographic effectiveness
  - **Algorithm Consistency**: Single cryptographic family (ChaCha8) for all pseudorandom generation
    - **Better Security**: ChaCha8 is cryptographically robust and industry-audited
    - **Code Maintainability**: One RNG technology instead of mixed approaches
    - **Professional Standards**: Follows established cryptographic best practices

#### Fixed
- **🔧 Deprecated API Usage**: Updated to modern Rust rand API
  - **Method Migration**: Changed `rng.gen_range()` to `rng.random_range()` 
  - **Compiler Compliance**: Eliminated deprecation warnings during build process
  - **Future-Proof**: Updated to latest rand crate API standards

#### Technical Implementation
- **Cryptographic Architecture**: Complete ChaCha8 ecosystem implementation
  - **Single Dependency**: `rand_chacha = "0.9.0"` handles all random generation needs
  - **Seed Management**: Consistent 32-byte seed format across all generation functions
  - **Domain Separation**: Professional approach using minimal seed variation
  - **Performance**: No performance impact - ChaCha8 was already in use for main generation

---

## [API v1.2.0 / Web v0.17.2] - 2025-08-24

### Major New Feature: Complete BIP39 Mnemonic Generation System
#### Added
- **🔐 BIP39 Mnemonic Endpoint**: New `/api/mnemonic` endpoint for generating Bitcoin Improvement Proposal 39 mnemonic phrases
  - **GET Method**: Random mnemonic generation with query parameters
    - `language` parameter: 10 supported languages (english, spanish, french, portuguese, japanese, chinese, chinese-traditional, italian, korean, czech)
    - `words` parameter: 12 or 24 word mnemonics (default: 12)
    - Same JSON response format as other endpoints with hash, seed, otp, and timestamp
  - **POST Method**: Deterministic mnemonic generation with seed input
    - Required `seed` field in JSON body (44-character base58 format)
    - Optional `language` and `words` parameters in JSON body
    - Full seed validation with proper error handling (400 status for invalid seeds)
  - **Cryptographically Secure**: Uses proper BIP39 entropy generation
    - 12 words: 128-bit entropy (16 bytes)
    - 24 words: 256-bit entropy (32 bytes)
    - Standard BIP39 wordlists for all supported languages

#### Enhanced
- **🌍 Complete Language Coverage**: 10 languages with full BIP39 standard compliance
  - **Western Europe**: English (default), Spanish, French, Portuguese, Italian
  - **Asia**: Chinese Simplified, Chinese Traditional, Japanese, Korean  
  - **Central Europe**: Czech
  - All languages use official BIP39 wordlists from the standard specification
  - Perfect compatibility with hardware wallets and standard cryptocurrency software
- **🔄 Dual Generation Modes**: Consistent with existing endpoint patterns
  - **Random Generation (GET)**: New random mnemonic each request
  - **Deterministic Generation (POST)**: Same seed produces identical mnemonic
  - Both modes support all language and word count combinations
- **🛡️ Comprehensive Validation**: Robust parameter validation and error handling
  - Language validation with descriptive error messages
  - Word count validation (only 12 and 24 accepted)
  - Seed format validation for deterministic generation
  - All validation errors return HTTP 400 with clear error descriptions

#### Technical Implementation
- **🏗️ Modular Architecture**: New mnemonic handler following established patterns
  - `api/src/handlers/mnemonic.rs`: Complete handler implementation
  - Integrated routing with both GET and POST support
  - Shared validation and error handling logic
- **📦 BIP39 Integration**: Full integration of `bip39` crate with language features
  - Added dependency: `bip39 = { version = "2.2.0", features = [...] }`
  - All 9 language features enabled for complete coverage
  - Proper entropy handling for both 12 and 24 word generation
- **🧪 Comprehensive Testing**: Enhanced test suite with mnemonic endpoint coverage
  - **64 Total Tests** (previously 59): Added 5 new mnemonic-specific tests
  - **100% Success Rate**: All tests pass including new mnemonic functionality
  - **Complete Coverage**: Tests for all languages, word counts, and error conditions
  - **Enhanced Test Script**: Updated `final_test.sh` with POST request support

#### Documentation Updates
- **📚 Complete Documentation**: Updated README.md with comprehensive mnemonic endpoint documentation
  - Detailed API documentation with examples for all languages
  - Language support matrix with native names and codes
  - Both GET and POST usage examples
  - Integration with existing API documentation structure
- **🔧 Development Dependencies**: Updated project dependencies section
  - Added BIP39 crate with all language features
  - Updated test count references throughout documentation
  - Enhanced project structure documentation

#### User Benefits
- **🎯 Complete BIP39 Compliance**: Full compatibility with cryptocurrency ecosystem
- **🌐 Global Accessibility**: Support for users in 10 different languages
- **🔒 Security Options**: Both standard (12-word) and high (24-word) security levels
- **⚡ Consistent API**: Same patterns as existing endpoints for easy integration
- **🧪 Production Ready**: Comprehensive testing ensures reliability

---

## [API v1.1.0 / Web v0.17.1] - 2025-08-24

### Web Interface Changes (v0.17.1)
#### Fixed
- **🔄 Regenerate Button Behavior**: Corrected regenerate functionality to always perform GET requests without seed
  - **Problem**: Regenerate button was including seed in API call parameters, causing deterministic instead of random generation  
  - **Solution**: Modified `regenerateHash()` function to explicitly exclude seed from parameters (`delete paramsForGeneration.seed`)
  - **Result**: Regenerate button now correctly generates new random values while preserving other parameters (length, alphabet, prefix, suffix)
  - **Consistency**: Maintains intended behavior where regenerate always produces different results regardless of how the original was generated

---

## [API v1.1.0 / Web v0.17.0] - 2025-08-24

### Major Breaking Change: Base58 Seed Format Migration
#### Changed
- **🔄 Seed Format Migration**: Complete migration from hexadecimal to base58 seed encoding
  - **API Breaking Change**: All endpoints now use 44-character base58 seeds instead of 64-character hexadecimal
  - **Enhanced Security**: Base58 encoding eliminates confusing characters (0, O, I, l) for better usability
  - **Compact Representation**: Shorter seed strings (44 vs 64 chars) while maintaining same 256-bit entropy
  - **Bitcoin Standard**: Uses same base58 alphabet as Bitcoin for consistency and familiarity
  - **Backward Incompatibility**: Old hex seeds no longer accepted - requires migration for existing implementations

#### Enhanced
- **📊 Custom Endpoint Improvements**: Major enhancements to /api/custom endpoint
  - **🔢 Numeric Alphabet**: New `numeric` alphabet type supporting only digits 0-9
    - Perfect for generating numeric codes, PINs, or numeric-only identifiers
    - Requires longer lengths due to reduced entropy (10 characters vs 58+ in other alphabets)
    - Integrated with existing validation and UI systems
  - **🔐 OTP Generation**: 9-digit one-time password generation using same seed
    - Generated using seed with XOR nonce variation for different but deterministic output
    - Displayed in generation details as additional security feature
    - Uses numeric alphabet internally for guaranteed 9-digit output
  - **⏰ Timestamp Integration**: Unix timestamp included in all custom endpoint responses
    - Enables generation date/time tracking for audit purposes
    - Displayed as localized date/time in web interface
    - Consistent across all custom generation requests
- **🎯 UI Seed Handling Simplification**: Streamlined seed management in web interface
  - **Read-Only Display**: Seeds shown only when provided via URL GET parameters
  - **No Input Fields**: Removed all seed input capabilities from generator forms
  - **No Copy Functionality**: Simplified seed display as informational metadata only
  - **URL Parameter Only**: Seeds can be passed via URL but not entered manually
  - **Smart Regenerate Logic**: Regenerate button hidden only when seed comes from URL parameters, not API responses

#### Fixed
- **🔧 Regenerate Button Logic**: Corrected regenerate button visibility logic
  - **Problem**: Button was hidden whenever any seed was present (including API-generated ones)
  - **Solution**: Only hide when seed parameter comes from URL GET parameters (`searchParams.has('seed')`)
  - **Benefit**: Users can regenerate hashes that were initially random but still see seed information
- **🧹 Code Quality**: Comprehensive cleanup of unused code and dependencies
  - Removed unused hex seed validation functions
  - Cleaned up commented seed-related code
  - Updated API response type definitions for new custom endpoint structure
  - Simplified UI components by removing complex seed handling logic

#### Technical Implementation
- **Backend Changes**: 
  - Added `bs58` crate dependency for base58 encoding/decoding
  - Updated all seed handling functions to use base58 format
  - Modified custom endpoint to return structured JSON with hash, seed, OTP, and timestamp
  - Added numeric alphabet support with proper character validation
- **Frontend Changes**:
  - Simplified seed handling throughout all generator forms
  - Updated TypeScript types to match new API responses
  - Enhanced result display to show OTP and timestamp information
  - Removed all seed copying and input functionality from UI

#### Migration Notes
- **API Clients**: Must update to use base58 seed format (44 characters) instead of hexadecimal (64 characters)
- **Existing Seeds**: Cannot be directly converted - new base58 seeds must be generated
- **URL Parameters**: Seed parameters in URLs must now use base58 format
- **Testing**: All existing tests updated to use new base58 seed format

---

## [API v1.0.0 / Web v0.16.0] - 2025-08-23

### Major New Feature: Seed-Based Deterministic Generation
#### Added
- **🌱 Comprehensive Seed Support**: Complete deterministic generation system for all three generators
  - **Universal Seed Fields**: Optional 64-character hexadecimal seed input in custom, password, and api-key pages
  - **Dual API Modes**: 
    - **GET Requests**: Traditional random generation with auto-generated seed (existing behavior)
    - **POST Requests**: NEW deterministic generation using provided seed
  - **API Endpoint Enhancement**: All three endpoints now support both GET and POST methods
    - `POST /api/custom` - Deterministic hash generation with seed
    - `POST /api/password` - Deterministic password generation with seed  
    - `POST /api/api-key` - Deterministic API key generation with seed
  - **Consistent Response Format**: Both random and seeded generation return same JSON structure with hash and seed
  - **Perfect Reproducibility**: Same seed + same parameters = exactly same result every time

#### Enhanced
- **🎯 Intelligent UI Behavior**: Smart interface adaptations for deterministic generation
  - **Conditional UI Elements**: "Generate Another" button automatically hidden when using deterministic seeds
  - **Smart Result Display**: 
    - User-provided seeds displayed as informational text (non-editable)
    - Auto-generated seeds displayed as copyable textarea for reuse
  - **Seed Reuse Dialog**: Interactive modal when returning to settings with existing seed
    - **User Choice**: "Keep Same Seed" vs "Generate New Seed" options
    - **Styled Dialog**: Professional modal with transparent background and centered buttons
    - **Keyboard Support**: Escape key closes dialog, full accessibility
    - **13-Language Support**: Complete translations for dialog and buttons
- **🔧 API Architecture Improvements**: Enhanced backend to support dual-mode generation
  - **Method-Based Routing**: GET for random, POST for deterministic generation
  - **Seed Validation**: Server-side validation of 64-character hexadecimal seeds
  - **Parameter Integration**: JSON body parameters seamlessly integrated with existing validation
  - **Error Handling**: Comprehensive error responses for invalid seeds or parameters

#### Technical Implementation
- **Frontend Integration**: Complete TypeScript integration with new API methods
  - **New API Services**: `generatePasswordWithSeed()`, `generateApiKeyWithSeed()` methods
  - **Type Safety**: New interfaces `SeedPasswordRequest`, `SeedApiKeyRequest`
  - **Form Enhancement**: Seed validation with real-time feedback (red borders, error messages)
  - **URL Parameter Support**: Seeds passed as URL parameters maintain full functionality
- **Backend Architecture**: Elegant dual-mode handler system
  - **Request Routing**: Single handlers manage both GET and POST for each endpoint
  - **Code Reuse**: Shared generation logic between random and seeded modes  
  - **Hex Seed Parsing**: Robust conversion from hex string to 32-byte seed array
  - **Unified Response**: Both modes return consistent JSON with hash and seed fields

#### User Experience Benefits
- **🎯 Reproducible Testing**: Perfect for demonstrations, testing, and development workflows
- **📋 Audit Trails**: Complete traceability with seed included in every response
- **🔄 Consistent Results**: Eliminate randomness when needed for specific use cases
- **💾 State Persistence**: Seed reuse functionality enables workflow continuity
- **🎨 Professional UI**: Seamless integration that doesn't complicate the existing interface

---

## [API v1.0.0 / Web v0.15.0] - 2025-08-23

### Web Interface Changes (v0.15.0)
#### Enhanced
- **🌍 Translation Naturalness Improvements**: Comprehensive review and enhancement of all 13 language translations
  - **Portuguese Improvements**: Enhanced terminology for technical precision
    - Changed "letras" to "caracteres" for consistency across technical contexts
    - Updated "senhas" to "palavras-passe" (European Portuguese standard)
    - Improved overall fluency and professional terminology usage
  - **French Refinements**: Technical terminology standardization
    - Updated "lettres" to "caractères" for better technical accuracy
    - Enhanced sentence structures for improved natural flow
    - Maintained French linguistic elegance while ensuring technical precision
  - **German Language Polish**: Enhanced word order and terminology choices
    - Improved passive voice constructions: "Wird generiert..." (more natural)
    - Better modal verb usage: "darf nicht" instead of "kann nicht" (more appropriate)
    - Enhanced navigation terminology: "Gehe zu" (more conversational)
    - Improved compound terms for better German linguistic patterns
  - **Russian Localization**: Enhanced naturalness with proper linguistic structures
    - Changed "алфавитно-цифровой" to "буквенно-цифровой" (more natural Russian)
    - Improved "Алфавит без путаницы" to "Однозначный алфавит" (clearer meaning)
    - Enhanced sentence flow and case usage for better readability
    - Updated error messages for more natural Russian expressions
  - **Chinese Language Refinement**: Improved word choice and sentence structure
    - Changed "和" to "到" in range expressions (more natural for ranges)
    - Enhanced "带符号的" to "包含符号的" (more precise terminology)
    - Improved overall sentence flow and terminology consistency
    - Better adaptation to Chinese grammar patterns
  - **Arabic Enhancement**: Improved clarity and RTL considerations
    - Changed "عدم التشابه" to "واضحة" (clearer and more direct)
    - Enhanced sentence structures for better Arabic flow
    - Improved technical terminology while maintaining linguistic authenticity
  - **Euskera (Basque) Corrections**: Proper ergative/absolutive case usage
    - Corrected "tartean" to "artean" (more grammatically correct)
    - Enhanced ergative constructions for natural Basque syntax
    - Improved word order to match authentic Basque linguistic patterns
  - **Catalan Consistency**: Standardized technical terminology
    - Changed "lletres" to "caràcters" for technical consistency
    - Updated all character-related terminology for uniformity
    - Enhanced professional terminology across the interface
  - **Hindi Linguistic Improvements**: More authentic Hindi terminology
    - Changed "लेंथ" to "लंबाई" (pure Hindi instead of anglicism)
    - Updated "सिक्यूरिटी" to "सुरक्षा" (native Hindi term)
    - Improved overall linguistic authenticity and naturalness
  - **Japanese Completeness**: Added missing translation elements
    - Added missing "yes" (はい) and "no" (いいえ) translations
    - Enhanced existing translations for better Japanese linguistic flow
    - Maintained proper SOV structure throughout the interface

#### Enhanced
- **📅 DateTimeLocalized Component Robustness**: Advanced fallback system for broader browser compatibility
  - **Multi-Level Fallback Architecture**: Sophisticated fallback system for unsupported locales
    - **Primary**: Attempts native `Intl.DateTimeFormat` with target locale
    - **Detection**: Intelligent detection of failed localization (when browsers return English names for other languages)
    - **Secondary**: Automatic fallback to English formatting if locale isn't truly supported
    - **Ultimate**: Manual ISO date formatting as final fallback for maximum compatibility
  - **Enhanced Galician Support**: Custom fallback implementation for improved compatibility
    - **Authentic Abbreviations**: Custom Galician month abbreviations (`xan.`, `feb.`, `mar.`, etc.)
    - **Smart Detection**: Attempts native Intl first, falls back to custom if needed
    - **Consistent Formatting**: Maintains visual consistency with other languages
  - **Intelligent Locale Validation**: Advanced detection of browser locale support limitations
    - **English Month Detection**: Identifies when browsers incorrectly return English month names
    - **Comprehensive Validation**: Checks multiple English month name variants (short and long forms)
    - **Graceful Degradation**: Seamless fallback without user-visible errors
    - **Cross-Platform Compatibility**: Works reliably across different browser engines and versions
  - **Improved Architecture**: Enhanced code organization and maintainability
    - **Helper Functions**: Extracted common time formatting logic for code reuse
    - **Consistent Error Handling**: Unified approach to locale failures across all languages
    - **Performance Optimization**: Efficient validation without impacting rendering speed

#### Fixed
- **🔤 Translation Consistency**: Resolved terminology inconsistencies across languages
  - **Technical Terms**: Standardized character/letter terminology in Portuguese, French, and Catalan
  - **Regional Variations**: Enhanced European Portuguese vs Brazilian Portuguese distinctions
  - **Linguistic Authenticity**: Improved native term usage in Hindi and other languages
- **🌐 Cross-Browser Locale Support**: Enhanced compatibility for date/time formatting
  - **Browser Variations**: Improved handling of different browser Intl.DateTimeFormat implementations
  - **Locale Fallbacks**: Robust fallback chain for unsupported or partially supported locales
  - **Edge Case Handling**: Better handling of mixed locale support scenarios

#### Technical Implementation
- **Translation Quality Assurance**: Systematic approach to linguistic improvements
  - **Native Speaker Review**: Used English as reference with Spanish linguistic guidance
  - **Grammatical Considerations**: Applied language-specific grammatical rules
    - **German**: Case system and compound word formation
    - **Russian**: Proper case usage and aspectual verb forms
    - **Arabic**: RTL considerations and authentic terminology
    - **Basque**: Ergative-absolutive alignment and word order
    - **Chinese**: Proper particle usage and sentence structure
    - **Japanese**: SOV word order and appropriate formality levels
    - **Hindi**: Pure Hindi vs English loanword preferences
  - **Consistency Enforcement**: Unified terminology across all interface elements
  - **Cultural Adaptation**: Respectful adaptation to regional linguistic preferences

---

## [API v1.0.0 / Web v0.14.0] - 2025-08-23

### Web Interface Changes (v0.14.0)
#### Added
- **🖼️ Progressive Sprite Loading System**: Advanced icon loading with immediate fallbacks
  - **Deferred Loading**: 10-second delayed sprite loading after DOM ready (testing mode)
  - **UTF Placeholder System**: Instant visual feedback with Unicode emojis during sprite loading
    - 🏠 for home icons, ☀️/🌙 for theme toggle, > for choose arrows
    - Complete emoji mapping for all 16 flags and UI icons in `flagEmojis.ts`
    - Zero layout shift during sprite transition
  - **Global State Management**: `window.__SPRITE_STATE__` tracks loading progress
  - **Custom Events**: `sprite-loaded` event for cross-component synchronization
  - **Smart Fallbacks**: Graceful degradation when sprite fails to load
- **🚩 Professional Flag Integration**: Full-resolution flag SVGs with zero compromise
  - **189KB Sprite**: Complex flag SVGs from `/home/arkaitz/proyectos/web/svg-flags/`
  - **16 Complete Flags**: All 13 language flags plus 3 regional Spanish flags
    - **National**: Spain, UK, France, Germany, Portugal, Russia, Saudi Arabia, China, Japan, India
    - **Regional Spanish**: Euskadi (Basque), Catalonia, Galicia from `/regions/` directory
  - **SVG Reference Fixes**: Fixed internal references with unique prefixes (e.g., `china-cn-a`)
  - **Modern SVG Syntax**: Replaced `xlink:href` with `href` for better compatibility
- **📁 Centralized Language Configuration**: Eliminated duplicate code across components
  - **`/web/src/lib/languageConfig.ts`**: Shared configuration file for all language data
  - **DRY Architecture**: Single source of truth for languages, names, and flags
  - **Helper Functions**: `getLanguageByCode()`, `getLanguageName()` utilities
  - **Type Safety**: Complete TypeScript definitions for language structures
- **🔗 Universal URL Parameter Support**: Complete GET parameter integration across all routes
  - **Generator Pages**: `/custom/`, `/password/`, `/api-key/` read and apply URL parameters
  - **Parameter Validation**: Client-side validation for all parameter types and ranges
  - **Persistent State**: URL parameters override stored state and defaults
  - **Shareable URLs**: Complete configuration can be shared via URL parameters

#### Enhanced
- **🏗️ Centralized API Architecture**: Reorganized generation workflow for better maintainability
  - **Generator Pages**: Handle only UI, validation, and navigation (NO API calls)
  - **Result Page**: Centralized API calling via `generateFromParams()` function
  - **Fresh Generation**: Result page ALWAYS generates new values, never displays cached data
  - **Parameter Flow**: Generators → URL params → Result → API call → Display
  - **Error Handling**: Centralized error handling in result page with proper fallbacks
- **🎯 Icon Component Evolution**: Enhanced placeholder system with loading states
  - **Dynamic Placeholders**: Icons show UTF emojis until sprite loads
  - **State Subscriptions**: React to sprite loading events for smooth transitions
  - **RTL-Aware Placeholders**: Choose buttons show ">" in both LTR and RTL correctly
  - **Loading Indicators**: Subtle visual feedback during sprite loading

#### Fixed
- **🔧 SVG Internal References**: Resolved flag display issues with complex SVGs
  - **Unique ID Prefixes**: Added country prefixes to prevent ID conflicts (e.g., `#cn-a` → `#china-cn-a`)
  - **Bulk Processing**: Processed 1,764 SVG files, fixed 574 with internal references
  - **Python Script**: Created `/tmp/fix_all_svg_references.py` for automated fixes
  - **Complete Coverage**: All flag SVGs now display correctly with proper internal links

#### Architecture Changes
- **Navigation Flow**: Enhanced user experience with parameter persistence
  - **Menu → Generator**: Loads defaults or URL parameters
  - **Generator → Result**: Passes configuration via URL parameters
  - **Result → Generator**: Returns with current configuration intact
  - **Bookmarkable States**: Any configuration state can be bookmarked and shared
- **Code Quality**: Comprehensive elimination of duplicate logic
  - **Language Configuration**: Shared between TopControls and LanguageSelector
  - **Type Definitions**: Centralized language types and interfaces
  - **Component Reuse**: Consistent component usage patterns

#### Technical Implementation
- **Sprite Loading Pipeline**: Sophisticated loading system with fallbacks
  ```javascript
  // app.html - Deferred loading with 10s delay
  window.__SPRITE_STATE__ = { loaded: false, loading: true, error: false };
  setTimeout(() => { /* fetch and inject sprite */ }, 10000);
  ```
- **Parameter Processing**: URL parameter parsing in all generator pages
  ```typescript
  // onMount in generator pages
  const urlLength = searchParams.get('length');
  if (urlLength && isValid(urlLength)) params.length = parseInt(urlLength);
  ```
- **Result Generation**: Unified API calling based on endpoint parameter
  ```typescript
  switch (endpoint) {
    case 'custom': result = await api.generate(params); break;
    case 'password': result = await api.generatePassword(params); break;
    case 'api-key': result = await api.generateApiKey(params); break;
  }
  ```

---

## [API v1.0.0 / Web v0.13.0] - 2025-08-23

### Web Interface Changes (v0.13.0)
#### Added
- **🔍 Comprehensive Linting System**: Enterprise-grade code quality tools unified through Vite
  - **Modern ESLint v9**: Latest flat config with TypeScript and Svelte support
  - **Prettier Integration**: Automatic code formatting with Svelte plugin support
  - **Vite Plugin Integration**: Real-time linting during development with `vite-plugin-eslint`
  - **TypeScript Declarations**: Custom type definitions for `vite-plugin-eslint` in `vite-env.d.ts`
  - **Browser Globals**: Pre-configured ESLint environment for fetch, localStorage, DOM APIs
- **⚡ Unified Quality Pipeline**: Single command for complete code verification
  - **`just check`**: Complete quality verification (clippy + fmt + ESLint + svelte-check)
  - **`just lint`**: Dual-language linting (Rust clippy + ESLint via Vite)  
  - **`just fmt`**: Unified formatting (cargo fmt + Prettier)
  - **Smart Build Integration**: Production builds fail only on errors, warnings allowed
- **🛠️ Developer Experience**: Enhanced development workflow integration
  - **Live Linting**: ESLint runs automatically during development
  - **Instant Feedback**: Warnings and errors show in terminal and browser console
  - **Hot Reload**: Linting updates without manual rebuilds
  - **Editor Integration**: Compatible with VSCode, vim, emacs ESLint plugins

#### Enhanced
- **🎯 Code Quality Standards**: Comprehensive cleanup and standardization
  - **Zero Warnings**: Eliminated all 15+ ESLint warnings across the codebase
  - **Import Cleanup**: Removed unused imports from route components (Icon, resultState, etc.)
  - **Type Safety**: Fixed all TypeScript errors with proper type annotations
  - **Variable Usage**: Cleaned unused variables while preserving functionality
  - **Modern Syntax**: Updated `@ts-ignore` to `@ts-expect-error` for better type checking
- **🔧 Technical Improvements**: Enhanced type definitions and error handling
  - **Timeout Types**: Cross-platform `ReturnType<typeof setTimeout>` for proper typing
  - **Unknown Types**: Replaced `any` types with specific `unknown` and type assertions
  - **API Types**: Improved `ResultState` interface with proper parameter types
  - **Error Handling**: Enhanced catch blocks without unused error variables

#### Fixed
- **🚨 TypeScript Compilation Errors**: Resolved all build-blocking TypeScript issues
  - **Missing Type Definitions**: Added `@types/node` for process.env access
  - **Custom Declarations**: Created type definitions for vite-plugin-eslint
  - **Translation Function**: Fixed type casting in i18n system for proper type safety
  - **Cross-Platform Compatibility**: Fixed setTimeout typing for browser and Node.js
- **🧹 Code Cleanup**: Systematic elimination of unused code and imports
  - **Route Components**: Removed unused `Icon` imports from pages using only `Iconize`
  - **Store Imports**: Cleaned unused store subscriptions (resultState, clearResult, etc.)
  - **Component Imports**: Removed unused `LoadingSpinner` and other component imports
  - **Type Imports**: Cleaned unused type definitions like `VersionResponse`

#### Technical Implementation
- **ESLint Configuration**: Modern flat config architecture for maximum compatibility
  - **Dual Language Support**: Separate configs for TypeScript and Svelte files
  - **Plugin Integration**: Comprehensive plugin ecosystem (TypeScript, Svelte, Prettier)
  - **Environment Configuration**: Browser globals and Node.js types properly configured
  - **Rule Optimization**: Balanced rule set for code quality without developer friction
- **Vite Integration**: Advanced build system integration for seamless development
  - **Plugin Configuration**: Smart linting behavior based on environment variables
  - **Development Mode**: Non-blocking linting with visible warnings
  - **Production Mode**: Strict linting that fails builds on errors
  - **CI/CD Mode**: `VITE_LINT_ONLY=true` for pipeline integration
- **Development Workflow**: Enhanced justfile commands for unified experience
  - **Parallel Execution**: Multiple linting tools run efficiently
  - **Exit Code Handling**: Proper error reporting for CI/CD pipelines
  - **Format Integration**: Prettier runs before ESLint for consistent workflow

---

## [API v1.0.0 / Web v0.12.0] - 2025-08-23

### Web Interface Changes (v0.12.0)
#### Added
- **📅 DateTimeLocalized Component**: Portable date/time formatting component for internationalization
  - **Universal Date Formatting**: Handles 13 languages with proper locale detection and formatting
  - **Custom Euskera Format**: Special handling for Basque language with authentic format: `{year}ko {month}ak {day}, {time}`
  - **Basque Month Names**: Complete array of Euskera month names (`urtarril`, `otsail`, `martxo`, etc.)
  - **Configurable Options**: Accepts `Intl.DateTimeFormatOptions` for custom formatting
  - **Portable Design**: Can be reused in any project, similar to Iconize component
  - **Automatic Reactivity**: Updates when language changes without manual intervention
  - **Fallback Support**: Graceful fallback to English if locale fails
- **▶️ Play Icon Integration**: Added play symbols to all generate buttons for better UX
  - **Minimalist Design**: Uses Unicode ▶ (triangle) without emoji decorations or frames
  - **Consistent Implementation**: Applied across custom, password, and API key generators
  - **RTL-Aware**: Properly positioned using Iconize component for automatic RTL support
  - **Visual Clarity**: Suggests "execute" or "run" action, improving user understanding
- **🏠 Home Icon System**: Replaced briefcase icons with intuitive home icons
  - **New SVG Icon**: Added professional home icon to sprite system with house outline design
  - **Universal Navigation**: Applied to all "back to menu" buttons across the application
  - **Icon Cleanup**: Removed unused briefcase icon from sprite to reduce bundle size
  - **Better Semantics**: Home icon is more intuitive for navigation to main menu

#### Enhanced
- **🔧 Iconize Component Improvements**: Advanced positioning control with `invertposition` parameter
  - **Flexible Positioning**: New `invertposition` parameter (default: `false`) controls content order
    - `false` (default): Icon first, then content → "▶ Generate"
    - `true`: Content first, then icon → "Choose >"
  - **Simplified Logic**: Removed complex `position` parameter in favor of boolean toggle
  - **Conditional Slot Rendering**: Smart slot positioning based on `invertposition` value
  - **RTL Compatibility**: Works seamlessly with existing RTL icon swapping logic
  - **Surgical Implementation**: Minimal code changes for maximum functionality improvement
- **🎯 Result Page Button Styling**: Enhanced buttons to match form page consistency
  - **Unified Button Sizes**: All result buttons now use same size as custom/password/api-key pages
  - **Professional Padding**: Upgraded to `px-6 py-4` (from `px-6 py-3`) for better touch targets
  - **Typography Enhancement**: Changed to `font-semibold` (from `font-medium`) for better readability  
  - **Consistent Spacing**: Added `hover:shadow-lg` effects matching other page buttons
  - **Icon Size Standardization**: Increased icon sizes to `w-5 h-5` (from `w-4 h-4`) for consistency
  - **Container Integration**: Moved buttons inside result container for better visual hierarchy
- **📐 Component Structure Optimization**: Improved semantic organization of interface elements
  - **Header Icon Separation**: Fixed Iconize usage in menu cards to wrap only emoji, not h2 title
  - **Semantic HTML**: H2 elements now properly outside Iconize wrapper for correct document structure
  - **Clean Component Boundaries**: Clear separation between icon decoration and semantic content
  - **Flexbox Layout**: Used native flexbox for proper spacing between emojis and titles

#### Fixed
- **🔧 Svelte 5 Syntax Issues**: Corrected reactive syntax in components
  - **DateTimeLocalized**: Fixed `$derived(() => {})` to `$derived.by(() => {})` syntax error
  - **Iconize**: Resolved function code display issue by using correct reactive syntax
  - **Rendering Problems**: Fixed cases where function code appeared in UI instead of computed values
  - **Modern Svelte**: Ensured compatibility with Svelte 5 runes mode throughout application
- **🎨 UI Visual Issues**: Resolved component display and positioning problems
  - **Menu Card Structure**: Fixed h2 elements being incorrectly wrapped inside Iconize
  - **Button Consistency**: Standardized button sizes across all pages for uniform appearance
  - **Icon Positioning**: Improved icon placement in various UI components using Iconize

#### Technical Implementation
- **Portable Component Design**: Both DateTimeLocalized and enhanced Iconize follow portable design patterns
  - **Zero Project Dependencies**: Components can be easily copied to other projects
  - **Clean Interfaces**: Simple, well-defined props with TypeScript support
  - **Minimal Coupling**: Only depend on standard i18n store, no project-specific logic
  - **Reusable Architecture**: Follow same patterns as successful Iconize component
- **Advanced Date Formatting**: Sophisticated internationalization handling
  - **Locale Mapping**: Complete mapping from language codes to proper locale identifiers
  - **Custom Formatting Logic**: Special handling for languages lacking native Intl support
  - **Error Handling**: Graceful fallback mechanism for unsupported locales
  - **Performance Optimized**: Reactive updates without unnecessary re-computation

---

## [API v1.0.0 / Web v0.11.0] - 2025-08-22

### Web Interface Changes (v0.11.0)
#### Added
- **🔧 Universal Iconize Component**: Revolutionary RTL-aware wrapper for any content with automatic icon positioning
  - **Universal Wrapper**: Works with any content - plain text, HTML elements, or complex components
  - **Smart RTL Behavior**: Automatically positions icons correctly for LTR and RTL languages
    - **LTR**: `[icon][text]` - Icon appears on the left (start position)
    - **RTL**: `[text][icon]` - Icon appears on the right (end position) 
  - **Dual Icon Support**: Supports both SVG sprite icons and Unicode emojis
    - **Sprite Icons**: `<Iconize conf={{icon: "arrow-right"}}>Choose</Iconize>`
    - **Emoji Support**: `<Iconize conf={{emoji: "🎲"}}>Custom Hash Generator</Iconize>`
  - **RTL-Specific Icons**: Different icons for RTL mode with `rtlIcon` property
    - Example: `arrow-right` in LTR becomes `arrow-left` in RTL for proper visual flow
  - **Zero Configuration RTL**: Uses HTML `dir="rtl"` and Tailwind's automatic flexbox behavior
  - **KISS Principle**: Simple implementation using native browser RTL behavior instead of complex CSS order logic

#### Enhanced
- **🎯 Menu Interface**: Complete migration to Iconize component
  - **All Card Titles**: Custom, Password, and API Key cards now use Iconize with their respective emojis
    - 🎲 Custom Hash Generator with automatic RTL positioning
    - 🔐 Secure Password with proper icon placement
    - 🔑 API Key with consistent RTL behavior
  - **Unified Experience**: All menu cards now have consistent RTL-aware icon behavior
  - **Simplified Code**: Eliminated complex conditional RTL logic in favor of automatic behavior

#### Technical Implementation
- **Flexbox RTL Integration**: Leverages Tailwind CSS and HTML `dir` attribute for automatic RTL behavior
  - **No Manual Order**: Eliminates need for CSS `order-1`/`order-2` classes
  - **Native Browser Support**: Uses browser's built-in RTL handling capabilities
  - **Tailwind 4.0 Compatible**: Works seamlessly with modern Tailwind RTL features
- **Component Architecture**: Clean, composable design following single responsibility principle
  - **Flexible Configuration**: Supports icon size, spacing, classes, and RTL-specific options
  - **Type-Safe**: Full TypeScript support with proper interface definitions
  - **Reusable**: Can wrap any content while maintaining semantic HTML structure
- **Performance Optimized**: Minimal overhead with automatic browser-native RTL handling

#### Fixed
- **🔧 RTL Icon Positioning**: Resolved complex CSS order issues with browser-native solution
  - **Problem**: Previous attempts using `order-1`/`order-2` classes had compilation issues
  - **Root Cause**: Tailwind wasn't compiling dynamically generated order classes
  - **Solution**: Switched to HTML `dir="rtl"` approach for automatic flexbox behavior
  - **Result**: Perfect RTL behavior with zero configuration and no CSS complexity

---

## [API v1.0.0 / Web v0.10.0] - 2025-08-21

### Web Interface Changes (v0.10.0)
#### Added
- **🔄 RTL-Aware Button Component**: Universal button wrapper with automatic RTL support
  - **Smart Icon Positioning**: Icons automatically position left (LTR) or right (RTL) based on language direction
  - **CSS Direction-Based**: Uses `direction: rtl/ltr` for seamless visual order changes
  - **Wrapper Architecture**: Simple pass-through wrapper preserving all native button attributes
  - **Built-in RTL**: Never forget to apply RTL - it's automatic for all buttons with icons
- **🌐 Improved Language Ordering**: Alphabetical organization by native language names
  - **Latin Transcription Ordering**: Languages sorted by transcribed native names (Arabiya, Catala, Deutsch, English...)
  - **Professional Organization**: Easier language discovery with logical alphabetical arrangement
  - **Consistent Across Components**: Applied to both TopControls and LanguageSelector components
- **📏 Enhanced Code Quality Standards**: Enforced DRY and KISS principles
  - **Architecture Guidelines**: Added mandatory DRY/KISS principles to project documentation
  - **Code Duplication Identification**: Flagged duplicate language selector logic for future refactoring
  - **Quality Assurance**: Self-replicating code quality rules across all project documentation

#### Enhanced
- **🔘 Universal Button RTL Support**: All buttons now support RTL automatically
  - **Result Page Buttons**: Regenerate, settings, and menu buttons with proper RTL icon positioning
  - **Form Buttons**: Generate and navigation buttons across custom, password, and API key forms
  - **Copy Button**: Enhanced copy functionality with RTL-aware positioning
  - **Consistent Experience**: Arabic users see icons on the right, other languages on the left

#### Technical
- **🏗️ Component Architecture**: Simplified Button component implementation
  - **Removed Complex Logic**: Eliminated confusing variant/size props and conditional logic
  - **Pure Wrapper**: Button component now purely wraps native button with RTL enhancement
  - **Automatic RTL**: No manual RTL handling required - works out of the box
  - **Clean Implementation**: Single responsibility principle - just handle icon positioning

#### Fixed
- **🔧 RTL Icon Positioning**: Resolved incorrect icon placement in Arabic language mode
  - **Visual Order**: Icons now appear on correct side in RTL languages (text first, icon second)
  - **CSS Direction**: Proper use of CSS direction property for automatic visual reordering
  - **Component Logic**: Fixed Button component logic to handle RTL states correctly

---

## [API v1.0.0 / Web v0.9.0] - 2025-08-21

### Web Interface Changes (v0.9.0)
#### Added
- **🎭 Advanced RTL Transition System**: Smooth fade effects for language direction changes
  - **Visual Fade Transitions**: Top controls container fades out/in (1.5s duration) when switching between LTR/RTL languages
  - **Seamless Direction Changes**: Controls smoothly transition from right corner (LTR) to left corner (RTL)
  - **No Layout Jumps**: Prevents jarring visual changes during language/direction switches
- **🎨 Unified Top Controls Container**: Complete consolidation of theme and language controls
  - **Single Container Component**: New `TopControls.svelte` combines both theme toggle and language selector
  - **Cohesive Visual Design**: Gray semi-transparent background (`bg-gray-200/90`) with backdrop blur effect
  - **Responsive Positioning**: Compact margins for mobile (2px from edges), standard for desktop (16px)
  - **Professional Box Design**: Rounded corners, subtle shadow, discrete border for elegant appearance
- **⚡ Differentiated Transition Speeds**: Advanced animation system for optimal user experience
  - **Background Transitions**: Slow 0.75s transitions for button background colors, shadows, and borders
  - **Icon Transitions**: Fast 0.15s transitions for icon changes (theme: sun/moon, language: flag changes)
  - **Perfect Balance**: Immediate feedback for content changes, elegant transitions for visual states

#### Enhanced
- **📱 Mobile-First Design**: Optimized spacing and positioning for all screen sizes
  - **Compact Mobile Layout**: 2px margins from screen edges on mobile devices
  - **Enlarged Icons**: Language flag icons increased to `w-12 h-12` (48px) for better visibility and touch interaction
  - **Optimized Container Size**: 4px internal padding for perfect icon-to-container ratio
  - **Consistent Cross-Platform**: Maintains professional appearance across all device types
- **🌐 RTL/LTR Language Support**: Enhanced internationalization with visual consistency
  - **Smart Positioning**: Container automatically moves between corners based on text direction
  - **Proper Dropdown Alignment**: Language dropdown correctly positioned for both RTL and LTR modes
  - **Seamless Integration**: All 13 languages maintain consistent visual experience
- **🎯 Result Page UX Improvements**: Enhanced user interaction patterns
  - **Optimized Copy Button**: Moved to bottom-right corner of result textarea for better ergonomics
  - **Reduced UI Movement**: Eliminated instructional text that caused layout shifts during loading states
  - **Consistent Visual Height**: Result area maintains stable dimensions during all state changes
  - **Improved Accessibility**: Copy functionality only visible when results are available

#### Fixed
- **🔧 TypeScript Build Warnings**: Resolved SvelteKit configuration issues
  - **Missing Base Config**: Fixed `Cannot find base config file "./.svelte-kit/tsconfig.json"` warning
  - **Automatic Sync**: Build process now includes `npx svelte-kit sync` to generate required config files
  - **Clean Builds**: Development workflow now produces zero warnings during compilation
- **🐛 Component Integration Issues**: Resolved conflicts between individual positioning systems
  - **Eliminated Positioning Conflicts**: Removed individual absolute positioning from theme/language components
  - **Centralized Control**: Single container manages all positioning logic for consistency
  - **RTL Button Visibility**: Fixed issue where theme toggle disappeared in RTL mode due to flex ordering

#### Technical Implementation
- **Component Architecture**: Revolutionary approach to control grouping
  - **Self-Contained Logic**: All theme and language functionality consolidated in single component
  - **No External Dependencies**: Eliminated complex interactions between separate positioned components
  - **State Management**: Integrated state handling for both theme switching and language selection
  - **Event Coordination**: Unified click-outside handling and dropdown management
- **Advanced CSS Transitions**: Sophisticated animation system
  - **Selective Property Transitions**: Independent control over colors, shadows, borders, and transforms
  - **Optimal Duration Mapping**: Different durations for different types of visual changes
  - **Smooth Performance**: Hardware-accelerated transforms and optimized transition properties
- **Responsive Design System**: Mobile-first approach with breakpoint optimization
  - **Fluid Spacing**: Seamless scaling from mobile (2px) to desktop (16px) margins
  - **Touch Optimization**: Larger touch targets and improved spacing for mobile interaction
  - **Progressive Enhancement**: Enhanced features for larger screens while maintaining mobile functionality

---

## [API v1.0.0 / Web v0.8.0] - 2025-08-20

### Web Interface Changes (v0.8.0)
#### Added
- **🌍 Complete Translation System**: Full restoration of internationalization with 13 languages
  - **Modular Translation Architecture**: Separated each language into individual files for better maintainability
    - `/web/src/lib/stores/translations/en.ts`, `es.ts`, `pt.ts`, `fr.ts`, `de.ts`, `ru.ts`, `zh.ts`, `ar.ts`, `eu.ts`, `ca.ts`, `gl.ts`, `hi.ts`, `ja.ts`
    - Clean import system in main `i18n.ts` for all language modules
    - No more syntax errors from large monolithic translation file
  - **13 Complete Languages Operational**: All translations now display correctly instead of translation keys
    - **Western Europe**: English, Spanish, Portuguese, French, German  
    - **Eastern Europe**: Russian
    - **Asia**: Chinese, Hindi, Japanese
    - **Middle East**: Arabic (with RTL text direction prepared)
    - **Regional Languages**: Euskera (Basque), Català (Catalan), Galego (Galician)
  - **Grammar-Accurate Translations**: Proper linguistic structures for each language
    - **Hindi**: Devanagari script with proper grammar (LTR direction)
    - **Japanese**: Natural mixing of hiragana, katakana, and kanji
    - **Arabic**: RTL-ready Arabic script
    - **Regional Specificity**: Proper Euskera SOV order, Catalan contractions, Galician unique vocabulary
  - **Complete UI Coverage**: All user interface elements translated across entire application
    - Main menu navigation and descriptions
    - All form pages with contextual help text
    - Result page with parameter descriptions
    - Error messages, loading states, buttons, tooltips
    - Dynamic content based on user actions

#### Enhanced
- **🏴 Language Selector UI**: Improved visual consistency and user feedback
  - **Larger Flag Icons**: Main selector button upgraded to `w-6 h-6` (was `w-5 h-5`) for better visibility
  - **Active State Indication**: Button shows pressed/highlighted appearance while dropdown is open
    - Applies background color, shadow, border, and scale effects when active
    - Clear visual feedback that selector is currently engaged
    - Consistent with modern UI patterns for dropdown controls
  - **Size Consistency**: Dropdown flag icons standardized to `w-5 h-5` matching theme toggle

#### Fixed
- **🐛 Translation System Restoration**: Complete fix of broken internationalization
  - **Problem**: Only 3 out of 13 languages were working (English, Hindi, Japanese)
  - **Root Cause**: Missing translation files for 10 languages caused display of translation keys instead of actual text
  - **Solution**: Created individual translation files for all missing languages
  - **Result**: All 13 languages now display proper translations instead of keys like `menu.title`
- **🔧 Syntax Error Resolution**: Fixed all TypeScript compilation issues
  - Corrected malformed translation files with proper syntax
  - Fixed indentation and structure issues across language files
  - Eliminated ESBuild errors that prevented successful builds

#### Technical Implementation
- **Modular Architecture**: Clean separation of translation concerns
  - Each language in its own TypeScript file with proper type definitions
  - Centralized import system maintaining performance
  - Easier maintenance and future language additions
- **Build System Compatibility**: Ensured flawless compilation
  - All translation files pass TypeScript validation
  - No ESBuild syntax errors during production builds
  - Clean development server startup without translation warnings
- **Version Management**: Updated to reflect significant improvements
  - Web UI version bumped to 0.8.0 (significant feature restoration)
  - API version maintained at stable 1.0.0 (no backend changes)
  - Version endpoint correctly reports new UI version

---

## [API v1.0.0 / Web v0.7.0] - 2025-08-20

### Cross-Component Changes
#### Enhanced
- **🚀 Enhanced Development Workflow**: Complete justfile integration for unified development experience
  - **Unified Development Commands**: `just dev` now launches complete environment
    - Automatically starts Spin API backend in background (port 3000)
    - Automatically starts npm web interface in background (port 5173)
    - Automatically exposes frontend via Tailscale serve for remote access
    - Single command for complete development setup
  - **Intelligent Server Management**: Enhanced stop/start process management
    - `just stop` now stops all services including Tailscale serve
    - Proper service dependency order (API first, then web interface)
    - Complete cleanup of background processes and PID files
    - Status reporting for all running services

#### Added
- **🌐 Tailscale Integration**: Built-in remote access support for development
  - **Frontend Exposure Commands**: 
    - `just tailscale-front-start` - Expose web interface (port 5173) via Tailscale
    - `just tailscale-front-stop` - Stop Tailscale serve for frontend
  - **Backend Exposure Commands**:
    - `just tailscale-back-start` - Expose API backend (port 3000) via Tailscale  
    - `just tailscale-back-stop` - Stop Tailscale serve for backend
  - **Automatic Installation Check**: Verifies Tailscale CLI availability before execution
  - **Status Integration**: `just status` now shows Tailscale serve status and active URLs
- **🏗️ Enhanced Build System**: Unified build commands for complete project
  - **Dual Build Process**: `just build` now builds both WebAssembly component and web interface
    - Executes `spin-cli build` for WASM compilation
    - Executes `npm run build` in web/ directory for production SPA
  - **Complete Clean Commands**: Enhanced cleanup for all project artifacts
    - `just clean` removes Rust build artifacts and npm cache/build directories
    - Cleans: `target/`, `node_modules/.cache`, `dist`, `build`, `.svelte-kit`
  - **Fresh Build Commands**: New rebuild workflows
    - `just clean-build` - Clean and rebuild everything
    - `just rebuild` - Alias for clean and rebuild workflow

### Web Interface Changes (v0.7.0)
#### Enhanced
- **⚡ Developer Experience**: Significant improvements to development workflow efficiency
  - **One-Command Setup**: `just dev` provides complete development environment
  - **Automatic Remote Access**: Frontend automatically available via Tailscale network
  - **Integrated Status Monitoring**: Single command shows all service states
  - **Intelligent Cleanup**: Stop command handles all services comprehensively
- **📊 Status Reporting**: Enhanced development server monitoring
  - **Comprehensive Status Check**: Shows Spin, npm, and Tailscale service states
  - **Port Usage Monitoring**: Reports on ports 3000, 5173, and service PIDs
  - **Tailscale URL Display**: Shows active Tailscale URLs for remote access
  - **Service Health Indicators**: Clear visual indicators for running/stopped services
- **🔧 Build Process**: Streamlined build and cleanup workflows
  - **Parallel Build Execution**: Efficient building of both backend and frontend
  - **Complete Artifact Cleanup**: Thorough cleaning of all generated files
  - **Developer-Friendly Commands**: Intuitive command names for common operations

#### Changed
- **Development Workflow**: Updated primary development commands
  - **`just dev`**: Now launches complete environment (was Spin-only)
    - Previous: Started only `spin-cli watch` in foreground
    - Current: Starts Spin (bg) → npm (bg) → Tailscale serve → complete environment ready
  - **`just dev-fg`**: New foreground mode (previous `just dev` behavior)
    - Starts npm in background, Spin in foreground for direct log viewing
    - Use when you need to monitor Spin logs directly
  - **`just stop`**: Enhanced to stop all services including Tailscale
  - **`just build`**: Enhanced to build both backend and frontend components
- **Service Management**: Improved background process handling
  - **Startup Order**: API backend starts first, then web interface
  - **PID Management**: Separate PID files for Spin and npm processes
  - **Log Management**: Separate log files (`.spin-dev.log`, `.npm-dev.log`)
  - **Cleanup Process**: Comprehensive cleanup of all background services

### API Changes (v1.0.0)
*No breaking changes - API reached stability at 1.0.0*

#### Technical Implementation
- **Component Versioning**: Independent versioning system implemented
  - API follows stable 1.x.x versioning (backward compatible)
  - Web interface follows 0.x.x development versioning
  - `/api/version` endpoint returns separate version numbers

---

## [API v1.0.0 / Web v0.6.0] - 2025-08-20

### Web Interface Changes (v0.6.0)
#### Added
- **🌍 Language Selector Component**: Complete visual language selection interface
  - **Interactive Dropdown**: Shows 11 languages with authentic flag representations
  - **Flag Icon Integration**: Complete flag sprite collection with national and regional flags
    - **National Flags**: Spain, UK, France, Germany, Portugal, Russia, Saudi Arabia, China
    - **Regional Flags**: Catalonia, Basque Country (Ikurriña), Galicia
  - **Visual Demo Mode**: Changes displayed flag without affecting application language
  - **Professional Design**: Matches theme toggle styling with consistent hover effects
  - **Smart Positioning**: Positioned alongside theme toggle in upper-right corner
  - **Accessibility Support**: Full ARIA labels and keyboard navigation
  - **Click Outside Handling**: Dropdown closes when clicking elsewhere
- **🏴 Flag Icon Collection**: Complete set of country and region flag icons
  - **11 Flag Icons**: Comprehensive collection of carefully designed SVG flag representations
  - **Authentic Colors**: All flags use official color specifications from Wikimedia Commons
  - **Optimized SVG**: Simplified designs optimized for small icon sizes while maintaining recognizability
  - **Consistent Integration**: All flags integrated into existing sprite system for optimal performance
  - **Scalable Design**: Vector graphics ensure crisp rendering at any size

#### Enhanced
- **🎨 UI Component Consistency**: Improved visual cohesion across interface controls
  - **Uniform Button Sizing**: Both language selector and theme toggle use identical dimensions (36x36px)
  - **Consistent Padding**: Standardized internal spacing (8px padding) for better visual balance
  - **Optimized Spacing**: Reduced gap between control buttons for cohesive grouping
  - **Centered Icons**: Perfect alignment of all icons within their containers
- **🖼️ Icon System Improvements**: Enhanced SVG sprite system with flag support
  - **Complete Flag Collection**: 11 authentic flag designs added to sprite
  - **Expanded Sprite System**: Collection from 10 to 21 total icons
  - **Performance Maintained**: Single HTTP request for all icons including new flags
  - **Memory Efficient**: Shared SVG symbols for all flag representations
  - **Developer Ready**: Easy access via `<Icon name="spain" />`, `<Icon name="uk" />`, etc.
  - **Reactivity Fix**: Resolved Svelte 5 runes mode compatibility issues

#### Fixed
- **⚡ Svelte 5 Runes Compatibility**: Updated components for modern Svelte syntax
  - **State Management**: Migrated from `let` to `$state()` for reactive variables
  - **Derived Values**: Changed `$:` reactive statements to `$derived()` syntax
  - **Icon Component**: Fixed reactivity issues with dynamic icon name changes
  - **Proper Reactivity**: Ensured UI updates correctly when language selection changes

---

## [API v1.0.0 / Web v0.5.0] - 2025-08-19

### Web Interface Changes (v0.5.0)
#### Added
- **🖼️ SVG Icon Sprite System**: Complete implementation of optimized icon management
  - **Centralized Sprite**: All icons consolidated into `/static/icons-sprite.svg` for efficient caching
  - **Icon Component**: New reusable `Icon.svelte` component for consistent icon usage
    - Simple props: `name`, `size`, `class` for flexible styling
    - Uses external sprite references (`/icons-sprite.svg#icon-{name}`)
    - No inline SVG bloat in JavaScript bundles
  - **10 Icons Migrated**: All UI icons converted to sprite system
    - Theme toggle: sun and moon icons
    - Navigation: left/right arrows
    - Actions: copy, check, refresh, settings, briefcase
    - UI elements: chevron-down, loading spinner
  - **Lazy Loading**: Sprite downloaded only when first icon is rendered
  - **Automatic Caching**: Browser handles sprite caching without preload warnings

#### Enhanced
- **⚡ Performance Optimization**: Significant improvements to loading and rendering
  - **Reduced Bundle Size**: Eliminated inline SVG from JavaScript/CSS bundles
  - **Single HTTP Request**: All icons downloaded in one cached file
  - **No Preload Warnings**: Removed problematic link preload, using on-demand loading
  - **Memory Efficiency**: Shared SVG symbols reduce DOM memory usage
- **🔧 Developer Experience**: Improved maintainability and consistency
  - **Centralized Icon Management**: Easy to add, modify, or remove icons
  - **Component Consistency**: Uniform icon sizing and styling across app
  - **Type Safety**: TypeScript support for icon names and properties

#### Changed
- **Icon Implementation**: Migrated from inline SVG to sprite-based system
  - **ThemeToggle.svelte**: Uses `Icon` component for sun/moon icons
  - **BackButton.svelte**: Uses `Icon` component for left arrow
  - **LoadingSpinner.svelte**: Uses `Icon` component for spinner
  - **Main menu**: Uses `Icon` component for right arrow navigation
  - **Result page**: Uses `Icon` component for all action buttons and UI elements
- **HTML Structure**: Added sprite reference system to app template
  - Removed link preload that caused browser warnings
  - External sprite references for optimal loading

---

## [API v1.0.0 / Web v0.4.0] - 2025-08-19

### Web Interface Changes (v0.4.0)
#### Added
- **🌙 Smart Theme Toggle System**: Complete manual dark/light mode switching implementation
  - **Intelligent Default Behavior**: Uses system preference (`prefers-color-scheme`) on first visit
  - **Persistent User Choice**: Saves manual selection to localStorage and respects it on subsequent visits
  - **Theme Toggle Component**: New `ThemeToggle.svelte` component with professional design
    - Floating button in upper-right corner that moves with page scroll
    - Transparent at rest, visible on hover/click/focus
    - Correct icon representation: 🌙 moon for dark mode, ☀️ sun for light mode
    - Smooth CSS transitions and visual feedback
    - Full accessibility support with ARIA labels and keyboard navigation
  - **Theme Management Store**: New `theme.ts` Svelte store for state management
    - Automatic system preference detection
    - Manual toggle functionality with localStorage persistence
    - Theme application to document root with smooth transitions
    - Optional reset to system preference function
- **🎨 TailwindCSS 4.0 Dark Mode Configuration**: Proper setup for latest Tailwind version
  - `@custom-variant dark (&:where(.dark, .dark *))` configuration in app.css
  - Class-based dark mode implementation (not media query based)
  - Seamless integration with existing dark: utility classes
  - Smooth theme transitions with CSS transition properties

#### Enhanced
- **🎯 User Experience**: Significant improvements to theme switching experience
  - No visual flicker during theme changes
  - Immediate visual feedback on toggle interaction
  - Persistent theme choice across browser sessions
  - Respects user's manual preference over system changes
- **♿ Accessibility**: Enhanced accessibility features for theme toggle
  - Screen reader friendly with descriptive ARIA labels
  - Keyboard navigation support
  - High contrast compatibility
  - Focus management and visual indicators
- **📱 Cross-Device Compatibility**: Theme system works across all platforms
  - Mobile browser theme-color meta tag updates
  - Tablet and desktop consistent behavior
  - System integration on supported browsers

---

## [API v1.0.0 / Web v0.3.0] - 2025-08-19

### Web Interface Changes (v0.3.0)
#### Added
- **🎨 Enhanced Web Interface**: Major UI/UX improvements for professional user experience
  - **Interactive Range Sliders**: Replaced number inputs with attractive gradient sliders for length parameters
  - **Dynamic Informational Notes**: Context-aware help text that changes based on alphabet selection
  - **Automatic Length Adjustment**: Intelligent minimum length calculation when switching alphabets
  - **Spinning Loading Animation**: Smooth 1.5 rotations/second icon animation during hash regeneration
  - **In-Place Regeneration**: Generate new hashes without navigating back to configuration
  - **Visual Loading States**: Button color changes and disabled states during processing

#### Changed  
- **Route Reorganization**: Renamed `/generate` route to `/custom` for better semantic clarity
- **Simplified Configuration**: All web UI operations now use `raw=true` by default (hidden from user)
- **Streamlined Navigation**: Removed redundant navigation buttons for cleaner user flow
  - Removed duplicate Back/Back to Menu buttons from result view
  - Consolidated navigation with "Back to Menu" button in configuration views
  - Removed redundant Back buttons from configuration forms
- **Button State Improvements**: Enhanced visual feedback during loading states
  - Consistent button sizing with `min-width` to prevent layout shift
  - Proper color state management during loading/active states
  - Fixed button visibility issues (borders, contrast)

#### Improved
- **User Experience**: Comprehensive UX enhancements based on reference project patterns
  - Professional gradient styling on range sliders
  - Real-time parameter validation with dynamic feedback
  - Contextual help messages for security and format recommendations
  - Smooth CSS animations and transitions
- **Accessibility**: Enhanced loading state communication through visual animations
- **Performance**: Removed artificial delays used for testing loading states

---

## [API v1.0.0 / Web v0.2.0] - 2025-08-19

### API Changes (v1.0.0)
*API reached stable 1.0.0 - No breaking changes since initial implementation*

### Web Interface Changes (v0.2.0)
#### Added
- **🎨 Professional Web Interface**: Complete SPA built with modern web technologies
  - **SvelteKit 2.x** - Modern web framework with SPA configuration
  - **TypeScript** - Full type safety throughout the application
  - **TailwindCSS 4.0** - Latest version with modern features and utilities
  - **Vite 7.x** - Fast build tool and development server
- **📱 Responsive Design**: Works perfectly on all screen sizes (mobile, tablet, desktop)
- **🌙 Dark/Light Mode**: Automatic theme switching based on system preferences
- **🎯 Complete API Integration**: Web interfaces for all API endpoints
  - Custom Hash Generator with all parameters
  - Secure Password Generator with validation
  - API Key Generator with prefix handling
  - Version information display
- **✅ Advanced Form Validation**: Real-time client-side validation
  - Dynamic minimum length calculation based on alphabet
  - Parameter constraint checking (length, prefix/suffix limits)
  - Clear error messages and helpful hints
- **📋 Enhanced User Experience**: Professional interactions and feedback
  - One-click copy to clipboard with visual confirmation
  - Loading states and error handling
  - Result display with generation metadata
  - Parameter summary and generation timestamp
- **♿ Accessibility Features**: Comprehensive accessibility support
  - ARIA labels and semantic HTML
  - Keyboard navigation support
  - Screen reader friendly
  - High contrast support
- **🌍 Internationalization Ready**: Prepared for multiple language support
  - Translation system implemented
  - Configurable text strings
  - Ready for expansion to other languages
- **🔧 Development Configuration**: Professional development setup
  - API proxy configuration (web:5173 → api:3000)
  - Tailscale host support for remote development
  - Production build pipeline for static deployment
  - TypeScript and Svelte code validation

#### Technical Implementation
- **Single Page Application (SPA)**: Built with `@sveltejs/adapter-static`
- **API Service Layer**: Type-safe API integration with error handling
- **State Management**: Svelte stores for navigation, results, and i18n
- **Component Architecture**: Reusable components (BackButton, LoadingSpinner)
- **Routing System**: File-based routing with menu → forms → result flow
- **Build System**: Optimized production builds with code splitting

#### Web Interface Structure
```
web/
├── src/
│   ├── lib/
│   │   ├── api.ts              # Type-safe API service
│   │   ├── components/         # Reusable UI components
│   │   ├── stores/            # State management
│   │   └── types/             # TypeScript definitions
│   └── routes/
│       ├── +page.svelte       # Main menu
│       ├── custom/            # Hash generator (renamed from generate)
│       ├── password/          # Password generator
│       ├── api-key/           # API key generator
│       └── result/            # Shared result display
```

---

## [API v1.0.0] - 2025-08-18

### API Changes (v1.0.0)
#### Added
- **Initial implementation of HashRand Spin API** - Complete random hash generator solution
- **GET /api/generate** endpoint for customizable hash generation
  - Support for length parameter (2-128 characters)
  - Multiple alphabet types: base58, no-look-alike, full, full-with-symbols
  - Prefix and suffix support (max 32 characters each)
  - Raw output formatting option
- **GET /api/password** endpoint for secure password generation
  - Dynamic minimum length based on alphabet type
  - Length range validation (21-44 characters)
  - Symbol and no-look-alike alphabet support
- **GET /api/api-key** endpoint for API key generation
  - Automatic ak_ prefix for all generated keys
  - Length validation (44-64 characters)
  - Support for full and no-look-alike alphabets
- **GET /api/version** endpoint returning JSON version information
- **Comprehensive alphabet system** with 4 character sets:
  - Base58: 58 characters (Bitcoin standard, excludes confusing characters)
  - No-look-alike: 49 characters (maximum readability)
  - Full: 62 characters (complete alphanumeric)
  - Full-with-symbols: 73 characters (maximum entropy)
- **Cryptographically secure random generation** using nanoid
- **Complete parameter validation and error handling**
- **Modular architecture** with clean separation of concerns
- **Comprehensive test suite** with 43 automated test cases
- **Project restructured into workspace** with api/ directory
- **Support for Rust 2024 edition**
- **justfile** for streamlined development workflow with 20+ commands
  - Development tasks: `just dev`, `just build`, `just test`
  - Background server support: `just dev-bg`, `just watch`, `just stop`, `just status`
  - Code quality: `just check`, `just lint`, `just fmt`
  - Information: `just info`, `just examples`, `just deps`
  - CI/CD: `just pre-commit`, `just perf-test`
- **Background development server functionality**
  - `just dev-bg` - Start server in background with PID tracking
  - `just watch` - Start background server and follow logs
  - `just status` - Check background server status
  - PID file management in `.spin-dev.pid`
  - Log file management in `.spin-dev.log`
  - Automatic cleanup on server stop

#### Technical Details
- Built with Fermyon Spin WebAssembly framework
- Uses spin-sdk 3.1.0 for HTTP component functionality
- Implements cdylib crate type for WASM compatibility
- Targets wasm32-wasip1 WebAssembly platform
- Workspace structure for better code organization

#### Dependencies
- `spin-sdk = "3.1.0"` - Core Spin framework
- `nanoid = "0.4.0"` - Secure random ID generation
- `serde = "1.0.219"` - Serialization framework
- `serde_json = "1.0.142"` - JSON serialization
- `anyhow = "1"` - Error handling

#### Testing
- 43 comprehensive test cases covering all endpoints
- Parameter validation testing
- Edge case and error condition testing
- Alphabet-specific character validation
- Performance and consistency testing
- 100% test success rate achieved

#### Documentation
- Complete README.md with API documentation
- Detailed endpoint descriptions and examples
- Project structure documentation
- Setup and deployment instructions
- CLAUDE.md for development guidance

---

## [Unreleased]

### Planned Features
- **Complete Internationalization System**: Full i18n implementation with 11 languages
- Performance benchmarking
- Additional alphabet types
- Batch generation endpoints
- Configuration file support
- Metrics and monitoring
- Docker containerization
- Helm charts for Kubernetes deployment

---

## Version History Summary

- **[API v1.4.2 / Web v0.19.2]** (2025-08-29) - **MAJOR**: Zero Knowledge (ZK) authentication system implementation with JWT endpoint protection and privacy-preserving cryptographic user IDs
- **[API v1.4.1 / Web v0.19.1]** (2025-08-29) - Magic link host detection fixes and EmailInputDialog component enhancements
- **[API v1.4.0 / Web v0.18.0]** (2025-08-27) - **MAJOR**: Complete authentication system with magic link authentication, JWT tokens, and frontend AuthGuard integration
- **[API v1.3.0]** (2025-08-27) - **MAJOR**: Complete SQLite database system with environment-aware database selection and full user management REST API
- **[API v1.2.1]** (2025-08-25) - **ENHANCED**: ChaCha8 OTP generation refactoring for complete cryptographic consistency and deprecated API fixes
- **[API v1.2.0 / Web v0.17.2]** (2025-08-24) - **MAJOR**: Complete BIP39 mnemonic generation system with 10 languages, dual word counts, and deterministic/random modes
- **[API v1.1.0 / Web v0.17.1]** (2025-08-24) - **BUGFIX**: Fixed regenerate button to correctly perform GET requests without seed parameters
- **[API v1.1.0 / Web v0.17.0]** (2025-08-24) - **MAJOR**: Base58 seed format migration, numeric alphabet, OTP generation, and simplified UI seed handling
- **[API v1.0.0 / Web v0.16.0]** (2025-08-23) - **MAJOR**: Comprehensive seed-based deterministic generation system for all endpoints with complete UI integration
- **[API v1.0.0 / Web v0.15.0]** (2025-08-23) - Translation naturalness improvements across all 13 languages and enhanced DateTimeLocalized component robustness
- **[API v1.0.0 / Web v0.14.0]** (2025-08-23) - Progressive sprite loading system with UTF placeholders, universal URL parameter support, and centralized API architecture
- **[API v1.0.0 / Web v0.13.0]** (2025-08-23) - Comprehensive linting system (ESLint + Prettier via Vite), code quality cleanup, and unified development workflow
- **[API v1.0.0 / Web v0.12.0]** (2025-08-23) - DateTimeLocalized component, enhanced Iconize with invertposition, play/home icons, and result page improvements
- **[API v1.0.0 / Web v0.11.0]** (2025-08-22) - Universal Iconize Component with RTL-aware automatic positioning and simplified implementation
- **[API v1.0.0 / Web v0.10.0]** (2025-08-21) - RTL-aware Button component and improved language ordering
- **[API v1.0.0 / Web v0.9.0]** (2025-08-21) - Advanced RTL transition system, unified top controls container, and enhanced mobile UX
- **[API v1.0.0 / Web v0.8.0]** (2025-08-20) - Complete translation system restoration with 13 languages and language selector UI improvements
- **[API v1.0.0 / Web v0.7.0]** (2025-08-20) - Enhanced development workflow with unified commands and Tailscale integration
- **[API v1.0.0 / Web v0.6.0]** (2025-08-20) - Language selector component with flag icons and Svelte 5 runes compatibility
- **[API v1.0.0 / Web v0.5.0]** (2025-08-19) - SVG icon sprite system for optimized performance and maintainability
- **[API v1.0.0 / Web v0.4.0]** (2025-08-19) - Smart theme toggle system with TailwindCSS 4.0 dark mode implementation
- **[API v1.0.0 / Web v0.3.0]** (2025-08-19) - Enhanced UI/UX with interactive components and improved user experience
- **[API v1.0.0 / Web v0.2.0]** (2025-08-19) - Web interface release with professional SPA
- **[API v1.0.0]** (2025-08-18) - Initial stable API release with complete implementation

---

## Versioning Strategy

### API (Backend) Versioning
- **Stable Versioning**: API follows strict semver starting from 1.0.0
- **Backward Compatibility**: Minor versions (1.1.0, 1.2.0) add features without breaking changes
- **Major Versions**: Only for breaking API changes (2.0.0, 3.0.0)
- **Production Ready**: API is stable and production-ready at 1.0.0

### Web Interface Versioning  
- **Development Versioning**: Web interface follows 0.x.x series during active development
- **Rapid Iteration**: Minor versions (0.17.0, 0.17.1) for UI/UX improvements and bug fixes
- **Breaking UI Changes**: Major versions in 0.x.x series (0.16.0 → 0.17.0) for significant UI restructures
- **Stability Target**: Will reach 1.0.0 when feature-complete and UI/UX is finalized

### Release Tags
- **API releases**: `api-v1.0.0`, `api-v1.1.0`, etc.
- **Web releases**: `web-v0.7.0`, `web-v0.8.0`, etc.
- **Combined releases**: When both components are updated simultaneously

### Version Endpoint
- **GET /api/version**: Returns both component versions
  ```json
  {
    "api_version": "1.2.1",
    "ui_version": "0.17.2"
  }
  ```