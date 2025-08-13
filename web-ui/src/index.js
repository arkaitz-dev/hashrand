// Import router
import { Router } from '@vaadin/router';

// Import localization configuration
import { getLocale, setLocale } from './localization.js';
import { sourceLocale, targetLocales } from './locales/locale-codes.js';

// Import API utilities
import { apiFetch } from './utils/api.js';

// Import all Lit components
import './components/header-title.js';
import './components/language-selector.js';
import './pages/menu.js';
import './pages/custom-hash.js';
import './pages/password.js';
import './pages/api-key.js';
import './pages/hash-result.js';


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

// Initialize locale when DOM is ready  
document.addEventListener('DOMContentLoaded', async () => {
    
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

    // Initialize router
    const router = new Router(document.getElementById('router-outlet'));
    router.setRoutes([
        {
            path: '/',
            component: 'menu-page'
        },
        {
            path: '/custom',
            component: 'custom-hash-page'
        },
        {
            path: '/password',
            component: 'password-page'
        },
        {
            path: '/api-key',
            component: 'api-key-page'
        },
        {
            path: '/custom/result',
            component: 'hash-result-page'
        },
        {
            path: '/password/result',
            component: 'hash-result-page'
        },
        {
            path: '/api-key/result',
            component: 'hash-result-page'
        }
    ]);
});