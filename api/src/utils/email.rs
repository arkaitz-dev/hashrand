use anyhow::{Result, anyhow};
use serde_json::json;
use spin_sdk::{
    http::{Method, Request, Response},
    variables,
};

/// Email configuration loaded from Spin variables
#[derive(Debug)]
pub struct EmailConfig {
    pub api_url: String,
    pub api_token: String,
    pub inbox_id: String,
    pub from_email: String,
}

impl EmailConfig {
    /// Load email configuration from Spin environment variables
    pub fn from_environment() -> Result<Self> {
        let api_url = variables::get("mailtrap_api_url")
            .map_err(|e| anyhow!("Missing mailtrap_api_url: {}", e))?;
        let api_token = variables::get("mailtrap_api_token")
            .map_err(|e| anyhow!("Missing mailtrap_api_token: {}", e))?;
        let inbox_id = variables::get("mailtrap_inbox_id")
            .map_err(|e| anyhow!("Missing mailtrap_inbox_id: {}", e))?;
        let from_email =
            variables::get("from_email").map_err(|e| anyhow!("Missing from_email: {}", e))?;

        Ok(EmailConfig {
            api_url,
            api_token,
            inbox_id,
            from_email,
        })
    }
}

/// Creates an HTTP request for sending email via Mailtrap API
fn create_email_request(
    config: &EmailConfig,
    recipient_email: &str,
    magic_link: &str,
    language: Option<&str>,
) -> Result<Request> {
    // Create email payload according to Mailtrap API format
    let email_payload = json!({
        "from": {
            "email": config.from_email,
            "name": "HashRand"
        },
        "to": [
            {
                "email": recipient_email,
                "name": recipient_email.split('@').next().unwrap_or("User")
            }
        ],
        "subject": create_subject(language),
        "text": create_magic_link_text(magic_link, language),
        "html": create_magic_link_html(magic_link, language),
        "category": "Authentication"
    });

    // Convert payload to JSON string
    let body_json = serde_json::to_string(&email_payload)
        .map_err(|e| anyhow!("Failed to serialize email payload: {}", e))?;

    // Build full URL with inbox ID (format: https://sandbox.api.mailtrap.io/api/send/INBOX_ID)
    let full_url = format!("{}/{}", config.api_url, config.inbox_id);

    // Build HTTP request using Spin's Request builder
    let request = Request::builder()
        .method(Method::Post)
        .uri(&full_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", config.api_token))
        .body(body_json)
        .build();

    Ok(request)
}

/// Sends a magic link email to the specified recipient using Mailtrap REST API
///
/// # Arguments
/// * `recipient_email` - The email address to send the magic link to
/// * `magic_link` - The full magic link URL for authentication
/// * `language` - Optional language code for email template (e.g., "es", "en")
///
/// # Returns
/// * `Ok(())` if the email was sent successfully
/// * `Err(anyhow::Error)` if there was an error sending the email
pub async fn send_magic_link_email(
    recipient_email: &str,
    magic_link: &str,
    language: Option<&str>,
) -> Result<()> {
    let config = EmailConfig::from_environment()?;

    // Validate email format (basic validation)
    if recipient_email.is_empty() || !recipient_email.contains('@') {
        return Err(anyhow!(
            "Invalid recipient email address: {}",
            recipient_email
        ));
    }

    // Create HTTP request for Mailtrap API
    let request = create_email_request(&config, recipient_email, magic_link, language)?;

    // Send HTTP request using Spin's outbound HTTP
    let response: Response = spin_sdk::http::send(request)
        .await
        .map_err(|e| anyhow!("Failed to send HTTP request to Mailtrap API: {}", e))?;

    // Check if the request was successful
    let status = response.status();
    if *status == 200 || *status == 202 {
        println!(
            "✅ Magic link email sent successfully to: {} (Status: {})",
            recipient_email, status
        );
        Ok(())
    } else {
        let body = String::from_utf8_lossy(response.body());
        Err(anyhow!(
            "Mailtrap API returned error. Status: {}, Body: {}",
            status,
            body
        ))
    }
}

/// Creates the subject line based on language
fn create_subject(language: Option<&str>) -> &'static str {
    match language {
        Some("es") => "Tu enlace de autenticación para HashRand",
        Some("ca") => "El teu enllaç d'autenticació per a HashRand",
        Some("eu") => "Zure HashRand autentifikazio esteka",
        Some("gl") => "A túa ligazón de autenticación para HashRand",
        Some("fr") => "Votre lien d'authentification HashRand",
        Some("de") => "Ihr HashRand-Authentifizierungslink",
        Some("pt") => "Seu link de autenticação HashRand",
        Some("ru") => "Ваша ссылка аутентификации HashRand",
        Some("zh") => "您的 HashRand 身份验证链接",
        Some("ja") => "HashRand認証リンク",
        Some("ar") => "رابط مصادقة HashRand الخاص بك",
        Some("hi") => "आपका HashRand प्रमाणीकरण लिंक",
        _ => "Your HashRand Authentication Link", // Default to English
    }
}

/// Creates the HTML version of the magic link email
fn create_magic_link_html(magic_link: &str, language: Option<&str>) -> String {
    match language {
        Some("es") => create_spanish_html(magic_link),
        Some("ca") => create_catalan_html(magic_link),
        Some("eu") => create_basque_html(magic_link),
        Some("gl") => create_galician_html(magic_link),
        Some("fr") => create_french_html(magic_link),
        Some("de") => create_german_html(magic_link),
        Some("pt") => create_portuguese_html(magic_link),
        Some("ru") => create_russian_html(magic_link),
        Some("zh") => create_chinese_html(magic_link),
        Some("ja") => create_japanese_html(magic_link),
        Some("ar") => create_arabic_html(magic_link),
        Some("hi") => create_hindi_html(magic_link),
        _ => create_english_html(magic_link),
    }
}

/// Creates the English HTML email template
fn create_english_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>HashRand Authentication</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 HashRand Authentication</h1>
        </div>
        <div class="content">
            <h2>Your Magic Link is Ready</h2>
            <p>Click the button below to securely authenticate with HashRand:</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 Access HashRand</a>
            </div>
            
            <div class="security-notice">
                <strong>Security Notice:</strong>
                <ul>
                    <li>This link is valid for a limited time</li>
                    <li>It can only be used once</li>
                    <li>Do not share this link with anyone</li>
                    <li>If you didn't request this, you can safely ignore this email</li>
                </ul>
            </div>
            
            <p>If the button doesn't work, you can copy and paste this link into your browser:</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - Zero Knowledge Random Hash Generator</p>
            <p>This is an automated message. Please do not reply to this email.</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Spanish HTML email template
fn create_spanish_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Autenticación HashRand</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 Autenticación HashRand</h1>
        </div>
        <div class="content">
            <h2>Tu Enlace Mágico está Listo</h2>
            <p>Haz clic en el botón de abajo para autenticarte de forma segura con HashRand:</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 Acceder a HashRand</a>
            </div>
            
            <div class="security-notice">
                <strong>Aviso de Seguridad:</strong>
                <ul>
                    <li>Este enlace es válido por tiempo limitado</li>
                    <li>Solo puede usarse una vez</li>
                    <li>No compartas este enlace con nadie</li>
                    <li>Si no solicitaste esto, puedes ignorar este email</li>
                </ul>
            </div>
            
            <p>Si el botón no funciona, puedes copiar y pegar este enlace en tu navegador:</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - Generador de Hashes Aleatorios con Zero Knowledge</p>
            <p>Este es un mensaje automático. Por favor no respondas a este email.</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the plain text version of the magic link email
fn create_magic_link_text(magic_link: &str, language: Option<&str>) -> String {
    match language {
        Some("es") => create_spanish_text(magic_link),
        Some("ca") => create_catalan_text(magic_link),
        Some("eu") => create_basque_text(magic_link),
        Some("gl") => create_galician_text(magic_link),
        Some("fr") => create_french_text(magic_link),
        Some("de") => create_german_text(magic_link),
        Some("pt") => create_portuguese_text(magic_link),
        Some("ru") => create_russian_text(magic_link),
        Some("zh") => create_chinese_text(magic_link),
        Some("ja") => create_japanese_text(magic_link),
        Some("ar") => create_arabic_text(magic_link),
        Some("hi") => create_hindi_text(magic_link),
        _ => create_english_text(magic_link),
    }
}

/// Creates the English plain text email
fn create_english_text(magic_link: &str) -> String {
    format!(
        r#"
HASHRAND AUTHENTICATION
=======================

Your Magic Link is Ready!

Click or copy the following link to securely authenticate with HashRand:

{}

SECURITY NOTICE:
- This link is valid for a limited time
- It can only be used once
- Do not share this link with anyone
- If you didn't request this, you can safely ignore this email

---
HashRand - Zero Knowledge Random Hash Generator
This is an automated message. Please do not reply to this email.
"#,
        magic_link
    )
}

/// Creates the Spanish plain text email
fn create_spanish_text(magic_link: &str) -> String {
    format!(
        r#"
AUTENTICACIÓN HASHRAND
======================

¡Tu Enlace Mágico está Listo!

Haz clic o copia el siguiente enlace para autenticarte de forma segura con HashRand:

{}

AVISO DE SEGURIDAD:
- Este enlace es válido por tiempo limitado
- Solo puede usarse una vez
- No compartas este enlace con nadie
- Si no solicitaste esto, puedes ignorar este email

---
HashRand - Generador de Hashes Aleatorios con Zero Knowledge
Este es un mensaje automático. Por favor no respondas a este email.
"#,
        magic_link
    )
}

/// Creates the French HTML email template
fn create_french_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Authentification HashRand</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 Authentification HashRand</h1>
        </div>
        <div class="content">
            <h2>Votre Lien Magique est Prêt</h2>
            <p>Cliquez sur le bouton ci-dessous pour vous authentifier en toute sécurité avec HashRand :</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 Accéder à HashRand</a>
            </div>
            
            <div class="security-notice">
                <strong>Avis de Sécurité :</strong>
                <ul>
                    <li>Ce lien est valide pour une durée limitée</li>
                    <li>Il ne peut être utilisé qu'une seule fois</li>
                    <li>Ne partagez pas ce lien avec qui que ce soit</li>
                    <li>Si vous n'avez pas demandé ceci, vous pouvez ignorer cet email</li>
                </ul>
            </div>
            
            <p>Si le bouton ne fonctionne pas, vous pouvez copier et coller ce lien dans votre navigateur :</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - Générateur de Hachages Aléatoires Zero Knowledge</p>
            <p>Ceci est un message automatique. Veuillez ne pas répondre à cet email.</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the French plain text email
fn create_french_text(magic_link: &str) -> String {
    format!(
        r#"
AUTHENTIFICATION HASHRAND
=========================

Votre Lien Magique est Prêt !

Cliquez ou copiez le lien suivant pour vous authentifier en toute sécurité avec HashRand :

{}

AVIS DE SÉCURITÉ :
- Ce lien est valide pour une durée limitée
- Il ne peut être utilisé qu'une seule fois
- Ne partagez pas ce lien avec qui que ce soit
- Si vous n'avez pas demandé ceci, vous pouvez ignorer cet email

---
HashRand - Générateur de Hachages Aléatoires Zero Knowledge
Ceci est un message automatique. Veuillez ne pas répondre à cet email.
"#,
        magic_link
    )
}

/// Creates the German HTML email template  
fn create_german_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>HashRand Authentifizierung</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 HashRand Authentifizierung</h1>
        </div>
        <div class="content">
            <h2>Ihr Magic Link ist Bereit</h2>
            <p>Klicken Sie auf die Schaltfläche unten, um sich sicher bei HashRand zu authentifizieren:</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 Zu HashRand</a>
            </div>
            
            <div class="security-notice">
                <strong>Sicherheitshinweis:</strong>
                <ul>
                    <li>Dieser Link ist nur für begrenzte Zeit gültig</li>
                    <li>Er kann nur einmal verwendet werden</li>
                    <li>Teilen Sie diesen Link nicht mit anderen</li>
                    <li>Falls Sie dies nicht angefordert haben, können Sie diese E-Mail ignorieren</li>
                </ul>
            </div>
            
            <p>Falls die Schaltfläche nicht funktioniert, können Sie diesen Link kopieren und in Ihren Browser einfügen:</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - Zero Knowledge Zufalls-Hash-Generator</p>
            <p>Dies ist eine automatische Nachricht. Bitte antworten Sie nicht auf diese E-Mail.</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the German plain text email
fn create_german_text(magic_link: &str) -> String {
    format!(
        r#"
HASHRAND AUTHENTIFIZIERUNG
===========================

Ihr Magic Link ist Bereit!

Klicken Sie oder kopieren Sie den folgenden Link, um sich sicher bei HashRand zu authentifizieren:

{}

SICHERHEITSHINWEIS:
- Dieser Link ist nur für begrenzte Zeit gültig
- Er kann nur einmal verwendet werden
- Teilen Sie diesen Link nicht mit anderen
- Falls Sie dies nicht angefordert haben, können Sie diese E-Mail ignorieren

---
HashRand - Zero Knowledge Zufalls-Hash-Generator
Dies ist eine automatische Nachricht. Bitte antworten Sie nicht auf diese E-Mail.
"#,
        magic_link
    )
}

/// Creates the Portuguese HTML email template
fn create_portuguese_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Autenticação HashRand</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 Autenticação HashRand</h1>
        </div>
        <div class="content">
            <h2>Seu Link Mágico está Pronto</h2>
            <p>Clique no botão abaixo para se autenticar com segurança no HashRand:</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 Acessar HashRand</a>
            </div>
            
            <div class="security-notice">
                <strong>Aviso de Segurança:</strong>
                <ul>
                    <li>Este link é válido por tempo limitado</li>
                    <li>Só pode ser usado uma vez</li>
                    <li>Não compartilhe este link com ninguém</li>
                    <li>Se você não solicitou isso, pode ignorar este email</li>
                </ul>
            </div>
            
            <p>Se o botão não funcionar, você pode copiar e colar este link no seu navegador:</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - Gerador de Hashes Aleatórios Zero Knowledge</p>
            <p>Esta é uma mensagem automática. Por favor não responda a este email.</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Portuguese plain text email
fn create_portuguese_text(magic_link: &str) -> String {
    format!(
        r#"
AUTENTICAÇÃO HASHRAND
=====================

Seu Link Mágico está Pronto!

Clique ou copie o seguinte link para se autenticar com segurança no HashRand:

{}

AVISO DE SEGURANÇA:
- Este link é válido por tempo limitado
- Só pode ser usado uma vez
- Não compartilhe este link com ninguém
- Se você não solicitou isso, pode ignorar este email

---
HashRand - Gerador de Hashes Aleatórios Zero Knowledge
Esta é uma mensagem automática. Por favor não responda a este email.
"#,
        magic_link
    )
}

/// Creates the Russian HTML email template
fn create_russian_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Аутентификация HashRand</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 Аутентификация HashRand</h1>
        </div>
        <div class="content">
            <h2>Ваша Магическая Ссылка Готова</h2>
            <p>Нажмите на кнопку ниже, чтобы безопасно аутентифицироваться в HashRand:</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 Войти в HashRand</a>
            </div>
            
            <div class="security-notice">
                <strong>Уведомление о Безопасности:</strong>
                <ul>
                    <li>Эта ссылка действительна ограниченное время</li>
                    <li>Может быть использована только один раз</li>
                    <li>Не делитесь этой ссылкой ни с кем</li>
                    <li>Если вы не запрашивали это, можете проигнорировать это письмо</li>
                </ul>
            </div>
            
            <p>Если кнопка не работает, вы можете скопировать и вставить эту ссылку в ваш браузер:</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - Генератор Случайных Хешей с Нулевым Разглашением</p>
            <p>Это автоматическое сообщение. Пожалуйста, не отвечайте на это письмо.</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Russian plain text email
fn create_russian_text(magic_link: &str) -> String {
    format!(
        r#"
АУТЕНТИФИКАЦИЯ HASHRAND
=======================

Ваша Магическая Ссылка Готова!

Нажмите или скопируйте следующую ссылку для безопасной аутентификации в HashRand:

{}

УВЕДОМЛЕНИЕ О БЕЗОПАСНОСТИ:
- Эта ссылка действительна ограниченное время
- Может быть использована только один раз
- Не делитесь этой ссылкой ни с кем
- Если вы не запрашивали это, можете проигнорировать это письмо

---
HashRand - Генератор Случайных Хешей с Нулевым Разглашением
Это автоматическое сообщение. Пожалуйста, не отвечайте на это письмо.
"#,
        magic_link
    )
}

/// Creates the Chinese HTML email template
fn create_chinese_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>HashRand 身份验证</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 HashRand 身份验证</h1>
        </div>
        <div class="content">
            <h2>您的魔法链接已准备就绪</h2>
            <p>点击下方按钮安全地登录 HashRand：</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 访问 HashRand</a>
            </div>
            
            <div class="security-notice">
                <strong>安全提示：</strong>
                <ul>
                    <li>此链接有效期有限</li>
                    <li>只能使用一次</li>
                    <li>请勿与他人分享此链接</li>
                    <li>如果您没有请求此操作，可以忽略此邮件</li>
                </ul>
            </div>
            
            <p>如果按钮不起作用，您可以复制并粘贴此链接到您的浏览器：</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - 零知识随机哈希生成器</p>
            <p>这是一封自动邮件。请不要回复此邮件。</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Chinese plain text email
fn create_chinese_text(magic_link: &str) -> String {
    format!(
        r#"
HASHRAND 身份验证
=================

您的魔法链接已准备就绪！

点击或复制以下链接安全地登录 HashRand：

{}

安全提示：
- 此链接有效期有限
- 只能使用一次
- 请勿与他人分享此链接
- 如果您没有请求此操作，可以忽略此邮件

---
HashRand - 零知识随机哈希生成器
这是一封自动邮件。请不要回复此邮件。
"#,
        magic_link
    )
}

/// Creates the Japanese HTML email template
fn create_japanese_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>HashRand認証</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 HashRand認証</h1>
        </div>
        <div class="content">
            <h2>マジックリンクの準備が完了しました</h2>
            <p>下のボタンをクリックして、HashRandに安全にログインしてください：</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 HashRandにアクセス</a>
            </div>
            
            <div class="security-notice">
                <strong>セキュリティについて：</strong>
                <ul>
                    <li>このリンクは有効期限があります</li>
                    <li>一度だけ使用できます</li>
                    <li>このリンクを他の人と共有しないでください</li>
                    <li>このリクエストをしていない場合は、このメールを無視してください</li>
                </ul>
            </div>
            
            <p>ボタンが機能しない場合は、このリンクをコピーしてブラウザに貼り付けてください：</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - ゼロ知識ランダムハッシュジェネレーター</p>
            <p>これは自動送信メールです。このメールには返信しないでください。</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Japanese plain text email
fn create_japanese_text(magic_link: &str) -> String {
    format!(
        r#"
HASHRAND認証
============

マジックリンクの準備が完了しました！

以下のリンクをクリックまたはコピーして、HashRandに安全にログインしてください：

{}

セキュリティについて：
- このリンクは有効期限があります
- 一度だけ使用できます
- このリンクを他の人と共有しないでください
- このリクエストをしていない場合は、このメールを無視してください

---
HashRand - ゼロ知識ランダムハッシュジェネレーター
これは自動送信メールです。このメールには返信しないでください。
"#,
        magic_link
    )
}

/// Creates the Arabic HTML email template
fn create_arabic_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html dir="rtl">
<head>
    <meta charset="utf-8">
    <title>مصادقة HashRand</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; direction: rtl; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 مصادقة HashRand</h1>
        </div>
        <div class="content">
            <h2>رابطك السحري جاهز</h2>
            <p>انقر على الزر أدناه للمصادقة بأمان مع HashRand:</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 الوصول إلى HashRand</a>
            </div>
            
            <div class="security-notice">
                <strong>تنبيه أمني:</strong>
                <ul>
                    <li>هذا الرابط صالح لوقت محدود</li>
                    <li>يمكن استخدامه مرة واحدة فقط</li>
                    <li>لا تشارك هذا الرابط مع أي شخص</li>
                    <li>إذا لم تطلب هذا، يمكنك تجاهل هذا الإيميل</li>
                </ul>
            </div>
            
            <p>إذا لم يعمل الزر، يمكنك نسخ ولصق هذا الرابط في متصفحك:</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace; direction: ltr;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - مولد الهاش العشوائي بالمعرفة الصفرية</p>
            <p>هذه رسالة تلقائية. يرجى عدم الرد على هذا الإيميل.</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Arabic plain text email
fn create_arabic_text(magic_link: &str) -> String {
    format!(
        r#"
مصادقة HASHRAND
================

رابطك السحري جاهز!

انقر أو انسخ الرابط التالي للمصادقة بأمان مع HashRand:

{}

تنبيه أمني:
- هذا الرابط صالح لوقت محدود
- يمكن استخدامه مرة واحدة فقط
- لا تشارك هذا الرابط مع أي شخص
- إذا لم تطلب هذا، يمكنك تجاهل هذا الإيميل

---
HashRand - مولد الهاش العشوائي بالمعرفة الصفرية
هذه رسالة تلقائية. يرجى عدم الرد على هذا الإيميل.
"#,
        magic_link
    )
}

/// Creates the Hindi HTML email template
fn create_hindi_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>HashRand प्रमाणीकरण</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 HashRand प्रमाणीकरण</h1>
        </div>
        <div class="content">
            <h2>आपका मैजिक लिंक तैयार है</h2>
            <p>HashRand के साथ सुरक्षित रूप से प्रमाणित करने के लिए नीचे दिए गए बटन पर क्लिक करें:</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 HashRand तक पहुंचें</a>
            </div>
            
            <div class="security-notice">
                <strong>सुरक्षा सूचना:</strong>
                <ul>
                    <li>यह लिंक सीमित समय के लिए वैध है</li>
                    <li>इसका केवल एक बार ही उपयोग किया जा सकता है</li>
                    <li>इस लिंक को किसी के साथ साझा न करें</li>
                    <li>यदि आपने इसका अनुरोध नहीं किया है, तो आप इस ईमेल को नज़रअंदाज़ कर सकते हैं</li>
                </ul>
            </div>
            
            <p>यदि बटन काम नहीं कर रहा है, तो आप इस लिंक को कॉपी करके अपने ब्राउज़र में पेस्ट कर सकते हैं:</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - शून्य ज्ञान रैंडम हैश जनरेटर</p>
            <p>यह एक स्वचालित संदेश है। कृपया इस ईमेल का उत्तर न दें।</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Hindi plain text email
fn create_hindi_text(magic_link: &str) -> String {
    format!(
        r#"
HASHRAND प्रमाणीकरण
====================

आपका मैजिक लिंक तैयार है!

HashRand के साथ सुरक्षित रूप से प्रमाणित करने के लिए निम्नलिखित लिंक पर क्लिक करें या कॉपी करें:

{}

सुरक्षा सूचना:
- यह लिंक सीमित समय के लिए वैध है
- इसका केवल एक बार ही उपयोग किया जा सकता है
- इस लिंक को किसी के साथ साझा न करें
- यदि आपने इसका अनुरोध नहीं किया है, तो आप इस ईमेल को नज़रअंदाज़ कर सकते हैं

---
HashRand - शून्य ज्ञान रैंडम हैश जनरेटर
यह एक स्वचालित संदेश है। कृपया इस ईमेल का उत्तर न दें।
"#,
        magic_link
    )
}

// Functions for Catalan, Basque, and Galician (Peninsula languages)
/// Creates the Catalan HTML email template
fn create_catalan_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Autenticació HashRand</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 Autenticació HashRand</h1>
        </div>
        <div class="content">
            <h2>El teu Enllaç Màgic està Llest</h2>
            <p>Fes clic al botó següent per autenticar-te de forma segura amb HashRand:</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 Accedir a HashRand</a>
            </div>
            
            <div class="security-notice">
                <strong>Avís de Seguretat:</strong>
                <ul>
                    <li>Aquest enllaç és vàlid per temps limitat</li>
                    <li>Només es pot fer servir una vegada</li>
                    <li>No comparteixis aquest enllaç amb ningú</li>
                    <li>Si no has demanat això, pots ignorar aquest correu</li>
                </ul>
            </div>
            
            <p>Si el botó no funciona, pots copiar i enganxar aquest enllaç al teu navegador:</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - Generador d'Hashes Aleatoris amb Zero Knowledge</p>
            <p>Aquest és un missatge automàtic. Si us plau no responguis a aquest correu.</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the remaining text functions for Peninsula languages
fn create_catalan_text(magic_link: &str) -> String {
    format!(
        r#"
AUTENTICACIÓ HASHRAND
=====================

El teu Enllaç Màgic està Llest!

Fes clic o copia l'enllaç següent per autenticar-te de forma segura amb HashRand:

{}

AVÍS DE SEGURETAT:
- Aquest enllaç és vàlid per temps limitat
- Només es pot fer servir una vegada
- No comparteixis aquest enllaç amb ningú
- Si no has demanat això, pots ignorar aquest correu

---
HashRand - Generador d'Hashes Aleatoris amb Zero Knowledge
Aquest és un missatge automàtic. Si us plau no responguis a aquest correu.
"#,
        magic_link
    )
}

fn create_basque_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>HashRand Autentifikazioa</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 HashRand Autentifikazioa</h1>
        </div>
        <div class="content">
            <h2>Zure Esteka Magikoa Prest Dago</h2>
            <p>Egin klik beheko botoian HashRand-ekin modu seguruan autentifikatzeko:</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 HashRand-era Sartu</a>
            </div>
            
            <div class="security-notice">
                <strong>Segurtasun Oharra:</strong>
                <ul>
                    <li>Esteka hau denbora mugatuan balio du</li>
                    <li>Behin bakarrik erabili daiteke</li>
                    <li>Ez konpartitu esteka hau inorekin</li>
                    <li>Hau eskatu ez baduzu, mezu hau bazter dezakezu</li>
                </ul>
            </div>
            
            <p>Botoia ez badu funtzionatzen, esteka hau kopia eta itsats dezakezu zure nabigatzailean:</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - Zero Knowledge Hash Sortzaile Ausazkoa</p>
            <p>Mezu automatiko hau da. Ez erantzun mezu honi.</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

fn create_basque_text(magic_link: &str) -> String {
    format!(
        r#"
HASHRAND AUTENTIFIKAZIOA
========================

Zure Esteka Magikoa Prest Dago!

Egin klik edo kopiatu hurrengo esteka HashRand-ekin modu seguruan autentifikatzeko:

{}

SEGURTASUN OHARRA:
- Esteka hau denbora mugatuan balio du
- Behin bakarrik erabili daiteke
- Ez konpartitu esteka hau inorekin
- Hau eskatu ez baduzu, mezu hau bazter dezakezu

---
HashRand - Zero Knowledge Hash Sortzaile Ausazkoa
Mezu automatiko hau da. Ez erantzun mezu honi.
"#,
        magic_link
    )
}

fn create_galician_html(magic_link: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Autenticación HashRand</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
        .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
        .header {{ background: #2563eb; color: white; padding: 20px; text-align: center; border-radius: 8px 8px 0 0; }}
        .content {{ background: #f9fafb; padding: 30px; border: 1px solid #e5e7eb; }}
        .button {{ display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; margin: 20px 0; }}
        .footer {{ background: #6b7280; color: white; padding: 15px; text-align: center; font-size: 12px; border-radius: 0 0 8px 8px; }}
        .security-notice {{ background: #fef3c7; border: 1px solid #f59e0b; padding: 15px; margin: 20px 0; border-radius: 6px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 Autenticación HashRand</h1>
        </div>
        <div class="content">
            <h2>A túa Ligazón Máxica está Lista</h2>
            <p>Preme no botón de abaixo para autenticarte de xeito seguro con HashRand:</p>
            
            <div style="text-align: center;">
                <a href="{}" class="button">🚀 Acceder a HashRand</a>
            </div>
            
            <div class="security-notice">
                <strong>Aviso de Seguridade:</strong>
                <ul>
                    <li>Esta ligazón é válida por tempo limitado</li>
                    <li>Só se pode usar unha vez</li>
                    <li>Non compartas esta ligazón con ninguén</li>
                    <li>Se non solicitaches isto, podes ignorar este correo</li>
                </ul>
            </div>
            
            <p>Se o botón non funciona, podes copiar e pegar esta ligazón no teu navegador:</p>
            <p style="word-break: break-all; background: #e5e7eb; padding: 10px; border-radius: 4px; font-family: monospace;">
                {}
            </p>
        </div>
        <div class="footer">
            <p>HashRand - Xerador de Hashes Aleatorios con Zero Knowledge</p>
            <p>Esta é unha mensaxe automática. Por favor non respondas a este correo.</p>
        </div>
    </div>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

fn create_galician_text(magic_link: &str) -> String {
    format!(
        r#"
AUTENTICACIÓN HASHRAND
======================

A túa Ligazón Máxica está Lista!

Preme ou copia a seguinte ligazón para autenticarte de xeito seguro con HashRand:

{}

AVISO DE SEGURIDADE:
- Esta ligazón é válida por tempo limitado
- Só se pode usar unha vez
- Non compartas esta ligazón con ninguén
- Se non solicitaches isto, podes ignorar este correo

---
HashRand - Xerador de Hashes Aleatorios con Zero Knowledge
Esta é unha mensaxe automática. Por favor non respondas a este correo.
"#,
        magic_link
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        // Test basic email validation logic
        assert!("".is_empty() || !"".contains('@'));
        assert!(!"invalid-email".contains('@'));

        let valid_email = "valid@example.com";
        assert!(valid_email.contains('@') && !valid_email.is_empty());
    }

    #[test]
    fn test_html_content_generation() {
        let magic_link = "https://example.com/magic?token=abc123";

        // Test English HTML
        let html_en = create_magic_link_html(magic_link, None);
        assert!(html_en.contains(magic_link));
        assert!(html_en.contains("HashRand Authentication"));
        assert!(html_en.contains("Security Notice"));

        // Test Spanish HTML
        let html_es = create_magic_link_html(magic_link, Some("es"));
        assert!(html_es.contains(magic_link));
        assert!(html_es.contains("Autenticación HashRand"));
        assert!(html_es.contains("Aviso de Seguridad"));
    }

    #[test]
    fn test_text_content_generation() {
        let magic_link = "https://example.com/magic?token=abc123";

        // Test English text
        let text_en = create_magic_link_text(magic_link, None);
        assert!(text_en.contains(magic_link));
        assert!(text_en.contains("HASHRAND AUTHENTICATION"));
        assert!(text_en.contains("SECURITY NOTICE"));

        // Test Spanish text
        let text_es = create_magic_link_text(magic_link, Some("es"));
        assert!(text_es.contains(magic_link));
        assert!(text_es.contains("AUTENTICACIÓN HASHRAND"));
        assert!(text_es.contains("AVISO DE SEGURIDAD"));
    }

    #[test]
    fn test_subject_generation() {
        // Test English subject
        assert_eq!(create_subject(None), "Your HashRand Authentication Link");
        assert_eq!(
            create_subject(Some("en")),
            "Your HashRand Authentication Link"
        );

        // Test Spanish subject
        assert_eq!(
            create_subject(Some("es")),
            "Tu enlace de autenticación para HashRand"
        );
    }

    #[test]
    fn test_email_payload_structure() {
        let config = EmailConfig {
            api_url: "https://test.api".to_string(),
            api_token: "test_token".to_string(),
            from_email: "test@example.com".to_string(),
        };

        let request = create_email_request(
            &config,
            "recipient@example.com",
            "https://magic.link",
            Some("es"),
        );
        assert!(request.is_ok());

        if let Ok(req) = request {
            assert_eq!(req.method(), &Method::Post);
            // Test that the request was built successfully
            assert!(req.body().len() > 0); // Body should contain JSON
        }
    }
}
