# HashRand Spin Web Interface

Professional web interface for the HashRand Spin API - a modern SPA built with SvelteKit, TypeScript, and TailwindCSS.

## Features

- **Modern SPA Architecture**: Built with SvelteKit as a Single Page Application
- **Professional Design**: Clean, responsive UI with dark/light mode support
- **Multi-endpoint Support**: Interfaces for all API endpoints:
  - Custom Hash Generator (`/api/generate`) - accessible via `/custom` route
  - Secure Password Generator (`/api/password`) 
  - API Key Generator (`/api/api-key`)
  - Version Information (`/api/version`)
- **Interactive UI Components**: Enhanced user experience with modern controls
  - **Range Sliders**: Beautiful gradient sliders for length parameter selection
  - **Dynamic Help Text**: Context-aware informational notes based on alphabet selection
  - **Automatic Adjustments**: Smart minimum length calculation when changing alphabets
  - **In-Place Regeneration**: Generate new hashes without navigating back to configuration
- **Visual Feedback**: Professional loading states and animations
  - **Spinning Animations**: Smooth icon rotations during hash generation
  - **Button States**: Proper color changes and disabled states during loading
  - **Consistent Sizing**: Fixed button dimensions to prevent layout shift
- **Parameter Validation**: Client-side validation for all parameters
- **Copy to Clipboard**: Easy result copying with visual feedback
- **Responsive Design**: Works on all screen sizes
- **Internationalization Ready**: Prepared for multiple language support
- **TypeScript**: Full type safety throughout the application

## Technology Stack

- **SvelteKit 2.x**: Modern web framework with SSG/SPA capabilities
- **TypeScript**: Type-safe development
- **TailwindCSS 4.x**: Utility-first CSS framework with modern features
- **Vite**: Fast build tool and development server
- **API Proxy**: Configured to work with HashRand Spin API on port 3000

## Development

### Prerequisites

- Node.js 18+ 
- HashRand Spin API running on `http://127.0.0.1:3000`

### Setup

```bash
# Install dependencies
npm install

# Start development server (runs on port 5173)
npm run dev

# Or start with host binding for network access
npm run dev -- --host
```

The web interface will be available at `http://localhost:5173` and will proxy API calls to the backend on port 3000.

### Building

```bash
# Build for production (SPA)
npm run build

# Preview production build
npm run preview
```

## API Integration

The web interface automatically proxies `/api/*` requests to the HashRand Spin backend API running on port 3000. No additional configuration needed for development.

## Project Structure

```
src/
├── app.css                 # Global styles with TailwindCSS
├── app.html                # HTML template
├── lib/
│   ├── api.ts             # API service layer
│   ├── components/        # Reusable Svelte components
│   ├── stores/            # Svelte stores for state management
│   └── types/             # TypeScript type definitions
└── routes/
    ├── +layout.svelte     # Root layout
    ├── +layout.ts         # SPA configuration
    ├── +page.svelte       # Main menu page
    ├── custom/            # Custom hash generator (renamed from generate/)
    ├── password/          # Password generator
    ├── api-key/           # API key generator
    └── result/            # Shared result display with in-place regeneration
```

## Features

### Navigation
- Clean menu-driven interface
- Streamlined navigation with consolidated controls
- Return to menu from any page
- In-place result regeneration without navigation

### Form Validation
- Real-time parameter validation
- Dynamic minimum length calculation based on alphabet
- Clear error messages and hints

### Result Display
- Formatted result presentation
- One-click copy to clipboard
- Generation metadata display
- Parameter summary

### Responsive Design
- Mobile-first approach
- Adaptive layouts for all screen sizes
- Touch-friendly interactions

### Accessibility
- ARIA labels and semantic HTML
- Keyboard navigation support
- Screen reader friendly
- High contrast support

## Deployment

The application builds as a static SPA that can be deployed to any static hosting service:

```bash
npm run build
# Deploy the 'build' directory to your hosting platform
```

## Configuration

### API Endpoint
The API endpoint is configured in `vite.config.ts`. For production, update the proxy target or configure your reverse proxy to route `/api/*` to your HashRand Spin API.

### Styling
TailwindCSS configuration can be customized in the generated config files. The application uses a professional blue/gray color scheme with automatic dark mode support.

## License

MIT