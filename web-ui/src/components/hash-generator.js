import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { msg, updateWhenLocaleChanges } from '@lit/localize';
import { buildApiUrl } from '../utils/api.js';

export class HashGenerator extends LitElement {
    @state()
    accessor currentView = 'menu';
    
    @state()
    accessor lastHashType = '';
    
    @state()
    accessor lastParameters = {};
    
    @state()
    accessor lastResult = '';

    static styles = css`
        :host {
            display: block;
        }
        
        /* Menu styles */
        .menu-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 2rem;
            margin: 2rem 0;
        }
        
        .menu-card {
            background: white;
            border: 2px solid #e1e8ed;
            border-radius: 12px;
            padding: 2rem;
            text-align: center;
            cursor: pointer;
            transition: all 0.3s ease;
            position: relative;
            overflow: hidden;
        }
        
        .menu-card::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            opacity: 0;
            transition: opacity 0.3s ease;
        }
        
        .menu-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 12px 24px rgba(0, 0, 0, 0.15);
            border-color: #667eea;
        }
        
        .menu-card:hover::before {
            opacity: 0.05;
        }
        
        .menu-icon {
            font-size: 3rem;
            margin-bottom: 1rem;
            position: relative;
            z-index: 1;
        }
        
        .menu-card h3 {
            color: #2c3e50;
            margin-bottom: 0.5rem;
            font-size: 1.5rem;
            position: relative;
            z-index: 1;
        }
        
        .menu-card p {
            color: #7f8c8d;
            font-size: 0.95rem;
            position: relative;
            z-index: 1;
            margin: 0;
        }
        
        /* View container styles */
        .view-container {
            display: none;
        }
        
        .view-container.active {
            display: block;
            animation: fadeIn 0.3s ease;
        }
        
        @keyframes fadeIn {
            from { 
                opacity: 0; 
                transform: translateY(10px); 
            }
            to { 
                opacity: 1; 
                transform: translateY(0); 
            }
        }
    `;
    
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
            <div id="menu-view" class="view-container ${this.currentView === 'menu' ? 'active' : ''}">
                <div class="menu-grid">
                    <div class="menu-card" data-mode="generate" @click=${this.handleMenuClick}>
                        <div class="menu-icon">🎲</div>
                        <h3>${msg('Generic Hash')}</h3>
                        <p>${msg('Generate customizable hashes with various alphabets')}</p>
                    </div>
                    
                    <div class="menu-card" data-mode="password" @click=${this.handleMenuClick}>
                        <div class="menu-icon">🔐</div>
                        <h3>${msg('Password')}</h3>
                        <p>${msg('Create strong passwords with symbols')}</p>
                    </div>
                    
                    <div class="menu-card" data-mode="apiKey" @click=${this.handleMenuClick}>
                        <div class="menu-icon">🔑</div>
                        <h3>${msg('API Key')}</h3>
                        <p>${msg('Generate secure API keys (ak_ prefix)')}</p>
                    </div>
                </div>
            </div>
            
            <!-- Configuration Views -->
            <generic-hash-view id="generate-view" class="view-container ${this.currentView === 'generate' ? 'active' : ''}"></generic-hash-view>
            <password-view id="password-view" class="view-container ${this.currentView === 'password' ? 'active' : ''}"></password-view>
            <api-key-view id="apikey-view" class="view-container ${this.currentView === 'apiKey' ? 'active' : ''}"></api-key-view>
            
            <!-- Result View -->
            <hash-result 
                id="result-view" 
                class="view-container ${this.currentView === 'result' ? 'active' : ''}"
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
        const resultView = this.shadowRoot.querySelector('#result-view');
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

customElements.define('hash-generator', HashGenerator);