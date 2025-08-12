import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { msg, updateWhenLocaleChanges } from '@lit/localize';

export class HeaderTitle extends LitElement {
    @state()
    accessor version = '';

    static styles = css`
        :host {
            display: block;
        }

        h1 {
            font-size: 2.5rem;
            margin-bottom: 0.5rem;
            font-weight: 700;
        }

        p {
            opacity: 0.9;
            font-size: 1.1rem;
            margin-bottom: 1rem;
        }

        .version {
            font-size: 0.8rem;
            opacity: 0.7;
            font-weight: 400;
        }

        .language-selector-container {
            display: flex;
            justify-content: center;
            margin-top: 0.5rem;
        }

        @media (max-width: 768px) {
            h1 {
                font-size: 2rem;
            }
        }
    `;

    constructor() {
        super();
        this.fetchVersion();
    }

    connectedCallback() {
        super.connectedCallback();
        updateWhenLocaleChanges(this);
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

    render() {
        return html`
            <h1>🎲 HashRand ${this.version ? html`<span class="version">v${this.version}</span>` : ''}</h1>
            <p>${msg('Secure Random Hash Generator with Multiple Alphabets')}</p>
            <div class="language-selector-container">
                <slot name="language-selector"></slot>
            </div>
        `;
    }
}

customElements.define('header-title', HeaderTitle);