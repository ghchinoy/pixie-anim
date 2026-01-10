# Pixie-Anim Tests & Benchmarking

This directory is the primary workspace for evaluating the performance and visual quality of Pixie-Anim. It contains automated tools for comparative analysis, subjective judging, and long-term regression tracking.

## 🚀 Unified Benchmarking with `pixie-bench`

The `pixie-bench` tool is a high-performance Rust binary that orchestrates the entire evaluation pipeline. It compares Pixie-Anim against industry standards: **Gifsicle**, **FFmpeg**, and **gifski**.

### Automated E2E Workflow
If you have a video file (MP4/WebM), you can run a full end-to-end test in one command:

```bash
./target/release/pixie-bench --input path/to/video.mp4 --name my_test_run --report tests/benchmarks.md
```

**What happens under the hood:**
1.  **Extraction**: `ffmpeg` extracts frames at 15fps (scaled to 640px) into `tests/fixtures/synthetic/my_test_run_frames/`.
2.  **Optimization**: All four engines (Pixie, Gifsicle, FFmpeg, gifski) generate optimized GIFs.
3.  **Subjective Judging**: The tool extracts key frames and sends them to **Gemini 3 Flash** for a 1-10 quality score and artifact analysis.
4.  **Reporting**: A Markdown table and detailed critique are appended to `tests/benchmarks.md`.

---

## ⛰️ Hill-Climbing: Iterative Benchmarking

The most efficient way to tune Pixie-Anim's quality is the "Hill-Climbing" workflow. This avoids the overhead of re-extracting frames from video.

1.  **Prepare Frames**: Run the E2E command once (see above).
2.  **Iterate on Parameters**: Run `pixie-bench` directly against the extracted frame directory while tweaking flags:
    ```bash
    ./target/release/pixie-bench --input tests/fixtures/synthetic/my_test_run_frames/ --name hill_climb_v1 --lossy 8 --fuzz 10
    ```
3.  **Review Reports**: Check `tests/benchmarks.md` to see if your changes improved the Subjective Score or reduced the file size.

---

## 🧪 Synthetic Asset Generation

We use AI to generate consistent, high-fidelity video fixtures for benchmarking.

**Standard Tooling Defaults:**
- **Model**: `veo-3.1-fast-generate-preview`
- **Duration**: 8s
- **Aspect Ratio**: 16:9

### Reference Profiles
| Profile | Prompt |
| :--- | :--- |
| **High Motion** | "A fast-paced drone shot through a neon-lit cyberpunk city with heavy rain and flickering lights." |
| **Low Motion** | "A static overhead shot of a minimalist workspace with a slow-moving clock hand." |
| **Complex Texture**| "A close-up of colorful liquid ink swirling in water, creating complex gradients and transitions." |

---

## 🧹 Housekeeping & Utilities

### Cleanup
To keep your disk usage low, use the cleanup utility:
```bash
./tests/cleanup_fixtures.sh         # Purges frames and GIFs, retains source videos
./tests/cleanup_fixtures.sh --all   # Purges everything
```

### Parity Tests
Verifies that the WASM module matches native behavior:
```bash
node tests/parity.js
```

### Legacy Scripts
While `pixie-bench` is the preferred tool, the original scripts remain available for simple automation:
- `tests/run_e2e_test.sh`: Bash wrapper for video extraction.
- `benchmark.sh`: The core logic used by `pixie-bench` for external tool calls.