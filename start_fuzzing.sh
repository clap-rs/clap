#!/bin/bash

# Start all diesel fuzzers in the background
# Usage: ./start_fuzzing.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FUZZ_DIR="$SCRIPT_DIR/fuzz"
LOG_DIR="$SCRIPT_DIR/fuzz_logs"

# Create log directory
mkdir -p "$LOG_DIR"

# Get timestamp for log files
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

echo "🚀 Starting diesel fuzzers..."
echo "📁 Logs directory: $LOG_DIR"
echo ""

cd "$FUZZ_DIR"

# Array of fuzz targets
TARGETS=(
    "fuzz_deserialize_sqlite"
    "fuzz_deserialize_postgres"
    "fuzz_serialize_sqlite"
    "fuzz_migration_parser"
    "fuzz_query_builder"
    "fuzz_json_deserialize"
)

# Start each fuzzer in the background
for TARGET in "${TARGETS[@]}"; do
    LOG_FILE="$LOG_DIR/${TARGET}_${TIMESTAMP}.log"
    echo "Starting $TARGET..."
    echo "  Log: $LOG_FILE"

    # Run fuzzer with 64MB max input size, unlimited time
    cargo fuzz run "$TARGET" -- \
        -max_len=65536 \
        -rss_limit_mb=2048 \
        > "$LOG_FILE" 2>&1 &

    echo "  PID: $!"
    echo ""

    # Small delay to avoid overwhelming the system
    sleep 2
done

echo "✅ All fuzzers started!"
echo ""
echo "To check status: ./check_fuzzing_status.sh"
echo "To stop all fuzzers: ./stop_fuzzing.sh"
