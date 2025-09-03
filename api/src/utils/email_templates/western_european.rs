//! Western European language email templates (French, German, Portuguese)
//! All templates include accessibility improvements and proper semantic markup.

/// Creates the French HTML email template
pub fn create_french_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Authentification HashRand</title>
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
            <h1>🔐 Authentification HashRand</h1>
        </header>
        <main class="content">
            <h2>Votre Lien Magique est Prêt</h2>
            <p>Cliquez sur le bouton ci-dessous pour vous authentifier en toute sécurité avec HashRand :</p>
            
            <div style="text-align: center;" role="group" aria-label="Action d'authentification">
                <a href="{}" class="button" style="color: white !important;" role="button" aria-label="Accéder à HashRand en toute sécurité">
                    🚀 Accéder à HashRand
                </a>
            </div>
            
            <div class="security-notice" role="alert" aria-label="Informations importantes de sécurité">
                <strong>Avis de Sécurité :</strong>
                <ul>
                    <li>Ce lien est valide pour une durée limitée</li>
                    <li>Il ne peut être utilisé qu'une seule fois</li>
                    <li>Ne partagez pas ce lien avec qui que ce soit</li>
                    <li>Si vous n'avez pas demandé ceci, vous pouvez ignorer cet email</li>
                </ul>
            </div>
            
            <p>Si le bouton ne fonctionne pas, vous pouvez copier et coller ce lien dans votre navigateur :</p>
            <div class="link-fallback" role="textbox" aria-label="Lien d'authentification alternatif">
                {}
            </div>
        </main>
        <footer class="footer">
            <p><strong>HashRand</strong> - Générateur de Hachages Aléatoires Zero Knowledge</p>
            <p>Ceci est un message automatique. Veuillez ne pas répondre à cet email.</p>
        </footer>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the French plain text email
pub fn create_french_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 AUTHENTIFICATION HASHRAND
════════════════════════════════════════════════

Votre Lien Magique est Prêt !

Pour vous authentifier en toute sécurité avec HashRand, veuillez copier et coller
le lien suivant dans votre navigateur web :

🚀 Lien d'Authentification :
{}

⚠️  INFORMATIONS IMPORTANTES DE SÉCURITÉ :
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Ce lien est valide pour une DURÉE LIMITÉE uniquement
✓ Ce lien ne peut être utilisé qu'UNE SEULE FOIS
✓ NE partagez PAS ce lien avec qui que ce soit
✓ Si vous n'avez pas demandé cette authentification, vous pouvez ignorer cet email

Besoin d'aide ? Cet email a été envoyé car quelqu'un a demandé une authentification
pour accéder à HashRand en utilisant votre adresse email.

────────────────────────────────────────────────
HASHRAND - Générateur de Hachages Aléatoires Zero Knowledge
────────────────────────────────────────────────
Ceci est un message automatisé de sécurité de noreply@mailer.hashrand.com
Veuillez ne pas répondre à cet email - les réponses ne sont pas surveillées.

Pour le support, visitez : https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}

/// Creates the German HTML email template (placeholder - needs full accessibility implementation)
pub fn create_german_html(magic_link: &str) -> String {
    // TODO: Implement full accessibility improvements for German
    format!(
        r#"
<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="utf-8">
    <title>HashRand Authentifizierung</title>
</head>
<body>
    <h1>🔐 HashRand Authentifizierung</h1>
    <h2>Ihr Magic Link ist Bereit</h2>
    <p><a href="{}">🚀 Zu HashRand</a></p>
    <p>Fallback link: {}</p>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the German plain text email
pub fn create_german_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 HASHRAND AUTHENTIFIZIERUNG
════════════════════════════════════════════════

Ihr Magic Link ist Bereit!

Um sich sicher bei HashRand zu authentifizieren, kopieren Sie bitte den folgenden 
Link und fügen ihn in Ihren Webbrowser ein:

🚀 Authentifizierungs-Link:
{}

⚠️  WICHTIGE SICHERHEITSINFORMATIONEN:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Dieser Link ist nur für BEGRENZTE ZEIT gültig
✓ Dieser Link kann nur EINMAL verwendet werden
✓ Teilen Sie diesen Link NICHT mit anderen
✓ Falls Sie diese Authentifizierung nicht angefordert haben, können Sie diese E-Mail ignorieren

Brauchen Sie Hilfe? Diese E-Mail wurde gesendet, weil jemand eine Authentifizierung
für den Zugang zu HashRand mit Ihrer E-Mail-Adresse angefordert hat.

────────────────────────────────────────────────
HASHRAND - Zero Knowledge Zufalls-Hash-Generator
────────────────────────────────────────────────
Dies ist eine automatisierte Sicherheitsnachricht von noreply@mailer.hashrand.com
Bitte antworten Sie nicht auf diese E-Mail - Antworten werden nicht überwacht.

Für Support besuchen Sie: https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}

/// Creates the Portuguese HTML email template (placeholder - needs full accessibility implementation)
pub fn create_portuguese_html(magic_link: &str) -> String {
    // TODO: Implement full accessibility improvements for Portuguese
    format!(
        r#"
<!DOCTYPE html>
<html lang="pt">
<head>
    <meta charset="utf-8">
    <title>Autenticação HashRand</title>
</head>
<body>
    <h1>🔐 Autenticação HashRand</h1>
    <h2>Seu Link Mágico está Pronto</h2>
    <p><a href="{}">🚀 Acessar HashRand</a></p>
    <p>Fallback link: {}</p>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Portuguese plain text email
pub fn create_portuguese_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 AUTENTICAÇÃO HASHRAND
════════════════════════════════════════════════

Seu Link Mágico está Pronto!

Para se autenticar com segurança no HashRand, por favor copie e cole 
o seguinte link no seu navegador web:

🚀 Link de Autenticação:
{}

⚠️  INFORMAÇÕES IMPORTANTES DE SEGURANÇA:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Este link é válido por TEMPO LIMITADO apenas
✓ Este link só pode ser usado UMA VEZ
✓ NÃO compartilhe este link com ninguém
✓ Se você não solicitou esta autenticação, pode ignorar este email

Precisa de ajuda? Este email foi enviado porque alguém solicitou autenticação
para acessar o HashRand usando seu endereço de email.

────────────────────────────────────────────────
HASHRAND - Gerador de Hashes Aleatórios Zero Knowledge
────────────────────────────────────────────────
Esta é uma mensagem automatizada de segurança de noreply@mailer.hashrand.com
Por favor não responda a este email - respostas não são monitoradas.

Para suporte, visite: https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}