import { LitElement, html, css } from 'lit';
import { property, state, query } from 'lit/decorators.js';

export class HashGenerator extends LitElement {
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

    @state()
    currentView = 'menu';

    constructor() {
        super();
        // Listen for back-to-menu events from child components
        this.addEventListener('back-to-menu', () => {
            this.switchView('menu');
        });
    }

    render() {
        return html`
            <!-- Menu View -->
            <div id="menu-view" class="view-container ${this.currentView === 'menu' ? 'active' : ''}">
                <div class="menu-grid">
                    <div class="menu-card" data-mode="generate" @click=${this.handleMenuClick}>
                        <div class="menu-icon">🎲</div>
                        <h3>Generic Hash</h3>
                        <p>Generate customizable hashes with various alphabets</p>
                    </div>
                    
                    <div class="menu-card" data-mode="password" @click=${this.handleMenuClick}>
                        <div class="menu-icon">🔐</div>
                        <h3>Password</h3>
                        <p>Create strong passwords with symbols</p>
                    </div>
                    
                    <div class="menu-card" data-mode="apiKey" @click=${this.handleMenuClick}>
                        <div class="menu-icon">🔑</div>
                        <h3>API Key</h3>
                        <p>Generate secure API keys (ak_ prefix)</p>
                    </div>
                </div>
            </div>
            
            <!-- Views will be inserted here by other components -->
            <generic-hash-view id="generate-view" class="view-container ${this.currentView === 'generate' ? 'active' : ''}"></generic-hash-view>
            <password-view id="password-view" class="view-container ${this.currentView === 'password' ? 'active' : ''}"></password-view>
            <api-key-view id="apikey-view" class="view-container ${this.currentView === 'apiKey' ? 'active' : ''}"></api-key-view>
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
}

customElements.define('hash-generator', HashGenerator);