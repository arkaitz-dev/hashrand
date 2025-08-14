import { LitElement, html } from 'lit';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import { getLocale, setLocale } from '../localization.js';
import { allLocales } from '../locales/locale-codes.js';
import { sharedStyles } from '../shared-styles.js';

export class ConfigViewMode extends LitElement {
    static properties = {
        currentLocale: { type: String },
        isOpen: { type: Boolean },
        isDark: { type: Boolean },
        version: { type: String }
    };

    constructor() {
        super();
        this.currentLocale = 'en';
        this.isOpen = false;
        this.isDark = false;
        this.version = '';
        this._documentClickHandler = this._handleDocumentClick.bind(this);
        
        // Detectar tema igual que theme-toggle
        this.detectTheme();
        this.fetchVersion();
    }
    
    detectTheme() {
        const savedTheme = localStorage.getItem('hashrand-theme');
        if (savedTheme) {
            this.isDark = savedTheme === 'dark';
        } else {
            this.isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        }
    }
    
    async fetchVersion() {
        try {
            const response = await fetch('/api/version');
            const data = await response.json();
            this.version = data.version;
        } catch (error) {
            console.error('Error fetching version:', error);
            this.version = '';
        }
    }

    static styles = sharedStyles;

    connectedCallback() {
        super.connectedCallback();
        updateWhenLocaleChanges(this);
        
        // Detectar idioma igual que en index.js
        this.detectAndSetCurrentLocale();
        
        document.addEventListener('click', this._documentClickHandler);
        
        // Update locale periodically in case it changes
        setTimeout(() => {
            this.detectAndSetCurrentLocale();
        }, 100);
        
        setTimeout(() => {
            this.detectAndSetCurrentLocale();
        }, 500);
    }
    
    detectAndSetCurrentLocale() {
        // Misma lógica que index.js para detectar idioma
        const storedLocale = localStorage.getItem('hashrand-locale');
        if (storedLocale && this.isValidLocale(storedLocale)) {
            this.currentLocale = storedLocale;
            return;
        }
        
        // Detectar del sistema
        const browserLocales = navigator.languages || [navigator.language];
        for (const browserLocale of browserLocales) {
            const langCode = browserLocale.split('-')[0].toLowerCase();
            if (this.isValidLocale(langCode)) {
                this.currentLocale = langCode;
                return;
            }
            if (browserLocale.toLowerCase().includes('zh')) {
                this.currentLocale = 'zh';
                return;
            }
        }
        
        // Fallback
        this.currentLocale = 'en';
    }
    
    isValidLocale(locale) {
        return allLocales.includes(locale);
    }

    disconnectedCallback() {
        super.disconnectedCallback();
        document.removeEventListener('click', this._documentClickHandler);
    }

    _handleDocumentClick(event) {
        if (!this.contains(event.target)) {
            this.isOpen = false;
            // Blur the button when clicking outside
            const button = this.shadowRoot.querySelector('.selector-button');
            if (button) {
                /** @type {HTMLElement} */ (button).blur();
            }
        }
    }

    toggleDropdown(event) {
        event.stopPropagation();
        this.isOpen = !this.isOpen;
    }

    async selectLanguage(locale, event) {
        event.stopPropagation();
        if (locale !== this.currentLocale) {
            await setLocale(locale);
            this.currentLocale = locale;
            document.documentElement.lang = locale;
            document.documentElement.dir = locale === 'ar' ? 'rtl' : 'ltr';
            localStorage.setItem('hashrand-locale', locale);
        }
        this.isOpen = false;
        // Blur the button after selection
        const button = this.shadowRoot.querySelector('.selector-button');
        if (button) {
            /** @type {HTMLElement} */ (button).blur();
        }
    }
    
    toggleTheme(event) {
        event.stopPropagation();
        this.isDark = !this.isDark;
        
        const html = document.documentElement;
        const body = document.body;
        
        if (this.isDark) {
            html.classList.add('dark');
            body.style.backgroundColor = '#1f2937'; // gray-800 - SAME as preloader
        } else {
            html.classList.remove('dark');
            body.style.backgroundColor = '#f9fafb'; // gray-50
        }
        
        localStorage.setItem('hashrand-theme', this.isDark ? 'dark' : 'light');
        this.isOpen = false;
        // Blur the button after theme toggle
        const button = this.shadowRoot.querySelector('.selector-button');
        if (button) {
            /** @type {HTMLElement} */ (button).blur();
        }
    }

    getLanguageFlag(locale) {
        const flags = {
            'en': '🇬🇧',
            'es': '🇪🇸',
            'fr': '🇫🇷',
            'pt': '🇵🇹',
            'de': '🇩🇪',
            'ru': '🇷🇺',
            'zh': '🇨🇳',
            'ar': '🇸🇦',
            'eu': '🇪🇺',
            'ca': '🇪🇸',
            'gl': '🇪🇸',
            'ja': '🇯🇵'
        };
        return flags[locale] || '🌐';
    }

    getLocaleDisplayName(locale) {
        const names = {
            'en': 'English',
            'es': 'Español',
            'fr': 'Français',
            'pt': 'Português',
            'de': 'Deutsch',
            'ru': 'Русский',
            'zh': '中文',
            'ar': 'العربية',
            'eu': 'Euskara',
            'ca': 'Català',
            'gl': 'Galego',
            'ja': '日本語'
        };
        return names[locale] || locale;
    }

    render() {
        const isRTL = document.documentElement.dir === 'rtl';
        return html`
            <button 
                class="flex items-center gap-2 px-2 py-2 bg-transparent border border-transparent rounded-lg text-white cursor-pointer text-sm transition-all duration-200 w-auto justify-center hover:bg-gray-100 hover:bg-opacity-15 focus:outline-none"
                @click=${this.toggleDropdown}
                aria-label="Select language"
                aria-expanded="${this.isOpen}"
                aria-haspopup="listbox"
            >
                <div class="flex items-center gap-2">
                    <span class="text-lg leading-none">${this.getLanguageFlag(this.currentLocale)}</span>
                    <span class="text-base leading-none">${this.isDark ? '🌙' : '☀️'}</span>
                </div>
                <svg class="w-3 h-3 transition-transform duration-200 text-white text-opacity-90 fill-current ${this.isOpen ? 'rotate-180' : ''}" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"/>
                </svg>
            </button>
            
            <div class="absolute top-full right-0 mt-2 min-w-45 max-h-96 bg-white rounded-lg shadow-lg border border-gray-200 overflow-y-auto z-1000 transition-all duration-200 ${this.isOpen ? 'opacity-100 visible translate-y-0' : 'opacity-0 invisible -translate-y-2'} ${isRTL ? 'left-0 right-auto' : ''}" role="listbox">
                <ul class="list-none p-1 m-0">
                    <!-- Theme toggle as first item -->
                    <li class="m-0 p-0">
                        <button
                            class="flex items-center gap-2 w-full px-3 py-2 bg-transparent border-none rounded text-gray-700 cursor-pointer text-sm text-left transition-colors duration-200 border-b border-gray-200 mb-1 font-medium hover:bg-gray-100"
                            @click=${this.toggleTheme}
                            role="option"
                        >
                            <span class="flag">${this.isDark ? '☀️' : '🌙'}</span>
                            <span>${this.isDark ? msg('Switch to Light') : msg('Switch to Dark')}</span>
                        </button>
                    </li>
                    <!-- Version separator -->
                    ${this.version ? html`
                        <li class="m-0 p-0">
                            <div class="px-3 py-2 text-center text-xs text-gray-500 bg-transparent border-none cursor-default font-normal">v${this.version}</div>
                        </li>
                    ` : ''}
                    <!-- Separator -->
                    <li class="h-px bg-gray-200 my-1 list-none"></li>
                    <!-- Language options -->
                    ${allLocales.map(locale => html`
                        <li class="m-0 p-0">
                            <button
                                class="flex items-center gap-2 w-full px-3 py-2 bg-transparent border-none rounded text-gray-700 cursor-pointer text-sm text-left transition-colors duration-200 ${locale === this.currentLocale ? 'bg-blue-50 text-blue-600 font-semibold' : ''} hover:bg-gray-100"
                                @click=${(e) => this.selectLanguage(locale, e)}
                                role="option"
                                aria-selected="${locale === this.currentLocale}"
                            >
                                <span class="flag">${this.getLanguageFlag(locale)}</span>
                                <span>${this.getLocaleDisplayName(locale)}</span>
                            </button>
                        </li>
                    `)}
                </ul>
            </div>
        `;
    }
}

customElements.define('config-view-mode', ConfigViewMode);