import { defineConfig } from 'vite'
import babel from 'vite-plugin-babel'

export default defineConfig({
  root: 'web-ui',
  publicDir: 'public',
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
    outDir: '../dist',
    emptyOutDir: true,
    rollupOptions: {
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
    },
    babel({
      babelConfig: {
        babelrc: false,
        configFile: false,
        plugins: [
          ["@babel/plugin-proposal-decorators", { 
            version: "2023-05"
          }]
        ]
      },
      filter: /\.(js|jsx)$/,
      include: 'web-ui/**'
    })
  ]
})