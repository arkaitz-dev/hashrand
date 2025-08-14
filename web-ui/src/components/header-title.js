// @ts-check
import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import sharedStyles from '../shared-styles.js';

/**
 * @typedef {Object} VersionResponse
 * @property {string} version - The application version
 */

/**
 * Header component displaying application title and version
 *
 * Fetches version information from the backend API and displays
 * the HashRand title with localized subtitle. Automatically updates
 * when locale changes.
 *
 * @extends LitElement
 */
export class HeaderTitle extends LitElement {
    @state()
    accessor version = '';

    static styles = [
        sharedStyles,
        css`
            :host {
                display: block;
            }
        `
    ];

    constructor() {
        super();
        this.fetchVersion();
    }

    connectedCallback() {
        super.connectedCallback();
        updateWhenLocaleChanges(this);
    }

    /**
     * Fetches version information from the backend API
     * Sets version to empty string if request fails
     * @returns {Promise<void>}
     */
    async fetchVersion() {
        try {
            const response = await fetch('/api/version');
            /** @type {VersionResponse} */
            const data = await response.json();
            this.version = data.version;
        } catch (error) {
            console.error('Error fetching version:', error);
            this.version = '';
        }
    }

    render() {
        return html`
            <h1 class="text-4xl md:text-5xl font-bold mb-2">🎲 HashRand ${this.version ? html`<span class="text-sm opacity-70 font-normal">v${this.version}</span>` : ''}</h1>
            <p class="opacity-90 text-lg">${msg('Secure Random Hash Generator with Multiple Alphabets')}</p>
        `;
    }
}

customElements.define('header-title', HeaderTitle);