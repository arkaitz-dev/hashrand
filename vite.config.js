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
    },
    // SPA fallback - redirect all non-asset routes to index.html
    historyApiFallback: {
      rewrites: [
        { from: /^\/api\/.*$/, to: function(context) {
          return context.parsedUrl.pathname;
        }},
        { from: /^\/assets\/.*$/, to: function(context) {
          return context.parsedUrl.pathname;
        }},
        { from: /./, to: '/index.html' }
      ]
    }
  },
  build: {
    outDir: '../dist',
    emptyOutDir: true,
    minify: true
  },
  plugins: [
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