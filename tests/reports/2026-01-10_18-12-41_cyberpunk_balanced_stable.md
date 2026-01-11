# Benchmark Report: cyberpunk_balanced_stable
Date: 2026-01-10 18:12:41.467535 -07:00
Input: "tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-180432-0.mp4"

### Notes
Sweep 2: Minimum lossiness and fuzz, returning to Ordered dither for maximum stability.

### Parameters
- Quality: 20
- Lossy: 1
- Fuzz: 1
- Dither: ordered

| Tool | Time (s) | Size (KB) | Score | SSIM | PSNR |
|------|----------|-----------|-------|------|------|
| Pixie-Anim | 5.635 | 32340.91 | 5.0 | 0.568 | 28.8 |
| Gifsicle | 6.917 | 57999.20 | 7.0 | 0.651 | 27.9 |
| FFmpeg | 8.972 | 60233.67 | 6.0 | 0.651 | 27.9 |
| gifski | 4.089 | 41419.68 | 7.0 | 0.660 | 31.7 |

### Subjective Reasoning
**Pixie-Anim**: The optimized GIF suffers from significant palette limitations inherent to the format. There is noticeable color banding in the vertical sky channel and heavy dithering across the dark pavement and misty areas. Fine details like individual rain streaks are almost entirely lost, replaced by a noisy dither pattern that will likely cause visible 'crawling' during playback. Neon light glows have lost their smooth falloff, appearing blocky and pixelated.

**Gifsicle**: The optimized GIF performs reasonably well given the high-contrast, color-rich nature of the source material. However, the conversion to a limited color palette is evident through heavy dithering (stippling) in the dark gradients of the buildings and the soft steam/fog effects on the street. While this dithering prevents harsh color banding, it introduces a constant graininess that obscures fine details like individual rain streaks and subtle building textures.

**FFmpeg**: The optimization maintains the overall atmosphere and vibrant neon colors of the cyberpunk scene. however, the GIF's 256-color limitation is very apparent in the gradients. Heavy dithering is used to represent the smooth transitions in the sky and the volumetric steam on the ground, resulting in a grainy texture. Fine details like individual rain streaks are significantly obscured compared to the original video frames.

**gifski**: The optimized GIF does an admirable job of preserving the high-contrast neon aesthetics and the overall atmosphere of the original video. However, because the source material is visually complex—featuring rain, steam, and numerous light sources—the GIF format's 256-color limitation is apparent. A heavy dithering pattern is visible across the sky, ground reflections, and steam areas to manage the color gradients. This introduces a persistent 'grainy' texture that was not present in the original, and fine details like individual rain streaks are partially lost to this noise.

