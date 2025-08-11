import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';

export class GenericHashView extends LitElement {
    @state()
    accessor lengthValue = 21;
    static styles = css`
        :host {
            display: block;
        }

        /* Import shared styles from main.css by including them directly */
        .wc-back-button {
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

        .wc-back-button:hover {
            background: #667eea;
            color: white;
            transform: translateY(0);
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
        }

        .wc-form-section {
            margin-bottom: 2rem;
        }

        .wc-form-section h2 {
            color: #2c3e50;
            margin-bottom: 1rem;
            font-size: 1.3rem;
            display: flex;
            align-items: center;
            gap: 0.5rem;
        }

        .wc-form-group {
            margin-bottom: 1.5rem;
        }

        .wc-form-group label {
            display: block;
            margin-bottom: 0.5rem;
            font-weight: 600;
            color: #34495e;
        }

        .wc-input, .wc-select {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e1e8ed;
            border-radius: 8px;
            font-size: 16px;
            font-family: inherit;
            transition: all 0.3s ease;
            background: white;
        }

        .wc-input:focus, .wc-select:focus {
            outline: none;
            border-color: #667eea;
            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }

        .wc-range-group {
            display: flex;
            align-items: center;
            gap: 1rem;
        }

        .wc-range-group input[type="range"] {
            flex: 1;
            height: 8px;
            border-radius: 4px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            outline: none;
            -webkit-appearance: none;
        }

        .wc-range-group input[type="range"]::-webkit-slider-thumb {
            appearance: none;
            width: 20px;
            height: 20px;
            border-radius: 50%;
            background: white;
            border: 3px solid #667eea;
            cursor: pointer;
            transition: all 0.2s ease;
        }

        .wc-range-group input[type="range"]::-webkit-slider-thumb:hover {
            transform: scale(1.1);
            box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
        }

        .wc-range-value {
            background: #667eea;
            color: white;
            padding: 8px 12px;
            border-radius: 6px;
            font-weight: bold;
            min-width: 40px;
            text-align: center;
        }

        .wc-button {
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
            margin-top: 1rem;
        }

        .wc-button:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
        }

    `;

    render() {
        return html`
            <button class="wc-back-button" @click=${this.handleBackClick}>← Back to Menu</button>
            
            <div class="wc-form-section">
                <h2>🎲 Generate Custom Hash</h2>
                
                <div class="wc-form-group">
                    <label for="generate-length">Length</label>
                    <div class="wc-range-group">
                        <input type="range" id="generate-length" min="2" max="128" .value=${this.lengthValue.toString()} @input=${this.handleLengthChange}>
                        <span class="wc-range-value">${this.lengthValue}</span>
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
                
                <button id="generate-btn" class="wc-button" @click=${this.handleGenerate}>Generate Hash</button>
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