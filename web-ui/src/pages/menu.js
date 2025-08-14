// @ts-check
import { LitElement, html, css } from 'lit';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import { Router } from '@vaadin/router';
import sharedStyles from '../shared-styles.js';

export class MenuPage extends LitElement {
    static styles = [
        sharedStyles,
        css`
            :host {
                display: block;
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
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 my-8">
                <div class="bg-white dark:bg-gray-700 border-2 border-gray-200 dark:border-gray-600 rounded-xl p-8 text-center cursor-pointer transition-all duration-300 relative overflow-hidden hover:-translate-y-1 hover:shadow-xl hover:border-blue-500 dark:hover:border-blue-400 group" 
                     @click=${() => Router.go('/custom')} 
                     @blur=${this.handleCardBlur} 
                     tabindex="0">
                    <div class="absolute inset-0 bg-gradient-to-br from-blue-500 to-indigo-600 opacity-0 group-hover:opacity-10 dark:group-hover:opacity-20 transition-opacity duration-300"></div>
                    <div class="text-5xl mb-4 relative z-10">🎲</div>
                    <h3 class="text-gray-800 dark:text-gray-100 mb-2 text-2xl font-semibold relative z-10">${msg('Generic Hash')}</h3>
                    <p class="text-gray-600 dark:text-gray-300 text-sm relative z-10">${msg('Generate customizable hashes with various alphabets')}</p>
                </div>
                
                <div class="bg-white dark:bg-gray-700 border-2 border-gray-200 dark:border-gray-600 rounded-xl p-8 text-center cursor-pointer transition-all duration-300 relative overflow-hidden hover:-translate-y-1 hover:shadow-xl hover:border-blue-500 dark:hover:border-blue-400 group" 
                     @click=${() => Router.go('/password')} 
                     @blur=${this.handleCardBlur} 
                     tabindex="0">
                    <div class="absolute inset-0 bg-gradient-to-br from-blue-500 to-indigo-600 opacity-0 group-hover:opacity-10 dark:group-hover:opacity-20 transition-opacity duration-300"></div>
                    <div class="text-5xl mb-4 relative z-10">🔐</div>
                    <h3 class="text-gray-800 dark:text-gray-100 mb-2 text-2xl font-semibold relative z-10">${msg('Password')}</h3>
                    <p class="text-gray-600 dark:text-gray-300 text-sm relative z-10">${msg('Create strong passwords with symbols')}</p>
                </div>
                
                <div class="bg-white dark:bg-gray-700 border-2 border-gray-200 dark:border-gray-600 rounded-xl p-8 text-center cursor-pointer transition-all duration-300 relative overflow-hidden hover:-translate-y-1 hover:shadow-xl hover:border-blue-500 dark:hover:border-blue-400 group" 
                     @click=${() => Router.go('/api-key')} 
                     @blur=${this.handleCardBlur} 
                     tabindex="0">
                    <div class="absolute inset-0 bg-gradient-to-br from-blue-500 to-indigo-600 opacity-0 group-hover:opacity-10 dark:group-hover:opacity-20 transition-opacity duration-300"></div>
                    <div class="text-5xl mb-4 relative z-10">🔑</div>
                    <h3 class="text-gray-800 dark:text-gray-100 mb-2 text-2xl font-semibold relative z-10">${msg('API Key')}</h3>
                    <p class="text-gray-600 dark:text-gray-300 text-sm relative z-10">${msg('Generate secure API keys (ak_ prefix)')}</p>
                </div>
            </div>
        `;
    }

    /**
     * Handles card blur to remove focus styling after click
     * Prevents persistent focus states that degrade UX
     * @param {Event} e - The blur event
     * @returns {void}
     */
    handleCardBlur(e) {
        // Ensure the card loses focus
        e.currentTarget.blur();
    }
}

customElements.define('menu-page', MenuPage);