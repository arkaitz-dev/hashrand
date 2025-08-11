// Import localization configuration
import { getLocale, setLocale } from './localization.js';
import { sourceLocale, targetLocales } from './locales/locale-codes.js';

// Import all Lit components
import './components/hash-generator.js';
import './components/generic-hash-view.js';
import './components/password-view.js';
import './components/api-key-view.js';
import './components/hash-result.js';
import './components/language-selector.js';

// Load version from API
async function loadVersion() {
    try {
        const response = await fetch('/api/version');
        if (response.ok) {
            const data = await response.json();
            const versionElement = document.getElementById('version');
            if (versionElement) {
                versionElement.textContent = `v${data.version}`;
            }
        }
    } catch (error) {
        console.warn('Could not load version:', error);
    }
}

// Initialize locale detection and persistence
function determineInitialLocale() {
    // 1. Check stored locale
    const storedLocale = localStorage.getItem('hashrand-locale');
    if (storedLocale && [sourceLocale, ...targetLocales].includes(storedLocale)) {
        return storedLocale;
    }

    // 2. Browser preferences
    const browserLocales = navigator.languages || [navigator.language];
    for (const browserLocale of browserLocales) {
        const langCode = browserLocale.split('-')[0].toLowerCase();
        if ([sourceLocale, ...targetLocales].includes(langCode)) {
            return langCode;
        }
        if (browserLocale.toLowerCase().includes('zh')) {
            return 'zh';
        }
    }

    return sourceLocale;
}

// Load version and initialize locale when DOM is ready  
document.addEventListener('DOMContentLoaded', async () => {
    await loadVersion();
    
    // Set initial locale
    const initialLocale = determineInitialLocale();
    console.log('Initial locale determined:', initialLocale);
    
    if (initialLocale !== sourceLocale) {
        await setLocale(initialLocale);
        console.log('Locale set to:', initialLocale);
    }
    
    // Set document properties
    document.documentElement.lang = getLocale();
    document.documentElement.dir = getLocale() === 'ar' ? 'rtl' : 'ltr';
});