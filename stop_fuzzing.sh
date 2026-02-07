#!/bin/bash

# Stop all running diesel fuzzers
# Usage: ./stop_fuzzing.sh

echo "🛑 Stopping diesel fuzzers..."
echo ""

# Array of fuzz targets
TARGETS=(
    "fuzz_deserialize_sqlite"
    "fuzz_deserialize_postgres"
    "fuzz_serialize_sqlite"
    "fuzz_migration_parser"
    "fuzz_query_builder"
    "fuzz_json_deserialize"
)

STOPPED_COUNT=0

for TARGET in "${TARGETS[@]}"; do
    # Find PIDs of running fuzzers
    PIDS=$(pgrep -f "cargo-fuzz.*$TARGET" || true)

    if [ -n "$PIDS" ]; then
        echo "Stopping $TARGET (PID: $PIDS)..."
        kill $PIDS 2>/dev/null || true
        STOPPED_COUNT=$((STOPPED_COUNT + 1))
    fi
done

echo ""
echo "✅ Stopped $STOPPED_COUNT fuzzer(s)"

# Wait a moment for processes to terminate
sleep 2

# Check if any are still running
REMAINING=$(pgrep -f "cargo-fuzz" | wc -l | tr -d ' ')
if [ "$REMAINING" -gt 0 ]; then
    echo "⚠️  Warning: $REMAINING fuzzer process(es) still running"
    echo "   Use 'pkill -9 -f cargo-fuzz' to force kill if needed"
else
    echo "✅ All fuzzers stopped"
fi
