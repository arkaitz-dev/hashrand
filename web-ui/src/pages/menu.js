// @ts-check
import { LitElement, html, css } from 'lit';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import sharedStyles from '../shared-styles.js';
import '../components/menu-item.js';

export class MenuPage extends LitElement {
    static styles = [
        sharedStyles,
        css`
            :host {
                display: block;
            }
            
            .hero-section {
                text-align: center !important;
                width: 100%;
                display: flex;
                flex-direction: column;
                align-items: center;
            }
            
            .menu-grid {
                display: grid;
                grid-template-columns: 1fr;
                gap: 1.5rem;
                width: 100%;
            }
            
            @media (min-width: 768px) {
                .menu-grid {
                    grid-template-columns: repeat(3, 1fr);
                    gap: 2rem;
                }
            }
        `
    ];
    
    connectedCallback() {
        super.connectedCallback();
        // Enable automatic re-rendering when locale changes
        updateWhenLocaleChanges(this);
    }

    render() {
        return html`
            <!-- Hero Section -->
            <div class="hero-section mb-12">
                <h2 class="text-2xl md:text-3xl font-light text-gray-700 dark:text-gray-300 mb-4">
                    ${msg('Secure Random Hash Generator with Multiple Alphabets')}
                </h2>
                <p class="text-gray-600 dark:text-gray-400 max-w-2xl mx-auto leading-relaxed">
                    ${msg('Choose your preferred generation method below to create cryptographically secure random strings')}
                </p>
            </div>
            
            <div class="menu-grid my-8">
                <menu-item 
                    route="/custom"
                    icon="🎲"
                    titleKey="Generic Hash"
                    descKey="Generate customizable hashes with various alphabets">
                </menu-item>
                
                <menu-item 
                    route="/password"
                    icon="🔐"
                    titleKey="Password"
                    descKey="Create strong passwords with symbols">
                </menu-item>
                
                <menu-item 
                    route="/api-key"
                    icon="🔑"
                    titleKey="API Key"
                    descKey="Generate secure API keys (ak_ prefix)">
                </menu-item>
            </div>
        `;
    }

}

customElements.define('menu-page', MenuPage);