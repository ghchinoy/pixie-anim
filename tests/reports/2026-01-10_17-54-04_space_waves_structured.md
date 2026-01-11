# Benchmark Report: space_waves_structured
Date: 2026-01-10 17:54:04.212240 -07:00
Input: "tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-161917-0.mp4"

### Notes
Initial run with structured reporting logic.

### Parameters
- Quality: 5
- Lossy: 8
- Fuzz: 5
- Dither: ordered

| Tool | Time (s) | Size (KB) | Score | SSIM | PSNR |
|------|----------|-----------|-------|------|------|
| Pixie-Anim | 1.897 | 9480.04 | 6.0 | 0.610 | 26.7 |
| Gifsicle | 2.301 | 19034.63 | 6.0 | 0.745 | 28.7 |
| FFmpeg | 3.185 | 19290.80 | 5.0 | 0.745 | 28.7 |
| gifski | 1.660 | 13589.32 | 7.0 | 0.737 | 28.8 |

### Subjective Reasoning
**Pixie-Anim**: The transition from original video to GIF format has significantly impacted the image quality, primarily due to the 256-color palette limitation. The complex gradients in the nebula and the subtle highlights on the waves exhibit noticeable color posterization and banding. To compensate for the color depth loss, a heavy dithering pattern has been applied, which introduces a grainy texture across the entire frame, particularly visible on the slopes of the cosmic waves.

**Gifsicle**: The complex cosmic scene with its vast array of colors and fine textures is heavily impacted by the GIF format's 256-color limit. While the overall color representation remains recognizable, the transition from smooth gradients to dithered patterns is highly apparent. Texture in the water waves is lost to graininess, and subtle halos around stars have been simplified.

**FFmpeg**: The conversion from a high-fidelity original to an optimized GIF shows significant degradation due to the complex nature of the scene. The original features intricate color gradients and fine-grained patterns (stars and shimmering water) that exceed the GIF format's 256-color palette. To compensate, aggressive dithering has been applied, which manifests as a heavy layer of noise/grain across the entire frame. This obscures fine texture in the waves and results in a loss of the 'crispness' seen in the original star fields.

**gifski**: The optimization handles the extremely complex color palette and high-frequency detail of the nebula and iridescent waves reasonably well. However, to compensate for the GIF's 256-color limitation, heavy dithering is applied throughout the image, resulting in a pervasive graininess. This is most noticeable in the smooth gradients of the sky and the bright specular highlights on the water surface, where the original's clarity is replaced by a noisy texture.

