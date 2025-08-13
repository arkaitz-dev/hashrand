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
                <div class="bg-white border-2 border-gray-200 rounded-xl p-8 text-center cursor-pointer transition-all duration-300 relative overflow-hidden hover:-translate-y-1 hover:shadow-xl hover:border-indigo-500 group" 
                     @click=${() => Router.go('/custom')} 
                     @blur=${this.handleCardBlur} 
                     tabindex="0">
                    <div class="absolute inset-0 bg-gradient-to-br from-indigo-500 to-purple-600 opacity-0 group-hover:opacity-5 transition-opacity duration-300"></div>
                    <div class="text-5xl mb-4 relative z-10">🎲</div>
                    <h3 class="text-gray-800 mb-2 text-2xl font-semibold relative z-10">${msg('Generic Hash')}</h3>
                    <p class="text-gray-500 text-sm relative z-10">${msg('Generate customizable hashes with various alphabets')}</p>
                </div>
                
                <div class="bg-white border-2 border-gray-200 rounded-xl p-8 text-center cursor-pointer transition-all duration-300 relative overflow-hidden hover:-translate-y-1 hover:shadow-xl hover:border-indigo-500 group" 
                     @click=${() => Router.go('/password')} 
                     @blur=${this.handleCardBlur} 
                     tabindex="0">
                    <div class="absolute inset-0 bg-gradient-to-br from-indigo-500 to-purple-600 opacity-0 group-hover:opacity-5 transition-opacity duration-300"></div>
                    <div class="text-5xl mb-4 relative z-10">🔐</div>
                    <h3 class="text-gray-800 mb-2 text-2xl font-semibold relative z-10">${msg('Password')}</h3>
                    <p class="text-gray-500 text-sm relative z-10">${msg('Create strong passwords with symbols')}</p>
                </div>
                
                <div class="bg-white border-2 border-gray-200 rounded-xl p-8 text-center cursor-pointer transition-all duration-300 relative overflow-hidden hover:-translate-y-1 hover:shadow-xl hover:border-indigo-500 group" 
                     @click=${() => Router.go('/api-key')} 
                     @blur=${this.handleCardBlur} 
                     tabindex="0">
                    <div class="absolute inset-0 bg-gradient-to-br from-indigo-500 to-purple-600 opacity-0 group-hover:opacity-5 transition-opacity duration-300"></div>
                    <div class="text-5xl mb-4 relative z-10">🔑</div>
                    <h3 class="text-gray-800 mb-2 text-2xl font-semibold relative z-10">${msg('API Key')}</h3>
                    <p class="text-gray-500 text-sm relative z-10">${msg('Generate secure API keys (ak_ prefix)')}</p>
                </div>
            </div>
        `;
    }

    handleCardBlur(e) {
        // Ensure the card loses focus
        e.currentTarget.blur();
    }
}

customElements.define('menu-page', MenuPage);