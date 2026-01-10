## Benchmark: reported_cat (Sat Jan 10 12:15:10 MST 2026)
Input: tests/fixtures/synthetic/veo-veo-3.1-fast-generate-preview-20260110-115322-0.mp4

--- 🏃 Starting Macro Benchmark: reported_cat ---
[1/4] Running Pixie-Anim (Dithered, Lossy)...
[2/4] Running Gifsicle -O3...
[3/4] Running 2-pass FFmpeg...
[4/4] Running gifski...
⚖️  Running Gemini Subjective Judge for all outputs...
  -> Judging Pixie-Anim...
  -> Judging Gifsicle...
  -> Judging FFmpeg...
  -> Judging gifski...

--- 📊 Macro Benchmark Results: reported_cat ---
Tool        | Time (s) | Size (KB) | Subjective Score (1-10)
------------|----------|-----------|-------------------------
Pixie-Anim  | 1.935 | 11168 | 6
Gifsicle    | 2.292 | 21852 | 6
FFmpeg      | 2.571 | 22784 | 5
gifski      | 1.543 | 15624 | 6
------------|----------|-----------|-------------------------

--- 🧠 Gemini Subjective Reasoning ---
Pixie-Anim: The optimized GIF shows significant dithering artifacts, particularly in the aurora borealis and the night sky, to compensate for the limited 256-color palette. While the overall scene remains recognizable, the smooth color gradients of the original are replaced by a grainy texture. Fine details in the cat's fur and the snow on the trees are noticeably softened and muddied by the dithering pattern.

Gifsicle:   The optimized GIF shows significant degradation compared to the original video. The most prominent issue is the heavy dithering and color banding within the aurora borealis and the dark sky, which is a direct result of the GIF's limited color palette. Fine textures, such as the cat's fur and the snow-laden tree branches, have lost their crispness and appear grainy.

FFmpeg:     The optimized GIF suffers from significant color banding in the aurora borealis gradients, a common limitation of the 8-bit palette. Extensive dithering is visible across the snowy foreground and the cat's shadow, creating a grainy texture. Fine details in the cat's fur and the needles of the snow-covered trees are noticeably softened.

gifski:     The optimization process has introduced significant dithering and graininess across the frame to compensate for the color palette limitations of the GIF format. While this approach successfully avoids severe color banding in the aurora gradients, it results in a pervasive 'noisy' texture that obscures fine details in the cat's fur and the snow. The clarity of the original video is noticeably reduced, though the color fidelity remains reasonably high for a GIF.

Compression Improvement (Pixie vs Gifsicle): 50.0%

---

