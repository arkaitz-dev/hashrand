class GenericHashView extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.render();
        this.bindEvents();
    }
    
    render() {
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: block;
                }
                
                /* Back button styles */
                .back-button {
                    background: transparent;
                    border: 2px solid #667eea;
                    color: #667eea;
                    margin-bottom: 1.5rem;
                    width: auto;
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
                
                .back-button:hover {
                    background: #667eea;
                    color: white;
                    transform: translateY(0);
                    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
                }
                
                /* Form styles */
                .form-section {
                    margin-bottom: 2rem;
                }
                
                .form-section h2 {
                    color: #2c3e50;
                    margin-bottom: 1rem;
                    font-size: 1.3rem;
                    display: flex;
                    align-items: center;
                    gap: 0.5rem;
                }
                
                .form-group {
                    margin-bottom: 1.5rem;
                }
                
                label {
                    display: block;
                    margin-bottom: 0.5rem;
                    font-weight: 600;
                    color: #34495e;
                }
                
                input, select, button {
                    width: 100%;
                    padding: 12px 16px;
                    border: 2px solid #e1e8ed;
                    border-radius: 8px;
                    font-size: 1rem;
                    transition: all 0.3s ease;
                    font-family: inherit;
                }
                
                input:focus, select:focus {
                    outline: none;
                    border-color: #667eea;
                    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
                }
                
                .range-group {
                    display: flex;
                    align-items: center;
                    gap: 1rem;
                }
                
                input[type="range"] {
                    flex: 1;
                }
                
                .range-value {
                    background: #667eea;
                    color: white;
                    padding: 8px 12px;
                    border-radius: 6px;
                    font-weight: 600;
                    min-width: 60px;
                    text-align: center;
                }
                
                /* Button styles */
                button {
                    background: #667eea;
                    color: white;
                    border: none;
                    cursor: pointer;
                    font-weight: 600;
                    text-transform: uppercase;
                    letter-spacing: 0.5px;
                    transition: all 0.3s ease;
                }
                
                button:hover:not(:disabled) {
                    background: #5a67d8;
                    transform: translateY(-2px);
                    box-shadow: 0 8px 20px rgba(102, 126, 234, 0.3);
                }
                
                button:active {
                    transform: translateY(0);
                }
                
                button:disabled {
                    opacity: 0.6;
                    cursor: not-allowed;
                }
                
                /* Result styles */
                .result-section {
                    background: #f8f9fa;
                    border-radius: 8px;
                    padding: 1.5rem;
                    margin-top: 2rem;
                    min-height: 120px;
                }
                
                .result-section h3 {
                    color: #2c3e50;
                    margin-bottom: 1rem;
                    margin-top: 0;
                }
                
                .result-display {
                    background: white;
                    border: 2px solid #e1e8ed;
                    border-radius: 6px;
                    padding: 1rem;
                    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
                    font-size: 1.1rem;
                    word-break: break-all;
                    min-height: 60px;
                    display: flex;
                    align-items: center;
                    position: relative;
                }
                
                .result-display.success {
                    border-color: #27ae60;
                    background: #d5f4e6;
                    color: #27ae60;
                }
                
                .result-display.error {
                    border-color: #e74c3c;
                    background: #f8d7da;
                    color: #e74c3c;
                }
                
                .copy-btn {
                    position: absolute;
                    top: 8px;
                    right: 8px;
                    width: auto;
                    padding: 6px 12px;
                    font-size: 0.8rem;
                    background: #667eea;
                    border-radius: 4px;
                }
                
                .loading {
                    display: inline-block;
                    width: 20px;
                    height: 20px;
                    border: 3px solid #f3f3f3;
                    border-top: 3px solid #667eea;
                    border-radius: 50%;
                    animation: spin 1s linear infinite;
                    margin-right: 10px;
                }
                
                @keyframes spin {
                    0% { transform: rotate(0deg); }
                    100% { transform: rotate(360deg); }
                }
                
                @media (max-width: 768px) {
                    .range-group {
                        flex-direction: column;
                        align-items: stretch;
                    }
                }
            </style>
            
            <button class="back-button">← Back to Menu</button>
            
            <div class="form-section">
                <h2>🎲 Generate Custom Hash</h2>
                
                <div class="form-group">
                    <label for="generate-length">Length</label>
                    <div class="range-group">
                        <input type="range" id="generate-length" min="2" max="128" value="21">
                        <span class="range-value">21</span>
                    </div>
                </div>
                
                <div class="form-group">
                    <label for="generate-alphabet">Alphabet Type</label>
                    <select id="generate-alphabet">
                        <option value="base58">Base58 (Bitcoin)</option>
                        <option value="no-look-alike">No Look-alike</option>
                        <option value="full">Full Alphanumeric</option>
                        <option value="full-with-symbols">Full with Symbols</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label for="generate-prefix">Prefix (optional)</label>
                    <input type="text" id="generate-prefix" placeholder="e.g., user_">
                </div>
                
                <div class="form-group">
                    <label for="generate-suffix">Suffix (optional)</label>
                    <input type="text" id="generate-suffix" placeholder="e.g., _temp">
                </div>
                
                <button id="generate-btn">Generate Hash</button>
            </div>
            
            <div class="result-section">
                <h3>Result</h3>
                <div id="generate-result" class="result-display">
                    <span>Generated hash will appear here</span>
                </div>
            </div>
        `;
    }
    
    bindEvents() {
        // Back button
        this.shadowRoot.querySelector('.back-button').addEventListener('click', () => {
            this.dispatchEvent(new CustomEvent('back-to-menu', { bubbles: true }));
        });
        
        // Range input
        const rangeInput = this.shadowRoot.getElementById('generate-length');
        const rangeValue = this.shadowRoot.querySelector('.range-value');
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
        resultDiv.innerHTML = '<span class="loading"></span>Generating...';
        resultDiv.className = 'result-display';
        
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
                resultDiv.innerHTML = `<span>${result}</span><button class="copy-btn" onclick="navigator.clipboard.writeText('${result}')">Copy</button>`;
                resultDiv.className = 'result-display success';
            } else {
                resultDiv.innerHTML = `<span>Error: ${response.statusText}</span>`;
                resultDiv.className = 'result-display error';
            }
        } catch (error) {
            resultDiv.innerHTML = `<span>Error: ${error.message}</span>`;
            resultDiv.className = 'result-display error';
        }
    }
}

customElements.define('generic-hash-view', GenericHashView);