## Project Context: Pixie-Anim

**Goal:** A high-performance, zero-dependency GIF optimizer following the "Pixo" philosophy of from-scratch algorithm implementation.

### Tech Stack
- **Core Engine:** Rust (targeted for WASM).
- **Frontend UI:** Lit WebComponents (Vite + TypeScript).
- **Issue Tracking:** `bd` (Beads) at `/home/ghchinoy/.local/bin/bd`.
- **Planning:** See `docs/plan.md` and `docs/architecture.dot`.

### Core Philosophy

1. **Zero Runtime Dependencies:** All codecs (LZW, etc.) must be implemented from scratch.
2. **Benchmarking First:** Every optimization must be measured against `gifsicle` and `ffmpeg` (see `pixie-anim-sn4`).
3. **Lossy Optimization:** Use K-Means quantization and inter-frame transparency to beat standard compression ratios.
4. **SIMD-First:** Target WASM SIMD for the nearest-color search bottleneck.

## Issue Tracking

This project uses **bd (beads)** for issue tracking.

Run `bd prime` for workflow context, or install hooks (`bd hooks install`) for auto-injection.


**Quick reference:**

- `bd ready` - Find unblocked work
- `bd create "Title" --type task --priority 2` - Create issue
- `bd close <id>` - Complete work
- `bd sync` - Sync with git (run at session end)

For full workflow details: `bd prime`

## Temporary File & Test Fixture Management Policy

1. **Synthetic Assets**: Large media files generated via AI (Veo, etc.) for benchmarking should be stored in `tests/fixtures/synthetic/`.
2. **Git Ignore**: All `.mp4`, `.gif`, `.jpg`, and `.png` files in `tests/fixtures/synthetic/` are ignored by git to keep the repository size manageable.
3. **Reproducibility**: When generating a synthetic asset, record the prompt and tool parameters in the relevant task or a `README.md` within the fixtures directory.
4. **Cleanup**: Temporary build artifacts or intermediate conversion files should be placed in the system temp directory or the project's ignored `target/` directory.


## Tool Heuristics & Lessons Learned

- **Veo (Video Generation)**: Prefer `veo-3.1-fast-generate-preview` (Veo 3) as it supports audio and offers faster generation. When using older models like `veo-2.0-generate-001`, `generate_audio` must be explicitly set to `false`.
- **Beads (Issue Tracking)**: Use `bd ready` to find work and `bd close <id>` to finish. Status updates are handled via `bd update <id> --status <status>`.
- **WASM & Web Integration**: The WASM bindings in `web/src/lib/pixie-wasm/` are git-ignored. If Vite reports missing imports for `pixie.js`, ensure `npm run build-wasm` has been executed. The `dev` script in `web/package.json` is configured to run this automatically.
- **WebCodecs**: `VideoFrame.copyTo` does not support scaling (no `destWidth`/`destHeight`). To resize frames during extraction, draw the `VideoFrame` to an `OffscreenCanvas` of the target size and use `getImageData`.
- **Temporal Stability**: Spatial error diffusion (Floyd-Steinberg) causes "shimmering" in video. For animations, prefer deterministic mask-based dithering (like Blue Noise or Ordered) to ensure static areas remain consistent across frames.
- **Crate Publication**: To verify a "Zero-Dependency" mandate, always test with `cargo build --no-default-features`. Ensure all public APIs use Struct-based parameter passing to avoid `too_many_arguments` lints and improve ergonomics.
- **Reference Code**: Always consult `~/dev/github/pixo` for SIMD patterns, WASM bindings, and "Pixo" stylistic conventions before implementing core modules.
- **WASM Versioning**: `wasm-bindgen-cli` MUST match the version in `Cargo.lock` exactly. The CI/CD pipeline in `.github/workflows/deploy.yml` handles this automatically by extracting the version at runtime. Always use `actions/cache` for the `wasm-bindgen` binary to avoid 4-minute re-compilations.
- **pnpm Supply Chain Policies**: When setting up the web environment via `pnpm install`, if pnpm halts or warns regarding ignored build scripts (such as `esbuild`), run `pnpm approve-builds` interactively to allow compiling Vite's dependencies.

## Code Standards for Codecs

1. **Bit-Level Testing**: All bit-packing logic (`BitWriter`) and variable-length encoders (`LZW`) must have unit tests with known-good outputs before being integrated into the `GifWriter`.
2. **Safety**: Adhere to `pixo`'s safety philosophy—forbid unsafe unless in SIMD kernels.
3. **Benchmarking**: Every core optimization should be accompanied by a `criterion` benchmark in `benches/`.


## WASM & Compilation Heuristics

- **Target Architecture**: When building for `wasm32-unknown-unknown`, always guard SIMD-specific modules (like `x86_64`) with `#[cfg(target_arch = "x86_64")]`.
- **Allocator**: Use `talc` as the global allocator for WASM to minimize binary size.
- **Rayon & WASM**: Standard `rayon` causes a `RuntimeError: unreachable` in WASM because it attempts to spawn a thread pool. Always disable the `rayon` feature for `wasm32-unknown-unknown` targets.
- **Naming**: To avoid linking collisions between the library and CLI binary, the library is named `pixie_anim_lib`. Use `use pixie_anim_lib::...` for imports.
- **Panic Strategy**: Criterion benchmarks require `panic = "unwind"`, while optimized releases use `panic = "abort"`. These are handled via profile overrides in `Cargo.toml`.
- **CLI Tooling**: If `wasm-bindgen` is missing from the shell path during `npm run build-wasm`, verify the version in `Cargo.lock` (e.g., `0.2.106`) and install it using `cargo install wasm-bindgen-cli --version <version>`. Ensure `~/.cargo/bin/` is in the terminal environment PATH.


## GIF Specification & Codec Heuristics

1. **Block Order**: The **Global Color Table** MUST immediately follow the **Logical Screen Descriptor** if the global palette flag is set. Writing extension blocks (like Netscape Loop) between them will break browser decoding.

2. **Lossy LZW**: We implement lossiness by matching "Fuzzy Neighbors" in the palette. This only works effectively if the palette is ordered by similarity (e.g., using the **Zeng Algorithm**).

3. **WASM Memory**: For large animations (70MB+), always revoke old Object URLs and reuse dictionary buffers in the LZW encoder to prevent browser hangs.
- **GIF Compositing**: Never assume GIF frames are full-screen. Always implement a "virtual canvas" when decoding GIFs to raw RGBA buffers for re-optimization; otherwise, sub-rectangle frames will cause buffer length mismatches in downstream encoders.
- **WASM Memory Safety**: When passing large buffers from JS to WASM, prefer `uint8Array.slice()` over `new Uint8Array(buffer, offset, length)`. The latter creates a view into WASM memory that can become "detached" if the WASM heap grows (e.g., during a large allocation in `encode_gif`), leading to memory access errors.

4. **Subjective Evaluation**: Use `gemini-3-flash-preview` for visual quality checks. Since it doesn't support `image/gif`, extract key frames as PNGs for comparison.

## Hill-Climbing & Quality Tuning Protocol

To improve Pixie-Anim's visual quality score, follow this iterative protocol:

1. **Establish Baseline**: Run a full E2E benchmark on a synthetic video fixture:
   ```bash
   ./target/release/pixie-bench --input tests/fixtures/synthetic/video.mp4 --name base_test --report tests/benchmarks.md
   ```
2. **Analyze Critique**: Review the `Subjective Reasoning` in the report. Look for keywords like "banding" (needs better quantization), "grainy" (needs better dithering), or "soft" (needs better detail preservation).
3. **Iterate (The Speed Dial)**: Tweak parameters or code, then run the benchmark against the *already extracted* frames to save time:
   ```bash
   ./target/release/pixie-bench --input tests/fixtures/synthetic/base_test_frames/ --original tests/fixtures/synthetic/video.mp4 --name iter_1 --quality 20 --lossy 5
   ```
4. **Preset Validation**: Use `scripts/preset_sweep_v2.sh` to ensure that any changes to the core engine still maintain the perceptual quality bars for the "High/Medium/Low" presets.
5. **Target Score**: Goal is a Subjective Score >= 7.0 while maintaining a size advantage of >30% over Gifsicle.

## Pre-Publishing Quality Gate (Checklist)

Before any `cargo publish`, the following sequence MUST be executed and verified:

1.  **Version Bump**: Update `Cargo.toml` version string (e.g., `0.1.x`).
2.  **Linting**: Run `cargo clippy --all-features` and fix all warnings.
3.  **Formatting**: Run `cargo fmt` to ensure standard style.
4.  **Unit Tests**: Run `cargo test --all-features` to verify logic integrity.
5.  **WASM Build**: Run `npm run build-wasm` in the `web/` directory to verify bindings and WASM stability.
6.  **Examples**: Verify all examples compile: `cargo check --examples --all-features`.
7.  **Benchmarks Validation**: Verify all benchmarks compile successfully by running `cargo check --benches --all-features`.
8.  **Dry Run**: Execute `cargo publish --dry-run` to catch metadata or manifest issues.

## Core Algorithm Refinements

- **Temporal Denoising**: We use "Cross-Frame Palette Re-indexing" (The Lazy Rule). If a pixel's color is within `fuzz / 2` of the previous frame's color at that coordinate, we reuse the previous palette index. This dramatically improves LZW compression and eliminates "shimmering" artifacts.
- **Perceptual Dithering**: Always perform dithering in CIELAB space. We cap error diffusion at 75% strength to prevent excessive grain.
- **Evaluation-Driven Presets**: High/Medium/Low presets are not arbitrary; they are grounded in **Synthetic MOS** scores from Gemini. We target specific size/quality ratios (e.g., High = Score 7.0+, Size -30% vs Gifsicle) and enforce these through the `preset_sweep` regression tool.

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds