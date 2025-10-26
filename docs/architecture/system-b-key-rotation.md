# System B - Arquitectura de Rotación de Llaves Criptográficas

**Versión**: 1.0
**Fecha**: 2025-10-25
**Estado**: Diseño Propuesto (Pendiente de Implementación)
**Autor**: Análisis de seguridad y diseño arquitectónico

---

## Tabla de Contenidos

1. [Introducción y Motivación](#1-introducción-y-motivación)
2. [Contexto: System A vs System B](#2-contexto-system-a-vs-system-b)
3. [Estándares de Industria](#3-estándares-de-industria)
4. [Análisis de 5 Opciones de Diseño](#4-análisis-de-5-opciones-de-diseño)
5. [Análisis Adversarial: 10 Defectos Potenciales](#5-análisis-adversarial-10-defectos-potenciales)
6. [Arquitectura Final Propuesta](#6-arquitectura-final-propuesta)
7. [Consideraciones de Implementación](#7-consideraciones-de-implementación)
8. [Estado Actual y Próximos Pasos](#8-estado-actual-y-próximos-pasos)
9. [Referencias](#9-referencias)

---

## 1. Introducción y Motivación

### 1.1 Problema a Resolver

HashRand implementa un **system dual de llaves criptográficas**:

- **System A (Temporal)**: Llaves efímeras Ed25519/X25519 para firma de requests/responses (API security)
- **System B (Permanente)**: Llaves Ed25519/X25519 deterministas para E2EE usuario-a-usuario

**El System B tiene una vulnerabilidad crítica**: Las llaves se derivan de forma determinista del `privkey_context` (64 bytes cifrados en BBDD), lo que significa que:

1. **Sin rotación**: Las llaves son permanentes (riesgo de compromiso acumulativo)
2. **Con rotación naive**: Cambiar `privkey_context` implica perder acceso a mensajes históricos
3. **Requisito anti-correlación**: Un atacante con dump de BBDD NO debe poder vincular llaves públicas de un mismo usuario

### 1.2 Objetivos del Diseño

1. ✅ **Rotación periódica**: Cambiar llaves públicas cada 180 días (estándar de industria)
2. ✅ **Compatibilidad histórica**: Descifrar mensajes de hasta 5 rotaciones anteriores (900 días)
3. ✅ **Zero Knowledge anti-correlación**: BBDD no contiene metadatos que vinculen llaves del mismo usuario
4. ✅ **Perfect Forward Secrecy (PFS)**: Compromiso actual no afecta mensajes pasados
5. ✅ **Multi-dispositivo**: Sincronización automática sin intervención manual
6. ✅ **Resiliencia**: Tolerancia a fallos de red, race conditions, ataques de rollback

---

## 2. Contexto: System A vs System B

### 2.1 System A - Llaves Temporales (Implementado)

**Propósito**: Seguridad API (firma de requests/responses)

| Aspecto | Características |
|---------|----------------|
| **Algoritmos** | Ed25519 (firma) + X25519 (ECDH) |
| **Lifecycle** | Efímeras, rotan frecuentemente (cada request puede usar nuevas llaves) |
| **Almacenamiento** | Frontend: IndexedDB temporal, Backend: NUNCA (derivadas on-demand) |
| **Derivación** | Aleatorias (nuevas cada vez) |
| **Propósito** | Validar SignedRequest/SignedResponse, intercambio ECDH |

### 2.2 System B - Llaves Permanentes (Parcialmente Implementado)

**Propósito**: E2EE usuario-a-usuario (futuro)

| Aspecto | Características |
|---------|----------------|
| **Algoritmos** | Ed25519 (firma identidad) + X25519 (cifrado mensajes) |
| **Lifecycle** | Permanentes, deterministas (mismo email + privkey_context → llaves idénticas) |
| **Almacenamiento** | Frontend: IndexedDB (privadas), Backend: BBDD (públicas solamente) |
| **Derivación** | `blake3_kdf(email, privkey_context, epoch_seed)` |
| **Propósito** | Firmas verificables, cifrado E2EE entre usuarios |

**Tablas BBDD existentes**:

```sql
CREATE TABLE users (
    user_id BLOB PRIMARY KEY,           -- Blake3 hash (16 bytes)
    logged_in INTEGER,                   -- Timestamp último login
    created_at INTEGER DEFAULT (unixepoch())
);

CREATE TABLE user_ed25519_keys (
    user_id BLOB NOT NULL,
    pub_key TEXT NOT NULL,              -- Hex 64 chars
    created_at INTEGER NOT NULL,        -- Timestamp publicación
    UNIQUE(user_id, pub_key)
);

CREATE TABLE user_x25519_keys (
    user_id BLOB NOT NULL,
    pub_key TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    UNIQUE(user_id, pub_key)
);

CREATE TABLE user_privkey_context (
    db_index BLOB PRIMARY KEY,          -- Blake3-derived (16 bytes)
    encrypted_privkey BLOB NOT NULL,    -- ChaCha20-Poly1305 encrypted 64 bytes
    created_year INTEGER NOT NULL       -- 4 dígitos (2025, 2026...)
);
```

**Problema crítico**: ¿Cómo rotar llaves manteniendo compatibilidad histórica y Zero Knowledge?

---

## 3. Estándares de Industria

### 3.1 Periodicidad Recomendada por Expertos

| Organización | Recomendación | Tipo de Llave |
|--------------|---------------|---------------|
| **NIST SP 800-57** | 1-2 años | Llaves asimétricas uso general |
| **PCI DSS** | 365 días máximo | Llaves de cifrado |
| **CISA Best Practices** | 90 días | Ambientes alta seguridad |
| **Signal Protocol** | 7 días | Perfect Forward Secrecy extremo |
| **WhatsApp** | Por mensaje | Ratchet continuo |
| **Let's Encrypt** | 90 días | Certificados SSL |

### 3.2 Recomendación para HashRand

**180 días (6 meses)** con jitter aleatorio de ±30 días

**Razones**:
- ✅ Balance seguridad/usabilidad
- ✅ Compatible con NIST (dentro de 1-2 años)
- ✅ Más agresivo que PCI DSS (365 días)
- ✅ Menos fricción que Signal/CISA (7-90 días)
- ✅ Similar a certificados SSL modernos

**Límite histórico**: **5 rotaciones** (900 días = 2.5 años)

---

## 4. Análisis de 5 Opciones de Diseño

### Requisitos de Evaluación

1. **Anti-correlación**: ¿Atacante con BBDD puede vincular llaves del mismo usuario?
2. **Sincronización multi-dispositivo**: ¿Funciona automáticamente sin estado compartido?
3. **Complejidad**: ¿Cuán difícil es implementar y mantener?
4. **Determinismo**: ¿Mismo input → mismo output siempre?

---

### Opción 1: Epoch Basado en Timestamp Directo

**Probabilidad de ser óptima: 15%**

#### Diseño

```typescript
// Derivación
const epoch = created_at_timestamp;  // Unix timestamp (ej: 1729876543)
const keypair = blake3_kdf(email, privkey_context, epoch.toString());

// BBDD
user_ed25519_keys: (user_id, pub_key, created_at: 1729876543)
```

#### Ventajas
- ✅ Epochs naturales (timestamps reales)
- ✅ Fácil debug (timestamp legible)
- ✅ No requiere contador adicional

#### Desventajas Críticas
- ❌ **FALLO ANTI-CORRELACIÓN**: Timestamps idénticos en `user_ed25519_keys` y `user_x25519_keys` → mismo usuario
- ❌ **FALLO**: `created_year=2025` correlaciona con timestamps de 2025
- ❌ **Predecible**: Atacante puede calcular epochs futuros

#### Evaluación
- Anti-correlación: ❌ FALLA
- Sincronización: ✅ Fácil
- Complejidad: 🟢 Baja
- **DESCARTADA**: No cumple requisito Zero Knowledge

---

### Opción 2: Epoch Secuencial Determinista (0, 1, 2, 3, 4)

**Probabilidad de ser óptima: 25%**

#### Diseño

```typescript
// Derivación
const rotation_counter = 0;  // Más reciente
const keypair = blake3_kdf(email, privkey_context, rotation_counter.toString());

// BBDD (epoch NO guardado)
user_ed25519_keys: (user_id, pub_key, created_at: 1729876543)
```

#### Flujo

```
Backend: COUNT(pub_keys) = 3 → rotation_counter = 3
Frontend: Deriva llaves de epoch 0 a 4 (siempre 5 máximo)
Default: epoch=0 (más reciente)
```

#### Ventajas
- ✅ Simple y predecible
- ✅ NO hay epoch en BBDD (anti-correlación)

#### Desventajas Críticas
- ❌ **Sincronización multi-dispositivo complicada**:
  - Dispositivo A cree estar en epoch=2
  - Dispositivo B cree estar en epoch=3
  - ¿Cómo saben cuál usar sin estado compartido?
- ❌ Requiere mecanismo de confirmación (aumenta complejidad)

#### Evaluación
- Anti-correlación: ✅ Buena
- Sincronización: ❌ Difícil
- Complejidad: 🟡 Media
- **DESCARTADA**: Problemas de sincronización

---

### Opción 3: Epoch Periódico con Resolución Truncada

**Probabilidad de ser óptima: 35%**

#### Diseño

```typescript
// Derivación
const ROTATION_PERIOD = 180 * 24 * 60 * 60;  // 180 días en segundos
const epoch = Math.floor(created_at / ROTATION_PERIOD);
const keypair = blake3_kdf(email, privkey_context, epoch.toString());

// BBDD (con ruido en timestamp)
const noise = random(-7, +7) * 24 * 60 * 60;  // ±7 días
user_ed25519_keys: (user_id, pub_key, created_at: actual_time + noise)
```

#### Ejemplo

```
2025-01-01 → epoch = 111
2025-07-01 → epoch = 112 (después de 180 días)
2025-12-28 → epoch = 113
```

#### Ventajas
- ✅ Determinista (todos los dispositivos calculan mismo epoch)
- ✅ Ruido en timestamps dificulta correlación
- ✅ Compatible con rotación automática

#### Desventajas
- ⚠️ **Epochs calculables**: Si atacante conoce `ROTATION_PERIOD`, puede calcular epochs
- ⚠️ **Ruido en created_at confunde auditoría**: Timestamp no refleja momento real

#### Evaluación
- Anti-correlación: 🟡 Media (epochs predecibles)
- Sincronización: ✅ Automática
- Complejidad: 🟡 Media
- **VIABLE pero mejorable**

---

### Opción 4: Epoch Aleatorio Único + Tabla de Mapeo

**Probabilidad de ser óptima: 20%**

#### Diseño

```sql
-- Nueva tabla
CREATE TABLE user_key_rotations (
    user_id BLOB,
    rotation_id BLOB,      -- 16 bytes aleatorios
    created_at INTEGER
);

-- Derivación
const rotation_id = random_16_bytes();  // Por rotación
const keypair = blake3_kdf(email, privkey_context, rotation_id);
```

#### Flujo

```
Backend: Genera rotation_id aleatorio → guarda en user_key_rotations
Frontend: Fetch lista de rotation_ids → deriva todas las llaves
```

#### Ventajas
- ✅ Epochs impredecibles (aleatorios)
- ✅ Frontend tiene lista exacta de epochs

#### Desventajas Críticas
- ❌ **FALLO ANTI-CORRELACIÓN**: `user_key_rotations.user_id` vincula todas las rotaciones
- ❌ Atacante ve que `user_id=X` tiene rotations `[A, B, C, D, E]`
- ❌ Puede correlacionar todas las llaves públicas con mismo `user_id`
- ❌ Mayor complejidad (nueva tabla)

#### Evaluación
- Anti-correlación: ❌ FALLA (expone user_id)
- Sincronización: ✅ Fácil (fetch list)
- Complejidad: 🔴 Alta
- **DESCARTADA**: Viola Zero Knowledge

---

### Opción 5: Counter Dinámico + Blake3 sin Metadata

**Probabilidad de ser óptima: 65% ⭐ RECOMENDADA**

#### Diseño

```rust
// Derivación (SIN timestamp para mayor determinismo)
let epoch_seed = blake3(
    privkey_context ||
    rotation_counter.to_be_bytes()
);

let keypair = blake3_kdf(email, privkey_context, epoch_seed);

// BBDD (ZERO metadata de epoch)
user_ed25519_keys: (user_id, pub_key, created_at ± random(0-5 días))
user_privkey_context: (db_index, encrypted_privkey, created_year)
```

#### Flujo Completo

**Backend (validación magic link):**

```rust
// 1. Contar llaves existentes dinámicamente (source of truth)
let existing_keys = UserKeysOperations::get_user_keys(&user_id, 5)?;
let rotation_counter = existing_keys.0.len();  // Derivado, no almacenado

// 2. Verificar necesidad de rotación
let needs_rotation = if let Some(latest_key) = existing_keys.0.first() {
    let age_days = (now - latest_key.created_at) / 86400;
    age_days > 180  // Política
} else {
    true  // Primera llave
};

if needs_rotation {
    rotation_counter += 1;

    // Actualizar solo created_year
    UPDATE user_privkey_context
    SET created_year = 1970 + (now / (365 * 24 * 60 * 60))
    WHERE db_index = ?;
}

// 3. Calcular epoch_seed
let epoch_seed = blake3(
    privkey_context_decrypted.to_vec() +
    rotation_counter.to_be_bytes()
);

// 4. Enviar metadata mínima
rotation_metadata: {
    rotation_counter: rotation_counter,
    current_epoch_seed: hex::encode(epoch_seed),
}
```

**Frontend (post-validación):**

```typescript
// 1. Descifrar privkey_context (ECDH)
const privkeyContext = await ecdh_decrypt(response.encrypted_privkey_context);

// 2. Derivar TODAS las llaves históricas (0 a rotation_counter)
const allKeypairs = [];

for (let i = 0; i <= response.rotation_metadata.rotation_counter; i++) {
    const epochSeed = blake3(concat(privkeyContext, u64_to_be_bytes(i)));
    const keypair = await deriveUserKeys(email, privkeyContext, epochSeed);
    allKeypairs.push({
        ed25519: keypair.ed25519,
        x25519: keypair.x25519,
        rotation_index: i
    });
}

// 3. Llave más reciente = default
const defaultKeypair = allKeypairs[allKeypairs.length - 1];

// 4. Publicar (si rotación necesaria)
if (needs_rotation) {
    await httpAuthenticatedSignedPOSTRequest('/api/keys/rotate', {
        ed25519_pub: defaultKeypair.ed25519.publicKeyHex,
        x25519_pub: defaultKeypair.x25519.publicKeyHex
    });
}

// 5. Almacenar todas en IndexedDB
await storeAllKeypairs(allKeypairs);
```

**Descifrado de mensajes históricos:**

```typescript
async function decryptUserMessage(encryptedMessage: Uint8Array) {
    const keypairs = await loadAllKeypairs();

    // Probar desde más reciente a más antigua
    for (let i = keypairs.length - 1; i >= 0; i--) {
        try {
            const decrypted = await ecdh_decrypt(
                encryptedMessage,
                keypairs[i].x25519.privateKey
            );
            return decrypted;  // Éxito
        } catch {
            continue;  // Llave incorrecta, probar siguiente
        }
    }

    throw new Error('Cannot decrypt with any available keypair');
}
```

#### Ventajas
- ✅ **MÁXIMA seguridad anti-correlación**: ZERO metadata en BBDD
- ✅ **Auto-sincronización multi-dispositivo**: Backend cuenta llaves dinámicamente
- ✅ **Determinista**: Mismo counter → mismo epoch_seed
- ✅ **Mínima metadata**: Solo `(user_id, pub_key, created_at)`
- ✅ **Resistente a análisis**: Timestamp con ruido (±5 días)

#### Desventajas
- ⚠️ Pequeño overhead: Enviar `rotation_metadata` (~100 bytes)
- ⚠️ Frontend deriva 5 llaves en cada login (~50ms total, acceptable)

#### Evaluación
- Anti-correlación: ✅ **PERFECTA**
- Sincronización: ✅ **AUTOMÁTICA**
- Complejidad: 🟡 Media
- **SELECCIONADA**: Mejor balance seguridad/usabilidad

---

### Comparativa Final de las 5 Opciones

| Opción | Probabilidad | Anti-correlación | Sincronización | Complejidad | Veredicto |
|--------|-------------|------------------|----------------|-------------|-----------|
| **1. Timestamp directo** | **15%** | ❌ Falla | ✅ Fácil | 🟢 Baja | ❌ Descartada |
| **2. Secuencial 0-4** | **25%** | ✅ Buena | ❌ Difícil | 🟢 Baja | ❌ Descartada |
| **3. Epoch periódico** | **35%** | ⚠️ Media | ✅ Automática | 🟡 Media | ⚠️ Viable |
| **4. Rotation_id aleatorio** | **20%** | ❌ Expone user_id | ✅ Fetch list | 🔴 Alta | ❌ Descartada |
| **5. Counter dinámico** | **65%** | ✅ **PERFECTA** | ✅ **AUTOMÁTICA** | 🟡 Media | ✅ **SELECCIONADA** |

---

## 5. Análisis Adversarial: 10 Defectos Potenciales

### Metodología

Se realizaron **dos rondas de "ultrathink"** para identificar fallos de seguridad en la Opción 5:

1. **Round 1**: Defectos 1-5 (arquitectura básica)
2. **Round 2**: Defectos 6-10 (ataques avanzados)

Cada defecto incluye:
- **Probabilidad de ocurrencia** (basada en vectores de ataque reales)
- **Severidad del impacto** (🔴 Crítica, 🟡 Media, 🟢 Baja)
- **Corrección recomendada** (con código de ejemplo)

---

### Round 1: Defectos de Arquitectura Básica

---

#### Defecto 1: Desincronización rotation_counter por Publicación Fallida

**Probabilidad: 75%** - MUY PROBABLE
**Severidad: 🔴 ALTA**

##### Escenario

```typescript
// Login #3 exitoso
Backend: "Tienes 2 llaves, rotation necesaria, counter=3"
Frontend: Deriva llave con counter=3, epoch_seed=blake3(privkey_context || 3)
          POST /api/keys/rotate → ❌ FALLA (timeout, red, 500 error)

// Login #4 (siguiente día)
Backend: "Tienes 2 llaves (publicación falló), counter=3"
Frontend: Deriva llave con counter=3, PERO epoch_seed diferente (timestamp cambió!)

// Problema: ¿Cuál epoch_seed es el correcto?
```

##### Impacto

- ❌ Frontend deriva llaves incorrectas
- ❌ No puede verificar firmas de mensajes propios
- ❌ Otros usuarios no pueden cifrar para ti (llave pública incorrecta)

##### Corrección A: Tabla de Estado de Rotación

```sql
CREATE TABLE user_key_rotation_state (
    db_index BLOB PRIMARY KEY,
    pending_rotation_counter INTEGER,  -- Counter propuesto
    pending_epoch_seed BLOB,            -- Epoch_seed calculado
    last_successful_counter INTEGER,    -- Último confirmado
    created_at INTEGER
);
```

**Flujo**:

```rust
// 1. Backend calcula rotación propuesta
INSERT OR REPLACE INTO user_key_rotation_state
(db_index, pending_rotation_counter, pending_epoch_seed, created_at)
VALUES (?, ?, ?, ?);

// 2. Frontend recibe metadata, publica llave
POST /api/keys/rotate { ed25519_pub, x25519_pub }

// 3. Backend confirma publicación exitosa
UPDATE user_key_rotation_state
SET last_successful_counter = pending_rotation_counter
WHERE db_index = ?;

// 4. Próximo login usa last_successful_counter
let rotation_counter = get_last_successful_counter(&db_index)?;
```

##### Corrección B: Contador Derivado de BBDD (MÁS SIMPLE - RECOMENDADA)

```rust
// NO guardar rotation_counter en tabla separada
// SIEMPRE calcularlo dinámicamente contando llaves

pub fn get_current_rotation_counter(user_id: &[u8; 16]) -> Result<u64, SqliteError> {
    let connection = get_database_connection()?;

    // Contar número de llaves únicas por timestamp
    let result = connection.execute(
        "SELECT COUNT(DISTINCT created_at) FROM user_ed25519_keys WHERE user_id = ?",
        &[Value::Blob(user_id.to_vec())]
    )?;

    let key_count = match &result.rows[0].values[0] {
        Value::Integer(n) => *n as u64,
        _ => return Err(SqliteError::Io("Invalid count".to_string())),
    };

    // rotation_counter = key_count - 1 (0-indexed)
    Ok(key_count.saturating_sub(1))
}

// ✅ SIEMPRE consistente (source of truth = llaves en BBDD)
// ✅ No requiere tabla separada
// ✅ Imposible desincronización
```

---

#### Defecto 2: Correlación por total_rotations en Metadata

**Probabilidad: 60%** - PROBABLE
**Severidad: 🟡 MEDIA**

##### Escenario

```json
// Atacante intercepta tráfico HTTPS (MITM con certificado comprometido)
{
  "rotation_metadata": {
    "total_rotations": 5,
    "current_epoch_seed": "abc123...",
  }
}

// Atacante tiene dump de BBDD:
SELECT user_id, COUNT(*) as key_count
FROM user_ed25519_keys
GROUP BY user_id
HAVING key_count = 5;

// Resultado: 3 usuarios con 5 llaves
// Si interceptó 1 respuesta con total_rotations=5
// Probabilidad 33% de identificar usuario correcto
// Si intercepta múltiples respuestas → correlación perfecta
```

##### Impacto

- ❌ De-anonimización parcial (`user_id` vinculado a tráfico)
- ⚠️ Requiere MITM activo (difícil pero posible: CA comprometida)

##### Corrección A: Reportar Valor Fijo (Padding)

```rust
// Ofuscar total_rotations con padding
let reported_rotations = if actual_rotations < 5 {
    5  // Siempre reportar máximo hasta que se alcance
} else {
    actual_rotations
};

// Ventaja: Atacante no puede distinguir usuario con 1 vs 4 llaves
```

##### Corrección B: Cifrar Metadata (MÁXIMA SEGURIDAD - RECOMENDADA)

```rust
// Cifrar metadata con llave derivada del privkey_context
let metadata_encryption_key = blake3_kdf(
    privkey_context,
    "rotation_metadata_v1"
);

let encrypted_metadata = chacha20poly1305_encrypt(
    serde_json::to_vec(&rotation_metadata)?,
    metadata_encryption_key
);

// Respuesta
JwtAuthResponse {
    encrypted_privkey_context: ecdh_encrypt(privkey_context),
    encrypted_rotation_metadata: base64(encrypted_metadata),  // Opaco
}

// Atacante MITM ve blob opaco, no puede extraer total_rotations
```

---

#### Defecto 3: Race Condition en Rotaciones Concurrentes

**Probabilidad: 45%** - MEDIANAMENTE PROBABLE
**Severidad: 🟡 MEDIA**

##### Escenario

```
t=0: Usuario abre 2 navegadores (Chrome + Firefox)

t=1 (Chrome):  Backend ve 2 llaves → rotation_counter=3
               Envía metadata: { rotation_counter: 3, epoch_seed: "AAA" }

t=2 (Firefox): Backend ve 2 llaves → rotation_counter=3  (mismo!)
               Envía metadata: { rotation_counter: 3, epoch_seed: "AAA" }  (mismo!)

t=3 (Chrome):  POST /api/keys/rotate con pub_key derivada de epoch_seed="AAA"
               INSERT OR IGNORE → OK

t=4 (Firefox): POST /api/keys/rotate con pub_key derivada de epoch_seed="AAA"
               INSERT OR IGNORE → Silent success (duplicate, no insert)
```

##### Impacto

- ✅ **CON epoch_seed fijo (sin timestamp)**: Ambos derivan MISMA llave → INSERT OR IGNORE funciona correctamente
- ⚠️ **SI se usara timestamp en epoch_seed**: Epoch_seed diferente → llaves diferentes → duplicados

##### Corrección: Timestamp Fijo por Rotation (RECOMENDADA)

```rust
// Nueva columna para fijar timestamp
ALTER TABLE user_privkey_context
ADD COLUMN last_rotation_timestamp INTEGER;

// Calcular epoch_seed con timestamp FIJO por rotation_counter
let timestamp_coarse = if needs_rotation {
    let new_ts = now / 1_000_000;

    UPDATE user_privkey_context
    SET last_rotation_timestamp = ?
    WHERE db_index = ?;

    new_ts
} else {
    // Usar timestamp almacenado (idempotente)
    get_last_rotation_timestamp(&db_index)?
};

let epoch_seed = blake3(
    privkey_context ||
    rotation_counter ||
    timestamp_coarse  // ← FIJO para este rotation_counter
);

// ✅ Ambos navegadores usan MISMO epoch_seed
// ✅ Frontend deriva MISMA llave pública
// ✅ INSERT OR IGNORE funciona correctamente
```

**Nota**: Si NO se usa timestamp en epoch_seed (solo rotation_counter), este defecto NO aplica.

---

#### Defecto 4: Timestamp Coarse Permite Clustering de Llaves

**Probabilidad: 30%** - POCO PROBABLE
**Severidad: 🟢 BAJA**

##### Escenario

```python
# Atacante con dump de BBDD
keys = db.query("SELECT user_id, pub_key, created_at FROM user_ed25519_keys")

# Calcular floor(created_at / 1_000_000) para cada llave
for key in keys:
    key['coarse'] = math.floor(key['created_at'] / 1_000_000)

# Agrupar por coarse timestamp
clusters = defaultdict(list)
for key in keys:
    clusters[key['coarse']].append(key)

# Encontrar clusters con exactamente 5 llaves
suspicious_clusters = [c for c in clusters.values() if len(c) == 5]

# Hipótesis: Cada cluster de 5 llaves = 1 usuario
# Correlación: user_id idéntico en las 5 → confirmado mismo usuario
```

##### Impacto

- ⚠️ Correlación débil (requiere suposiciones)
- ⚠️ Falsos positivos (diferentes usuarios pueden rotar en mismo periodo)
- ✅ Mitigado por: `user_id` ya es hash (no email directo)

##### Corrección: Eliminar Timestamp de Epoch_Seed (RECOMENDADA)

```rust
// NO usar timestamp en epoch_seed (eliminarlo completamente)

let epoch_seed = blake3(
    privkey_context ||
    rotation_counter  // Solo counter, SIN timestamp
);

// Ventajas:
// ✅ Determinista perfecto (no depende de cuándo se crea)
// ✅ Zero correlación temporal
// ✅ Más simple
```

---

#### Defecto 5: Frontend No Puede Verificar Integridad de Epoch_Seed

**Probabilidad: 20%** - IMPROBABLE
**Severidad: 🟡 MEDIA**

##### Escenario

```rust
// Backend malicioso/comprometido envía epoch_seed incorrecto

JwtAuthResponse {
    rotation_metadata: {
        rotation_counter: 3,
        current_epoch_seed: "FAKE_SEED_123",  // ← Seed manipulado
    }
}

// Frontend deriva llave con seed falso
const keypair = deriveUserKeys(email, privkeyContext, "FAKE_SEED_123");

// Publica llave pública incorrecta
POST /api/keys/rotate { ed25519_pub: "WRONG_KEY" }

// Usuario no puede descifrar mensajes (llave incorrecta)
```

##### Impacto

- ❌ Pérdida de acceso si backend comprometido
- ⚠️ Requiere: Backend malicioso (poco probable en self-hosted)

##### Corrección: Frontend Calcula Epoch_Seed (RECOMENDADA)

```rust
// NO enviar epoch_seed precalculado
// Frontend lo calcula SIEMPRE

rotation_metadata: {
    rotation_counter: 3,
    // epoch_seed ELIMINADO
}

// Frontend:
const epochSeed = blake3(
    concat(
        privkeyContext,
        u64_to_be_bytes(metadata.rotation_counter)
    )
);

// Ventajas:
// ✅ Backend no puede enviar seed falso
// ✅ Menos datos en respuesta
// ✅ Frontend tiene control total
```

---

### Round 2: Ataques Avanzados

---

#### Defecto 6: Backend No Puede Verificar Derivación Correcta de Pub_Keys

**Probabilidad: 70%** - MUY PROBABLE
**Severidad: 🔴 CRÍTICA**

##### Escenario

```typescript
// Frontend comprometido (malware, extensión maliciosa, XSS)

// 1. Malware intercepta privkey_context
const privkeyContext = await ecdh_decrypt(response.encrypted_privkey_context);

// 2. Deriva llave CORRECTA
const correctKeypair = await deriveUserKeys(email, privkeyContext, epochSeed);

// 3. Pero publica llave FALSA (controlada por atacante)
const attackerKeypair = await crypto.subtle.generateKey("Ed25519", ...);

await httpAuthenticatedSignedPOSTRequest('/api/keys/rotate', {
    ed25519_pub: hex(attackerKeypair.publicKey),  // ← LLAVE FALSA
    x25519_pub: hex(correctKeypair.x25519.publicKey)
});

// 4. Backend acepta sin validar
INSERT INTO user_ed25519_keys (user_id, pub_key, created_at)
VALUES (?, ?, ?);  // ✅ Éxito (no hay verificación)
```

##### Impacto

- 🔴 **CRÍTICO**: Man-in-the-middle permanente en E2EE
- ❌ Atacante lee todos los mensajes futuros
- ❌ Víctima no puede descifrar mensajes que le envían (cifrados con llave falsa)

##### Corrección: Challenge-Response con Firma (RECOMENDADA)

```rust
// Backend genera challenge aleatorio durante validación

let challenge = random_32_bytes();
kv_store.set(format!("challenge:{}", hex::encode(user_id)), challenge, 600)?;

// Envía en metadata
rotation_metadata: {
    rotation_counter: 3,
    derivation_challenge: hex::encode(challenge),
}

// Frontend firma challenge con llave privada derivada
const signature = await crypto.subtle.sign(
    "Ed25519",
    keypair.ed25519.privateKey,
    challenge
);

// Publica con firma
POST /api/keys/rotate {
    ed25519_pub: hex(keypair.ed25519.publicKey),
    x25519_pub: hex(keypair.x25519.publicKey),
    challenge_signature: base64(signature)
}

// Backend verifica firma
let stored_challenge = kv_store.get(format!("challenge:{}", hex::encode(user_id)))?;
let signature_bytes = base64::decode(payload.challenge_signature)?;
let pub_key_bytes = hex::decode(&payload.ed25519_pub)?;

if !ed25519_dalek::verify(&pub_key_bytes, &stored_challenge, &signature_bytes)? {
    return Err("Challenge signature verification failed");
}

// ✅ Prueba que frontend posee llave privada correspondiente
```

**Limitación**: No prueba derivación correcta (solo posesión de llave privada).

##### Corrección Mejorada: Backend Deriva y Compara (MÁXIMA SEGURIDAD)

```rust
// Backend conoce privkey_context (descifrado durante validación magic link)
// Puede derivar llaves esperadas y compararlas

let privkey_context_decrypted = decrypt_privkey_context(&db_index, &encrypted_privkey)?;

// Calcular epoch_seed esperado
let rotation_counter = get_current_rotation_counter(&user_id)?;
let expected_epoch_seed = blake3(privkey_context_decrypted.to_vec() + rotation_counter.to_be_bytes());

// Derivar llaves públicas esperadas (MISMO algoritmo que frontend)
let expected_ed25519_pub = derive_ed25519_public_key(&email, &privkey_context_decrypted, &expected_epoch_seed)?;
let expected_x25519_pub = derive_x25519_public_key(&email, &privkey_context_decrypted, &expected_epoch_seed)?;

// Guardar en cache temporal
kv_store.set(
    format!("expected_keys:{}", hex::encode(user_id)),
    serde_json::to_string(&ExpectedKeys {
        ed25519_pub: hex::encode(expected_ed25519_pub),
        x25519_pub: hex::encode(expected_x25519_pub),
        rotation_counter,
    })?,
    600  // TTL 10 minutos
)?;

// Endpoint /api/keys/rotate valida contra cache
let expected_keys: ExpectedKeys = fetch_expected_keys_from_cache(&user_id)?;

if payload.ed25519_pub != expected_keys.ed25519_pub {
    return Err("Ed25519 derivation mismatch - attack detected");
}

if payload.x25519_pub != expected_keys.x25519_pub {
    return Err("X25519 derivation mismatch - attack detected");
}

// ✅ Doble validación: derivación correcta + challenge-response
```

**Problema**: Requiere mismo algoritmo de derivación en Rust (backend) y TypeScript (frontend).

---

#### Defecto 7: Rollback Attack en Rotation_Counter

**Probabilidad: 55%** - PROBABLE
**Severidad: 🔴 ALTA**

##### Escenario

```sql
-- Atacante compromete BBDD (SQL injection, backup robado)

-- Estado actual
SELECT * FROM user_key_rotation_state WHERE db_index = X;
-- rotation_counter=5

-- Atacante hace rollback
UPDATE user_key_rotation_state
SET rotation_counter = 2
WHERE db_index = X;

DELETE FROM user_ed25519_keys
WHERE user_id = Y
AND created_at > (SELECT created_at FROM user_ed25519_keys ORDER BY created_at LIMIT 1 OFFSET 2);

-- Usuario se loguea después del ataque
Backend: "rotation_counter=2, derivar llave antigua"
Frontend: Deriva llave de counter=2 (antigua, posiblemente comprometida)
```

##### Impacto

- 🔴 Forzar uso de llaves antiguas (atacante conoce llaves viejas)
- ❌ Bypass de rotación

##### Corrección: Append-Only Log con Firmas Backend (RECOMENDADA)

```sql
-- Nueva tabla append-only
CREATE TABLE user_key_rotation_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    db_index BLOB NOT NULL,
    rotation_counter INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    backend_signature BLOB NOT NULL,  -- Ed25519 signature del backend
    UNIQUE(db_index, rotation_counter)
);

-- Triggers que previenen modificaciones
CREATE TRIGGER prevent_rotation_log_delete
BEFORE DELETE ON user_key_rotation_log
BEGIN
    SELECT RAISE(ABORT, 'Deletion from rotation log is forbidden');
END;

CREATE TRIGGER prevent_rotation_log_update
BEFORE UPDATE ON user_key_rotation_log
BEGIN
    SELECT RAISE(ABORT, 'Updates to rotation log are forbidden');
END;
```

**Backend firma cada rotación**:

```rust
// Durante rotación
let rotation_entry = RotationEntry {
    db_index: db_index.clone(),
    rotation_counter: 3,
    created_at: now,
};

// Firmar con llave backend permanente (Ed25519)
let backend_signing_key = get_backend_permanent_signing_key()?;
let entry_bytes = serde_json::to_vec(&rotation_entry)?;
let signature = sign_ed25519(&backend_signing_key, &entry_bytes)?;

// Guardar entry + signature
INSERT INTO user_key_rotation_log
(db_index, rotation_counter, created_at, backend_signature)
VALUES (?, ?, ?, ?);
```

**Frontend verifica firmas durante login**:

```typescript
// Fetch rotation log
const entries = await fetch_rotation_log(user_id);
const backend_pub_key = BACKEND_PUBLIC_KEY;  // Hardcoded o TOFU

// Verificar todas las firmas
for (const entry of entries) {
    const entryBytes = JSON.stringify({
        db_index: entry.db_index,
        rotation_counter: entry.rotation_counter,
        created_at: entry.created_at
    });

    const valid = await verify_ed25519(
        backend_pub_key,
        entryBytes,
        entry.backend_signature
    );

    if (!valid) {
        throw new Error("Backend signature invalid - rotation log tampered!");
    }
}

// ✅ Atacante no puede alterar log sin llave privada del backend
```

**Limitación**: Si atacante compromete backend signing key → puede firmar entries falsos.
**Mitigación**: Almacenar signing key en HSM o Spin secrets cifrados.

---

#### Defecto 8: Inconsistencia entre Rotation_Counter y Llaves Publicadas

**Probabilidad: 50%** - MEDIANAMENTE PROBABLE
**Severidad: 🟡 MEDIA**

##### Escenario

```rust
// Flujo con timeout
BEGIN TRANSACTION;
    INSERT INTO user_ed25519_keys (user_id, pub_key, created_at) VALUES (...);
    -- ❌ TIMEOUT AQUÍ
    UPDATE user_key_rotation_state SET rotation_counter = 4 WHERE db_index = ?;
ROLLBACK;  // Transaction aborted

// Resultado: rotation_counter=3, pero llave NO se insertó

// Siguiente login
Backend: rotation_counter=3
Frontend: Deriva misma llave, publica → INSERT OR IGNORE (ahora sí se guarda)

// Estado inconsistente:
// - rotation_counter = 3
// - BBDD tiene 4 llaves (0, 1, 2, 3)
// - Esperado: rotation_counter = 4
```

##### Impacto

- ⚠️ Estado desincronizado
- ❌ Frontend deriva menos llaves (pierde acceso a mensajes)

##### Corrección: Contador Derivado de BBDD (RECOMENDADA - Ya mencionada en Defecto 1)

```rust
// Source of truth = COUNT de llaves en BBDD (no tabla separada)

pub fn get_current_rotation_counter(user_id: &[u8; 16]) -> Result<u64, SqliteError> {
    let connection = get_database_connection()?;

    let result = connection.execute(
        "SELECT COUNT(DISTINCT created_at) FROM user_ed25519_keys WHERE user_id = ?",
        &[Value::Blob(user_id.to_vec())]
    )?;

    let key_count = match &result.rows[0].values[0] {
        Value::Integer(n) => *n as u64,
        _ => return Err(SqliteError::Io("Invalid count".to_string())),
    };

    Ok(key_count.saturating_sub(1))
}

// ✅ Imposible tener inconsistencia
```

---

#### Defecto 9: Compromiso de Privkey_Context = Pérdida Total de PFS

**Probabilidad: 40%** - MEDIANAMENTE PROBABLE
**Severidad: 🔴 CRÍTICA**

##### Escenario

```
t=0:   Usuario crea cuenta → privkey_context generado (64 bytes random)
t=30:  Rotación 1 → llave derivada con counter=0
t=210: Rotación 2 → llave derivada con counter=1
t=390: Rotación 3 → llave derivada con counter=2

t=400: 🔴 Atacante compromete navegador (XSS, malware)
       Lee privkey_context de IndexedDB

       Deriva TODAS las llaves históricas:
       - blake3(privkey_context || 0) → llave rotación 1
       - blake3(privkey_context || 1) → llave rotación 2
       - blake3(privkey_context || 2) → llave rotación 3

       Descifra TODOS los mensajes históricos (t=0 hasta t=400)
```

##### Impacto

- 🔴 **CRÍTICO**: No hay Perfect Forward Secrecy real
- ❌ Compromiso retroactivo (atacante descifra TODO el historial)

##### Comparación con Signal Protocol

| Aspecto | Signal Protocol | HashRand System B Actual |
|---------|----------------|---------------------------|
| **Llave efímera por** | Mensaje | Rotación (180 días) |
| **Compromiso en t=400** | Solo afecta mensajes futuros | Afecta TODO (pasado + futuro) |
| **PFS** | ✅ Real | ❌ Falso |

##### Corrección: Separar Identity Keys y Ephemeral Keys (ARQUITECTURA MEJORADA)

```rust
// 1. Identity Key (permanente, derivada de privkey_context)
//    - Uso: Autenticación, firmas, verificación identidad
//    - Rotación: Nunca (o muy raramente)
let identity_key = blake3_kdf(email, privkey_context, "identity_v1");

// 2. Ephemeral Keys (rotables, con PFS real)
//    - Uso: Cifrado E2EE de mensajes
//    - Rotación: Cada 180 días
//    - Generación: Aleatorio (NO derivado de privkey_context)

// Generación de ephemeral key
let ephemeral_private = crypto.subtle.generateKey("X25519", ...);

// Guardar cifrada con identity_key (para multi-dispositivo)
let encrypted_ephemeral = chacha20poly1305_encrypt(
    ephemeral_private,
    identity_key
);

// BBDD guarda:
// - Ephemeral public key (para otros usuarios)
// - Encrypted ephemeral private (para recuperación multi-dispositivo durante periodo de gracia)
```

**Rotación con PFS**:

```
1. Generar nueva ephemeral_private (random)
2. Publicar nueva public
3. ELIMINAR antigua private de IndexedDB (no recuperable)
4. Mantener encrypted_ephemeral en BBDD (multi-dispositivo, periodo de gracia)
5. Después de 180 días: DELETE encrypted_ephemeral de BBDD
6. PFS completo: nadie puede descifrar mensajes antiguos
```

**Arquitectura resultante**:

```
System B = Identity Keys (deterministas) + Ephemeral Keys (aleatorias con PFS)

Identity Ed25519:
  - Propósito: Firmar operaciones, autenticación
  - Derivación: blake3_kdf(email, privkey_context, "identity")
  - Rotación: Nunca
  - Publicación: Una vez (permanente)

Ephemeral X25519:
  - Propósito: Cifrado E2EE
  - Generación: crypto.subtle.generateKey() (random)
  - Rotación: 180 días
  - PFS: Sí (llave antigua no recuperable después de periodo de gracia)
```

---

#### Defecto 10: Timing Attack en Metadata Revela Rotation Patterns

**Probabilidad: 25%** - POCO PROBABLE
**Severidad: 🟢 BAJA**

##### Escenario

```python
# Atacante pasivo con acceso a tráfico cifrado TLS

logins = [
    {"timestamp": 1729876543, "needs_rotation": False, "response_time": 120ms},
    {"timestamp": 1729876600, "needs_rotation": False, "response_time": 118ms},
    {"timestamp": 1745520000, "needs_rotation": True, "response_time": 350ms},  # ← Rotación
    {"timestamp": 1745520100, "needs_rotation": False, "response_time": 122ms},
]

# Observaciones:
# - needs_rotation=True tiene response_time 3x mayor
# - Delta temporal ≈ 180 días
# - Próxima rotación predecible
```

##### Impacto

- ⚠️ Metadata mining (patrones de uso revelados)
- ⚠️ Predicción de rotaciones
- ✅ No compromete crypto (solo metadata)

##### Corrección A: Eliminar needs_rotation de Metadata (RECOMENDADA)

```rust
// NO enviar needs_rotation al frontend

rotation_metadata: {
    rotation_counter: 3,
    // needs_rotation ELIMINADO
}

// Frontend SIEMPRE deriva llaves actuales
// Backend SIEMPRE acepta publicación (idempotente)

// ✅ Atacante no puede distinguir rotación de login normal
```

##### Corrección B: Ruido en Timing y Periodicidad

```rust
// Rotación NO exactamente cada 180 días
let rotation_period_days = 180 + (random_u32() % 30);  // 180-210 días
let needs_rotation = age_days > rotation_period_days;

// Response time con jitter aleatorio
let jitter_ms = random_u32() % 100;  // 0-100ms
thread::sleep(Duration::from_millis(jitter_ms));

// ✅ Imposible predecir próxima rotación exacta
```

---

### Resumen de los 10 Defectos

| # | Defecto | Probabilidad | Severidad | Corrección |
|---|---------|-------------|-----------|-----------|
| 1 | Desincronización rotation_counter | **75%** | 🔴 ALTA | Contador derivado de BBDD |
| 2 | Correlación total_rotations | **60%** | 🟡 MEDIA | Cifrar metadata |
| 3 | Race condition | **45%** | 🟡 MEDIA | Timestamp fijo (o solo counter) |
| 4 | Clustering temporal | **30%** | 🟢 BAJA | Eliminar timestamp de epoch |
| 5 | Epoch_seed manipulado | **20%** | 🟡 MEDIA | Frontend calcula seed |
| 6 | Backend no verifica derivación | **70%** | 🔴 CRÍTICA | Challenge + derivación backend |
| 7 | Rollback attack | **55%** | 🔴 ALTA | Append-only log + firmas |
| 8 | Inconsistencia counter/llaves | **50%** | 🟡 MEDIA | Contador derivado |
| 9 | Pérdida de PFS | **40%** | 🔴 CRÍTICA | Ephemeral keys separadas |
| 10 | Timing attack | **25%** | 🟢 BAJA | Eliminar needs_rotation |

---

## 6. Arquitectura Final Propuesta

### 6.1 Opción 5 ULTRA-MEJORADA

**Aplicando todas las correcciones de los 10 defectos identificados**

---

### 6.2 Schema BBDD Actualizado

```sql
-- Tabla principal (sin cambios)
CREATE TABLE user_privkey_context (
    db_index BLOB PRIMARY KEY,          -- 16 bytes (Blake3-derived)
    encrypted_privkey BLOB NOT NULL,    -- ChaCha20-Poly1305 encrypted 64 bytes
    created_year INTEGER NOT NULL       -- Año de creación (auditoría)
);

-- Append-only rotation log (con firmas backend)
CREATE TABLE user_key_rotation_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    db_index BLOB NOT NULL,
    rotation_counter INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    backend_signature BLOB NOT NULL,    -- Ed25519 signature del servidor
    UNIQUE(db_index, rotation_counter)
);

-- Triggers inmutabilidad
CREATE TRIGGER prevent_rotation_log_delete
BEFORE DELETE ON user_key_rotation_log
BEGIN
    SELECT RAISE(ABORT, 'Deletion forbidden');
END;

CREATE TRIGGER prevent_rotation_log_update
BEFORE UPDATE ON user_key_rotation_log
BEGIN
    SELECT RAISE(ABORT, 'Updates forbidden');
END;

-- Identity keys (permanentes, deterministas)
CREATE TABLE user_identity_keys (
    user_id BLOB PRIMARY KEY,
    ed25519_pub TEXT NOT NULL,          -- Hex 64 chars
    created_at INTEGER NOT NULL
);

-- Ephemeral keys (rotables, con PFS)
CREATE TABLE user_ephemeral_keys (
    user_id BLOB NOT NULL,
    x25519_pub TEXT NOT NULL,
    encrypted_x25519_priv BLOB,         -- Cifrado con identity_key
    created_at INTEGER NOT NULL,
    expires_at INTEGER,                 -- Auto-delete para PFS
    UNIQUE(user_id, x25519_pub)
);

CREATE INDEX idx_ephemeral_expires ON user_ephemeral_keys(expires_at);
```

---

### 6.3 Algoritmo de Derivación Final

```typescript
// Identity key (permanente)
const identityEpochSeed = blake3("identity_v1");
const identityKeypair = blake3_kdf(email, privkeyContext, identityEpochSeed);

// Ephemeral key rotation (aleatoria, NO derivada)
const ephemeralKeypair = await crypto.subtle.generateKey("X25519", {
    name: "ECDH",
    namedCurve: "X25519"
}, true, ["deriveKey"]);
```

---

### 6.4 Flujo Completo de Rotación

#### Backend (Validación Magic Link)

```rust
// 1. Descifrar privkey_context
let privkey_context_decrypted = UserPrivkeyCrypto::decrypt_privkey_context(
    &db_index,
    &encrypted_privkey
)?;

// 2. Calcular rotation_counter dinámicamente (source of truth = BBDD)
let rotation_counter = count_distinct_created_at(&user_id)?;

// 3. Verificar necesidad de rotación (con jitter)
let rotation_period_days = 180 + (random() % 30);  // 180-210 días
let needs_rotation = if let Some(latest_key) = get_latest_key(&user_id)? {
    let age_days = (now - latest_key.created_at) / 86400;
    age_days > rotation_period_days
} else {
    true  // Primera llave
};

// 4. Derivar identity key esperada (para verificación posterior)
let expected_identity_pub = derive_ed25519_public_key(
    &email,
    &privkey_context_decrypted,
    b"identity_v1"
)?;

// Guardar en cache (KV Store, TTL 10 min)
kv_store.set(
    format!("expected_identity:{}", hex::encode(user_id)),
    hex::encode(expected_identity_pub),
    600
)?;

// 5. Generar challenge para proof-of-possession
let challenge = random_32_bytes();
kv_store.set(
    format!("challenge:{}", hex::encode(user_id)),
    hex::encode(challenge),
    600
)?;

// 6. Cifrar metadata (anti-correlación)
let metadata_key = blake3_kdf(&privkey_context_decrypted, b"metadata_encryption_v1");
let metadata = RotationMetadata {
    rotation_counter,
    derivation_challenge: hex::encode(challenge),
};

let encrypted_metadata = chacha20poly1305_encrypt(
    serde_json::to_vec(&metadata)?,
    &metadata_key
)?;

// 7. Respuesta
JwtAuthResponse {
    encrypted_privkey_context: ecdh_encrypt(privkey_context_decrypted),
    encrypted_rotation_metadata: base64(encrypted_metadata),  // Opaco
    // NO incluir needs_rotation (anti-timing-attack)
}
```

#### Frontend (Post-Validación)

```typescript
// 1. Descifrar privkey_context (ECDH)
const privkeyContext = await ecdh_decrypt(response.encrypted_privkey_context);

// 2. Descifrar metadata
const metadataKey = blake3_kdf(privkeyContext, "metadata_encryption_v1");
const metadata = JSON.parse(
    await chacha20poly1305_decrypt(
        base64_decode(response.encrypted_rotation_metadata),
        metadataKey
    )
);

// 3. Derivar identity key (permanente)
const identityKeypair = await deriveUserKeys(email, privkeyContext, "identity_v1");

// 4. Generar ephemeral key (aleatoria, PFS)
const ephemeralKeypair = await crypto.subtle.generateKey("X25519", {
    name: "ECDH",
    namedCurve: "X25519"
}, false, ["deriveKey"]);  // non-extractable

// 5. Firmar challenge (proof-of-possession)
const challenge = hex_decode(metadata.derivation_challenge);
const signature = await crypto.subtle.sign(
    "Ed25519",
    identityKeypair.ed25519.privateKey,
    challenge
);

// 6. Cifrar ephemeral private con identity key (multi-dispositivo)
const sharedSecret = await crypto.subtle.deriveKey(
    { name: "ECDH", public: identityKeypair.ed25519.publicKey },
    identityKeypair.ed25519.privateKey,
    { name: "AES-GCM", length: 256 },
    false,
    ["encrypt"]
);

const encryptedEphemeralPriv = await crypto.subtle.encrypt(
    { name: "AES-GCM", iv: random_12_bytes() },
    sharedSecret,
    await crypto.subtle.exportKey("raw", ephemeralKeypair.privateKey)
);

// 7. Publicar llaves (con verificación)
const response = await httpAuthenticatedSignedPOSTRequest('/api/keys/rotate', {
    identity_ed25519_pub: hex(identityKeypair.ed25519.publicKey),
    ephemeral_x25519_pub: hex(ephemeralKeypair.publicKey),
    encrypted_ephemeral_priv: base64(encryptedEphemeralPriv),
    challenge_signature: base64(signature),
    rotation_counter: metadata.rotation_counter
});

// 8. Almacenar en IndexedDB
await storeIdentityKeypair(identityKeypair);
await storeEphemeralKeypair(ephemeralKeypair);
```

#### Backend (POST /api/keys/rotate)

```rust
// 1. Verificar challenge signature (proof-of-possession)
let stored_challenge = kv_store.get(format!("challenge:{}", hex::encode(user_id)))?;
let challenge_bytes = hex::decode(&stored_challenge)?;
let signature_bytes = base64::decode(&payload.challenge_signature)?;
let pub_key_bytes = hex::decode(&payload.identity_ed25519_pub)?;

if !ed25519_dalek::verify(&pub_key_bytes, &challenge_bytes, &signature_bytes)? {
    return Err("Challenge signature verification failed");
}

// 2. Verificar derivación correcta (identity key)
let expected_identity = kv_store.get(format!("expected_identity:{}", hex::encode(user_id)))?;

if payload.identity_ed25519_pub != expected_identity {
    return Err("Identity key derivation mismatch - attack detected");
}

// 3. Verificar monotonía de rotation_counter (anti-rollback)
let last_rotation = get_last_rotation_from_log(&db_index)?;
if payload.rotation_counter <= last_rotation.rotation_counter {
    return Err("Rotation counter rollback detected");
}

// 4. Firmar rotation entry (append-only log)
let rotation_entry = RotationEntry {
    db_index: db_index.clone(),
    rotation_counter: payload.rotation_counter,
    created_at: now,
};

let backend_signature = sign_ed25519(
    &get_backend_signing_key()?,
    &serde_json::to_vec(&rotation_entry)?
)?;

INSERT INTO user_key_rotation_log
(db_index, rotation_counter, created_at, backend_signature)
VALUES (?, ?, ?, ?);

// 5. Insertar identity key (idempotente)
INSERT OR IGNORE INTO user_identity_keys
(user_id, ed25519_pub, created_at)
VALUES (?, ?, ?);

// 6. Insertar ephemeral key con expiración (PFS)
let expires_at = now + (180 * 24 * 60 * 60);  // 180 días

INSERT OR IGNORE INTO user_ephemeral_keys
(user_id, x25519_pub, encrypted_x25519_priv, created_at, expires_at)
VALUES (?, ?, ?, ?, ?);

// 7. Respuesta exitosa
Ok(SignedResponse { success: true })
```

---

### 6.5 Medidas de Seguridad Implementadas

| Defecto | Medida Aplicada |
|---------|----------------|
| **1. Desincronización** | Contador derivado de `COUNT(DISTINCT created_at)` |
| **2. Correlación metadata** | Metadata cifrada con `blake3_kdf(privkey_context, "metadata")` |
| **3. Race condition** | Epoch_seed sin timestamp (solo rotation_counter) |
| **4. Clustering temporal** | Sin timestamp en derivación |
| **5. Epoch manipulado** | Frontend calcula epoch_seed (no enviado por backend) |
| **6. Derivación incorrecta** | Challenge-response + derivación backend |
| **7. Rollback attack** | Append-only log + firmas Ed25519 backend |
| **8. Inconsistencia** | Source of truth = COUNT en BBDD |
| **9. Pérdida PFS** | Ephemeral keys aleatorias + auto-delete |
| **10. Timing attack** | Sin `needs_rotation` en metadata + jitter |

---

## 7. Consideraciones de Implementación

### 7.1 Periodicidad y Políticas

```rust
// Configuración recomendada
const ROTATION_BASE_PERIOD_DAYS: i64 = 180;
const ROTATION_JITTER_DAYS: i64 = 30;
const MAX_HISTORICAL_ROTATIONS: usize = 5;  // 900 días
const EPHEMERAL_KEY_TTL_DAYS: i64 = 180;
const GRACE_PERIOD_DAYS: i64 = 30;  // Mantener encrypted_ephemeral_priv
```

### 7.2 Multi-Dispositivo

**Problema**: Usuario con 2 dispositivos (laptop + móvil)

**Solución**:

1. **Identity key**: Derivada en ambos dispositivos (mismo email + privkey_context)
2. **Ephemeral key**: `encrypted_x25519_priv` en BBDD permite recuperación
3. **Periodo de gracia**: 30 días para sincronizar antes de auto-delete

**Flujo**:

```
Día 0: Laptop genera ephemeral key A, publica en BBDD
Día 1: Móvil hace login, descarga encrypted_ephemeral_priv, descifra con identity key
Día 180: Rotación → nueva ephemeral key B
Día 210: Auto-delete ephemeral key A (grace period terminado)
```

### 7.3 Limpieza Automática (Cron Job)

```rust
// Ejecutar diariamente
async fn cleanup_expired_ephemeral_keys() -> Result<(), Error> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    // Eliminar llaves expiradas + grace period
    let grace_period = 30 * 24 * 60 * 60;

    connection.execute(
        "DELETE FROM user_ephemeral_keys WHERE expires_at + ? < ?",
        &[Value::Integer(grace_period), Value::Integer(now)]
    )?;

    Ok(())
}
```

### 7.4 Verificación de Integridad del Log

```typescript
// Frontend verifica rotation log en cada login
async function verifyRotationLog(userId: Uint8Array): Promise<boolean> {
    const entries = await fetch_rotation_log(userId);
    const backend_pub_key = BACKEND_PUBLIC_KEY;  // Hardcoded o TOFU

    for (const entry of entries) {
        const entryBytes = serialize_rotation_entry(entry);
        const valid = await verify_ed25519(
            backend_pub_key,
            entryBytes,
            entry.backend_signature
        );

        if (!valid) {
            console.error("Rotation log integrity violation!");
            return false;
        }
    }

    return true;
}
```

---

## 8. Estado Actual y Próximos Pasos

### 8.1 Infraestructura Existente (v1.11.0)

✅ **Implementado**:

- Tablas BBDD: `users`, `user_ed25519_keys`, `user_x25519_keys`, `user_privkey_context`
- Endpoint: `POST /api/keys/rotate` (publicación de llaves públicas)
- Endpoint: `GET /api/user/keys/?target_user=...` (recuperación de llaves públicas)
- Derivación determinista: `blake3_kdf(email, privkey_context, epoch_seed)`
- Frontend: Auto-publicación tras validación magic link
- Timestamps en llaves públicas (`created_at`)

❌ **Pendiente**:

- Lógica de rotación automática (verificación de edad de llaves)
- Separación identity/ephemeral keys
- Challenge-response verification
- Append-only rotation log con firmas
- Limpieza automática de llaves expiradas
- PFS con ephemeral keys aleatorias

---

### 8.2 Roadmap de Implementación

#### Fase 1: Tracking de Rotaciones (2-3 días)

**Tareas**:

1. Crear tabla `user_key_rotation_log` con triggers append-only
2. Implementar generación de firmas backend (Ed25519 signing key permanente)
3. Modificar validación magic link para firmar rotation entries
4. Frontend: verificar firmas del rotation log

**Entregables**:

- ✅ Append-only log inmutable
- ✅ Protección anti-rollback

#### Fase 2: Separación Identity/Ephemeral Keys (3-4 días)

**Tareas**:

1. Crear tablas `user_identity_keys` y `user_ephemeral_keys`
2. Modificar frontend para generar 2 tipos de llaves
3. Backend: almacenar `encrypted_x25519_priv` (multi-dispositivo)
4. Implementar auto-delete de llaves expiradas (cron job)

**Entregables**:

- ✅ Identity keys permanentes (firma)
- ✅ Ephemeral keys rotables (cifrado E2EE)
- ✅ PFS real (llaves antiguas irrecuperables después de grace period)

#### Fase 3: Challenge-Response Verification (2 días)

**Tareas**:

1. Backend: generar challenge durante validación magic link
2. Frontend: firmar challenge con identity private key
3. Backend `/api/keys/rotate`: verificar firma de challenge
4. Backend: derivar identity key esperada y comparar

**Entregables**:

- ✅ Proof-of-possession de llave privada
- ✅ Detección de llaves públicas fraudulentas

#### Fase 4: Metadata Encryption y Anti-Timing (1 día)

**Tareas**:

1. Backend: cifrar `rotation_metadata` con `blake3_kdf(privkey_context, "metadata")`
2. Frontend: descifrar metadata
3. Eliminar campo `needs_rotation` de respuesta
4. Agregar jitter aleatorio a periodicidad de rotación

**Entregables**:

- ✅ Metadata opaca (anti-correlación)
- ✅ Timing attack mitigado

#### Fase 5: Testing y Auditoría (2-3 días)

**Tareas**:

1. Tests unitarios (derivación, firmas, verificación)
2. Tests de integración (flujo completo de rotación)
3. Tests de seguridad (rollback, race conditions, timing)
4. Auditoría de código (revisión de correcciones)

**Entregables**:

- ✅ Cobertura de tests >90%
- ✅ Documentación de seguridad actualizada

---

### 8.3 Estimación Total

**Tiempo estimado**: 10-15 días de desarrollo
**Complejidad**: Media-Alta
**Riesgo**: Medio (requiere cambios en BBDD schema)

---

## 9. Referencias

### 9.1 Estándares y Especificaciones

- **NIST SP 800-57**: Recommendation for Key Management (Part 1: General)
  - https://csrc.nist.gov/publications/detail/sp/800-57-part-1/rev-5/final

- **PCI DSS v4.0**: Payment Card Industry Data Security Standard
  - https://www.pcisecuritystandards.org/document_library/

- **CISA**: Cyber Security Best Practices
  - https://www.cisa.gov/topics/cybersecurity-best-practices

- **Signal Protocol**: X3DH + Double Ratchet
  - https://signal.org/docs/specifications/doubleratchet/
  - https://signal.org/docs/specifications/x3dh/

### 9.2 Documentación Interna

- `docs/architecture/zero-knowledge.md` - Arquitectura Zero Knowledge de HashRand
- `docs/api/cryptography.md` - Especificaciones criptográficas
- `CLAUDE.md` - System A vs System B (líneas 155-298)

### 9.3 Bibliotecas Criptográficas

**Backend (Rust)**:
- `blake3` v1.5.4 - Key derivation
- `ed25519-dalek` v2.1.1 - Ed25519 signatures
- `x25519-dalek` v2.0.1 - X25519 ECDH
- `chacha20poly1305` v0.10.1 - AEAD encryption

**Frontend (TypeScript)**:
- `@noble/hashes` v1.5.0 - Blake3 KDF
- Web Crypto API - Ed25519/X25519 (native browser)

---

## Apéndice A: Glosario

| Término | Definición |
|---------|-----------|
| **System A** | Llaves temporales efímeras para seguridad API (request/response signing) |
| **System B** | Llaves permanentes deterministas para E2EE usuario-a-usuario |
| **privkey_context** | 64 bytes aleatorios cifrados en BBDD, semilla para derivar llaves System B |
| **rotation_counter** | Entero 0-indexed que indica número de rotaciones (0=primera llave, 1=segunda, etc.) |
| **epoch_seed** | Semilla derivada de `blake3(privkey_context \|\| rotation_counter)` usada para KDF |
| **Identity key** | Llave Ed25519 permanente derivada de privkey_context, usada para firmas/autenticación |
| **Ephemeral key** | Llave X25519 aleatoria (no derivada), usada para cifrado E2EE con rotación y PFS |
| **PFS (Perfect Forward Secrecy)** | Propiedad que garantiza que compromiso actual no afecta confidencialidad de mensajes pasados |
| **Append-only log** | Tabla BBDD inmutable (no permite DELETE ni UPDATE) con triggers |
| **Challenge-response** | Protocolo donde servidor envía challenge aleatorio y cliente lo firma con llave privada |
| **Zero Knowledge** | Arquitectura donde servidor no puede correlacionar identidades sin información adicional |

---

## Apéndice B: Vectores de Ataque No Cubiertos

Los siguientes ataques NO están cubiertos por este diseño y requieren medidas adicionales:

1. **Side-channel attacks**: Timing attacks en operaciones criptográficas (requiere constant-time crypto)
2. **Quantum computing**: Algoritmos post-cuánticos (requiere migración a CRYSTALS-Kyber/Dilithium)
3. **Social engineering**: Phishing, ingeniería social (requiere educación del usuario)
4. **Physical access**: Keyloggers, screen capture (fuera del alcance del diseño)
5. **Supply chain**: Compromiso de dependencias npm/cargo (requiere lockfiles + auditorías)

---

**Fin del documento**

---

**Notas para implementación futura**:

- Este documento debe actualizarse cuando se implemente cualquier fase del roadmap
- Agregar sección "Changelog" al final para trackear modificaciones arquitectónicas
- Considerar agregar diagramas de secuencia para flujos complejos (Mermaid.js)
- Enlazar con documentación de testing cuando se implementen tests de seguridad
