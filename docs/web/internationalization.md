# Internationalization System

HashRand features **complete internationalization** with support for **13 languages**, including advanced RTL support, cultural adaptations, and professional translation quality.

## Supported Languages

### 🌍 13-Language Support

| Language             | Code | Direction | Features                            |
| -------------------- | ---- | --------- | ----------------------------------- |
| **🇪🇸 Spanish**       | `es` | LTR       | European Spanish terminology        |
| **🇺🇸 English**       | `en` | LTR       | Default language, technical terms   |
| **🇫🇷 French**        | `fr` | LTR       | Professional technical terminology  |
| **🇩🇪 German**        | `de` | LTR       | Technical German terminology        |
| **🇵🇹 Portuguese**    | `pt` | LTR       | European Portuguese preference      |
| **🇷🇺 Russian**       | `ru` | LTR       | Technical Russian terminology       |
| **🇨🇳 Chinese**       | `zh` | LTR       | Simplified Chinese, technical terms |
| **🇯🇵 Japanese**      | `ja` | LTR       | Technical Japanese, SOV grammar     |
| **🇸🇦 Arabic**        | `ar` | RTL       | Right-to-left with proper direction |
| **🇮🇳 Hindi**         | `hi` | LTR       | Native terminology over anglicisms  |
| **🏴󠁥󠁳󠁣󠁴󠁿 Catalan**  | `ca` | LTR       | Technical Catalan terminology       |
| **🏴󠁥󠁳󠁧󠁡󠁿 Galician** | `gl` | LTR       | Technical Galician terminology      |
| **🏴󠁥󠁳󠁰󠁶󠁿 Basque**   | `eu` | LTR       | Ergative/absolutive cases           |

## Translation Quality Standards

### 🎯 Professional Translation Quality

All translations undergo comprehensive review and enhancement for authentic, natural language use:

#### Linguistic Authenticity

- **Native Terminology Preference**: Use of native terms over anglicisms
  - Hindi: "लंबाई" (lambāī) instead of "लेंथ" (length)
  - Spanish: "contraseña" instead of "password" borrowing
- **Regional Variations**: Respect for regional linguistic preferences
  - European Portuguese: "palavras-passe" vs Brazilian "senhas"
  - Spanish: Technical terminology aligned with RAE standards

#### Technical Precision

- **Consistent Terminology**: Uniform translation of technical terms
  - "Characters" vs "letters" distinction maintained across Portuguese, French, and Catalan
  - Cryptographic terminology consistently translated
- **Grammar Compliance**: Proper grammatical structures for each language
  - Basque: Correct ergative/absolutive case usage
  - Japanese: Proper SOV (Subject-Object-Verb) word order
  - German: Appropriate compound word formation

#### Cultural Adaptation

- **RTL Optimization**: Arabic terminology optimized for right-to-left reading
- **Cultural Context**: Chinese range expressions using culturally appropriate formats
- **Technical Communication**: Adapted technical communication styles per language

## RTL (Right-to-Left) Support

### 🔄 Advanced RTL Implementation

#### Arabic Language Support

- **Automatic Direction Detection**: Browser-native RTL behavior
- **Zero-Config RTL**: Built-in RTL support without manual text direction handling
- **Complex Flag Integration**: Full-resolution flag SVGs for Arabic regions
- **Email Template RTL**: Arabic email templates include `dir="rtl"` attribute

#### RTL-Aware Components

- **Universal Iconize Component**: Revolutionary RTL-aware wrapper for any content
- **Smart RTL Buttons**: Automatic icon positioning for right-to-left languages
- **Seamless Direction Changes**: Smooth transitions between text directions
- **Never Manual Handling**: Browser-native behavior eliminates manual text direction management

## Advanced Localization Features

### 📅 Date Localization System

**DateTimeLocalized Component** with multi-level fallbacks:

- **Browser Compatibility**: Intelligent detection of failed locale support
- **Custom Fallbacks**: Authentic Galician abbreviations and manual formatting
- **Cross-Platform Reliability**: Works on all browser engines with graceful degradation
- **Multi-level Fallbacks**: Multiple fallback layers for robust date display

#### Fallback Hierarchy

```javascript
1. Native browser Intl.DateTimeFormat with target locale
2. Custom language-specific formatting
3. Manual fallback with authentic abbreviations
4. English fallback as ultimate safety net
```

### 🏳️ Complex Flag Integration

- **Multiple Regions**: Full-resolution flag SVGs from various regions
- **Regional Flags**: Euskadi (Basque), Catalonia, Galicia flags included
- **High Quality**: Professional flag SVGs with zero quality compromise
- **Optimized Loading**: Progressive loading with immediate placeholders

### 🔤 Language Ordering

- **Alphabetically Organized**: Languages ordered by native language names
- **Native Script Priority**: Languages displayed in their native scripts
- **Cultural Sensitivity**: Respectful presentation of all languages
- **Easy Selection**: Intuitive language selection interface

## Email Internationalization

### 📧 Multilingual Email Templates

All magic link authentication emails support the full 13-language set with **dedicated text-specific translations**:

#### Template Features

- **HTML + Plain Text**: Dual format ensures compatibility with all email clients
- **Text-Specific Translations**: Separate translation keys optimized for plain text email clients
  - `text_intro`: Plain text version without HTML button references ("Use the link below" vs "Click the button below")
  - `text_access_label`: Text-appropriate access instructions ("Access Link to HashRand")
  - `text_security_section`: Localized security information headers ("⚠️ Security Information:")
- **RTL Email Support**: Arabic templates include proper `dir="rtl"` markup for HTML and native RTL text
- **Professional Branding**: Consistent "HashRand" branding across all languages
- **Security Messaging**: Clear magic link expiration and security information
- **Cultural Adaptation**: Native terminology and proper grammar for each language
- **Architectural Separation**: HTML concerns (buttons, styling) properly separated from plain text content
- **Fallback System**: Automatic fallback to English for unsupported language codes

#### Usage Examples

```bash
# Spanish email template
curl -X POST "http://localhost:3000/api/login/" \
  -H "Content-Type: application/json" \
  -d '{"email": "usuario@ejemplo.com", "email_lang": "es"}'

# Arabic email with RTL support
curl -X POST "http://localhost:3000/api/login/" \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "email_lang": "ar"}'

# Fallback to English for unsupported codes
curl -X POST "http://localhost:3000/api/login/" \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "email_lang": "unsupported"}'
```

## Implementation Architecture

### Frontend i18n System

- **Svelte Store Integration**: Reactive translation system
- **Dynamic Language Loading**: Load translations on demand
- **Persistent Language Selection**: User preference stored in IndexedDB (v0.19.14+)
- **URL Parameter Support**: Language can be set via URL parameters
- **Fallback Handling**: Graceful fallbacks for missing translations

### Translation Keys Structure

```json
{
  "nav": {
    "custom": "Custom Hash",
    "password": "Password",
    "apiKey": "API Key",
    "mnemonic": "Mnemonic"
  },
  "auth": {
    "login": "Login",
    "email": "Email Address",
    "sendMagicLink": "Send Magic Link"
  },
  "generation": {
    "length": "Length",
    "alphabet": "Alphabet",
    "generate": "Generate"
  },
  "email": {
    "magic_link": {
      "subject": "Your Magic Link for HashRand",
      "title": "HashRand",
      "subtitle": "Random Hash Generator",
      "greeting": "Hello!",
      "intro": "You requested a secure login link. Click the button below to authenticate and access your account.",
      "button_text": "Access HashRand",
      "manual_link_intro": "If the button doesn't work, copy and paste this link into your browser:",
      "security_warning": "This link will expire in 5 minutes and can only be used once.",
      "security_notice": "If you didn't request this login link, please ignore this email.",
      "footer_text": "HashRand - Random Hash Generator",
      "no_reply_notice": "This is an automated message. Please do not reply to this email.",
      "text_intro": "You requested a secure login link. Use the link below to authenticate and access your account.",
      "text_access_label": "Access Link to HashRand",
      "text_security_section": "⚠️ Security Information:"
    }
  }
}
```

### BIP39 Mnemonic Language Integration

The BIP39 mnemonic generation supports native language word lists:

- **English** (english, en) - Default BIP39 word list
- **Spanish** (spanish, es) - Official Spanish BIP39 words
- **French** (french, fr) - French BIP39 word list
- **Portuguese** (portuguese, pt) - Portuguese BIP39 words
- **Japanese** (japanese, ja) - Japanese Hiragana BIP39 words
- **Chinese Simplified** (chinese, zh) - Simplified Chinese characters
- **Chinese Traditional** (chinese-traditional, zh-tw) - Traditional Chinese
- **Italian** (italian, it) - Italian BIP39 word list
- **Korean** (korean, ko) - Korean BIP39 words
- **Czech** (czech, cs) - Czech BIP39 word list

## Development & Maintenance

### Translation Management

- **Centralized Translation Files**: All translations in structured JSON files
- **Translation Scripts**: Automated scripts for adding missing translations
- **Quality Assurance**: Professional review process for all translations
- **Consistency Checks**: Automated checks for translation completeness
- **Cultural Review**: Native speaker review for cultural appropriateness

### Testing & Quality

- **Multi-language Testing**: Automated tests for all supported languages
- **RTL Testing**: Specific tests for right-to-left languages
- **Email Template Testing**: Validation of email templates in all languages
- **Accessibility Testing**: Screen reader testing for all languages
- **Visual Regression**: UI testing across all language variants

---

_For interface features, see [Interface Documentation](./interface.md)_  
_For component architecture, see [Components Documentation](./components.md)_  
_For authentication integration, see [Authentication Documentation](../api/authentication.md)_
