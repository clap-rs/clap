#!/bin/bash

# Check status of running diesel fuzzers
# Usage: ./check_fuzzing_status.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_DIR="$SCRIPT_DIR/fuzz_logs"

echo "🔍 Fuzzing Status Check"
echo "======================="
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

# Check each fuzzer
RUNNING_COUNT=0
for TARGET in "${TARGETS[@]}"; do
    # Find PID of running fuzzer
    PID=$(pgrep -f "cargo-fuzz.*$TARGET" || true)

    if [ -n "$PID" ]; then
        echo "✅ Fuzzer $(( ++RUNNING_COUNT )) (PID $PID): RUNNING"
        RUNNING_COUNT=$((RUNNING_COUNT))
    else
        echo "❌ Fuzzer for $TARGET: NOT RUNNING"
    fi
done

echo ""
echo "Summary: $RUNNING_COUNT/6 fuzzers running"
echo ""

# Show latest stats from log files
echo "📊 Latest Stats:"
echo "==============="
echo ""

for TARGET in "${TARGETS[@]}"; do
    # Find the most recent log file for this target
    LOG_FILE=$(ls -t "$LOG_DIR/${TARGET}_"*.log 2>/dev/null | head -1)

    if [ -n "$LOG_FILE" ] && [ -f "$LOG_FILE" ]; then
        echo "$TARGET:"
        # Get last 3 lines that contain stats (lines with #)
        tail -100 "$LOG_FILE" | grep "^#" | tail -3 || echo "  No stats yet"
        echo ""
    fi
done

# Check for crashes
echo "💥 Crashes Found:"
echo "================="

CRASH_DIR="$SCRIPT_DIR/fuzz/artifacts"
if [ -d "$CRASH_DIR" ]; then
    CRASH_COUNT=$(find "$CRASH_DIR" -type f -name "crash-*" -o -name "leak-*" -o -name "timeout-*" 2>/dev/null | wc -l | tr -d ' ')
    if [ "$CRASH_COUNT" -gt 0 ]; then
        echo "  Found $CRASH_COUNT crash(es)!"
        find "$CRASH_DIR" -type f \( -name "crash-*" -o -name "leak-*" -o -name "timeout-*" \) 2>/dev/null | while read crash; do
            echo "    - $(basename $(dirname $crash))/$(basename $crash)"
        done
    else
        echo "  None found yet (keep running!)"
    fi
else
    echo "  None found yet (keep running!)"
fi
