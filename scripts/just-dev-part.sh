echo "Starting complete development environment..."

# Start spin-cli watch in background (first - API backend)
echo "Starting spin-cli watch in background..."
nohup spin-cli watch --runtime-config-file runtime-config.toml -f spin-dev.toml > .spin-dev.log 2>&1 &
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
    echo ""
    echo "🚀 Development environment ready!"
    echo "================================="
    [ "$SPIN_SUCCESS" = true ] && echo "  API: http://localhost:3000"
    [ "$NPM_SUCCESS" = true ] && echo "  Web: http://localhost:5173"
    echo ""
    echo "Management commands:"
    echo "  Logs: tail -f .spin-dev.log .npm-dev.log"
    echo "  Stop: just stop"
    echo "  Status: just status"

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