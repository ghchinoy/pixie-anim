#!/bin/bash

# Pixie-Anim vs Gifsicle vs FFmpeg vs gifski Benchmark Runner
# Usage: ./benchmark.sh <frame_dir> <original_video> <output_name>

if [ "$#" -ne 3 ]; then
    echo "Usage: ./benchmark.sh <frame_dir> <original_video> <output_name>"
    exit 1
fi

FRAME_DIR=$1
ORIGINAL_VIDEO=$2
NAME=$3

# Binary Checks
for cmd in ffmpeg gifsicle gifski bc; do
    if ! command -v $cmd &> /dev/null; then
        echo "❌ Error: $cmd is not installed. Please install it to run benchmarks."
        exit 1
    fi
done

PIXIE_OUT="tests/fixtures/synthetic/${NAME}_pixie.gif"
BASELINE_OUT="tests/fixtures/synthetic/${NAME}_baseline.gif"
GIFSICLE_OUT="tests/fixtures/synthetic/${NAME}_gifsicle.gif"
FFMPEG_OUT="tests/fixtures/synthetic/${NAME}_ffmpeg.gif"
GIFSKI_OUT="tests/fixtures/synthetic/${NAME}_gifski.gif"
PALETTE_OUT="tests/fixtures/synthetic/${NAME}_palette.png"

# Ensure environment is set for the judge
if [ -z "$GEMINI_API_KEY" ] && [ ! -f .env ]; then
    echo "⚠️  Warning: GEMINI_API_KEY not found in environment or .env file. Judge will fail."
fi

echo "--- 🏃 Starting Macro Benchmark: $NAME ---"

# 1. Run Pixie-Anim
echo "[1/4] Running Pixie-Anim (Dithered, Lossy)..."
START=$(date +%s%N)
cargo run --release --features="cli" --bin pixie-anim -- $FRAME_DIR/*.png --fps 15 --dither --lossy 8 --output $PIXIE_OUT > /dev/null 2>&1
END=$(date +%s%N)
PIXIE_TIME=$(echo "scale=3; ($END - $START) / 1000000000" | bc)

# 2. Run Gifsicle -O3
echo "[2/4] Running Gifsicle -O3..."
# Create a simple baseline for Gifsicle
ffmpeg -y -i $FRAME_DIR/frame%03d.png -vf "palettegen=max_colors=256" tests/fixtures/synthetic/tmp_palette.png > /dev/null 2>&1
ffmpeg -y -i $FRAME_DIR/frame%03d.png -i tests/fixtures/synthetic/tmp_palette.png -lavfi "paletteuse" $BASELINE_OUT > /dev/null 2>&1
START=$(date +%s%N)
gifsicle -O3 $BASELINE_OUT -o $GIFSICLE_OUT
END=$(date +%s%N)
GIFSICLE_TIME=$(echo "scale=3; ($END - $START) / 1000000000" | bc)

# 3. Run 2-pass FFmpeg (High Quality)
echo "[3/4] Running 2-pass FFmpeg..."
START=$(date +%s%N)
ffmpeg -y -i $FRAME_DIR/frame%03d.png -vf "palettegen" $PALETTE_OUT > /dev/null 2>&1
ffmpeg -y -i $FRAME_DIR/frame%03d.png -i $PALETTE_OUT -lavfi "paletteuse" $FFMPEG_OUT > /dev/null 2>&1
END=$(date +%s%N)
FFMPEG_TIME=$(echo "scale=3; ($END - $START) / 1000000000" | bc)

# 4. Run gifski (Ultra Quality)
echo "[4/4] Running gifski..."
START=$(date +%s%N)
gifski -o $GIFSKI_OUT $FRAME_DIR/*.png > /dev/null 2>&1
END=$(date +%s%N)
GIFSKI_TIME=$(echo "scale=3; ($END - $START) / 1000000000" | bc)

# 5. Run Gemini Judge for all tools
echo "⚖️  Running Gemini Subjective Judge for all outputs..."

run_judge() {
    local target_file=$1
    local result=$(cargo run --release --features="cli" --bin judge -- $ORIGINAL_VIDEO $target_file 2>/dev/null)
    echo "$result"
}

echo "  -> Judging Pixie-Anim..."
RES_PIXIE=$(run_judge $PIXIE_OUT)
SCORE_PIXIE=$(echo "$RES_PIXIE" | grep -o '"score": [0-9]*' | head -1 | cut -d' ' -f2)
REASON_PIXIE=$(echo "$RES_PIXIE" | sed -n 's/.*"reasoning": "\([^"]*\)".*/\1/p' | head -1)

echo "  -> Judging Gifsicle..."
RES_GIFSICLE=$(run_judge $GIFSICLE_OUT)
SCORE_GIFSICLE=$(echo "$RES_GIFSICLE" | grep -o '"score": [0-9]*' | head -1 | cut -d' ' -f2)
REASON_GIFSICLE=$(echo "$RES_GIFSICLE" | sed -n 's/.*"reasoning": "\([^"]*\)".*/\1/p' | head -1)

echo "  -> Judging FFmpeg..."
RES_FFMPEG=$(run_judge $FFMPEG_OUT)
SCORE_FFMPEG=$(echo "$RES_FFMPEG" | grep -o '"score": [0-9]*' | head -1 | cut -d' ' -f2)
REASON_FFMPEG=$(echo "$RES_FFMPEG" | sed -n 's/.*"reasoning": "\([^"]*\)".*/\1/p' | head -1)

echo "  -> Judging gifski..."
RES_GIFSKI=$(run_judge $GIFSKI_OUT)
SCORE_GIFSKI=$(echo "$RES_GIFSKI" | grep -o '"score": [0-9]*' | head -1 | cut -d' ' -f2)
REASON_GIFSKI=$(echo "$RES_GIFSKI" | sed -n 's/.*"reasoning": "\([^"]*\)".*/\1/p' | head -1)

# 6. Results
PIXIE_SIZE=$(du -k "$PIXIE_OUT" | cut -f1)
GIFSICLE_SIZE=$(du -k "$GIFSICLE_OUT" | cut -f1)
FFMPEG_SIZE=$(du -k "$FFMPEG_OUT" | cut -f1)
GIFSKI_SIZE=$(du -k "$GIFSKI_OUT" | cut -f1)

echo ""
echo "--- 📊 Macro Benchmark Results: $NAME ---"
echo "Tool        | Time (s) | Size (KB) | Subjective Score (1-10)"
echo "------------|----------|-----------|-------------------------"
echo "Pixie-Anim  | $PIXIE_TIME | $PIXIE_SIZE | $SCORE_PIXIE"
echo "Gifsicle    | $GIFSICLE_TIME | $GIFSICLE_SIZE | $SCORE_GIFSICLE"
echo "FFmpeg      | $FFMPEG_TIME | $FFMPEG_SIZE | $SCORE_FFMPEG"
echo "gifski      | $GIFSKI_TIME | $GIFSKI_SIZE | $SCORE_GIFSKI"
echo "------------|----------|-----------|-------------------------"

echo ""
echo "--- 🧠 Gemini Subjective Reasoning ---"
echo "Pixie-Anim: $REASON_PIXIE"
echo ""
echo "Gifsicle:   $REASON_GIFSICLE"
echo ""
echo "FFmpeg:     $REASON_FFMPEG"
echo ""
echo "gifski:     $REASON_GIFSKI"

# Calculate improvement vs Gifsicle
if [ ! -z "$GIFSICLE_SIZE" ] && [ "$GIFSICLE_SIZE" -ne 0 ]; then
    SIZE_DIFF=$(echo "scale=1; (1 - $PIXIE_SIZE / $GIFSICLE_SIZE) * 100" | bc)
    echo ""
    echo "Compression Improvement (Pixie vs Gifsicle): $SIZE_DIFF%"
fi
