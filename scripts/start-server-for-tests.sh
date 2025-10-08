#!/usr/bin/env bash
# Start test server with specified log level
# Usage: ./start-server-for-tests.sh [info|debug]

LOG_LEVEL=${1:-info}

echo "Starting test server with RUST_LOG=$LOG_LEVEL..."

# Get the project root directory (parent of scripts/)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Change to project root
cd "$PROJECT_ROOT"

# Load environment variables from .env
export $(cat .env | grep -v '^#' | xargs)

# Override/Set RUST_LOG with specified level
export RUST_LOG=$LOG_LEVEL

# Start spin-cli watch in background (API backend only for tests)
nohup spin-cli watch --runtime-config-file runtime-config.toml -f spin-dev.toml > .spin-dev.log 2>&1 &
SPIN_PID=$!
echo $SPIN_PID > .spin-dev.pid

# Wait for server to start
sleep 3

if kill -0 $SPIN_PID 2>/dev/null; then
    echo "✓ Test server started successfully"
    echo "  PID: $SPIN_PID"
    echo "  Port: 3000"
    echo "  Log level: RUST_LOG=$LOG_LEVEL"
    echo "  Log file: .spin-dev.log"
    exit 0
else
    echo "✗ Failed to start test server"
    rm -f .spin-dev.pid
    exit 1
fi
