import { LitElement, html, css } from 'lit';

export class HashResult extends LitElement {
    static properties = {
        hashType: { type: String },      // 'generic', 'password', 'apiKey'
        generatedHash: { type: String },  // The generated result
        parameters: { type: Object },     // Generation parameters for regeneration
        isLoading: { type: Boolean, state: true },     // Loading state
        error: { type: String, state: true }           // Error message if any
    };

    static styles = css`
        :host {
            display: block;
        }

        .result-container {
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
        }

        /* Navigation buttons */
        .nav-buttons {
            display: flex;
            gap: 1rem;
            margin-bottom: 2rem;
        }

        .nav-button {
            background: transparent;
            border: 2px solid #667eea;
            color: #667eea;
            padding: 10px 20px;
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            font-weight: 600;
            cursor: pointer;
            border-radius: 8px;
            font-size: 1rem;
            transition: all 0.3s ease;
        }

        .nav-button:hover {
            background: #667eea;
            color: white;
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
        }

        /* Result display section */
        .result-section {
            background: white;
            border-radius: 12px;
            padding: 2rem;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
            margin-bottom: 2rem;
        }

        .result-header {
            display: flex;
            align-items: center;
            gap: 0.75rem;
            margin-bottom: 1.5rem;
        }

        .result-icon {
            font-size: 2rem;
        }

        .result-title {
            color: #2c3e50;
            font-size: 1.5rem;
            font-weight: 600;
            margin: 0;
        }

        .result-display {
            background: #f8f9fa;
            border: 2px solid #e1e8ed;
            border-radius: 8px;
            padding: 1.5rem;
            font-family: 'Courier New', monospace;
            font-size: 1.1rem;
            word-break: break-all;
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 1rem;
            min-height: 60px;
        }

        .result-display.success {
            background: #d4edda;
            border-color: #c3e6cb;
            color: #155724;
        }

        .result-display.error {
            background: #f8d7da;
            border-color: #f5c6cb;
            color: #721c24;
        }

        .result-text {
            flex: 1;
            overflow-wrap: break-word;
        }

        .copy-button {
            background: #28a745;
            color: white;
            border: none;
            padding: 8px 16px;
            border-radius: 6px;
            cursor: pointer;
            font-weight: 600;
            transition: all 0.3s ease;
            flex-shrink: 0;
        }

        .copy-button:hover {
            background: #218838;
            transform: translateY(-1px);
        }

        .copy-button.copied {
            background: #6c757d;
        }

        /* Parameters display */
        .parameters-section {
            background: #f8f9fa;
            border-radius: 8px;
            padding: 1rem;
            margin-top: 1rem;
        }

        .parameters-title {
            color: #6c757d;
            font-size: 0.9rem;
            font-weight: 600;
            margin-bottom: 0.5rem;
        }

        .parameter-item {
            display: flex;
            justify-content: space-between;
            padding: 0.25rem 0;
            font-size: 0.9rem;
        }

        .parameter-label {
            color: #6c757d;
        }

        .parameter-value {
            color: #2c3e50;
            font-weight: 500;
        }

        /* Regenerate button */
        .regenerate-button {
            width: 100%;
            padding: 15px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            border-radius: 8px;
            font-size: 1.1rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s ease;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 0.5rem;
        }

        .regenerate-button:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
        }

        .regenerate-button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
            transform: none;
        }

        /* Loading spinner */
        .loading-spinner {
            display: inline-block;
            width: 20px;
            height: 20px;
            border: 3px solid #f3f3f3;
            border-top: 3px solid #667eea;
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }

        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }
    `;

    constructor() {
        super();
        this.hashType = 'generic';
        this.generatedHash = '';
        this.parameters = {};
        this.isLoading = false;
        this.error = null;
    }

    render() {
        return html`
            <div class="result-container">
                <!-- Navigation buttons -->
                <div class="nav-buttons">
                    <button class="nav-button" @click=${this.handleBackToConfig}>
                        ← Back to Configuration
                    </button>
                    <button class="nav-button" @click=${this.handleBackToMenu}>
                        ⌂ Back to Menu
                    </button>
                </div>

                <!-- Result section -->
                <div class="result-section">
                    <div class="result-header">
                        <span class="result-icon">${this.getIcon()}</span>
                        <h2 class="result-title">${this.getTitle()}</h2>
                    </div>

                    <div class="result-display ${this.getResultClass()}">
                        ${this.isLoading ? html`
                            <span class="loading-spinner"></span>
                            <span>Generating...</span>
                        ` : this.error ? html`
                            <span class="result-text">Error: ${this.error}</span>
                        ` : html`
                            <span class="result-text">${this.generatedHash}</span>
                            <button class="copy-button" @click=${this.handleCopy}>
                                Copy
                            </button>
                        `}
                    </div>

                    ${this.parameters && Object.keys(this.parameters).length > 0 ? html`
                        <div class="parameters-section">
                            <div class="parameters-title">Configuration Used:</div>
                            ${this.renderParameters()}
                        </div>
                    ` : ''}
                </div>

                <!-- Regenerate button -->
                <button class="regenerate-button" @click=${this.handleRegenerate} ?disabled=${this.isLoading}>
                    ${this.isLoading ? html`
                        <span class="loading-spinner"></span>
                        <span>Regenerating...</span>
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
            case 'password': return 'Generated Password';
            case 'apiKey': return 'Generated API Key';
            default: return 'Generated Hash';
        }
    }

    getResultClass() {
        if (this.error) return 'error';
        if (this.generatedHash && !this.isLoading) return 'success';
        return '';
    }

    getRegenerateButtonText() {
        switch(this.hashType) {
            case 'apiKey': 
                // Check if there are parameters (length) configured
                return Object.keys(this.parameters).length > 0 ? 'Regenerate with Same Configuration' : 'Regenerate';
            default: return 'Regenerate with Same Configuration';
        }
    }

    renderParameters() {
        const params = [];
        
        if (this.parameters.length) {
            params.push(html`
                <div class="parameter-item">
                    <span class="parameter-label">Length:</span>
                    <span class="parameter-value">${this.parameters.length}</span>
                </div>
            `);
        }
        
        if (this.parameters.alphabet) {
            const alphabetNames = {
                'base58': 'Base58 (Bitcoin)',
                'no-look-alike': 'No Look-alike',
                'full': 'Full Alphanumeric',
                'full-with-symbols': 'Full with Symbols'
            };
            params.push(html`
                <div class="parameter-item">
                    <span class="parameter-label">Alphabet:</span>
                    <span class="parameter-value">${alphabetNames[this.parameters.alphabet] || this.parameters.alphabet}</span>
                </div>
            `);
        }
        
        if (this.parameters.prefix) {
            params.push(html`
                <div class="parameter-item">
                    <span class="parameter-label">Prefix:</span>
                    <span class="parameter-value">${this.parameters.prefix}</span>
                </div>
            `);
        }
        
        if (this.parameters.suffix) {
            params.push(html`
                <div class="parameter-item">
                    <span class="parameter-label">Suffix:</span>
                    <span class="parameter-value">${this.parameters.suffix}</span>
                </div>
            `);
        }
        
        return params;
    }

    async handleCopy() {
        try {
            await navigator.clipboard.writeText(this.generatedHash);
            // Temporarily change button text
            const button = this.shadowRoot.querySelector('.copy-button');
            if (button) {
                button.textContent = 'Copied!';
                button.classList.add('copied');
                setTimeout(() => {
                    button.textContent = 'Copy';
                    button.classList.remove('copied');
                }, 2000);
            }
        } catch (error) {
            console.error('Failed to copy:', error);
        }
    }

    handleBackToConfig() {
        this.dispatchEvent(new CustomEvent('back-to-config', { 
            bubbles: true,
            composed: true,
            detail: { hashType: this.hashType }
        }));
    }

    handleBackToMenu() {
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