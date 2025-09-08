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
- **Persistent User Choice**: Theme preference saved in localStorage
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

All generator pages support GET parameters for direct configuration and sharing:

### Supported Parameters

```bash
# Custom hash generator with parameters
http://localhost:5173/custom/?length=32&alphabet=base58&prefix=app_&suffix=_v1

# Password generator with specific settings  
http://localhost:5173/password/?length=25&alphabet=no-look-alike

# API key generator with custom length
http://localhost:5173/api-key/?length=50&alphabet=full

# BIP39 mnemonic with language and word count
http://localhost:5173/mnemonic/?language=spanish&words=24
```

### Centralized API Architecture
- **Generator Pages** (`/custom/`, `/password/`, `/api-key/`, `/mnemonic/`): Handle UI and navigation with GET parameter support
- **Result Page** (`/result/`): Centralized API calling based on URL parameters  
- **Fresh Generation**: Result page always generates new values, never displays cached results
- **Shareable URLs**: Complete configuration can be shared via URL parameters

## Authentication Integration

### 🔐 Privacy-First Authentication System
- **Explore First, Authenticate Later**: All generator pages accessible without login
- **On-Demand Authentication**: Authentication dialog appears only when clicking "Generate"
- **Privacy-First Design**: Server never stores or processes email addresses
- **Dialog-Based Protection**: Modern authentication flow with modal dialogs for all generation routes
- **Always-Visible Session Button**: User icon (👤) always visible for consistent authentication access

### Authentication Dialog System
- **Unified Experience**: Consistent authentication across all generation pages
- **Two-step Process**: Email input with validation and confirmation
- **Advanced State Preservation**: Form parameters encoded as base58 URL-safe strings
- **Universal Integration**: Works across all generator pages (custom/, password/, api-key/, mnemonic/)
- **Professional Design**: Matches existing dialog components

### State Management
- **State-Aware Form Handling**: Automatic parameter preservation through authentication flow
- **JSON Parameter Encoding**: Form parameters encoded as base58 URL-safe strings
- **Temporary Storage**: Parameters stored in localStorage with automatic cleanup
- **Seamless Restoration**: Form state restored after authentication completion
- **Clean User Flow**: Smooth transition from form → authentication → result generation

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
- **Read-only Seed Display**: Seeds shown only when provided via URL parameters (non-editable)
- **Base58 Validation**: 44-character base58 seed format with visual feedback
- **Smart UI Behavior**: Regenerate button hidden only when seed provided via URL parameters
- **Informational Display**: Seeds shown as informational text without copy functionality
- **Simplified Integration**: Clean seed handling without complex UI interactions

### URL Seed Integration
```bash
# URL with seed parameter (read-only display)
http://localhost:5173/custom/?seed=2R7KDyMvBTv3WLAY8AAiBNFgBkv7zHvjpTp6U2eWMGfR&length=32

# API-generated seeds (informational metadata)
# Displayed as informational metadata without copy functionality
```

### Seed Behavior
- **URL-provided seeds**: Shows as read-only informational text
- **API-generated seeds**: Displayed as informational metadata without copy functionality
- **No Seed Input Fields**: Simplified interface without complex interactions
- **44-character Base58**: Standard format with visual validation

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
- **LocalStorage Integration**: Persistent storage for user preferences
- **URL State Sync**: Parameters synchronized with browser URL
- **Form State Persistence**: Automatic form state preservation

### Build & Deployment
- **SPA Build**: Static build for deployment to any static hosting
- **Asset Optimization**: Optimized images, icons, and resources
- **Progressive Loading**: Lazy loading of non-critical resources
- **Cache Headers**: Optimized caching strategy for static assets

---

*For internationalization features, see [Internationalization Documentation](./internationalization.md)*  
*For component architecture, see [Components Documentation](./components.md)*  
*For deployment options, see [Production Deployment](../deployment/production.md)*