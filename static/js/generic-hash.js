class GenericHashView extends HTMLElement {
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
                <h2>🎲 Generate Custom Hash</h2>
                
                <div class="wc-form-group">
                    <label for="generate-length">Length</label>
                    <div class="wc-range-group">
                        <input type="range" id="generate-length" min="2" max="128" value="21">
                        <span class="wc-range-value">21</span>
                    </div>
                </div>
                
                <div class="wc-form-group">
                    <label for="generate-alphabet">Alphabet Type</label>
                    <select id="generate-alphabet" class="wc-select">
                        <option value="base58">Base58 (Bitcoin)</option>
                        <option value="no-look-alike">No Look-alike</option>
                        <option value="full">Full Alphanumeric</option>
                        <option value="full-with-symbols">Full with Symbols</option>
                    </select>
                </div>
                
                <div class="wc-form-group">
                    <label for="generate-prefix">Prefix (optional)</label>
                    <input type="text" id="generate-prefix" class="wc-input" placeholder="e.g., user_">
                </div>
                
                <div class="wc-form-group">
                    <label for="generate-suffix">Suffix (optional)</label>
                    <input type="text" id="generate-suffix" class="wc-input" placeholder="e.g., _temp">
                </div>
                
                <button id="generate-btn" class="wc-button">Generate Hash</button>
            </div>
            
            <div class="wc-result-section">
                <h3>Result</h3>
                <div id="generate-result" class="wc-result-display">
                    <span>Generated hash will appear here</span>
                </div>
            </div>
        `;
    }
    
    bindEvents() {
        // Back button
        this.shadowRoot.querySelector('.wc-back-button').addEventListener('click', () => {
            this.dispatchEvent(new CustomEvent('back-to-menu', { bubbles: true }));
        });
        
        // Range input
        const rangeInput = this.shadowRoot.getElementById('generate-length');
        const rangeValue = this.shadowRoot.querySelector('.wc-range-value');
        rangeInput.addEventListener('input', (e) => {
            rangeValue.textContent = e.target.value;
        });
        
        // Generate button
        this.shadowRoot.getElementById('generate-btn').addEventListener('click', () => {
            this.generateHash();
        });
    }
    
    async generateHash() {
        const length = this.shadowRoot.getElementById('generate-length').value;
        const alphabet = this.shadowRoot.getElementById('generate-alphabet').value;
        const prefix = this.shadowRoot.getElementById('generate-prefix').value;
        const suffix = this.shadowRoot.getElementById('generate-suffix').value;
        const resultDiv = this.shadowRoot.getElementById('generate-result');
        
        // Show loading
        resultDiv.innerHTML = '<span class="wc-loading"></span>Generating...';
        resultDiv.className = 'wc-result-display';
        
        try {
            const params = new URLSearchParams({
                length: length,
                alphabet: alphabet,
                raw: 'true'
            });
            
            if (prefix) params.append('prefix', prefix);
            if (suffix) params.append('suffix', suffix);
            
            const response = await fetch(`/api/generate?${params}`);
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

customElements.define('generic-hash-view', GenericHashView);