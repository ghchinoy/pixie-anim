#!/bin/bash

# Cleanup script for synthetic test fixtures
# Usage: ./tests/cleanup_fixtures.sh [--all]

KEEP_VIDEO=true

if [ "$1" == "--all" ]; then
    KEEP_VIDEO=false
    echo "🗑️  Full cleanup requested (including videos)..."
else
    echo "🧹 Cleaning up intermediate artifacts (retaining videos)..."
fi

SYNTH_DIR="tests/fixtures/synthetic"

# 1. Remove frame directories
find "$SYNTH_DIR" -maxdepth 1 -type d -name "*_frames" -exec rm -rf {} +

# 2. Remove GIFs and PNGs
find "$SYNTH_DIR" -maxdepth 1 -type f \( -name "*.gif" -o -name "*.png" \) -not -name "README.md" -not -name ".gitkeep" -exec rm -f {} +

# 3. Optionally remove videos
if [ "$KEEP_VIDEO" = false ]; then
    find "$SYNTH_DIR" -maxdepth 1 -type f \( -name "*.mp4" -o -name "*.webm" \) -exec rm -f {} +
fi

echo "✨ Cleanup complete."
