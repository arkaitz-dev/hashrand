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
                class="bg-transparent border border-transparent text-white px-3 py-1.5 rounded-md cursor-pointer text-base flex items-center gap-1.5 transition-all duration-200 hover:bg-white hover:bg-opacity-10 hover:border-white hover:border-opacity-20 focus:outline-none" 
                @click=${this.toggleDropdown}
            >
                <span>${this.getLanguageFlag(this.currentLocale)}</span>
                <span>🌐</span>
            </button>
            
            <div class="absolute top-full z-50 min-w-[180px] mt-1 bg-white rounded-lg shadow-xl transition-all duration-200 ${this.showDropdown ? 'opacity-100 pointer-events-auto' : 'opacity-0 pointer-events-none'} ${isRTL ? 'left-0' : 'right-0'}">
                ${allLocales.map(locale => html`
                    <button
                        class="flex items-center gap-2 w-full px-4 py-3 text-slate-700 cursor-pointer border-none bg-none text-left text-base hover:bg-gray-50 transition-colors duration-150 ${locale === this.currentLocale ? 'bg-blue-50 text-blue-600 font-medium' : ''}"
                        @click=${() => this.selectLanguage(locale)}
                    >
                        <span>${this.getLanguageFlag(locale)}</span>
                        <span>${this.getLocaleDisplayName(locale)}</span>
                    </button>
                `)}
            </div>
        `;
    }
}

customElements.define('language-selector', LanguageSelector);