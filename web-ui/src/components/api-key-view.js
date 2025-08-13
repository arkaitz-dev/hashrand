import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import sharedStyles from '../shared-styles.js';

export class ApiKeyView extends LitElement {
    @state()
    accessor lengthValue = 44;
    
    @state()
    accessor alphabetType = 'full';
    
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
        // Dynamic minimum length based on alphabet for same entropy (~262 bits)
        const minLength = this.alphabetType === 'no-look-alike' ? 47 : 44;
        
        // Adjust length if switching alphabets and current length is below minimum
        if (this.lengthValue < minLength) {
            this.lengthValue = minLength;
        }
        
        const alphabetDescription = this.alphabetType === 'no-look-alike' 
            ? msg('no-look-alike alphabet (easy to type)')
            : msg('full alphanumeric alphabet');
        
        return html`
            <button class="bg-transparent border-2 border-indigo-500 text-indigo-500 mb-6 w-auto px-5 py-2.5 inline-flex items-center gap-2 font-semibold cursor-pointer rounded-lg text-base transition-all duration-300 hover:bg-indigo-500 hover:text-white hover:shadow-lg hover:shadow-indigo-500/30 focus:outline-none" @click=${this.handleBackClick}>${msg('← Back to Menu')}</button>
            
            <div class="mb-8">
                <h2 class="text-slate-700 mb-4 text-xl flex items-center gap-2">${msg('🔑 Generate API Key')}</h2>
                
                <div class="mb-6">
                    <label for="apikey-alphabet" class="block mb-2 font-semibold text-slate-600">${msg('Alphabet Type')}</label>
                    <select id="apikey-alphabet" class="w-full px-4 py-3 border-2 border-gray-200 rounded-lg text-base font-inherit transition-all duration-300 bg-white focus:outline-none focus:border-indigo-500 focus:shadow-[0_0_0_3px_rgba(102,126,234,0.1)]" @change=${this.handleAlphabetChange}>
                        <option value="full" ?selected=${this.alphabetType === 'full'}>
                            ${msg('Full Alphanumeric (Maximum Compatibility)')}
                        </option>
                        <option value="no-look-alike" ?selected=${this.alphabetType === 'no-look-alike'}>
                            ${msg('No Look-alike (Easy to Type)')}
                        </option>
                    </select>
                </div>
                
                <div class="mb-6">
                    <label for="apikey-length" class="block mb-2 font-semibold text-slate-600">${msg('Length')} (${minLength}-64 ${msg('characters')})</label>
                    <div class="flex items-center gap-4">
                        <input type="range" id="apikey-length" min="${minLength}" max="64" .value=${this.lengthValue.toString()} @input=${this.handleLengthChange} class="flex-1 h-2 bg-gradient-to-r from-indigo-500 to-purple-600 rounded appearance-none outline-none slider">
                        <span class="bg-indigo-500 text-white px-3 py-2 rounded-md font-bold min-w-[40px] text-center">${this.lengthValue}</span>
                    </div>
                </div>
                
                <div class="bg-blue-50 border border-blue-200 rounded-md p-4 my-4 text-sm text-blue-700">
                    <strong>${msg('Format:')}:</strong> ak_ ${msg('prefix')} + ${this.lengthValue} ${msg('random characters using')} ${alphabetDescription}
                    <br><strong>${msg('Security:')}:</strong> 
                    ${this.alphabetType === 'no-look-alike' 
                        ? msg('No Look-alike excludes confusable characters. Minimum 47 characters for equivalent security.')
                        : msg('Full alphanumeric provides maximum compatibility. Minimum 44 characters for strong security.')}
                </div>
                
                <button id="apikey-btn" class="w-full py-4 bg-gradient-to-r from-indigo-500 to-purple-600 text-white border-none rounded-lg text-lg font-semibold cursor-pointer transition-all duration-300 mt-4 hover:-translate-y-0.5 hover:shadow-2xl focus:outline-none" @click=${this.handleGenerate}>${msg('Generate API Key')}</button>
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

    handleAlphabetChange(e) {
        const target = e.target;
        this.alphabetType = target.value;
        
        // Adjust length to minimum if needed when switching alphabets
        const minLength = this.alphabetType === 'no-look-alike' ? 47 : 44;
        if (this.lengthValue < minLength) {
            this.lengthValue = minLength;
        }
    }
    
    handleGenerate() {
        const length = this.shadowRoot.querySelector('#apikey-length').value;
        const alphabet = this.shadowRoot.querySelector('#apikey-alphabet').value;
        
        const parameters = {
            length: parseInt(length),
            alphabet: alphabet
        };
        
        // Emit event to parent component
        this.dispatchEvent(new CustomEvent('generate-hash', {
            bubbles: true,
            composed: true,
            detail: {
                hashType: 'apiKey',
                parameters: parameters
            }
        }));
    }
}

customElements.define('api-key-view', ApiKeyView);