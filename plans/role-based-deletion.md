# Plan: Role-Based Deletion (Emisor vs Receptor)

**Fecha**: 2025-10-09
**Objetivo**: Implementar lógica de borrado diferenciada según rol (emisor/receptor) reutilizando código existente

---

## 🎯 Arquitectura Objetivo

### Emisor (`role = sender, pending_reads = -1`)
- **Botón visible**: SIEMPRE (sin condiciones)
- **Puede borrar**: SIEMPRE (sin validar `pending_reads`)
- **Al borrar elimina**:
  - ✅ Su entrada en `shared_secrets` (tabla principal)
  - ✅ La entrada en `shared_secrets_tracking` (referencia compartida)
- **Efecto**: Receptor pierde acceso (tracking desaparece)

### Receptor (`role = receiver, pending_reads = 1-10`)
- **Botón visible**: Solo si `pending_reads > 0`
- **Puede borrar**: Solo si `pending_reads > 0`
- **Al borrar elimina**:
  - ✅ Su entrada en `shared_secrets` (solo su copia)
  - ❌ La entrada en `shared_secrets_tracking` **NO se toca**
- **Efecto**: Emisor mantiene acceso (tracking permanece)

### Validación al Servir (GET)
- Si no existe entrada en `shared_secrets_tracking`:
  - → Emisor borró todo
  - → Eliminar entrada de `shared_secrets` del receptor (cleanup)
  - → Retornar error 410 Gone

---

## 📋 TAREAS

### ✅ FRONTEND (Ya completado - solo verificación)

**Archivo**: `web/src/routes/shared-secret/[hash]/+page.svelte`

**Estado actual**:
```svelte
<!-- Delete Button (always shown for sender, hidden for consumed receiver) -->
{#if secret.role === 'sender' || secret.pending_reads > 0}
```

**Verificación**: ✅ Ya implementado correctamente en línea 434

---

### 🔧 BACKEND - Parte 1: Crear función para borrar tracking

**Archivo**: `api/src/database/operations/shared_secret_storage.rs`

**Tarea 1.1**: Crear función `delete_tracking_by_reference_hash`
- **Ubicación**: Después de `store_tracking_record` (línea ~400)
- **Función**: Eliminar entrada de `shared_secrets_tracking` por `reference_hash`
- **Patrón SOLID**: Single Responsibility (solo elimina tracking)
- **Código**:

```rust
/// Delete tracking record by reference_hash
///
/// # Arguments
/// * `reference_hash` - 32-byte reference hash
///
/// # Returns
/// * `Result<bool, SqliteError>` - true if deleted, false if not found
pub fn delete_tracking_by_reference_hash(
    reference_hash: &[u8; REFERENCE_HASH_LENGTH],
) -> Result<bool, SqliteError> {
    let connection = get_database_connection()?;

    debug!("🗑️ Deleting tracking record by reference_hash");

    connection.execute(
        "DELETE FROM shared_secrets_tracking WHERE reference_hash = ?",
        &[Value::Blob(reference_hash.to_vec())],
    )?;

    // Check if row was deleted (rowcount would be ideal but not available in Spin SDK)
    // For now, return true (assume deletion happened if no error)
    debug!("✅ Tracking record deleted (or didn't exist)");
    Ok(true)
}
```

**Estimación**: 10 minutos
**Riesgo**: Bajo (función nueva, no afecta código existente)

---

### 🔧 BACKEND - Parte 2: Crear función para verificar existencia de tracking

**Archivo**: `api/src/database/operations/shared_secret_storage.rs`

**Tarea 2.1**: Crear función `tracking_exists`
- **Ubicación**: Después de `get_pending_reads_from_tracking` (línea ~310)
- **Función**: Verificar si existe entrada de tracking
- **Patrón DRY**: Reutiliza lógica de `get_pending_reads_from_tracking`
- **Código**:

```rust
/// Check if tracking record exists by reference_hash
///
/// # Arguments
/// * `reference_hash` - 32-byte reference hash
///
/// # Returns
/// * `Result<bool, SqliteError>` - true if exists, false if not
pub fn tracking_exists(
    reference_hash: &[u8; REFERENCE_HASH_LENGTH],
) -> Result<bool, SqliteError> {
    Ok(Self::get_pending_reads_from_tracking(reference_hash)?.is_some())
}
```

**Estimación**: 5 minutos
**Riesgo**: Bajo (wrapper simple de función existente)

---

### 🔧 BACKEND - Parte 3: Modificar lógica de borrado

**Archivo**: `api/src/handlers/shared_secret/deletion.rs`

**Tarea 3.1**: Modificar `delete_secret_validated_v2`
- **Ubicación**: Línea 86-147
- **Cambios**:
  1. Extraer `role` del hash (ya se hace en línea 101-103)
  2. Bifurcar lógica según `role`
  3. Emisor: borrar shared_secrets + tracking (sin validar pending_reads)
  4. Receptor: validar pending_reads > 0, borrar solo shared_secrets

**Código modificado** (líneas 86-147):

```rust
/// Delete secret with role-based validation (v2 - UPDATED)
fn delete_secret_validated_v2(
    encrypted_hash: &[u8; 40],
    user_id_from_jwt: &[u8; USER_ID_LENGTH],
    crypto_material: &CryptoMaterial,
) -> Result<Response, String> {
    // ============================================================================
    // 3-LAYER VALIDATION: Checksum → Ownership → Database
    // ============================================================================

    // Layer 1: Decrypt ChaCha20 hash
    let decrypted_hash = SharedSecretCrypto::decrypt_url_hash(encrypted_hash)
        .map_err(|e| format!("Failed to decrypt hash: {}", e))?;

    // Layer 2: Validate checksum + Extract components (reference_hash, user_id, role)
    let (reference_hash, user_id_from_hash, role) =
        SharedSecretCrypto::validate_and_extract_hash(&decrypted_hash)
            .map_err(|e| format!("Invalid hash checksum: {}", e))?;

    // Layer 3: CRITICAL - Validate ownership (user_id from JWT must match user_id from hash)
    if user_id_from_jwt != &user_id_from_hash {
        return Err(
            "Access denied: You cannot delete a shared secret that doesn't belong to you"
                .to_string(),
        );
    }

    // Generate db_index for database lookup
    let db_index = SharedSecretCrypto::generate_db_index(&reference_hash, &user_id_from_hash)
        .map_err(|e| format!("Failed to generate db_index: {}", e))?;

    // ============================================================================
    // ROLE-BASED DELETION LOGIC
    // ============================================================================

    match role {
        SecretRole::Sender => {
            // EMISOR: Borrar TODO (shared_secrets + tracking)
            // No validar pending_reads (emisor puede borrar siempre)

            // 1. Delete from shared_secrets
            let deleted = SharedSecretStorage::delete_secret(&db_index)
                .map_err(|e| format!("Failed to delete secret: {}", e))?;

            if !deleted {
                return Err("Secret not found or already deleted".to_string());
            }

            // 2. Delete from tracking (elimina referencia compartida)
            SharedSecretStorage::delete_tracking_by_reference_hash(&reference_hash)
                .map_err(|e| format!("Failed to delete tracking: {}", e))?;

            // Success response
            let response_json = json!({
                "success": true,
                "message": "Secret deleted successfully (sender - full deletion)",
                "role": "sender"
            });

            create_signed_endpoint_response(&response_json, crypto_material)
                .map_err(|e| format!("Failed to create signed response: {}", e))
        }

        SecretRole::Receiver => {
            // RECEPTOR: Validar pending_reads > 0, borrar solo shared_secrets

            // Read secret to get pending_reads from tracking
            let (_, pending_reads, _, _) = SharedSecretOps::read_secret(&db_index)
                .map_err(|e| format!("Failed to read secret: {}", e))?;

            // Validate: Only allow deletion if pending_reads > 0
            if pending_reads == 0 {
                return Err(
                    "Cannot delete secret: all reads have been consumed"
                        .to_string(),
                );
            }

            // Delete only from shared_secrets (tracking permanece)
            let deleted = SharedSecretStorage::delete_secret(&db_index)
                .map_err(|e| format!("Failed to delete secret: {}", e))?;

            if !deleted {
                return Err("Secret not found or already deleted".to_string());
            }

            // Success response
            let response_json = json!({
                "success": true,
                "message": "Secret deleted successfully (receiver - partial deletion)",
                "role": "receiver"
            });

            create_signed_endpoint_response(&response_json, crypto_material)
                .map_err(|e| format!("Failed to create signed response: {}", e))
        }
    }
}
```

**Estimación**: 20 minutos
**Riesgo**: Medio (modifica lógica core, requiere testing exhaustivo)

---

### 🔧 BACKEND - Parte 4: Validación al servir (GET)

**Archivo**: `api/src/handlers/shared_secret/retrieval.rs`

**Tarea 4.1**: Agregar validación de tracking en `handle_get_secret_v2`
- **Ubicación**: Después de decodificar y validar hash, antes de leer secret
- **Lógica**: Si tracking no existe → cleanup → error 410 Gone

**Buscar línea actual**:
```rust
// Read secret from database (uses db_index)
let (encrypted_payload, expires_at, pending_reads, role) =
```

**Insertar ANTES**:

```rust
// ============================================================================
// VALIDATION: Check if tracking exists (if sender deleted, cleanup receiver)
// ============================================================================
if !SharedSecretStorage::tracking_exists(&reference_hash)
    .map_err(|e| format!("Failed to check tracking existence: {}", e))?
{
    // Tracking doesn't exist → Sender deleted everything
    // Cleanup: delete receiver's shared_secrets entry if exists
    let _ = SharedSecretStorage::delete_secret(&db_index); // Ignore errors (may not exist)

    return Err(
        "Secret no longer available: sender has deleted it"
            .to_string(),
    );
}
```

**Estimación**: 15 minutos
**Riesgo**: Medio (afecta flujo principal de GET)

---

## 🧪 TESTING

### Test Cases

**TC1: Emisor borra secreto**
- ✅ Emisor accede con `pending_reads = -1`
- ✅ Botón "Eliminar" visible
- ✅ Click en eliminar → Confirmación → Borrado exitoso
- ✅ Verificar: shared_secrets (emisor) eliminado
- ✅ Verificar: shared_secrets_tracking eliminado
- ✅ Receptor intenta acceder → Error 410 Gone

**TC2: Receptor borra secreto (con lecturas restantes)**
- ✅ Receptor accede con `pending_reads = 3`
- ✅ Botón "Eliminar" visible
- ✅ Click en eliminar → Confirmación → Borrado exitoso
- ✅ Verificar: shared_secrets (receptor) eliminado
- ✅ Verificar: shared_secrets_tracking PERMANECE
- ✅ Emisor puede seguir accediendo

**TC3: Receptor intenta borrar secreto consumido**
- ✅ Receptor accede con `pending_reads = 0`
- ✅ Botón "Eliminar" NO visible
- ❌ Intento manual de DELETE → Error 400 (pending_reads == 0)

**TC4: Receptor intenta acceder tras borrado de emisor**
- ✅ Emisor borra secreto (tracking eliminado)
- ✅ Receptor intenta acceder → Error 410 Gone
- ✅ Entrada de shared_secrets (receptor) auto-limpiada

---

## 📊 ORDEN DE IMPLEMENTACIÓN

1. **Backend Parte 1**: Crear `delete_tracking_by_reference_hash` (10 min)
2. **Backend Parte 2**: Crear `tracking_exists` (5 min)
3. **Backend Parte 3**: Modificar `delete_secret_validated_v2` (20 min)
4. **Backend Parte 4**: Agregar validación en GET (15 min)
5. **Testing**: Ejecutar test cases (30 min)
6. **Frontend**: Verificación (ya implementado) (5 min)

**Total estimado**: ~85 minutos

---

## ⚠️ PRECAUCIONES

1. **No modificar firmas de funciones existentes** (SOLID - Open/Closed Principle)
2. **Crear nuevas funciones en lugar de modificar existentes** cuando sea posible
3. **Reutilizar funciones de validación existentes** (DRY)
4. **Logging exhaustivo** en cada operación de borrado (debug/info)
5. **Manejar errores de SQLite** correctamente (no asumir éxito)
6. **Testing manual obligatorio** antes de commit (no confiar solo en tests automatizados)

---

## 📝 NOTAS TÉCNICAS

### Estructura de Tablas

**`shared_secrets`**:
- `id` (db_index): 32 bytes - PRIMARY KEY
- `encrypted_payload`: BLOB
- `expires_at`: INTEGER
- `role`: TEXT ('sender' | 'receiver')

**`shared_secrets_tracking`**:
- `reference_hash`: 32 bytes - PRIMARY KEY (compartida emisor/receptor)
- `pending_reads`: INTEGER (-1 para emisor, 1-10 para receptor)
- `max_reads`: INTEGER
- `read_at`: INTEGER (nullable)
- `expires_at`: INTEGER
- `created_at`: INTEGER

### Funciones Reutilizadas

- `SharedSecretCrypto::decrypt_url_hash()` - Desencripta hash ChaCha20
- `SharedSecretCrypto::validate_and_extract_hash()` - Valida checksum y extrae componentes
- `SharedSecretCrypto::generate_db_index()` - Genera db_index (reference_hash + user_id)
- `SharedSecretStorage::delete_secret()` - Elimina de shared_secrets
- `SharedSecretStorage::get_pending_reads_from_tracking()` - Lee pending_reads de tracking
- `SharedSecretOps::read_secret()` - Lee secret completo (wrapper)

### Imports Necesarios

```rust
use super::shared_secret_types::SecretRole; // Para match role
```

---

## ✅ CHECKLIST DE COMPLETITUD

- [ ] Función `delete_tracking_by_reference_hash` creada
- [ ] Función `tracking_exists` creada
- [ ] `delete_secret_validated_v2` modificada con lógica role-based
- [ ] Validación de tracking en GET implementada
- [ ] Tests manuales TC1-TC4 ejecutados exitosamente
- [ ] `just check` pasa sin errores
- [ ] `just test` pasa sin errores (si tests automatizados existen)
- [ ] Commit con mensaje descriptivo
- [ ] Plan actualizado con resultados

---

**Fin del plan**
