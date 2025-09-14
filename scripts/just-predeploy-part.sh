echo "🚀 Preparing for production deployment..."
echo "======================================="

# Build web interface for production
echo "Building web interface for production..."
cd web && npm run build && cd ..

# Verify dist directory was created
if [ ! -d "web/dist" ]; then
echo "Error: web/dist directory not found after build"
exit 1
fi

echo "✓ Web interface built successfully ($(du -sh web/dist | cut -f1) in web/dist/)"

# Build WebAssembly component
echo "Building WebAssembly backend component..."
spin-cli build -f spin-prod.toml

# Start backend only (includes static file serving)
echo "Starting backend with static file serving..."
nohup spin-cli up --runtime-config-file runtime-config.toml -f spin-prod.toml > .spin-predeploy.log 2>&1 &
SPIN_PID=$!
echo $SPIN_PID > .spin-predeploy.pid

# Wait for backend to start
sleep 3

if kill -0 $SPIN_PID 2>/dev/null; then
echo "✓ Backend started (PID: $SPIN_PID)"

# Check if port 3000 is responding
if curl -s http://localhost:3000/api/version > /dev/null; then
echo "Backend API responding on port 3000"
else
echo "Backend started but API not yet responding (may need more time)"
fi

# Start Tailscale serve for external access
echo "Starting Tailscale serve for external access..."
if command -v tailscale &> /dev/null; then
tailscale serve --bg 3000
echo "Tailscale serve started on port 3000"
echo ""
echo "Production deployment ready!"
echo "=============================="
echo "  Local access: http://localhost:3000"
echo "  Tailscale access: Check 'tailscale serve status'"
echo "  Backend log: tail -f .spin-predeploy.log"
echo "  Stop: just stop"
echo "  Status: just status"
echo ""
echo "Note: Both web interface and API are served from port 3000"
echo "      Web files: Served statically from /web/dist/"
echo "      API endpoints: Available at /api/*"
else
echo "Tailscale not available, skipping external access setup"
echo ""
echo "Local production deployment ready!"
echo "===================================="
echo "  Access: http://localhost:3000"
echo "  Log: tail -f .spin-predeploy.log"
echo "  Stop: just stop"
fi
else
echo "Failed to start backend"
rm -f .spin-predeploy.pid
exit 1
fi