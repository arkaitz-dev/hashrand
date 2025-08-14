// @ts-check
import { LitElement, html } from 'lit';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import { Router } from '@vaadin/router';
import { sharedStyles } from '../shared-styles.js';

/**
 * MenuItem component - Individual card for menu navigation
 * 
 * @property {string} route - The route to navigate to when clicked
 * @property {string} icon - The emoji icon to display
 * @property {string} titleKey - The localization key for the title
 * @property {string} descKey - The localization key for the description
 */
export class MenuItem extends LitElement {
    static properties = {
        route: { type: String },
        icon: { type: String },
        titleKey: { type: String },
        descKey: { type: String }
    };

    static styles = sharedStyles;

    constructor() {
        super();
        this.route = '';
        this.icon = '';
        this.titleKey = '';
        this.descKey = '';
    }

    connectedCallback() {
        super.connectedCallback();
        updateWhenLocaleChanges(this);
    }

    /**
     * Handle click navigation
     */
    handleClick() {
        if (this.route) {
            Router.go(this.route);
        }
    }

    /**
     * Handle blur to remove focus styling after click
     * @param {Event} e - The blur event
     */
    handleBlur(e) {
        /** @type {HTMLElement} */ (e.currentTarget).blur();
    }

    render() {
        return html`
            <div 
                class="bg-white dark:bg-gray-600 border-2 border-gray-500 dark:border-none rounded-xl p-8 text-center cursor-pointer transition-all duration-300 relative overflow-hidden shadow-xl dark:shadow-black/40 hover:-translate-y-1 hover:shadow-2xl hover:shadow-black/25 dark:hover:shadow-black/60 hover:border-blue-600 dark:hover:border-blue-300 focus:outline-none focus:border-blue-600 focus:shadow-blue-100 dark:focus:border-blue-300"
                @click=${this.handleClick}
                @blur=${this.handleBlur}
                tabindex="0"
                role="button"
                aria-label="${msg(this.titleKey)} - ${msg(this.descKey)}"
            >
                <div class="absolute inset-0 bg-gradient-to-br from-blue-600 to-indigo-600 opacity-0 transition-opacity duration-300 hover:opacity-5"></div>
                
                <span class="text-5xl mb-4 block">${this.icon}</span>
                
                <h3 class="text-xl font-semibold text-gray-900 dark:text-gray-50 mb-2 relative z-10">
                    ${msg(this.titleKey)}
                </h3>
                
                <p class="text-sm text-gray-600 dark:text-gray-200 leading-snug relative z-10">
                    ${msg(this.descKey)}
                </p>
            </div>
        `;
    }
}

customElements.define('menu-item', MenuItem);