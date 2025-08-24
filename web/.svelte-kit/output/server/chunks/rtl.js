import { D as store_get, E as attr_class, G as attr, M as escape_html, I as unsubscribe_stores, B as pop, z as push, F as stringify } from "./index.js";
import { w as writable, i as derived } from "./exports.js";
function getInitialState() {
  if (typeof window !== "undefined" && window.__SPRITE_STATE__) {
    return window.__SPRITE_STATE__;
  }
  return {
    loaded: false,
    loading: true,
    error: false
  };
}
const spriteState = writable(getInitialState());
const iconEmojis = {
  // Theme icons
  sun: "☀️",
  // Light mode icon
  moon: "🌙",
  // Dark mode icon
  // Navigation icons
  "arrow-left": ">",
  // Simple arrow for choose buttons (LTR) - flips to < in RTL
  "arrow-right": ">",
  // Simple arrow for choose buttons (RTL) - flips to < in RTL
  "chevron-down": "🔽",
  // Chevron down for expandable sections
  home: "🏠",
  // Home/house icon for back to menu buttons
  // UI icons
  heart: "❤️",
  // Heart for "made with love"
  check: "✅",
  // Checkmark for success states
  copy: "📋",
  // Copy to clipboard
  refresh: "🔄",
  // Refresh/regenerate icon
  settings: "⚙️",
  // Settings/gear icon
  // Flag emojis
  // Countries with standard UTF flag emojis
  saudi: "🇸🇦",
  // Saudi Arabia (Arabic)
  germany: "🇩🇪",
  // Germany (Deutsch)
  uk: "🇬🇧",
  // United Kingdom (English)
  spain: "🇪🇸",
  // Spain (Español)
  france: "🇫🇷",
  // France (Français)
  india: "🇮🇳",
  // India (Hindi)
  japan: "🇯🇵",
  // Japan (日本語)
  portugal: "🇵🇹",
  // Portugal (Português)
  russia: "🇷🇺",
  // Russia (Русский)
  china: "🇨🇳",
  // China (中文)
  // Regional flags without standard UTF emojis - use white flag
  catalonia: "🏳️",
  // Catalonia (no UTF emoji) - use white flag
  basque: "🏳️",
  // Basque Country (no UTF emoji) - use white flag
  galicia: "🏳️",
  // Galicia (no UTF emoji) - use white flag
  // Generic fallback for any unknown icon
  unknown: "❓"
};
function getIconEmoji(iconName) {
  return iconEmojis[iconName] || iconEmojis.unknown;
}
function hasProperIconEmoji(iconName) {
  const emoji = iconEmojis[iconName];
  return Boolean(emoji && emoji !== iconEmojis.unknown);
}
function Icon($$payload, $$props) {
  push();
  var $$store_subs;
  let {
    name,
    size = "w-5 h-5",
    class: className = "",
    placeholder = "auto"
  } = $$props;
  const iconId = `#icon-${name}`;
  const computedPlaceholder = () => {
    if (placeholder === "auto") {
      if (hasProperIconEmoji(name)) {
        return getIconEmoji(name);
      }
      return "spinner";
    }
    return placeholder;
  };
  if (store_get($$store_subs ??= {}, "$spriteState", spriteState).loaded) {
    $$payload.out.push("<!--[-->");
    $$payload.out.push(`<svg${attr_class(`${stringify(size)} ${stringify(className)}`)} aria-hidden="true"><use${attr("href", iconId)}></use></svg>`);
  } else {
    $$payload.out.push("<!--[!-->");
    if (computedPlaceholder() === "spinner") {
      $$payload.out.push("<!--[-->");
      $$payload.out.push(`<div${attr_class(`${stringify(size)} ${stringify(className)} flex items-center justify-center`)}><svg class="animate-spin h-4 w-4 text-gray-400" viewBox="0 0 24 24" aria-hidden="true"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg></div>`);
    } else {
      $$payload.out.push("<!--[!-->");
      $$payload.out.push(`<div${attr_class(`${stringify(size)} ${stringify(className)} flex items-center justify-center text-lg`)}>${escape_html(computedPlaceholder())}</div>`);
    }
    $$payload.out.push(`<!--]-->`);
  }
  $$payload.out.push(`<!--]-->`);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
const languages = [
  { code: "ar", name: "العربية", flag: "saudi" },
  // Arabiya
  { code: "ca", name: "Català", flag: "catalonia" },
  // Catala
  { code: "de", name: "Deutsch", flag: "germany" },
  // Deutsch
  { code: "en", name: "English", flag: "uk" },
  // English
  { code: "es", name: "Español", flag: "spain" },
  // Espanol
  { code: "eu", name: "Euskera", flag: "basque" },
  // Euskera
  { code: "fr", name: "Français", flag: "france" },
  // Francais
  { code: "gl", name: "Galego", flag: "galicia" },
  // Galego
  { code: "hi", name: "हिंदी", flag: "india" },
  // Hindi
  { code: "ja", name: "日本語", flag: "japan" },
  // Nihongo
  { code: "pt", name: "Português", flag: "portugal" },
  // Portugues
  { code: "ru", name: "Русский", flag: "russia" },
  // Russkiy
  { code: "zh", name: "中文", flag: "china" }
  // Zhongwen
];
function findLanguageByCode(code) {
  return languages.find((lang) => lang.code === code) || languages[0];
}
function getSupportedLanguageCodes() {
  return languages.map((lang) => lang.code);
}
const en = {
  common: {
    back: "Back",
    generate: "Generate",
    copy: "Copy",
    copied: "Copied!",
    backToMenu: "Back to Menu",
    loading: "Generating...",
    error: "Error occurred",
    result: "Result",
    choose: "Choose",
    type: "Type",
    length: "Length",
    generated: "Generated",
    format: "Format",
    security: "Security",
    loadingVersion: "Loading version...",
    versionsUnavailable: "Versions unavailable",
    generationDetails: "Generation Details",
    parametersUsed: "Parameters Used",
    generateAnother: "Generate Another",
    adjustSettings: "Adjust Settings",
    navigateTo: "Navigate to",
    selectLanguage: "Select language",
    switchToLightMode: "Switch to light mode",
    switchToDarkMode: "Switch to dark mode",
    characters: "letters",
    alphabet: "Alphabet",
    generatedValue: "Generated Value",
    clickToSelect: "Click the text area to select all, or use the copy button",
    waitGenerating: "Please wait while generating new value...",
    unknownEndpoint: "Unknown endpoint type",
    failedToCopy: "Failed to copy",
    fallbackCopyFailed: "Fallback copy failed",
    failedToRegenerate: "Failed to regenerate",
    failedToLoadVersions: "Failed to load versions",
    mustBeBetween: "must be between",
    and: "and",
    cannotExceed: "cannot exceed",
    optionalPrefix: "Optional prefix",
    optionalSuffix: "Optional suffix",
    prefixCannotExceed: "Prefix cannot exceed 32 letters",
    suffixCannotExceed: "Suffix cannot exceed 32 letters",
    yes: "Yes",
    no: "No",
    seedUsed: "Seed Used",
    copySeed: "Copy Seed",
    optionalSeed: "Optional seed (64 hex characters)",
    seedInvalid: "Seed must be exactly 64 hexadecimal characters",
    reuseSeedTitle: "Reuse the same seed?",
    reuseSeedMessage: "Do you want to reuse the same seed to generate the same result, or do you prefer to generate a new random seed?",
    keepSameSeed: "Keep the same seed",
    generateNewSeed: "Generate new seed",
    seed: "Seed",
    otp: "OTP"
  },
  menu: {
    title: "Hash Generator",
    subtitle: "Choose a generation method",
    version: "Version",
    brandName: "HashRand Spin",
    description: "Cryptographically secure hash, password, and API key generator"
  },
  custom: {
    title: "Custom Hash Generator",
    description: "Generate customized random hashes",
    generateHash: "Generate Hash",
    length: "Length",
    alphabet: "Alphabet Type",
    prefix: "Prefix",
    suffix: "Suffix",
    lengthMustBeBetween: "Length must be between 2 and 128",
    bitcoinDescription: "Bitcoin alphabet, excludes confusing letters",
    maxReadabilityDescription: "Maximum readability, 49 letters",
    completeAlphanumericDescription: "Complete alphanumeric set",
    maxEntropyDescription: "Maximum entropy with symbols",
    numericDescription: "Only digits 0-9, requires longer length",
    failedToGenerateHash: "Failed to generate hash"
  },
  password: {
    title: "Secure Password Generator",
    description: "Generate secure passwords",
    generatePassword: "Generate Password",
    length: "Length",
    alphabet: "Character Set",
    maxSecurityDescription: "Maximum security with symbols (73 chars)",
    easyReadDescription: "Easy to read and type (49 chars)",
    securityNote: "Security Note:",
    securityDescription: "Passwords are generated using cryptographically secure random generation. They are not stored or logged anywhere.",
    noLookAlikeNote: "No Look-alike alphabet excludes confusable letters. Minimum {0} letters for equivalent security.",
    fullAlphabetNote: "Full alphabet with symbols provides maximum entropy. Minimum {0} letters for strong security.",
    failedToGeneratePassword: "Failed to generate password"
  },
  apiKey: {
    title: "API Key Generator",
    description: "Generate API keys with ak_ prefix",
    generateApiKey: "Generate API Key",
    length: "Length",
    alphabet: "Character Set",
    standardAlphanumericDescription: "Standard alphanumeric (62 chars)",
    noConfusingDescription: "No confusing letters (49 chars)",
    formatNotice: 'All API keys are generated with the "ak_" prefix for easy identification. The specified length refers only to the random letters generated (prefix not counted).',
    securityNotice: "Store API keys securely and never expose them in client-side code or version control. Treat them with the same care as passwords.",
    randomCharacters: "random characters using",
    noLookAlikeAlphabet: "(easy to type)",
    fullAlphanumericAlphabet: "(maximum compatibility)",
    noLookAlikeNote: "No Look-alike excludes confusing characters. Minimum {0} characters for equivalent security.",
    fullAlphanumericNote: "Full alphanumeric provides maximum compatibility. Minimum {0} characters for strong security.",
    failedToGenerateApiKey: "Failed to generate API key"
  },
  alphabets: {
    base58: "Base58 (Bitcoin alphabet)",
    "no-look-alike": "No Look-alike",
    full: "Full Alphanumeric",
    "full-with-symbols": "Full with Symbols",
    numeric: "Numeric (0-9)"
  }
};
const hi = {
  common: {
    back: "वापस",
    generate: "जनरेट",
    copy: "कॉपी",
    copied: "कॉपी हो गया!",
    backToMenu: "मेन्यू में वापस",
    loading: "जनरेट हो रहा है...",
    error: "त्रुटि हुई",
    result: "परिणाम",
    choose: "चुनें",
    type: "प्रकार",
    length: "लंबाई",
    generated: "जनरेट हुआ",
    format: "फॉर्मेट",
    security: "सुरक्षा",
    loadingVersion: "वर्जन लोड हो रहा है...",
    versionsUnavailable: "वर्जन उपलब्ध नहीं हैं",
    generationDetails: "जनरेशन विवरण",
    parametersUsed: "उपयोग किए गए पैरामीटर",
    generateAnother: "दूसरा जनरेट करें",
    adjustSettings: "सेटिंग्स अडजस्ट करें",
    navigateTo: "यहां जाएं",
    selectLanguage: "भाषा चुनें",
    switchToLightMode: "लाइट मोड में जाएं",
    switchToDarkMode: "डार्क मोड में जाएं",
    characters: "अक्षर",
    alphabet: "अल्फाबेट",
    generatedValue: "जनरेट हुई वैल्यू",
    clickToSelect: "सब सेलेक्ट करने के लिए टेक्स्ट एरिया पर क्लिक करें, या कॉपी बटन का उपयोग करें",
    waitGenerating: "कृपया नई वैल्यू जनरेट होने तक प्रतीक्षा करें...",
    unknownEndpoint: "अज्ञात एंडपॉइंट प्रकार",
    failedToCopy: "कॉपी नहीं हो सका",
    fallbackCopyFailed: "बैकअप कॉपी नहीं हुई",
    failedToRegenerate: "दोबारा जनरेट नहीं हुआ",
    failedToLoadVersions: "वर्जन लोड नहीं हुए",
    mustBeBetween: "के बीच होना चाहिए",
    and: "और",
    cannotExceed: "से अधिक नहीं हो सकता",
    optionalPrefix: "वैकल्पिक प्रीफिक्स",
    optionalSuffix: "वैकल्पिक सफिक्स",
    prefixCannotExceed: "प्रीफिक्स 32 अक्षरों से अधिक नहीं हो सकता",
    suffixCannotExceed: "सफिक्स 32 अक्षरों से अधिक नहीं हो सकता",
    seedUsed: "इस्तेमाल किया गया सीड",
    copySeed: "सीड कॉपी करें",
    optionalSeed: "वैकल्पिक सीड (64 हेक्स अक्षर)",
    seedInvalid: "सीड में ठीक 64 हेक्साडेसिमल अक्षर होने चाहिए",
    reuseSeedTitle: "वही सीड दोबारा इस्तेमाल करें?",
    reuseSeedMessage: "क्या आप वही परिणाम पाने के लिए वही सीड दोबारा इस्तेमाल करना चाहते हैं, या आप नया रैंडम सीड जनरेट करना पसंद करेंगे?",
    keepSameSeed: "वही सीड रखें",
    generateNewSeed: "नया सीड जनरेट करें",
    seed: "सीड"
  },
  menu: {
    title: "हैश जनरेटर",
    subtitle: "जनरेशन मेथड चुनें",
    version: "वर्जन",
    brandName: "HashRand Spin",
    description: "क्रिप्टोग्राफिक रूप से सुरक्षित हैश, पासवर्ड और API की जनरेटर"
  },
  custom: {
    title: "कस्टम हैश जनरेटर",
    description: "कस्टम रैंडम हैश जनरेट करें",
    generateHash: "हैश जनरेट करें",
    length: "लंबाई",
    alphabet: "अल्फाबेट टाइप",
    prefix: "प्रीफिक्स",
    suffix: "सफिक्स",
    lengthMustBeBetween: "लेंथ 2 और 128 के बीच होनी चाहिए",
    bitcoinDescription: "Bitcoin अल्फाबेट, भ्रामक अक्षरों को छोड़ता है",
    maxReadabilityDescription: "अधिकतम पठनीयता, 49 अक्षर",
    completeAlphanumericDescription: "पूर्ण अल्फान्यूमेरिक सेट",
    maxEntropyDescription: "प्रतीकों के साथ अधिकतम एंट्रॉपी",
    failedToGenerateHash: "हैश जनरेट नहीं हुआ"
  },
  password: {
    title: "सुरक्षित पासवर्ड जनरेटर",
    description: "सुरक्षित पासवर्ड जनरेट करें",
    generatePassword: "पासवर्ड जनरेट करें",
    length: "लंबाई",
    alphabet: "कैरेक्टर सेट",
    maxSecurityDescription: "प्रतीकों के साथ अधिकतम सिक्यूरिटी (73 अक्षर)",
    easyReadDescription: "पढ़ने और टाइप करने में आसान (49 अक्षर)",
    securityNote: "सिक्यूरिटी नोट:",
    securityDescription: "पासवर्ड क्रिप्टोग्राफिक रूप से सुरक्षित रैंडम जनरेशन का उपयोग करके बनाए जाते हैं। वे कहीं भी स्टोर या लॉग नहीं किए जाते हैं।",
    noLookAlikeNote: "नो लुक-अलाइक अल्फाबेट भ्रामक अक्षरों को छोड़ता है। समकक्ष सिक्यूरिटी के लिए न्यूनतम {0} अक्षर।",
    fullAlphabetNote: "प्रतीकों के साथ पूर्ण अल्फाबेट अधिकतम एंट्रॉपी प्रदान करता है। मजबूत सिक्यूरिटी के लिए न्यूनतम {0} अक्षर।",
    failedToGeneratePassword: "पासवर्ड जनरेट नहीं हुआ"
  },
  apiKey: {
    title: "API की जनरेटर",
    description: "ak_ प्रीफिक्स के साथ API की जनरेट करें",
    generateApiKey: "API की जनरेट करें",
    length: "लंबाई",
    alphabet: "कैरेक्टर सेट",
    standardAlphanumericDescription: "स्टैंडर्ड अल्फान्यूमेरिक (62 अक्षर)",
    noConfusingDescription: "भ्रामक अक्षर नहीं (49 अक्षर)",
    formatNotice: 'सभी API की आसान पहचान के लिए "ak_" प्रीफिक्स के साथ जनरेट होती हैं। निर्दिष्ट लेंथ केवल जनरेट किए गए रैंडम अक्षरों को संदर्भित करती है (प्रीफिक्स गिना नहीं जाता)।',
    securityNotice: "API की को सुरक्षित रूप से स्टोर करें और उन्हें क्लाइंट-साइड कोड या वर्जन कंट्रोल में कभी एक्सपोज न करें। उन्हें पासवर्ड की तरह सावधानी से ट्रीट करें।",
    formatPrefix: "ak_ प्रीफिक्स +",
    randomCharacters: "रैंडम अक्षर के साथ",
    noLookAlikeAlphabet: "नो लुक-अलाइक अल्फाबेट (टाइप करने में आसान)",
    fullAlphanumericAlphabet: "पूर्ण अल्फान्यूमेरिक अल्फाबेट",
    fullAlphanumericNote: "पूर्ण अल्फान्यूमेरिक अल्फाबेट अधिकतम संगतता प्रदान करता है। मजबूत सुरक्षा के लिए न्यूनतम {0} वर्ण आवश्यक।",
    failedToGenerateApiKey: "API की जनरेट नहीं हुई"
  },
  alphabets: {
    base58: "Base58 (बिटकॉइन अल्फाबेट)",
    "no-look-alike": "नो लुक-अलाइक",
    full: "पूर्ण अल्फान्यूमेरिक",
    "full-with-symbols": "प्रतीकों के साथ पूर्ण"
  }
};
const ja = {
  common: {
    back: "戻る",
    generate: "生成",
    copy: "コピー",
    copied: "コピーしました！",
    backToMenu: "メニューに戻る",
    loading: "生成中...",
    error: "エラーが発生しました",
    result: "結果",
    choose: "選択",
    type: "タイプ",
    length: "文字数",
    generated: "生成済み",
    format: "フォーマット",
    security: "セキュリティ",
    loadingVersion: "バージョン読み込み中...",
    versionsUnavailable: "バージョン情報が利用できません",
    generationDetails: "生成詳細",
    parametersUsed: "使用したパラメータ",
    generateAnother: "もう一度生成",
    adjustSettings: "設定を調整",
    navigateTo: "移動する",
    selectLanguage: "言語を選択",
    switchToLightMode: "ライトモードに切り替え",
    switchToDarkMode: "ダークモードに切り替え",
    characters: "文字",
    alphabet: "アルファベット",
    generatedValue: "生成された値",
    clickToSelect: "テキストエリアをクリックして全選択するか、コピーボタンをお使いください",
    waitGenerating: "新しい値を生成中です。しばらくお待ちください...",
    unknownEndpoint: "不明なエンドポイントタイプ",
    failedToCopy: "コピーに失敗しました",
    fallbackCopyFailed: "フォールバックコピーに失敗しました",
    failedToRegenerate: "再生成に失敗しました",
    failedToLoadVersions: "バージョン読み込みに失敗しました",
    mustBeBetween: "は次の範囲である必要があります：",
    and: "から",
    cannotExceed: "を超えることはできません",
    optionalPrefix: "プレフィックス（オプション）",
    optionalSuffix: "サフィックス（オプション）",
    prefixCannotExceed: "プレフィックスは32文字を超えることはできません",
    suffixCannotExceed: "サフィックスは32文字を超えることはできません",
    yes: "はい",
    no: "いいえ",
    seedUsed: "使用されたシード",
    copySeed: "シードをコピー",
    optionalSeed: "オプションのシード（64文字の16進数）",
    seedInvalid: "シードは正確に64文字の16進数である必要があります",
    reuseSeedTitle: "同じシードを再利用しますか？",
    reuseSeedMessage: "同じ結果を得るために同じシードを再利用しますか、それとも新しいランダムシードを生成することを好みますか？",
    keepSameSeed: "同じシードを保持",
    generateNewSeed: "新しいシードを生成",
    seed: "シード"
  },
  menu: {
    title: "ハッシュ生成器",
    subtitle: "生成方法を選択してください",
    version: "バージョン",
    brandName: "HashRand Spin",
    description: "暗号学的に安全なハッシュ、パスワード、APIキーの生成器"
  },
  custom: {
    title: "カスタムハッシュ生成器",
    description: "カスタムランダムハッシュを生成",
    generateHash: "ハッシュを生成",
    length: "文字数",
    alphabet: "アルファベットタイプ",
    prefix: "プレフィックス",
    suffix: "サフィックス",
    lengthMustBeBetween: "文字数は2から128の間である必要があります",
    bitcoinDescription: "Bitcoinアルファベット、紛らわしい文字を除外",
    maxReadabilityDescription: "最大の可読性、49文字",
    completeAlphanumericDescription: "完全な英数字セット",
    maxEntropyDescription: "記号を含む最大エントロピー",
    failedToGenerateHash: "ハッシュの生成に失敗しました"
  },
  password: {
    title: "安全なパスワード生成器",
    description: "安全なパスワードを生成",
    generatePassword: "パスワードを生成",
    length: "文字数",
    alphabet: "文字セット",
    maxSecurityDescription: "記号を含む最大セキュリティ（73文字）",
    easyReadDescription: "読みやすく入力しやすい（49文字）",
    securityNote: "セキュリティに関する注意：",
    securityDescription: "パスワードは暗号学的に安全な乱数生成を使用して作成されます。どこにも保存やログ記録されません。",
    noLookAlikeNote: "紛らわしくないアルファベットは混同しやすい文字を除外します。同等のセキュリティには最低{0}文字が必要です。",
    fullAlphabetNote: "記号を含む完全なアルファベットは最大エントロピーを提供します。強力なセキュリティには最低{0}文字が必要です。",
    failedToGeneratePassword: "パスワードの生成に失敗しました"
  },
  apiKey: {
    title: "APIキー生成器",
    description: "ak_プレフィックス付きAPIキーを生成",
    generateApiKey: "APIキーを生成",
    length: "文字数",
    alphabet: "文字セット",
    standardAlphanumericDescription: "標準英数字（62文字）",
    noConfusingDescription: "紛らわしい文字なし（49文字）",
    formatNotice: "すべてのAPIキーは識別しやすくするために「ak_」プレフィックスが付加されます。指定した文字数は生成されるランダム文字のみを指します（プレフィックスは含みません）。",
    securityNotice: "APIキーは安全に保存し、クライアント側コードやバージョン管理システムに絶対に公開しないでください。パスワードと同じように慎重に扱ってください。",
    formatPrefix: "ak_プレフィックス +",
    randomCharacters: "ランダム文字を使用：",
    noLookAlikeAlphabet: "紛らわしくないアルファベット（入力しやすい）",
    fullAlphanumericAlphabet: "完全英数字アルファベット",
    fullAlphanumericNote: "完全英数字アルファベットは最大の互換性を提供します。強力なセキュリティには最低{0}文字が必要です。",
    failedToGenerateApiKey: "APIキーの生成に失敗しました"
  },
  alphabets: {
    base58: "Base58（ビットコインアルファベット）",
    "no-look-alike": "紛らわしくない",
    full: "完全英数字",
    "full-with-symbols": "記号を含む完全"
  }
};
const es = {
  common: {
    back: "Volver",
    generate: "Generar",
    copy: "Copiar",
    copied: "¡Copiado!",
    backToMenu: "Volver al Menú",
    loading: "Generando...",
    error: "Ha ocurrido un error",
    result: "Resultado",
    choose: "Elegir",
    type: "Tipo",
    length: "Longitud",
    generated: "Generado",
    format: "Formato",
    security: "Seguridad",
    loadingVersion: "Cargando versión...",
    versionsUnavailable: "Versiones no disponibles",
    generationDetails: "Detalles de Generación",
    parametersUsed: "Parámetros Utilizados",
    generateAnother: "Generar Otro",
    adjustSettings: "Ajustar Configuración",
    navigateTo: "Navegar a",
    selectLanguage: "Seleccionar idioma",
    switchToLightMode: "Cambiar a modo claro",
    switchToDarkMode: "Cambiar a modo oscuro",
    characters: "caracteres",
    alphabet: "Alfabeto",
    generatedValue: "Valor Generado",
    clickToSelect: "Haz clic en el área de texto para seleccionar todo, o usa el botón de copiar",
    waitGenerating: "Por favor, espera mientras se genera un nuevo valor...",
    unknownEndpoint: "Tipo de endpoint desconocido",
    failedToCopy: "Error al copiar",
    fallbackCopyFailed: "Error en copia de respaldo",
    failedToRegenerate: "Error al regenerar",
    failedToLoadVersions: "Error al cargar versiones",
    mustBeBetween: "debe estar entre",
    and: "y",
    cannotExceed: "no puede exceder",
    optionalPrefix: "Prefijo opcional",
    optionalSuffix: "Sufijo opcional",
    prefixCannotExceed: "El prefijo no puede exceder 32 letras",
    suffixCannotExceed: "El sufijo no puede exceder 32 letras",
    yes: "Sí",
    no: "No",
    seedUsed: "Semilla Utilizada",
    copySeed: "Copiar Semilla",
    optionalSeed: "Semilla opcional (64 caracteres hex)",
    seedInvalid: "La semilla debe tener exactamente 64 caracteres hexadecimales",
    reuseSeedTitle: "¿Reutilizar la misma semilla?",
    reuseSeedMessage: "¿Deseas reutilizar la misma semilla para generar el mismo resultado, o prefieres generar una nueva semilla aleatoria?",
    keepSameSeed: "Mantener la misma semilla",
    generateNewSeed: "Generar nueva semilla",
    seed: "Semilla",
    otp: "OTP"
  },
  menu: {
    title: "Generador de Hash",
    subtitle: "Elige un método de generación",
    version: "Versión",
    brandName: "HashRand Spin",
    description: "Generador criptográficamente seguro de hashes, contraseñas y claves API"
  },
  custom: {
    title: "Generador de Hash Personalizado",
    description: "Genera hashes aleatorios personalizados",
    generateHash: "Generar Hash",
    length: "Longitud",
    alphabet: "Tipo de Alfabeto",
    prefix: "Prefijo",
    suffix: "Sufijo",
    lengthMustBeBetween: "La longitud debe estar entre 2 y 128",
    bitcoinDescription: "Alfabeto Bitcoin, excluye letras confusas",
    maxReadabilityDescription: "Máxima legibilidad, 49 letras",
    completeAlphanumericDescription: "Conjunto alfanumérico completo",
    maxEntropyDescription: "Máxima entropía con símbolos",
    numericDescription: "Solo dígitos 0-9, requiere mayor longitud",
    failedToGenerateHash: "Error al generar hash"
  },
  password: {
    title: "Generador de Contraseñas Seguras",
    description: "Genera contraseñas seguras",
    generatePassword: "Generar Contraseña",
    length: "Longitud",
    alphabet: "Conjunto de Caracteres",
    maxSecurityDescription: "Máxima seguridad con símbolos (73 chars)",
    easyReadDescription: "Fácil de leer y escribir (49 chars)",
    securityNote: "Nota de Seguridad:",
    securityDescription: "Las contraseñas se generan usando generación aleatoria criptográficamente segura. No se almacenan ni registran en ningún lugar.",
    noLookAlikeNote: "El alfabeto Sin Confusión excluye letras confundibles. Mínimo {0} letras para seguridad equivalente.",
    fullAlphabetNote: "El alfabeto completo con símbolos proporciona máxima entropía. Mínimo {0} letras para seguridad fuerte.",
    failedToGeneratePassword: "Error al generar contraseña"
  },
  apiKey: {
    title: "Generador de Claves API",
    description: "Genera claves API con prefijo ak_",
    generateApiKey: "Generar Clave API",
    length: "Longitud",
    alphabet: "Conjunto de Caracteres",
    standardAlphanumericDescription: "Alfanumérico estándar (62 chars)",
    noConfusingDescription: "Sin letras confusas (49 chars)",
    formatNotice: 'Todas las claves API se generan con el prefijo "ak_" para fácil identificación. La longitud especificada se refiere solo a las letras aleatorias generadas (prefijo no contado).',
    securityNotice: "Almacena las claves API de forma segura y nunca las expongas en código del lado del cliente o control de versiones. Trátalas con el mismo cuidado que las contraseñas.",
    formatPrefix: "prefijo ak_ +",
    randomCharacters: "letras aleatorias con",
    noLookAlikeAlphabet: "alfabeto sin confusión (fácil de escribir)",
    fullAlphanumericAlphabet: "alfabeto alfanumérico completo",
    noLookAlikeNote: "El alfabeto Sin Confusión excluye letras confundibles. Mínimo {0} caracteres para seguridad equivalente.",
    fullAlphanumericNote: "El alfabeto completo alfanumérico proporciona buena entropía. Mínimo {0} caracteres para seguridad fuerte.",
    failedToGenerateApiKey: "Error al generar clave API"
  },
  alphabets: {
    base58: "Base58 (alfabeto Bitcoin)",
    "no-look-alike": "Sin Confusión",
    full: "Alfanumérico Completo",
    "full-with-symbols": "Completo con Símbolos",
    numeric: "Numérico (0-9)"
  }
};
const pt = {
  common: {
    back: "Voltar",
    generate: "Gerar",
    copy: "Copiar",
    copied: "Copiado!",
    backToMenu: "Voltar ao Menu",
    loading: "Gerando...",
    error: "Ocorreu um erro",
    result: "Resultado",
    choose: "Escolher",
    type: "Tipo",
    length: "Comprimento",
    generated: "Gerado",
    format: "Formato",
    security: "Segurança",
    loadingVersion: "Carregando versão...",
    versionsUnavailable: "Versões indisponíveis",
    generationDetails: "Detalhes da Geração",
    parametersUsed: "Parâmetros Utilizados",
    generateAnother: "Gerar Outro",
    adjustSettings: "Ajustar Configurações",
    navigateTo: "Navegar para",
    selectLanguage: "Selecionar idioma",
    switchToLightMode: "Mudar para modo claro",
    switchToDarkMode: "Mudar para modo escuro",
    characters: "caracteres",
    alphabet: "Alfabeto",
    generatedValue: "Valor Gerado",
    clickToSelect: "Clique na área de texto para selecionar tudo, ou use o botão copiar",
    waitGenerating: "Por favor, aguarde enquanto um novo valor é gerado...",
    unknownEndpoint: "Tipo de endpoint desconhecido",
    failedToCopy: "Falha ao copiar",
    fallbackCopyFailed: "Falha na cópia de backup",
    failedToRegenerate: "Falha ao regenerar",
    failedToLoadVersions: "Falha ao carregar versões",
    mustBeBetween: "deve estar entre",
    and: "e",
    cannotExceed: "não pode exceder",
    optionalPrefix: "Prefixo opcional",
    optionalSuffix: "Sufixo opcional",
    prefixCannotExceed: "O prefixo não pode exceder 32 caracteres",
    suffixCannotExceed: "O sufixo não pode exceder 32 caracteres",
    yes: "Sim",
    no: "Não",
    seedUsed: "Semente Utilizada",
    copySeed: "Copiar Semente",
    optionalSeed: "Semente opcional (64 caracteres hex)",
    seedInvalid: "A semente deve ter exatamente 64 caracteres hexadecimais",
    reuseSeedTitle: "Reutilizar a mesma semente?",
    reuseSeedMessage: "Deseja reutilizar a mesma semente para gerar o mesmo resultado, ou prefere gerar uma nova semente aleatória?",
    keepSameSeed: "Manter a mesma semente",
    generateNewSeed: "Gerar nova semente",
    seed: "Semente",
    otp: "OTP"
  },
  menu: {
    title: "Gerador de Hash",
    subtitle: "Escolha um método de geração",
    version: "Versão",
    brandName: "HashRand Spin",
    description: "Gerador criptograficamente seguro de hashes, senhas e chaves API"
  },
  custom: {
    title: "Gerador de Hash Personalizado",
    description: "Gere hashes aleatórios personalizados",
    generateHash: "Gerar Hash",
    length: "Comprimento",
    alphabet: "Tipo de Alfabeto",
    prefix: "Prefixo",
    suffix: "Sufixo",
    lengthMustBeBetween: "O comprimento deve estar entre 2 e 128",
    bitcoinDescription: "Alfabeto Bitcoin, exclui letras confusas",
    maxReadabilityDescription: "Máxima legibilidade, 49 letras",
    completeAlphanumericDescription: "Conjunto alfanumérico completo",
    maxEntropyDescription: "Máxima entropia com símbolos",
    numericDescription: "Apenas dígitos 0-9, requer comprimento maior",
    failedToGenerateHash: "Falha ao gerar hash"
  },
  password: {
    title: "Gerador de Senhas Seguras",
    description: "Gere senhas seguras",
    generatePassword: "Gerar Senha",
    length: "Comprimento",
    alphabet: "Conjunto de Caracteres",
    maxSecurityDescription: "Máxima segurança com símbolos (73 chars)",
    easyReadDescription: "Fácil de ler e digitar (49 chars)",
    securityNote: "Nota de Segurança:",
    securityDescription: "As palavras-passe são geradas utilizando geração aleatória criptograficamente segura. Não são armazenadas nem registadas em lugar algum.",
    noLookAlikeNote: "O alfabeto Sem Confusão exclui letras confundíveis. Mínimo {0} letras para segurança equivalente.",
    fullAlphabetNote: "O alfabeto completo com símbolos fornece máxima entropia. Mínimo {0} letras para segurança forte.",
    failedToGeneratePassword: "Falha ao gerar senha"
  },
  apiKey: {
    title: "Gerador de Chaves API",
    description: "Gere chaves API com prefixo ak_",
    generateApiKey: "Gerar Chave API",
    length: "Comprimento",
    alphabet: "Conjunto de Caracteres",
    standardAlphanumericDescription: "Alfanumérico padrão (62 chars)",
    noConfusingDescription: "Sem letras confusas (49 chars)",
    formatNotice: 'Todas as chaves API são geradas com o prefixo "ak_" para fácil identificação. O comprimento especificado refere-se apenas às letras aleatórias geradas (prefixo não contado).',
    securityNotice: "Armazene as chaves API com segurança e nunca as exponha em código do lado do cliente ou controlo de versões. Trate-as com o mesmo cuidado que as palavras-passe.",
    formatPrefix: "prefixo ak_ +",
    randomCharacters: "caracteres aleatórios com",
    noLookAlikeAlphabet: "alfabeto sem confusão (fácil de digitar)",
    fullAlphanumericAlphabet: "alfabeto alfanumérico completo",
    fullAlphanumericNote: "O alfabeto alfanumérico completo oferece máxima compatibilidade. Mínimo de {0} caracteres para segurança forte.",
    failedToGenerateApiKey: "Falha ao gerar chave API"
  },
  alphabets: {
    base58: "Base58 (alfabeto Bitcoin)",
    "no-look-alike": "Sem Confusão",
    full: "Alfanumérico Completo",
    "full-with-symbols": "Completo com Símbolos",
    numeric: "Numérico (0-9)"
  }
};
const fr = {
  common: {
    back: "Retour",
    generate: "Générer",
    copy: "Copier",
    copied: "Copié !",
    backToMenu: "Retour au Menu",
    loading: "Génération...",
    error: "Une erreur s'est produite",
    result: "Résultat",
    choose: "Choisir",
    type: "Type",
    length: "Longueur",
    generated: "Généré",
    format: "Format",
    security: "Sécurité",
    loadingVersion: "Chargement de la version...",
    versionsUnavailable: "Versions indisponibles",
    generationDetails: "Détails de Génération",
    parametersUsed: "Paramètres Utilisés",
    generateAnother: "Générer un Autre",
    adjustSettings: "Ajuster les Paramètres",
    navigateTo: "Naviguer vers",
    selectLanguage: "Sélectionner la langue",
    switchToLightMode: "Passer en mode clair",
    switchToDarkMode: "Passer en mode sombre",
    characters: "lettres",
    alphabet: "Alphabet",
    generatedValue: "Valeur Générée",
    clickToSelect: "Cliquez sur la zone de texte pour tout sélectionner, ou utilisez le bouton copier",
    waitGenerating: "Veuillez patienter pendant la génération d'une nouvelle valeur...",
    unknownEndpoint: "Type de point de terminaison inconnu",
    failedToCopy: "Échec de la copie",
    fallbackCopyFailed: "Échec de la copie de secours",
    failedToRegenerate: "Échec de la régénération",
    failedToLoadVersions: "Échec du chargement des versions",
    mustBeBetween: "doit être entre",
    and: "et",
    cannotExceed: "ne peut pas dépasser",
    optionalPrefix: "Préfixe optionnel",
    optionalSuffix: "Suffixe optionnel",
    prefixCannotExceed: "Le préfixe ne peut pas dépasser 32 caractères",
    suffixCannotExceed: "Le suffixe ne peut pas dépasser 32 caractères",
    yes: "Oui",
    no: "Non",
    seedUsed: "Graine Utilisée",
    copySeed: "Copier Graine",
    optionalSeed: "Graine optionnelle (64 caractères hex)",
    seedInvalid: "La graine doit avoir exactement 64 caractères hexadécimaux",
    reuseSeedTitle: "Réutiliser la même graine?",
    reuseSeedMessage: "Voulez-vous réutiliser la même graine pour générer le même résultat, ou préférez-vous générer une nouvelle graine aléatoire?",
    keepSameSeed: "Garder la même graine",
    generateNewSeed: "Générer une nouvelle graine",
    seed: "Graine"
  },
  menu: {
    title: "Générateur de Hash",
    subtitle: "Choisissez une méthode de génération",
    version: "Version",
    brandName: "HashRand Spin",
    description: "Générateur cryptographiquement sécurisé de hashs, mots de passe et clés API"
  },
  custom: {
    title: "Générateur de Hash Personnalisé",
    description: "Générez des hashs aléatoires personnalisés",
    generateHash: "Générer Hash",
    length: "Longueur",
    alphabet: "Type d'Alphabet",
    prefix: "Préfixe",
    suffix: "Suffixe",
    lengthMustBeBetween: "La longueur doit être entre 2 et 128",
    bitcoinDescription: "Alphabet Bitcoin, exclut les lettres ambigues",
    maxReadabilityDescription: "Lisibilité maximale, 49 lettres",
    completeAlphanumericDescription: "Ensemble alphanumérique complet",
    maxEntropyDescription: "Entropie maximale avec symboles",
    failedToGenerateHash: "Échec de la génération du hash"
  },
  password: {
    title: "Générateur de Mots de Passe Sécurisés",
    description: "Générez des mots de passe sécurisés",
    generatePassword: "Générer Mot de Passe",
    length: "Longueur",
    alphabet: "Jeu de Caractères",
    maxSecurityDescription: "Sécurité maximale avec symboles (73 chars)",
    easyReadDescription: "Facile à lire et taper (49 chars)",
    securityNote: "Note de Sécurité :",
    securityDescription: "Les mots de passe sont générés en utilisant une génération aléatoire cryptographiquement sécurisée. Ils ne sont stockés ni journalisés nulle part.",
    noLookAlikeNote: "L'alphabet sans ambiguïté exclut les caractères confus. Minimum {0} caractères pour une sécurité équivalente.",
    fullAlphabetNote: "L'alphabet complet avec symboles fournit une entropie maximale. Minimum {0} caractères pour une sécurité forte.",
    failedToGeneratePassword: "Échec de la génération du mot de passe"
  },
  apiKey: {
    title: "Générateur de Clés API",
    description: "Générez des clés API avec le préfixe ak_",
    generateApiKey: "Générer Clé API",
    length: "Longueur",
    alphabet: "Jeu de Caractères",
    standardAlphanumericDescription: "Alphanumérique standard (62 chars)",
    noConfusingDescription: "Aucune lettre confuse (49 chars)",
    formatNotice: 'Toutes les clés API sont générées avec le préfixe "ak_" pour une identification facile. La longueur spécifiée ne concerne que les caractères aléatoires générés (préfixe non compté).',
    securityNotice: "Stockez les clés API en sécurité et ne les exposez jamais dans le code côté client ou le contrôle de version. Traitez-les avec le même soin que les mots de passe.",
    formatPrefix: "préfixe ak_ +",
    randomCharacters: "caractères aléatoires avec",
    noLookAlikeAlphabet: "alphabet sans ambiguïté (facile à taper)",
    fullAlphanumericAlphabet: "alphabet alphanumérique complet",
    fullAlphanumericNote: "L'alphabet alphanumérique complet offre une compatibilité maximale. Minimum {0} caractères pour une sécurité forte.",
    failedToGenerateApiKey: "Échec de la génération de la clé API"
  },
  alphabets: {
    base58: "Base58 (alphabet Bitcoin)",
    "no-look-alike": "Sans Ambiguïté",
    full: "Alphanumérique Complet",
    "full-with-symbols": "Complet avec Symboles"
  }
};
const de = {
  common: {
    back: "Zurück",
    generate: "Generieren",
    copy: "Kopieren",
    copied: "Kopiert!",
    backToMenu: "Zurück zum Menü",
    loading: "Wird generiert...",
    error: "Ein Fehler ist aufgetreten",
    result: "Ergebnis",
    choose: "Wählen",
    type: "Typ",
    length: "Länge",
    generated: "Generiert",
    format: "Format",
    security: "Sicherheit",
    loadingVersion: "Version wird geladen...",
    versionsUnavailable: "Versionen nicht verfügbar",
    generationDetails: "Generierungsdetails",
    parametersUsed: "Verwendete Parameter",
    generateAnother: "Neuen generieren",
    adjustSettings: "Einstellungen anpassen",
    navigateTo: "Gehe zu",
    selectLanguage: "Sprache auswählen",
    switchToLightMode: "Zum hellen Modus wechseln",
    switchToDarkMode: "Zum dunklen Modus wechseln",
    characters: "Zeichen",
    alphabet: "Alphabet",
    generatedValue: "Generierter Wert",
    clickToSelect: "Klicken Sie auf das Textfeld, um alles auszuwählen, oder verwenden Sie den Kopieren-Button",
    waitGenerating: "Bitte warten Sie, während ein neuer Wert generiert wird...",
    unknownEndpoint: "Unbekannter Endpoint-Typ",
    failedToCopy: "Kopieren fehlgeschlagen",
    fallbackCopyFailed: "Ersatzkopie fehlgeschlagen",
    failedToRegenerate: "Neugenerierung fehlgeschlagen",
    failedToLoadVersions: "Laden der Versionen fehlgeschlagen",
    mustBeBetween: "muss zwischen",
    and: "und",
    cannotExceed: "darf nicht überschreiten",
    optionalPrefix: "Optionales Präfix",
    optionalSuffix: "Optionales Suffix",
    prefixCannotExceed: "Das Präfix darf 32 Zeichen nicht überschreiten",
    suffixCannotExceed: "Das Suffix darf 32 Zeichen nicht überschreiten",
    seedUsed: "Verwendeter Seed",
    copySeed: "Seed Kopieren",
    optionalSeed: "Optionaler Seed (64 Hex-Zeichen)",
    seedInvalid: "Seed muss genau 64 hexadezimale Zeichen haben",
    reuseSeedTitle: "Denselben Seed wiederverwenden?",
    reuseSeedMessage: "Möchten Sie denselben Seed wiederverwenden, um das gleiche Ergebnis zu erhalten, oder bevorzugen Sie es, einen neuen zufälligen Seed zu generieren?",
    keepSameSeed: "Denselben Seed behalten",
    generateNewSeed: "Neuen Seed generieren",
    seed: "Seed"
  },
  menu: {
    title: "Hash-Generator",
    subtitle: "Wählen Sie eine Generierungsmethode",
    version: "Version",
    brandName: "HashRand Spin",
    description: "Kryptographisch sicherer Generator für Hashes, Passwörter und API-Schlüssel"
  },
  custom: {
    title: "Benutzerdefinierter Hash-Generator",
    description: "Generieren Sie benutzerdefinierte zufällige Hashes",
    generateHash: "Hash Generieren",
    length: "Länge",
    alphabet: "Alphabet-Typ",
    prefix: "Präfix",
    suffix: "Suffix",
    lengthMustBeBetween: "Die Länge muss zwischen 2 und 128 liegen",
    bitcoinDescription: "Bitcoin-Alphabet, schließt verwirrende Zeichen aus",
    maxReadabilityDescription: "Maximale Lesbarkeit, 49 Zeichen",
    completeAlphanumericDescription: "Vollständiger alphanumerischer Satz",
    maxEntropyDescription: "Maximale Entropie mit Symbolen",
    failedToGenerateHash: "Hash-Generierung fehlgeschlagen"
  },
  password: {
    title: "Sicherer Passwort-Generator",
    description: "Generieren Sie sichere Passwörter",
    generatePassword: "Passwort generieren",
    length: "Länge",
    alphabet: "Zeichensatz",
    maxSecurityDescription: "Maximale Sicherheit mit Symbolen (73 Zeichen)",
    easyReadDescription: "Leicht zu lesen und einzugeben (49 Zeichen)",
    securityNote: "Sicherheitshinweis:",
    securityDescription: "Passwörter werden mittels kryptographisch sicherer Zufallsgenerierung erstellt. Sie werden nirgendwo gespeichert oder protokolliert.",
    noLookAlikeNote: "Das eindeutige Alphabet schließt verwechselbare Zeichen aus. Mindestens {0} Zeichen für gleichwertige Sicherheit.",
    fullAlphabetNote: "Das vollständige Alphabet mit Symbolen bietet maximale Entropie. Mindestens {0} Zeichen für starke Sicherheit.",
    failedToGeneratePassword: "Passwort-Generierung fehlgeschlagen"
  },
  apiKey: {
    title: "API-Schlüssel-Generator",
    description: "Generieren Sie API-Schlüssel mit ak_-Präfix",
    generateApiKey: "API-Schlüssel generieren",
    length: "Länge",
    alphabet: "Zeichensatz",
    standardAlphanumericDescription: "Standard-Alphanumerisch (62 Zeichen)",
    noConfusingDescription: "Keine verwechselbaren Zeichen (49 Zeichen)",
    formatNotice: 'Alle API-Schlüssel werden zur einfachen Identifizierung mit dem Präfix "ak_" generiert. Die angegebene Länge bezieht sich nur auf die generierten Zufallszeichen (Präfix nicht mitgezählt).',
    securityNotice: "Bewahren Sie API-Schlüssel sicher auf und geben Sie sie niemals in clientseitigem Code oder der Versionskontrolle preis. Behandeln Sie sie mit derselben Sorgfalt wie Passwörter.",
    formatPrefix: "ak_-Präfix +",
    randomCharacters: "Zufallszeichen mit",
    noLookAlikeAlphabet: "eindeutigem Alphabet (leicht einzugeben)",
    fullAlphanumericAlphabet: "vollständigem alphanumerischem Alphabet",
    fullAlphanumericNote: "Das vollständige alphanumerische Alphabet bietet maximale Kompatibilität. Mindestens {0} Zeichen für starke Sicherheit.",
    failedToGenerateApiKey: "API-Schlüssel-Generierung fehlgeschlagen"
  },
  alphabets: {
    base58: "Base58 (Bitcoin-Alphabet)",
    "no-look-alike": "Eindeutig",
    full: "Vollständig Alphanumerisch",
    "full-with-symbols": "Vollständig mit Symbolen"
  }
};
const ru = {
  common: {
    back: "Назад",
    generate: "Генерировать",
    copy: "Копировать",
    copied: "Скопировано!",
    backToMenu: "Вернуться в меню",
    loading: "Генерация...",
    error: "Произошла ошибка",
    result: "Результат",
    choose: "Выбрать",
    type: "Тип",
    length: "Длина",
    generated: "Сгенерирован",
    format: "Формат",
    security: "Безопасность",
    loadingVersion: "Загрузка версии...",
    versionsUnavailable: "Версии недоступны",
    generationDetails: "Детали генерации",
    parametersUsed: "Используемые параметры",
    generateAnother: "Сгенерировать ещё",
    adjustSettings: "Настроить параметры",
    navigateTo: "Перейти к",
    selectLanguage: "Выбрать язык",
    switchToLightMode: "Переключиться на светлый режим",
    switchToDarkMode: "Переключиться на тёмный режим",
    characters: "символов",
    alphabet: "Алфавит",
    generatedValue: "Сгенерированное значение",
    clickToSelect: "Нажмите на текстовое поле для выделения всего текста или используйте кнопку копирования",
    waitGenerating: "Пожалуйста, подождите, пока генерируется новое значение...",
    unknownEndpoint: "Неизвестный тип эндпоинта",
    failedToCopy: "Ошибка копирования",
    fallbackCopyFailed: "Ошибка резервного копирования",
    failedToRegenerate: "Ошибка повторной генерации",
    failedToLoadVersions: "Ошибка загрузки версий",
    mustBeBetween: "должна быть между",
    and: "и",
    cannotExceed: "не может превышать",
    optionalPrefix: "Необязательный префикс",
    optionalSuffix: "Необязательный суффикс",
    prefixCannotExceed: "Префикс не может превышать 32 символа",
    suffixCannotExceed: "Суффикс не может превышать 32 символа",
    seedUsed: "Используемое зерно",
    copySeed: "Копировать зерно",
    optionalSeed: "Необязательное зерно (64 шестнадцатеричных символа)",
    seedInvalid: "Зерно должно содержать ровно 64 шестнадцатеричных символа",
    reuseSeedTitle: "Переиспользовать то же зерно?",
    reuseSeedMessage: "Хотите переиспользовать то же зерно для получения того же результата, или предпочитаете сгенерировать новое случайное зерно?",
    keepSameSeed: "Оставить то же зерно",
    generateNewSeed: "Сгенерировать новое зерно",
    seed: "Зерно"
  },
  menu: {
    title: "Генератор хешей",
    subtitle: "Выберите метод генерации",
    version: "Версия",
    brandName: "HashRand Spin",
    description: "Криптографически безопасный генератор хешей, паролей и API-ключей"
  },
  custom: {
    title: "Настраиваемый генератор хешей",
    description: "Генерируйте настраиваемые случайные хеши",
    generateHash: "Генерировать хеш",
    length: "Длина",
    alphabet: "Тип алфавита",
    prefix: "Префикс",
    suffix: "Суффикс",
    lengthMustBeBetween: "Длина должна быть между 2 и 128",
    bitcoinDescription: "Bitcoin-алфавит, исключает путающие символы",
    maxReadabilityDescription: "Максимальная читаемость, 49 символов",
    completeAlphanumericDescription: "Полный буквенно-цифровой набор",
    maxEntropyDescription: "Максимальная энтропия с символами",
    failedToGenerateHash: "Не удалось сгенерировать хеш"
  },
  password: {
    title: "Генератор безопасных паролей",
    description: "Генерируйте безопасные пароли",
    generatePassword: "Генерировать пароль",
    length: "Длина",
    alphabet: "Набор символов",
    maxSecurityDescription: "Максимальная безопасность с символами (73 символа)",
    easyReadDescription: "Легко читать и набирать (49 символов)",
    securityNote: "Примечание по безопасности:",
    securityDescription: "Пароли генерируются с использованием криптографически безопасной случайной генерации. Они нигде не хранятся и не записываются.",
    noLookAlikeNote: "Однозначный алфавит исключает путающие символы. Минимум {0} символов для эквивалентной безопасности.",
    fullAlphabetNote: "Полный алфавит с символами обеспечивает максимальную энтропию. Минимум {0} символов для надёжной безопасности.",
    failedToGeneratePassword: "Не удалось сгенерировать пароль"
  },
  apiKey: {
    title: "Генератор API-ключей",
    description: "Генерируйте API-ключи с префиксом ak_",
    generateApiKey: "Генерировать API-ключ",
    length: "Длина",
    alphabet: "Набор символов",
    standardAlphanumericDescription: "Стандартный буквенно-цифровой (62 символа)",
    noConfusingDescription: "Без путающих символов (49 символов)",
    formatNotice: 'Все API-ключи генерируются с префиксом "ak_" для простой идентификации. Указанная длина относится только к генерируемым случайным символам (префикс не учитывается).',
    securityNotice: "Храните API-ключи в безопасности и никогда не размещайте их в клиентском коде или системе контроля версий. Обращайтесь с ними так же бережно, как с паролями.",
    formatPrefix: "префикс ak_ +",
    randomCharacters: "случайных символов с",
    noLookAlikeAlphabet: "однозначный алфавит (легко набирать)",
    fullAlphanumericAlphabet: "полный буквенно-цифровой алфавит",
    fullAlphanumericNote: "Полный буквенно-цифровой алфавит обеспечивает максимальную совместимость. Минимум {0} символов для надёжной безопасности.",
    failedToGenerateApiKey: "Не удалось сгенерировать API-ключ"
  },
  alphabets: {
    base58: "Base58 (алфавит Bitcoin)",
    "no-look-alike": "Однозначный",
    full: "Полный буквенно-цифровой",
    "full-with-symbols": "Полный с символами"
  }
};
const zh = {
  common: {
    back: "返回",
    generate: "生成",
    copy: "复制",
    copied: "已复制！",
    backToMenu: "返回菜单",
    loading: "生成中...",
    error: "发生错误",
    result: "结果",
    choose: "选择",
    type: "类型",
    length: "长度",
    generated: "已生成",
    format: "格式",
    security: "安全性",
    loadingVersion: "加载版本中...",
    versionsUnavailable: "版本不可用",
    generationDetails: "生成详情",
    parametersUsed: "使用的参数",
    generateAnother: "再生成一个",
    adjustSettings: "调整设置",
    navigateTo: "导航到",
    selectLanguage: "选择语言",
    switchToLightMode: "切换到浅色模式",
    switchToDarkMode: "切换到深色模式",
    characters: "个字符",
    alphabet: "字母表",
    generatedValue: "生成的值",
    clickToSelect: "点击文本区域全选，或使用复制按钮",
    waitGenerating: "请等待生成新值...",
    unknownEndpoint: "未知的端点类型",
    failedToCopy: "复制失败",
    fallbackCopyFailed: "备用复制失败",
    failedToRegenerate: "重新生成失败",
    failedToLoadVersions: "加载版本失败",
    mustBeBetween: "必须在",
    and: "到",
    cannotExceed: "不能超过",
    optionalPrefix: "可选前缀",
    optionalSuffix: "可选后缀",
    prefixCannotExceed: "前缀不能超过32个字符",
    suffixCannotExceed: "后缀不能超过32个字符",
    seedUsed: "使用的种子",
    copySeed: "复制种子",
    optionalSeed: "可选种子（64个十六进制字符）",
    seedInvalid: "种子必须是64个十六进制字符",
    reuseSeedTitle: "重用相同的种子？",
    reuseSeedMessage: "您想重用相同的种子来生成相同的结果，还是更愿意生成新的随机种子？",
    keepSameSeed: "保持相同的种子",
    generateNewSeed: "生成新种子",
    seed: "种子"
  },
  menu: {
    title: "哈希生成器",
    subtitle: "选择生成方法",
    version: "版本",
    brandName: "HashRand Spin",
    description: "密码学安全的哈希、密码和API密钥生成器"
  },
  custom: {
    title: "自定义哈希生成器",
    description: "生成自定义随机哈希",
    generateHash: "生成哈希",
    length: "长度",
    alphabet: "字母表类型",
    prefix: "前缀",
    suffix: "后缀",
    lengthMustBeBetween: "长度必须在2到128之间",
    bitcoinDescription: "Bitcoin字母表，排除混淆字符",
    maxReadabilityDescription: "最大可读性，49个字符",
    completeAlphanumericDescription: "完整字母数字集合",
    maxEntropyDescription: "包含符号的最大熵",
    failedToGenerateHash: "生成哈希失败"
  },
  password: {
    title: "安全密码生成器",
    description: "生成安全密码",
    generatePassword: "生成密码",
    length: "长度",
    alphabet: "字符集",
    maxSecurityDescription: "包含符号的最高安全性（73个字符）",
    easyReadDescription: "易读易输入（49个字符）",
    securityNote: "安全提示：",
    securityDescription: "密码使用密码学安全的随机生成技术。不会在任何地方存储或记录。",
    noLookAlikeNote: "无混淆字母表排除易混淆字符。等效安全性至少需要{0}个字符。",
    fullAlphabetNote: "包含符号的完整字母表提供最大熵值。强安全性至少需要{0}个字符。",
    failedToGeneratePassword: "生成密码失败"
  },
  apiKey: {
    title: "API密钥生成器",
    description: "生成带ak_前缀的API密钥",
    generateApiKey: "生成API密钥",
    length: "长度",
    alphabet: "字符集",
    standardAlphanumericDescription: "标准字母数字（62个字符）",
    noConfusingDescription: "无混淆字符（49个字符）",
    formatNotice: '所有API密钥都带有"ak_"前缀以便于识别。指定长度仅指生成的随机字符（不包括前缀）。',
    securityNotice: "请安全存储API密钥，切勿在客户端代码或版本控制中暴露它们。请像对待密码一样谨慎处理。",
    formatPrefix: "ak_前缀 +",
    randomCharacters: "个随机字符，采用",
    noLookAlikeAlphabet: "无混淆字母表（易输入）",
    fullAlphanumericAlphabet: "完整字母数字字母表",
    fullAlphanumericNote: "完整字母数字字母表提供最大兼容性。强安全性至少需要{0}个字符。",
    failedToGenerateApiKey: "生成API密钥失败"
  },
  alphabets: {
    base58: "Base58（Bitcoin字母表）",
    "no-look-alike": "无混淆",
    full: "完整字母数字",
    "full-with-symbols": "包含符号"
  }
};
const ar = {
  common: {
    back: "رجوع",
    generate: "توليد",
    copy: "نسخ",
    copied: "تم النسخ!",
    backToMenu: "العودة إلى القائمة",
    loading: "جاري التوليد...",
    error: "حدث خطأ",
    result: "النتيجة",
    choose: "اختر",
    type: "النوع",
    length: "الطول",
    generated: "تم التوليد",
    format: "التنسيق",
    security: "الأمان",
    loadingVersion: "تحميل الإصدار...",
    versionsUnavailable: "الإصدارات غير متاحة",
    generationDetails: "تفاصيل التوليد",
    parametersUsed: "المعاملات المستخدمة",
    generateAnother: "توليد آخر",
    adjustSettings: "تعديل الإعدادات",
    navigateTo: "الانتقال إلى",
    selectLanguage: "اختر اللغة",
    switchToLightMode: "التبديل إلى الوضع الفاتح",
    switchToDarkMode: "التبديل إلى الوضع المظلم",
    characters: "حرف",
    alphabet: "الأبجدية",
    generatedValue: "القيمة المولدة",
    clickToSelect: "انقر على منطقة النص لتحديد الكل، أو استخدم زر النسخ",
    waitGenerating: "يرجى الانتظار أثناء توليد قيمة جديدة...",
    unknownEndpoint: "نوع نقطة نهاية غير معروف",
    failedToCopy: "فشل في النسخ",
    fallbackCopyFailed: "فشل في النسخ الاحتياطي",
    failedToRegenerate: "فشل في إعادة التوليد",
    failedToLoadVersions: "فشل في تحميل الإصدارات",
    mustBeBetween: "يجب أن يكون بين",
    and: "و",
    cannotExceed: "لا يمكن أن يتجاوز",
    optionalPrefix: "البادئة اختيارية",
    optionalSuffix: "اللاحقة اختيارية",
    prefixCannotExceed: "البادئة لا يمكن أن تتجاوز 32 حرفاً",
    suffixCannotExceed: "اللاحقة لا يمكن أن تتجاوز 32 حرفاً",
    yes: "نعم",
    no: "لا",
    seedUsed: "البذرة المستخدمة",
    copySeed: "نسخ البذرة",
    optionalSeed: "بذرة اختيارية (64 حرف سادس عشري)",
    seedInvalid: "يجب أن تكون البذرة 64 حرفاً سادس عشرياً بالضبط",
    reuseSeedTitle: "إعادة استخدام نفس البذرة؟",
    reuseSeedMessage: "هل تريد إعادة استخدام نفس البذرة لتوليد نفس النتيجة، أم تفضل توليد بذرة عشوائية جديدة؟",
    keepSameSeed: "الاحتفاظ بنفس البذرة",
    generateNewSeed: "توليد بذرة جديدة",
    seed: "بذرة"
  },
  menu: {
    title: "مولد الهاش",
    subtitle: "اختر طريقة التوليد",
    version: "الإصدار",
    brandName: "HashRand Spin",
    description: "مولد آمن تشفيرياً للهاش وكلمات المرور ومفاتيح API"
  },
  custom: {
    title: "مولد الهاش المخصص",
    description: "ولد هاش عشوائي مخصص",
    generateHash: "توليد هاش",
    length: "الطول",
    alphabet: "نوع الأبجدية",
    prefix: "البادئة",
    suffix: "اللاحقة",
    lengthMustBeBetween: "الطول يجب أن يكون بين 2 و 128",
    bitcoinDescription: "أبجدية Bitcoin، تستبعد الأحرف المربكة",
    maxReadabilityDescription: "أقصى قابلية قراءة، 49 حرف",
    completeAlphanumericDescription: "مجموعة أبجدية رقمية كاملة",
    maxEntropyDescription: "أقصى عشوائية مع الرموز",
    failedToGenerateHash: "فشل في توليد الهاش"
  },
  password: {
    title: "مولد كلمات المرور الآمنة",
    description: "ولد كلمات مرور آمنة",
    generatePassword: "توليد كلمة مرور",
    length: "الطول",
    alphabet: "مجموعة الأحرف",
    maxSecurityDescription: "أقصى أمان مع الرموز (73 حرف)",
    easyReadDescription: "سهل القراءة والكتابة (49 حرف)",
    securityNote: "ملاحظة أمنية:",
    securityDescription: "يتم توليد كلمات المرور باستخدام توليد عشوائي آمن تشفيرياً. لا يتم تخزينها أو تسجيلها في أي مكان.",
    noLookAlikeNote: "الأبجدية الواضحة تستبعد الأحرف المربكة. الحد الأدنى {0} حرفاً للأمان المكافئ.",
    fullAlphabetNote: "الأبجدية الكاملة مع الرموز توفر أقصى عشوائية. الحد الأدنى {0} حرفاً للأمان القوي.",
    failedToGeneratePassword: "فشل في توليد كلمة المرور"
  },
  apiKey: {
    title: "مولد مفاتيح API",
    description: "ولد مفاتيح API مع البادئة ak_",
    generateApiKey: "توليد مفتاح API",
    length: "الطول",
    alphabet: "مجموعة الأحرف",
    standardAlphanumericDescription: "أبجدية رقمية قياسية (62 حرف)",
    noConfusingDescription: "بدون أحرف مربكة (49 حرف)",
    formatNotice: 'جميع مفاتيح API تُولد مع البادئة "ak_" للتعرف السهل. الطول المحدد يشير فقط إلى الأحرف العشوائية المولدة (البادئة غير محسوبة).',
    securityNotice: "احفظ مفاتيح API بأمان ولا تعرضها أبداً في كود العميل أو التحكم في الإصدارات. عاملها بنفس عناية كلمات المرور.",
    formatPrefix: "بادئة ak_ +",
    randomCharacters: "حرف عشوائي مع",
    noLookAlikeAlphabet: "أبجدية واضحة (سهلة الكتابة)",
    fullAlphanumericAlphabet: "أبجدية رقمية كاملة",
    fullAlphanumericNote: "توفر الأبجدية الرقمية الكاملة أقصى توافق. الحد الأدنى {0} حرفاً للأمان القوي.",
    failedToGenerateApiKey: "فشل في توليد مفتاح API"
  },
  alphabets: {
    base58: "Base58 (أبجدية Bitcoin)",
    "no-look-alike": "واضحة",
    full: "أبجدية رقمية كاملة",
    "full-with-symbols": "كاملة مع الرموز"
  }
};
const eu = {
  common: {
    back: "Atzera",
    generate: "Sortu",
    copy: "Kopiatu",
    copied: "Kopiatuta!",
    backToMenu: "Menura itzuli",
    loading: "Sortzen...",
    error: "Errorea gertatu da",
    result: "Emaitza",
    choose: "Aukeratu",
    type: "Mota",
    length: "Luzeera",
    generated: "Sortutakoa",
    format: "Formatua",
    security: "Segurtasuna",
    loadingVersion: "Bertsioa kargatzen...",
    versionsUnavailable: "Bertsioak ez daude eskuragarri",
    generationDetails: "Sorkuntzaren Xehetasunak",
    parametersUsed: "Erabilitako Parametroak",
    generateAnother: "Beste bat sortu",
    adjustSettings: "Ezarpenak aldatu",
    navigateTo: "Hona nabigatu",
    selectLanguage: "Hizkuntza aukeratu",
    switchToLightMode: "Argi modura aldatu",
    switchToDarkMode: "Ilun modura aldatu",
    characters: "hizki",
    alphabet: "Alfabetoa",
    generatedValue: "Sortutako Balioa",
    clickToSelect: "Testu-eremuan klik egin guztia hautatzeko, edo kopiatzeko botoia erabili",
    waitGenerating: "Mesedez, itxaron balio berria sortzen den bitartean...",
    unknownEndpoint: "Amaiera-puntu mota ezezaguna",
    failedToCopy: "Kopiatzeak huts egin du",
    fallbackCopyFailed: "Ordezko kopiaketa-sistemak huts egin du",
    failedToRegenerate: "Berriz sortzeak huts egin du",
    failedToLoadVersions: "Bertsioak kargatzeak huts egin du",
    mustBeBetween: "artean egon behar du",
    and: "eta",
    cannotExceed: "ezin du gainditu",
    optionalPrefix: "Aukerako aurrizkia",
    optionalSuffix: "Aukerako atzizkia",
    prefixCannotExceed: "Aurrizkiak 32 hizki ezin ditu gainditu",
    suffixCannotExceed: "Atzizkiak 32 hizki ezin ditu gainditu",
    seedUsed: "Erabilitako Hazia",
    copySeed: "Hazia Kopiatu",
    optionalSeed: "Aukerako hazia (64 hex karaktere)",
    seedInvalid: "Haziak 64 karaktere hexadezimal izan behar ditu",
    reuseSeedTitle: "Hazia berrerabili?",
    reuseSeedMessage: "Emaitza bera sortzeko hazi bera berrerabili nahi duzu, ala ausazko hazi berria sortu nahi duzu?",
    keepSameSeed: "Hazi bera mantendu",
    generateNewSeed: "Hazi berria sortu",
    seed: "Hazia"
  },
  menu: {
    title: "Hash-Sortzailea",
    subtitle: "Sorkuntzako metodoa aukeratu",
    version: "Bertsioa",
    brandName: "HashRand Spin",
    description: "Hash, pasahitz eta API gako kriptografikoki seguruak sortzeko tresna"
  },
  custom: {
    title: "Hash-Sortzaile Pertsonalizatua",
    description: "Nahi bezalako ausazko hash-ak sortu",
    generateHash: "Hash-a sortu",
    length: "Luzeera",
    alphabet: "Alfabeto Mota",
    prefix: "Aurrizkia",
    suffix: "Atzizkia",
    lengthMustBeBetween: "Luzerak 2 eta 128 artean egon behar du",
    bitcoinDescription: "Bitcoin-alfabetoa, hizki nahasgarriak kanpoan uzten ditu",
    maxReadabilityDescription: "Irakurgarritasun handiena, 49 hizki",
    completeAlphanumericDescription: "Alfabeto alfanumeriko osoa",
    maxEntropyDescription: "Entropia handiena sinboloekin",
    failedToGenerateHash: "Hash-a sortzeak huts egin du"
  },
  password: {
    title: "Pasahitz Seguruen Sortzailea",
    description: "Pasahitz seguruak sortu",
    generatePassword: "Pasahitza sortu",
    length: "Luzeera",
    alphabet: "Hizki-multzoa",
    maxSecurityDescription: "Segurtasun handiena sinboloekin (73 hizki)",
    easyReadDescription: "Erraz irakurri eta idazteko (49 hizki)",
    securityNote: "Segurtasunari buruzko oharra:",
    securityDescription: "Pasahitzak kriptografikoki segurua den ausazko sorkuntzaren bidez sortzen dira. Ez dira inon gordetzen edo erregistratzen.",
    noLookAlikeNote: "Hizki nahasgarririk gabeko alfabetoak hizki antzekoak kanpoan uzten ditu. Segurtasun baliokiderako gutxienez {0} hizki behar dira.",
    fullAlphabetNote: "Sinboloak dituen alfabeto osoak entropia handiena ematen du. Segurtasun sendorako gutxienez {0} hizki behar dira.",
    failedToGeneratePassword: "Pasahitza sortzeak huts egin du"
  },
  apiKey: {
    title: "API Gakoen Sortzailea",
    description: "ak_ aurrizkidun API gakoak sortu",
    generateApiKey: "API gakoa sortu",
    length: "Luzeera",
    alphabet: "Hizki-multzoa",
    standardAlphanumericDescription: "Alfabeto alfanumeriko estandarra (62 hizki)",
    noConfusingDescription: "Hizki nahasgarririk gabe (49 hizki)",
    formatNotice: 'API gako guztiak "ak_" aurrizkiarekin sortzen dira identifikazioa errazagoa izateko. Zehaztutako luzerak soilik sortutako ausazko hizkiak hartzen ditu kontuan (aurrizkirik gabe).',
    securityNotice: "API gakoak modu seguruan gorde eta inoiz ez jarri bezero-aldeko kodean edo bertsio-kontrolean. Pasahitzekin bezalako arretaz tratatu.",
    formatPrefix: "ak_ aurrizkia +",
    randomCharacters: "ausazko hizki hauek erabiliz",
    noLookAlikeAlphabet: "hizki nahasgarririk gabeko alfabetoa (erraz idazteko)",
    fullAlphanumericAlphabet: "alfabeto alfanumeriko osoa",
    fullAlphanumericNote: "Alfabeto alfanumeriko osoak bateragarritasun maximoa eskaintzen du. Segurtasun sendorako gutxienez {0} hizki behar.",
    failedToGenerateApiKey: "API gakoa sortzeak huts egin du"
  },
  alphabets: {
    base58: "Base58 (Bitcoin alfabetoa)",
    "no-look-alike": "Nahasgarririk gabe",
    full: "Alfabeto Alfanumeriko Osoa",
    "full-with-symbols": "Osoa Sinboloekin"
  }
};
const ca = {
  common: {
    back: "Enrere",
    generate: "Generar",
    copy: "Copiar",
    copied: "Copiat!",
    backToMenu: "Tornar al menú",
    loading: "Generant...",
    error: "S'ha produït un error",
    result: "Resultat",
    choose: "Triar",
    type: "Tipus",
    length: "Longitud",
    generated: "Generat",
    format: "Format",
    security: "Seguretat",
    loadingVersion: "Carregant versió...",
    versionsUnavailable: "Versions no disponibles",
    generationDetails: "Detalls de generació",
    parametersUsed: "Paràmetres utilitzats",
    generateAnother: "Generar un altre",
    adjustSettings: "Ajustar configuració",
    navigateTo: "Navegar a",
    selectLanguage: "Seleccionar idioma",
    switchToLightMode: "Canviar al mode clar",
    switchToDarkMode: "Canviar al mode fosc",
    characters: "caràcters",
    alphabet: "Alfabet",
    generatedValue: "Valor generat",
    clickToSelect: "Feu clic a l'àrea de text per seleccionar-ho tot, o utilitzeu el botó de copiar",
    waitGenerating: "Si us plau, espereu mentre es genera un nou valor...",
    unknownEndpoint: "Tipus d'endpoint desconegut",
    failedToCopy: "Ha fallat la còpia",
    fallbackCopyFailed: "Ha fallat la còpia de reserva",
    failedToRegenerate: "Ha fallat la regeneració",
    failedToLoadVersions: "Ha fallat carregar les versions",
    mustBeBetween: "ha d'estar entre",
    and: "i",
    cannotExceed: "no pot superar",
    optionalPrefix: "Prefix opcional",
    optionalSuffix: "Sufix opcional",
    prefixCannotExceed: "El prefix no pot superar 32 caràcters",
    suffixCannotExceed: "El sufix no pot superar 32 caràcters",
    seedUsed: "Llavor Utilitzada",
    copySeed: "Copiar Llavor",
    optionalSeed: "Llavor opcional (64 caràcters hex)",
    seedInvalid: "La llavor ha de tenir exactament 64 caràcters hexadecimals",
    reuseSeedTitle: "Reutilitzar la mateixa llavor?",
    reuseSeedMessage: "Voleu reutilitzar la mateixa llavor per generar el mateix resultat, o preferiu generar una nova llavor aleatòria?",
    keepSameSeed: "Mantenir la mateixa llavor",
    generateNewSeed: "Generar nova llavor",
    seed: "Llavor"
  },
  menu: {
    title: "Generador de Hash",
    subtitle: "Trieu un mètode de generació",
    version: "Versió",
    brandName: "HashRand Spin",
    description: "Generador criptogràficament segur de hashs, contrasenyes i claus API"
  },
  custom: {
    title: "Generador de Hash personalitzat",
    description: "Genereu hashs aleatoris personalitzats",
    generateHash: "Generar Hash",
    length: "Longitud",
    alphabet: "Tipus d'alfabet",
    prefix: "Prefix",
    suffix: "Sufix",
    lengthMustBeBetween: "La longitud ha d'estar entre 2 i 128",
    bitcoinDescription: "Alfabet Bitcoin, exclou caràcters confusos",
    maxReadabilityDescription: "Màxima llegibilitat, 49 caràcters",
    completeAlphanumericDescription: "Conjunt alfanumèric complet",
    maxEntropyDescription: "Màxima entropia amb símbols",
    failedToGenerateHash: "Ha fallat generar el hash"
  },
  password: {
    title: "Generador de contrasenyes segures",
    description: "Genereu contrasenyes segures",
    generatePassword: "Generar Contrasenya",
    length: "Longitud",
    alphabet: "Conjunt de caràcters",
    maxSecurityDescription: "Màxima seguretat amb símbols (73 caràcters)",
    easyReadDescription: "Fàcil de llegir i escriure (49 caràcters)",
    securityNote: "Nota de seguretat:",
    securityDescription: "Les contrasenyes es generen utilitzant generació aleatòria criptogràficament segura. No s'emmagatzemen ni es registren enlloc.",
    noLookAlikeNote: "L'alfabet sense confusió exclou caràcters confusos. Mínim {0} caràcters per seguretat equivalent.",
    fullAlphabetNote: "L'alfabet complet amb símbols proporciona màxima entropia. Mínim {0} caràcters per seguretat forta.",
    failedToGeneratePassword: "Ha fallat generar la contrasenya"
  },
  apiKey: {
    title: "Generador de claus API",
    description: "Genereu claus API amb prefix ak_",
    generateApiKey: "Generar Clau API",
    length: "Longitud",
    alphabet: "Conjunt de caràcters",
    standardAlphanumericDescription: "Alfanumèric estàndard (62 caràcters)",
    noConfusingDescription: "Sense caràcters confusos (49 caràcters)",
    formatNotice: 'Totes les claus API es generen amb el prefix "ak_" per facilitar la identificació. La longitud especificada es refereix només als caràcters aleatoris generats (prefix no comptat).',
    securityNotice: "Emmagatzemeu les claus API de forma segura i mai les exposeu en codi del costat del client o control de versions. Tracteu-les amb la mateixa cura que les contrasenyes.",
    formatPrefix: "prefix ak_ +",
    randomCharacters: "caràcters aleatoris amb",
    noLookAlikeAlphabet: "alfabet sense confusió (fàcil d'escriure)",
    fullAlphanumericAlphabet: "alfabet alfanumèric complet",
    fullAlphanumericNote: "L'alfabet alfanumèric complet proporciona màxima compatibilitat. Mínim {0} caràcters per a seguretat forta.",
    failedToGenerateApiKey: "Ha fallat generar la clau API"
  },
  alphabets: {
    base58: "Base58 (alfabet Bitcoin)",
    "no-look-alike": "Sense confusió",
    full: "Alfanumèric complet",
    "full-with-symbols": "Complet amb símbols"
  }
};
const gl = {
  common: {
    back: "Atrás",
    generate: "Xerar",
    copy: "Copiar",
    copied: "Copiado!",
    backToMenu: "Volver ao menú",
    loading: "Xerando...",
    error: "Produciuse un erro",
    result: "Resultado",
    choose: "Escoller",
    type: "Tipo",
    length: "Lonxitude",
    generated: "Xerado",
    format: "Formato",
    security: "Seguridade",
    loadingVersion: "Cargando versión...",
    versionsUnavailable: "Versións non dispoñibles",
    generationDetails: "Detalles de xeración",
    parametersUsed: "Parámetros utilizados",
    generateAnother: "Xerar outro",
    adjustSettings: "Axustar configuración",
    navigateTo: "Navegar a",
    selectLanguage: "Seleccionar idioma",
    switchToLightMode: "Cambiar ao modo claro",
    switchToDarkMode: "Cambiar ao modo escuro",
    characters: "caracteres",
    alphabet: "Alfabeto",
    generatedValue: "Valor xerado",
    clickToSelect: "Fai clic na área de texto para seleccionar todo, ou usa o botón copiar",
    waitGenerating: "Por favor, agarda mentres se xera un novo valor...",
    unknownEndpoint: "Tipo de endpoint descoñecido",
    failedToCopy: "Fallou ao copiar",
    fallbackCopyFailed: "Fallou a copia de respaldo",
    failedToRegenerate: "Fallou ao rexerar",
    failedToLoadVersions: "Fallou cargar as versións",
    mustBeBetween: "debe estar entre",
    and: "e",
    cannotExceed: "non pode superar",
    optionalPrefix: "Prefixo opcional",
    optionalSuffix: "Sufixo opcional",
    prefixCannotExceed: "O prefixo non pode superar 32 caracteres",
    suffixCannotExceed: "O sufixo non pode superar 32 caracteres",
    seedUsed: "Semente Utilizada",
    copySeed: "Copiar Semente",
    optionalSeed: "Semente opcional (64 caracteres hex)",
    seedInvalid: "A semente debe ter exactamente 64 caracteres hexadecimais",
    reuseSeedTitle: "Reutilizar a mesma semente?",
    reuseSeedMessage: "Queres reutilizar a mesma semente para xerar o mesmo resultado, ou prefires xerar unha nova semente aleatoria?",
    keepSameSeed: "Manter a mesma semente",
    generateNewSeed: "Xerar nova semente",
    seed: "Semente"
  },
  menu: {
    title: "Xerador de Hash",
    subtitle: "Escolle un método de xeración",
    version: "Versión",
    brandName: "HashRand Spin",
    description: "Xerador criptograficamente seguro de hashes, contrasinais e chaves API"
  },
  custom: {
    title: "Xerador de Hash personalizado",
    description: "Xera hashes aleatorios personalizados",
    generateHash: "Xerar Hash",
    length: "Lonxitude",
    alphabet: "Tipo de alfabeto",
    prefix: "Prefixo",
    suffix: "Sufixo",
    lengthMustBeBetween: "A lonxitude debe estar entre 2 e 128",
    bitcoinDescription: "Alfabeto Bitcoin, exclúe caracteres confusos",
    maxReadabilityDescription: "Máxima lexibilidade, 49 caracteres",
    completeAlphanumericDescription: "Conxunto alfanumérico completo",
    maxEntropyDescription: "Máxima entropía con símbolos",
    failedToGenerateHash: "Fallou xerar o hash"
  },
  password: {
    title: "Xerador de contrasinais seguros",
    description: "Xera contrasinais seguros",
    generatePassword: "Xerar Contrasinal",
    length: "Lonxitude",
    alphabet: "Conxunto de caracteres",
    maxSecurityDescription: "Máxima seguridade con símbolos (73 caracteres)",
    easyReadDescription: "Fácil de ler e escribir (49 caracteres)",
    securityNote: "Nota de seguridade:",
    securityDescription: "Os contrasinais xéranse usando xeración aleatoria criptograficamente segura. Non se almacenan nin se rexistran en ningures.",
    noLookAlikeNote: "O alfabeto sen confusión exclúe letras confundibles. Mínimo {0} letras para seguridade equivalente.",
    fullAlphabetNote: "O alfabeto completo con símbolos proporciona máxima entropía. Mínimo {0} letras para seguridade forte.",
    failedToGeneratePassword: "Fallou xerar o contrasinal"
  },
  apiKey: {
    title: "Xerador de chaves API",
    description: "Xera chaves API con prefixo ak_",
    generateApiKey: "Xerar Chave API",
    length: "Lonxitude",
    alphabet: "Conxunto de caracteres",
    standardAlphanumericDescription: "Alfanumérico estándar (62 caracteres)",
    noConfusingDescription: "Sen letras confusas (49 letras)",
    formatNotice: 'Todas as chaves API xéranse co prefixo "ak_" para facilitar a identificación. A lonxitude especificada refírese só ás letras aleatorias xeradas (prefixo non contado).',
    securityNotice: "Almacena as chaves API de forma segura e nunca as exponñas en código do lado do cliente ou control de versións. Trátalas co mesmo coidado que os contrasinais.",
    formatPrefix: "prefixo ak_ +",
    randomCharacters: "letras aleatorias con",
    noLookAlikeAlphabet: "alfabeto sen confusión (fácil de escribir)",
    fullAlphanumericAlphabet: "alfabeto alfanumérico completo",
    fullAlphanumericNote: "O alfabeto alfanumérico completo proporciona máxima compatibilidade. Mínimo {0} caracteres para seguridade forte.",
    failedToGenerateApiKey: "Fallou xerar a chave API"
  },
  alphabets: {
    base58: "Base58 (alfabeto Bitcoin)",
    "no-look-alike": "Sen confusión",
    full: "Alfanumérico completo",
    "full-with-symbols": "Completo con símbolos"
  }
};
function detectBrowserLanguage() {
  if (typeof window === "undefined") {
    return "en";
  }
  const browserLanguages = [navigator.language, ...navigator.languages || []];
  const supportedLanguages = new Set(getSupportedLanguageCodes());
  for (const browserLang of browserLanguages) {
    const langCode = browserLang.split("-")[0].toLowerCase();
    if (supportedLanguages.has(langCode)) {
      return langCode;
    }
  }
  return "en";
}
function initializeLanguage() {
  if (typeof window === "undefined") {
    return "en";
  }
  const storedLang = localStorage.getItem("preferred-language");
  if (storedLang && getSupportedLanguageCodes().includes(storedLang)) {
    return storedLang;
  }
  const detectedLang = detectBrowserLanguage();
  localStorage.setItem("preferred-language", detectedLang);
  return detectedLang;
}
const currentLanguage = writable(initializeLanguage());
currentLanguage.subscribe((language) => {
  if (typeof window !== "undefined") {
    localStorage.setItem("preferred-language", language);
  }
});
const translations = {
  // Clean modular translations
  en,
  es,
  pt,
  fr,
  de,
  ru,
  zh,
  ar,
  eu,
  ca,
  gl,
  hi,
  ja
};
function t(key, lang = "en") {
  const keys = key.split(".");
  function getTranslationFromLang(targetLang) {
    let value = translations[targetLang];
    for (const k of keys) {
      if (value && typeof value === "object" && k in value) {
        value = value[k];
      } else {
        return null;
      }
    }
    return typeof value === "string" ? value : null;
  }
  let result = getTranslationFromLang(lang);
  if (result) return result;
  if (lang !== "en") {
    result = getTranslationFromLang("en");
    if (result) return result;
  }
  return key;
}
const _ = derived(currentLanguage, (lang) => {
  return (key) => t(key, lang);
});
derived(currentLanguage, (lang) => {
  return translations[lang] || translations.en;
});
const RTL_LANGUAGES = /* @__PURE__ */ new Set(["ar"]);
const isRTL = derived(currentLanguage, (lang) => {
  return RTL_LANGUAGES.has(lang);
});
derived(isRTL, (rtl) => {
  return rtl ? "rtl" : "ltr";
});
export {
  Icon as I,
  _,
  currentLanguage as c,
  findLanguageByCode as f,
  isRTL as i
};
