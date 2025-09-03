//! Peninsula languages email templates (Catalan, Basque, Galician)
//! Regional languages of the Iberian Peninsula

/// Creates the Catalan HTML email template (placeholder)
pub fn create_catalan_html(magic_link: &str) -> String {
    // TODO: Implement full accessibility improvements
    format!(
        r#"
<!DOCTYPE html>
<html lang="ca">
<head>
    <meta charset="utf-8">
    <title>Autenticació HashRand</title>
</head>
<body>
    <h1>🔐 Autenticació HashRand</h1>
    <h2>El teu Enllaç Màgic està Llest</h2>
    <p><a href="{}" style="color: white !important; background: #3b82f6; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block; font-weight: bold;">🚀 Accedir a HashRand</a></p>
    <p>Fallback link: {}</p>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Catalan plain text email
pub fn create_catalan_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 AUTENTICACIÓ HASHRAND
════════════════════════════════════════════════

El teu Enllaç Màgic està Llest!

Per autenticar-te de forma segura amb HashRand, si us plau copia i enganxa el 
següent enllaç al teu navegador web:

🚀 Enllaç d'Autenticació:
{}

⚠️  INFORMACIÓ IMPORTANT DE SEGURETAT:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Aquest enllaç és vàlid només per TEMPS LIMITAT
✓ Aquest enllaç només es pot usar UNA VEGADA
✓ NO comparteixis aquest enllaç amb ningú més
✓ Si no has demanat aquesta autenticació, pots ignorar aquest correu

Necessites ajuda? Aquest correu s'ha enviat perquè algú ha demanat autenticació
per accedir a HashRand utilitzant la teva adreça de correu.

────────────────────────────────────────────────
HASHRAND - Generador d'Hashes Aleatoris amb Zero Knowledge
────────────────────────────────────────────────
Aquest és un missatge automàtic de seguretat de noreply@mailer.hashrand.com
Si us plau no responguis a aquest correu - les respostes no són monitorades.

Per suport, visita: https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}

/// Creates the Basque HTML email template (placeholder)
pub fn create_basque_html(magic_link: &str) -> String {
    // TODO: Implement full accessibility improvements
    format!(
        r#"
<!DOCTYPE html>
<html lang="eu">
<head>
    <meta charset="utf-8">
    <title>HashRand Autentifikazioa</title>
</head>
<body>
    <h1>🔐 HashRand Autentifikazioa</h1>
    <h2>Zure Esteka Magikoa Prest Dago</h2>
    <p><a href="{}" style="color: white !important; background: #3b82f6; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block; font-weight: bold;">🚀 HashRand-era Sartu</a></p>
    <p>Fallback link: {}</p>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Basque plain text email
pub fn create_basque_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 HASHRAND AUTENTIFIKAZIOA
════════════════════════════════════════════════

Zure Esteka Magikoa Prest Dago!

HashRand-ekin modu seguruan autentifikatzeko, mesedez kopiatu eta itsatsi hurrengo 
esteka zure web nabigatzailean:

🚀 Autentifikazio Esteka:
{}

⚠️  SEGURTASUN INFORMAZIO GARRANTZITSUA:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Esteka hau DENBORA MUGATUAN soilik da baliagarria
✓ Esteka hau BEHIN bakarrik erabil daiteke
✓ EZ partekatu esteka hau beste inorekin
✓ Autentifikazio hau eskatu ez baduzu, mezu hau baztertu dezakezu

Laguntza behar duzu? Mezu hau bidali da norbaitek zure helbide elektronikoa erabiliz
HashRand-era sartzeko autentifikazioa eskatu duelako.

────────────────────────────────────────────────
HASHRAND - Zero Knowledge Hash Sortzaile Ausazkoa
────────────────────────────────────────────────
Hau noreply@mailer.hashrand.com-etik bidali den segurtasun mezu automatiko bat da
Mesedez ez erantzun mezu honi - erantzunak ez dira kontrolatzen.

Laguntzarako, bisitatu: https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}

/// Creates the Galician HTML email template (placeholder)
pub fn create_galician_html(magic_link: &str) -> String {
    // TODO: Implement full accessibility improvements
    format!(
        r#"
<!DOCTYPE html>
<html lang="gl">
<head>
    <meta charset="utf-8">
    <title>Autenticación HashRand</title>
</head>
<body>
    <h1>🔐 Autenticación HashRand</h1>
    <h2>A túa Ligazón Máxica está Lista</h2>
    <p><a href="{}" style="color: white !important; background: #3b82f6; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block; font-weight: bold;">🚀 Acceder a HashRand</a></p>
    <p>Fallback link: {}</p>
</body>
</html>
"#,
        magic_link, magic_link
    )
}

/// Creates the Galician plain text email
pub fn create_galician_text(magic_link: &str) -> String {
    format!(
        r#"
════════════════════════════════════════════════
🔐 AUTENTICACIÓN HASHRAND
════════════════════════════════════════════════

A túa Ligazón Máxica está Lista!

Para autenticarte de xeito seguro con HashRand, por favor copia e pega a seguinte 
ligazón no teu navegador web:

🚀 Ligazón de Autenticación:
{}

⚠️  INFORMACIÓN IMPORTANTE DE SEGURIDADE:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Esta ligazón é válida por TEMPO LIMITADO únicamente
✓ Esta ligazón só pode usarse UNA VEZ
✓ NON compartas esta ligazón con ninguén máis
✓ Se non solicitaches esta autenticación, podes ignorar este correo

Precisas axuda? Este correo foi enviado porque alguén solicitou autenticación
para acceder a HashRand usando o teu enderezo de correo.

────────────────────────────────────────────────
HASHRAND - Xerador de Hashes Aleatorios con Zero Knowledge
────────────────────────────────────────────────
Esta é unha mensaxe automática de seguridade de noreply@mailer.hashrand.com
Por favor non respondas a este correo - as respostas non son monitorizadas.

Para soporte, visita: https://github.com/arkaitz-dev/hashrand-spin
"#,
        magic_link
    )
}