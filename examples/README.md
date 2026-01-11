# Pixie-Anim Examples

This directory contains examples demonstrating how to use the `pixie-anim` library to optimize image sequences and convert video files to GIFs.

## Prerequisites

All examples require the `image` feature to be enabled. To run them, use:

```bash
cargo run --example <example_name> --features image
```

## Available Examples

### 1. `basic_optimization.rs`
The most direct way to use the library. It takes a list of PNG frame paths and generates an optimized GIF.

*   **Logic**: Uses `optimize_sequence` directly.
*   **Assumptions**: Requires pre-extracted PNG frames. By default, it looks for frames in `tests/fixtures/synthetic/space_waves_frames/`.
*   **Use Case**: When you already have frames in memory or on disk and want to apply Pixie's optimization engine.

### 2. `video_to_gif.rs`
A more comprehensive real-world example showing the full pipeline from MP4 to GIF.

*   **Logic**: 
    1.  Spawns `ffmpeg` to extract frames at 15fps.
    2.  Collects paths and runs `optimize_sequence`.
    3.  Cleans up the temporary frame directory.
*   **Assumptions**: 
    *   Requires **FFmpeg** to be installed and available in the system PATH.
    *   Requires a video file at `tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-161917-0.mp4`.
*   **Use Case**: Building a CLI tool or service that converts video uploads to highly optimized, short-form GIFs.

---

## Configuration Tips

When using `OptimizationOptions`, keep these heuristics in mind:

*   **quality**: Iterations for K-Means. `5` is fast, `20` is high-fidelity.
*   **lossy**: The "secret sauce". Values between `5` and `15` provide massive size reductions with minimal visual impact.
*   **dither**: 
    *   `Ordered`: Best for video (maximum temporal stability).
    *   `BlueNoise`: Best for a "film grain" look.
    *   `FloydSteinberg`: Best for static image quality, but can "shimmer" in video.
*   **fuzz**: Higher values increase transparency masking, reducing size for high-motion or noisy backgrounds.
