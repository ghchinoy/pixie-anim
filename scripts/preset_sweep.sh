#!/bin/bash
# Sweep for High/Medium/Low presets

VIDEO="tests/fixtures/synthetic/space_waves_v2.mp4"
CARGO_RUN="cargo run --release --bin pixie-bench --features cli"

echo "🧪 Starting Preset Sweep..."

# LOW: Focus on smallest size, acceptable quality
echo "📉 Testing LOW PRESET..."
$CARGO_RUN -- --input $VIDEO --name sweep_low --quality 3 --lossy 15 --fuzz 15 --dither none --notes "Candidate for Low Quality (Extreme Compression)"

# MEDIUM: Balanced
echo "⚖️ Testing MEDIUM PRESET..."
$CARGO_RUN -- --input $VIDEO --name sweep_medium --quality 6 --lossy 5 --fuzz 5 --dither blue --dither-strength 0.6 --notes "Candidate for Medium Quality (Balanced)"

# HIGH: Maximum fidelity
echo "🌟 Testing HIGH PRESET..."
$CARGO_RUN -- --input $VIDEO --name sweep_high --quality 15 --lossy 0 --fuzz 2 --dither floyd --dither-strength 0.8 --notes "Candidate for High Quality (Fidelity First)"

echo "✅ Sweep Complete. Check tests/reports/ for Gemini judge reasoning."
