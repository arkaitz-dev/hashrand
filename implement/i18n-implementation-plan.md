# Internacionalización (i18n) Implementation Plan

## Objetivo
Implementar sistema de internacionalización completo para la interfaz web de HashRand usando @lit/localize.

## Estado: COMPLETADO ✅

## Requisitos Implementados

### 1. Idiomas Soportados (8)
- ✅ Inglés (EN) - Base/Source
- ✅ Español (ES)
- ✅ Francés (FR)
- ✅ Portugués (PT)
- ✅ Alemán (DE)
- ✅ Ruso (RU)
- ✅ Chino Mandarín (ZH)
- ✅ Árabe (AR) - con soporte RTL

### 2. Características
- ✅ Detección automática del idioma del navegador
- ✅ Selector manual de idioma en el header
- ✅ Persistencia de selección en localStorage
- ✅ Cambio dinámico sin recargar página
- ✅ Soporte RTL para árabe

## Implementación Técnica

### Arquitectura
- **Librería**: @lit/localize (oficial de Lit)
- **Modo**: Runtime (permite cambio dinámico)
- **Patrón**: Configuración oficial según documentación

### Estructura de Archivos
```
web-ui/
├── src/
│   ├── localization.js       # Configuración central
│   ├── locales/              # Archivos de traducción
│   │   ├── locale-codes.js   # Generado automáticamente
│   │   ├── es.js
│   │   ├── fr.js
│   │   ├── pt.js
│   │   ├── de.js
│   │   ├── ru.js
│   │   ├── zh.js
│   │   └── ar.js
│   └── components/
│       └── language-selector.js  # Selector de idioma
└── xliff/                    # Archivos de intercambio
    ├── es.xlf
    ├── fr.xlf
    └── ...
```

### Flujo de Trabajo
1. **Extracción**: `npx lit-localize extract`
2. **Traducción**: Editar archivos XLIFF
3. **Build**: `npx lit-localize build`
4. **Fix sintaxis**: Script sed para corregir indentación

## Problemas Resueltos

### 1. Archivos con sintaxis incorrecta
- **Problema**: lit-localize generaba archivos con indentación inconsistente
- **Solución**: Script sed para corregir automáticamente la indentación

### 2. Dependencias circulares
- **Problema**: language-selector importaba desde index.js que importaba language-selector
- **Solución**: Mover configuración de localization a archivo separado

### 3. Modo Transform vs Runtime
- **Problema**: Transform no permitía cambio dinámico de idioma
- **Solución**: Usar modo Runtime siguiendo patrón oficial

## Comandos Útiles

```bash
# Extraer mensajes para traducción
npx lit-localize extract

# Generar archivos de locale
npx lit-localize build

# Corregir sintaxis de archivos generados
for file in web-ui/src/locales/*.js; do
  sed -i "s/^'/      '/g" "$file"
done
```

## Componentes Actualizados
- ✅ hash-generator.js
- ✅ generic-hash-view.js
- ✅ password-view.js
- ✅ api-key-view.js
- ✅ hash-result.js
- ✅ language-selector.js (nuevo)

## Pendiente
- ⚠️ Visibilidad del selector de idioma (funciona pero no siempre visible)

## Notas de Implementación
- Todos los textos envueltos con `msg()`
- Cada componente llama `updateWhenLocaleChanges(this)`
- Detección automática basada en navigator.languages
- Persistencia en localStorage con key 'hashrand-locale'