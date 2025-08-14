import { LitElement, html, css } from 'lit';

export class ThemeToggle extends LitElement {
    static properties = {
        isDark: { type: Boolean }
    };

    constructor() {
        super();
        console.log('ThemeToggle constructor called');
        // Check for saved theme preference or default to system preference
        const savedTheme = localStorage.getItem('hashrand-theme');
        if (savedTheme) {
            this.isDark = savedTheme === 'dark';
        } else {
            this.isDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
        }
        console.log('ThemeToggle isDark:', this.isDark);
        this.applyTheme();
    }

    static styles = css`
        :host {
            display: block !important;
            position: relative;
            background: red; /* Debug color */
            width: 40px;
            height: 40px;
        }

        .toggle-button {
            display: flex !important;
            align-items: center;
            justify-content: center;
            padding: 8px;
            background: rgba(255, 255, 255, 0.9);
            border: 2px solid #fff;
            border-radius: 8px;
            color: #333;
            cursor: pointer;
            transition: all 0.2s ease;
            width: 100%;
            height: 100%;
            font-size: 18px;
        }

        .toggle-button:hover {
            background: rgba(255, 255, 255, 1);
            transform: scale(1.05);
        }

        .icon {
            width: 20px;
            height: 20px;
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
        console.log('ThemeToggle render() called, isDark:', this.isDark);
        return html`
            <button
                class="toggle-button"
                @click=${this.toggleTheme}
                title="${this.isDark ? 'Switch to light mode' : 'Switch to dark mode'}"
            >
                ${this.isDark ? '☀️' : '🌙'}
            </button>
        `;
    }
}

customElements.define('theme-toggle', ThemeToggle);