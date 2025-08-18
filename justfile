# HashRand Spin - Development Tasks
# Run `just` to see available commands

# Show available commands
default:
    @just --list

# Build the WebAssembly component
build:
    spin-cli build

# Start the application locally
up:
    spin-cli up

# Stop any running development servers (foreground and background)
stop:
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
    
    # Kill any running spin-cli watch processes
    pkill -f "spin-cli watch" 2>/dev/null && echo "✓ Stopped spin-cli watch" || echo "• No spin-cli watch process found"
    # Kill any processes using port 3000
    lsof -ti:3000 | xargs kill -9 2>/dev/null && echo "✓ Freed port 3000" || echo "• Port 3000 was free"
    # Kill any spin-cli processes
    pkill -f "spin-cli" 2>/dev/null && echo "✓ Stopped other spin-cli processes" || echo "• No other spin-cli processes found"
    
    # Clean up log file
    [ -f .spin-dev.log ] && rm -f .spin-dev.log && echo "✓ Cleaned up log file"
    
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
    
    # Check if spin-cli watch is running
    if pgrep -f "spin-cli watch" > /dev/null; then
        echo "✓ spin-cli watch process(es) running (PID(s): $(pgrep -f 'spin-cli watch' | tr '\n' ' '))"
    else
        echo "✗ No spin-cli watch processes running"
    fi
    
    # Check if port 3000 is in use
    if lsof -ti:3000 > /dev/null 2>&1; then
        echo "✓ Port 3000 is in use (PID: $(lsof -ti:3000))"
        echo "  API should be available at: http://localhost:3000"
    else
        echo "✗ Port 3000 is free"
    fi
    
    # Check if any spin-cli processes are running
    SPIN_PROCESSES=$(pgrep -f "spin-cli" | wc -l)
    if [ $SPIN_PROCESSES -gt 0 ]; then
        echo "✓ Total $SPIN_PROCESSES spin-cli process(es) running"
    else
        echo "✗ No spin-cli processes running"
    fi

# Start development server with auto-reload (stops existing servers first)
dev: stop
    echo "Starting development server..."
    spin-cli watch

# Start development server in background (persistent after terminal close)
dev-bg: stop
    #!/usr/bin/env bash
    echo "Starting development server in background..."
    nohup spin-cli watch > .spin-dev.log 2>&1 &
    echo $! > .spin-dev.pid
    sleep 2
    if kill -0 $(cat .spin-dev.pid) 2>/dev/null; then
        echo "✓ Development server started in background (PID: $(cat .spin-dev.pid))"
        echo "  Logs: tail -f .spin-dev.log"
        echo "  Stop: just stop"
        echo "  Status: just status"
    else
        echo "✗ Failed to start development server"
        rm -f .spin-dev.pid
        exit 1
    fi

# Watch mode: start dev server in background and follow logs (Ctrl+C stops watching only)
watch: dev-bg
    echo "Following development server logs (Ctrl+C to stop watching)..."
    tail -f .spin-dev.log

# Run comprehensive test suite
test:
    ./final_test.sh

# Run linting checks
lint:
    cd api && cargo clippy -- -D warnings

# Format code
fmt:
    cd api && cargo fmt

# Check code quality (lint + format check)
check:
    cd api && cargo clippy -- -D warnings
    cd api && cargo fmt --check

# Update all dependencies
update:
    cd api && cargo update

# Add a new dependency (usage: just add <crate-name>)
add crate:
    cd api && cargo add {{crate}}

# Clean build artifacts
clean:
    cargo clean

# Deploy to Fermyon Cloud
deploy:
    spin-cli deploy

# Run development server in background and execute tests
test-dev:
    #!/usr/bin/env bash
    # Stop any existing servers first
    just stop > /dev/null 2>&1
    echo "Starting development server in background..."
    spin-cli watch > /dev/null 2>&1 &
    SPIN_PID=$!
    sleep 3
    echo "Running tests..."
    ./final_test.sh
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

# Development workflow shortcuts
alias d := dev
alias t := test
alias b := build
alias c := check
alias f := fmt
alias l := lint