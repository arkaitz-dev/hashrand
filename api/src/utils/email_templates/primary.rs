//! Primary language email templates (English and Spanish)
//! These are the most commonly used templates with full accessibility improvements.

/// Creates the English HTML email template with accessibility improvements
pub fn create_english_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>HashRand Authentication</title>
    <style>
        /* Reset and base styles */
        body {{ 
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, Arial, sans-serif; 
            line-height: 1.6; 
            color: #1f2937; 
            margin: 0; 
            padding: 0; 
            background-color: #f9fafb;
        }}
        .container {{ 
            max-width: 600px; 
            margin: 0 auto; 
            padding: 20px; 
            background-color: #ffffff;
        }}
        .header {{ 
            background: #1e40af; 
            color: #ffffff; 
            padding: 24px; 
            text-align: center; 
            border-radius: 8px 8px 0 0; 
        }}
        .header h1 {{ 
            margin: 0; 
            font-size: 24px; 
            font-weight: 600;
        }}
        .content {{ 
            background: #ffffff; 
            padding: 32px; 
            border-left: 1px solid #d1d5db; 
            border-right: 1px solid #d1d5db;
        }}
        .content h2 {{ 
            color: #111827; 
            font-size: 20px; 
            margin-top: 0; 
            margin-bottom: 16px;
        }}
        .content p {{ 
            color: #374151; 
            font-size: 16px; 
            margin: 16px 0;
        }}
        .button {{ 
            display: inline-block; 
            background: #3b82f6; 
            color: #ffffff; 
            padding: 16px 32px; 
            text-decoration: none; 
            border-radius: 8px; 
            margin: 24px 0; 
            font-weight: 600;
            font-size: 16px;
            border: 3px solid #3b82f6;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}
        .button:hover {{ 
            background: #2563eb; 
            border-color: #2563eb;
            color: #ffffff;
            box-shadow: 0 6px 12px rgba(0, 0, 0, 0.2);
            transform: translateY(-1px);
        }}
        .footer {{ 
            background: #374151; 
            color: #ffffff; 
            padding: 20px; 
            text-align: center; 
            font-size: 14px; 
            border-radius: 0 0 8px 8px; 
        }}
        .footer p {{ 
            margin: 8px 0; 
            color: #f3f4f6;
        }}
        .security-notice {{ 
            background: #fffbeb; 
            border: 2px solid #f59e0b; 
            padding: 20px; 
            margin: 24px 0; 
            border-radius: 8px;
            color: #92400e;
        }}
        .security-notice strong {{ 
            color: #78350f; 
            font-size: 16px;
        }}
        .security-notice ul {{ 
            margin: 12px 0; 
            padding-left: 20px;
        }}
        .security-notice li {{ 
            margin: 8px 0; 
            color: #92400e;
        }}
        .link-fallback {{ 
            word-break: break-all; 
            background: #f5f5f5; 
            padding: 16px; 
            border-radius: 6px; 
            font-family: 'Courier New', monospace; 
            font-size: 14px; 
            border: 1px solid #e5e5e5;
            color: #333333;
            font-weight: 500;
            box-shadow: none;
        }}
        /* Dark mode support */
        @media (prefers-color-scheme: dark) {{
            .link-fallback {{ 
                background: #f0f0f0; 
                color: #444444; 
                border-color: #cccccc;
            }}
        }}
        /* Accessibility improvements */
        .sr-only {{ 
            position: absolute; 
            width: 1px; 
            height: 1px; 
            padding: 0; 
            margin: -1px; 
            overflow: hidden; 
            clip: rect(0, 0, 0, 0); 
            white-space: nowrap; 
            border: 0;
        }}
    </style>
</head>
<body>
    <div class="container" role="main">
        <header class="header">
            <h1>🔐 HashRand Authentication</h1>
        </header>
        <main class="content">
            <h2>Your Magic Link is Ready</h2>
            <p>Click the button below to securely authenticate with HashRand:</p>
            
            <div style="text-align: center;" role="group" aria-label="Authentication action">
                <a href="{}" class="button" style="color: white !important;" role="button" aria-label="Access HashRand securely">
                    🚀 Access HashRand
                </a>
            </div>
            
            <div class="security-notice" role="alert" aria-label="Important security information">
                <strong>Security Notice:</strong>
                <ul>
                    <li>This link is valid for a limited time</li>
                    <li>It can only be used once</li>
                    <li>Do not share this link with anyone</li>
                    <li>If you didn't request this, you can safely ignore this email</li>
                </ul>
            </div>
            
            <p>If the button doesn't work, you can copy and paste this link into your browser:</p>
            <div class="link-fallback" role="textbox" aria-label="Fallback authentication link">
                {}
            </div>
        </main>
        <footer class="footer">
            <p><strong>HashRand</strong> - Zero Knowledge Random Hash Generator</p>
            <p>This is an automated message. Please do not reply to this email.</p>
        </footer>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the English plain text email
pub fn create_english_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 HASHRAND AUTHENTICATION
════════════════════════════════════════════════

Your Magic Link is Ready!

To securely authenticate with HashRand, please copy and paste 
the following link into your web browser:

🚀 Authentication Link:
{}

⚠️  IMPORTANT SECURITY INFORMATION:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ This link is valid for a LIMITED TIME only
✓ This link can only be used ONCE
✓ DO NOT share this link with anyone else
✓ If you didn't request this authentication, you can safely ignore this email

Need help? This email was sent because someone requested authentication 
to access HashRand using your email address.

────────────────────────────────────────────────
HASHRAND - Zero Knowledge Random Hash Generator
────────────────────────────────────────────────
This is an automated security message from noreply@mailer.hashrand.com
Please do not reply to this email - replies are not monitored.

For support, visit: https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}

/// Creates the Spanish HTML email template with accessibility improvements
pub fn create_spanish_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Autenticación HashRand</title>
    <style>
        /* Reset and base styles */
        body {{ 
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, Arial, sans-serif; 
            line-height: 1.6; 
            color: #1f2937; 
            margin: 0; 
            padding: 0; 
            background-color: #f9fafb;
        }}
        .container {{ 
            max-width: 600px; 
            margin: 0 auto; 
            padding: 20px; 
            background-color: #ffffff;
        }}
        .header {{ 
            background: #1e40af; 
            color: #ffffff; 
            padding: 24px; 
            text-align: center; 
            border-radius: 8px 8px 0 0; 
        }}
        .header h1 {{ 
            margin: 0; 
            font-size: 24px; 
            font-weight: 600;
        }}
        .content {{ 
            background: #ffffff; 
            padding: 32px; 
            border-left: 1px solid #d1d5db; 
            border-right: 1px solid #d1d5db;
        }}
        .content h2 {{ 
            color: #111827; 
            font-size: 20px; 
            margin-top: 0; 
            margin-bottom: 16px;
        }}
        .content p {{ 
            color: #374151; 
            font-size: 16px; 
            margin: 16px 0;
        }}
        .button {{ 
            display: inline-block; 
            background: #3b82f6; 
            color: #ffffff; 
            padding: 16px 32px; 
            text-decoration: none; 
            border-radius: 8px; 
            margin: 24px 0; 
            font-weight: 600;
            font-size: 16px;
            border: 3px solid #3b82f6;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}
        .button:hover {{ 
            background: #2563eb; 
            border-color: #2563eb;
            color: #ffffff;
            box-shadow: 0 6px 12px rgba(0, 0, 0, 0.2);
            transform: translateY(-1px);
        }}
        .footer {{ 
            background: #374151; 
            color: #ffffff; 
            padding: 20px; 
            text-align: center; 
            font-size: 14px; 
            border-radius: 0 0 8px 8px; 
        }}
        .footer p {{ 
            margin: 8px 0; 
            color: #f3f4f6;
        }}
        .security-notice {{ 
            background: #fffbeb; 
            border: 2px solid #f59e0b; 
            padding: 20px; 
            margin: 24px 0; 
            border-radius: 8px;
            color: #92400e;
        }}
        .security-notice strong {{ 
            color: #78350f; 
            font-size: 16px;
        }}
        .security-notice ul {{ 
            margin: 12px 0; 
            padding-left: 20px;
        }}
        .security-notice li {{ 
            margin: 8px 0; 
            color: #92400e;
        }}
        .link-fallback {{ 
            word-break: break-all; 
            background: #f5f5f5; 
            padding: 16px; 
            border-radius: 6px; 
            font-family: 'Courier New', monospace; 
            font-size: 14px; 
            border: 1px solid #e5e5e5;
            color: #333333;
            font-weight: 500;
            box-shadow: none;
        }}
        /* Dark mode support */
        @media (prefers-color-scheme: dark) {{
            .link-fallback {{ 
                background: #f0f0f0; 
                color: #444444; 
                border-color: #cccccc;
            }}
        }}
        /* Accessibility improvements */
        .sr-only {{ 
            position: absolute; 
            width: 1px; 
            height: 1px; 
            padding: 0; 
            margin: -1px; 
            overflow: hidden; 
            clip: rect(0, 0, 0, 0); 
            white-space: nowrap; 
            border: 0;
        }}
    </style>
</head>
<body>
    <div class="container" role="main">
        <header class="header">
            <h1>🔐 Autenticación HashRand</h1>
        </header>
        <main class="content">
            <h2>Tu Enlace Mágico está Listo</h2>
            <p>Haz clic en el botón de abajo para autenticarte de forma segura con HashRand:</p>
            
            <div style="text-align: center;" role="group" aria-label="Acción de autenticación">
                <a href="{}" class="button" style="color: white !important;" role="button" aria-label="Acceder a HashRand de forma segura">
                    🚀 Acceder a HashRand
                </a>
            </div>
            
            <div class="security-notice" role="alert" aria-label="Información importante de seguridad">
                <strong>Aviso de Seguridad:</strong>
                <ul>
                    <li>Este enlace es válido por tiempo limitado</li>
                    <li>Solo puede usarse una vez</li>
                    <li>No compartas este enlace con nadie</li>
                    <li>Si no solicitaste esto, puedes ignorar este email</li>
                </ul>
            </div>
            
            <p>Si el botón no funciona, puedes copiar y pegar este enlace en tu navegador:</p>
            <div class="link-fallback" role="textbox" aria-label="Enlace de autenticación alternativo">
                {}
            </div>
        </main>
        <footer class="footer">
            <p><strong>HashRand</strong> - Generador de Hashes Aleatorios con Zero Knowledge</p>
            <p>Este es un mensaje automático. Por favor no respondas a este email.</p>
        </footer>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Spanish plain text email
pub fn create_spanish_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 AUTENTICACIÓN HASHRAND  
════════════════════════════════════════════════

¡Tu Enlace Mágico está Listo!

Para autenticarte de forma segura con HashRand, por favor copia y pega 
el siguiente enlace en tu navegador web:

🚀 Enlace de Autenticación:
{}

⚠️  INFORMACIÓN IMPORTANTE DE SEGURIDAD:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Este enlace es válido por TIEMPO LIMITADO únicamente
✓ Este enlace solo puede usarse UNA VEZ
✓ NO compartas este enlace con nadie más
✓ Si no solicitaste esta autenticación, puedes ignorar este email

¿Necesitas ayuda? Este email fue enviado porque alguien solicitó 
autenticación para acceder a HashRand usando tu dirección de correo.

────────────────────────────────────────────────
HASHRAND - Generador de Hashes Aleatorios con Zero Knowledge
────────────────────────────────────────────────
Este es un mensaje automatizado de seguridad de noreply@mailer.hashrand.com
Por favor no respondas a este email - las respuestas no son monitoreadas.

Para soporte, visita: https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}