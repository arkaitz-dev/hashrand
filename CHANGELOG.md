# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Component Versions:**
- **API**: Stable backend (starts from 1.0.0)  
- **Web**: User interface (evolving, 0.x.x series)

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
- **Rapid Iteration**: Minor versions (0.1.0, 0.2.0) for UI/UX improvements and new features
- **Breaking UI Changes**: Major versions in 0.x.x series (0.1.0 → 0.2.0) for significant UI restructures
- **Stability Target**: Will reach 1.0.0 when feature-complete and UI/UX is finalized

### Release Tags
- **API releases**: `api-v1.0.0`, `api-v1.1.0`, etc.
- **Web releases**: `web-v0.7.0`, `web-v0.8.0`, etc.
- **Combined releases**: When both components are updated simultaneously

### Version Endpoint
- **GET /api/version**: Returns both component versions
  ```json
  {
    "api_version": "1.0.0",
    "ui_version": "0.8.0"
  }
  ```