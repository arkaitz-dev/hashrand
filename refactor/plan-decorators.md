# Refactor Plan - Lit Decorators Migration
**Date:** 2025-08-11
**Goal:** Convert Lit components from standard syntax to decorator syntax

## Current State Analysis

### Architecture
- **Framework:** Lit 3.3.1 with standard JavaScript (no TypeScript)
- **Build Tool:** Vite 7.1.1 (no Babel/TypeScript configured)
- **Components:** 5 Lit Web Components using standard property syntax
- **Pattern:** `static properties = { ... }` instead of decorators

### Components to Refactor
1. `hash-generator.js` - Main container component
2. `generic-hash-view.js` - Generic hash configuration
3. `password-view.js` - Password generation view
4. `api-key-view.js` - API key generation view  
5. `hash-result.js` - Result display component

## Configuration Strategy

### Option 1: Babel with Experimental Decorators (Recommended)
**Pros:**
- Works with JavaScript (no TypeScript needed)
- Mature, stable solution
- Better browser compatibility via transpilation

**Cons:**
- Requires Babel dependencies
- Slightly larger build setup

### Option 2: TypeScript Migration
**Pros:**
- Better type safety
- Native decorator support
- Modern development experience

**Cons:**
- Requires full TypeScript migration
- More complex refactoring
- Learning curve if team not familiar

**Decision:** Use **Option 1 (Babel)** to maintain JavaScript and minimize changes

## Implementation Tasks

### Phase 1: Build Configuration
1. ✅ Install Babel dependencies:
   - `@babel/core`
   - `@babel/plugin-proposal-decorators`  
   - `@babel/plugin-proposal-class-properties`
   - `vite-plugin-babel`

2. ✅ Configure Vite with Babel plugin

3. ✅ Create Babel configuration for decorators

### Phase 2: Component Refactoring
Transform each component from:
```javascript
static properties = {
  value: { type: String, state: true }
};
```

To decorator syntax:
```javascript
@property({ type: String })
value = '';

@state()
privateState = '';
```

### Phase 3: Testing & Validation
1. ✅ Verify development server works
2. ✅ Test production build
3. ✅ Validate all functionality preserved
4. ✅ Check bundle size impact

## Dependencies to Add
```json
{
  "devDependencies": {
    "@babel/core": "^7.24.0",
    "@babel/plugin-proposal-decorators": "^7.24.0",
    "@babel/plugin-proposal-class-properties": "^7.18.6",
    "vite-plugin-babel": "^1.2.0"
  }
}
```

## Vite Configuration Updates
```javascript
import babel from 'vite-plugin-babel';

plugins: [
  babel({
    babelConfig: {
      babelrc: false,
      configFile: false,
      plugins: [
        ["@babel/plugin-proposal-decorators", { version: "2023-05" }],
        "@babel/plugin-proposal-class-properties"
      ]
    }
  }),
  // ... existing plugins
]
```

## Component Migration Map

| Component | Properties | State | Events | Complexity |
|-----------|-----------|-------|--------|------------|
| hash-generator | 0 | 4 | 5 | High |
| generic-hash-view | 0 | 4 | 2 | Medium |
| password-view | 0 | 1 | 2 | Low |
| api-key-view | 0 | 1 | 2 | Low |
| hash-result | 4 | 0 | 2 | Medium |

## Risk Assessment
- **Low Risk:** Simple property/state conversions
- **Medium Risk:** Build configuration changes
- **Mitigation:** Keep backup of working configuration

## Rollback Strategy
1. Git stash/commit current state
2. Revert package.json if issues
3. Restore original component syntax
4. Remove Babel configuration

## Success Criteria
- ✅ All components use decorator syntax
- ✅ Development server runs without errors
- ✅ Production build successful
- ✅ All functionality preserved
- ✅ No performance degradation
- ✅ Clean, readable code

## Next Steps After Migration
- Consider TypeScript migration in future
- Add component unit tests
- Document decorator patterns for team