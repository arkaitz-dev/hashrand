# Implementation Plan - Web Interface Menu Navigation

## Source Analysis
- **Source Type**: Feature Enhancement Request
- **Core Features**: 
  - Replace immediate API call with menu-based navigation
  - Three main modes: Generic Hash, Password, API Key
  - Mode-specific forms with appropriate options
- **Dependencies**: None (uses existing Web Components)
- **Complexity**: Medium - Significant UI restructuring

## Current State Analysis
- Single form loads immediately and calls API on first load
- All generation options visible at once (mixed modes)
- Password options toggle dynamically
- Special buttons for API Key and Password modes

## Target Integration
- **Integration Points**: 
  - Modify HTML template in `src/main.rs` (lines 535-1022)
  - Update Web Component JavaScript logic
  - Maintain existing REST API endpoints
- **Affected Files**: 
  - `src/main.rs` (HTML template constant)
- **Pattern Matching**: 
  - Keep Web Components architecture
  - Maintain responsive design patterns
  - Preserve CSS styles and transitions

## Implementation Tasks

### Phase 1: Menu Implementation
- [x] Analyze current implementation structure
- [ ] Create main menu view with three cards (Generic Hash, Password, API Key)
- [ ] Style menu cards with icons and descriptions
- [ ] Implement menu card hover effects and transitions

### Phase 2: View Management
- [ ] Create view switching mechanism in Web Component
- [ ] Add state management for current view
- [ ] Implement "Back to Menu" navigation
- [ ] Add smooth transitions between views

### Phase 3: Generic Hash Form
- [ ] Create dedicated view for Generic Hash
- [ ] Include length slider (2-128)
- [ ] Add alphabet selection dropdown
- [ ] Include prefix/suffix inputs
- [ ] Add generate button

### Phase 4: Password Form
- [ ] Create dedicated view for Password mode
- [ ] Add password length slider (21-44)
- [ ] Display password strength indicators
- [ ] Include copy functionality
- [ ] Add generate button

### Phase 5: API Key Form
- [ ] Create dedicated view for API Key
- [ ] Show fixed format information (ak_ + 44 chars)
- [ ] Display security information
- [ ] Add generate button
- [ ] Include copy functionality

### Phase 6: Polish & Testing
- [ ] Test all navigation flows
- [ ] Verify API calls work correctly
- [ ] Test responsive design on mobile
- [ ] Add loading states for all forms
- [ ] Ensure accessibility features work

## Technical Approach

### View Structure
```javascript
// Views enum
const VIEWS = {
    MENU: 'menu',
    GENERIC: 'generic',
    PASSWORD: 'password',
    API_KEY: 'api_key'
};

// Current view state
let currentView = VIEWS.MENU;
```

### Menu Cards HTML
```html
<div class="menu-grid">
    <div class="menu-card" data-mode="generic">
        <div class="menu-icon">🎲</div>
        <h3>Generic Hash</h3>
        <p>Customizable random strings with various alphabets</p>
    </div>
    <div class="menu-card" data-mode="password">
        <div class="menu-icon">🔐</div>
        <h3>Password</h3>
        <p>Strong passwords with symbols (21-44 chars)</p>
    </div>
    <div class="menu-card" data-mode="api_key">
        <div class="menu-icon">🔑</div>
        <h3>API Key</h3>
        <p>Secure API keys with ak_ prefix (256-bit entropy)</p>
    </div>
</div>
```

### CSS Additions
```css
.menu-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 2rem;
    margin: 2rem 0;
}

.menu-card {
    background: white;
    border-radius: 12px;
    padding: 2rem;
    text-align: center;
    cursor: pointer;
    transition: all 0.3s ease;
    border: 2px solid transparent;
}

.menu-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 12px 24px rgba(0, 0, 0, 0.15);
    border-color: #667eea;
}

.menu-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
}

.view-container {
    display: none;
}

.view-container.active {
    display: block;
    animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
}

.back-button {
    background: transparent;
    border: 2px solid #667eea;
    color: #667eea;
    margin-bottom: 1.5rem;
    width: auto;
    padding: 10px 20px;
}

.back-button:hover {
    background: #667eea;
    color: white;
}
```

## Validation Checklist
- [ ] Menu displays correctly on load (no API calls)
- [ ] All three menu options are clickable
- [ ] Each mode shows appropriate form fields
- [ ] Back navigation works from all views
- [ ] API calls only happen on explicit generate actions
- [ ] Copy functionality works in all modes
- [ ] Responsive design maintained
- [ ] No console errors
- [ ] Smooth transitions between views
- [ ] Loading states show correctly
- [ ] Error handling works properly

## Risk Mitigation
- **Potential Issues**: 
  - Breaking existing functionality
  - CSS conflicts with shadow DOM
  - State management complexity
- **Rollback Strategy**: 
  - Git commit before changes
  - Keep original code commented during development
  - Test incrementally