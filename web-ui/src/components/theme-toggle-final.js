import { LitElement, html, css } from 'lit';

export class ThemeToggle extends LitElement {
    static properties = {
        isDark: { type: Boolean }
    };

    constructor() {
        super();
        // Detectar preferencia del sistema igual que el preloader
        const savedTheme = localStorage.getItem('hashrand-theme');
        if (savedTheme) {
            this.isDark = savedTheme === 'dark';
        } else {
            // Forzar detección más robusta del tema del sistema
            this.isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        }
        
        
        // Aplicar tema inmediatamente
        this.applyTheme();
    }

    static styles = css`
        :host {
            display: block;
        }

        .toggle-button {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 36px;
            height: 36px;
            padding: 0;
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(8px);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 8px;
            color: white;
            cursor: pointer;
            transition: all 0.2s ease;
            font-size: 18px;
        }

        .toggle-button:hover {
            background: rgba(255, 255, 255, 0.2);
            border-color: rgba(255, 255, 255, 0.3);
            transform: scale(1.05);
        }

        .toggle-button:focus {
            outline: none;
            box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.3);
        }

        .toggle-button:active {
            transform: scale(0.95);
        }
    `;

    applyTheme() {
        const html = document.documentElement;
        const body = document.body;
        
        if (this.isDark) {
            html.classList.add('dark');
            body.style.backgroundColor = '#1f2937'; // gray-800 - SAME as preloader
            body.style.color = '#f9fafb'; // gray-50
        } else {
            html.classList.remove('dark');
            body.style.backgroundColor = '#f9fafb'; // gray-50
            body.style.color = '#111827'; // gray-900
        }
        
        localStorage.setItem('hashrand-theme', this.isDark ? 'dark' : 'light');
    }

    toggleTheme() {
        this.isDark = !this.isDark;
        this.applyTheme();
    }

    render() {
        return html`
            <button
                class="toggle-button"
                @click=${this.toggleTheme}
                title="${this.isDark ? 'Switch to light mode' : 'Switch to dark mode'}"
                aria-label="${this.isDark ? 'Switch to light mode' : 'Switch to dark mode'}"
            >
                ${this.isDark ? '☀️' : '🌙'}
            </button>
        `;
    }
}

customElements.define('theme-toggle', ThemeToggle);