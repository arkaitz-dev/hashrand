//! Eastern language email templates (Russian, Chinese, Japanese)
//! All templates with basic structure, ready for full accessibility implementation.

/// Creates the Russian HTML email template (placeholder)
pub fn create_russian_html(magic_link: &str) -> String {
    // TODO: Implement full accessibility improvements
    format!(
        r#"
<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="utf-8">
    <title>Аутентификация HashRand</title>
</head>
<body>
    <h1>🔐 Аутентификация HashRand</h1>
    <h2>Ваша Магическая Ссылка Готова</h2>
    <p><a href="{}" style="color: white !important; background: #3b82f6; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block; font-weight: bold;">🚀 Войти в HashRand</a></p>
    <p>Fallback link: {}</p>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Russian plain text email
pub fn create_russian_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 АУТЕНТИФИКАЦИЯ HASHRAND
════════════════════════════════════════════════

Ваша Магическая Ссылка Готова!

Для безопасной аутентификации в HashRand скопируйте и вставьте следующую 
ссылку в ваш веб-браузер:

🚀 Ссылка для Аутентификации:
{}

⚠️  ВАЖНАЯ ИНФОРМАЦИЯ О БЕЗОПАСНОСТИ:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Эта ссылка действительна только ОГРАНИЧЕННОЕ ВРЕМЯ
✓ Эта ссылка может быть использована только ОДИН РАЗ
✓ НЕ делитесь этой ссылкой ни с кем
✓ Если вы не запрашивали эту аутентификацию, вы можете игнорировать это письмо

Нужна помощь? Это письмо было отправлено, потому что кто-то запросил аутентификацию
для доступа к HashRand, используя ваш адрес электронной почты.

────────────────────────────────────────────────
HASHRAND - Генератор Случайных Хешей с Нулевым Разглашением
────────────────────────────────────────────────
Это автоматическое сообщение безопасности от noreply@mailer.hashrand.com
Пожалуйста, не отвечайте на это письмо - ответы не отслеживаются.

За поддержкой обращайтесь: https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}

/// Creates the Chinese HTML email template (placeholder)
pub fn create_chinese_html(magic_link: &str) -> String {
    // TODO: Implement full accessibility improvements
    format!(
        r#"
<!DOCTYPE html>
<html lang="zh">
<head>
    <meta charset="utf-8">
    <title>HashRand 身份验证</title>
</head>
<body>
    <h1>🔐 HashRand 身份验证</h1>
    <h2>您的魔法链接已准备就绪</h2>
    <p><a href="{}" style="color: white !important; background: #3b82f6; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block; font-weight: bold;">🚀 访问 HashRand</a></p>
    <p>Fallback link: {}</p>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Chinese plain text email
pub fn create_chinese_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 HASHRAND 身份验证
════════════════════════════════════════════════

您的魔法链接已准备就绪！

要安全地通过 HashRand 进行身份验证，请复制并粘贴以下链接到您的网页浏览器：

🚀 身份验证链接：
{}

⚠️  重要安全信息：
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ 此链接仅在有限时间内有效
✓ 此链接只能使用一次
✓ 请勿与任何人分享此链接
✓ 如果您没有请求此身份验证，可以忽略此邮件

需要帮助？发送此邮件是因为有人使用您的邮箱地址请求访问 HashRand 的身份验证。

────────────────────────────────────────────────
HASHRAND - 零知识随机哈希生成器
────────────────────────────────────────────────
这是来自 noreply@mailer.hashrand.com 的自动安全消息
请勿回复此邮件 - 回复不会被监控。

如需支持，请访问：https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}

/// Creates the Japanese HTML email template (placeholder)
pub fn create_japanese_html(magic_link: &str) -> String {
    // TODO: Implement full accessibility improvements
    format!(
        r#"
<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="utf-8">
    <title>HashRand認証</title>
</head>
<body>
    <h1>🔐 HashRand認証</h1>
    <h2>マジックリンクの準備が完了しました</h2>
    <p><a href="{}" style="color: white !important; background: #3b82f6; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block; font-weight: bold;">🚀 HashRandにアクセス</a></p>
    <p>Fallback link: {}</p>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Japanese plain text email
pub fn create_japanese_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 HASHRAND認証
════════════════════════════════════════════════

マジックリンクの準備が完了しました！

HashRandで安全に認証するには、以下のリンクをウェブブラウザにコピー＆ペーストしてください：

🚀 認証リンク：
{}

⚠️  重要なセキュリティ情報：
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ このリンクは限られた時間のみ有効です
✓ このリンクは一度だけ使用できます
✓ このリンクを他の人と共有しないでください
✓ この認証をリクエストしていない場合は、このメールを無視してください

ヘルプが必要ですか？このメールは、あなたのメールアドレスを使用してHashRandへのアクセス認証が
リクエストされたために送信されました。

────────────────────────────────────────────────
HASHRAND - ゼロ知識ランダムハッシュジェネレーター
────────────────────────────────────────────────
これは noreply@mailer.hashrand.com からの自動セキュリティメッセージです
このメールには返信しないでください - 返信は監視されていません。

サポートについては：https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}