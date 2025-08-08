class HashGenerator extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.currentView = 'menu';
        this.render();
        this.bindEvents();
    }
    
    render() {
        this.shadowRoot.innerHTML = `
            <style>
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
            </style>
            
            <!-- Menu View -->
            <div id="menu-view" class="view-container active">
                <div class="menu-grid">
                    <div class="menu-card" data-mode="generate">
                        <div class="menu-icon">🎲</div>
                        <h3>Generic Hash</h3>
                        <p>Generate customizable hashes with various alphabets</p>
                    </div>
                    
                    <div class="menu-card" data-mode="password">
                        <div class="menu-icon">🔐</div>
                        <h3>Password</h3>
                        <p>Create strong passwords with symbols</p>
                    </div>
                    
                    <div class="menu-card" data-mode="apiKey">
                        <div class="menu-icon">🔑</div>
                        <h3>API Key</h3>
                        <p>Generate secure API keys (ak_ prefix)</p>
                    </div>
                </div>
            </div>
            
            <!-- Views will be inserted here by other components -->
            <generic-hash-view id="generate-view" class="view-container"></generic-hash-view>
            <password-view id="password-view" class="view-container"></password-view>
            <api-key-view id="apikey-view" class="view-container"></api-key-view>
        `;
    }
    
    bindEvents() {
        // Menu navigation
        this.shadowRoot.querySelectorAll('.menu-card').forEach(card => {
            card.addEventListener('click', (e) => {
                const mode = e.currentTarget.dataset.mode;
                this.switchView(mode);
            });
        });
        
        // Listen for back-to-menu events from child components
        this.shadowRoot.addEventListener('back-to-menu', () => {
            this.switchView('menu');
        });
    }
    
    switchView(viewName) {
        // Hide all views
        this.shadowRoot.querySelectorAll('.view-container').forEach(view => {
            view.classList.remove('active');
        });
        
        // Show selected view
        if (viewName === 'menu') {
            this.shadowRoot.getElementById('menu-view').classList.add('active');
        } else if (viewName === 'generate') {
            this.shadowRoot.getElementById('generate-view').classList.add('active');
        } else if (viewName === 'password') {
            this.shadowRoot.getElementById('password-view').classList.add('active');
        } else if (viewName === 'apiKey') {
            this.shadowRoot.getElementById('apikey-view').classList.add('active');
        }
        
        this.currentView = viewName;
    }
}

customElements.define('hash-generator', HashGenerator);