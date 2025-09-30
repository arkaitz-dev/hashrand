# Security Plan: Token+Cookie Separation & Domain-Based Cookie Management

**Fecha:** 2025-09-30
**Estado:** IMPLEMENTACIÓN FASE 1 Y FASE 2 COMPLETADAS
**Prioridad:** CRÍTICA - SEGURIDAD

## ✅ FASE 1 COMPLETADA

### Backend Implementado:
- ✅ Guardar ui_host en blob cifrado (con backward compatibility)
- ✅ Extraer ui_host del blob en validación
- ✅ Usar Domain attribute en cookies (login inicial)
- ✅ Hacer ui_host obligatorio sin fallback

### Frontend Implementado:
- ✅ Crear extractDomain() para obtener solo hostname
- ✅ Actualizar todos los usos de ui_host
- ✅ Configurar credentials: 'omit' por defecto
- ✅ validateMagicLink con credentials: 'include'

## ✅ FASE 2 COMPLETADA

### Backend Implementado:
- ✅ Propagar Domain en refresh: Extraer hostname del Host header y usarlo en cookie renovada
- ✅ Middleware validación token+cookie: Creado `auth_validation_middleware.rs`
- ✅ Integrado middleware en `protected_endpoint_middleware.rs` (endpoints POST autenticados)
- ✅ Integrado middleware en `jwt_middleware_auth.rs` (endpoints GET autenticados)

### Detalles de Implementación:
- **Archivo creado:** `api/src/utils/auth_validation_middleware.rs`
  - Función `validate_no_simultaneous_tokens()` rechaza requests con ambos tokens
  - Status code 403 (Forbidden) para violaciones de seguridad
  - Logs detallados de violaciones y validaciones exitosas
  - Tests unitarios incluidos

- **Integración en endpoints protegidos:**
  - POST autenticados: Validación en `extract_jwt_info()` de `protected_endpoint_middleware.rs`
  - GET autenticados: Validación al inicio de `validate_bearer_token()` en `jwt_middleware_auth.rs`
  - Nota: Auto-refresh desde cookies en GET endpoints queda deshabilitado (correcto con `credentials: 'omit'`)

- **Domain en refresh:**
  - Función `extract_hostname_from_host_header()` en `refresh_token.rs`
  - Extrae hostname del Host header (validación de formato)
  - Usa hostname en cookie renovada con Domain attribute
  - Backward compatibility: Si no hay Host header, cookie sin Domain

## ⏸️ FASE 3 PENDIENTE

### Tareas Restantes:
1. **Testing completo del flujo de autenticación:**
   - Login con magic link
   - Refresh token (tramos 1/3 y 2/3)
   - Logout
   - Endpoints autenticados (generators)
   - Verificar rechazo de requests con ambos tokens
   - Verificar que cookies NO se envían a endpoints autenticados

## 🎯 OBJETIVO

Implementar separación estricta entre access token (Authorization header) y refresh token (HttpOnly cookie) para prevenir que un request pueda enviar ambos simultáneamente. Además, implementar gestión de cookies basada en Domain attribute para máxima seguridad.

## 🔒 RAZONES DE SEGURIDAD

### 1. Separación Token + Cookie
**PROBLEMA:** Actualmente el backend acepta requests con ambos tokens simultáneamente.

**RIESGO:**
- Confusión en la lógica de autenticación
- Posible explotación si un atacante captura un access token y también tiene acceso a cookies
- Violación del principio de diseño: cada tipo de token debe tener un único canal

**SOLUCIÓN:** Rechazar cualquier request que incluya `Authorization: Bearer <token>` + cookie `refresh_token` simultáneamente.

### 2. Credentials 'omit' por defecto
**PROBLEMA:** El frontend no especifica `credentials: 'omit'` explícitamente.

**RIESGO:**
- El navegador puede enviar cookies automáticamente en ciertos contextos
- Endpoints autenticados con JWT no deben recibir cookies (solo Authorization header)
- Puede causar problemas CORS y envío no deseado de cookies

**SOLUCIÓN:**
- `credentials: 'omit'` por defecto en TODOS los endpoints
- EXCEPCIONES: `/api/login/magiclink/` (para recibir cookie) y `/api/refresh` (para enviar cookie)

### 3. Domain Attribute en Cookies
**PROBLEMA:** Las cookies no incluyen `Domain` attribute, lo que las hace menos seguras.

**RIESGO:**
- La cookie solo funciona para el hostname exacto donde se creó
- No funciona en subdominios si es necesario
- Más difícil auditar y validar el origen correcto

**SOLUCIÓN:**
- Guardar `ui_host` (solo hostname) en blob cifrado de magiclinks
- Usar ese hostname como `Domain` attribute en la cookie
- En refresh, extraer Domain de la cookie recibida (nunca del request)

### 4. ui_host OBLIGATORIO
**PROBLEMA:** `ui_host` es opcional con fallback al request header.

**RIESGO:**
- El servidor podría generar cookies con Domain incorrecto si confía en headers manipulables
- Fallbacks ocultan errores de integración

**SOLUCIÓN:**
- Hacer `ui_host` obligatorio sin fallback
- Si no existe, fallar con error claro
- Garantizar que el frontend siempre provee el hostname correcto

## 📋 PLAN DE IMPLEMENTACIÓN

### FASE 1: BACKEND - Validación Token+Cookie

#### 1.1. Crear Middleware de Validación
**Archivo:** `api/src/utils/auth_validation_middleware.rs`

```rust
/// Validates that requests don't contain both Authorization header and refresh_token cookie
pub fn validate_no_simultaneous_tokens(req: &Request) -> Result<(), String> {
    let has_auth_header = req.header("authorization").is_some();
    let has_refresh_cookie = req.header("cookie")
        .and_then(|h| h.as_str())
        .map(|cookies| cookies.contains("refresh_token="))
        .unwrap_or(false);

    if has_auth_header && has_refresh_cookie {
        Err("SECURITY: Request contains both Authorization header and refresh cookie - forbidden".to_string())
    } else {
        Ok(())
    }
}
```

**INTEGRAR EN:**
- `protected_endpoint_middleware.rs:extract_jwt_info()` (endpoints autenticados POST)
- `jwt_middleware_auth.rs` (endpoints autenticados GET)

#### 1.2. Hacer ui_host Obligatorio
**Archivo:** `api/src/utils/auth/magic_link_gen.rs:34-79`

**ANTES:**
```rust
let host_url = Self::determine_host_url(req, payload.ui_host.as_deref());
```

**DESPUÉS:**
```rust
let ui_host = payload.ui_host.as_deref()
    .ok_or_else(|| /* error response */)?;
println!("🔒 [SECURITY] ui_host OBLIGATORIO recibido: {}", ui_host);
```

**TAMBIÉN EN:** `api/src/utils/auth/magic_link_token_gen.rs:64-75` (eliminar fallback)

#### 1.3. Guardar ui_host en Blob Cifrado
**Archivos:**
- `api/src/database/operations/magic_link_storage.rs:28-100`
- `api/src/database/operations/magic_link_validation.rs:30-155`
- `api/src/database/operations/magic_link_types.rs` (añadir campo)

**NUEVO FORMATO PAYLOAD:**
```
[encryption_blob:44][pub_key:32][ui_host_len:2][ui_host:variable][next_param:variable]
```

**VALIDACIÓN:** Añadir campo `ui_host: String` en `ValidationResult`

#### 1.4. Cookie con Domain Attribute
**Archivo:** `api/src/utils/auth/magic_link_auth_response_builder.rs:82-106`

**MODIFICAR:**
```rust
fn create_secure_refresh_cookie(refresh_token: &str, domain: &str) -> anyhow::Result<String> {
    let cookie_value = format!(
        "refresh_token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Domain={}; Path=/",
        refresh_token,
        refresh_duration_minutes * 60,
        domain
    );
    // ...
}
```

**INTEGRAR:** Pasar `ui_host` del blob a la función

#### 1.5. Extraer Domain en Refresh
**Archivo:** `api/src/utils/auth/refresh_token.rs:16-450`

**AÑADIR FUNCIÓN:**
```rust
fn extract_domain_from_cookie_header(cookie_header: &str) -> Option<String> {
    // Parse Domain attribute from Set-Cookie format
    // Return only if Domain is explicitly set
}
```

**USAR EN refresh:** Líneas 323-327 y 334-339 para crear nueva cookie con Domain extraído

### FASE 2: FRONTEND - Domain Extraction & Credentials

#### 2.1. Crear Función extractDomain()
**Archivo NUEVO:** `web/src/lib/utils/domain-extractor.ts`

```typescript
/**
 * Extract domain (hostname only) from current location
 * SECURITY: Returns only hostname without protocol or port
 */
export function extractDomain(): string {
    if (typeof window === 'undefined') {
        throw new Error('extractDomain() can only be called in browser context');
    }
    return window.location.hostname;
}
```

#### 2.2. Actualizar Usos de ui_host
**Archivos:**
- `web/src/lib/stores/auth/auth-actions.ts:24`
- `web/src/lib/utils/auth.ts:38`
- `web/src/lib/components/AuthConfirmDialogContent.svelte:32`

**CAMBIO:**
```typescript
// ANTES
const ui_host = window.location.origin; // http://localhost:5173

// DESPUÉS
import { extractDomain } from '$lib/utils/domain-extractor';
const ui_host = extractDomain(); // localhost
```

#### 2.3. Configurar Credentials en Requests
**Archivo:** `web/src/lib/httpSignedRequests.ts`

**CAMBIOS:**
1. `httpSignedPOSTRequest` (L39-84): Default `credentials: 'omit'`
2. `httpSignedGETRequest` (L97-141): Añadir `credentials: 'omit'`
3. `httpAuthenticatedSignedPOSTRequest` (L153-204): Añadir `credentials: 'omit'`
4. `httpAuthenticatedSignedGETRequest` (L216-271): Añadir `credentials: 'omit'`
5. `httpSignedDELETERequest` (L283-323): Default `credentials: 'omit'`

**EXCEPCIÓN EN:** `httpSignedAuthenticatedDELETE` (L358) mantener `credentials: 'include'`

#### 2.4. Actualizar validateMagicLink
**Archivo:** `web/src/lib/api/api-auth-operations.ts:63-71`

**AÑADIR:**
```typescript
return await httpSignedPOSTRequest<{ magiclink: string }, LoginResponse>(
    `${API_BASE}/login/magiclink/`,
    { magiclink: magicToken },
    false,
    { credentials: 'include' } // NUEVO: Para recibir cookie
);
```

### FASE 3: LOGGING & DEBUGGING

#### 3.1. Backend Logs
Añadir en puntos críticos:
- Validación token+cookie (rechazos)
- Recepción de ui_host (valor recibido)
- Almacenamiento en blob (confirmación)
- Extracción de blob (valor recuperado)
- Creación de cookie con Domain (valor usado)
- Extracción de Domain en refresh (valor extraído)

#### 3.2. Frontend Flash Messages
Añadir en:
- `requestMagicLink`: "🔒 Domain enviado: {domain}"
- `validateMagicLink`: "🍪 Cookie recibida correctamente"
- `refreshToken`: "🔄 Domain de cookie: {domain}"
- Errores de validación

## 🚨 PUNTOS CRÍTICOS

### 1. ORDEN DE IMPLEMENTACIÓN
**IMPORTANTE:** Implementar en este orden exacto:
1. Backend: Guardar ui_host en blob (compatibilidad hacia atrás)
2. Frontend: Enviar solo hostname
3. Backend: Hacer ui_host obligatorio
4. Backend: Usar Domain en cookies
5. Backend: Validación token+cookie
6. Frontend: Configurar credentials

### 2. COMPATIBILIDAD
- Los magic links existentes en DB NO tendrán ui_host en el blob
- Añadir manejo de compatibilidad: si no existe ui_host en blob, usar fallback temporal
- Después de migración completa, eliminar fallback

### 3. TESTING
Después de cada fase:
1. Login completo (magic link)
2. Refresh token (ambos tramos 1/3 y 2/3)
3. Logout
4. Endpoints autenticados (POST generators)

### 4. ROLLBACK
Si algo falla:
1. Revertir cambios de credentials en frontend primero
2. Luego revertir validación token+cookie en backend
3. Mantener ui_host en blob (no causa problemas)

## 📝 VERIFICACIÓN POST-IMPLEMENTACIÓN

### Checklist Backend
- [x] Requests con token+cookie simultáneos son rechazados
- [x] ui_host es obligatorio en /api/login/
- [x] ui_host se guarda en blob cifrado
- [x] ui_host se extrae del blob en /api/login/magiclink/
- [x] Cookie incluye Domain attribute
- [x] Refresh extrae Domain del Host header
- [x] Logs muestran información de debug

### Checklist Frontend
- [x] extractDomain() retorna solo hostname
- [x] Todos los usos de ui_host usan extractDomain()
- [x] credentials: 'omit' en endpoints autenticados
- [x] credentials: 'include' solo en /login/magiclink/ y /refresh
- [x] Flash messages muestran información útil (logs de consola)

### Checklist Testing
- [ ] Flujo completo de login funciona
- [ ] Refresh en tramo 1/3 funciona (sin rotación)
- [ ] Refresh en tramo 2/3 funciona (con rotación)
- [ ] Logout funciona correctamente
- [ ] Generadores autenticados funcionan
- [ ] No se envían cookies a endpoints autenticados

## 🔐 IMPACTO EN SEGURIDAD

**MEJORAS:**
1. ✅ Separación estricta de canales de autenticación
2. ✅ Domain-based cookies para mejor control
3. ✅ Validación explícita de origen (ui_host obligatorio)
4. ✅ Prevención de envío accidental de cookies
5. ✅ Mejor auditabilidad con logs detallados

**RIESGOS MITIGADOS:**
1. ✅ Token hijacking con múltiples canales
2. ✅ Cookie scope confusion
3. ✅ CSRF mejorado con Domain explícito
4. ✅ Envío no intencionado de credentials

---

**NOTA:** Este documento debe preservarse durante todo el proceso de implementación y servir como referencia para debugging si algo falla.