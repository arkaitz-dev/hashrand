import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { updateWhenLocaleChanges } from '@lit/localize';
import { getLocale, setLocale } from '../localization.js';
import { allLocales } from '../locales/locale-codes.js';
import sharedStyles from '../shared-styles.js';

export class LanguageSelector extends LitElement {
    static properties = {
        currentLocale: { type: String },
        showDropdown: { type: Boolean }
    };

    constructor() {
        super();
        this.currentLocale = 'en';
        this.showDropdown = false;
    }

    static styles = [
        sharedStyles,
        css`
            :host {
                display: inline-block !important;
                position: relative;
                visibility: visible !important;
            }
        `
    ];

    connectedCallback() {
        super.connectedCallback();
        console.log('LanguageSelector connected, current locale:', getLocale());
        
        updateWhenLocaleChanges(this);
        
        // Update currentLocale from actual locale
        this.currentLocale = getLocale();
        
        // Listen for locale changes
        this.updateLocaleFromSystem();
        
        // Set initial dropdown alignment
        this.updateDropdownAlignment();

        document.addEventListener('click', this.handleDocumentClick.bind(this));
    }

    updateLocaleFromSystem() {
        // Check periodically for locale changes until it's properly set
        const checkLocale = () => {
            const actualLocale = getLocale();
            if (this.currentLocale !== actualLocale) {
                console.log(`LanguageSelector updating from ${this.currentLocale} to ${actualLocale}`);
                this.currentLocale = actualLocale;
                this.updateDropdownAlignment();
                this.requestUpdate();
            }
        };
        
        // Check immediately and after a short delay to handle async locale setting
        checkLocale();
        setTimeout(checkLocale, 100);
        setTimeout(checkLocale, 500);
    }

    updateDropdownAlignment() {
        // Update dropdown alignment based on document direction
        this.updateComplete.then(() => {
            const dropdown = this.shadowRoot.querySelector('.language-dropdown');
            if (dropdown) {
                const isRTL = document.documentElement.dir === 'rtl';
                if (isRTL) {
                    dropdown.classList.add('rtl-align');
                } else {
                    dropdown.classList.remove('rtl-align');
                }
            }
        });
    }

    disconnectedCallback() {
        super.disconnectedCallback();
        document.removeEventListener('click', this.handleDocumentClick.bind(this));
    }

    handleDocumentClick(event) {
        if (!this.shadowRoot.contains(event.target)) {
            this.showDropdown = false;
        }
    }

    toggleDropdown(event) {
        event.stopPropagation();
        this.showDropdown = !this.showDropdown;
    }

    async selectLanguage(locale) {
        if (locale !== this.currentLocale) {
            console.log(`Changing locale to: ${locale}`);
            await setLocale(locale);
            this.currentLocale = locale;
            
            // Update document properties
            document.documentElement.lang = locale;
            document.documentElement.dir = locale === 'ar' ? 'rtl' : 'ltr';
            
            // Update dropdown alignment after direction change
            this.updateDropdownAlignment();
            
            // Store preference
            localStorage.setItem('hashrand-locale', locale);
        }
        this.showDropdown = false;
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
            'eu': 'EUS',  // Euskara
            'ca': 'CAT',  // Català  
            'gl': 'GAL',  // Galego
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
                class="bg-white bg-opacity-10 backdrop-blur-sm border border-white border-opacity-20 text-white px-3 py-1.5 rounded-lg cursor-pointer text-base flex items-center gap-2 transition-all duration-200 hover:bg-opacity-20 focus:outline-none focus:ring-2 focus:ring-white focus:ring-opacity-30" 
                @click=${this.toggleDropdown}
                aria-label="Select language"
                aria-expanded="${this.showDropdown}"
            >
                <span class="text-lg">${this.getLanguageFlag(this.currentLocale)}</span>
                <span class="text-lg">🌐</span>
                <svg class="w-4 h-4 transition-transform duration-200 ${this.showDropdown ? 'rotate-180' : ''}" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"/>
                </svg>
            </button>
            
            ${this.showDropdown ? html`
                <ul class="absolute top-full right-0 z-50 min-w-[180px] mt-2 bg-white dark:bg-gray-800 rounded-lg shadow-xl border border-gray-200 dark:border-gray-700 overflow-hidden list-none p-0 m-0">
                    ${allLocales.map(locale => html`
                        <li class="m-0 p-0">
                            <button
                                class="flex items-center gap-3 w-full px-4 py-2.5 text-gray-700 dark:text-gray-200 cursor-pointer border-none bg-transparent text-left text-sm hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors duration-150 ${locale === this.currentLocale ? 'bg-blue-50 dark:bg-blue-900 text-blue-600 dark:text-blue-400 font-semibold' : ''}"
                                @click=${() => this.selectLanguage(locale)}
                                role="option"
                                aria-selected="${locale === this.currentLocale}"
                            >
                                <span class="text-base">${this.getLanguageFlag(locale)}</span>
                                <span>${this.getLocaleDisplayName(locale)}</span>
                            </button>
                        </li>
                    `)}
                </ul>
            ` : ''}
        `;
    }
}

customElements.define('language-selector', LanguageSelector);