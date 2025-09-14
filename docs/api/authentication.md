# Zero Knowledge Authentication System

HashRand implements a **true Zero Knowledge architecture** where the server operates with complete user privacy, never storing or processing personal identifying information.

## Authentication Flow

```
POST /api/login/         # Generate magic link (no email storage)
GET /api/login/?magiclink=...  # Validate magic link and get JWT tokens
DELETE /api/login/       # Clear refresh token cookie (logout)
POST /api/refresh        # Refresh expired access tokens using HttpOnly cookies
```

## Magic Link Generation

**POST /api/login/:**
```json
{
  "email": "user@example.com",
  "ui_host": "http://localhost:5173",
  "next": "/result?endpoint=mnemonic&language=english&words=12",
  "email_lang": "es"
}
```

**Request Parameters:**
- `email` (required) - User email address for magic link delivery
- `ui_host` (optional) - Frontend URL for magic link generation
- `next` (optional) - URL path for post-authentication redirection (e.g., "/result?endpoint=mnemonic&words=12")
- `email_lang` (optional) - Language code for email template (e.g., "es", "fr", "ar")

**Response:**
```json
{
  "message": "Magic link generated successfully. Check development logs for the link.",
  "dev_magic_link": "http://localhost:5173/?magiclink=Ax1wogC82pgTzrfDu8QZhr"
}
```

## Magic Link Validation

**GET /api/login/?magiclink=TOKEN**

**Response:**
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer", 
  "expires_in": 180,
  "user_id": "HpGAge9YJ7uMvw4QV5qDPk",
  "next": "/result?endpoint=mnemonic&language=english&words=12"
}
```

## JWT Dual Token System with 2/3 Time-Based Refresh Logic

- **Access Token**: 1 minute validity (development: testing), 20 minutes (production)
- **Refresh Token**: 4 minutes validity (development: testing), 240 minutes (production)
- **HttpOnly Cookie**: Refresh token stored securely with HttpOnly, Secure, SameSite=Strict

### Enterprise-Grade 2/3 Time-Based Token Management

HashRand implements intelligent token lifecycle management that optimizes security and user experience:

#### **Two-Phase Refresh Strategy:**

1. **First 1/3 of Refresh Token Lifetime (Conservative Phase)**
   - **Time Window**: 0 → 80 seconds (for 4-minute refresh tokens)
   - **Behavior**: Only renews access token, preserves existing refresh token
   - **Rationale**: Minimizes token regeneration overhead while maintaining security

2. **Last 2/3 of Refresh Token Lifetime (Security Reset Phase)**
   - **Time Window**: 80+ seconds → expiration (for 4-minute refresh tokens)
   - **Behavior**: Complete token pair reset (both access + refresh renewed)
   - **Rationale**: Proactive security hardening when approaching expiration

#### **Automatic Token Refresh Flow:**

```bash
# Phase 1: Conservative refresh (first 1/3 - keeps existing refresh token)
curl -X POST "http://localhost:3000/api/custom?length=8" \
  -H "Authorization: Bearer expired-access-token" \
  -H "Cookie: refresh_token=valid-refresh-token"
# Response: HTTP 200 + x-new-access-token header (no Set-Cookie)

# Phase 2: Security reset (last 2/3 - resets both tokens)
curl -X POST "http://localhost:3000/api/custom?length=8" \
  -H "Authorization: Bearer expired-access-token" \
  -H "Cookie: refresh_token=valid-but-aging-token"
# Response: HTTP 200 + x-new-access-token + Set-Cookie: refresh_token=new-token
```

#### **Dual Token Expiration Handling:**

When both access and refresh tokens expire simultaneously, the system provides comprehensive cleanup:

```bash
# Both tokens expired scenario
curl -X POST "http://localhost:3000/api/custom?length=8" \
  -H "Authorization: Bearer expired-access-token" \
  -H "Cookie: refresh_token=expired-refresh-token"
# Response: HTTP 401
# Body: {"error": "Both access and refresh tokens have expired. Please re-authenticate."}
# Headers: Set-Cookie: refresh_token=; Max-Age=0; HttpOnly; Secure; SameSite=Strict; Path=/
```

**Frontend Integration:**
- **Smart Detection**: `isDualTokenExpiry()` identifies dual expiration scenarios
- **Automatic Cleanup**: `handleDualTokenExpiry()` clears sessionStorage and triggers re-authentication
- **Seamless UX**: Users see clean authentication dialog without technical error messages

### Logout System

```bash
# Logout and clear refresh token cookie
curl -X DELETE "http://localhost:3000/api/login/"
# Response: {"message": "Logged out successfully"}
# Sets: Set-Cookie: refresh_token=; HttpOnly; Secure; SameSite=Strict; Max-Age=0; Path=/
```

## Zero Knowledge Features

### Complete Data Privacy
- **No Email Storage**: Server never stores or processes email addresses
- **Email Privacy**: Emails used only for magic link delivery, never stored
- **Audit Trail Privacy**: All logs use Base58 usernames, not personal data
- **Compliance Ready**: GDPR/CCPA compliant by design - no personal data to manage

### Cryptographic User Identity System

```
Email Input → Blake2b Hash → Blake2b-keyed → Per-User Salt → Argon2id → Blake2b-variable → 16-byte user_id
                               (hmac_key)     (unique salt)   (19456KB)                      ↓
                                                                                    Base58 Username Display (~22 chars)
```

**Key Properties:**
- **Deterministic**: Same email always generates same user_id for consistency
- **One-Way**: Cryptographically impossible to reverse user_id back to email
- **High Security**: Argon2id with 19456KB memory cost following OWASP 2024 standards
- **User-Friendly**: Base58 encoding provides readable usernames without confusing characters

### Magic Link Cryptographic Verification & Encryption

```
User_ID + Timestamp → ChaCha8RNG[44] → nonce[12] + secret_key[32] → ChaCha20 Encrypt → Base58 Token (32 bytes → 44 chars)
Blake2b-keyed(raw_magic_link, hmac_key) → Blake2b-variable[16] → Database Hash Index
```

**Security Architecture:**
- **ChaCha20 Encryption**: 32-byte encrypted magic link data using ChaCha20 stream cipher
- **Blake2b-keyed Integrity**: Prevents modification and tampering of magic links
- **Database Validation**: Additional security layer through token presence verification
- **Time-Limited**: 5-minute expiration prevents replay attacks (development: 15 minutes)
- **One-Time Use**: Magic links consumed immediately after validation
- **Optimized Length**: 44-character Base58 tokens (reduced from previous 66-character implementation)
- **No Email Reference**: Magic tokens contain only cryptographic hashes, never emails

## Email Integration & Multilingual Support

### Production Email Delivery

The magic link authentication system includes **complete Mailtrap email integration** with enhanced text-plain internationalization:

```bash
# Email delivery via Mailtrap REST API
- **Endpoint**: https://sandbox.api.mailtrap.io/api/send/{inbox_id}
- **Authentication**: Bearer token authentication
- **Format**: HTML + plain text dual format with text-specific translations
- **Internationalization**: 13 languages with dedicated plain text optimization
- **Architecture**: Proper separation of HTML and text concerns
- **Confirmation**: HTTP 200/202 status validation with error handling
- **Fallback**: Console logging when email delivery fails (development mode)
```

### Comprehensive Multilingual Email Templates

Magic link emails are delivered in **13 languages** matching the web UI language selector:

**Supported Languages:**
- **🇪🇸 Spanish** (`es`) - Español con terminología nativa profesional
- **🇺🇸 English** (`en`) - Professional technical terminology (default)
- **🇫🇷 French** (`fr`) - Français avec terminologie technique précise
- **🇩🇪 German** (`de`) - Deutsch mit professioneller technischer Sprache
- **🇵🇹 Portuguese** (`pt`) - Português europeu com terminologia técnica
- **🇷🇺 Russian** (`ru`) - Русский с технической терминологией
- **🇨🇳 Chinese** (`zh`) - 中文（简体）技术术语
- **🇯🇵 Japanese** (`ja`) - 日本語の技術用語
- **🇸🇦 Arabic** (`ar`) - العربية مع اتجاه النص من اليمين إلى اليسار
- **🇮🇳 Hindi** (`hi`) - हिन्दी तकनीकी शब्दावली के साथ
- **🏴󠁥󠁳󠁣󠁴󠁿 Catalan** (`ca`) - Català amb terminologia tècnica precisa
- **🏴󠁥󠁳󠁧󠁡󠁿 Galician** (`gl`) - Galego con terminoloxía técnica
- **🏴󠁥󠁳󠁰󠁶󠁿 Basque** (`eu`) - Euskera termino tekniko egokiekin

### Email Template Features
- **HTML + Plain Text**: Dual format with text-specific translations for optimal compatibility
- **Text-Specific Translation Keys**: Dedicated plain text versions optimized for text-only email clients
  - `text_intro`: "Use the link below" (vs HTML "Click the button below")
  - `text_access_label`: "Access Link to HashRand" (text-appropriate instructions)
  - `text_security_section`: Localized security headers ("⚠️ Security Information:")
- **RTL Support**: Arabic template includes `dir="rtl"` for proper right-to-left display
- **Professional Branding**: Consistent "HashRand" branding across all languages
- **Security Messaging**: Clear magic link expiration and security information in each language
- **Cultural Adaptation**: Native terminology and proper grammar for each language
- **Architectural Separation**: HTML concerns (buttons, styling) properly separated from plain text
- **Fallback System**: Automatic fallback to English for unsupported language codes

### Email Configuration
```env
# Required environment variables for email integration
SPIN_VARIABLE_MAILTRAP_API_TOKEN=your-mailtrap-api-token
SPIN_VARIABLE_MAILTRAP_INBOX_ID=your-inbox-id

# Optional email settings
SPIN_VARIABLE_FROM_EMAIL=noreply@hashrand.dev  # Default sender
```

## Usage Examples

```bash
# Request magic link
curl -X POST "http://localhost:3000/api/login/" \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com"}'

# Validate magic link (from development log)
curl "http://localhost:3000/api/login/?magiclink=Ax1wogC82pgTzrfDu8QZhr"

# Request magic link in Spanish
curl -X POST "http://localhost:3000/api/login/" \
  -H "Content-Type: application/json" \
  -d '{"email": "usuario@ejemplo.com", "email_lang": "es"}'

# Request magic link in Arabic (RTL support)
curl -X POST "http://localhost:3000/api/login/" \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "email_lang": "ar"}'

# Request magic link with fallback to English
curl -X POST "http://localhost:3000/api/login/" \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "email_lang": "unsupported"}'
```

## Zero Knowledge Architecture Benefits

### Scalability & Performance
- **Deterministic Lookups**: Same email always produces same user_id for O(1) user identification
- **No PII Indexes**: Database indexes only on cryptographic hashes, never personal data
- **Stateless Sessions**: JWT tokens eliminate need for server-side session storage
- **Horizontal Scaling**: Zero Knowledge architecture supports distributed deployments

### Development & Operations
- **Safe Logging**: All application logs use Base58 usernames, safe to store and analyze
- **Testing Friendly**: Short token durations (20s access, 2min refresh) enable rapid testing cycles
- **Debug Safety**: Development logs never contain personal information
- **Incident Response**: Security incidents don't expose user personal data

---

*For endpoint usage, see [API Endpoints](./endpoints.md)*  
*For cryptographic details, see [Cryptography Documentation](./cryptography.md)*