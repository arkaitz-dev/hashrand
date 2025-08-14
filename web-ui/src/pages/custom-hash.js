import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import { Router } from '@vaadin/router';
import { buildApiUrl } from '../utils/api.js';
import sharedStyles from '../shared-styles.js';

export class CustomHashView extends LitElement {
    @state()
    accessor lengthValue = 21;
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
            <button class="bg-transparent border-2 border-blue-600 text-blue-600 dark:border-blue-400 dark:text-blue-400 mb-6 w-auto px-5 py-2.5 inline-flex items-center gap-2 font-semibold cursor-pointer rounded-lg text-base transition-all duration-200 hover:bg-blue-600 dark:hover:bg-blue-500 hover:text-white hover:shadow-md focus:outline-none" @click=${this.handleBackClick}>${msg('← Back to Menu')}</button>
            
            <div class="mb-8">
                <h2 class="text-gray-800 dark:text-gray-200 mb-4 text-xl flex items-center gap-2">${msg('🎲 Generate Custom Hash')}</h2>
                
                <div class="mb-6">
                    <label for="generate-length" class="block mb-2 font-semibold text-gray-700 dark:text-gray-300">${msg('Length')}</label>
                    <div class="flex items-center gap-4">
                        <input type="range" id="generate-length" min="2" max="128" .value=${this.lengthValue.toString()} @input=${this.handleLengthChange} class="flex-1 h-2 bg-blue-600 rounded appearance-none outline-none slider">
                        <span class="bg-blue-600 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center">${this.lengthValue}</span>
                    </div>
                </div>
                
                <div class="mb-6">
                    <label for="generate-alphabet" class="block mb-2 font-semibold text-gray-700 dark:text-gray-300">${msg('Alphabet Type')}</label>
                    <select id="generate-alphabet" class="w-full px-4 py-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg text-base font-inherit transition-all duration-200 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:ring-opacity-20">
                        <option value="base58">${msg('Base58 (Bitcoin)')}</option>
                        <option value="no-look-alike">${msg('No Look-alike')}</option>
                        <option value="full">${msg('Full Alphanumeric')}</option>
                        <option value="full-with-symbols">${msg('Full with Symbols')}</option>
                    </select>
                </div>
                
                <div class="mb-6">
                    <label for="generate-prefix" class="block mb-2 font-semibold text-gray-700 dark:text-gray-300">${msg('Prefix (optional)')}</label>
                    <input type="text" id="generate-prefix" class="w-full px-4 py-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg text-base font-inherit transition-all duration-200 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:ring-opacity-20" placeholder="${msg('e.g., user_')}">
                </div>
                
                <div class="mb-6">
                    <label for="generate-suffix" class="block mb-2 font-semibold text-gray-700 dark:text-gray-300">${msg('Suffix (optional)')}</label>
                    <input type="text" id="generate-suffix" class="w-full px-4 py-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg text-base font-inherit transition-all duration-200 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:ring-opacity-20" placeholder="${msg('e.g., _temp')}">
                </div>
                
                <button id="generate-btn" class="w-full py-4 bg-blue-600 hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600 text-white border-none rounded-lg text-lg font-semibold cursor-pointer transition-all duration-200 mt-4 hover:shadow-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800" @click=${this.handleGenerate}>${msg('Generate Hash')}</button>
            </div>
        `;
    }

    handleBackClick() {
        Router.go('/');
    }

    handleLengthChange(e) {
        const target = e.target;
        this.lengthValue = parseInt(target.value);
    }

    async handleGenerate() {
        const length = /** @type {HTMLInputElement} */ (this.shadowRoot.querySelector('#generate-length')).value;
        const alphabet = /** @type {HTMLInputElement} */ (this.shadowRoot.querySelector('#generate-alphabet')).value;
        const prefix = /** @type {HTMLInputElement} */ (this.shadowRoot.querySelector('#generate-prefix')).value;
        const suffix = /** @type {HTMLInputElement} */ (this.shadowRoot.querySelector('#generate-suffix')).value;
        
        const parameters = {
            length: parseInt(length),
            alphabet: alphabet,
            prefix: prefix,
            suffix: suffix
        };
        
        // Store parameters in sessionStorage for the result page
        sessionStorage.setItem('hashrand-last-params', JSON.stringify({
            type: 'custom',
            parameters: parameters
        }));
        
        try {
            // Make API call
            const params = new URLSearchParams({
                length: String(parameters.length || 21),
                alphabet: parameters.alphabet || 'base58',
                raw: 'true'
            });
            
            if (parameters.prefix) params.append('prefix', parameters.prefix);
            if (parameters.suffix) params.append('suffix', parameters.suffix);
            
            const url = buildApiUrl(`/api/generate?${params}`);
            const response = await fetch(url);
            const result = await response.text();
            
            if (response.ok) {
                // Store result and navigate to result page
                sessionStorage.setItem('hashrand-last-result', result);
                Router.go('/custom/result');
            } else {
                throw new Error(response.statusText);
            }
        } catch (error) {
            // Store error and navigate to result page
            sessionStorage.setItem('hashrand-last-error', error.message);
            Router.go('/custom/result');
        }
    }
}

customElements.define('custom-hash-page', CustomHashView);