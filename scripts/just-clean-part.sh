echo "Cleaning Rust build artifacts..."
cargo clean
echo "Cleaning npm artifacts and dist directory..."
cd web
# Remove build outputs and generated files
rm -rf dist build .svelte-kit
# Remove npm/pnpm/yarn caches
rm -rf node_modules/.cache node_modules/.vite node_modules/.pnpm-state
# Remove TypeScript cache
rm -rf node_modules/.tsc node_modules/.tsbuildinfo
# Remove test coverage and temporary files
rm -rf coverage .nyc_output .tmp .temp
# Remove Vite cache and temp files
rm -rf .vite .vite-cache vite.config.js.timestamp-*
# Remove SvelteKit server and client output
rm -rf .svelte-kit/output .svelte-kit/generated .svelte-kit/types
cd ..
# Remove development log files and PID files
rm -f .spin-dev.log .npm-dev.log .spin-dev.pid .npm-dev.pid
# Remove any temporary files at project root
rm -rf .tmp .temp *.tmp *.temp
echo "✓ All build artifacts and caches cleaned"