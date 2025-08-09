# Refactor Plan - 2025-08-09 16:00 (New Session)

## Current Problem Analysis
- **Issue 1**: "Back to Menu" button functionality unclear (though code seems correct)
- **Issue 2**: Result display area duplicated in all configuration views
- **Issue 3**: Navigation flow mixes configuration and results in same view
- **User Request**: Separate configuration from results with improved navigation

## Refactoring Objectives
1. **Create dedicated result view**: Separate from configuration views
2. **Extract common result component**: Eliminate duplication
3. **Improve navigation flow**: Clear 3-button navigation in result view
   - Back to configuration (modify parameters)
   - Back to main menu (start over)
   - Regenerate (same parameters)

## Proposed Architecture

### Component Structure
```
hash-generator.js (Main container)
├── Menu View (existing)
├── Configuration Views (refactored - no results)
│   ├── generic-hash-view.js (config only)
│   ├── password-view.js (config only)  
│   └── api-key-view.js (config only)
└── Result View (new)
    └── hash-result.js (common result display component)
```

### Navigation Flow
1. **Menu** → Select generation type
2. **Configuration View** → Set parameters → Generate button
3. **Result View** → Shows result with:
   - Top: "Back to Config" | "Back to Menu" buttons
   - Middle: Generated result with copy button
   - Bottom: "Regenerate" button (same parameters)

## Refactoring Tasks

### Priority 1: Create Result Component
- [ ] Create `hash-result.js` component for result display
- [ ] Add properties for hash type, parameters, and result
- [ ] Implement 3-button navigation system
- [ ] Add regeneration functionality

### Priority 2: Update Main Container
- [ ] Add result view to `hash-generator.js`
- [ ] Implement navigation to result view
- [ ] Add state management for generation parameters
- [ ] Handle navigation events from result component

### Priority 3: Refactor Configuration Views
- [ ] Remove result section from `generic-hash-view.js`
- [ ] Remove result section from `password-view.js`
- [ ] Remove result section from `api-key-view.js`
- [ ] Update generate buttons to pass parameters and navigate

### Priority 4: State Management
- [ ] Store generation parameters when generating
- [ ] Pass parameters to result view
- [ ] Enable regeneration with same parameters
- [ ] Preserve configuration when navigating back

## Implementation Details

### Result Component Properties
```javascript
static properties = {
    hashType: { type: String },      // 'generic', 'password', 'apiKey'
    generatedHash: { type: String },  // The result
    parameters: { type: Object },     // Generation parameters
    isLoading: { type: Boolean },     // Loading state
    error: { type: String }           // Error message if any
}
```

### Parameter Structure Example
```javascript
// For generic hash
parameters = {
    length: 21,
    alphabet: 'base58',
    prefix: '',
    suffix: ''
}

// For password
parameters = {
    length: 21
}

// For API key
parameters = {}  // No parameters needed
```

### Event Communication
- `generate-hash` event: Config view → Main container (with parameters)
- `show-result` event: Main container → Result view (with hash and params)
- `regenerate` event: Result view → Main container (trigger new generation)
- `back-to-config` event: Result view → Main container
- `back-to-menu` event: Result view → Main container

## Validation Checklist
- [ ] All generation types work correctly
- [ ] Navigation flows smoothly between views
- [ ] Regenerate maintains exact same parameters
- [ ] Copy to clipboard works in result view
- [ ] Loading states display properly
- [ ] Error handling works correctly
- [ ] No duplicate code remains
- [ ] Build successful (npm run build)
- [ ] Development server works (npm run dev)

## Risk Assessment
- **Low Risk**: Component extraction with clear interfaces
- **Medium Risk**: State management between views
- **Mitigation**: Test each component individually before integration

## De-Para Mapping
| Component | Before | After | Status |
|-----------|--------|-------|--------|
| generic-hash-view | Has result section | Config only | Pending |
| password-view | Has result section | Config only | Pending |
| api-key-view | Has result section | Config only | Pending |
| hash-result | Not exists | Common result display | Pending |
| hash-generator | 3 views | 4 views (+ result) | Pending |

## Notes
- Preserve all CSS styles and animations
- Maintain API compatibility
- Use Lit's property system for state management
- Keep event-based communication pattern
- Ensure smooth transitions between views