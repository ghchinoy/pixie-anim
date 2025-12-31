#!/bin/bash

# Pixie-Anim vs Gifsicle vs FFmpeg Benchmark Runner
# Usage: ./benchmark.sh <frame_dir> <original_video> <output_name>

if [ "$#" -ne 3 ]; then
    echo "Usage: ./benchmark.sh <frame_dir> <original_video> <output_name>"
    exit 1
fi

FRAME_DIR=$1
ORIGINAL_VIDEO=$2
NAME=$3
PIXIE_OUT="tests/fixtures/synthetic/${NAME}_pixie.gif"
BASELINE_OUT="tests/fixtures/synthetic/${NAME}_baseline.gif"
GIFSICLE_OUT="tests/fixtures/synthetic/${NAME}_gifsicle.gif"
FFMPEG_OUT="tests/fixtures/synthetic/${NAME}_ffmpeg.gif"
PALETTE_OUT="tests/fixtures/synthetic/${NAME}_palette.png"

# Ensure environment is set for the judge
if [ -z "$GEMINI_API_KEY" ] && [ ! -f .env ]; then
    echo "⚠️  Warning: GEMINI_API_KEY not found in environment or .env file. Judge will fail."
fi

echo "--- 🏃 Starting Macro Benchmark: $NAME ---"

# 1. Run Pixie-Anim
echo "[1/6] Running Pixie-Anim (Dithered)..."
START=$(date +%s%N)
cargo run --release --features="cli" --bin pixie-anim -- $FRAME_DIR/*.png --fps 15 --dither --output $PIXIE_OUT > /dev/null 2>&1
END=$(date +%s%N)
PIXIE_TIME=$(echo "scale=3; ($END - $START) / 1000000000" | bc)

# 2. Create Baseline via FFmpeg (needed for Gifsicle)
echo "[2/5] Creating baseline GIF via FFmpeg..."
ffmpeg -y -i $FRAME_DIR/frame%03d.png -vf "palettegen=max_colors=256" tests/fixtures/synthetic/tmp_palette.png > /dev/null 2>&1
ffmpeg -y -i $FRAME_DIR/frame%03d.png -i tests/fixtures/synthetic/tmp_palette.png -lavfi "paletteuse" $BASELINE_OUT > /dev/null 2>&1

# 3. Run Gifsicle -O3
echo "[3/5] Running Gifsicle -O3..."
START=$(date +%s%N)
gifsicle -O3 $BASELINE_OUT -o $GIFSICLE_OUT
END=$(date +%s%N)
GIFSICLE_TIME=$(echo "scale=3; ($END - $START) / 1000000000" | bc)

# 4. Run 2-pass FFmpeg (High Quality)
echo "[4/6] Running 2-pass FFmpeg..."
START=$(date +%s%N)
ffmpeg -y -i $FRAME_DIR/frame%03d.png -vf "palettegen" $PALETTE_OUT > /dev/null 2>&1
ffmpeg -y -i $FRAME_DIR/frame%03d.png -i $PALETTE_OUT -lavfi "paletteuse" $FFMPEG_OUT > /dev/null 2>&1
END=$(date +%s%N)
FFMPEG_TIME=$(echo "scale=3; ($END - $START) / 1000000000" | bc)

# 5. Run gifski (Ultra Quality)
echo "[5/6] Running gifski..."
GIFSKI_OUT="tests/fixtures/synthetic/${NAME}_gifski.gif"
START=$(date +%s%N)
gifski -o $GIFSKI_OUT $FRAME_DIR/*.png > /dev/null 2>&1
END=$(date +%s%N)
GIFSKI_TIME=$(echo "scale=3; ($END - $START) / 1000000000" | bc)

# 6. Run Gemini Judge
echo "[6/6] Running Gemini Subjective Judge..."
JUDGE_RESULT=$(cargo run --release --features="cli" --bin judge -- $ORIGINAL_VIDEO $PIXIE_OUT 2>/dev/null)
SCORE=$(echo "$JUDGE_RESULT" | grep -o '"score": [0-9]*' | head -1 | cut -d' ' -f2)
REASONING=$(echo "$JUDGE_RESULT" | sed -n 's/.*"reasoning": "\([^"]*\)".*/\1/p' | head -1)

# 7. Results
PIXIE_SIZE=$(du -k "$PIXIE_OUT" | cut -f1)
GIFSICLE_SIZE=$(du -k "$GIFSICLE_OUT" | cut -f1)
FFMPEG_SIZE=$(du -k "$FFMPEG_OUT" | cut -f1)
GIFSKI_SIZE=$(du -k "$GIFSKI_OUT" | cut -f1)

echo ""
echo "--- 📊 Benchmark Results: $NAME ---"
echo "Tool        | Time (s) | Size (KB) | Subjective Score (1-10)"
echo "------------|----------|-----------|-------------------------"
echo "Pixie-Anim  | $PIXIE_TIME | $PIXIE_SIZE | $SCORE"
echo "Gifsicle    | $GIFSICLE_TIME | $GIFSICLE_SIZE | -"
echo "FFmpeg      | $FFMPEG_TIME | $FFMPEG_SIZE | -"
echo "gifski      | $GIFSKI_TIME | $GIFSKI_SIZE | -"
echo "------------|----------|-----------|-------------------------"
echo ""
echo "Gemini Reasoning: $REASONING"

# Calculate improvement vs Gifsicle
if [ ! -z "$GIFSICLE_SIZE" ] && [ "$GIFSICLE_SIZE" -ne 0 ]; then
    SIZE_DIFF=$(echo "scale=1; (1 - $PIXIE_SIZE / $GIFSICLE_SIZE) * 100" | bc)
    echo "Compression Improvement vs Gifsicle: $SIZE_DIFF%"
fi