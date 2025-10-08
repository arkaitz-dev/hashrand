# HashRand - Development Tasks
# Run `just` to see available commands

# Environment file configuration
set dotenv-load := true

# IMPORTANT: just has native .env file loading capability
# - Now configured to load .env by default for development
# - Variables are available to all recipes without explicit sourcing
# - Spin reads SPIN_VARIABLE_* variables directly from the loaded environment

# Show available commands
default:
    @just --list

# Build both WebAssembly component and web interface
build:
    #!/usr/bin/env bash
    source scripts/just-build-part.sh

# Start the application locally
up:
    spin-cli up --runtime-config-file runtime-config.toml -f spin-dev.toml

# Stop any running development servers (foreground and background)
stop: tailscale-stop
    #!/usr/bin/env bash
    source scripts/just-stop-part.sh

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
    source scripts/just-dev-part.sh

# Start complete development environment with DEBUG logging (RUST_LOG=debug)
dev-debug: stop
    #!/usr/bin/env bash
    source scripts/just-dev-debug-part.sh

# Watch mode: start dev server in background and follow logs (Ctrl+C stops watching only)
watch: dev
    echo "Following development server logs (Ctrl+C to stop watching)..."
    echo "Logs from both Spin (.spin-dev.log) and NPM (.npm-dev.log):"
    echo "============================================================="
    tail -f .spin-dev.log .npm-dev.log

# Run bash integration tests only (35 tests)
# Automatically activates/deactivates email dry-run mode
test-bash:
    #!/usr/bin/env bash
    echo "Running bash integration tests (35 tests)..."
    echo "Note: Email dry-run mode managed automatically"
    echo ""
    ./scripts/final_test.sh

# Run Playwright API tests only (16 tests)
# Automatically activates/deactivates email dry-run mode via globalSetup/Teardown
test-api:
    #!/usr/bin/env bash
    echo "Running Playwright API tests (16 tests)..."
    echo "Note: Email dry-run mode managed automatically via globalSetup/Teardown"
    echo ""
    cd web && npm run test:api

# Run comprehensive test suite with INFO logging (default)
# Restarts server with RUST_LOG=info before running tests
test:
    just _run-tests info

# Run comprehensive test suite with DEBUG logging (verbose)
# Restarts server with RUST_LOG=debug before running tests
test-debug:
    just _run-tests debug

# Internal: Run complete test suite with specified log level
# Manages server lifecycle: stop → start with log level → test → stop
_run-tests log_level:
    #!/usr/bin/env bash
    echo "=========================================="
    echo "Running comprehensive test suite"
    echo "Log level: RUST_LOG={{log_level}}"
    echo "=========================================="
    echo ""

    # Stop any existing servers
    echo "Stopping existing servers..."
    just stop > /dev/null 2>&1

    # Start server with specified log level
    echo "Starting test server with RUST_LOG={{log_level}}..."
    bash scripts/start-server-for-tests.sh {{log_level}}

    # Run bash tests
    echo ""
    echo "Running bash integration tests (35 tests)..."
    echo "Note: Email dry-run mode managed automatically"
    echo ""
    ./scripts/final_test.sh

    # Run playwright tests
    echo ""
    echo "Running Playwright API tests (16 tests)..."
    echo "Note: Email dry-run mode managed automatically via globalSetup/Teardown"
    echo ""
    export TEST_API_ONLY=true && cd web && npm run test:api

    # Stop server
    echo ""
    echo "Stopping test server..."
    just stop > /dev/null 2>&1

    echo ""
    echo "=========================================="
    echo "✅ All tests completed!"
    echo "Log level used: RUST_LOG={{log_level}}"
    echo "=========================================="

# Run linting checks
lint:
    cd api && cargo clippy -- -D warnings
    cd web && npm run lint

# Format code (Rust + TypeScript/JavaScript/HTML)
format:
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
    source scripts/just-clean-part.sh

# Clean and rebuild everything
clean-build: clean build

# Rebuild everything from scratch
rebuild: clean build

# Deploy to Fermyon Cloud with secrets
deploy:
    set dotenv-filename := ".env-prod"
    #!/usr/bin/env bash
    source scripts/just-deploy-part.sh

# Prepare for production deployment (compile web UI, start backend only with static serving, start tailscale)
predeploy: stop clean
    set dotenv-filename := ".env-prod"
    #!/usr/bin/env bash
    source scripts/just-predeploy-part.sh

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
    @echo "HashRand - Random Hash Generator API"
    @echo "====================================="
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
alias dd := dev-debug
alias t := test
alias td := test-debug
alias b := build
alias c := check
alias f := format
alias fmt := format
alias l := lint
