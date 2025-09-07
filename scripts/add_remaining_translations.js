#!/usr/bin/env node

/**
 * Script para añadir traducciones de magic link error a idiomas restantes
 * Idiomas ya completados: en, es, fr, de, pt
 * Idiomas pendientes: ru, zh, ar, eu, ca, gl, hi, ja
 */

const fs = require('fs');
const path = require('path');

// Traducciones para idiomas pendientes
const translations = {
    // Ruso
    ru: {
        understand: 'Я понимаю',
        magicLinkError: {
            title: 'Ошибка магической ссылки',
            message: 'Эта магическая ссылка может использоваться только в исходном браузере',
            explanation: 'По соображениям безопасности магические ссылки должны открываться в том же браузере, где они были запрошены.',
            securityTitle: 'Зачем эта мера безопасности?',
            securityExplanation: 'Эта двухфакторная проверка гарантирует, что только человек, который запросил магическую ссылку из этого конкретного браузера, может её использовать.'
        }
    },
    
    // Chino
    zh: {
        understand: '我明白了',
        magicLinkError: {
            title: '魔法链接错误',
            message: '此魔法链接只能在原始浏览器中使用',
            explanation: '出于安全考虑，魔法链接必须在请求它们的同一浏览器中打开。',
            securityTitle: '为什么要采取这种安全措施？',
            securityExplanation: '这种双因素验证确保只有从这个特定浏览器请求魔法链接的人才能使用它。'
        }
    },
    
    // Árabe
    ar: {
        understand: 'أفهم',
        magicLinkError: {
            title: 'خطأ في الرابط السحري',
            message: 'يمكن استخدام هذا الرابط السحري فقط في المتصفح الأصلي',
            explanation: 'لأسباب أمنية، يجب فتح الروابط السحرية في نفس المتصفح حيث تم طلبها.',
            securityTitle: 'لماذا هذا الإجراء الأمني؟',
            securityExplanation: 'هذا التحقق ثنائي العوامل يضمن أن الشخص الذي طلب الرابط السحري من هذا المتصفح المحدد فقط يمكنه استخدامه.'
        }
    },
    
    // Euskera
    eu: {
        understand: 'Ulertzen dut',
        magicLinkError: {
            title: 'Lotura Magikoaren Errorea',
            message: 'Lotura magiko hau jatorrizko nabigatzailean soilik erabil daiteke',
            explanation: 'Segurtasun arrazoiengatik, lotura magikoak eskatu ziren nabigatzaile berean ireki behar dira.',
            securityTitle: 'Zergatik segurtasun neurri hau?',
            securityExplanation: 'Bi faktoreko balidapen honek nabigatzaile espezifiko honetatik lotura magikoa eskatu zuen pertsonak soilik erabil dezakeela ziurtatzen du.'
        }
    },
    
    // Catalán
    ca: {
        understand: 'Ho entenc',
        magicLinkError: {
            title: 'Error d\'Enllaç Màgic',
            message: 'Aquest enllaç màgic només es pot utilitzar al navegador original',
            explanation: 'Per raons de seguretat, els enllaços màgics han d\'obrir-se al mateix navegador on van ser sol·licitats.',
            securityTitle: 'Per què aquesta mesura de seguretat?',
            securityExplanation: 'Aquesta validació de doble factor garanteix que només la persona que va sol·licitar l\'enllaç màgic des d\'aquest navegador específic pot utilitzar-lo.'
        }
    },
    
    // Gallego
    gl: {
        understand: 'Entendo',
        magicLinkError: {
            title: 'Erro de Ligazón Máxica',
            message: 'Esta ligazón máxica só pode utilizarse no navegador orixinal',
            explanation: 'Por razóns de seguridade, as ligazóns máxicas deben abrirse no mesmo navegador onde foron solicitadas.',
            securityTitle: 'Por que esta medida de seguridade?',
            securityExplanation: 'Esta validación de dobre factor garante que só a persoa que solicitou a ligazón máxica desde este navegador específico pode utilizala.'
        }
    },
    
    // Hindi
    hi: {
        understand: 'मैं समझ गया',
        magicLinkError: {
            title: 'मैजिक लिंक त्रुटि',
            message: 'इस मैजिक लिंक का उपयोग केवल मूल ब्राउज़र में किया जा सकता है',
            explanation: 'सुरक्षा कारणों से, मैजिक लिंक को उसी ब्राउज़र में खोला जाना चाहिए जहाँ उनका अनुरोध किया गया था।',
            securityTitle: 'यह सुरक्षा उपाय क्यों?',
            securityExplanation: 'यह दो-कारक सत्यापन सुनिश्चित करता है कि केवल वह व्यक्ति जिसने इस विशिष्ट ब्राउज़र से मैजिक लिंक का अनुरोध किया था, वह इसका उपयोग कर सकता है।'
        }
    },
    
    // Japonés
    ja: {
        understand: '理解しました',
        magicLinkError: {
            title: 'マジックリンクエラー',
            message: 'このマジックリンクは元のブラウザでのみ使用できます',
            explanation: 'セキュリティ上の理由により、マジックリンクは要求された同じブラウザで開かれる必要があります。',
            securityTitle: 'なぜこのセキュリティ対策？',
            securityExplanation: 'この二要素認証により、この特定のブラウザからマジックリンクを要求した人のみが使用できることが保証されます。'
        }
    }
};

// Directorio de traducciones
const translationsDir = path.join(__dirname, '../web/src/lib/stores/translations');

console.log('Añadiendo traducciones para idiomas pendientes...');

// Procesar cada idioma
for (const [lang, trans] of Object.entries(translations)) {
    const filePath = path.join(translationsDir, `${lang}.ts`);
    
    try {
        // Leer archivo existente
        let content = fs.readFileSync(filePath, 'utf8');
        
        // Añadir 'understand' en la sección common (buscar 'close:')
        if (!content.includes('understand:')) {
            content = content.replace(
                /(close:\s*'[^']+')(\s+})/,
                `$1,\n\t\tunderstand: '${trans.understand}'$2`
            );
        }
        
        // Añadir magicLinkError en la sección auth (buscar 'isCorrect:')
        if (!content.includes('magicLinkError:')) {
            const magicLinkErrorStr = `\t\tmagicLinkError: {
\t\t\ttitle: '${trans.magicLinkError.title}',
\t\t\tmessage: '${trans.magicLinkError.message}',
\t\t\texplanation: '${trans.magicLinkError.explanation}',
\t\t\tsecurityTitle: '${trans.magicLinkError.securityTitle}',
\t\t\tsecurityExplanation: '${trans.magicLinkError.securityExplanation}'
\t\t}`;
            
            content = content.replace(
                /(isCorrect:\s*'[^']+')(\s+}[\s\r\n]+},[\s\r\n]+logout:)/,
                `$1,\n${magicLinkErrorStr}$2`
            );
        }
        
        // Escribir archivo actualizado
        fs.writeFileSync(filePath, content, 'utf8');
        console.log(`✓ Actualizado ${lang}.ts`);
        
    } catch (error) {
        console.error(`✗ Error procesando ${lang}.ts:`, error.message);
    }
}

console.log('\n¡Traducciones actualizadas correctamente!');
console.log('Idiomas actualizados: ru, zh, ar, eu, ca, gl, hi, ja');
console.log('Idiomas ya completados: en, es, fr, de, pt');