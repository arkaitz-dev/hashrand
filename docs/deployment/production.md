# Production Deployment

Complete guide for deploying HashRand to production environments.

## Unified Backend Deployment (Recommended)

### Production Deployment with Unified Backend

```bash
# Complete production deployment with unified backend (recommended)
just predeploy
```

This single command will:
- 🧹 **Stop & Clean**: Stop all services and clean build artifacts
- 🏗️ **Build Web Interface**: Compile SvelteKit SPA for production
- ⚙️ **Build Backend**: Compile WebAssembly backend component
- 🚀 **Start Unified Server**: Launch backend serving both API and static files
- 🌐 **External Access**: Automatically start Tailscale serve for remote access
- ✅ **Verify Deployment**: Test API connectivity and confirm services

### Unified Architecture Benefits

- **Single Port Deployment**: Both web interface and API served from port 3000
- **No Proxy Required**: Backend directly serves static files using `spin-fileserver`
- **Production Ready**: Optimized Vite build with caching and compression
- **Remote Access**: Automatic Tailscale integration for external access

### Access URLs

- **Local**: `http://localhost:3000` (both web interface and API)
- **Remote**: `https://your-tailscale-name.ts.net` (via Tailscale serve)

### Management Commands

```bash
just status    # Check deployment status (predeploy server + Tailscale)
just stop      # Stop all services (including predeploy and Tailscale)
tail -f .spin-predeploy.log  # Monitor deployment logs
```

## Cloud Deployment (Traditional)

### API Deployment to Fermyon Cloud

```bash
# Deploy to Fermyon Cloud (requires account with secrets)
SPIN_VARIABLE_JWT_SECRET="your-production-secret" \
SPIN_VARIABLE_MAGIC_LINK_HMAC_KEY="your-production-secret" \
SPIN_VARIABLE_USER_ID_HMAC_KEY="your-production-secret" \
SPIN_VARIABLE_ARGON2_SALT="your-production-secret" \
SPIN_VARIABLE_MAILTRAP_API_TOKEN="your-mailtrap-token" \
SPIN_VARIABLE_MAILTRAP_INBOX_ID="your-inbox-id" \
spin-cli deploy --runtime-config-file runtime-config.toml

# Or using justfile (loads from .env automatically)
just deploy
```

### Web Interface Deployment (Separate)

```bash
# Build static SPA
cd web && npm run build

# Deploy the 'dist' directory to any static hosting service:
# - Vercel, Netlify, GitHub Pages
# - AWS S3 + CloudFront
# - Any CDN or static file server

# For production, configure reverse proxy to route /api/* to your Spin API
```

## Production Configuration

### Required Environment Variables

```bash
# Production secrets (64 hex chars each)
JWT_SECRET=your-production-jwt-secret
MAGIC_LINK_HMAC_KEY=your-production-hmac-key
ARGON2_SALT=your-production-argon2-salt
CHACHA_ENCRYPTION_KEY=your-production-chacha-key

# Email service configuration
MAILTRAP_API_TOKEN=your-production-mailtrap-token
MAILTRAP_INBOX_ID=your-production-inbox-id

# Production mode
NODE_ENV=production
```

### Security Checklist

- ✅ **Unique Production Secrets**: Never reuse development secrets
- ✅ **Secret Rotation Plan**: Implement regular secret rotation
- ✅ **HTTPS Only**: Ensure all production traffic uses HTTPS
- ✅ **Domain Configuration**: Configure proper CORS and domain settings
- ✅ **Database Backups**: Regular SQLite database backups
- ✅ **Monitoring**: Set up monitoring and alerting
- ✅ **Rate Limiting**: Configure rate limiting at infrastructure level

## Deployment Platforms

### Fermyon Cloud

```bash
# Install Fermyon Cloud CLI
curl -fsSL https://cloud.fermyon.com/downloads/install.sh | bash

# Login to Fermyon Cloud
spin cloud login

# Deploy with secrets
spin cloud deploy --runtime-config-file runtime-config.toml
```

### Self-Hosted Spin

```bash
# Install Spin on server
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash

# Configure systemd service
sudo systemctl enable spin-hashrand
sudo systemctl start spin-hashrand
```

### Docker Deployment

```dockerfile
# Dockerfile for containerized deployment
FROM ghcr.io/fermyon/spin:latest

COPY target/wasm32-wasi/release/*.wasm /app/
COPY spin.toml /app/
COPY runtime-config.toml /app/
COPY web/dist/ /app/static/

WORKDIR /app
CMD ["spin", "up", "--listen", "0.0.0.0:3000"]
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: hashrand
spec:
  replicas: 3
  selector:
    matchLabels:
      app: hashrand
  template:
    metadata:
      labels:
        app: hashrand
    spec:
      containers:
      - name: hashrand
        image: hashrand:latest
        ports:
        - containerPort: 3000
        env:
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: hashrand-secrets
              key: jwt-secret
```

## Performance Optimization

### Production Optimizations

- **WebAssembly Compilation**: Optimized WASM builds with `--release`
- **Static Asset Compression**: Gzip/Brotli compression for static files
- **CDN Integration**: Serve static assets from CDN
- **Database Optimization**: SQLite optimizations for production workloads
- **Connection Pooling**: Efficient database connection management

### Monitoring & Observability

```bash
# Performance monitoring
- Cold start times: ~5ms
- Response times: <1ms for most requests
- Memory usage: ~2MB baseline
- Throughput: >10,000 requests/second
```

### Scaling Considerations

- **Horizontal Scaling**: Multiple Spin instances behind load balancer
- **Database Scaling**: SQLite suitable for moderate loads, consider PostgreSQL for high traffic
- **Geographic Distribution**: Deploy to multiple regions for global performance
- **Caching Strategy**: Implement caching layer for frequently accessed data

## Security in Production

### Network Security

- **HTTPS Termination**: TLS termination at load balancer or CDN
- **CORS Configuration**: Proper CORS headers for production domains
- **Rate Limiting**: Implement rate limiting to prevent abuse
- **DDoS Protection**: Use CDN with DDoS protection capabilities

### Application Security

- **Secret Management**: Use proper secret management systems
- **Database Encryption**: Encrypt SQLite databases at rest
- **Audit Logging**: Comprehensive audit logging for security events
- **Security Headers**: Implement security headers (CSP, HSTS, etc.)

### Zero Knowledge Compliance

- **GDPR/CCPA Ready**: No personal data stored, compliant by design
- **Privacy Audit**: Regular privacy compliance audits
- **Data Retention**: Automatic cleanup of expired sessions and tokens
- **Transparency**: Clear privacy policy and data handling documentation

## Backup & Recovery

### Database Backup

```bash
# SQLite backup strategy
sqlite3 data/hashrand.db ".backup hashrand-backup-$(date +%Y%m%d).db"

# Automated backup script
#!/bin/bash
BACKUP_DIR="/backups/hashrand"
DATE=$(date +%Y%m%d_%H%M%S)
sqlite3 data/hashrand.db ".backup $BACKUP_DIR/hashrand-$DATE.db"
```

### Disaster Recovery

- **Regular Backups**: Automated daily backups of SQLite databases
- **Secret Recovery**: Secure backup of production secrets
- **Configuration Management**: Version control for all configuration
- **Recovery Testing**: Regular disaster recovery testing

---

*For configuration details, see [Configuration Guide](./configuration.md)*  
*For development setup, see [Development Guide](./development.md)*  
*For quick start, see [Quick Start Guide](./quick-start.md)*