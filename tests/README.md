# Pixie-Anim Tests & Benchmarking

This directory contains parity tests and serves as the staging area for benchmarking assets.

## Synthetic Benchmarking Assets

We use AI-generated video as synthetic benchmarks to test the performance and compression of the Pixie-Anim engine across various motion profiles.

### Generation Workflow

Large media files are generated via **Veo 3.1 Fast** and stored in `tests/fixtures/synthetic/`. These files are ignored by git to maintain repository health.

**How to generate and test a new benchmark (Automated):**
1. Generate a video using `veo_t2v` and save to `tests/fixtures/synthetic/my_video.mp4`.
2. Run the E2E test script which handles extraction and benchmarking:
   ```bash
   ./tests/run_e2e_test.sh tests/fixtures/synthetic/my_video.mp4 my_test_name
   ```

**How to generate the 'cyberpunk_drone' benchmark (Manual):**
1. Use the `veo_t2v` tool from the [Vertex AI GenMedia MCP Servers](https://github.com/GoogleCloudPlatform/vertex-ai-creative-studio/tree/main/experiments/mcp-genmedia) with the following parameters:
   - **Prompt**: "A fast-paced drone shot through a neon-lit cyberpunk city with heavy rain and flickering lights."
   - **Model**: `veo-3.1-fast-generate-preview`
   - **Duration**: 8s
   - **Output Directory**: `tests/fixtures/synthetic/`
2. Extract frames at 15fps:
   ```bash
   mkdir -p tests/fixtures/synthetic/cyberpunk_frames
   ffmpeg -i tests/fixtures/synthetic/veo_output.mp4 -vf "fps=15" tests/fixtures/synthetic/cyberpunk_frames/frame%03d.png
   ```

**Standard Tooling Defaults:**
- **Model**: `veo-3.1-fast-generate-preview`
- **Duration**: 8s
- **Aspect Ratio**: 16:9

### Reference Prompts

1. **High Motion**: "A fast-paced drone shot through a neon-lit cyberpunk city with heavy rain and flickering lights."
2. **Low Motion/Flat**: "A static overhead shot of a minimalist workspace with a slow-moving clock hand."
3. **Complex Texture**: "A close-up of colorful liquid ink swirling in water, creating complex gradients and transitions."

## Parity Tests
- `parity.js`: Verifies that the WASM module correctly initializes and exports the expected functions. Can be run via `node tests/parity.js` after building the WASM module.

## Benchmarking Script
The root `benchmark.sh` uses these fixtures to compare Pixie-Anim against `gifsicle` and `ffmpeg`.
