# Benchmark Report: cyberpunk_smooth_ordered
Date: 2026-01-10 18:22:41.294338 -07:00
Input: "tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-180432-0.mp4"

### Notes
Testing Dither Strength reduction (60%) to reduce grain.

### Parameters
- Quality: 25
- Lossy: 0
- Fuzz: 0
- Dither: ordered (Strength: 0.6)

| Tool | Time (s) | Size (KB) | Score | SSIM | PSNR |
|------|----------|-----------|-------|------|------|
| Pixie-Anim | 7.771 | 39377.29 | 4.0 | 0.621 | 29.8 |
| Gifsicle | 6.858 | 57999.20 | 7.0 | 0.651 | 27.9 |
| FFmpeg | 8.069 | 60233.67 | 7.0 | 0.651 | 27.9 |
| gifski | 4.076 | 41419.68 | 7.0 | 0.660 | 31.7 |

### Subjective Reasoning
**Pixie-Anim**: The optimized GIF suffers from significant quality degradation due to the 256-color palette limitation. The complex gradients in the sky show heavy banding/posterization, and the fine textures of the steam and wet pavement are replaced by noticeable dithering patterns. Fine details like rain streaks are almost entirely lost, which would likely result in 'shimmering' or noise in motion.

**Gifsicle**: The optimization maintains high vibrancy in the neon signage, which is impressive given the color-limited GIF format. However, the complex atmospheric effects like steam, rain, and the dark sky gradients exhibit significant dithering noise to compensate for the 256-color palette. Texture on the wet pavement remains relatively sharp, but the smooth light falloff from the neon signs shows slight stepping/banding.

**FFmpeg**: The optimized GIF maintains the vibrant neon aesthetic and overall atmosphere of the cyberpunk scene. However, the format's 256-color limitation is apparent. The most significant trade-off is the heavy dithering used to render the steam and fog on the street, which creates a grainy texture not present in the original. The fine detail of the rain streaks is largely lost or converted into noise. Some color banding is visible in the dark sky gradient and the subtle transitions of light reflections on the wet pavement.

**gifski**: The optimized version preserves the overall cyberpunk aesthetic and high contrast, but the transition to a GIF-limited palette introduces significant dithering patterns, particularly in the sky gradients and the ground-level steam. Reflections on the wet pavement lose their fine grain and sharpness, appearing slightly blocky in the optimized frames.

