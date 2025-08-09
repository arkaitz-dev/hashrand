# Refactor Plan - Web UI Directory Restructure (Adjusted) - 2025-08-09

## Ajuste Basado en Feedback del Usuario

**Requirement**: Mantener los comandos npm ejecutables desde la raíz del proyecto (sin `cd web-ui`)

## Solución: Estructura Híbrida

### 📁 Nueva Estructura Propuesta (Ajustada)

```
hashrand/
├── Cargo.toml              # Configuración Rust
├── package.json            # Configuración npm (queda en raíz)
├── package-lock.json       # Lock file npm (queda en raíz)  
├── vite.config.js          # Config Vite (queda en raíz, paths ajustados)
├── src/                    # Solo código Rust
│   ├── main.rs
│   ├── cli.rs
│   ├── server.rs
│   └── generators/
├── web-ui/                 # Archivos de UI organizados
│   ├── index.html
│   └── src/
│       ├── index.js
│       ├── components/     # Componentes Lit
│       └── css/
└── dist/                   # Build de producción
```

### 🔄 Archivos a Mover (Revisado)

| Origen | Destino | Notas |
|--------|---------|--------|
| `/index.html` | `/web-ui/index.html` | Entry point HTML |
| `/src/components/` | `/web-ui/src/components/` | Componentes Lit (5 archivos) |
| `/src/index.js` | `/web-ui/src/index.js` | Entry point JS |
| `/static/css/main.css` | `/web-ui/src/css/main.css` | Estilos CSS |
| `/package.json` | **QUEDA EN RAÍZ** | ✅ npm desde raíz |
| `/package-lock.json` | **QUEDA EN RAÍZ** | ✅ npm desde raíz |
| `/vite.config.js` | **QUEDA EN RAÍZ** | ✅ npm desde raíz |

### ⚙️ Configuraciones Necesarias

#### 1. Actualizar `vite.config.js` (en raíz)
```javascript
export default defineConfig({
  // Entry point HTML en subdirectorio
  build: {
    rollupOptions: {
      input: './web-ui/index.html'
    },
    outDir: 'dist'
  },
  // Assets desde web-ui
  publicDir: 'web-ui/public',
  // Server proxy para API
  server: {
    proxy: {
      '/api': 'http://localhost:8080'
    }
  }
});
```

#### 2. Actualizar paths en `web-ui/index.html`
```html
<link rel="stylesheet" href="/src/css/main.css">
<script type="module" src="/src/index.js"></script>
```

#### 3. Mantener `package.json` en raíz con scripts ajustados
```json
{
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  }
}
```

### ⚡ Flujos de Trabajo Finales

**Desarrollo:**
```bash
npm run dev          # ✅ Desde raíz - UI en localhost:3000
cargo run -- --serve 8080  # API server
```

**Producción:**
```bash
npm run build        # ✅ Desde raíz - genera dist/
cargo run -- --serve 8080  # Sirve desde dist/
```

### ✅ Ventajas de Esta Solución

1. **Comandos npm desde raíz** ✅ Como solicitaste
2. **Separación clara de archivos** ✅ UI organizada en web-ui/
3. **Sin cambios en Rust** ✅ Servidor sigue igual
4. **Configuración centralizada** ✅ package.json y vite.config.js en raíz
5. **Estructura familiar** ✅ Mantiene convenciones que conoces

### 📋 Pasos de Ejecución Simplificados

1. Crear directorio `/web-ui/src/`
2. Mover archivos HTML/JS/CSS a `web-ui/`
3. Actualizar paths en `vite.config.js`
4. Actualizar paths en `web-ui/index.html`
5. Mantener `package.json` y `vite.config.js` en raíz
6. Eliminar archivos legacy
7. Validar workflows

---

**¿Te parece mejor esta solución?** Mantienes `npm run dev` y `npm run build` desde la raíz, pero los archivos de UI quedan organizados en `web-ui/`.