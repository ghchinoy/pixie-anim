#!/bin/bash
# Refined Sweep for High/Medium/Low presets

VIDEO="tests/fixtures/synthetic/space_waves_v2.mp4"
CARGO_RUN="cargo run --release --bin pixie-bench --features cli"

echo "🧪 Starting Refined Preset Sweep..."

# LOW: Smaller but aiming for 5.0+ score
echo "📉 Testing LOW PRESET (Refined)..."
$CARGO_RUN -- --input $VIDEO --name refined_low --quality 4 --lossy 20 --fuzz 10 --dither none --notes "Refined Low: Max compression"

# MEDIUM: Aiming for 6.0+ score
echo "⚖️ Testing MEDIUM PRESET (Refined)..."
$CARGO_RUN -- --input $VIDEO --name refined_medium --quality 10 --lossy 8 --fuzz 4 --dither floyd --dither-strength 0.7 --notes "Refined Medium: Better gradients"

# HIGH: Aiming for 7.0+ score (beat Gifsicle)
echo "🌟 Testing HIGH PRESET (Refined)..."
$CARGO_RUN -- --input $VIDEO --name refined_high --quality 30 --lossy 0 --fuzz 1 --dither floyd --dither-strength 0.75 --notes "Refined High: Fidelity priority"

echo "✅ Refined Sweep Complete."
