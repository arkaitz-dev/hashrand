# Implementation Plan - Sistema de Autenticación con JWT y Magic Links
*Creado: 2025-08-27*

## Análisis de Requerimientos

### Funcionalidad Solicitada
- **Sistema de login** que se activa al intentar generar hash en las vistas: custom/, password/, api-key/, mnemonic/
- **Diálogo de autenticación** similar al de result/ con input para email
- **Magic Link Authentication** via email con JWT tokens
- **Doble token system**: Access token (15 min) + Refresh token HttpOnly cookie (1 semana)
- **Endpoint /api/login/** con POST (email) y GET (magic link)
- **Base de datos** para gestionar sesiones y tokens
- **Logging durante desarrollo** en lugar de envío real de emails

### Arquitectura Actual
- **API**: Rust + Fermyon Spin con SQLite
- **Web**: SvelteKit + TypeScript con sistema de stores
- **Base de datos**: SQLite con ambiente dev/prod automático
- **Estilos**: TailwindCSS con tema dark/light

## Dependencias Requeridas

### API Backend (Rust)
- `jsonwebtoken = "9.3.0"` - JWT token generation/validation
- `chrono = "0.4.34"` - Date/time handling for token expiration
- `uuid = "1.10.0"` - Unique identifiers for magic links
- `base64 = "0.22.1"` - Base64 encoding for tokens

### Web Frontend (TypeScript)
- Usar dependencias existentes de SvelteKit
- Añadir tipos para auth en `types/index.ts`

## Modificaciones de Base de Datos

### Nueva tabla `auth_sessions`
```sql
CREATE TABLE auth_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL,
    magic_token TEXT NOT NULL UNIQUE,
    access_token TEXT,
    refresh_token TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    magic_expires_at DATETIME NOT NULL,
    access_expires_at DATETIME,
    refresh_expires_at DATETIME,
    is_used BOOLEAN DEFAULT FALSE
);
```

### Modificación tabla `users`
- Añadir campo `last_login` para tracking
- Modificar para usar email como identificador principal

## Plan de Implementación

### Fase 1: Backend API - Autenticación Core
- [ ] Añadir dependencias JWT al Cargo.toml
- [ ] Crear modelo `AuthSession` en database/models.rs
- [ ] Crear operaciones CRUD para auth_sessions en database/operations.rs
- [ ] Crear utilidades JWT en utils/jwt.rs
- [ ] Implementar handler login en handlers/login.rs
- [ ] Actualizar routing en utils/routing.rs para /api/login/

### Fase 2: Backend API - Magic Link System
- [ ] Implementar generación de magic links
- [ ] Implementar validación y consumo de magic links
- [ ] Añadir logging para desarrollo (simulación email)
- [ ] Implementar refresh token logic
- [ ] Añadir middleware de autenticación para endpoints protegidos

### Fase 3: Frontend - Auth Store y Servicios
- [ ] Crear store de autenticación en stores/auth.ts
- [ ] Añadir tipos TypeScript para auth en types/index.ts
- [ ] Crear servicio API para auth en api.ts
- [ ] Implementar interceptor para refresh tokens automático

### Fase 4: Frontend - Componentes UI
- [ ] Crear componente LoginDialog.svelte
- [ ] Crear componente AuthGuard.svelte
- [ ] Modificar GenerateButton.svelte para integrar auth
- [ ] Añadir manejo de parámetros ?magiclink= en +layout.svelte

### Fase 5: Frontend - Integración con Generadores
- [ ] Modificar custom/+page.svelte para auth guard
- [ ] Modificar password/+page.svelte para auth guard
- [ ] Modificar api-key/+page.svelte para auth guard
- [ ] Modificar mnemonic/+page.svelte para auth guard

### Fase 6: Testing y Validación
- [ ] Crear tests para endpoints de auth
- [ ] Probar flujo completo de magic link
- [ ] Validar refresh token automático
- [ ] Probar integración con todos los generadores
- [ ] Verificar manejo de errores y edge cases

## Puntos de Integración

### API Endpoints Afectados
- **Nuevo**: `POST /api/login/` - Envío de magic link por email
- **Nuevo**: `GET /api/login/?magiclink=...` - Validación y login
- **Modificados**: `/api/custom`, `/api/password`, `/api/api-key`, `/api/mnemonic` - Require auth

### Frontend Components Afectados
- **Nuevo**: `LoginDialog.svelte` - Modal de autenticación
- **Nuevo**: `AuthGuard.svelte` - Protección de rutas
- **Modificados**: Todas las páginas de generadores
- **Modificados**: `GenerateButton.svelte` - Check auth antes de generate

### Stores y Estado
- **Nuevo**: `auth.ts` - Estado de autenticación global
- **Modificado**: `navigation.ts` - Integrar con auth state

## Riesgos y Mitigación

### Seguridad
- **Riesgo**: JWT secret hardcodeado
- **Mitigación**: Usar variable de entorno o generar dinámicamente
- **Riesgo**: Magic links en logs
- **Mitigación**: Enmascarar tokens en logs de producción

### UX/UI
- **Riesgo**: Interrupción del flujo de usuario
- **Mitigación**: Modal no-intrusivo, conservar parámetros de form
- **Riesgo**: Token expiration durante uso
- **Mitigación**: Refresh automático transparente

### Desarrollo
- **Riesgo**: Magic links difíciles de probar
- **Mitigación**: Logging claro y modo dev con links simples

## Validación Final

### Checklist de Funcionalidad
- [ ] Magic link se genera correctamente
- [ ] Email se loggea en desarrollo
- [ ] JWT tokens se crean con expiración correcta
- [ ] Refresh token funciona automáticamente  
- [ ] Auth guard bloquea acceso no autorizado
- [ ] Parámetros de formulario se conservan post-auth
- [ ] Flujo completo funciona en todos los generadores
- [ ] Manejo de errores es user-friendly

### Checklist de Integración
- [ ] Sin impacto en funcionalidad existente
- [ ] Estilos coherentes con tema actual
- [ ] Funciona en modo dark/light
- [ ] Responsive en mobile
- [ ] Accesibilidad mantenida
- [ ] Performance no degradada

## Estimación
- **Complejidad**: Alta (sistema completo de auth)
- **Tiempo estimado**: Implementación completa en una sesión intensiva
- **Archivos afectados**: ~15 archivos (8 nuevos, 7 modificados)
- **Dependencias nuevas**: 4 en backend

---
*Estado: Planning Complete - Ready for Implementation*