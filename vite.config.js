import { defineConfig } from 'vite'
import babel from 'vite-plugin-babel'
import tailwindcssPostcss from '@tailwindcss/postcss'
import { visualizer } from 'rollup-plugin-visualizer'

export default defineConfig({
  root: 'web-ui',
  base: '/',
  publicDir: 'public',
  css: {
    postcss: {
      plugins: [
        tailwindcssPostcss,
      ],
    },
  },
  server: {
    port: 3000,
    host: true,
    allowedHosts: ['elite.faun-pirate.ts.net', 'localhost', '127.0.0.1'],
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
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true,
        pure_funcs: ['console.log', 'console.info', 'console.debug', 'console.trace'],
        passes: 2
      },
      mangle: {
        safari10: true
      },
      format: {
        comments: false
      }
    },
    rollupOptions: {
      output: {
        manualChunks: (id) => {
          // Separate vendor chunks for better caching
          if (id.includes('node_modules')) {
            if (id.includes('lit') || id.includes('@lit')) {
              return 'lit-core';
            }
            return 'vendor';
          }
          // Group all locale files together
          if (id.includes('/src/locales/') && id.endsWith('.js')) {
            return 'locales';
          }
        }
      },
      // Tree-shaking optimizations
      treeshake: {
        moduleSideEffects: false,
        propertyReadSideEffects: false,
        tryCatchDeoptimization: false
      }
    },
    // Enable CSS code splitting
    cssCodeSplit: true,
    // Optimize chunk size warnings
    chunkSizeWarningLimit: 500
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
    }),
    // Bundle analysis (only in build mode)
    process.env.ANALYZE && visualizer({
      filename: '../dist/stats.html',
      open: false,
      gzipSize: true,
      brotliSize: true
    })
  ].filter(Boolean)
})