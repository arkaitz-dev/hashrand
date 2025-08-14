// @ts-check
import { LitElement, html, css } from 'lit';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import { Router } from '@vaadin/router';

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

    static styles = css`
        :host {
            display: block;
            width: 100%;
            height: 100%;
        }

        .menu-card {
            background: white;
            border: 2px solid #6b7280;
            border-radius: 0.75rem;
            padding: 2rem;
            text-align: center;
            cursor: pointer;
            transition: all 0.3s ease;
            position: relative;
            overflow: hidden;
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
        }

        /* Dark mode styles */
        :host-context(.dark) .menu-card,
        :host-context(html.dark) .menu-card {
            background: #4b5563 !important; /* gray-600 - más claro que header (gray-700) pero más oscuro que contenedor (gray-500) */
            border: none !important; /* sin bordes */
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.4), 0 10px 10px -5px rgba(0, 0, 0, 0.4);
        }

        .menu-card:hover {
            transform: translateY(-4px);
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
            border-color: #3b82f6;
        }

        :host-context(.dark) .menu-card:hover {
            border-color: #93c5fd;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.6);
        }

        .menu-card:focus {
            outline: none;
            border-color: #3b82f6;
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
        }

        :host-context(.dark) .menu-card:focus {
            border-color: #93c5fd;
        }

        .hover-overlay {
            position: absolute;
            inset: 0;
            background: linear-gradient(to bottom right, #3b82f6, #6366f1);
            opacity: 0;
            transition: opacity 0.3s ease;
        }

        .menu-card:hover .hover-overlay {
            opacity: 0.1;
        }

        :host-context(.dark) .menu-card:hover .hover-overlay {
            opacity: 0.2;
        }

        .menu-icon {
            font-size: 3rem;
            line-height: 1;
            margin-bottom: 1rem;
            position: relative;
            z-index: 10;
        }

        .menu-title {
            color: #111827;
            margin-bottom: 0.5rem;
            font-size: 1.5rem;
            font-weight: 600;
            position: relative;
            z-index: 10;
        }

        :host-context(.dark) .menu-title,
        :host-context(html.dark) .menu-title {
            color: #f9fafb !important; /* gray-50 - texto claro para el fondo gray-600 */
        }

        .menu-description {
            color: #374151;
            font-size: 0.875rem;
            line-height: 1.25rem;
            position: relative;
            z-index: 10;
        }

        :host-context(.dark) .menu-description,
        :host-context(html.dark) .menu-description {
            color: #e5e7eb !important; /* gray-200 - texto más suave para descripción */
        }
    `;

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
                class="menu-card"
                @click=${this.handleClick}
                @blur=${this.handleBlur}
                tabindex="0"
                role="button"
                aria-label="${msg(this.titleKey)}"
            >
                <div class="hover-overlay"></div>
                <div class="menu-icon">${this.icon}</div>
                <h3 class="menu-title">${msg(this.titleKey)}</h3>
                <p class="menu-description">${msg(this.descKey)}</p>
            </div>
        `;
    }
}

customElements.define('menu-item', MenuItem);