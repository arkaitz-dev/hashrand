import { LitElement, html, css } from 'lit';
import { property, state } from 'lit/decorators.js';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import sharedStyles from '../shared-styles.js';

export class HashResult extends LitElement {
    @property({ type: String })
    accessor hashType = 'generic';      // 'generic', 'password', 'apiKey'
    
    @property({ type: String })
    accessor generatedHash = '';  // The generated result
    
    @property({ type: Object })
    accessor parameters = {};     // Generation parameters for regeneration
    
    @state()
    accessor isLoading = false;     // Loading state
    
    @state()
    accessor error = null;           // Error message if any

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
            <div class="max-w-3xl mx-auto p-8">
                <!-- Navigation buttons -->
                <div class="flex gap-4 mb-8">
                    <button class="bg-transparent border-2 border-indigo-500 text-indigo-500 px-5 py-2.5 inline-flex items-center gap-2 font-semibold cursor-pointer rounded-lg text-base transition-all duration-300 hover:bg-indigo-500 hover:text-white hover:-translate-y-0.5 hover:shadow-lg focus:outline-none" 
                            @click=${this.handleBackToConfig}>
                        ← ${msg('Back to Configuration')}
                    </button>
                    <button class="bg-transparent border-2 border-indigo-500 text-indigo-500 px-5 py-2.5 inline-flex items-center gap-2 font-semibold cursor-pointer rounded-lg text-base transition-all duration-300 hover:bg-indigo-500 hover:text-white hover:-translate-y-0.5 hover:shadow-lg focus:outline-none" 
                            @click=${this.handleBackToMenu}>
                        ⌂ ${msg('Back to Menu')}
                    </button>
                </div>

                <!-- Result section -->
                <div class="bg-white rounded-xl p-8 shadow-md mb-8">
                    <div class="flex items-center gap-3 mb-6">
                        <span class="text-3xl">${this.getIcon()}</span>
                        <h2 class="text-gray-800 text-2xl font-semibold m-0">${this.getTitle()}</h2>
                    </div>

                    <div class="bg-gray-50 border-2 ${this.getResultClasses()} rounded-lg p-6 font-mono text-lg break-all flex items-center justify-between gap-4 min-h-[60px]">
                        ${this.isLoading ? html`
                            <span class="inline-block w-5 h-5 border-[3px] border-gray-300 border-t-indigo-500 rounded-full animate-spin"></span>
                            <span>${msg('Generating...')}</span>
                        ` : this.error ? html`
                            <span class="flex-1 break-words">${msg('Error')}: ${this.error}</span>
                        ` : html`
                            <span class="flex-1 break-words">${this.generatedHash}</span>
                            <button class="${this.getCopyButtonClasses()}" @click=${this.handleCopy}>
                                ${msg('Copy')}
                            </button>
                        `}
                    </div>

                    ${this.parameters && Object.keys(this.parameters).length > 0 ? html`
                        <div class="bg-gray-50 rounded-lg p-4 mt-4">
                            <div class="text-gray-500 text-sm font-semibold mb-2">${msg('Configuration Used')}:</div>
                            ${this.renderParameters()}
                        </div>
                    ` : ''}
                </div>

                <!-- Regenerate button -->
                <button class="w-full p-4 bg-gradient-to-r from-indigo-500 to-purple-600 text-white border-none rounded-lg text-lg font-semibold cursor-pointer transition-all duration-300 flex items-center justify-center gap-2 hover:-translate-y-0.5 hover:shadow-xl disabled:opacity-60 disabled:cursor-not-allowed disabled:transform-none" 
                        @click=${this.handleRegenerate} 
                        ?disabled=${this.isLoading}>
                    ${this.isLoading ? html`
                        <span class="inline-block w-5 h-5 border-[3px] border-gray-300 border-t-white rounded-full animate-spin"></span>
                        <span>${msg('Regenerating...')}</span>
                    ` : html`
                        <span>🔄</span>
                        <span>${this.getRegenerateButtonText()}</span>
                    `}
                </button>
            </div>
        `;
    }

    getIcon() {
        switch(this.hashType) {
            case 'password': return '🔐';
            case 'apiKey': return '🔑';
            default: return '🎲';
        }
    }

    getTitle() {
        switch(this.hashType) {
            case 'password': return msg('Generated Password');
            case 'apiKey': return msg('Generated API Key');
            default: return msg('Generated Hash');
        }
    }

    getResultClasses() {
        if (this.error) return 'border-red-300 bg-red-50 text-red-800';
        if (this.generatedHash && !this.isLoading) return 'border-green-300 bg-green-50 text-green-800';
        return 'border-gray-200';
    }

    getCopyButtonClasses() {
        return 'bg-green-600 text-white border-none px-4 py-2 rounded-md cursor-pointer font-semibold transition-all duration-300 flex-shrink-0 hover:bg-green-700 hover:-translate-y-0.5 copied:bg-gray-500';
    }

    getRegenerateButtonText() {
        switch(this.hashType) {
            case 'apiKey': 
                // Check if there are parameters (length) configured
                return Object.keys(this.parameters).length > 0 ? msg('Regenerate with Same Configuration') : msg('Regenerate');
            default: return msg('Regenerate with Same Configuration');
        }
    }

    renderParameters() {
        const params = [];
        
        if (this.parameters.length) {
            params.push(html`
                <div class="flex justify-between py-1 text-sm">
                    <span class="text-gray-500">${msg('Length')}:</span>
                    <span class="text-gray-800 font-medium">${this.parameters.length}</span>
                </div>
            `);
        }
        
        if (this.parameters.alphabet) {
            let alphabetName = this.parameters.alphabet;
            switch (this.parameters.alphabet) {
                case 'base58':
                    alphabetName = msg('Base58 (Bitcoin)');
                    break;
                case 'no-look-alike':
                    alphabetName = msg('No Look-alike');
                    break;
                case 'full':
                    alphabetName = msg('Full Alphanumeric');
                    break;
                case 'full-with-symbols':
                    alphabetName = msg('Full with Symbols');
                    break;
            }
            params.push(html`
                <div class="flex justify-between py-1 text-sm">
                    <span class="text-gray-500">${msg('Alphabet')}:</span>
                    <span class="text-gray-800 font-medium">${alphabetName}</span>
                </div>
            `);
        }
        
        if (this.parameters.prefix) {
            params.push(html`
                <div class="flex justify-between py-1 text-sm">
                    <span class="text-gray-500">${msg('Prefix')}:</span>
                    <span class="text-gray-800 font-medium">${this.parameters.prefix}</span>
                </div>
            `);
        }
        
        if (this.parameters.suffix) {
            params.push(html`
                <div class="flex justify-between py-1 text-sm">
                    <span class="text-gray-500">${msg('Suffix')}:</span>
                    <span class="text-gray-800 font-medium">${this.parameters.suffix}</span>
                </div>
            `);
        }
        
        return params;
    }

    async handleCopy() {
        try {
            await navigator.clipboard.writeText(this.generatedHash);
            // Temporarily change button text
            const button = this.shadowRoot.querySelector('button[class*="bg-green"]');
            if (button) {
                button.textContent = msg('Copied!');
                button.classList.remove('bg-green-600', 'hover:bg-green-700');
                button.classList.add('bg-gray-500');
                setTimeout(() => {
                    button.textContent = msg('Copy');
                    button.classList.remove('bg-gray-500');
                    button.classList.add('bg-green-600', 'hover:bg-green-700');
                }, 2000);
            }
        } catch (error) {
            console.error('Failed to copy:', error);
        }
    }

    handleBackToConfig(e) {
        // Remove focus from button after click
        if (e && e.currentTarget) {
            e.currentTarget.blur();
        }
        
        this.dispatchEvent(new CustomEvent('back-to-config', { 
            bubbles: true,
            composed: true,
            detail: { hashType: this.hashType }
        }));
    }

    handleBackToMenu(e) {
        // Remove focus from button after click
        if (e && e.currentTarget) {
            e.currentTarget.blur();
        }
        
        this.dispatchEvent(new CustomEvent('back-to-menu', { 
            bubbles: true,
            composed: true 
        }));
    }

    async handleRegenerate() {
        this.dispatchEvent(new CustomEvent('regenerate', { 
            bubbles: true,
            composed: true,
            detail: {
                hashType: this.hashType,
                parameters: this.parameters
            }
        }));
    }
}

customElements.define('hash-result', HashResult);