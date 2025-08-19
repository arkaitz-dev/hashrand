# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-08-18

### Added
- Initial implementation of HashRand Spin API
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
- Comprehensive alphabet system with 4 character sets:
  - Base58: 58 characters (Bitcoin standard, excludes confusing characters)
  - No-look-alike: 49 characters (maximum readability)
  - Full: 62 characters (complete alphanumeric)
  - Full-with-symbols: 73 characters (maximum entropy)
- Cryptographically secure random generation using nanoid
- Complete parameter validation and error handling
- Modular architecture with clean separation of concerns
- Comprehensive test suite with 43 automated test cases
- Project restructured into workspace with api/ directory
- Support for Rust 2024 edition
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

### Technical Details
- Built with Fermyon Spin WebAssembly framework
- Uses spin-sdk 3.1.0 for HTTP component functionality
- Implements cdylib crate type for WASM compatibility
- Targets wasm32-wasip1 WebAssembly platform
- Workspace structure for better code organization

### Dependencies
- `spin-sdk = "3.1.0"` - Core Spin framework
- `nanoid = "0.4.0"` - Secure random ID generation
- `serde = "1.0.219"` - Serialization framework
- `serde_json = "1.0.142"` - JSON serialization
- `anyhow = "1"` - Error handling

### Testing
- 43 comprehensive test cases covering all endpoints
- Parameter validation testing
- Edge case and error condition testing
- Alphabet-specific character validation
- Performance and consistency testing
- 100% test success rate achieved

### Documentation
- Complete README.md with API documentation
- Detailed endpoint descriptions and examples
- Project structure documentation
- Setup and deployment instructions
- CLAUDE.md for development guidance

## [0.2.0] - 2025-08-19

### Added
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

### Technical Implementation
- **Single Page Application (SPA)**: Built with `@sveltejs/adapter-static`
- **API Service Layer**: Type-safe API integration with error handling
- **State Management**: Svelte stores for navigation, results, and i18n
- **Component Architecture**: Reusable components (BackButton, LoadingSpinner)
- **Routing System**: File-based routing with menu → forms → result flow
- **Build System**: Optimized production builds with code splitting

### Web Interface Structure
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
│       ├── generate/          # Hash generator
│       ├── password/          # Password generator
│       ├── api-key/           # API key generator
│       └── result/            # Shared result display
```

### Updated Documentation
- **README.md**: Added web interface sections and full development setup
- **CLAUDE.md**: Updated architecture to include web interface
- **Web README.md**: Complete documentation for web interface development

## [0.3.0] - 2025-08-19

### Added
- **🎨 Enhanced Web Interface**: Major UI/UX improvements for professional user experience
  - **Interactive Range Sliders**: Replaced number inputs with attractive gradient sliders for length parameters
  - **Dynamic Informational Notes**: Context-aware help text that changes based on alphabet selection
  - **Automatic Length Adjustment**: Intelligent minimum length calculation when switching alphabets
  - **Spinning Loading Animation**: Smooth 1.5 rotations/second icon animation during hash regeneration
  - **In-Place Regeneration**: Generate new hashes without navigating back to configuration
  - **Visual Loading States**: Button color changes and disabled states during processing

### Changed  
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

### Improved
- **User Experience**: Comprehensive UX enhancements based on reference project patterns
  - Professional gradient styling on range sliders
  - Real-time parameter validation with dynamic feedback
  - Contextual help messages for security and format recommendations
  - Smooth CSS animations and transitions
- **Accessibility**: Enhanced loading state communication through visual animations
- **Performance**: Removed artificial delays used for testing loading states

### Technical Implementation
- **CSS Animations**: Custom `animate-spin-fast` keyframe animation for button icons
- **Dynamic Classes**: Improved Tailwind class application for button states
- **Component Updates**: Enhanced all configuration views (custom, password, api-key)
- **State Management**: Better loading state handling across components
- **Route Mapping**: Updated internal routing with backward compatibility
- **Unified Styling**: Standardized button and slider styles across all generator views
  - Consistent color scheme: solid colors instead of complex gradients
  - Uniform hover states and transitions (duration-200, hover:shadow-lg)
  - Simplified visual effects while maintaining color identity per endpoint

### Documentation
- Updated route references from `/generate` to `/custom` throughout codebase
- Enhanced form interaction patterns following modern web standards
- Improved button state management documentation

## [0.4.0] - 2025-08-19

### Added
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

### Enhanced
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

### Technical Implementation
- **Component Architecture**: New theme-related components and stores
  - `/src/lib/components/ThemeToggle.svelte` - Theme toggle button component
  - `/src/lib/stores/theme.ts` - Theme state management store
  - Updated `+layout.svelte` with theme integration
- **CSS Architecture**: Modern TailwindCSS 4.0 implementation
  - Custom variant configuration for dark mode
  - Smooth transition utilities
  - Class-based theme switching (not media query)
- **State Management**: Sophisticated theme preference handling
  - System preference detection as default
  - localStorage persistence for manual choices
  - Automatic theme application on store subscription
- **Browser Integration**: Enhanced mobile and desktop experience
  - Dynamic meta theme-color updates for mobile browsers
  - Proper CSS transition handling
  - Cross-browser compatibility

### Changed
- **Theme System**: Upgraded from simple system preference to intelligent manual control
  - Previous: Only `prefers-color-scheme` media query support
  - Current: Manual toggle with system preference fallback
- **Layout Structure**: Enhanced main layout with theme toggle integration
  - Added `relative` positioning to main container
  - Integrated theme toggle component
  - Improved z-index management

### Technical Dependencies
- No new external dependencies added
- Leverages existing SvelteKit, TypeScript, and TailwindCSS 4.0 infrastructure
- Uses browser APIs: `localStorage`, `matchMedia`, `classList`

### Documentation Updates
- Updated README.md with Smart Theme System features
- Enhanced project structure documentation with new components
- Added technical implementation details

## [0.5.0] - 2025-08-19

### Added
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

### Enhanced
- **⚡ Performance Optimization**: Significant improvements to loading and rendering
  - **Reduced Bundle Size**: Eliminated inline SVG from JavaScript/CSS bundles
  - **Single HTTP Request**: All icons downloaded in one cached file
  - **No Preload Warnings**: Removed problematic link preload, using on-demand loading
  - **Memory Efficiency**: Shared SVG symbols reduce DOM memory usage
- **🔧 Developer Experience**: Improved maintainability and consistency
  - **Centralized Icon Management**: Easy to add, modify, or remove icons
  - **Component Consistency**: Uniform icon sizing and styling across app
  - **Type Safety**: TypeScript support for icon names and properties

### Changed
- **Icon Implementation**: Migrated from inline SVG to sprite-based system
  - **ThemeToggle.svelte**: Uses `Icon` component for sun/moon icons
  - **BackButton.svelte**: Uses `Icon` component for left arrow
  - **LoadingSpinner.svelte**: Uses `Icon` component for spinner
  - **Main menu**: Uses `Icon` component for right arrow navigation
  - **Result page**: Uses `Icon` component for all action buttons and UI elements
- **HTML Structure**: Added sprite reference system to app template
  - Removed link preload that caused browser warnings
  - External sprite references for optimal loading

### Technical Implementation
- **Component Architecture**: New icon management system
  - `/src/lib/components/Icon.svelte` - Reusable icon component
  - `/static/icons-sprite.svg` - Centralized sprite file with 10 symbols
  - Updated all components to use new `Icon` component
- **Loading Strategy**: Optimized sprite loading approach
  - External references (`href="/icons-sprite.svg#icon-{name}"`)
  - Browser-managed caching without manual preload
  - No console warnings or performance issues
- **Export System**: Icon component available through `$lib` imports
  - Added to `/src/lib/index.ts` for convenient importing
  - Consistent import pattern across codebase

### Performance Benefits
- **Network**: Single cached request replaces 10+ inline SVG elements
- **Memory**: Shared SVG symbols reduce DOM node count
- **Bundle**: JavaScript/CSS sizes reduced by removing inline SVG
- **Cache**: Long-lived sprite cache improves repeat visit performance

## [0.6.0] - 2025-08-19

### Added
- **🏴 Flag Icon Collection**: Complete set of country and region flag icons for future internationalization
  - **11 Flag Icons**: Comprehensive collection of carefully designed SVG flag representations
    - **National Flags**: Spain, UK, France, Germany, Portugal, Russia, Saudi Arabia, China
    - **Regional Flags**: Catalonia, Basque Country (Ikurriña), Galicia
  - **Authentic Colors**: All flags use official color specifications from Wikimedia Commons
  - **Optimized SVG**: Simplified designs optimized for small icon sizes while maintaining recognizability
  - **Consistent Integration**: All flags integrated into existing sprite system for optimal performance
  - **Scalable Design**: Vector graphics ensure crisp rendering at any size

### Enhanced
- **🎨 Icon Sprite System**: Expanded sprite collection from 10 to 21 total icons
  - **Performance Maintained**: Single HTTP request for all icons including new flags
  - **Memory Efficient**: Shared SVG symbols for all flag representations
  - **Developer Ready**: Easy access via `<Icon name="spain" />`, `<Icon name="uk" />`, etc.
- **🌍 Internationalization Preparation**: Foundation laid for multi-language support
  - Flag icons ready for language selection UI
  - Cultural representation for major markets
  - Regional support for Spain (Catalonia, Basque, Galicia)

### Technical Implementation
- **Flag Design Optimization**: Complex flags simplified for icon usage
  - **UK Flag**: Full Union Jack design with proper cross positioning and clipping
  - **China Flag**: Large star with simplified dots for small stars
  - **Catalonia**: Traditional four red stripes on yellow background (La Senyera)
  - **Basque Country**: Complete Ikurriña with red field, green saltire, and white cross
  - **Galicia**: Corrected design with white field and blue diagonal stripe
  - **Simplified Flags**: Saudi Arabia (green field), Portugal (vertical stripes)
- **SVG Structure**: Proper viewBox ratios maintaining flag proportions
  - Standard flags: 3:2 ratio (viewBox="0 0 6 4")
  - Special cases: Custom ratios for accurate representation
  - Color fidelity: Hex codes from official specifications

### Usage Examples
```svelte
<!-- European flags -->
<Icon name="spain" size="w-6 h-6" />
<Icon name="france" size="w-6 h-6" />
<Icon name="germany" size="w-6 h-6" />
<Icon name="uk" size="w-6 h-6" />
<Icon name="portugal" size="w-6 h-6" />

<!-- Other regions -->
<Icon name="china" size="w-6 h-6" />
<Icon name="russia" size="w-6 h-6" />
<Icon name="saudi" size="w-6 h-6" />

<!-- Spanish regions -->
<Icon name="catalonia" size="w-6 h-6" />
<Icon name="basque" size="w-6 h-6" />
<Icon name="galicia" size="w-6 h-6" />
```

### Ready for Future Features
- **Language Selection**: Flag icons prepared for language picker UI
- **Regional Localization**: Support for regional variants within countries
- **Cultural Adaptation**: Visual elements ready for international markets
- **Accessibility**: All flags include proper naming for screen readers

## [Unreleased]

### Planned Features
- Multi-language internationalization using flag icons
- Language selection interface with flag representations
- Regional content adaptation
- Performance benchmarking
- Additional alphabet types
- Batch generation endpoints
- Configuration file support
- Metrics and monitoring
- Docker containerization
- Helm charts for Kubernetes deployment

---

## Version History Summary

- **0.6.0** (2025-08-19) - Flag icon collection for internationalization (11 country/region flags)
- **0.5.0** (2025-08-19) - SVG icon sprite system for optimized performance and maintainability
- **0.4.0** (2025-08-19) - Smart theme toggle system with TailwindCSS 4.0 dark mode implementation
- **0.3.0** (2025-08-19) - Enhanced UI/UX with interactive components and improved user experience
- **0.2.0** (2025-08-19) - Web interface release with professional SPA
- **0.1.0** (2025-08-18) - Initial release with complete API implementation