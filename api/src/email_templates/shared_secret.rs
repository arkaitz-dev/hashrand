use maud::{DOCTYPE, PreEscaped, html};
use rust_i18n::t;

/// Render shared secret receiver email using Maud template with i18n support
///
/// # Arguments
/// * `secret_url` - The complete secret URL for the receiver
/// * `reference` - The reference hash (Base58)
/// * `sender_email` - Email of the sender
/// * `expires_hours` - Expiration time in hours
/// * `max_reads` - Maximum number of reads allowed
/// * `language` - Language code (e.g., "en", "es", "eu")
///
/// # Returns
/// * (subject, html_body, text_body) tuple
#[allow(dead_code)]
pub fn render_shared_secret_receiver_email(
    secret_url: &str,
    reference: &str,
    sender_email: &str,
    expires_hours: i64,
    max_reads: i64,
    language: &str,
) -> (String, String, String) {
    // Set the locale for this email
    rust_i18n::set_locale(language);

    let subject = format!(
        "{} [Ref: {}]",
        t!("email.shared_secret.receiver.subject"),
        reference
    );
    let html_body = render_receiver_html_body(
        secret_url,
        reference,
        sender_email,
        expires_hours,
        max_reads,
        language,
    );
    let text_body = render_receiver_text_body(
        secret_url,
        reference,
        sender_email,
        expires_hours,
        max_reads,
        language,
    );

    (subject, html_body, text_body)
}

/// Render shared secret sender email (copy) using Maud template with i18n support
///
/// # Arguments
/// * `secret_url` - The complete secret URL for the sender
/// * `reference` - The reference hash (Base58)
/// * `receiver_email` - Email of the receiver
/// * `expires_hours` - Expiration time in hours
/// * `language` - Language code (e.g., "en", "es", "eu")
///
/// # Returns
/// * (subject, html_body, text_body) tuple
#[allow(dead_code)]
pub fn render_shared_secret_sender_email(
    secret_url: &str,
    reference: &str,
    receiver_email: &str,
    expires_hours: i64,
    language: &str,
) -> (String, String, String) {
    // Set the locale for this email
    rust_i18n::set_locale(language);

    let subject = format!(
        "{} [Ref: {}]",
        t!("email.shared_secret.sender.subject"),
        reference
    );
    let html_body = render_sender_html_body(
        secret_url,
        reference,
        receiver_email,
        expires_hours,
        language,
    );
    let text_body = render_sender_text_body(
        secret_url,
        reference,
        receiver_email,
        expires_hours,
        language,
    );

    (subject, html_body, text_body)
}

#[allow(dead_code)]
fn render_receiver_html_body(
    secret_url: &str,
    reference: &str,
    sender_email: &str,
    expires_hours: i64,
    max_reads: i64,
    language: &str,
) -> String {
    // RTL languages that need right-to-left text direction
    let is_rtl = matches!(language, "ar" | "he" | "fa" | "ur");

    let markup = html! {
        (DOCTYPE)
        html lang=(language) dir=(if is_rtl { "rtl" } else { "ltr" }) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                title { (t!("email.shared_secret.receiver.subject")) " [Ref: " (reference) "]" }
                style type="text/css" {
                    (PreEscaped(include_str!("email_styles.css")))
                }
            }
            body {
                div.email-container {
                    div.email-header {
                        h1 { (t!("email.shared_secret.receiver.title")) }
                        p { (t!("email.shared_secret.receiver.subtitle")) }
                    }

                    div.email-body {
                        p.greeting { (t!("email.shared_secret.receiver.greeting")) }

                        p.intro-text {
                            (t!("email.shared_secret.receiver.intro", sender = sender_email))
                        }

                        div.security-info style="background: #f3f4f6; padding: 15px; border-radius: 8px; margin: 20px 0;" {
                            p style="margin: 5px 0;" {
                                "📧 " strong { (t!("email.shared_secret.receiver.sender_label")) ": " } (sender_email)
                            }
                            p style="margin: 5px 0;" {
                                "🔢 " strong { (t!("email.shared_secret.receiver.reference_label")) ": " } code { (reference) }
                            }
                            p style="margin: 5px 0;" {
                                "⏰ " strong { (t!("email.shared_secret.receiver.expires_label")) ": " }
                                (t!("email.shared_secret.receiver.expires_value", hours = expires_hours))
                            }
                            p style="margin: 5px 0;" {
                                "👀 " strong { (t!("email.shared_secret.receiver.reads_label")) ": " }
                                (t!("email.shared_secret.receiver.reads_value", reads = max_reads))
                            }
                        }

                        div style="text-align: center; margin: 30px 0;" {
                            a.action-button href=(secret_url) {
                                (t!("email.shared_secret.receiver.button_text"))
                            }
                        }

                        div.manual-link {
                            p { (t!("email.shared_secret.receiver.manual_link_intro")) }
                            code { (secret_url) }
                        }

                        div.security-info {
                            p { "⚠️ " (t!("email.shared_secret.receiver.security_warning")) }
                        }

                        p.security-notice {
                            "🔒 " (t!("email.shared_secret.receiver.security_notice"))
                        }
                    }

                    div.email-footer {
                        p.footer-text { (t!("email.shared_secret.receiver.footer_text")) }
                        p.no-reply-notice { (t!("email.shared_secret.receiver.no_reply_notice")) }
                    }
                }
            }
        }
    };

    markup.into_string()
}

#[allow(dead_code)]
fn render_receiver_text_body(
    secret_url: &str,
    reference: &str,
    sender_email: &str,
    expires_hours: i64,
    max_reads: i64,
    language: &str,
) -> String {
    // Ensure locale is set for this text rendering
    rust_i18n::set_locale(language);

    format!(
        r#"{title} - {subtitle}
{separator}

{greeting}

{intro_text}

{info_section}
📧 {sender_label}: {sender_email}
🔢 {reference_label}: {reference}
⏰ {expires_label}: {expires_value}
👀 {reads_label}: {reads_value}

{access_instructions}
{secret_url}

{security_section}
• {security_warning}

• {security_notice}

{footer_separator}
{footer_text}
{no_reply_notice}
        "#,
        title = t!("email.shared_secret.receiver.title"),
        subtitle = t!("email.shared_secret.receiver.subtitle"),
        separator = "=".repeat(50),
        greeting = t!("email.shared_secret.receiver.greeting"),
        intro_text = t!(
            "email.shared_secret.receiver.text_intro",
            sender = sender_email
        ),
        info_section = t!("email.shared_secret.receiver.text_info_section"),
        sender_label = t!("email.shared_secret.receiver.sender_label"),
        reference_label = t!("email.shared_secret.receiver.reference_label"),
        expires_label = t!("email.shared_secret.receiver.expires_label"),
        expires_value = t!(
            "email.shared_secret.receiver.expires_value",
            hours = expires_hours
        ),
        reads_label = t!("email.shared_secret.receiver.reads_label"),
        reads_value = t!(
            "email.shared_secret.receiver.reads_value",
            reads = max_reads
        ),
        access_instructions = format_args!(
            ">> {} <<",
            t!("email.shared_secret.receiver.text_access_label")
        ),
        security_section = t!("email.shared_secret.receiver.text_security_section"),
        security_warning = t!("email.shared_secret.receiver.security_warning"),
        security_notice = t!("email.shared_secret.receiver.security_notice"),
        footer_separator = "-".repeat(50),
        footer_text = t!("email.shared_secret.receiver.footer_text"),
        no_reply_notice = t!("email.shared_secret.receiver.no_reply_notice"),
        sender_email = sender_email,
        reference = reference,
        secret_url = secret_url
    )
}

#[allow(dead_code)]
fn render_sender_html_body(
    secret_url: &str,
    reference: &str,
    receiver_email: &str,
    expires_hours: i64,
    language: &str,
) -> String {
    // RTL languages that need right-to-left text direction
    let is_rtl = matches!(language, "ar" | "he" | "fa" | "ur");

    let markup = html! {
        (DOCTYPE)
        html lang=(language) dir=(if is_rtl { "rtl" } else { "ltr" }) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                title { (t!("email.shared_secret.sender.subject")) " [Ref: " (reference) "]" }
                style type="text/css" {
                    (PreEscaped(include_str!("email_styles.css")))
                }
            }
            body {
                div.email-container {
                    div.email-header {
                        h1 { (t!("email.shared_secret.sender.title")) }
                        p { (t!("email.shared_secret.sender.subtitle")) }
                    }

                    div.email-body {
                        p.greeting { (t!("email.shared_secret.sender.greeting")) }

                        p.intro-text {
                            (t!("email.shared_secret.sender.intro", receiver = receiver_email))
                        }

                        div.security-info style="background: #f3f4f6; padding: 15px; border-radius: 8px; margin: 20px 0;" {
                            p style="margin: 5px 0;" {
                                "📧 " strong { (t!("email.shared_secret.sender.receiver_label")) ": " } (receiver_email)
                            }
                            p style="margin: 5px 0;" {
                                "🔢 " strong { (t!("email.shared_secret.sender.reference_label")) ": " } code { (reference) }
                            }
                            p style="margin: 5px 0;" {
                                "⏰ " strong { (t!("email.shared_secret.sender.expires_label")) ": " }
                                (t!("email.shared_secret.sender.expires_value", hours = expires_hours))
                            }
                        }

                        div style="text-align: center; margin: 30px 0;" {
                            a.action-button href=(secret_url) {
                                (t!("email.shared_secret.sender.button_text"))
                            }
                        }

                        div.manual-link {
                            p { (t!("email.shared_secret.sender.manual_link_intro")) }
                            code { (secret_url) }
                        }

                        div.security-info {
                            p { "ℹ️ " (t!("email.shared_secret.sender.info_notice")) }
                        }

                        p.security-notice {
                            "🔒 " (t!("email.shared_secret.sender.security_notice"))
                        }
                    }

                    div.email-footer {
                        p.footer-text { (t!("email.shared_secret.sender.footer_text")) }
                        p.no-reply-notice { (t!("email.shared_secret.sender.no_reply_notice")) }
                    }
                }
            }
        }
    };

    markup.into_string()
}

#[allow(dead_code)]
fn render_sender_text_body(
    secret_url: &str,
    reference: &str,
    receiver_email: &str,
    expires_hours: i64,
    language: &str,
) -> String {
    // Ensure locale is set for this text rendering
    rust_i18n::set_locale(language);

    format!(
        r#"{title} - {subtitle}
{separator}

{greeting}

{intro_text}

{info_section}
📧 {receiver_label}: {receiver_email}
🔢 {reference_label}: {reference}
⏰ {expires_label}: {expires_value}

{access_instructions}
{secret_url}

{info_notice}

{security_notice}

{footer_separator}
{footer_text}
{no_reply_notice}
        "#,
        title = t!("email.shared_secret.sender.title"),
        subtitle = t!("email.shared_secret.sender.subtitle"),
        separator = "=".repeat(50),
        greeting = t!("email.shared_secret.sender.greeting"),
        intro_text = t!(
            "email.shared_secret.sender.text_intro",
            receiver = receiver_email
        ),
        info_section = t!("email.shared_secret.sender.text_info_section"),
        receiver_label = t!("email.shared_secret.sender.receiver_label"),
        reference_label = t!("email.shared_secret.sender.reference_label"),
        expires_label = t!("email.shared_secret.sender.expires_label"),
        expires_value = t!(
            "email.shared_secret.sender.expires_value",
            hours = expires_hours
        ),
        access_instructions = format_args!(
            ">> {} <<",
            t!("email.shared_secret.sender.text_access_label")
        ),
        info_notice = t!("email.shared_secret.sender.info_notice"),
        security_notice = t!("email.shared_secret.sender.security_notice"),
        footer_separator = "-".repeat(50),
        footer_text = t!("email.shared_secret.sender.footer_text"),
        no_reply_notice = t!("email.shared_secret.sender.no_reply_notice"),
        receiver_email = receiver_email,
        reference = reference,
        secret_url = secret_url
    )
}
