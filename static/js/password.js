class PasswordView extends HTMLElement {
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
                <h2>🔐 Generate Password</h2>
                
                <div class="wc-form-group">
                    <label for="password-length">Length (21-44 characters)</label>
                    <div class="wc-range-group">
                        <input type="range" id="password-length" min="21" max="44" value="21">
                        <span class="wc-range-value">21</span>
                    </div>
                </div>
                
                <div class="wc-info-box">
                    <strong>Password Strength:</strong> Uses full alphanumeric alphabet with symbols for maximum security.
                </div>
                
                <button id="password-btn" class="wc-button">Generate Password</button>
            </div>
            
            <div class="wc-result-section">
                <h3>Result</h3>
                <div id="password-result" class="wc-result-display">
                    <span>Generated password will appear here</span>
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
        const rangeInput = this.shadowRoot.getElementById('password-length');
        const rangeValue = this.shadowRoot.querySelector('.wc-range-value');
        rangeInput.addEventListener('input', (e) => {
            rangeValue.textContent = e.target.value;
        });
        
        // Generate button
        this.shadowRoot.getElementById('password-btn').addEventListener('click', () => {
            this.generatePassword();
        });
    }
    
    async generatePassword() {
        const length = this.shadowRoot.getElementById('password-length').value;
        const resultDiv = this.shadowRoot.getElementById('password-result');
        
        // Show loading
        resultDiv.innerHTML = '<span class="wc-loading"></span>Generating...';
        resultDiv.className = 'wc-result-display';
        
        try {
            const params = new URLSearchParams({
                length: length,
                raw: 'true'
            });
            
            const response = await fetch(`/api/password?${params}`);
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

customElements.define('password-view', PasswordView);