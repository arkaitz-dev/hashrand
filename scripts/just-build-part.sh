echo "Building WebAssembly component..."
spin-cli build -f spin-dev.toml
echo "Building web interface..."
cd web && npx svelte-kit sync && npm run build