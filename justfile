# HashRand Spin - Development Tasks
# Run `just` to see available commands
#
# IMPORTANT: just has native .env file loading capability
# - Automatically loads .env from current directory (can be disabled with --no-dotenv)
# - Variables from .env are available to all recipes without explicit sourcing
# - Spin reads SPIN_VARIABLE_* variables directly from the loaded environment

# Show available commands
default:
    @just --list

# Build both WebAssembly component and web interface
build:
    #!/usr/bin/env bash
    echo "Building WebAssembly component..."
    spin-cli build
    echo "Building web interface..."
    cd web && npx svelte-kit sync && npm run build

# Start the application locally
up:
    spin-cli up --runtime-config-file runtime-config.toml -f spin-dev.toml

# Stop any running development servers (foreground and background)
stop: tailscale-stop
    #!/usr/bin/env bash
    echo "Stopping development servers..."
    
    # Kill background server if PID file exists
    if [ -f .spin-dev.pid ]; then
        PID=$(cat .spin-dev.pid)
        if kill -0 $PID 2>/dev/null; then
            kill $PID 2>/dev/null && echo "✓ Stopped background server (PID: $PID)" || echo "• Failed to stop background server"
        fi
        rm -f .spin-dev.pid
    fi
    
    # Kill predeploy server if PID file exists
    if [ -f .spin-predeploy.pid ]; then
        PID=$(cat .spin-predeploy.pid)
        if kill -0 $PID 2>/dev/null; then
            kill $PID 2>/dev/null && echo "✓ Stopped predeploy server (PID: $PID)" || echo "• Failed to stop predeploy server"
        fi
        rm -f .spin-predeploy.pid
    fi
    
    # Kill background npm server if PID file exists
    if [ -f .npm-dev.pid ]; then
        PID=$(cat .npm-dev.pid)
        if kill -0 $PID 2>/dev/null; then
            kill $PID 2>/dev/null && echo "✓ Stopped npm background server (PID: $PID)" || echo "• Failed to stop npm background server"
        fi
        rm -f .npm-dev.pid
    fi
    
    # Kill any running spin-cli watch processes
    pkill -f "spin-cli watch" 2>/dev/null && echo "✓ Stopped spin-cli watch" || echo "• No spin-cli watch process found"
    # Kill any processes using port 3000
    lsof -ti:3000 | xargs kill -9 2>/dev/null && echo "✓ Freed port 3000" || echo "• Port 3000 was free"
    # Kill any spin-cli processes
    pkill -f "spin-cli" 2>/dev/null && echo "✓ Stopped other spin-cli processes" || echo "• No other spin-cli processes found"
    
    # Kill any npm dev processes and free port 5173
    pkill -f "npm run dev" 2>/dev/null && echo "✓ Stopped npm run dev" || echo "• No npm run dev process found"
    lsof -ti:5173 | xargs kill -9 2>/dev/null && echo "✓ Freed port 5173" || echo "• Port 5173 was free"
    
    # Clean up log files
    [ -f .spin-dev.log ] && rm -f .spin-dev.log && echo "✓ Cleaned up spin log file"
    [ -f .npm-dev.log ] && rm -f .npm-dev.log && echo "✓ Cleaned up npm log file"
    [ -f .spin-predeploy.log ] && rm -f .spin-predeploy.log && echo "✓ Cleaned up predeploy log file"
    
    echo "Development servers stopped."

# Check status of development servers (shows background processes and port usage)
status:
    #!/usr/bin/env bash
    echo "Development Server Status:"
    echo "=========================="
    
    # Check background server status
    if [ -f .spin-dev.pid ]; then
        PID=$(cat .spin-dev.pid)
        if kill -0 $PID 2>/dev/null; then
            echo "✓ Background server running (PID: $PID)"
            if [ -f .spin-dev.log ]; then
                echo "  Log file: .spin-dev.log ($(wc -l < .spin-dev.log) lines)"
            fi
        else
            echo "✗ PID file exists but process is dead (cleaning up...)"
            rm -f .spin-dev.pid
        fi
    else
        echo "• No background server PID file"
    fi
    
    # Check npm background server status
    if [ -f .npm-dev.pid ]; then
        PID=$(cat .npm-dev.pid)
        if kill -0 $PID 2>/dev/null; then
            echo "✓ NPM background server running (PID: $PID)"
            if [ -f .npm-dev.log ]; then
                echo "  Log file: .npm-dev.log ($(wc -l < .npm-dev.log) lines)"
            fi
        else
            echo "✗ NPM PID file exists but process is dead (cleaning up...)"
            rm -f .npm-dev.pid
        fi
    else
        echo "• No npm background server PID file"
    fi
    
    # Check predeploy server status
    if [ -f .spin-predeploy.pid ]; then
        PID=$(cat .spin-predeploy.pid)
        if kill -0 $PID 2>/dev/null; then
            echo "✓ Predeploy server running (PID: $PID)"
            if [ -f .spin-predeploy.log ]; then
                echo "  Log file: .spin-predeploy.log ($(wc -l < .spin-predeploy.log) lines)"
            fi
        else
            echo "✗ Predeploy PID file exists but process is dead (cleaning up...)"
            rm -f .spin-predeploy.pid
        fi
    else
        echo "• No predeploy server PID file"
    fi
    
    # Check if spin-cli watch is running
    if pgrep -f "spin-cli watch" > /dev/null; then
        echo "✓ spin-cli watch process(es) running (PID(s): $(pgrep -f 'spin-cli watch' | tr '\n' ' '))"
    else
        echo "✗ No spin-cli watch processes running"
    fi
    
    # Check if npm run dev is running
    if pgrep -f "npm run dev" > /dev/null; then
        echo "✓ npm run dev process(es) running (PID(s): $(pgrep -f 'npm run dev' | tr '\n' ' '))"
    else
        echo "✗ No npm run dev processes running"
    fi
    
    # Check if port 3000 is in use
    if lsof -ti:3000 > /dev/null 2>&1; then
        echo "✓ Port 3000 is in use (PID: $(lsof -ti:3000))"
        echo "  API should be available at: http://localhost:3000"
    else
        echo "✗ Port 3000 is free"
    fi
    
    # Check if port 5173 is in use
    if lsof -ti:5173 > /dev/null 2>&1; then
        echo "✓ Port 5173 is in use (PID: $(lsof -ti:5173))"
        echo "  Web interface should be available at: http://localhost:5173"
    else
        echo "✗ Port 5173 is free"
    fi
    
    # Check if any spin-cli processes are running
    SPIN_PROCESSES=$(pgrep -f "spin-cli" | wc -l)
    if [ $SPIN_PROCESSES -gt 0 ]; then
        echo "✓ Total $SPIN_PROCESSES spin-cli process(es) running"
    else
        echo "✗ No spin-cli processes running"
    fi
    
    echo ""
    echo "Tailscale Status:"
    echo "================"
    # Check if tailscale is available
    if ! command -v tailscale &> /dev/null; then
        echo "✗ Tailscale CLI not installed"
    else
        # Check tailscale serve status
        TAILSCALE_STATUS=$(tailscale serve status 2>/dev/null)
        if [ -n "$TAILSCALE_STATUS" ]; then
            echo "✓ Tailscale serve is active:"
            echo "$TAILSCALE_STATUS" | sed 's/^/  /'
        else
            echo "• Tailscale serve is not active"
            echo "  Start frontend: just tailscale-front-start"
            echo "  Start backend: just tailscale-back-start"
        fi
    fi

# Start development servers in foreground mode (npm in bg, spin in fg)
dev-fg: stop
    #!/usr/bin/env bash
    echo "Starting development servers..."
    
    # Start npm dev server in background
    echo "Starting npm run dev in background..."
    cd web
    nohup npm run dev > ../.npm-dev.log 2>&1 &
    NPM_PID=$!
    echo $NPM_PID > ../.npm-dev.pid
    cd ..
    
    # Wait a moment and check if npm dev started successfully
    sleep 2
    if kill -0 $NPM_PID 2>/dev/null; then
        echo "✓ NPM dev server started (PID: $NPM_PID, Port: 5173)"
    else
        echo "✗ Failed to start npm dev server"
        rm -f .npm-dev.pid
    fi
    
    # Start spin-cli watch (foreground)
    echo "Starting spin-cli watch (foreground)..."
    echo "  API: http://localhost:3000"
    echo "  Web: http://localhost:5173"
    echo "  Press Ctrl+C to stop"
    spin-cli watch -f spin-dev.toml

# Start complete development environment (both servers in background)
dev: stop
    #!/usr/bin/env bash
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

# Watch mode: start dev server in background and follow logs (Ctrl+C stops watching only)
watch: dev
    echo "Following development server logs (Ctrl+C to stop watching)..."
    echo "Logs from both Spin (.spin-dev.log) and NPM (.npm-dev.log):"
    echo "============================================================="
    tail -f .spin-dev.log .npm-dev.log

# Run comprehensive test suite
test:
    ./scripts/final_test.sh

# Run linting checks
lint:
    cd api && cargo clippy -- -D warnings
    cd web && npm run lint

# Format code
fmt:
    cd api && cargo fmt
    cd web && npm run format

# Check code quality (lint + format check)
check:
    cd api && cargo clippy -- -D warnings
    cd api && cargo fmt --check
    cd web && npm run lint
    cd web && npm run check

# Update all dependencies
update:
    cd api && cargo update

# Add a new dependency (usage: just add <crate-name>)
add crate:
    cd api && cargo add {{crate}}

# Clean build artifacts
clean:
    #!/usr/bin/env bash
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

# Clean and rebuild everything
clean-build: clean build

# Rebuild everything from scratch
rebuild: clean build

# Deploy to Fermyon Cloud with secrets
deploy:
    #!/usr/bin/env bash
    # Read secrets from .env file and deploy to Fermyon Cloud
    source .env
    spin-cli deploy --runtime-config-file runtime-config.toml -f spin-prod.toml \
        --variable jwt_secret="${JWT_SECRET:-${SPIN_VARIABLE_JWT_SECRET}}" \
        --variable magic_link_hmac_key="${MAGIC_LINK_HMAC_KEY:-${SPIN_VARIABLE_MAGIC_LINK_HMAC_KEY}}" \
        --variable argon2_salt="${ARGON2_SALT:-${SPIN_VARIABLE_ARGON2_SALT}}"

# Prepare for production deployment (compile web UI, start backend only with static serving, start tailscale)
predeploy: stop clean
    #!/usr/bin/env bash
    echo "🚀 Preparing for production deployment..."
    echo "======================================="
    
    # Build web interface for production
    echo "Building web interface for production..."
    cd web
    npm run build
    cd ..
    
    # Verify dist directory was created
    if [ ! -d "web/dist" ]; then
        echo "✗ Error: web/dist directory not found after build"
        exit 1
    fi
    
    echo "✓ Web interface built successfully ($(du -sh web/dist | cut -f1) in web/dist/)"
    
    # Build WebAssembly component
    echo "Building WebAssembly backend component..."
    spin-cli build
    
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
            echo "✓ Backend API responding on port 3000"
        else
            echo "⚠ Backend started but API not yet responding (may need more time)"
        fi
        
        # Start Tailscale serve for external access
        echo "Starting Tailscale serve for external access..."
        if command -v tailscale &> /dev/null; then
            tailscale serve --bg 3000
            echo "✓ Tailscale serve started on port 3000"
            echo ""
            echo "🎉 Production deployment ready!"
            echo "==============================="
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
            echo "⚠ Tailscale not available, skipping external access setup"
            echo ""
            echo "🎉 Local production deployment ready!"
            echo "====================================="
            echo "  Access: http://localhost:3000"
            echo "  Log: tail -f .spin-predeploy.log"
            echo "  Stop: just stop"
        fi
    else
        echo "✗ Failed to start backend"
        rm -f .spin-predeploy.pid
        exit 1
    fi

# Run development server in background and execute tests
test-dev:
    #!/usr/bin/env bash
    # Stop any existing servers first
    just stop > /dev/null 2>&1
    echo "Starting development server in background..."
    spin-cli watch --runtime-config-file runtime-config.toml -f spin-dev.toml > /dev/null 2>&1 &
    SPIN_PID=$!
    sleep 3
    echo "Running tests..."
    ./scripts/final_test.sh
    echo "Stopping development server..."
    kill $SPIN_PID 2>/dev/null || true
    just stop > /dev/null 2>&1

# Show project information
info:
    @echo "HashRand Spin - Random Hash Generator API"
    @echo "========================================="
    @echo "Version: $(grep '^version' api/Cargo.toml | cut -d'"' -f2)"
    @echo "Rust Edition: $(grep '^edition' api/Cargo.toml | cut -d'"' -f2)"
    @echo "Rust Version: $(grep '^rust-version' api/Cargo.toml | cut -d'"' -f2)"
    @echo ""
    @echo "Endpoints:"
    @echo "  GET /api/generate  - Customizable hash generation"
    @echo "  GET /api/password  - Secure password generation"
    @echo "  GET /api/api-key   - API key generation"
    @echo "  GET /api/version   - Version information"
    @echo ""
    @echo "Development URL: http://localhost:3000"

# Show dependency information
deps:
    @echo "Dependencies:"
    @cd api && cargo tree --depth 1

# Generate documentation
docs:
    cd api && cargo doc --open

# Check if spin-cli is available
check-spin:
    @which spin-cli > /dev/null || (echo "Error: spin-cli not found. Please install Fermyon Spin CLI." && exit 1)
    @echo "✓ spin-cli is available: $(spin-cli --version)"

# Full development setup check
setup-check: check-spin
    @echo "Checking Rust installation..."
    @rustc --version
    @cargo --version
    @echo "✓ Development environment ready"

# Run all quality checks before commit
pre-commit: check test
    @echo "✅ All quality checks passed!"

# Show logs from the last run
logs:
    @echo "Recent logs from .spin/logs/:"
    @find .spin/logs -name "*.log" -exec tail -20 {} \; 2>/dev/null || echo "No logs found. Run 'just dev' first."

# Performance test (rapid requests)
perf-test:
    #!/usr/bin/env bash
    echo "Running performance test..."
    echo "Testing 100 rapid requests to /api/generate..."
    time for i in {1..100}; do
        curl -s http://localhost:3000/api/generate > /dev/null
    done
    echo "Performance test completed."

# Show API examples
examples:
    @echo "API Usage Examples:"
    @echo "==================="
    @echo ""
    @echo "# Generate default hash (21 chars, base58)"
    @echo "curl http://localhost:3000/api/generate"
    @echo ""
    @echo "# Generate custom hash with prefix/suffix"
    @echo "curl 'http://localhost:3000/api/generate?length=16&prefix=app_&suffix=_key'"
    @echo ""
    @echo "# Generate secure password"
    @echo "curl 'http://localhost:3000/api/password?length=32'"
    @echo ""
    @echo "# Generate API key"
    @echo "curl 'http://localhost:3000/api/api-key?length=50'"
    @echo ""
    @echo "# Get version"
    @echo "curl http://localhost:3000/api/version"

# Check if tailscale is available
check-tailscale:
    @which tailscale > /dev/null || (echo "Error: tailscale not found. Please install Tailscale CLI." && exit 1)
    @echo "✓ tailscale is available"

# Start Tailscale serve for frontend (port 5173)
tailscale-front-start: check-tailscale
    #!/usr/bin/env bash
    echo "Starting Tailscale serve for frontend (port 5173)..."
    tailscale serve --bg 5173
    echo "✓ Tailscale serve started for frontend"
    echo "  Frontend now accessible via Tailscale network"
    echo "  Check status: just status"

# Stop Tailscale serve for frontend
tailscale-front-stop:
    #!/usr/bin/env bash
    echo "Stopping Tailscale serve..."
    tailscale serve --https=443 off
    echo "✓ Tailscale serve stopped"

# Stop all Tailscale serve instances
tailscale-stop:
    #!/usr/bin/env bash
    if command -v tailscale &> /dev/null; then
        echo "Stopping all Tailscale serve instances..."
        tailscale serve --https=443 off 2>/dev/null || echo "• No Tailscale serve instances to stop"
        # Also try stopping specific port configurations
        tailscale serve --bg 3000 off 2>/dev/null || true
        tailscale serve --bg 5173 off 2>/dev/null || true
        echo "✓ All Tailscale serve instances stopped"
    else
        echo "• Tailscale CLI not available, skipping"
    fi

# Start Tailscale serve for backend API (port 3000)
tailscale-back-start: check-tailscale
    #!/usr/bin/env bash
    echo "Starting Tailscale serve for backend API (port 3000)..."
    tailscale serve --bg 3000
    echo "✓ Tailscale serve started for backend API"
    echo "  API now accessible via Tailscale network"
    echo "  Check status: just status"

# Stop Tailscale serve for backend (same as frontend stop)
tailscale-back-stop: tailscale-front-stop

# Development workflow shortcuts
alias d := dev
alias t := test
alias b := build
alias c := check
alias f := fmt
alias l := lint