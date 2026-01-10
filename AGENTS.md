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
- **Reference Code**: Always consult `~/dev/github/pixo` for SIMD patterns, WASM bindings, and "Pixo" stylistic conventions before implementing core modules.

## Code Standards for Codecs

1. **Bit-Level Testing**: All bit-packing logic (`BitWriter`) and variable-length encoders (`LZW`) must have unit tests with known-good outputs before being integrated into the `GifWriter`.


2. **Safety**: Adhere to `pixo`'s safety philosophy—forbid unsafe unless in SIMD kernels.


3. **Benchmarking**: Every core optimization should be accompanied by a `criterion` benchmark in `benches/`.


## WASM & Compilation Heuristics


- **Target Architecture**: When building for `wasm32-unknown-unknown`, always guard SIMD-specific modules (like `x86_64`) with `#[cfg(target_arch = "x86_64")]`.


- **Allocator**: Use `talc` as the global allocator for WASM to minimize binary size.

- **Naming**: To avoid linking collisions between the library and CLI binary, the library is named `pixie_anim_lib`. Use `use pixie_anim_lib::...` for imports.


- **Panic Strategy**: Criterion benchmarks require `panic = "unwind"`, while optimized releases use `panic = "abort"`. These are handled via profile overrides in `Cargo.toml`.


## GIF Specification & Codec Heuristics

1. **Block Order**: The **Global Color Table** MUST immediately follow the **Logical Screen Descriptor** if the global palette flag is set. Writing extension blocks (like Netscape Loop) between them will break browser decoding.



2. **Lossy LZW**: We implement lossiness by matching "Fuzzy Neighbors" in the palette. This only works effectively if the palette is ordered by similarity (e.g., using the **Zeng Algorithm**).



3. **WASM Memory**: For large animations (70MB+), always revoke old Object URLs and reuse dictionary buffers in the LZW encoder to prevent browser hangs.

4. **Subjective Evaluation**: Use `gemini-3-flash-preview` for visual quality checks. Since it doesn't support `image/gif`, extract key frames as PNGs for comparison.




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
