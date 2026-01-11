# Benchmark Report: cyberpunk_pure_fid
Date: 2026-01-10 18:13:57.764220 -07:00
Input: "tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-180432-0.mp4"

### Notes
Sweep 3: Pure fidelity (no loss, no fuzz), Blue noise, higher sampling quality.

### Parameters
- Quality: 25
- Lossy: 0
- Fuzz: 0
- Dither: blue

| Tool | Time (s) | Size (KB) | Score | SSIM | PSNR |
|------|----------|-----------|-------|------|------|
| Pixie-Anim | 13.641 | 37176.04 | 4.0 | 0.603 | 29.2 |
| Gifsicle | 6.901 | 57999.20 | 7.0 | 0.651 | 27.9 |
| FFmpeg | 8.071 | 60233.67 | 6.0 | 0.651 | 27.9 |
| gifski | 4.082 | 41419.68 | 6.0 | 0.660 | 31.7 |

### Subjective Reasoning
**Pixie-Anim**: The optimization suffers significantly from the limitations of the GIF format's color palette. The original scene's high dynamic range and complex gradients (sky and neon glows) are replaced with heavy dithering and visible color banding. Texture loss is prominent in the atmospheric steam and rain effects, which appear blotchy in the optimized frames.

**Gifsicle**: The optimized GIF manages to preserve the high-contrast neon lighting and overall composition of the original cyberpunk scene. However, the 256-color palette limitation is evident, particularly in the smooth sky gradients and the fine particles of the steam/fog at street level. These areas exhibit significant dithering and some color banding. Fine details like the falling rain are largely smoothed out compared to the original.

**FFmpeg**: The optimized GIF shows significant dithering artifacts, particularly in the dark sky gradients and the low-contrast steam/fog areas on the street level. While the vibrant neon colors are maintained, the 256-color palette limitation is evident in the graininess of the shadows and the loss of fine texture in the rain and pavement reflections. Temporal consistency appears stable across frames, though the heavy dithering likely causes a 'boiling' noise effect in motion.

**gifski**: The conversion to the GIF format has significantly impacted the visual fidelity of the scene. Due to the 256-color palette limitation, there is heavy dithering across the frame, particularly in the mid-tones and the ground reflections. The smooth gradients around the neon lights and in the sky exhibit noticeable banding (posterization). Fine details, such as individual rain streaks and the crispness of the Japanese characters on the signage, have been softened or lost within the dither patterns.

