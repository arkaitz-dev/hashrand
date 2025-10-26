# Análisis Exhaustivo: `old_x25519_pub_key_hex` Parameter

## Resumen Ejecutivo

**Conclusión**: El parámetro `old_x25519_pub_key_hex` en `handle_key_rotation()` es **código vestigial que NO se usa** y puede eliminarse de forma segura.

## 1. Ubicaciones del Parámetro

### 1.1. Definición de función
**Archivo**: `api/src/utils/auth/refresh_token/period_2_3.rs:38`
```rust
pub fn handle_key_rotation(
    username: &str,
    old_ed25519_pub_key_hex: &str,      // ✅ SE USA
    _old_x25519_pub_key_hex: &str,      // ❌ NO SE USA (prefijo _)
    new_ed25519_pub_key_hex: &str,      // ✅ SE USA
    new_x25519_pub_key_hex: &str,       // ✅ SE USA
    domain: Option<String>,
) -> anyhow::Result<Response>
```

### 1.2. Call site
**Archivo**: `api/src/utils/auth/refresh_token/mod.rs:76-83`
```rust
period_2_3::handle_key_rotation(
    username,
    &ed25519_pub_key_hex,               // ✅ OLD Ed25519 from JWT
    &hex::encode(x25519_pub_key),       // ⚠️ OLD X25519 from JWT - passed but NOT USED
    &refresh_payload.new_ed25519_pub_key,
    &refresh_payload.new_x25519_pub_key,
    domain,
)
```

## 2. Arquitectura de Derivación de Claves Backend

### 2.1. Claves Ed25519 (Signing)
```rust
// OLD Ed25519: Se USA para firmar la respuesta (MITM protection)
SignedResponseGenerator::create_signed_response_with_rotation(
    payload,
    &user_id,
    old_ed25519_pub_key_hex,  // ✅ FIRMA con clave VIEJA
    new_ed25519_pub_key_hex,  // ✅ Incluye clave NUEVA en payload
)
```

**Razón**: El cliente puede verificar la firma con su clave vieja (la que tiene actualmente), mientras recibe la nueva para el próximo ciclo.

### 2.2. Claves X25519 (ECDH)
```rust
// Backend X25519 derivation (period_2_3.rs:93)
let backend_x25519_public = get_backend_x25519_public_key(
    &user_id,
    new_x25519_pub_key_hex  // ⚠️ Solo usa la NUEVA
);
```

**Clave de la arquitectura** (`api/src/utils/crypto/backend_keys.rs:56-87`):
```rust
fn derive_x25519_session_keypair(
    user_id: &[u8],
    client_x25519_pub_key_hex: &str,  // ✅ CRITICAL: Se usa en la derivación
) -> Result<(X25519PrivateKey, X25519PublicKey), SqliteError> {
    // Línea 70-72: Combina user_id + client X25519 pub key
    let mut combined = Vec::new();
    combined.extend_from_slice(user_id);
    combined.extend_from_slice(client_x25519_pub_key_hex.as_bytes());

    // Línea 75: Deriva usando Blake3
    let seed = blake3_keyed_variable(&x25519_derivation_key, &combined, 32);

    // La clave backend X25519 es DETERMINÍSTICA basada en la X25519 del CLIENTE
    let x25519_private = X25519PrivateKey::from(seed);
    let x25519_public = X25519PublicKey::from(&x25519_private);

    Ok((x25519_private, x25519_public))
}
```

**Derivación**:
```
backend_x25519_keypair = blake3(X25519_DERIVATION_KEY, user_id + client_x25519_pub_key_hex)
```

## 3. Por Qué NO se Necesita `old_x25519_pub_key_hex`

### 3.1. La rotación X25519 es "clean cut"

**PERIOD 1/3** (sin rotación):
```rust
// Usa la clave X25519 ACTUAL del JWT refresh token
get_backend_x25519_public_key(&user_id, &x25519_pub_key_hex)
                                        ^^^^^^^^^^^^^^^^^^
                                        VIEJA (del JWT)
```

**PERIOD 2/3** (con rotación):
```rust
// Usa la clave X25519 NUEVA del cliente
get_backend_x25519_public_key(&user_id, new_x25519_pub_key_hex)
                                        ^^^^^^^^^^^^^^^^^^^^^^
                                        NUEVA (del cliente)
```

### 3.2. No hay overlap ni transición gradual

- **Ed25519**: Necesita OLD para firmar (el cliente verifica con la que tiene)
- **X25519**: Solo necesita NEW para derivar el nuevo shared secret

**Razones**:
1. No hay datos pendientes cifrados con el shared secret viejo
2. El cliente ya está autenticado (refresh token válido)
3. La derivación backend cambia INMEDIATAMENTE con la nueva client key
4. No hay ventana de tiempo donde coexistan ambas claves

### 3.3. Test de rotación confirma el comportamiento

**Test**: `api/src/utils/crypto/backend_keys.rs:228-254`
```rust
fn test_key_rotation_produces_different_key() {
    let old_pub_key_hex = "c".repeat(64);
    let new_pub_key_hex = "d".repeat(64);

    let key_old = get_backend_x25519_public_key(user_id, &old_pub_key_hex);
    let key_new = get_backend_x25519_public_key(user_id, &new_pub_key_hex);

    // Confirma que cuando el cliente rota → backend rota también
    assert_ne!(key_old.as_bytes(), key_new.as_bytes(),
        "Key rotation should produce different backend X25519 key");
}
```

## 4. JWT Refresh Token Structure

**Archivo**: `api/src/utils/jwt/types.rs:26-47`
```rust
pub struct RefreshTokenClaims {
    pub sub: String,                    // user_id
    pub exp: i64,                       // expiration
    pub iat: i64,                       // issued at
    pub token_type: String,
    pub session_id: i64,
    pub ed25519_pub_key: [u8; 32],     // ✅ OLD Ed25519 (se extrae y usa)
    pub x25519_pub_key: [u8; 32],      // ⚠️ OLD X25519 (se extrae pero NO se usa)
    pub domain: Option<String>,
}
```

**Extracción en mod.rs**:
```rust
let ed25519_pub_key = &claims.ed25519_pub_key;  // ✅ Se usa
let x25519_pub_key = &claims.x25519_pub_key;    // ⚠️ Se pasa pero no se usa
let ed25519_pub_key_hex = hex::encode(ed25519_pub_key);
```

## 5. Frontend Behavior

**Archivo**: `web/src/lib/api/api-auth-operations/refresh.ts:33-36`
```typescript
// Generate NEW Ed25519 AND X25519 keypairs for potential rotation
const newKeypairs = await generateKeypairs();
const newEd25519PubKeyHex = newKeypairs.ed25519.publicKeyHex;
const newX25519PubKeyHex = newKeypairs.x25519.publicKeyHex;
```

El frontend **SIEMPRE** genera AMBAS claves nuevas antes del refresh, independientemente de si habrá rotación o no.

## 6. Búsqueda de Dependencias

### 6.1. Archivos que mencionan `old_x25519`
```bash
$ grep -r "old_x25519" api/
```

**Resultado**: Solo 2 archivos:
1. `period_2_3.rs` (definición del parámetro)
2. `mod.rs` (call site)

**NO hay**:
- Tests que verifiquen el parámetro
- Validaciones que lo usen
- Documentación que dependa de él
- Lógica condicional que lo evalúe

### 6.2. Archivos que usan `x25519` (43 total)
Ninguno hace referencia específica a `old_x25519_pub_key_hex` excepto los 2 mencionados.

## 7. Impacto de Eliminación

### 7.1. Cambios necesarios

**1. Eliminar parámetro de la función**:
```rust
// Antes
pub fn handle_key_rotation(
    username: &str,
    old_ed25519_pub_key_hex: &str,
    _old_x25519_pub_key_hex: &str,  // ❌ ELIMINAR
    new_ed25519_pub_key_hex: &str,
    new_x25519_pub_key_hex: &str,
    domain: Option<String>,
) -> anyhow::Result<Response>

// Después
pub fn handle_key_rotation(
    username: &str,
    old_ed25519_pub_key_hex: &str,
    new_ed25519_pub_key_hex: &str,
    new_x25519_pub_key_hex: &str,
    domain: Option<String>,
) -> anyhow::Result<Response>
```

**2. Eliminar argumento del call site**:
```rust
// Antes
period_2_3::handle_key_rotation(
    username,
    &ed25519_pub_key_hex,
    &hex::encode(x25519_pub_key),  // ❌ ELIMINAR
    &refresh_payload.new_ed25519_pub_key,
    &refresh_payload.new_x25519_pub_key,
    domain,
)

// Después
period_2_3::handle_key_rotation(
    username,
    &ed25519_pub_key_hex,
    &refresh_payload.new_ed25519_pub_key,
    &refresh_payload.new_x25519_pub_key,
    domain,
)
```

**3. Actualizar documentación**:
```rust
/// # Arguments
/// * `username` - Base58 encoded username
/// * `old_ed25519_pub_key_hex` - Current (OLD) Ed25519 public key hex string
/// * `old_x25519_pub_key_hex` - Current (OLD) X25519 public key hex string  // ❌ ELIMINAR
/// * `new_ed25519_pub_key_hex` - New Ed25519 public key hex string from client
/// * `new_x25519_pub_key_hex` - New X25519 public key hex string from client
```

### 7.2. Impacto en tests
- **Tests unitarios**: ✅ No existen tests que usen `old_x25519_pub_key_hex`
- **Tests E2E**: ✅ No hay referencias en tests (solo 3 archivos totales lo mencionan)
- **Tests de rotación**: ✅ `key-rotation.spec.ts` no usa este parámetro

### 7.3. Impacto en funcionalidad
- **Auth flow**: ✅ Sin impacto (no se usa en ninguna validación)
- **ECDH derivation**: ✅ Sin impacto (solo usa `new_x25519_pub_key_hex`)
- **Signature validation**: ✅ Sin impacto (usa `old_ed25519_pub_key_hex`)
- **Token generation**: ✅ Sin impacto (usa `new_x25519_pub_key_hex`)

## 8. Verificación de Seguridad

### 8.1. ¿Hay algún caso donde necesitaríamos la OLD X25519?

**Pregunta**: ¿Podría haber datos cifrados con el shared secret viejo que necesiten descifrarse?

**Respuesta**: **NO**, porque:

1. **Shared secrets se crean on-demand**: No hay estado persistente cifrado
2. **Magic links se descifran con ChaCha20**: No usan X25519
3. **JWT tokens se firman con Ed25519**: No usan X25519
4. **ECDH solo para `shared_secrets`**: Cada secret usa las claves ACTUALES al momento de creación

### 8.2. ¿Podría haber un race condition?

**Escenario hipotético**: Cliente crea shared secret → rota claves → receptor intenta acceder

**Análisis**:
```rust
// Shared secret creation (sender)
let backend_x25519_private = get_backend_x25519_private_key(&sender_user_id, &sender_x25519_pub_key_hex)?;

// Shared secret retrieval (receiver)
let backend_x25519_private = get_backend_x25519_private_key(&receiver_user_id, &receiver_x25519_pub_key_hex)?;
```

**Respuesta**: **NO hay race condition** porque:
- El `sender_x25519_pub_key_hex` se guarda EN el shared secret mismo
- La derivación backend usa LA CLAVE QUE TENÍA EL SENDER cuando creó el secret
- No depende de la clave ACTUAL del sender

## 9. Historia del Parámetro (Análisis Inferido)

### 9.1. Diseño original (hipótesis)
```
Probablemente el diseño inicial consideró:
1. Simetría con Ed25519 (OLD + NEW)
2. Posible validación de transición
3. Soporte para rollback
4. Mantener ambas claves activas temporalmente
```

### 9.2. Evolución actual
```
La implementación final muestra:
1. Ed25519: Usa OLD para firma (necesario para verificación)
2. X25519: Solo usa NEW para derivación (no necesita OLD)
3. No hay periodo de transición para X25519
4. La rotación es atómica (hard switch)
```

## 10. Recomendación Final

### 10.1. Acción recomendada
**ELIMINAR** el parámetro `old_x25519_pub_key_hex` completamente:

✅ **Seguro**: No se usa en ninguna parte
✅ **Probado**: Tests no dependen de él
✅ **Limpio**: Elimina código vestigial
✅ **Mantenible**: Reduce complejidad innecesaria

### 10.2. Pasos de implementación
1. Eliminar parámetro de firma en `period_2_3.rs`
2. Eliminar argumento del call site en `mod.rs`
3. Actualizar documentación (docstrings)
4. Compilar y verificar warnings desaparecen
5. Ejecutar suite completa de tests
6. Commit con mensaje descriptivo

### 10.3. Riesgos
**NINGUNO**: El parámetro tiene prefijo `_` (marker de "intencionalmente no usado") y no hay ninguna referencia en el cuerpo de la función.

---

**Fecha de análisis**: 2025-10-26
**Analista**: Claude Code
**Issue**: hashrand-24
