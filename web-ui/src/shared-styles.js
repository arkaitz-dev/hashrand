import { css, unsafeCSS } from 'lit';

// For now, let's define the essential Tailwind utilities manually
// This will be replaced with generated code from TailwindCSS build
const tailwindUtilities = unsafeCSS(`
  /* Reset base */
  *,::before,::after{--tw-border-spacing-x:0;--tw-border-spacing-y:0;--tw-translate-x:0;--tw-translate-y:0;--tw-rotate:0;--tw-skew-x:0;--tw-skew-y:0;--tw-scale-x:1;--tw-scale-y:1;--tw-pan-x: ;--tw-pan-y: ;--tw-pinch-zoom: ;--tw-scroll-snap-strictness:proximity;--tw-gradient-from-position: ;--tw-gradient-via-position: ;--tw-gradient-to-position: ;--tw-ordinal: ;--tw-slashed-zero: ;--tw-numeric-figure: ;--tw-numeric-spacing: ;--tw-numeric-fraction: ;--tw-ring-inset: ;--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:rgb(59 130 246 / 0.5);--tw-ring-offset-shadow:0 0 #0000;--tw-ring-shadow:0 0 #0000;--tw-shadow:0 0 #0000;--tw-shadow-colored:0 0 #0000;--tw-blur: ;--tw-brightness: ;--tw-contrast: ;--tw-grayscale: ;--tw-hue-rotate: ;--tw-invert: ;--tw-saturate: ;--tw-sepia: ;--tw-drop-shadow: ;--tw-backdrop-blur: ;--tw-backdrop-brightness: ;--tw-backdrop-contrast: ;--tw-backdrop-grayscale: ;--tw-backdrop-hue-rotate: ;--tw-backdrop-invert: ;--tw-backdrop-opacity: ;--tw-backdrop-saturate: ;--tw-backdrop-sepia: ;--tw-contain-size: ;--tw-contain-layout: ;--tw-contain-paint: ;--tw-contain-style: }

  /* Layout */
  .block{display:block}
  .inline{display:inline}
  .flex{display:flex}
  .inline-flex{display:inline-flex}
  .grid{display:grid}
  .hidden{display:none}
  .absolute{position:absolute}
  .relative{position:relative}
  .top-0{top:0px}
  .top-4{top:1rem}
  .right-0{right:0px}
  .right-4{right:1rem}
  .bottom-0{bottom:0px}
  .left-0{left:0px}

  /* Flexbox & Grid */
  .flex-col{flex-direction:column}
  .items-start{align-items:flex-start}
  .items-center{align-items:center}
  .justify-start{justify-content:flex-start}
  .justify-center{justify-content:center}
  .justify-between{justify-content:space-between}
  .gap-1{gap:0.25rem}
  .gap-2{gap:0.5rem}
  .gap-3{gap:0.75rem}
  .gap-4{gap:1rem}
  .gap-6{gap:1.5rem}

  /* Sizing */
  .w-full{width:100%}
  .w-auto{width:auto}
  .w-12{width:3rem}
  .w-20{width:5rem}
  .h-full{height:100%}
  .h-auto{height:auto}
  .h-12{height:3rem}
  .h-20{height:5rem}
  .max-w-sm{max-width:24rem}
  .max-w-md{max-width:28rem}
  .max-w-lg{max-width:32rem}
  .max-h-96{max-height:24rem}
  .min-w-48{min-width:12rem}

  /* Spacing */
  .p-0{padding:0px}
  .p-2{padding:0.5rem}
  .p-3{padding:0.75rem}
  .p-4{padding:1rem}
  .p-6{padding:1.5rem}
  .p-8{padding:2rem}
  .px-3{padding-left:0.75rem;padding-right:0.75rem}
  .px-4{padding-left:1rem;padding-right:1rem}
  .py-2{padding-top:0.5rem;padding-bottom:0.5rem}
  .py-3{padding-top:0.75rem;padding-bottom:0.75rem}
  .m-0{margin:0px}
  .mx-auto{margin-left:auto;margin-right:auto}
  .mb-4{margin-bottom:1rem}
  .mt-6{margin-top:1.5rem}

  /* Colors */
  .bg-white{background-color:rgb(255 255 255)}
  .bg-gray-50{background-color:rgb(249 250 251)}
  .bg-gray-100{background-color:rgb(243 244 246)}
  .bg-gray-800{background-color:rgb(31 41 55)}
  .bg-gray-900{background-color:rgb(17 24 39)}
  .bg-blue-600{background-color:rgb(37 99 235)}
  .bg-blue-700{background-color:rgb(29 78 216)}
  .text-white{color:rgb(255 255 255)}
  .text-gray-600{color:rgb(75 85 99)}
  .text-gray-700{color:rgb(55 65 81)}
  .text-gray-900{color:rgb(17 24 39)}
  .text-blue-600{color:rgb(37 99 235)}

  /* Opacity variants */
  .bg-white.bg-opacity-90{background-color:rgba(255, 255, 255, 0.9)}
  .bg-white.bg-opacity-100{background-color:rgba(255, 255, 255, 1.0)}
  .bg-opacity-10{--tw-bg-opacity:0.1}
  .bg-opacity-15{--tw-bg-opacity:0.15}
  .border-opacity-30{--tw-border-opacity:0.3}
  
  /* Hover states for opacity */
  .hover\\:bg-opacity-100:hover.bg-white{background-color:rgba(255, 255, 255, 1.0)}
  .hover\\:bg-opacity-15:hover{--tw-bg-opacity:0.15}
  .hover\\:border-opacity-30:hover{--tw-border-opacity:0.3}

  /* Width & Height extended */
  .w-5{width:1.25rem}
  .h-5{height:1.25rem}
  .w-3{width:0.75rem}
  .h-3{height:0.75rem}
  .min-w-45{min-width:11.25rem}

  /* Z-index */
  .z-1000{z-index:1000}

  /* Visibility */
  .visible{visibility:visible}
  .invisible{visibility:hidden}
  .opacity-0{opacity:0}
  .opacity-100{opacity:1}

  /* Transform */
  .transform{transform:var(--tw-transform)}
  .rotate-180{--tw-rotate:180deg;transform:translate(var(--tw-translate-x),var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))}
  .-translate-y-2{--tw-translate-y:-0.5rem;transform:translate(var(--tw-translate-x),var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))}

  /* List styles */
  .list-none{list-style-type:none}
  
  /* Additional spacing */
  .px-2{padding-left:0.5rem;padding-right:0.5rem}
  .py-1{padding-top:0.25rem;padding-bottom:0.25rem}
  .mb-1{margin-bottom:0.25rem}

  /* Extended colors */
  .bg-gray-200{background-color:rgb(229 231 235)}
  .bg-blue-50{background-color:rgb(239 246 255)}
  .bg-blue-800{background-color:rgb(30 64 175)}
  .text-gray-500{color:rgb(107 114 128)}
  .text-blue-500{color:rgb(59 130 246)}

  /* Font weight */
  .font-normal{font-weight:400}

  /* Borders */
  .border-b{border-bottom-width:1px}

  /* Layout spacing */
  .space-y-1 > * + *{margin-top:0.25rem}

  /* Additional utilities needed */
  .top-full{top:100%}
  .mt-2{margin-top:0.5rem}
  .overflow-y-auto{overflow-y:auto}
  .text-left{text-align:left}
  .border-none{border-style:none}
  .cursor-default{cursor:default}
  .h-px{height:1px}
  .my-1{margin-top:0.25rem;margin-bottom:0.25rem}
  
  /* Text opacity */
  .text-opacity-90{--tw-text-opacity:0.9}
  
  /* Transform needed for dropdown */
  .translate-y-0{--tw-translate-y:0px;transform:translate(var(--tw-translate-x),var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))}

  /* Fill */
  .fill-current{fill:currentColor}

  /* Extended utilities for menu items */
  .text-5xl{font-size:3rem;line-height:1}
  .rounded-xl{border-radius:0.75rem}
  .p-8{padding:2rem}
  .mb-4{margin-bottom:1rem}
  .mb-2{margin-bottom:0.5rem}
  .z-10{z-index:10}
  .inset-0{top:0px;right:0px;bottom:0px;left:0px}
  .bg-gradient-to-br{background-image:linear-gradient(to bottom right, var(--tw-gradient-stops))}
  .from-blue-600{--tw-gradient-from:#2563eb;--tw-gradient-to:rgb(37 99 235 / 0);--tw-gradient-stops:var(--tw-gradient-from), var(--tw-gradient-to)}
  .to-indigo-600{--tw-gradient-to:#4f46e5}
  .opacity-5{opacity:0.05}
  .transition-opacity{transition-property:opacity;transition-timing-function:cubic-bezier(0.4, 0, 0.2, 1);transition-duration:150ms}
  .duration-300{transition-duration:300ms}
  .-translate-y-1{--tw-translate-y:-0.25rem;transform:translate(var(--tw-translate-x),var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))}
  .shadow-2xl{box-shadow:0 25px 50px -12px rgb(0 0 0 / 0.25)}
  .shadow-xl{box-shadow:0 20px 25px -5px rgb(0 0 0 / 0.1), 0 10px 10px -5px rgb(0 0 0 / 0.04)}
  .leading-snug{line-height:1.375}
  .tabindex{tabindex:0}
  
  /* Dark mode variants */
  .dark\\:bg-gray-600{background-color:rgb(75 85 99)}
  .dark\\:border-none{border-style:none}
  .dark\\:text-gray-50{color:rgb(249 250 251)}
  .dark\\:text-gray-200{color:rgb(229 231 235)}
  .dark\\:shadow-black\\/40{box-shadow:0 20px 25px -5px rgba(0, 0, 0, 0.4), 0 10px 10px -5px rgba(0, 0, 0, 0.4)}
  
  /* Hover states */
  .hover\\:-translate-y-1:hover{--tw-translate-y:-0.25rem;transform:translate(var(--tw-translate-x),var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y))}
  .hover\\:shadow-2xl:hover{box-shadow:0 25px 50px -12px rgb(0 0 0 / 0.25)}
  .hover\\:border-blue-600:hover{border-color:rgb(37 99 235)}
  .hover\\:opacity-5:hover{opacity:0.05}
  
  /* Dark mode hover states */
  .dark\\:hover\\:shadow-black\\/60:hover{box-shadow:0 25px 50px -12px rgba(0, 0, 0, 0.6)}
  .dark\\:hover\\:border-blue-300:hover{border-color:rgb(147 197 253)}
  
  /* Focus states */
  .focus\\:border-blue-600:focus{border-color:rgb(37 99 235)}
  .focus\\:shadow-blue-100:focus{box-shadow:0 0 0 3px rgb(219 234 254 / 0.5)}
  .dark\\:focus\\:border-blue-300:focus{border-color:rgb(147 197 253)}

  /* Borders */
  .border{border-width:1px}
  .border-2{border-width:2px}
  .border-gray-200{border-color:rgb(229 231 235)}
  .border-gray-300{border-color:rgb(209 213 219)}
  .border-gray-600{border-color:rgb(75 85 99)}
  .border-transparent{border-color:transparent}
  .rounded{border-radius:0.25rem}
  .rounded-md{border-radius:0.375rem}
  .rounded-lg{border-radius:0.5rem}
  .rounded-xl{border-radius:0.75rem}

  /* Shadows */
  .shadow{box-shadow:0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)}
  .shadow-md{box-shadow:0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)}
  .shadow-lg{box-shadow:0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)}

  /* Typography */
  .text-xs{font-size:0.75rem;line-height:1rem}
  .text-sm{font-size:0.875rem;line-height:1.25rem}
  .text-base{font-size:1rem;line-height:1.5rem}
  .text-lg{font-size:1.125rem;line-height:1.75rem}
  .text-xl{font-size:1.25rem;line-height:1.75rem}
  .font-medium{font-weight:500}
  .font-semibold{font-weight:600}
  .font-bold{font-weight:700}
  .text-center{text-align:center}
  .leading-none{line-height:1}

  /* Transitions */
  .transition-all{transition-property:all;transition-timing-function:cubic-bezier(0.4, 0, 0.2, 1);transition-duration:150ms}
  .transition-colors{transition-property:color, background-color, border-color, text-decoration-color, fill, stroke;transition-timing-function:cubic-bezier(0.4, 0, 0.2, 1);transition-duration:150ms}
  .duration-200{transition-duration:200ms}
  .ease-in-out{transition-timing-function:cubic-bezier(0.4, 0, 0.2, 1)}

  /* Interactive states */
  .cursor-pointer{cursor:pointer}
  .select-none{user-select:none}

  /* Hover states */
  .hover\\:bg-gray-100:hover{background-color:rgb(243 244 246)}
  .hover\\:bg-blue-700:hover{background-color:rgb(29 78 216)}
  .hover\\:text-blue-700:hover{color:rgb(29 78 216)}
  .hover\\:shadow-md:hover{box-shadow:0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)}

  /* Focus states */
  .focus\\:outline-none:focus{outline:2px solid transparent;outline-offset:2px}
  .focus\\:ring-2:focus{--tw-ring-offset-shadow:var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color);--tw-ring-shadow:var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);box-shadow:var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000)}
  .focus\\:ring-blue-500:focus{--tw-ring-opacity:1;--tw-ring-color:rgb(59 130 246 / var(--tw-ring-opacity))}

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .dark\\:bg-gray-800{background-color:rgb(31 41 55)}
    .dark\\:bg-gray-900{background-color:rgb(17 24 39)}
    .dark\\:text-white{color:rgb(255 255 255)}
    .dark\\:text-gray-300{color:rgb(209 213 219)}
    .dark\\:border-gray-600{border-color:rgb(75 85 99)}
  }

  /* Class-based dark mode */
  .dark .dark\\:bg-gray-800{background-color:rgb(31 41 55)}
  .dark .dark\\:bg-gray-900{background-color:rgb(17 24 39)}
  .dark .dark\\:text-white{color:rgb(255 255 255)}
  .dark .dark\\:text-gray-300{color:rgb(209 213 219)}
  .dark .dark\\:border-gray-600{border-color:rgb(75 85 99)}
`);

// Create shared styles that include Tailwind utilities
export const sharedStyles = [
  tailwindUtilities,
  
  // Common component styles
  css`
    :host {
      display: block;
    }
    
    /* Common focus management */
    button:focus {
      outline: none;
    }
  `
];

export default sharedStyles;