import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import { buildApiUrl } from '../utils/api.js';
import sharedStyles from '../shared-styles.js';

export class MenuPage extends LitElement {
    @state()
    accessor currentView = 'menu';
    
    @state()
    accessor lastHashType = '';
    
    @state()
    accessor lastParameters = {};
    
    @state()
    accessor lastResult = '';

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
        
        // Listen for navigation events
        this.addEventListener('back-to-menu', (e) => {
            e.stopPropagation();
            this.switchView('menu');
        });
        
        this.addEventListener('back-to-config', (e) => {
            e.stopPropagation();
            const hashType = e.detail?.hashType || this.lastHashType;
            if (hashType) {
                this.switchView(hashType);
            }
        });
        
        // Listen for generation events from config views
        this.addEventListener('generate-hash', async (e) => {
            e.stopPropagation();
            const { hashType, parameters } = e.detail;
            await this.generateHash(hashType, parameters);
        });
        
        // Listen for regenerate event from result view
        this.addEventListener('regenerate', async (e) => {
            e.stopPropagation();
            const { hashType, parameters } = e.detail;
            await this.generateHash(hashType, parameters);
        });
    }

    render() {
        return html`
            <!-- Menu View -->
            <div id="menu-page" class="${this.currentView === 'menu' ? 'block' : 'hidden'}">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 my-8">
                    <div class="bg-white border-2 border-gray-200 rounded-xl p-8 text-center cursor-pointer transition-all duration-300 relative overflow-hidden hover:-translate-y-1 hover:shadow-xl hover:border-indigo-500 group" 
                         data-mode="generate" 
                         @click=${this.handleMenuClick} 
                         @blur=${this.handleCardBlur} 
                         tabindex="0">
                        <div class="absolute inset-0 bg-gradient-to-br from-indigo-500 to-purple-600 opacity-0 group-hover:opacity-5 transition-opacity duration-300"></div>
                        <div class="text-5xl mb-4 relative z-10">🎲</div>
                        <h3 class="text-gray-800 mb-2 text-2xl font-semibold relative z-10">${msg('Generic Hash')}</h3>
                        <p class="text-gray-500 text-sm relative z-10">${msg('Generate customizable hashes with various alphabets')}</p>
                    </div>
                    
                    <div class="bg-white border-2 border-gray-200 rounded-xl p-8 text-center cursor-pointer transition-all duration-300 relative overflow-hidden hover:-translate-y-1 hover:shadow-xl hover:border-indigo-500 group" 
                         data-mode="password" 
                         @click=${this.handleMenuClick} 
                         @blur=${this.handleCardBlur} 
                         tabindex="0">
                        <div class="absolute inset-0 bg-gradient-to-br from-indigo-500 to-purple-600 opacity-0 group-hover:opacity-5 transition-opacity duration-300"></div>
                        <div class="text-5xl mb-4 relative z-10">🔐</div>
                        <h3 class="text-gray-800 mb-2 text-2xl font-semibold relative z-10">${msg('Password')}</h3>
                        <p class="text-gray-500 text-sm relative z-10">${msg('Create strong passwords with symbols')}</p>
                    </div>
                    
                    <div class="bg-white border-2 border-gray-200 rounded-xl p-8 text-center cursor-pointer transition-all duration-300 relative overflow-hidden hover:-translate-y-1 hover:shadow-xl hover:border-indigo-500 group" 
                         data-mode="apiKey" 
                         @click=${this.handleMenuClick} 
                         @blur=${this.handleCardBlur} 
                         tabindex="0">
                        <div class="absolute inset-0 bg-gradient-to-br from-indigo-500 to-purple-600 opacity-0 group-hover:opacity-5 transition-opacity duration-300"></div>
                        <div class="text-5xl mb-4 relative z-10">🔑</div>
                        <h3 class="text-gray-800 mb-2 text-2xl font-semibold relative z-10">${msg('API Key')}</h3>
                        <p class="text-gray-500 text-sm relative z-10">${msg('Generate secure API keys (ak_ prefix)')}</p>
                    </div>
                </div>
            </div>
            
            <!-- Configuration Views -->
            <generic-hash-page id="generate-page" class="${this.currentView === 'generate' ? 'block animate-fadeIn' : 'hidden'}"></generic-hash-page>
            <password-page id="password-page" class="${this.currentView === 'password' ? 'block animate-fadeIn' : 'hidden'}"></password-page>
            <api-key-page id="apikey-page" class="${this.currentView === 'apiKey' ? 'block animate-fadeIn' : 'hidden'}"></api-key-page>
            
            <!-- Result View -->
            <hash-result 
                id="result-page" 
                class="${this.currentView === 'result' ? 'block animate-fadeIn' : 'hidden'}"
                .hashType=${this.lastHashType}
                .generatedHash=${this.lastResult}
                .parameters=${this.lastParameters}>
            </hash-result>
        `;
    }

    handleMenuClick(e) {
        const target = e.currentTarget;
        const mode = target.dataset.mode;
        if (mode) {
            this.switchView(mode);
        }
        // Remove focus after click
        target.blur();
    }
    
    handleCardBlur(e) {
        // Ensure the card loses focus
        e.currentTarget.blur();
    }

    switchView(viewName) {
        this.currentView = viewName;
    }
    
    async generateHash(hashType, parameters) {
        // Store the parameters for regeneration
        this.lastHashType = hashType;
        this.lastParameters = parameters;
        
        // Switch to result view and show loading
        this.currentView = 'result';
        
        // Wait for the next frame to ensure the view is rendered
        await this.updateComplete;
        
        // Update result component to show loading
        const resultView = this.shadowRoot.querySelector('#result-page');
        if (resultView) {
            resultView.isLoading = true;
            resultView.error = null;
        }
        
        try {
            let response;
            let url;
            
            // Build the appropriate API call based on hash type
            if (hashType === 'apiKey') {
                const params = new URLSearchParams({ 
                    length: parameters.length || 44,
                    raw: 'true'
                });
                // Add alphabet parameter if specified
                if (parameters.alphabet) {
                    params.append('alphabet', parameters.alphabet);
                }
                url = buildApiUrl(`/api/api-key?${params}`);
                response = await fetch(url);
            } else if (hashType === 'password') {
                const params = new URLSearchParams({ 
                    length: parameters.length || 21 
                });
                // Add alphabet parameter if specified
                if (parameters.alphabet) {
                    params.append('alphabet', parameters.alphabet);
                }
                url = buildApiUrl(`/api/password?${params}`);
                response = await fetch(url);
            } else {
                // Generic hash
                const params = new URLSearchParams({
                    length: parameters.length || 21,
                    alphabet: parameters.alphabet || 'base58',
                    raw: 'true'
                });
                
                if (parameters.prefix) params.append('prefix', parameters.prefix);
                if (parameters.suffix) params.append('suffix', parameters.suffix);
                
                url = buildApiUrl(`/api/generate?${params}`);
                response = await fetch(url);
            }
            
            const result = await response.text();
            
            if (response.ok) {
                this.lastResult = result;
                if (resultView) {
                    resultView.generatedHash = result;
                    resultView.isLoading = false;
                }
            } else {
                throw new Error(response.statusText);
            }
        } catch (error) {
            if (resultView) {
                resultView.error = error.message;
                resultView.isLoading = false;
            }
        }
    }
}

customElements.define('menu-page', MenuPage);