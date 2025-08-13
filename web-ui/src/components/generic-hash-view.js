import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import sharedStyles from '../shared-styles.js';

export class GenericHashView extends LitElement {
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
            <button class="bg-transparent border-2 border-indigo-500 text-indigo-500 mb-6 w-auto px-5 py-2.5 inline-flex items-center gap-2 font-semibold cursor-pointer rounded-lg text-base transition-all duration-300 hover:bg-indigo-500 hover:text-white hover:shadow-lg hover:shadow-indigo-500/30 focus:outline-none" @click=${this.handleBackClick}>${msg('← Back to Menu')}</button>
            
            <div class="mb-8">
                <h2 class="text-slate-700 mb-4 text-xl flex items-center gap-2">${msg('🎲 Generate Custom Hash')}</h2>
                
                <div class="mb-6">
                    <label for="generate-length" class="block mb-2 font-semibold text-slate-600">${msg('Length')}</label>
                    <div class="flex items-center gap-4">
                        <input type="range" id="generate-length" min="2" max="128" .value=${this.lengthValue.toString()} @input=${this.handleLengthChange} class="flex-1 h-2 bg-gradient-to-r from-indigo-500 to-purple-600 rounded appearance-none outline-none slider">
                        <span class="bg-indigo-500 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center">${this.lengthValue}</span>
                    </div>
                </div>
                
                <div class="mb-6">
                    <label for="generate-alphabet" class="block mb-2 font-semibold text-slate-600">${msg('Alphabet Type')}</label>
                    <select id="generate-alphabet" class="w-full px-4 py-3 border-2 border-gray-200 rounded-lg text-base font-inherit transition-all duration-300 bg-white focus:outline-none focus:border-indigo-500 focus:shadow-[0_0_0_3px_rgba(102,126,234,0.1)]">
                        <option value="base58">${msg('Base58 (Bitcoin)')}</option>
                        <option value="no-look-alike">${msg('No Look-alike')}</option>
                        <option value="full">${msg('Full Alphanumeric')}</option>
                        <option value="full-with-symbols">${msg('Full with Symbols')}</option>
                    </select>
                </div>
                
                <div class="mb-6">
                    <label for="generate-prefix" class="block mb-2 font-semibold text-slate-600">${msg('Prefix (optional)')}</label>
                    <input type="text" id="generate-prefix" class="w-full px-4 py-3 border-2 border-gray-200 rounded-lg text-base font-inherit transition-all duration-300 bg-white focus:outline-none focus:border-indigo-500 focus:shadow-[0_0_0_3px_rgba(102,126,234,0.1)]" placeholder="${msg('e.g., user_')}">
                </div>
                
                <div class="mb-6">
                    <label for="generate-suffix" class="block mb-2 font-semibold text-slate-600">${msg('Suffix (optional)')}</label>
                    <input type="text" id="generate-suffix" class="w-full px-4 py-3 border-2 border-gray-200 rounded-lg text-base font-inherit transition-all duration-300 bg-white focus:outline-none focus:border-indigo-500 focus:shadow-[0_0_0_3px_rgba(102,126,234,0.1)]" placeholder="${msg('e.g., _temp')}">
                </div>
                
                <button id="generate-btn" class="w-full py-4 bg-gradient-to-r from-indigo-500 to-purple-600 text-white border-none rounded-lg text-lg font-semibold cursor-pointer transition-all duration-300 mt-4 hover:-translate-y-0.5 hover:shadow-2xl focus:outline-none" @click=${this.handleGenerate}>${msg('Generate Hash')}</button>
            </div>
        `;
    }

    handleBackClick() {
        this.dispatchEvent(new CustomEvent('back-to-menu', { 
            bubbles: true,
            composed: true 
        }));
    }

    handleLengthChange(e) {
        const target = e.target;
        this.lengthValue = parseInt(target.value);
    }

    handleGenerate() {
        const length = this.shadowRoot.querySelector('#generate-length').value;
        const alphabet = this.shadowRoot.querySelector('#generate-alphabet').value;
        const prefix = this.shadowRoot.querySelector('#generate-prefix').value;
        const suffix = this.shadowRoot.querySelector('#generate-suffix').value;
        
        const parameters = {
            length: parseInt(length),
            alphabet: alphabet,
            prefix: prefix,
            suffix: suffix
        };
        
        // Emit event to parent component
        this.dispatchEvent(new CustomEvent('generate-hash', {
            bubbles: true,
            composed: true,
            detail: {
                hashType: 'generate',
                parameters: parameters
            }
        }));
    }
}

customElements.define('generic-hash-view', GenericHashView);