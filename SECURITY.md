# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

The security of `hashrand` is important to us. If you discover a security vulnerability, please report it responsibly.

### How to Report

Please **DO NOT** report security vulnerabilities through public GitHub issues, discussions, or pull requests.

Instead, please send an email to:
- **Email**: me@arkaitz.dev
- **Subject**: `[SECURITY] hashrand vulnerability report`

### What to Include

Please include the following information in your report:

1. **Description**: A clear description of the vulnerability
2. **Steps to Reproduce**: Detailed steps to reproduce the issue
3. **Impact**: Your assessment of the potential impact
4. **Affected Versions**: Which versions are affected
5. **Suggested Fix**: If you have ideas for how to fix the vulnerability

### Example Report Format

```
Subject: [SECURITY] hashrand vulnerability report

Description:
Path traversal vulnerability in --path parameter allowing file creation outside intended directories

Steps to Reproduce:
1. Run: hashrand --touch --path "../../../etc" --prefix "malicious_"
2. Observe file created outside intended directory

Impact:
Allows attackers to create files in arbitrary locations with appropriate permissions

Affected Versions:
All versions prior to 0.1.0

Suggested Fix:
Implement path canonicalization and validation (already implemented in v0.1.0)
```

### Response Timeline

- **Acknowledgment**: Within 48 hours of receiving your report
- **Initial Assessment**: Within 5 business days
- **Status Updates**: Weekly updates on investigation progress
- **Resolution**: Target resolution within 30 days for critical issues

### Security Response Process

1. **Triage**: We assess the report and determine severity
2. **Investigation**: We investigate and validate the vulnerability
3. **Fix Development**: We develop and test a fix
4. **Coordinated Disclosure**: We coordinate release timing with you
5. **Public Disclosure**: We publish security advisory and release fix

### Disclosure Policy

- **Critical/High Severity**: 90-day coordinated disclosure
- **Medium Severity**: 120-day coordinated disclosure  
- **Low Severity**: Best effort timeline

We believe in responsible disclosure and will work with you to ensure vulnerabilities are addressed promptly while minimizing risk to users.

## Security Features

`hashrand` includes several built-in security features:

### Implemented Protections

- **Path Validation**: Prevents directory traversal attacks
- **Resource Limits**: Prevents DoS through resource exhaustion
- **Secure Randomness**: Uses cryptographically secure random generation
- **Error Handling**: Graceful error handling without information disclosure
- **Audit Logging**: Optional operation tracking without sensitive data exposure
- **Permission Control**: Unix file/directory permission management

### Security Assumptions

- Users have legitimate access to the system
- File system permissions are properly configured
- OS-provided entropy is available and secure
- Runtime environment is trusted

## Security Contact

For security-related questions or concerns:
- **Email**: me@arkaitz.dev
- **GPG**: Available upon request

## Acknowledgments

We appreciate the security research community and will acknowledge researchers who report vulnerabilities responsibly, unless they prefer to remain anonymous.

### Hall of Fame

*No vulnerabilities have been reported yet.*

## Security Updates

Security updates will be:
- Released as patch versions (e.g., 0.1.1, 0.1.2)
- Announced in release notes with CVE numbers if applicable
- Tagged with "security" label in GitHub releases
- Documented in CHANGELOG.md with detailed impact assessment

## Scope

### In Scope
- Path traversal vulnerabilities
- Resource exhaustion attacks
- Information disclosure issues
- Cryptographic weaknesses
- File permission bypass

### Out of Scope
- Network-based attacks (except for HTTP server mode security)
- Physical access attacks
- Social engineering
- Issues requiring OS-level privileges
- Third-party dependency vulnerabilities (report to respective maintainers)

## Security Practices

### Development
- All code changes undergo security review
- Dependencies are regularly audited with `cargo audit`
- Security features are comprehensively tested
- Threat modeling is performed for new features

### Release
- Security-focused testing before each release
- Dependency vulnerability scanning
- Documentation review for security implications
- Coordinated disclosure of any identified issues

### HTTP Server Deployment

**⚠️ CRITICAL**: When deploying the HTTP server mode (`--serve`), the following security practices are **MANDATORY**:

1. **Always use SSL/TLS**: The server must be deployed behind a reverse proxy that enforces HTTPS
2. **Never expose directly to internet**: The built-in server does not provide encryption
3. **Recommended deployment**:
   ```
   Internet → HTTPS → Reverse Proxy (nginx/Apache/Caddy) → HTTP → hashrand server (localhost)
   ```
4. **Example nginx configuration**:
   ```nginx
   server {
       listen 443 ssl http2;
       server_name api.example.com;
       
       ssl_certificate /path/to/cert.pem;
       ssl_certificate_key /path/to/key.pem;
       
       location /api/ {
           proxy_pass http://127.0.0.1:8080;
           proxy_set_header Host $host;
           proxy_set_header X-Real-IP $remote_addr;
           proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
           proxy_set_header X-Forwarded-Proto $scheme;
       }
   }
   ```
5. **Security headers**: Configure your reverse proxy to add security headers (HSTS, CSP, etc.)
6. **Rate limiting**: Implement rate limiting at the reverse proxy level
7. **Access control**: Use firewall rules to ensure the hashrand server only accepts connections from localhost

---

*This security policy is effective as of August 6, 2025 and will be updated as necessary.*