class ApiKeyView extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.render();
        this.bindEvents();
    }
    
    render() {
        this.shadowRoot.innerHTML = `
            <link rel="stylesheet" href="/css/main.css">
            <style>
                :host {
                    display: block;
                }
            </style>
            
            <button class="wc-back-button">← Back to Menu</button>
            
            <div class="wc-form-section">
                <h2>🔑 Generate API Key</h2>
                
                <div class="wc-info-box">
                    <strong>Format:</strong> ak_ prefix + 44 random characters using full alphanumeric alphabet (256-bit entropy)
                </div>
                
                <button id="apikey-btn" class="wc-button">Generate API Key</button>
            </div>
            
            <div class="wc-result-section">
                <h3>Result</h3>
                <div id="apikey-result" class="wc-result-display">
                    <span>Generated API key will appear here</span>
                </div>
            </div>
        `;
    }
    
    bindEvents() {
        // Back button
        this.shadowRoot.querySelector('.wc-back-button').addEventListener('click', () => {
            this.dispatchEvent(new CustomEvent('back-to-menu', { bubbles: true }));
        });
        
        // Generate button
        this.shadowRoot.getElementById('apikey-btn').addEventListener('click', () => {
            this.generateApiKey();
        });
    }
    
    async generateApiKey() {
        const resultDiv = this.shadowRoot.getElementById('apikey-result');
        
        // Show loading
        resultDiv.innerHTML = '<span class="wc-loading"></span>Generating...';
        resultDiv.className = 'wc-result-display';
        
        try {
            const params = new URLSearchParams({
                raw: 'true'
            });
            
            const response = await fetch(`/api/api-key?${params}`);
            const result = await response.text();
            
            if (response.ok) {
                // Keep the existing structure, just update the content
                resultDiv.innerHTML = `<span>${result}</span><button class="wc-copy-btn" onclick="navigator.clipboard.writeText('${result}')">Copy</button>`;
                resultDiv.className = 'wc-result-display success';
            } else {
                resultDiv.innerHTML = `<span>Error: ${response.statusText}</span>`;
                resultDiv.className = 'wc-result-display error';
            }
        } catch (error) {
            resultDiv.innerHTML = `<span>Error: ${error.message}</span>`;
            resultDiv.className = 'wc-result-display error';
        }
    }
}

customElements.define('api-key-view', ApiKeyView);