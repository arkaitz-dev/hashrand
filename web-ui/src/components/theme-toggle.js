import { LitElement, html } from 'lit';
import { sharedStyles } from '../shared-styles.js';

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

    static styles = sharedStyles;

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
                class="flex items-center justify-center p-2 bg-white border-2 border-white rounded-lg text-gray-700 cursor-pointer transition-all duration-200 w-full h-full text-lg hover:shadow-md focus:outline-none"
                @click=${this.toggleTheme}
                title="${this.isDark ? 'Switch to light mode' : 'Switch to dark mode'}"
            >
                <span class="w-5 h-5 text-center leading-none">
                    ${this.isDark ? '☀️' : '🌙'}
                </span>
            </button>
        `;
    }
}

customElements.define('theme-toggle', ThemeToggle);