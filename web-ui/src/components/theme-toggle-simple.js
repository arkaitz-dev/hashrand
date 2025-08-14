import { LitElement, html, css } from 'lit';

export class ThemeToggleSimple extends LitElement {
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
            this.isDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
        }
        
        // Aplicar tema inmediatamente
        this.applyTheme();
    }

    static styles = css`
        :host {
            display: block;
            width: 50px;
            height: 50px;
            background: red;
            border: 3px solid yellow;
            margin: 5px;
        }

        button {
            width: 100%;
            height: 100%;
            font-size: 24px;
            background: blue;
            color: white;
            border: none;
            cursor: pointer;
        }
    `;

    applyTheme() {
        if (this.isDark) {
            document.documentElement.classList.add('dark');
        } else {
            document.documentElement.classList.remove('dark');
        }
        localStorage.setItem('hashrand-theme', this.isDark ? 'dark' : 'light');
    }

    toggleTheme() {
        this.isDark = !this.isDark;
        this.applyTheme();
    }

    render() {
        return html`
            <button @click=${this.toggleTheme}>
                ${this.isDark ? '☀' : '🌙'}
            </button>
        `;
    }
}

customElements.define('theme-toggle-simple', ThemeToggleSimple);