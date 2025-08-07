# Web Interface Implementation Plan - 2025-08-06

## Source Analysis
- **Request**: Add web interface to hashrand server at route `/`
- **Requirements**: 
  - Show interface when accessing `/` in server mode (-s)
  - Allow calling API with different options preconfigured
  - Option to customize parameters and show resulting hash
  - Use Web Components standard
  - Professional, responsive CSS design
- **Integration**: Add to existing Axum server in `src/main.rs`

## Current Server Analysis
- Server has 3 API endpoints:
  - `/api/generate?length=21&alphabet=base58`
  - `/api/api-key` 
  - `/api/password?length=21`
- Uses Axum router with middleware for CORS, rate limiting, body limits
- Server binding configurable (127.0.0.1 or 0.0.0.0)

## Target Integration
- **Integration Points**: Add route handler for `/` in existing Router
- **Affected Files**: `src/main.rs` (add HTML response handler)
- **Pattern Matching**: Follow existing async handler pattern
- **Dependencies**: No new dependencies needed (use axum's Html response)

## Implementation Tasks

### Phase 1: Basic Web Interface
- [ ] Create HTML template with embedded CSS and JavaScript
- [ ] Add `/` route handler returning HTML response
- [ ] Design responsive layout with form controls
- [ ] Implement Web Components for hash generation interface

### Phase 2: API Integration
- [ ] Add JavaScript to call existing API endpoints
- [ ] Create form controls for all available options:
  - Length slider (2-128)
  - Alphabet selection (base58, no-look-alike, full, full-with-symbols)
  - Prefix/suffix inputs
  - API key generation button
  - Password generation with length
- [ ] Display generated results

### Phase 3: Professional Styling
- [ ] Responsive CSS design (mobile-first)
- [ ] Professional color scheme and typography
- [ ] Interactive elements with hover/focus states
- [ ] Loading states for API calls
- [ ] Error handling and user feedback

### Phase 4: Web Components Implementation
- [ ] Create custom elements following Web Components standard
- [ ] Encapsulated styling with Shadow DOM
- [ ] Reusable components for different hash types
- [ ] Clean separation of concerns

## Technical Approach

### HTML Structure
```html
<!DOCTYPE html>
<html>
<head>
  <title>HashRand Web Interface</title>
  <style>/* Professional responsive CSS */</style>
</head>
<body>
  <hash-generator></hash-generator>
  <script>/* Web Components implementation */</script>
</body>
</html>
```

### Web Component Structure
- `<hash-generator>` - Main interface component
- `<hash-options>` - Configuration panel
- `<hash-result>` - Display generated hash
- `<api-status>` - Show API call status/errors

### Route Handler Pattern
```rust
async fn handle_root() -> axum::response::Html<&'static str> {
    axum::response::Html(HTML_TEMPLATE)
}
```

## Validation Checklist
- [ ] Web interface loads at `/` route
- [ ] All API endpoints accessible from interface
- [ ] Responsive design works on mobile/desktop
- [ ] Web Components follow standard correctly
- [ ] Professional appearance and UX
- [ ] Error handling for API failures
- [ ] No broken functionality in CLI mode
- [ ] Security: no XSS vulnerabilities in HTML

## Risk Mitigation
- **HTML Injection**: Use static HTML template, no user input in HTML
- **CORS Issues**: Already handled by existing CORS middleware
- **Large Binary Size**: Single HTML string, minimal impact
- **Security**: Interface only calls existing secure API endpoints

## Implementation Notes
- Keep HTML as const string in Rust code for single binary distribution
- Use modern CSS features (Grid, Flexbox) for responsive design
- Implement proper error handling for network requests
- Follow accessibility best practices (ARIA labels, semantic HTML)