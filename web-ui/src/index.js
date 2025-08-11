// Import all Lit components
import './components/hash-generator.js';
import './components/generic-hash-view.js';
import './components/password-view.js';
import './components/api-key-view.js';
import './components/hash-result.js';

// Load version from API
async function loadVersion() {
    try {
        const response = await fetch('/api/version');
        if (response.ok) {
            const data = await response.json();
            const versionElement = document.getElementById('version');
            if (versionElement) {
                versionElement.textContent = `v${data.version}`;
            }
        }
    } catch (error) {
        console.warn('Could not load version:', error);
    }
}

// Load version when DOM is ready
document.addEventListener('DOMContentLoaded', loadVersion);