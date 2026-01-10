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

## Benchmark: cat_q20 (2026-01-10 13:38:31.960354 -07:00)
Input: "tests/fixtures/synthetic/rust_cat_test_frames/"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.692 | 11180.04 | 0.0 |
| Gifsicle | 2.460 | 21849.95 | 0.0 |
| FFmpeg | 2.448 | 21911.65 | 0.0 |
| gifski | 1.526 | 14832.96 | 0.0 |

### Subjective Reasoning
**Pixie-Anim**: 

**Gifsicle**: 

**FFmpeg**: 

**gifski**: 


---

## Benchmark: cat_no_dither (2026-01-10 13:38:42.250291 -07:00)
Input: "tests/fixtures/synthetic/rust_cat_test_frames/"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.476 | 11160.28 | 0.0 |
| Gifsicle | 2.385 | 21849.95 | 0.0 |
| FFmpeg | 2.380 | 21911.65 | 0.0 |
| gifski | 1.619 | 14832.96 | 0.0 |

### Subjective Reasoning
**Pixie-Anim**: 

**Gifsicle**: 

**FFmpeg**: 

**gifski**: 


---

## Benchmark: cat_q20_judged (2026-01-10 13:39:46.436892 -07:00)
Input: "tests/fixtures/synthetic/rust_cat_test_frames/"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.572 | 11180.04 | 4.0 |
| Gifsicle | 2.339 | 21849.95 | 6.0 |
| FFmpeg | 2.824 | 21911.65 | 6.0 |
| gifski | 1.555 | 14832.96 | 7.0 |

### Subjective Reasoning
**Pixie-Anim**: The optimization suffers significantly from the 256-color limitation inherent to the GIF format. The most prominent issue is the heavy dithering throughout the aurora borealis and the night sky, which replaces smooth gradients with a grainy, speckled texture. There is also noticeable color banding around the moon and in the transitions of the green and purple light. The fine texture of the cat's fur has lost its softness, appearing much coarser and more pixelated compared to the original video.

**Gifsicle**: The conversion to GIF has introduced significant color banding and posterization in the smooth gradients of the aurora borealis and the night sky. The 256-color palette limitation is very apparent, resulting in a loss of depth. Heavy dithering is visible in the sky and on the snow's surface to compensate for the lack of colors, which degrades the overall texture quality.

**FFmpeg**: The optimized GIF suffers from noticeable color banding, particularly within the smooth gradients of the aurora borealis and the night sky. Dithering is used extensively to manage the limited color palette, which creates a grainy texture in the darker regions and on the cat's fur. However, the temporal consistency between frames appears stable, and the overall scene remains clear and recognizable.

**gifski**: The optimized GIF shows significant dithering artifacts, particularly in the smooth gradients of the aurora borealis and the night sky, which is a common limitation of the 256-color GIF format. While the overall color representation is preserved, the fine texture of the cat's fur and the soft luminance of the snow are noticeably noisier and less detailed than the original. Temporal consistency appears stable across the three frames, though the dithering pattern creates a graininess not present in the source.


---

## Benchmark: cat_no_dither_judged (2026-01-10 13:40:24.361963 -07:00)
Input: "tests/fixtures/synthetic/rust_cat_test_frames/"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.568 | 11160.28 | 5.0 |
| Gifsicle | 2.336 | 21849.95 | 7.0 |
| FFmpeg | 2.386 | 21911.65 | 6.0 |
| gifski | 1.529 | 14832.96 | 7.0 |

### Subjective Reasoning
**Pixie-Anim**: The conversion to GIF has resulted in significant quality loss, primarily due to the limited color palette. The smooth gradients of the aurora borealis in the original frames are replaced by heavy dithering and noticeable color banding in the optimized versions. There is a substantial loss of fine detail in the cat's fur and the texture of the snow, which now appear grainy and noisy.

**Gifsicle**: The optimized GIF shows significant dithering artifacts in the gradients of the Aurora Borealis and the dark night sky. This is a common limitation of the 256-color palette in GIFs. However, the detail on the cat's fur and the foreground snow remains relatively sharp and well-preserved.

**FFmpeg**: The optimization process has introduced significant dithering and color banding, particularly in the aurora borealis and night sky gradients. This is a common limitation of the GIF format's 256-color palette when handling complex, high-contrast scenes. Fine texture in the snow and the cat's fur has been softened or replaced by dither patterns, leading to a loss of detail. The starfield appears noisier due to these artifacts.

**gifski**: The optimized GIF shows noticeable dithering artifacts, particularly in the smooth gradients of the aurora borealis and the glow around the moon. This is a result of the 256-color limitation inherent to the GIF format. While the cat's fur and the structural details of the trees remain well-preserved, the fine texture of the snow in the foreground appears noisier due to the quantization process.


---

## Benchmark: cat_hillclimb_v2 (2026-01-10 13:42:42.886513 -07:00)
Input: "tests/fixtures/synthetic/rust_cat_test_frames/"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.726 | 10615.46 | 6.0 |
| Gifsicle | 2.295 | 21849.95 | 5.0 |
| FFmpeg | 2.874 | 21911.65 | 6.0 |
| gifski | 1.692 | 14832.96 | 7.0 |

### Subjective Reasoning
**Pixie-Anim**: The optimization shows significant quality degradation due to the GIF's 256-color limitation. The smooth gradients of the aurora borealis are replaced with heavy, visible dithering patterns to prevent banding. Similarly, the soft texture of the snow is lost to a grainy dithered appearance. While the subject (the cat) maintains decent edge clarity, the overall image feels noisy and lacks the high-fidelity depth of the original.

**Gifsicle**: The optimization process has introduced significant color banding in the gradients of the aurora borealis, which is a common limitation of the 8-bit color palette in GIFs. Heavy dithering is visible across the night sky and the shadowed regions of the snow, creating a grainy texture. Fine details in the cat's fur and the sharpness of the stars are noticeably reduced compared to the original video frames.

**FFmpeg**: The optimized GIF suffers from significant dithering artifacts throughout the sky and aurora borealis gradients, which is a common limitation of the 256-color GIF palette. The smooth transitions of the original video are replaced with visible stippling/noise. While the cat's texture is relatively well-preserved, the snowy landscape loses some micro-detail and develops a grainy appearance. Temporal consistency is likely compromised as the dither patterns appear to shift between frames, which usually results in a 'shimmering' effect in dark areas during playback.

**gifski**: The optimized GIF handles the complex lighting of the Aurora Borealis reasonably well, but the 8-bit palette limitation is evident. Significant dithering is visible in the sky's gradients to prevent harsh color banding, resulting in a grainy texture throughout the upper half of the frame. Fine details in the cat's fur and the pine needles are slightly muddied compared to the original video frames.


---

## Benchmark: cat_hillclimb_v3 (2026-01-10 13:45:18.406094 -07:00)
Input: "tests/fixtures/synthetic/rust_cat_test_frames/"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.920 | 10612.12 | 6.0 |
| Gifsicle | 2.335 | 21849.95 | 5.0 |
| FFmpeg | 2.761 | 21911.65 | 6.0 |
| gifski | 1.626 | 14832.96 | 7.0 |

### Subjective Reasoning
**Pixie-Anim**: The optimization shows significant artifacts typical of GIF's 256-color limitation. The complex gradients of the aurora borealis result in heavy dithering, which creates a grainy appearance across the sky. The fine texture of the cat's fur is noticeably simplified, and the smooth shadows on the snow have been replaced with stippled patterns. However, the overall composition and contrast remain intact.

**Gifsicle**: The optimization suffers significantly from the limitations of the GIF format's color palette. The smooth, complex gradients of the aurora borealis and the night sky exhibit heavy dithering and noticeable color banding. While the cat's silhouette and main textures are preserved, the atmospheric depth of the original is lost due to the grainy dither pattern applied across the frame.

**FFmpeg**: The transition to a limited color palette for the GIF format has caused significant degradation in the sky and aurora. High-contrast gradients in the aurora borealis exhibit heavy dithering and visible color banding. While the cat's silhouette and large patterns are preserved, fine texture in the fur and the crispness of the stars are lost due to compression and palette optimization.

**gifski**: The optimized GIF performs reasonably well given the complexity of the scene (high-contrast stars and smooth aurora gradients). However, there is heavy dithering throughout the sky to compensate for the limited color palette, which replaces smooth transitions with a grainy texture. Fine details in the cat's fur and the snow's surface are softened compared to the original.


---

## Benchmark: cat_hillclimb_v4_lossless (2026-01-10 13:49:19.340352 -07:00)
Input: "tests/fixtures/synthetic/rust_cat_test_frames/"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.576 | 15097.36 | 6.0 |
| Gifsicle | 2.237 | 21849.95 | 6.0 |
| FFmpeg | 2.391 | 21911.65 | 7.0 |
| gifski | 1.582 | 14832.96 | 7.0 |

### Subjective Reasoning
**Pixie-Anim**: The optimized GIF suffers from significant dithering and graininess, which is highly visible in the smooth gradients of the aurora borealis and the dark sky. This is a result of the 256-color palette limitation. While the main subject (the cat) retains its shape and basic texture, the fine details of the fur and the soft snow on the trees have been replaced by noise. The stars have also become less distinct due to the heavy dither patterns.

**Gifsicle**: The optimization process to GIF format has significantly impacted the image quality. The most prominent issue is the heavy dithering used to compensate for the 256-color palette, which creates a grainy texture across the entire image. This is particularly noticeable in the sky and on the cat's fur. Additionally, the smooth color gradients of the aurora borealis now exhibit visible banding and 'noisy' transitions compared to the original video frames.

**FFmpeg**: The conversion to GIF introduces noticeable dithering patterns in the sky and aurora borealis to compensate for the limited color palette. While the spatial consistency between frames is good, the smooth gradients of the original are replaced by graininess. There is a slight loss of fine texture in the cat's fur and the shadows on the snow.

**gifski**: The optimized GIF shows noticeable color banding in the complex gradients of the aurora borealis and the night sky, a common limitation of the 256-color GIF palette. Heavy dithering is visible in the foreground snow and shadowed areas to compensate for color loss. While the cat's fur texture is relatively well-preserved, there is a loss of fine-grain detail in the starry background and the softer edges of the clouds.


---

## Benchmark: cat_blue_noise (2026-01-10 14:19:36.658827 -07:00)
Input: "tests/fixtures/synthetic/rust_cat_test_frames/"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 3.396 | 11441.87 | 4.0 |
| Gifsicle | 2.320 | 21849.95 | 7.0 |
| FFmpeg | 2.534 | 21911.65 | 6.0 |
| gifski | 1.563 | 14832.96 | 5.0 |

### Subjective Reasoning
**Pixie-Anim**: The optimization suffers from extreme dithering artifacts across the entire frame. While the general colors are maintained, the smooth gradients of the aurora and the soft texture of the snow are replaced with a heavy, noisy grain. Fine details on the cat's fur and the pine needles are significantly obscured by the dither pattern. The high-frequency noise will likely cause 'crawling' artifacts during playback.

**Gifsicle**: The complex color gradients of the aurora borealis present a significant challenge for the GIF format, resulting in visible dithering and subtle color banding in the sky. While the central subject (the cat) retains good detail and texture, the fine patterns of the falling snow and the smooth transitions in the background sky suffer from the limited 256-color palette.

**FFmpeg**: The optimized GIF shows significant dithering artifacts, particularly in the gradients of the aurora borealis and the night sky. This is a result of the 256-color palette limitation of the GIF format attempting to reproduce smooth color transitions. While the main subject (the cat) and the high-contrast snow-covered trees maintain acceptable detail, the overall image texture feels grainy compared to the clean original.

**gifski**: The optimization shows significant quality degradation due to the 256-color limitation of the GIF format. There is heavy dithering across the entire image, particularly noticeable on the snow and in the dark areas of the sky. The smooth gradients of the aurora borealis have been replaced by coarse banding and dither patterns. Fine details in the cat's fur and the pine needles on the trees are partially obscured by these artifacts.


---

## Benchmark: cat_blue_noise_v2 (2026-01-10 14:20:29.049670 -07:00)
Input: "tests/fixtures/synthetic/rust_cat_test_frames/"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 3.491 | 11441.87 | 4.0 |
| Gifsicle | 2.349 | 21849.95 | 6.0 |
| FFmpeg | 2.608 | 21911.65 | 6.0 |
| gifski | 1.633 | 14832.96 | 6.0 |

### Subjective Reasoning
**Pixie-Anim**: The optimized GIF exhibits significant quality degradation due to aggressive dithering across all frames. While this helps prevent harsh color banding in the aurora gradients, it creates a pervasive 'noise' or 'grain' that obscures fine textures in the cat's fur and the snowy ground. The 256-color palette limitation is very apparent, resulting in a loss of the original's crispness and depth.

**Gifsicle**: The optimization suffers significantly in the aurora borealis regions, where the limited GIF color palette causes noticeable color banding and stair-stepping in the gradients. To compensate for the color depth loss, heavy dithering is applied throughout the starry sky and snow, resulting in a grainier texture compared to the smooth original. The structural detail of the cat's fur is mostly preserved, but the overall aesthetic quality is diminished by the background artifacts.

**FFmpeg**: The conversion to GIF introduces significant dithering and color banding, particularly in the complex gradients of the aurora borealis and the night sky. While the main subject (the cat) retains its form, the fine texture of its fur becomes grainy. The snow on the trees also loses some of its soft detail, appearing more pixelated.

**gifski**: The optimized frames exhibit significant dithering across the entire image to compensate for the limited GIF color palette, particularly visible in the aurora borealis and the night sky. While this preserves the general color accuracy, it introduces a grainy texture that obscures fine details like individual snowflakes and fur patterns. The smooth gradients of the original are replaced by stippled patterns, which often leads to 'boiling' noise in motion.


---

## Benchmark: cat_ordered (2026-01-10 14:22:16.296725 -07:00)
Input: "tests/fixtures/synthetic/rust_cat_test_frames/"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.985 | 10609.03 | 6.0 |
| Gifsicle | 2.342 | 21849.95 | 6.0 |
| FFmpeg | 2.538 | 21911.65 | 5.0 |
| gifski | 1.573 | 14832.96 | 6.0 |

### Subjective Reasoning
**Pixie-Anim**: The optimization process has introduced significant dithering noise throughout the image to compensate for the GIF's 256-color limitation. This is most noticeable in the smooth gradients of the aurora borealis and the night sky, which now appear grainy. The fine texture of the cat's fur and the soft snow has been replaced by high-frequency noise artifacts, though the overall sharpness and composition remain intact.

**Gifsicle**: The conversion to GIF introduces significant dithering across the aurora borealis and sky to compensate for the limited color palette. This results in a grainy appearance in areas that should be smooth gradients. Fine details in the cat's fur and the snow texture are also somewhat obscured by the dithering patterns.

**FFmpeg**: The optimization shows significant quality degradation typical of the GIF format's 256-color limitation. The most prominent issue is heavy color banding and blocky transitions in the aurora borealis gradients. Extensive dithering (noise) is visible across the entire image, particularly in the dark night sky and the shadows on the snow, which muddies the fine textures of the cat's fur and the crispness of the stars.

**gifski**: The optimized version suffers from significant dithering artifacts across the sky gradients and the snow-covered foreground, which is typical for GIFs trying to replicate high-color depth scenes like an aurora borealis. While the overall composition and contrast are maintained, the fine textures of the cat's fur and the pine needles on the trees are noticeably degraded and replaced by granular noise.


---

## Benchmark: space_waves (2026-01-10 16:20:20.635965 -07:00)
Input: "tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-161917-0.mp4"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.877 | 9821.73 | 4.0 |
| Gifsicle | 2.115 | 19034.63 | 6.0 |
| FFmpeg | 3.051 | 19290.80 | 6.0 |
| gifski | 1.705 | 13589.32 | 6.0 |

### Subjective Reasoning
**Pixie-Anim**: The optimization is heavily impacted by the limitations of the GIF format, particularly given the complexity of the cosmic source material. There is excessive dithering across the entire frame, which transforms the smooth, gaseous textures of the original nebula into a grainy, noisy pattern. Color fidelity is significantly reduced, leading to visible banding in the dark gradients of space. Fine details, such as smaller stars and the delicate 'foam' on the wave crests, are largely lost or obscured by the dither noise.

**Gifsicle**: The optimization suffers significantly due to the inherent limitations of the GIF format when handling complex, high-dynamic-range scenes. The smooth gradients of the space nebula and the iridescent waves are replaced with heavy dithering patterns to compensate for the 256-color palette. This results in a persistent 'grainy' texture across the entire image, which obscures the fine detail of the star field and the silky texture of the waves seen in the original.

**FFmpeg**: The optimization shows clear signs of the GIF format's technical limitations when handling complex, high-dynamic-range cosmic scenes. The original's smooth gradients in the nebulae are replaced by noticeable dithering patterns used to approximate the color palette, which introduces a pervasive graininess. While the high-contrast elements remain legible, the 'painterly' texture of the waves is lost to noise, and fine star details are slightly blurred or absorbed by the dither pattern.

**gifski**: The optimized GIF shows significant graininess due to heavy dithering, which is used to compensate for the 256-color limit in a scene with complex gradients and high-frequency star detail. While it avoids harsh banding, the fine details of the starfield and the nebular textures are noticeably degraded and blurred.


---

## Benchmark: space_waves_regression (2026-01-10 16:25:26.541450 -07:00)
Input: "tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-161917-0.mp4"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.997 | 9821.73 | 6.0 |
| Gifsicle | 2.250 | 19034.63 | 6.0 |
| FFmpeg | 3.207 | 19290.80 | 6.0 |
| gifski | 1.736 | 13589.32 | 7.0 |

### Subjective Reasoning
**Pixie-Anim**: The optimization struggles with the high-complexity gradients of the cosmic scene. The GIF's 256-color limitation leads to significant dithering across the wave surfaces and noticeable banding in the nebular clouds. While the overall structure is preserved, the fine crystalline texture of the water and the pinpoint sharpness of the stars are compromised.

**Gifsicle**: The optimization suffers significantly from the limited GIF color palette. The original features complex gradients in both the cosmic background and the iridescent waves, which are represented in the optimized version using aggressive dithering. This leads to a substantial loss of fine detail and the introduction of a pervasive grainy texture. While the general composition and vibrant colors are maintained, the clarity of the celestial bodies and the smooth transitions of the water's surface are degraded.

**FFmpeg**: The conversion from original video to optimized GIF introduces significant texture changes. While the overall color vibrancy is maintained through a well-distributed palette, the format's 256-color limitation necessitates heavy dithering across the entire frame. This is particularly noticeable in the smooth gradients of the space background and the shimmering wave surfaces, which now appear grainy. Fine details in the distant starfield are partially obscured by the noise pattern.

**gifski**: The optimized GIF shows heavy dithering throughout the image, which is a necessary trade-off for the limited 256-color palette given the complex gradients of the original cosmic scene. While the core structure and highlights of the waves are preserved, the fine detail and smooth transitions in the nebulae and dark space areas are replaced by a grainy texture. Temporal consistency appears maintained as the dither pattern seems stable between the frames provided.


---

## Benchmark: space_waves_api_regression (2026-01-10 16:27:55.022151 -07:00)
Input: "tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-161917-0.mp4"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.867 | 9821.73 | 7.0 |
| Gifsicle | 2.178 | 19034.63 | 7.0 |
| FFmpeg | 2.854 | 19290.80 | 6.0 |
| gifski | 1.702 | 13589.32 | 7.0 |

### Subjective Reasoning
**Pixie-Anim**: The optimization handles a very complex color palette (nebula and cosmic waves) reasonably well by using heavy dithering to avoid flat-color banding. While the vibrant purples and golds are preserved, the image suffers from significant graininess due to the 256-color limitation of the GIF format. Texture detail on the waves remains surprisingly sharp, but the smooth gradients of the original nebula are now replaced by a coarse dithered pattern.

**Gifsicle**: The optimized GIF maintains the overall color profile and high-level composition of the original scene. However, due to the complex color gradients and fine details in the nebula and water-like surface, heavy dithering is present throughout the frames. This creates a pervasive grainy texture that masks the original's smooth transitions and fine specular highlights.

**FFmpeg**: The optimized GIF shows significant texture degradation compared to the original due to the heavy dithering required to manage the complex color gradients and star fields. The 256-color limitation is very apparent, resulting in a pervasive graininess across both the sky and the wave surfaces. While dithering helps mitigate severe color banding in the nebula, it obscures the fine specular highlights and the sharpness of the star points.

**gifski**: The optimized GIF handles an extremely challenging cosmic scene with a wide color gamut and complex gradients reasonably well. To avoid severe color banding in the nebulae and sky, a dense dithering pattern was applied. While this preserves the general appearance of gradients, it introduces significant graininess and 'noise' compared to the smooth original. Fine textures in the gaseous waves have lost their soft, ethereal quality, becoming more granular.


---

## Benchmark: space_waves_final_regression (2026-01-10 16:33:14.570815 -07:00)
Input: "tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-161917-0.mp4"

| Tool | Time (s) | Size (KB) | Score |
|------|----------|-----------|-------|
| Pixie-Anim | 1.877 | 9821.73 | 6.0 |
| Gifsicle | 2.181 | 19034.63 | 5.0 |
| FFmpeg | 2.990 | 19290.80 | 5.0 |
| gifski | 1.632 | 13589.32 | 6.0 |

### Subjective Reasoning
**Pixie-Anim**: The transition from the original video to the optimized GIF results in significant quality degradation due to the 256-color palette limitation. The complex gradients in the nebula and the iridescent wave surface suffer from heavy dithering and visible color banding. While the overall composition and 'glow' are maintained, the fine 'stardust' texture of the original is replaced by a noisy, patterned dither.

**Gifsicle**: The optimized GIF suffers from significant quality loss due to the inherent 256-color limitation of the format being applied to a complex, multi-tonal scene. The most prominent issue is heavy dithering across the sky and sea, which replaces smooth gradients with a grainy texture. Fine details, such as smaller stars and the delicate ripples on the waves, are blurred or lost entirely to compression.

**FFmpeg**: The optimization process has significantly impacted the visual quality, primarily due to the limitations of the GIF format's 256-color palette. Heavy dithering is visible across the entire image to compensate for the complex gradients in the nebula and the waves, leading to a pervasive grainy texture. Additionally, there is a complete loss of temporal motion in the provided optimized frames (p1, p2, and p3 are identical), whereas the original frames show progression in the wave patterns and cosmic movement.

**gifski**: The complex gradients of the nebulae and the iridescent water surface are poorly handled by the GIF format's 256-color limit. While the optimizer used heavy dithering to prevent color banding, this has resulted in a significant amount of noise and a grainy texture across the entire image. Fine details, such as the smaller stars in the background, have lost their sharp definition and are blended into the dithering pattern.


---

