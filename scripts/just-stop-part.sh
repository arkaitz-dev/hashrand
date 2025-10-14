echo "Stopping development servers..."

# Cleanup: Deactivate dry-run mode before stopping (in case tests failed abruptly)
# This ensures development starts with clean state (emails enabled)
echo "Cleaning up dry-run state..."
curl -s http://localhost:3000/api/test/dry-run?enabled=false > /dev/null 2>&1 && \
    echo "✓ Dry-run mode deactivated" || \
    echo "• Server not running or dry-run already off"

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