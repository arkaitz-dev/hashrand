import { defineConfig } from 'vite'

export default defineConfig({
  root: '.',
  publicDir: 'static/assets',
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8080',
        changeOrigin: true
      }
    }
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    rollupOptions: {
      input: 'index.html',
      output: {
        manualChunks: undefined
      }
    }
  },
  plugins: [
    {
      name: 'move-scripts-to-body',
      transformIndexHtml(html) {
        // Simple approach: move all module scripts from head to end of body
        const scriptRegex = /<script([^>]*?type="module"[^>]*?)><\/script>/g;
        let scripts = '';
        
        // Extract module scripts from head
        html = html.replace(/<head>([\s\S]*?)<\/head>/, (match, headContent) => {
          const cleanHead = headContent.replace(scriptRegex, (scriptMatch) => {
            scripts += `    ${scriptMatch}\n`;
            return '';
          });
          return `<head>${cleanHead}</head>`;
        });
        
        // Insert scripts before closing body tag
        html = html.replace('</body>', `${scripts}</body>`);
        
        return html;
      }
    }
  ]
})