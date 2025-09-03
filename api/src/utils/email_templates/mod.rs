//! Email template modules for HashRand authentication emails
//! 
//! This module contains all email templates organized by language groups:
//! - Primary: English and Spanish (most used)
//! - Western European: French, German, Portuguese  
//! - Eastern: Russian, Chinese, Japanese
//! - RTL & Other: Arabic, Hindi
//! - Peninsula: Catalan, Basque, Galician

pub mod primary;
pub mod western_european;
pub mod eastern;
pub mod rtl_other;
pub mod peninsula;

use primary::*;
use western_european::*;
use eastern::*;
use rtl_other::*;
use peninsula::*;

/// Creates the appropriate HTML email template based on language
pub fn create_html_template(magic_link: &str, language: Option<&str>) -> String {
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
        _ => create_english_html(magic_link), // Default to English
    }
}

/// Creates the appropriate plain text email template based on language
pub fn create_text_template(magic_link: &str, language: Option<&str>) -> String {
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
        _ => create_english_text(magic_link), // Default to English
    }
}

/// Creates the appropriate email subject based on language
pub fn create_subject(language: Option<&str>) -> &'static str {
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