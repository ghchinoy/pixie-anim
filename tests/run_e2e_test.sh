#!/bin/bash

# End-to-End Test and Benchmark Wrapper
# Usage: ./tests/run_e2e_test.sh <input_video> <test_name> [<report_file>]

if [ "$#" -lt 2 ]; then
    echo "Usage: ./tests/run_e2e_test.sh <input_video> <test_name> [<report_file>]"
    exit 1
fi

INPUT_VIDEO=$1
TEST_NAME=$2
REPORT_FILE=$3
FRAME_DIR="tests/fixtures/synthetic/${TEST_NAME}_frames"

if ! command -v ffmpeg &> /dev/null; then
    echo "❌ Error: ffmpeg is not installed. Required for frame extraction."
    exit 1
fi

# 1. Prepare Frame Directory
echo "🎞️  Extracting frames from $INPUT_VIDEO..."
mkdir -p "$FRAME_DIR"
ffmpeg -y -i "$INPUT_VIDEO" -vf "fps=15,scale=640:-1" "$FRAME_DIR/frame%03d.png" > /dev/null 2>&1

if [ $? -ne 0 ]; then
    echo "❌ Error: Frame extraction failed."
    exit 1
fi

# 2. Run Benchmark
# benchmark.sh <frame_dir> <original_video> <output_name>
if [ -n "$REPORT_FILE" ]; then
    echo "📝 Running benchmark and saving report to $REPORT_FILE..."
    {
        echo "## Benchmark: $TEST_NAME ($(date))"
        echo "Input: $INPUT_VIDEO"
        echo ""
        ./benchmark.sh "$FRAME_DIR" "$INPUT_VIDEO" "$TEST_NAME"
        echo -e "\n---\n"
    } | tee -a "$REPORT_FILE"
else
    ./benchmark.sh "$FRAME_DIR" "$INPUT_VIDEO" "$TEST_NAME"
fi

# 3. Cleanup
if [[ "$*" == *"--cleanup"* ]]; then
    echo "🧹 Cleaning up intermediate frames..."
    rm -rf "$FRAME_DIR"
else
    echo "💾 Frames retained in $FRAME_DIR for iterative benchmarking."
fi

echo "✅ E2E Test Complete for $TEST_NAME"
