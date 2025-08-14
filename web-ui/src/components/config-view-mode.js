import { LitElement, html, css } from 'lit';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import { getLocale, setLocale } from '../localization.js';
import { allLocales } from '../locales/locale-codes.js';

export class ConfigViewMode extends LitElement {
    static properties = {
        currentLocale: { type: String },
        isOpen: { type: Boolean },
        isDark: { type: Boolean }
    };

    constructor() {
        super();
        this.currentLocale = 'en';
        this.isOpen = false;
        this.isDark = false;
        this._documentClickHandler = this._handleDocumentClick.bind(this);
        
        // Detectar tema igual que theme-toggle
        this.detectTheme();
    }
    
    detectTheme() {
        const savedTheme = localStorage.getItem('hashrand-theme');
        if (savedTheme) {
            this.isDark = savedTheme === 'dark';
        } else {
            this.isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        }
    }

    static styles = css`
        :host {
            display: block;
            position: relative;
        }

        .selector-button {
            display: flex;
            align-items: center;
            gap: 6px;
            padding: 8px 10px;
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 8px;
            color: white;
            cursor: pointer;
            font-size: 14px;
            transition: all 0.2s ease;
            width: auto;
            justify-content: center;
        }

        .selector-button:hover {
            background: rgba(255, 255, 255, 0.15);
            border-color: rgba(255, 255, 255, 0.3);
        }

        .selector-button:focus {
            outline: none;
            box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.3);
        }

        .button-content {
            display: flex;
            align-items: center;
            gap: 6px;
        }

        .flag {
            font-size: 18px;
            line-height: 1;
        }

        .theme-icon {
            font-size: 16px;
            line-height: 1;
        }

        .arrow {
            width: 12px;
            height: 12px;
            transition: transform 0.2s ease;
            color: white;
            fill: currentColor;
        }

        .arrow.open {
            transform: rotate(180deg);
        }

        .dropdown {
            position: absolute;
            top: calc(100% + 8px);
            right: 0;
            min-width: 180px;
            max-height: 300px;
            background: white;
            border-radius: 8px;
            box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
            border: 1px solid #e5e7eb;
            overflow-y: auto;
            z-index: 1000;
            opacity: 0;
            visibility: hidden;
            transform: translateY(-10px);
            transition: all 0.2s ease;
        }

        .dropdown.rtl {
            right: auto;
            left: 0;
        }

        .dropdown-separator {
            height: 1px;
            background: #e5e7eb;
            margin: 4px 0;
            list-style: none;
        }

        .theme-button {
            border-bottom: 1px solid #e5e7eb;
            margin-bottom: 4px;
            font-weight: 500;
        }

        /* Dark mode styles for new elements */
        :host-context(.dark) .dropdown-separator {
            background: #374151;
        }

        :host-context(.dark) .theme-button {
            border-bottom-color: #374151;
        }

        .dropdown.open {
            opacity: 1;
            visibility: visible;
            transform: translateY(0);
        }

        .dropdown-list {
            list-style: none;
            padding: 4px;
            margin: 0;
        }

        .dropdown-item {
            margin: 0;
            padding: 0;
        }

        .dropdown-button {
            display: flex;
            align-items: center;
            gap: 10px;
            width: 100%;
            padding: 10px 12px;
            background: transparent;
            border: none;
            border-radius: 4px;
            color: #374151;
            cursor: pointer;
            font-size: 14px;
            text-align: left;
            transition: background-color 0.15s ease;
        }

        .dropdown-button:hover {
            background-color: #f3f4f6;
        }

        .dropdown-button.active {
            background-color: #eff6ff;
            color: #2563eb;
            font-weight: 600;
        }

        /* Dark mode styles */
        :host-context(.dark) .dropdown {
            background: #1f2937;
            border-color: #374151;
        }

        :host-context(.dark) .dropdown-button {
            color: #e5e7eb;
        }

        :host-context(.dark) .dropdown-button:hover {
            background-color: #374151;
        }

        :host-context(.dark) .dropdown-button.active {
            background-color: #1e3a8a;
            color: #60a5fa;
        }
    `;

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
                class="selector-button"
                @click=${this.toggleDropdown}
                aria-label="Select language"
                aria-expanded="${this.isOpen}"
                aria-haspopup="listbox"
            >
                <div class="button-content">
                    <span class="flag">${this.getLanguageFlag(this.currentLocale)}</span>
                    <span class="theme-icon">${this.isDark ? '🌙' : '☀️'}</span>
                </div>
                <svg class="arrow ${this.isOpen ? 'open' : ''}" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"/>
                </svg>
            </button>
            
            <div class="dropdown ${this.isOpen ? 'open' : ''} ${isRTL ? 'rtl' : ''}" role="listbox">
                <ul class="dropdown-list">
                    <!-- Theme toggle as first item -->
                    <li class="dropdown-item">
                        <button
                            class="dropdown-button theme-button"
                            @click=${this.toggleTheme}
                            role="option"
                        >
                            <span class="flag">${this.isDark ? '☀️' : '🌙'}</span>
                            <span>${this.isDark ? msg('Switch to Light') : msg('Switch to Dark')}</span>
                        </button>
                    </li>
                    <!-- Separator -->
                    <li class="dropdown-separator"></li>
                    <!-- Language options -->
                    ${allLocales.map(locale => html`
                        <li class="dropdown-item">
                            <button
                                class="dropdown-button ${locale === this.currentLocale ? 'active' : ''}"
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