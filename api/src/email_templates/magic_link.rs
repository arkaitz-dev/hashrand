use maud::{DOCTYPE, PreEscaped, html};
use rust_i18n::t;

/// Render magic link email using Maud template with i18n support
///
/// # Arguments
/// * `magic_link` - The complete magic link URL
/// * `language` - Language code (e.g., "en", "es", "eu")
///
/// # Returns
/// * Complete HTML email as String
pub fn render_magic_link_email(magic_link: &str, language: &str) -> (String, String, String) {
    // Set the locale for this email
    rust_i18n::set_locale(language);

    let subject = t!("email.magic_link.subject").to_string();
    let html_body = render_html_body(magic_link, language);
    let text_body = render_text_body(magic_link, language);

    (subject, html_body, text_body)
}

fn render_html_body(magic_link: &str, language: &str) -> String {
    // RTL languages that need right-to-left text direction
    let is_rtl = matches!(language, "ar" | "he" | "fa" | "ur");

    let markup = html! {
        (DOCTYPE)
        html lang=(language) dir=(if is_rtl { "rtl" } else { "ltr" }) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                title { (t!("email.magic_link.subject")) }
                style type="text/css" {
                    (PreEscaped(include_str!("email_styles.css")))
                }
            }
            body {
                div.email-container {
                    div.email-header {
                        h1 { (t!("email.magic_link.title")) }
                        p { (t!("email.magic_link.subtitle")) }
                    }

                    div.email-body {
                        p.greeting { (t!("email.magic_link.greeting")) }

                        p.intro-text { (t!("email.magic_link.intro")) }

                        div style="text-align: center; margin: 30px 0;" {
                            a.action-button href=(magic_link) {
                                (t!("email.magic_link.button_text"))
                            }
                        }

                        div.manual-link {
                            p { (t!("email.magic_link.manual_link_intro")) }
                            code { (magic_link) }
                        }

                        div.security-info {
                            p { "⏰ " (t!("email.magic_link.security_warning")) }
                        }

                        p.security-notice {
                            "🔒 " (t!("email.magic_link.security_notice"))
                        }
                    }

                    div.email-footer {
                        p.footer-text { (t!("email.magic_link.footer_text")) }
                        p.no-reply-notice { (t!("email.magic_link.no_reply_notice")) }
                    }
                }
            }
        }
    };

    markup.into_string()
}

fn render_text_body(magic_link: &str, language: &str) -> String {
    // Ensure locale is set for this text rendering
    rust_i18n::set_locale(language);

    format!(
        r#"{title} - {subtitle}
{separator}

{greeting}

{intro_text}

{access_instructions}
{magic_link}

{security_section}
{security_warning}

{security_notice}

{footer_separator}
{footer_text}
{no_reply_notice}
        "#,
        title = t!("email.magic_link.title"),
        subtitle = t!("email.magic_link.subtitle"),
        separator = "=".repeat(50),
        greeting = t!("email.magic_link.greeting"),
        intro_text = t!("email.magic_link.text_intro"),
        access_instructions = format_args!(">> {} <<", t!("email.magic_link.text_access_label")),
        security_section = t!("email.magic_link.text_security_section"),
        security_warning = format_args!("• {}", t!("email.magic_link.security_warning")),
        security_notice = format_args!("• {}", t!("email.magic_link.security_notice")),
        footer_separator = "-".repeat(50),
        footer_text = t!("email.magic_link.footer_text"),
        no_reply_notice = t!("email.magic_link.no_reply_notice"),
        magic_link = magic_link
    )
}
