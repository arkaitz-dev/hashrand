//! RTL and other special script email templates (Arabic RTL, Hindi Devanagari)
//! Arabic includes proper RTL support with dir="rtl"
//! Hindi uses Devanagari script but is LTR

/// Creates the Arabic HTML email template (RTL) - (placeholder)
pub fn create_arabic_html(magic_link: &str) -> String {
    // TODO: Implement full accessibility improvements
    format!(
        r#"
<!DOCTYPE html>
<html lang="ar" dir="rtl">
<head>
    <meta charset="utf-8">
    <title>مصادقة HashRand</title>
    <style>
        body {{ direction: rtl; text-align: right; }}
    </style>
</head>
<body>
    <h1>🔐 مصادقة HashRand</h1>
    <h2>رابطك السحري جاهز</h2>
    <p><a href="{}" style="color: white !important; background: #3b82f6; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block; font-weight: bold;">🚀 الوصول إلى HashRand</a></p>
    <p style="direction: ltr; text-align: left;">Fallback link: {}</p>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Arabic plain text email (RTL)
pub fn create_arabic_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 مصادقة HASHRAND
════════════════════════════════════════════════

رابطك السحري جاهز!

للمصادقة بأمان مع HashRand، يرجى نسخ ولصق الرابط التالي في متصفح الويب الخاص بك:

🚀 رابط المصادقة:
{}

⚠️  معلومات أمنية مهمة:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ هذا الرابط صالح لفترة محدودة فقط
✓ يمكن استخدام هذا الرابط مرة واحدة فقط
✓ لا تشارك هذا الرابط مع أي شخص آخر
✓ إذا لم تطلب هذه المصادقة، يمكنك تجاهل هذا الإيميل

تحتاج مساعدة؟ تم إرسال هذا الإيميل لأن شخصاً ما طلب المصادقة للوصول إلى HashRand 
باستخدام عنوان بريدك الإلكتروني.

────────────────────────────────────────────────
HASHRAND - مولد الهاش العشوائي بالمعرفة الصفرية
────────────────────────────────────────────────
هذه رسالة أمنية تلقائية من noreply@mailer.hashrand.com
يرجى عدم الرد على هذا الإيميل - الردود لا تتم مراقبتها.

للدعم، قم بزيارة: https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}

/// Creates the Hindi HTML email template (LTR Devanagari script) - (placeholder)
pub fn create_hindi_html(magic_link: &str) -> String {
    // TODO: Implement full accessibility improvements
    format!(
        r#"
<!DOCTYPE html>
<html lang="hi">
<head>
    <meta charset="utf-8">
    <title>HashRand प्रमाणीकरण</title>
</head>
<body>
    <h1>🔐 HashRand प्रमाणीकरण</h1>
    <h2>आपका मैजिक लिंक तैयार है</h2>
    <p><a href="{}" style="color: white !important; background: #3b82f6; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block; font-weight: bold;">🚀 HashRand तक पहुंचें</a></p>
    <p>Fallback link: {}</p>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Hindi plain text email (LTR Devanagari script)
pub fn create_hindi_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 HASHRAND प्रमाणीकरण
════════════════════════════════════════════════

आपका मैजिक लिंक तैयार है!

HashRand के साथ सुरक्षित रूप से प्रमाणित करने के लिए, कृपया निम्नलिखित लिंक को 
अपने वेब ब्राउज़र में कॉपी और पेस्ट करें:

🚀 प्रमाणीकरण लिंक:
{}

⚠️  महत्वपूर्ण सुरक्षा जानकारी:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ यह लिंक केवल सीमित समय के लिए वैध है
✓ इस लिंक का केवल एक बार ही उपयोग किया जा सकता है
✓ इस लिंक को किसी के साथ साझा न करें
✓ यदि आपने इस प्रमाणीकरण का अनुरोध नहीं किया है, तो आप इस ईमेल को अनदेखा कर सकते हैं

सहायता चाहिए? यह ईमेल इसलिए भेजा गया है क्योंकि किसी ने आपके ईमेल पते का उपयोग करके 
HashRand तक पहुंच के लिए प्रमाणीकरण का अनुरोध किया है।

────────────────────────────────────────────────
HASHRAND - शून्य ज्ञान रैंडम हैश जनरेटर
────────────────────────────────────────────────
यह noreply@mailer.hashrand.com से एक स्वचालित सुरक्षा संदेश है
कृपया इस ईमेल का उत्तर न दें - उत्तर की निगरानी नहीं की जाती है।

समर्थन के लिए, यहां जाएं: https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}