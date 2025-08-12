# Development commands for hashrand project

# Launch npm dev server in background
dev-npm:
    #!/bin/bash
    if lsof -Pi :3000 -sTCP:LISTEN -t >/dev/null; then
        echo "Port 3000 already in use - killing existing process"
        pkill -f "vite.*--port 3000" || true
        sleep 1
    fi
    echo "Starting npm dev server on port 3000..."
    nohup npm run dev > /tmp/hashrand-npm.log 2>&1 & echo $! > /tmp/hashrand-npm.pid
    echo "npm dev server started (PID: $(cat /tmp/hashrand-npm.pid))"
    echo "Logs: tail -f /tmp/hashrand-npm.log"



# Launch cargo watch server in background
dev-cargo:
    #!/bin/bash
    if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null; then
        echo "Port 8080 already in use - killing existing process"
        pkill -f "cargo.*run.*serve" || true
        sleep 1
    fi
    echo "Starting cargo watch server on port 8080..."
    nohup cargo watch -x 'run -- --serve 8080' > /tmp/hashrand-cargo.log 2>&1 & echo $! > /tmp/hashrand-cargo.pid
    echo "cargo watch server started (PID: $(cat /tmp/hashrand-cargo.pid))"
    echo "Logs: tail -f /tmp/hashrand-cargo.log"


# Launch both development servers
dev: dev-cargo dev-npm
    #!/bin/bash
    echo "Development environment ready!"
    echo "Frontend: http://localhost:3000"
    echo "Backend:  http://localhost:8080"
    echo ""
    
    # Configure Tailscale if available
    if command -v tailscale &> /dev/null; then
        echo "Configuring Tailscale serve..."
        # First stop any existing serve configuration
        tailscale serve --https=443 off 2>/dev/null || true
        # Then start serving the frontend
        tailscale serve --bg http://localhost:3000
        # Try to get the Tailscale hostname
        if command -v jq &> /dev/null; then
            TAILSCALE_URL="https://$(tailscale status --json | jq -r '.Self.DNSName' | sed 's/\.$//')"
            echo "Tailscale: $TAILSCALE_URL"
        else
            echo "Tailscale: https://elite.faun-pirate.ts.net/"
            echo "(Note: Install jq for automatic hostname detection)"
        fi
    fi
    
    echo ""
    echo "Stop servers with: just stop-dev"

# Stop development servers
stop-dev:
    #!/bin/bash
    echo "Stopping development servers..."
    
    # Stop Tailscale serve if available
    if command -v tailscale &> /dev/null; then
        echo "Stopping Tailscale serve..."
        tailscale serve --https=443 off 2>/dev/null || true
    fi
    
    if [ -f /tmp/hashrand-npm.pid ]; then
        kill $(cat /tmp/hashrand-npm.pid) 2>/dev/null || true
        rm -f /tmp/hashrand-npm.pid
        echo "npm dev server stopped"
    fi
    if [ -f /tmp/hashrand-cargo.pid ]; then
        kill $(cat /tmp/hashrand-cargo.pid) 2>/dev/null || true
        rm -f /tmp/hashrand-cargo.pid
        echo "cargo watch server stopped"
    fi
    # Cleanup any remaining processes
    pkill -f "vite.*--port 3000" 2>/dev/null || true
    pkill -f "cargo.*run.*serve" 2>/dev/null || true
    echo "All development servers stopped"

# Show development server status
status:
    #!/bin/bash
    echo "Development server status:"
    echo ""
    if lsof -Pi :3000 -sTCP:LISTEN -t >/dev/null; then
        echo "✓ npm dev server running on port 3000"
    else
        echo "✗ npm dev server not running"
    fi
    if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null; then
        echo "✓ cargo watch server running on port 8080"
    else
        echo "✗ cargo watch server not running"
    fi

# Build for production
build:
    npm run build
    cargo build --release

# Run production server
serve:
    ./target/release/hashrand --serve 8080

# Test commands
npm-test:
    #!/bin/bash
    cd web-ui
    if npm run | grep -q "test"; then
        npm test
    else
        echo "npm test script not configured - skipping npm tests"
        echo "Frontend tests should be configured in the future"
    fi

cargo-test:
    cargo test

# Run all tests
test: npm-test cargo-test
    @echo "All tests completed!"

# Install binary locally (runs tests first)
install: test
    npm run build
    cargo install --path .