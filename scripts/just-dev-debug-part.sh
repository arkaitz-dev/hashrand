echo "Starting complete development environment with DEBUG logging (app-only)..."

# Load and export environment variables from .env
echo "Loading environment variables from .env..."
export $(cat .env | grep -v '^#' | xargs)

# Set DEBUG logging level for frontend
export VITE_LOG_LEVEL=debug
echo "Setting rust_log=hashrand=debug,info (app-only debug for backend)"

# Start spin-cli watch in background with rust_log override (first - API backend)
echo "Starting spin-cli watch in background with app-only DEBUG logging..."
nohup spin-cli watch --runtime-config-file runtime-config.toml -f spin-dev.toml --variable rust_log="hashrand=debug,info" > .spin-dev.log 2>&1 &
SPIN_PID=$!
echo $SPIN_PID > .spin-dev.pid

# Start npm dev server in background (second - web interface)
echo "Starting npm run dev in background..."
cd web
nohup npm run dev > ../.npm-dev.log 2>&1 &
NPM_PID=$!
echo $NPM_PID > ../.npm-dev.pid
cd ..

# Wait and verify both services started
sleep 3

SPIN_SUCCESS=false
NPM_SUCCESS=false

if kill -0 $SPIN_PID 2>/dev/null; then
    echo "✓ Spin dev server started in background (PID: $SPIN_PID)"
    SPIN_SUCCESS=true
else
    echo "✗ Failed to start spin dev server"
    rm -f .spin-dev.pid
fi

if kill -0 $NPM_PID 2>/dev/null; then
    echo "✓ NPM dev server started in background (PID: $NPM_PID)"
    NPM_SUCCESS=true
else
    echo "✗ Failed to start npm dev server"
    rm -f .npm-dev.pid
fi

if [ "$NPM_SUCCESS" = true ] || [ "$SPIN_SUCCESS" = true ]; then
    # Cleanup: Ensure dry-run mode is OFF (recovery from abruptly failed tests)
    # This guarantees normal email operation in development
    if [ "$SPIN_SUCCESS" = true ]; then
        echo ""
        echo "Ensuring email dry-run mode is OFF..."
        sleep 1  # Give server time to be fully ready
        curl -s http://localhost:3000/api/test/dry-run?enabled=false > /dev/null 2>&1 && \
            echo "✓ Email mode: NORMAL (dry-run OFF)" || \
            echo "• Could not verify dry-run state (server may still be starting)"
    fi

    echo ""
    echo "🚀 Development environment ready (DEBUG MODE)!"
    echo "=============================================="
    [ "$SPIN_SUCCESS" = true ] && echo "  API: http://localhost:3000 (rust_log=hashrand=debug,info)"
    [ "$NPM_SUCCESS" = true ] && echo "  Web: http://localhost:5173 (VITE_LOG_LEVEL=debug)"
    echo ""
    echo "Management commands:"
    echo "  Logs: just la (API) / just lw (Web) / just w (both)"
    echo "  Stop: just stop"
    echo "  Status: just status"
    echo ""
    echo "⚠️  DEBUG logging active for hashrand (no Spin/Wasmtime noise)"

    # Start Tailscale serve for frontend if npm is running
    if [ "$NPM_SUCCESS" = true ]; then
        echo ""
        echo "Starting Tailscale serve for remote access..."
        just tailscale-front-start
    fi
else
    echo "✗ Failed to start development servers"
    exit 1
fi
