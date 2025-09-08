# Component Architecture

The HashRand web interface is built with a modular component architecture using **SvelteKit**, providing reusable, accessible, and maintainable components.

## Core Component Library

### 🎨 UI Components

#### BackButton.svelte
- **Purpose**: Consistent navigation back functionality
- **Features**: 
  - Keyboard navigation support
  - Customizable styling
  - Accessible ARIA labels
- **Usage**: Used across all generator pages for navigation

#### LoadingSpinner.svelte
- **Purpose**: Loading animations during API calls
- **Features**:
  - Smooth spinning animations
  - Customizable size and colors
  - CSS-based animations for performance
- **Usage**: Displayed during hash generation and authentication

#### Icon.svelte
- **Purpose**: SVG icon sprite component
- **Features**:
  - Progressive SVG sprite loading
  - UTF emoji placeholders for instant feedback
  - 189KB professional sprite with zero quality compromise
  - Smart loading states with smooth transitions
  - User silhouette icon (👤) with filled design matching theme system
- **Usage**: Icons throughout the interface, including consistent session management

#### Iconize.svelte
- **Purpose**: Universal RTL-aware wrapper for any content
- **Features**:
  - Automatic RTL detection and handling
  - Smart icon positioning for right-to-left languages
  - Zero-config RTL support
  - Works with any content type
- **Usage**: Wrapper for buttons and content in RTL languages

### 🌙 Theme System

#### ThemeToggle.svelte
- **Purpose**: Dark/light mode toggle control
- **Features**:
  - Manual theme toggle in upper-right corner
  - System preference detection on first visit
  - Persistent user choice in localStorage
  - Smooth transitions and visual feedback
  - Accessible ARIA labels
- **State Management**: Integrated with theme store

### 🔐 Authentication Components

#### AuthStatusButton.svelte
- **Purpose**: Session management button with consistent user icon
- **Features**:
  - Always-visible authentication button regardless of session state
  - Consistent filled user silhouette icon (👤) for all states
  - Dual functionality: login trigger (unauthenticated) and user menu (authenticated)
  - Enhanced icon sizes (`w-5 h-5 sm:w-6 sm:h-6`) for better visibility
  - Integrated with authentication store for state management
- **Design**: Solid user icon with `fill="currentColor"` matching theme system

#### DialogContainer.svelte
- **Purpose**: Unified modal dialog system
- **Features**:
  - Professional modal dialogs for all authentication flows
  - Keyboard navigation (`Escape` key support)
  - Accessible ARIA attributes (`role="dialog"`, `aria-modal="true"`)
  - Backdrop click handling
  - Focus management
- **Accessibility**: Full keyboard navigation and screen reader support

#### AuthDialogContent.svelte
- **Purpose**: Authentication dialog content
- **Features**:
  - Two-step authentication process
  - Email input with validation
  - Magic link request handling
  - Integration with authentication store
  - Error handling and feedback
- **Validation**: Real-time email validation with feedback

#### AuthConfirmDialogContent.svelte
- **Purpose**: Email confirmation dialog
- **Features**:
  - Confirmation step after email submission
  - Visual feedback for magic link sent
  - Timer display for link expiration
  - Resend functionality
  - Mobile-optimized debug display (20 seconds for tablet development)

### 🌐 Internationalization Components

#### DateTimeLocalized.svelte
- **Purpose**: Robust date/time localization
- **Features**:
  - Multi-level fallback system
  - Browser compatibility detection
  - Custom fallbacks for specific languages (Galician abbreviations)
  - Cross-platform reliability
  - Graceful degradation on all browser engines
- **Fallback Hierarchy**: Native Intl → Custom formatting → Manual fallback → English

### 📱 Responsive Layout Components

#### Layout Components
- **+layout.svelte**: Root layout with navigation and theme system
- **Navigation**: Responsive navigation for all screen sizes
- **Header**: Consistent header across all pages
- **Footer**: Minimal footer with essential links

## State Management Architecture

### 🏪 Svelte Stores

#### Authentication Store (`auth.ts`)
```typescript
interface AuthState {
  isAuthenticated: boolean;
  accessToken: string | null;
  userId: string | null;
  isLoading: boolean;
  error: string | null;
}
```
- **Features**: JWT token management, automatic refresh, session persistence
- **Methods**: `login()`, `logout()`, `refreshToken()`, `checkAuth()`

#### Theme Store (`theme.ts`)
```typescript
interface ThemeState {
  current: 'light' | 'dark' | 'system';
  effective: 'light' | 'dark';
}
```
- **Features**: System preference detection, persistent storage, reactive updates
- **Methods**: `toggleTheme()`, `setTheme()`, `detectSystemTheme()`

#### Internationalization Store (`i18n.ts`)
```typescript
interface I18nState {
  currentLanguage: string;
  translations: Record<string, any>;
  isRTL: boolean;
  textDirection: 'ltr' | 'rtl';
}
```
- **Features**: Dynamic language loading, RTL detection, persistent language selection
- **Methods**: `setLanguage()`, `translate()`, `detectRTL()`

#### Navigation Store (`navigation.ts`)
```typescript
interface NavigationState {
  currentRoute: string;
  previousRoute: string;
  breadcrumbs: string[];
}
```
- **Features**: Route tracking, breadcrumb generation, navigation history
- **Methods**: `navigate()`, `goBack()`, `updateBreadcrumbs()`

#### Result Store (`result.ts`)
```typescript
interface ResultState {
  lastResult: GenerationResult | null;
  isGenerating: boolean;
  error: string | null;
  parameters: GenerationParameters;
}
```
- **Features**: Result caching, parameter preservation, error handling
- **Methods**: `generateHash()`, `clearResult()`, `updateParameters()`

## Advanced Component Features

### 📋 Form Handling

#### Parameter Preservation System
- **Base58 URL Encoding**: Form parameters encoded as base58 URL-safe strings
- **LocalStorage Integration**: Temporary storage with automatic cleanup
- **State Restoration**: Seamless form state restoration after authentication
- **Universal Integration**: Works across all generator pages

#### Real-time Validation
- **Parameter Validation**: Immediate feedback on parameter changes
- **Visual Feedback**: Color-coded validation states
- **Context-aware Help**: Dynamic help text based on current settings
- **Error Prevention**: Prevents invalid parameter combinations

### 🎛️ Interactive Controls

#### Range Sliders
- **Beautiful Styling**: Gradient styling with custom CSS
- **Touch Optimization**: Mobile-friendly touch interaction
- **Real-time Updates**: Immediate parameter updates
- **Accessibility**: Keyboard navigation support
- **Visual Feedback**: Clear indication of current values

#### Clipboard Integration
- **One-click Copying**: Copy results with visual feedback
- **Success Indicators**: Toast notifications for copy success
- **Error Handling**: Graceful fallback when clipboard unavailable
- **Keyboard Support**: Copy with keyboard shortcuts (Ctrl+C)

### 🔄 State Synchronization

#### URL Parameter Sync
- **Automatic Synchronization**: Parameters synchronized with browser URL
- **Shareable URLs**: Complete configuration shareable via URL
- **Browser History**: Proper browser back/forward support
- **Deep Linking**: Direct access to configured generator states

#### Form State Management
- **Persistent State**: Form state preserved across navigation
- **Validation State**: Validation results maintained
- **Parameter Binding**: Two-way data binding for all parameters
- **Reset Functionality**: Clear form state and return to defaults

## Component Testing

### 🧪 Testing Architecture
- **Unit Tests**: Individual component testing with Vitest
- **Integration Tests**: Component interaction testing
- **Accessibility Tests**: ARIA compliance and keyboard navigation
- **Visual Regression**: Screenshot testing for UI consistency
- **Cross-browser Testing**: Compatibility across different browsers

### Testing Examples
```typescript
// Component testing example
import { render, fireEvent } from '@testing-library/svelte';
import ThemeToggle from './ThemeToggle.svelte';

test('theme toggle switches between light and dark', async () => {
  const { getByRole } = render(ThemeToggle);
  const toggle = getByRole('button');
  
  await fireEvent.click(toggle);
  expect(document.documentElement).toHaveClass('dark');
  
  await fireEvent.click(toggle);
  expect(document.documentElement).toHaveClass('light');
});
```

## Performance Optimization

### ⚡ Component Performance
- **Lazy Loading**: Non-critical components loaded on demand
- **Code Splitting**: Components split by route for optimal loading
- **Tree Shaking**: Unused component code eliminated in build
- **Bundle Analysis**: Regular analysis of component bundle sizes

### Memory Management
- **Store Cleanup**: Automatic cleanup of unused store subscriptions
- **Event Listeners**: Proper cleanup of event listeners in components
- **DOM References**: Efficient management of DOM element references
- **Resource Disposal**: Cleanup of resources in component lifecycle

---

*For interface features, see [Interface Documentation](./interface.md)*  
*For internationalization, see [Internationalization Documentation](./internationalization.md)*  
*For development setup, see [Development Guide](../deployment/development.md)*