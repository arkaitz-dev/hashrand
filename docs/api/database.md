# Database System

HashRand includes a **complete SQLite database system** for user management with Spin variable-based database selection and Zero Knowledge privacy architecture.

## Database Architecture

### Spin Variable-Based Selection

The application selects the appropriate database using Spin configuration variables:

- **Development Database**: `hashrand-dev.db` - Used when `database_name = "hashrand-dev"` (spin-dev.toml)
- **Production Database**: `hashrand.db` - Used when `database_name = "hashrand"` (spin-prod.toml)
- **Configuration-Driven**: Database selection through Spin variables, no hardcoded logic
- **Environment Separation**: Complete isolation between development and production databases
- **Table Auto-Creation**: All tables created automatically on first access

### Configuration Files

- **`runtime-config.toml`**: Defines database paths for both environments
- **`spin-dev.toml`**: Development configuration with `database_name = "hashrand-dev"`
- **`spin-prod.toml`**: Production configuration with `database_name = "hashrand"`
- **`data/`**: Directory containing SQLite database files (gitignored)

## Zero Knowledge Database Schema

### Users Table

```sql
CREATE TABLE users (
    user_id BLOB PRIMARY KEY,           -- 16-byte cryptographic hash (no PII)
    created_at INTEGER DEFAULT (unixepoch())  -- Unix timestamp (timezone-agnostic)
);
```

**Key Features:**
- **No Personal Information**: Only cryptographic hashes stored
- **Primary Key**: 16-byte Blake2b-variable hash derived from email
- **Timezone Agnostic**: Unix timestamps for universal compatibility
- **Privacy Compliant**: GDPR/CCPA compliant by design

### Magic Links Table

```sql
CREATE TABLE magiclinks (
    token_hash BLOB PRIMARY KEY,        -- 16-byte Blake2b-variable hash of encrypted token
    timestamp INTEGER NOT NULL,         -- Original timestamp used in magic link creation
    encryption_blob BLOB NOT NULL,      -- 44 bytes: nonce[12] + secret_key[32] from ChaCha8RNG
    next_param TEXT,                     -- Optional next destination parameter
    expires_at INTEGER NOT NULL         -- Unix timestamp expiration
    -- No user data, emails, or PII - only cryptographic hashes and encryption metadata
);
```

**Key Features:**
- **Encrypted Token Storage**: ChaCha20-encrypted magic link data
- **Blake2b-variable Hashing**: 16-byte primary key for optimal indexing
- **Expiration Management**: Automatic cleanup of expired tokens
- **Zero PII**: No personal information stored anywhere

### Legacy Users Table (for reference)

The system also maintains a legacy users table for backward compatibility:

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## Database Operations

### User Management

The Zero Knowledge system provides these operations:

```rust
// Create user from email (Zero Knowledge)
pub async fn create_zk_user(email: &str) -> Result<[u8; 16], DatabaseError> {
    let user_id = derive_user_id(email);
    // Insert user_id into database without storing email
    insert_user_id(&user_id).await
}

// Check if user exists (Zero Knowledge)
pub async fn user_exists(email: &str) -> Result<bool, DatabaseError> {
    let user_id = derive_user_id(email);
    check_user_id_exists(&user_id).await
}
```

### Magic Link Operations

```rust
// Store magic link (encrypted)
pub async fn store_magic_link(
    token: &[u8],
    user_id: &[u8; 16], 
    expires_at: i64,
    next_param: Option<String>
) -> Result<(), DatabaseError> {
    let token_hash = blake2b_variable_hash(token, 16);
    let encryption_blob = chacha20_encrypt(user_id, timestamp);
    insert_magic_link(&token_hash, encryption_blob, expires_at, next_param).await
}

// Validate and consume magic link with Ed25519 signature verification (v0.19.14+)
// NOTE: This is pseudo-code - actual implementation in api/src/utils/auth/magic_link_val.rs
pub async fn validate_magic_link_secure(token: &[u8], signature: &str) -> Result<Option<[u8; 16]>, DatabaseError> {
    // 1. Decrypt and validate magic link token
    let token_hash = blake2b_variable_hash(token, 16);
    let link_data = get_magic_link(&token_hash).await?;

    if let Some(data) = link_data {
        if data.expires_at > current_timestamp() {
            // 2. Extract Ed25519 public key from encrypted payload
            let (user_id, pub_key) = chacha20_decrypt_with_auth_data(&data.encryption_blob)?;

            // 3. Verify Ed25519 signature of the magic link token
            if ed25519_verify(token, signature, &pub_key) {
                delete_magic_link(&token_hash).await?;
                Ok(Some(user_id))
            } else {
                Err(DatabaseError::InvalidSignature)
            }
        } else {
            // Cleanup expired link
            delete_magic_link(&token_hash).await?;
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
```

## Environment-Aware Database Selection

### Request Host Detection

```rust
pub fn get_database_name(host: Option<&str>) -> &'static str {
    match host {
        Some("localhost") | Some("elite.faun-pirate.ts.net") => "hashrand-dev",
        _ => "hashrand"
    }
}
```

### Development vs Production

| **Environment** | **Host Pattern** | **Database** | **Features** |
|-----------------|------------------|--------------|--------------|
| **Development** | `localhost`, `*.ts.net` | `hashrand-dev.db` | Extended expiration, console logs |
| **Production** | All other hosts | `hashrand.db` | Standard security timeouts |

## Database Maintenance

### Automatic Cleanup

The system includes automatic cleanup of expired records:

```sql
-- Clean expired magic links (runs automatically)
DELETE FROM magiclinks WHERE expires_at < unixepoch();

-- Optional: Clean old users (manual operation)
DELETE FROM users WHERE created_at < unixepoch() - 86400 * 30; -- 30 days
```

### Performance Optimization

- **Indexed Primary Keys**: All BLOB primary keys use Blake2b-variable for optimal distribution
- **Timestamp Indexing**: Unix timestamps enable efficient range queries
- **Minimal Storage**: Only essential cryptographic data stored
- **No Text Indexing**: Eliminates potential information leakage through index structures

## Integration with Spin

### Configuration in `spin.toml`

```toml
[[component]]
id = "hashrand-api"
source = "target/wasm32-wasi/release/hashrand_spin_api.wasm"

[component.config]
allowed_http_hosts = ["*"]

[[component.trigger.http]]
route = "/api/..."

[component.variables]
database_url = { required = true }

[component.build]
command = "cargo build --target wasm32-wasi --release"
```

### Runtime Configuration in `runtime-config.toml`

```toml
[variables]
database_url = "sqlite://data/hashrand.db"

[variables.hashrand-dev]
database_url = "sqlite://data/hashrand-dev.db"
```

## Development Usage

### Database Files Location

```
data/
├── hashrand-dev.db      # Development database
└── hashrand.db          # Production database (created when needed)
```

### Example Operations

```bash
# Development requests automatically use hashrand-dev.db
curl -X POST "http://localhost:3000/api/users" \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com"}'

# Production requests use hashrand.db
curl -X POST "https://api.hashrand.com/api/users" \
  -H "Content-Type: application/json" \
  -d '{"username":"produser","email":"prod@example.com"}'
```

### Database Inspection

```bash
# Inspect development database
sqlite3 data/hashrand-dev.db

# Common queries
.tables                          # List all tables
SELECT hex(user_id) FROM users;  # View user IDs (as hex)
SELECT COUNT(*) FROM magiclinks; # Count active magic links
SELECT * FROM magiclinks WHERE expires_at > unixepoch(); # Active links
```

## Security Considerations

### Zero Knowledge Properties

- **No PII Storage**: Database never contains emails, names, or personal information
- **Cryptographic Indexing**: All keys and indexes use cryptographic hashes
- **Forward Secrecy**: User identification possible but email recovery impossible
- **Audit Safe**: Database dumps contain no sensitive information

### Access Control

- **Application Level**: All database access through application logic only
- **No Direct Access**: Database files not exposed through web server
- **Environment Separation**: Development and production databases isolated
- **Spin Sandboxing**: Database access restricted by Spin component permissions

---

*For API usage, see [API Endpoints](./endpoints.md)*  
*For cryptographic details, see [Cryptography Documentation](./cryptography.md)*  
*For Zero Knowledge architecture, see [Zero Knowledge Documentation](../architecture/zero-knowledge.md)*