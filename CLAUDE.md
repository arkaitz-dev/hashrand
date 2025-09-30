# CLAUDE.md

HashRand Spin: Random hash generator con Fermyon Spin + WebAssembly. REST API completa e interfaz web para hashes, contraseñas, API keys y frases mnemónicas BIP39 criptográficamente seguras.

**Arquitectura**: Workspace con API Backend (`/api/` - Rust+Spin, puerto 3000) e Interfaz Web (`/web/` - SvelteKit+TypeScript+TailwindCSS, puerto 5173)

## Security Standards - CRITICAL RULE
**🔐 MANDATORY: Follow the highest security standards for secret management (API keys, passwords, salts, secrets in general):**
- **NEVER hardcode secrets in source code** - Always use environment variables
- **Immediately audit existing code** for hardcoded secrets when joining a project
- **Proactively suggest security improvements** when reviewing any authentication/crypto code
- **Use cryptographically secure random generation** for all secrets (32+ bytes minimum)
- **Implement proper secret rotation capabilities** from day one
- **Document secret management** in project README and deployment guides
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying
- **This is a CRITICAL security responsibility** - Treat any hardcoded secret as a HIGH PRIORITY vulnerability

## Email Testing Standards - CRITICAL RULE - NEVER DELETE
**📧 MANDATORY: For ALL email service testing (Mailtrap, SendGrid, etc.):**
- **ALWAYS send test emails to these addresses ONLY:** `me@arkaitz.dev`, `arkaitzmugica@protonmail.com`
- **NEVER use random or external email addresses for testing** - This prevents spam and respects privacy
- **This rule applies to ALL email services and projects** - No exceptions
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying
- **Add this rule to global ~/.claude/CLAUDE.md** - Must be in all projects
- **This is EXTREMELY IMPORTANT and must NEVER be forgotten or overlooked**

## Comandos Esenciales
```bash
just dev         # PRIMARY: Entorno completo de desarrollo (API + Web + Tailscale)
just stop        # Parar todos los servicios
just status      # Estado de servicios
just test        # Ejecutar 39 bash tests (35 API + 4 key rotation)
just check       # Calidad código (clippy + fmt + ESLint + svelte-check)
just build       # Build API (WASM) + Web (SPA)

# Tests Playwright (browser-less, perfecto para CI/CD)
cd web && npm run test:api          # 16 API tests sin browser
cd web && npm run test:api:verbose  # Output detallado
```

## Arquitectura General
**Backend** (`api/src/`): handlers/, database/ (SQLite Zero Knowledge), utils/ (JWT, auth, ChaCha20)
**Frontend** (`web/src/`): routes/ (SPA), lib/components/ (AuthGuard, dialogs), lib/stores/ (auth, i18n 13 idiomas)
**Auth**: Zero Knowledge magic links + JWT (access 20s dev, refresh 2min dev)

## Endpoints Clave
- `POST /api/{custom,password,api-key,mnemonic}` - Generación (JWT protegido)
- `POST/GET /api/login/` - Auth flow con magic links Zero Knowledge
- `GET/POST/DELETE /api/users` - Gestión usuarios (JWT protegido) ⚠️ **FUTURO**
- `GET /api/version` - Público (sin auth)

## Reglas de Desarrollo

**CRITICAL RULES:**
- **ALWAYS use justfile**: Check for justfile existence first. If exists, USE IT instead of manual commands. Run `just` to see tasks.
- **Code Changes**: Comment lines before changing (not MD files). Ask for approval. If liked, erase comments; if not, restore original.
- **Be Surgical**: Modify only necessary code. Write meaningful comments. Follow DRY and KISS principles.

## SOLID & DRY Architecture Standards - CRITICAL RULE
**🏗️ MANDATORY: Follow enterprise-grade architecture principles in ALL code creation and modification:**
- **ALWAYS apply SOLID and DRY principles** - Every time code is created or modified, observe possibility of making it more universal and reusable
- **225-line module limit** - If any file exceeds 225 lines, study modularization possibilities
- **Mandatory size check** - Check file sizes after EVERY coding task completion (creation/editing)
- **Modular thinking** - Consider Single Responsibility Principle for every function and class
- **Universal patterns** - Create reusable composables/modules to eliminate code duplication
- **Active code scanning** - Each time you edit or create new code, try to remember if there are other points in the project where similar or identical operations are being performed, to try to apply DRY, SOLID and KISS whenever possible
- **Pattern recognition enforcement** - Each time you edit or create new code, try to remember if there are other points in the project where similar or identical operations are being performed, to try to apply DRY, SOLID and KISS whenever possible
- **Copy this rule to EVERY project CLAUDE.md** - Never delete when compacting/simplifying
- **This applies to ALL programming languages** - TypeScript, Rust, Python, JavaScript, etc.
- **Enterprise standard compliance** - All modules must be easily testable and modifiable in isolation
- **Styling**: DO NOT CHANGE existing CSS/TailwindCSS unless EXPLICITLY requested. New components must follow existing project styling.
- **Versioning**: CHANGELOG changes must be reflected in config files (package.json, Cargo.toml).
- **Spin Projects**: Use `spin-cli` not `cargo`. Only `cargo add/update/fmt/clippy` allowed.

## Git Workflow
**CRITICAL: ALWAYS use `git add .` for ALL commits**
- Git's .gitignore intelligence is superior to manual file selection
- Prevents missing files, saves time, eliminates human error
- NEVER manually select files with `git add file1 file2`
- Standard workflow: `git add . && git commit -m "message" && git push`

## Tech Stack
- **Backend**: Rust 2024 + Fermyon Spin + WebAssembly + SQLite
- **Frontend**: SvelteKit 2.x + TypeScript + TailwindCSS 4.0 + Vite 7.x
- **Quality**: ESLint 9 + Prettier + Clippy + TypeScript strict

## Context7 MCP Usage Rules
**CRITICAL: ALWAYS follow this Context7 usage pattern - NEVER delete this section**

**🎯 Simple Mandatory Rule:**
- **Need external updated documentation?** → **Use Context7**
- **Internal code/logic work?** → **No Context7 needed**

**✅ USE Context7 when:**
- Implementing with external libraries/frameworks ("use context7 for FastAPI auth")
- Updating dependencies (breaking changes, new APIs)
- Integrating external services (Stripe, Auth0, AWS, etc.)
- Debugging framework-specific issues (Spin, SvelteKit, etc.)
- Need updated best practices (security, performance)
- Working with fast-evolving technologies (WASM, Rust, JS frameworks)

**❌ DON'T use Context7 for:**
- Internal code refactoring
- Business logic specific to your project
- Basic git operations
- Pure styling (CSS/TailwindCSS)
- Debugging your custom code
- Project management tasks

**IMPORTANT**: This rule must be copied to ALL project CLAUDE.md files. Never delete when simplifying/compacting CLAUDE.md.

## Última Sesión: Playwright API-Only Tests Implementation (2025-10-01)

### ✅ Implementación Completa: 16 Tests API Playwright (v0.21.6)

**NUEVO**: Suite completa de tests API-only sin dependencias de browser, perfecta para Arch Linux y CI/CD.

#### Archivos Creados

**Tests API (3 archivos, 774 líneas)**:
1. **`web/tests/api/auth-api.spec.ts`** (226 líneas, 4 tests)
   - Magic link request con validación Ed25519
   - Rechazo de requests sin firma (400)
   - Rechazo de firmas inválidas (400)
   - Múltiples requests concurrentes

2. **`web/tests/api/auth-full-flow.spec.ts`** (202 líneas, 2 tests) - **CLAVE**
   - Flujo completo con extracción de magic link de logs backend
   - Múltiples extracciones con validación de unicidad
   - Replica patrón bash: `grep "Generated magic_link" .spin-dev.log`

3. **`web/tests/api/crypto-validation.spec.ts`** (346 líneas, 10 tests)
   - Ed25519: keypair generation, signing/verification, hex conversion (3 tests)
   - SignedRequest: deterministic serialization, identical signatures, query params (3 tests)
   - Base64/JSON: URL-safe encoding, recursive key sorting, deterministic serialization (3 tests)
   - TestSessionManager: in-memory session state (1 test)

**Documentación**:
- `web/tests/README.md` - Documentación completa de test suite

#### Características Clave

- ✅ **Sin browser** - Funciona en Arch Linux, CI/CD minimalista
- ✅ **Magic link extraction** - Lee logs backend (`.spin-dev.log`) matching bash pattern
- ✅ **Validación Ed25519** - Verificación criptográfica completa con @noble/curves
- ✅ **Módulos universales** - Reutiliza código frontend producción (SOLID/DRY/KISS)
- ✅ **Timestamps reales** - `Math.floor(Date.now() / 1000)` con determinismo por test
- ✅ **Emails autorizados** - Solo `me@arkaitz.dev`, `arkaitzmugica@protonmail.com`, `arkaitzmugica@gmail.com`
- ✅ **100% success rate** - Los 16 tests pasan consistentemente

#### Comandos Test

```bash
# Tests API-only (sin browser)
cd web && npm run test:api          # Output estándar
cd web && npm run test:api:verbose  # Logs detallados
cd web && npx playwright test api/  # Comando directo
```

#### Mejoras de Calidad

**Timestamps**: Cambiados de hardcoded (`1234567890`) a reales
- Mantiene determinismo dentro de cada ejecución
- Validación realista entre diferentes runs

**Email Validation**: Todos los tests usan solo emails autorizados
- Previene spam y respeta privacidad
- Consistente con políticas de seguridad producción

#### Documentación Actualizada

- ✅ `README.md` - Test count: **55 automated tests** (35 bash + 16 Playwright API + 4 key rotation)
- ✅ `docs/guides/testing.md` - Nueva sección Playwright API tests con categorías detalladas
- ✅ `docs/E2E_TESTING_IMPLEMENTATION_PLAN.md` - Phase 5 documentando API-only tests
- ✅ `CHANGELOG.md` - Nueva entrada v0.21.6 con implementación completa
- ✅ `web/package.json` - Version: 0.21.4 → 0.21.6

#### Estadísticas

- **Archivos creados**: 3 tests + 1 README = 4 archivos nuevos
- **Líneas totales**: 774 líneas de código test
- **Tests añadidos**: **16 API-only tests**
- **Success rate**: **100%** (todos los tests pasando)
- **Coverage**: Authentication flow, validación criptográfica, extracción magic link

#### Archivos Modificados

**Documentación (5 archivos)**:
- `README.md` - Test count y comandos Playwright
- `docs/guides/testing.md` - Sección completa Playwright API tests
- `docs/E2E_TESTING_IMPLEMENTATION_PLAN.md` - Phase 5 añadida
- `CHANGELOG.md` - Nueva entrada v0.21.6
- `web/package.json` - Version bump

**Frontend (previo - Phase 1-4)**:
- `web/src/lib/ed25519/ed25519-core.ts` - Módulo universal Ed25519
- `web/src/lib/crypto/signedRequest-core.ts` - SignedRequest puro
- `web/playwright.config.ts` - Configuración Playwright
- `web/tests/utils/` - TestSessionManager + auth helpers
- `web/tests/e2e/` - 21 E2E tests (requieren browser)

#### Próximos Pasos Sugeridos

- Integrar tests API en pipeline CI/CD (no requieren browser)
- Considerar E2E tests para validación completa (requieren setup browser)
- Expandir coverage a endpoints protegidos usando magic link extraction

---

## Sesión Anterior: Critical Bug Fix + Test Automation (2025-09-30)

### 🐛 Bug Crítico Corregido: Refresh Token Ed25519 Public Key (v1.6.23)

**Problema**: Refresh tokens contenían `pub_key = [0,0,0,0,...]` (all zeros) en lugar de la Ed25519 public key real del usuario, rompiendo completamente el sistema de key rotation.

**Root Cause**: Cadena de funciones no pasaba correctamente el `pub_key_bytes` desde login hasta creación de refresh token:
1. `magic_link_jwt_generator.rs` → No pasaba pub_key a `create_refresh_token()`
2. `jwt/tokens.rs` → Parámetro `session_id` en lugar de `pub_key`
3. `custom_token_api.rs` → Fallback a `[0u8; 32]` cuando recibía None

**Solución Implementada**:
- **5 archivos modificados** en cadena de JWT token creation:
  - `api/src/utils/jwt/tokens.rs` - Cambio de firma de función
  - `api/src/utils/jwt/utils.rs` - Public API wrapper actualizado
  - `api/src/utils/auth/magic_link_jwt_generator.rs` - Pasa pub_key a refresh token
  - `api/src/utils/jwt_middleware_renewal.rs` - Usa pub_key de refresh token claims
  - `api/src/utils/jwt_middleware_auth.rs` - Usa pub_key de refresh token claims

**Impacto**: Key rotation system ahora 100% funcional. Backend puede validar Ed25519 signatures durante `/api/refresh`.

### ✅ Test Automation: Sistema 2/3 Key Rotation

**Nuevo Test Automatizado**: `scripts/test_2_3_system.sh` (382 líneas)
- **4 tests completos** del ciclo de vida 2/3 system
- **100% success rate** después del bug fix
- **Duración**: ~7 minutos (incluye esperas de expiración)

**Tests Implementados**:
1. **Test 1 (t=0s)**: Token válido → Hash generado exitosamente
2. **Test 2 (t=62s)**: Refresh parcial (TRAMO 1/3) → Solo access token renovado
3. **Test 3 (t=110s)**: KEY ROTATION (TRAMO 2/3) → Rotación completa con nueva keypair
4. **Test 4 (t=431s)**: Doble expiración → 401 correcto

**Key Rotation Flow Implementado** (Test 3):
```bash
# Secuencia crítica para rotación Ed25519
1. Preservar OLD private key (cp .old)
2. Generar NEW keypair
3. Guardar NEW private key (cp .new)
4. Restaurar OLD private key para firmar
5. Firmar request con OLD key (payload contiene NEW pub_key)
6. Después de rotación exitosa, cambiar a NEW private key
```

**¿Por qué esta secuencia?**
- Request DEBE firmarse con OLD private key (backend valida con OLD pub_key del refresh token actual)
- Payload contiene NEW pub_key para que backend lo use en nuevos tokens
- Solo después de rotación exitosa, cliente cambia a NEW private key

### 🧹 Limpieza de Scripts Obsoletos

**Eliminados (3 archivos)**:
- `scripts/test_2_3_complete.sh` - Test antiguo supersedido
- `scripts/debug_test.sh` - Debug helpers obsoletos
- `scripts/test_key_rotation.sh` - Test manual antiguo
- `cookies_test.txt` - Archivos temporales no usados

**Mantenidos**:
- `scripts/final_test.sh` - Main test suite (35 tests API)
- `scripts/test_2_3_system.sh` - Sistema 2/3 key rotation (4 tests)
- Helpers: `create_signed_request.js`, `verify_signed_response.js`, `sign_query_params.js`, `generate_hash.js`

### 📚 Documentación Actualizada

**6 archivos actualizados** para reflejar cambios:
1. **CHANGELOG.md** - Nueva entrada v1.6.23 con root cause analysis completo
2. **api/Cargo.toml** - Versión: 1.6.21 → 1.6.23
3. **docs/guides/testing.md** - Nueva sección "Ed25519 Key Rotation Testing (2/3 System)"
4. **docs/guides/key-rotation-testing.md** - Sección "Test Automation" completamente reescrita
5. **README.md** - Tests actualizados: "35 tests" → "39 tests (35 + 4 key rotation)"
6. **CLAUDE.md** - Esta entrada de sesión

### 📊 Estadísticas de Sesión

- **37 archivos modificados** (+1046, -840 líneas)
- **5 backend fixes** críticos en cadena JWT
- **1 test nuevo** con 4 sub-tests (382 líneas)
- **3 scripts obsoletos** eliminados
- **6 documentos** actualizados
- **Versión**: API v1.6.21 → v1.6.23

### ⚠️ Pending Items

**Ninguno** - Sesión completamente finalizada:
- ✅ Bug crítico corregido y verificado
- ✅ Test automatizado creando y funcionando 100%
- ✅ Scripts obsoletos eliminados
- ✅ Documentación completamente actualizada
- ✅ Versiones incrementadas correctamente

### 🎯 Next Steps (Futuras Sesiones)

**Ninguno urgente**. Sistema 2/3 key rotation completamente funcional y testeado.

**Posibles mejoras futuras**:
- E2E tests con Playwright para key rotation UI flow
- Performance benchmarks de rotación de claves
- Monitoreo de métricas de rotación en producción

---

## Historial Técnico (ver CHANGELOG.md para detalles completos)

### Logros Clave del Proyecto:
- **Zero Knowledge Auth**: Sistema completo donde servidor nunca almacena emails/PII. User IDs criptográficos con Blake3 pipeline
- **Enterprise Architecture**: Refactorización 3,698 líneas monolíticas → módulos <225 líneas. Arquitectura modular DRY/SOLID
- **Blake3 Migration**: Migración completa Blake2→Blake3 con WASM SIMD optimization. Performance ~100x en magic links
- **Ed25519 Integration**: Sistema completo frontend-backend con firmas digitales. WebCrypto + Noble fallback
- **Ed25519 Key Rotation**: Sistema automático 2/3 time window. Rotación transparente de claves sin interrupción usuario ✅ **v1.6.23: Bug crítico corregido**
- **URL Encryption**: Sistema ultra-compacto de cifrado parámetros URL (66% reducción) con FIFO rotation. ChaCha20-Poly1305
- **Dual-Token JWT**: Auto-refresh transparente con HttpOnly cookies. Access tokens 20s dev, refresh 2min dev
- **100% SignedResponse**: TODOS endpoints (excepto `/api/version`) validan Ed25519 y emiten SignedResponse
- **Email System**: Mailtrap integration con templates 13 idiomas + RTL support. Custom domain `mailer.hashrand.com`
- **Testing**: 39 tests automatizados con 100% success rate (35 API + 4 key rotation). Coverage completo auth flow + generación

## Detalles Adicionales
Ver README.md y CHANGELOG.md para detalles completos de implementación.