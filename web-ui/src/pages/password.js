import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import { Router } from '@vaadin/router';
import { buildApiUrl } from '../utils/api.js';
import sharedStyles from '../shared-styles.js';

export class PasswordView extends LitElement {
    @state()
    accessor lengthValue = 21;
    
    @state()
    accessor alphabetType = 'full-with-symbols';
    
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
        // Dynamic minimum length based on alphabet for same entropy (~130 bits)
        const minLength = this.alphabetType === 'no-look-alike' ? 24 : 21;
        
        // Adjust length if switching alphabets and current length is below minimum
        if (this.lengthValue < minLength) {
            this.lengthValue = minLength;
        }
        
        return html`
            <button class="bg-transparent border-2 border-blue-600 text-blue-600 dark:border-blue-400 dark:text-blue-400 mb-6 w-auto px-5 py-2.5 inline-flex items-center gap-2 font-semibold cursor-pointer rounded-lg text-base transition-all duration-200 hover:bg-blue-600 dark:hover:bg-blue-500 hover:text-white hover:shadow-md focus:outline-none" @click=${this.handleBackClick}>${msg('← Back to Menu')}</button>
            
            <div class="mb-8">
                <h2 class="text-gray-800 dark:text-gray-200 mb-4 text-xl flex items-center gap-2">${msg('🔐 Generate Password')}</h2>
                
                <div class="mb-6">
                    <label for="password-alphabet" class="block mb-2 font-semibold text-gray-700 dark:text-gray-300">${msg('Alphabet Type')}</label>
                    <select id="password-alphabet" class="w-full px-4 py-3 border-2 border-gray-200 dark:border-gray-600 rounded-lg text-base font-inherit transition-all duration-200 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:ring-opacity-20" @change=${this.handleAlphabetChange}>
                        <option value="full-with-symbols" ?selected=${this.alphabetType === 'full-with-symbols'}>
                            ${msg('Full with Symbols (Maximum Security)')}
                        </option>
                        <option value="no-look-alike" ?selected=${this.alphabetType === 'no-look-alike'}>
                            ${msg('No Look-alike (Easy to Type)')}
                        </option>
                    </select>
                </div>
                
                <div class="mb-6">
                    <label for="password-length" class="block mb-2 font-semibold text-slate-600">${msg('Length')} (${minLength}-44 ${msg('characters')})</label>
                    <div class="flex items-center gap-4">
                        <input type="range" id="password-length" min="${minLength}" max="44" .value=${this.lengthValue.toString()} @input=${this.handleLengthChange} class="flex-1 h-2 bg-gradient-to-r from-indigo-500 to-purple-600 rounded appearance-none outline-none slider">
                        <span class="bg-indigo-500 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center">${this.lengthValue}</span>
                    </div>
                </div>
                
                <div class="bg-blue-50 border border-blue-200 rounded-md p-4 my-4 text-sm text-blue-700">
                    <strong>${msg('Security Note:')}:</strong> 
                    ${this.alphabetType === 'no-look-alike' 
                        ? msg('No Look-alike alphabet excludes confusable characters. Minimum 24 characters for equivalent security.')
                        : msg('Full alphabet with symbols provides maximum entropy. Minimum 21 characters for strong security.')}
                </div>
                
                <button id="password-btn" class="w-full py-4 bg-gradient-to-r from-indigo-500 to-purple-600 text-white border-none rounded-lg text-lg font-semibold cursor-pointer transition-all duration-300 mt-4 hover:-translate-y-0.5 hover:shadow-2xl focus:outline-none" @click=${this.handleGenerate}>${msg('Generate Password')}</button>
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

    handleAlphabetChange(e) {
        const target = e.target;
        this.alphabetType = target.value;
        
        // Adjust length to minimum if needed when switching alphabets
        const minLength = this.alphabetType === 'no-look-alike' ? 24 : 21;
        if (this.lengthValue < minLength) {
            this.lengthValue = minLength;
        }
    }

    async handleGenerate() {
        const length = this.shadowRoot.querySelector('#password-length').value;
        const alphabet = this.shadowRoot.querySelector('#password-alphabet').value;
        
        const parameters = {
            length: parseInt(length),
            alphabet: alphabet
        };
        
        // Store parameters in sessionStorage for the result page
        sessionStorage.setItem('hashrand-last-params', JSON.stringify({
            type: 'password',
            parameters: parameters
        }));
        
        try {
            // Make API call
            const params = new URLSearchParams({ 
                length: parameters.length || 21 
            });
            // Add alphabet parameter if specified
            if (parameters.alphabet) {
                params.append('alphabet', parameters.alphabet);
            }
            const url = buildApiUrl(`/api/password?${params}`);
            const response = await fetch(url);
            const result = await response.text();
            
            if (response.ok) {
                // Store result and navigate to result page
                sessionStorage.setItem('hashrand-last-result', result);
                Router.go('/password/result');
            } else {
                throw new Error(response.statusText);
            }
        } catch (error) {
            // Store error and navigate to result page
            sessionStorage.setItem('hashrand-last-error', error.message);
            Router.go('/password/result');
        }
    }
}

customElements.define('password-page', PasswordView);