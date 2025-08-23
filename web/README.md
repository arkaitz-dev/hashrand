# HashRand Spin Web Interface

Professional web interface for the HashRand Spin API - a modern SPA built with SvelteKit 2.x, TypeScript, and TailwindCSS 4.0.

## 🎯 Features

### Core Functionality

- **Modern SPA Architecture**: Built with SvelteKit as a Single Page Application with static adapter
- **Multi-endpoint Support**: Complete interfaces for all API endpoints:
  - **Custom Hash Generator** (`/api/generate`) - accessible via `/custom` route
  - **Secure Password Generator** (`/api/password`) - with strength validation
  - **API Key Generator** (`/api/api-key`) - with `ak_` prefix handling
  - **Version Information** (`/api/version`) - displays both API and UI versions
- **Professional Design**: Clean, responsive UI with comprehensive design system

### 🎨 Visual Design System

- **🌙 Smart Theme Toggle**: Manual dark/light mode switching with system preference detection
  - Intelligent default: Uses system preference on first visit
  - Persistent choice: Manual selection saved to localStorage
  - Floating toggle in upper-right corner with smooth transitions
  - Accessible with ARIA labels and keyboard navigation
- **🖼️ SVG Icon Sprite System**: Optimized icon management with 21+ icons
  - Centralized sprite at `/static/icons-sprite.svg` for efficient caching
  - Universal `Icon.svelte` component for consistent usage
  - Theme icons (sun/moon), navigation arrows, action buttons, UI elements
- **🌍 Complete Internationalization**: Full i18n system with 13 languages featuring professional translation quality
  - **Languages**: English, Spanish, Portuguese, French, German, Russian, Chinese, Arabic, Hindi, Japanese, Euskera, Català, Galego
  - **Enhanced Naturalness**: Comprehensive linguistic review ensuring authentic, natural-sounding translations
    - **Native Terminology**: Preferred pure language terms over anglicisms (Hindi, Russian, German)
    - **Regional Accuracy**: European Portuguese vs Brazilian distinctions, authentic Basque cases
    - **Technical Consistency**: Unified "characters" terminology across Romance languages
    - **Cultural Adaptation**: Proper Arabic RTL terminology and Chinese natural expressions
  - **RTL Support**: Right-to-left text direction for Arabic with automatic behavior
  - **Language Selector**: Interactive dropdown with authentic flag representations
  - **Regional Flags**: Includes Catalonia, Basque Country (Ikurriña), Galicia flags
  - **Robust Date Localization**: Advanced fallback system for cross-browser compatibility

### 🔧 Advanced UI Components

- **🔄 Universal Iconize Component**: Revolutionary RTL-aware wrapper for any content
  - **Smart RTL Behavior**: Automatically positions icons correctly for LTR and RTL languages
  - **Dual Icon Support**: Works with both SVG sprite icons and Unicode emojis
  - **Flexible Positioning**: `invertposition` parameter controls content order
  - **Zero Configuration RTL**: Uses HTML `dir="rtl"` and browser-native flexbox behavior
- **📅 DateTimeLocalized Component**: Portable date/time formatting for internationalization
  - **Universal Date Formatting**: Handles 13 languages with proper locale detection
  - **Custom Euskera Format**: Special handling for Basque language formatting
  - **Configurable Options**: Accepts `Intl.DateTimeFormatOptions` for custom formatting
- **📱 Responsive Design**: Mobile-first approach with breakpoint optimization
  - **Touch Optimization**: Larger touch targets and improved mobile spacing
  - **Adaptive Layouts**: Works perfectly on mobile, tablet, and desktop
  - **Progressive Enhancement**: Enhanced features for larger screens

### ⚡ Interactive Features

- **Interactive Range Sliders**: Beautiful gradient sliders for length parameter selection
- **Dynamic Help Text**: Context-aware informational notes based on alphabet selection
- **Automatic Adjustments**: Smart minimum length calculation when changing alphabets
- **In-Place Regeneration**: Generate new hashes without navigating back to configuration
- **Visual Feedback**: Professional loading states and animations
  - **Spinning Animations**: Smooth icon rotations during hash generation (1.5 rotations/second)
  - **Button States**: Proper color changes and disabled states during loading
  - **Consistent Sizing**: Fixed button dimensions to prevent layout shift
- **Advanced Form Validation**: Real-time client-side validation with dynamic constraints
- **📋 Copy to Clipboard**: One-click copying with visual confirmation and feedback
- **Parameter Validation**: Comprehensive client-side validation for all parameters

### 🛠️ Development Features

- **🔍 Comprehensive Linting System**: Enterprise-grade code quality tools
  - **Modern ESLint v9**: Latest flat config with TypeScript and Svelte 5 support
  - **Prettier Integration**: Automatic code formatting with Svelte plugin
  - **Vite Plugin Integration**: Real-time linting during development
  - **Live Feedback**: Warnings and errors show in terminal and browser console
- **TypeScript**: Full type safety throughout the application with strict checking
- **♿ Accessibility**: Comprehensive accessibility support
  - ARIA labels and semantic HTML structure
  - Keyboard navigation support for all interactive elements
  - Screen reader friendly with descriptive content
  - High contrast support and focus management

## 🚀 Technology Stack

### Core Framework

- **SvelteKit 2.x**: Modern web framework configured as Single Page Application (SPA)
- **Svelte 5.0**: Reactive UI framework with latest runes syntax
- **TypeScript 5.0+**: Full type safety with strict configuration
- **Vite 7.x**: Fast build tool and development server with optimized builds

### Styling & Design

- **TailwindCSS 4.0**: Latest utility-first CSS framework with modern features
- **@tailwindcss/vite**: Native Vite integration for optimal performance
- **@tailwindcss/typography**: Enhanced typography support for content
- **PostCSS 8.5+**: Modern CSS processing pipeline

### Development & Quality Tools

- **ESLint 9.34**: Modern flat config with comprehensive rule sets
- **@typescript-eslint/\***: TypeScript-specific linting and parsing
- **eslint-plugin-svelte**: Svelte-specific linting rules and best practices
- **Prettier 3.6+**: Code formatting with Svelte plugin support
- **vite-plugin-eslint**: Real-time linting integration with Vite
- **svelte-check**: TypeScript validation for Svelte components

### Build & Deployment

- **@sveltejs/adapter-static**: SPA adapter for static deployment
- **API Proxy Configuration**: Automatic proxying to HashRand Spin API on port 3000
- **Tailscale Integration**: Built-in remote development access support

## 🛠️ Development

### Prerequisites

- **Node.js 18+**: For package management and build tools
- **HashRand Spin API**: Backend API running on `http://127.0.0.1:3000`

### Quick Start

```bash
# Clone and navigate to web interface
git clone <repository-url>
cd hashrand-spin/web

# Install all dependencies
npm install

# Start development server (port 5173)
npm run dev

# Or with host binding for network access
npm run dev -- --host
```

### Development Commands

```bash
# Development
npm run dev          # Start dev server with hot reload
npm run dev -- --host   # Start with network access

# Building
npm run build        # Build production SPA
npm run preview      # Preview production build locally

# Code Quality (integrated with Vite)
npm run lint         # Run ESLint + Prettier checks via Vite
npm run format       # Auto-format code with Prettier
npm run check        # TypeScript and Svelte validation

# Development Integration
npm run prepare      # Svelte-kit sync for development setup
```

### URLs & Access

- **Local Development**: http://localhost:5173
- **Network Access**: http://your-ip:5173 (with `--host` flag)
- **API Proxy**: Automatic `/api/*` proxying to port 3000
- **Tailscale**: Optional remote access (configured via parent justfile)

## API Integration

The web interface automatically proxies `/api/*` requests to the HashRand Spin backend API running on port 3000. No additional configuration needed for development.

## 📁 Project Structure

```
web/
├── README.md                    # This documentation
├── package.json                 # Dependencies and scripts
├── vite.config.ts              # Vite config with ESLint integration
├── svelte.config.js            # SvelteKit SPA configuration
├── tailwind.config.js          # TailwindCSS 4.0 configuration
├── tsconfig.json               # TypeScript configuration
├── eslint.config.js            # Modern ESLint v9 flat config
├── .prettierrc                 # Prettier formatting rules
├── .prettierignore             # Prettier ignore patterns
├── src/
│   ├── app.html                # HTML template with meta tags
│   ├── app.css                 # Global styles with TailwindCSS 4.0
│   ├── vite-env.d.ts           # Vite and plugin type definitions
│   ├── lib/
│   │   ├── api.ts              # Type-safe API service layer
│   │   ├── components/         # Reusable UI components
│   │   │   ├── BackButton.svelte         # Navigation component
│   │   │   ├── DateTimeLocalized.svelte  # i18n date/time formatting
│   │   │   ├── Footer.svelte             # App footer with version info
│   │   │   ├── Icon.svelte               # SVG sprite icon component
│   │   │   ├── Iconize.svelte            # Universal RTL-aware wrapper
│   │   │   ├── LanguageSelector.svelte   # Language dropdown with flags
│   │   │   ├── LoadingSpinner.svelte     # Loading animation
│   │   │   ├── ThemeToggle.svelte        # Dark/light mode toggle
│   │   │   └── TopControls.svelte        # Unified theme/language controls
│   │   ├── stores/             # Svelte stores for state management
│   │   │   ├── i18n.ts                   # Internationalization system
│   │   │   ├── navigation.ts             # Route and navigation state
│   │   │   ├── result.ts                 # Generation results state
│   │   │   ├── rtl.ts                    # RTL/LTR text direction
│   │   │   ├── theme.ts                  # Theme management
│   │   │   └── translations/             # Language files (13 languages)
│   │   │       ├── en.ts, es.ts, pt.ts, fr.ts, de.ts
│   │   │       ├── ru.ts, zh.ts, ar.ts, hi.ts, ja.ts
│   │   │       └── eu.ts, ca.ts, gl.ts
│   │   └── types/              # TypeScript type definitions
│   │       └── index.ts                  # API types and interfaces
│   └── routes/                 # SvelteKit file-based routing
│       ├── +layout.svelte      # Root layout with navigation
│       ├── +layout.ts          # SPA configuration
│       ├── +page.svelte        # Main menu page
│       ├── custom/             # Custom hash generator
│       │   └── +page.svelte
│       ├── password/           # Password generator
│       │   └── +page.svelte
│       ├── api-key/            # API key generator
│       │   └── +page.svelte
│       └── result/             # Shared result display
│           └── +page.svelte
├── static/                     # Static assets
│   ├── favicon.png             # Browser favicon
│   ├── icons-sprite.svg        # SVG icon sprite (21+ icons)
│   └── robots.txt              # Search engine instructions
└── dist/                       # Production SPA build output
```

## 🌟 Key Features Detail

### Navigation & User Flow

- **Clean Menu Interface**: Card-based design with emoji icons and clear descriptions
- **Streamlined Navigation**: Consolidated controls with unified theme/language selector
- **Return to Menu**: Accessible from any page with consistent back button
- **In-Place Regeneration**: Generate new results without navigating back to configuration
- **Smart Routing**: File-based routing with proper SPA configuration

### Form Validation & Interaction

- **Real-time Validation**: Dynamic parameter checking with instant feedback
- **Smart Length Calculation**: Automatic minimum length adjustment based on alphabet type
- **Interactive Sliders**: Gradient range sliders with precise value display
- **Context-Aware Help**: Dynamic informational notes based on current selection
- **Error Prevention**: Clear constraints and helpful validation messages

### Result Display & Actions

- **Professional Formatting**: Clean result presentation with metadata
- **One-Click Copy**: Clipboard integration with visual confirmation
- **Generation Details**: Parameter summary, timestamp, and generation metadata
- **Action Buttons**: Regenerate, settings, and navigation with consistent styling
- **Loading States**: Smooth animations and disabled states during processing

### Internationalization & RTL

- **13 Complete Languages**: Full translations for major world languages
- **RTL Support**: Automatic right-to-left layout for Arabic
- **Smart Direction**: Browser-native RTL with automatic content reordering
- **Cultural Localization**: Proper date/time formatting per language
- **Flag Integration**: Authentic flag icons for visual language identification

### Theme & Design System

- **Smart Theme Detection**: Automatic system preference detection
- **Manual Override**: User preference saved with localStorage persistence
- **Smooth Transitions**: Professional theme switching with visual feedback
- **Design Consistency**: Unified color scheme and component styling
- **Mobile Optimization**: Touch-friendly controls with proper spacing

## Deployment

The application builds as a static SPA that can be deployed to any static hosting service:

```bash
npm run build
# Deploy the 'build' directory to your hosting platform
```

## Configuration

### API Endpoint

The API endpoint is configured in `vite.config.ts`. For production, update the proxy target or configure your reverse proxy to route `/api/*` to your HashRand Spin API.

### Styling

TailwindCSS configuration can be customized in the generated config files. The application uses a professional blue/gray color scheme with automatic dark mode support.

## License

MIT
