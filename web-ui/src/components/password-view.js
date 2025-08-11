import { LitElement, html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { msg, updateWhenLocaleChanges } from '@lit/localize';

export class PasswordView extends LitElement {
    @state()
    accessor lengthValue = 21;
    
    @state()
    accessor alphabetType = 'full-with-symbols';
    
    static styles = css`
        :host {
            display: block;
        }

        /* Shared styles from main.css */
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

        .wc-select {
            width: 100%;
            padding: 12px 16px;
            border: 2px solid #e1e8ed;
            border-radius: 8px;
            font-size: 16px;
            font-family: inherit;
            transition: all 0.3s ease;
            background: white;
        }

        .wc-select:focus {
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

        .wc-info-box {
            background: #e3f2fd;
            border: 1px solid #bbdefb;
            border-radius: 6px;
            padding: 1rem;
            margin: 1rem 0;
            font-size: 0.9rem;
            color: #1976d2;
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

    connectedCallback() {
        super.connectedCallback();
        // Enable automatic re-rendering when locale changes
        updateWhenLocaleChanges(this);
    }

    render() {
        // Dynamic minimum length based on alphabet for same entropy (~130 bits)
        const minLength = this.alphabetType === 'no-look-alike' ? 24 : 21;
        
        // Adjust length if switching alphabets and current length is below minimum
        if (this.lengthValue < minLength) {
            this.lengthValue = minLength;
        }
        
        return html`
            <button class="wc-back-button" @click=${this.handleBackClick}>${msg('← Back to Menu')}</button>
            
            <div class="wc-form-section">
                <h2>${msg('🔐 Generate Password')}</h2>
                
                <div class="wc-form-group">
                    <label for="password-alphabet">${msg('Alphabet Type')}</label>
                    <select id="password-alphabet" class="wc-select" @change=${this.handleAlphabetChange}>
                        <option value="full-with-symbols" ?selected=${this.alphabetType === 'full-with-symbols'}>
                            ${msg('Full with Symbols (Maximum Security)')}
                        </option>
                        <option value="no-look-alike" ?selected=${this.alphabetType === 'no-look-alike'}>
                            ${msg('No Look-alike (Easy to Type)')}
                        </option>
                    </select>
                </div>
                
                <div class="wc-form-group">
                    <label for="password-length">${msg('Length')} (${minLength}-44 ${msg('characters')})</label>
                    <div class="wc-range-group">
                        <input type="range" id="password-length" min="${minLength}" max="44" .value=${this.lengthValue.toString()} @input=${this.handleLengthChange}>
                        <span class="wc-range-value">${this.lengthValue}</span>
                    </div>
                </div>
                
                <div class="wc-info-box">
                    <strong>${msg('Security Note:')}:</strong> 
                    ${this.alphabetType === 'no-look-alike' 
                        ? msg('No Look-alike alphabet excludes confusable characters. Minimum 24 characters for equivalent security.')
                        : msg('Full alphabet with symbols provides maximum entropy. Minimum 21 characters for strong security.')}
                </div>
                
                <button id="password-btn" class="wc-button" @click=${this.handleGenerate}>${msg('Generate Password')}</button>
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
        const minLength = this.alphabetType === 'no-look-alike' ? 24 : 21;
        if (this.lengthValue < minLength) {
            this.lengthValue = minLength;
        }
    }

    handleGenerate() {
        const length = this.shadowRoot.querySelector('#password-length').value;
        const alphabet = this.shadowRoot.querySelector('#password-alphabet').value;
        
        const parameters = {
            length: parseInt(length),
            alphabet: alphabet
        };
        
        // Emit event to parent component
        this.dispatchEvent(new CustomEvent('generate-hash', {
            bubbles: true,
            composed: true,
            detail: {
                hashType: 'password',
                parameters: parameters
            }
        }));
    }
}

customElements.define('password-view', PasswordView);