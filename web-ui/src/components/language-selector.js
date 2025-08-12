import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { updateWhenLocaleChanges } from '@lit/localize';
import { getLocale, setLocale } from '../localization.js';
import { allLocales } from '../locales/locale-codes.js';

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

    static styles = css`
        :host {
            display: inline-block !important;
            position: relative;
            visibility: visible !important;
        }

        .language-button {
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.2);
            color: white;
            padding: 0.25rem 0.5rem;
            border-radius: 4px;
            cursor: pointer;
            font-size: 0.8rem;
            display: flex;
            align-items: center;
            gap: 0.25rem;
        }

        .language-dropdown {
            position: absolute;
            top: 100%;
            left: 0;
            background: white;
            border-radius: 8px;
            box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
            z-index: 1000;
            min-width: 160px;
            opacity: 0;
            pointer-events: none;
            margin-top: 0.25rem;
        }

        .language-dropdown.show {
            opacity: 1;
            pointer-events: auto;
        }

        .language-option {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.75rem 1rem;
            color: #2c3e50;
            cursor: pointer;
            border: none;
            background: none;
            width: 100%;
            text-align: left;
            font-size: 0.9rem;
        }

        .language-option:hover {
            background: #f8f9fa;
        }

        .language-option.current {
            background: #e3f2fd;
            color: #1976d2;
            font-weight: 500;
        }
    `;

    connectedCallback() {
        super.connectedCallback();
        console.log('LanguageSelector connected, current locale:', getLocale());
        
        updateWhenLocaleChanges(this);
        
        // Update currentLocale from actual locale
        this.currentLocale = getLocale();
        
        // Listen for locale changes
        this.updateLocaleFromSystem();

        document.addEventListener('click', this.handleDocumentClick.bind(this));
    }

    updateLocaleFromSystem() {
        // Check periodically for locale changes until it's properly set
        const checkLocale = () => {
            const actualLocale = getLocale();
            if (this.currentLocale !== actualLocale) {
                console.log(`LanguageSelector updating from ${this.currentLocale} to ${actualLocale}`);
                this.currentLocale = actualLocale;
                this.requestUpdate();
            }
        };
        
        // Check immediately and after a short delay to handle async locale setting
        checkLocale();
        setTimeout(checkLocale, 100);
        setTimeout(checkLocale, 500);
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
            'ar': '🇸🇦'
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
            'ar': 'العربية'
        };
        return names[locale] || locale;
    }

    render() {
        return html`
            <button 
                class="language-button" 
                @click=${this.toggleDropdown}
            >
                <span>${this.getLanguageFlag(this.currentLocale)}</span>
                <span>${this.getLocaleDisplayName(this.currentLocale)}</span>
            </button>
            
            <div class="language-dropdown ${this.showDropdown ? 'show' : ''}">
                ${allLocales.map(locale => html`
                    <button
                        class="language-option ${locale === this.currentLocale ? 'current' : ''}"
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