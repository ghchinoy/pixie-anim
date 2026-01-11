# Benchmark Report: cyberpunk_high_fid
Date: 2026-01-10 18:11:31.188614 -07:00
Input: "tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-180432-0.mp4"

### Notes
Sweep 1: Lowering lossiness and fuzz, switching to Blue Noise for better texture.

### Parameters
- Quality: 20
- Lossy: 2
- Fuzz: 2
- Dither: blue

| Tool | Time (s) | Size (KB) | Score | SSIM | PSNR |
|------|----------|-----------|-------|------|------|
| Pixie-Anim | 13.612 | 31474.87 | 6.0 | 0.526 | 28.2 |
| Gifsicle | 6.940 | 57999.20 | 7.0 | 0.651 | 27.9 |
| FFmpeg | 7.396 | 60233.67 | 6.0 | 0.651 | 27.9 |
| gifski | 3.927 | 41419.68 | 7.0 | 0.660 | 31.7 |

### Subjective Reasoning
**Pixie-Anim**: The optimized GIF shows significant dithering across the sky and in the steam/fog elements on the street, which is a common limitation of the GIF format when handling complex gradients. While the vibrant neon colors are relatively well-maintained, the smooth transitions in the atmospheric lighting are replaced by 'noisy' dither patterns. Fine details on the vertical signs and the textures of the buildings show noticeable softening compared to the original frames.

**Gifsicle**: The optimized GIF performs reasonably well given the complexity of the scene and the inherent limitations of the GIF format. The vibrant neon colors are preserved, but there is significant dithering in the sky and the steam/mist on the ground. Color banding is visible in the gradients around the bright neon signs, and the fine granularity of the mist has been lost to blotchier dithered patches.

**FFmpeg**: The optimization shows significant color banding in the sky and neon sign glows due to the limited GIF color palette. Heavy dithering is used to compensate for the lack of smooth gradients, particularly visible in the steam/fog on the ground and dark building textures, leading to a grainy appearance. Fine details like rain streaks are partially lost or obscured by the dither pattern.

**gifski**: The optimized GIF maintains good structural integrity and legible neon signage. However, the scene's complexity—featuring dark gradients, atmospheric steam, and vibrant neon light—strains the 256-color palette. Significant dithering is visible in the sky and smoke plumes, which replaces smooth transitions with grainy patterns. Fine details like rain streaks are largely lost or merged into the dithering noise.

