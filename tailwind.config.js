/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class', // Enable dark mode via class
  content: [
    './web-ui/index.html',
    './web-ui/src/**/*.js',
  ],
  theme: {
    extend: {
      animation: {
        fadeIn: 'fadeIn 0.3s ease-in-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0', transform: 'translateY(10px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
      },
    },
  },
  plugins: [],
  // Production optimizations
  corePlugins: {
    // Disable unused utilities to reduce CSS size
    preflight: true,
    container: false,
    accessibility: false,
    backgroundAttachment: false,
    backgroundClip: false,
    backgroundImage: true,
    backgroundOpacity: true,
    backgroundPosition: false,
    backgroundRepeat: false,
    backgroundSize: false,
    blur: false,
    brightness: false,
    contrast: false,
    dropShadow: false,
    filter: false,
    grayscale: false,
    hueRotate: false,
    invert: false,
    saturate: false,
    sepia: false,
    backdropBlur: true,
    backdropBrightness: false,
    backdropContrast: false,
    backdropFilter: false,
    backdropGrayscale: false,
    backdropHueRotate: false,
    backdropInvert: false,
    backdropOpacity: false,
    backdropSaturate: false,
    backdropSepia: false,
    divideColor: false,
    divideOpacity: false,
    divideStyle: false,
    divideWidth: false,
    ringColor: true,
    ringOffsetColor: true,
    ringOffsetWidth: true,
    ringOpacity: true,
    ringWidth: true,
  }
}