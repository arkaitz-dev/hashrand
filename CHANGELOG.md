# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Component Versions:**
- **API**: Stable backend (starts from 1.0.0)  
- **Web**: User interface (evolving, 0.x.x series)

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