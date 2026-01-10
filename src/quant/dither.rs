//! Perceptual Floyd-Steinberg Dithering.

use crate::quant::Rgb;
use crate::simd::{find_nearest_color_lab, PlanarLabPalette};
use crate::color::{rgb_to_lab, Lab};

/// Applies perceptual Floyd-Steinberg dithering to a frame.
/// 
/// Uses CIELAB color space for distance matching and reduces 
/// error strength to 75% to prevent "grainy" artifacts.
pub fn dither_frame(width: u16, height: u16, pixels: &[Rgb], palette: &[Rgb]) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    let mut indices = vec![0u8; w * h];
    
    // 1. Prepare Perceptual Palette
    let lab_palette: Vec<Lab> = palette.iter()
        .map(|p| rgb_to_lab(p.r, p.g, p.b))
        .collect();
    let planar_palette = PlanarLabPalette::from_lab(&lab_palette);
    
    // Error buffers (storing errors as f32 for Lab space precision)
    let mut error_buf_l = vec![0.0f32; w * h];
    let mut error_buf_a = vec![0.0f32; w * h];
    let mut error_buf_b = vec![0.0f32; w * h];
    
    // Dither Strength (75%)
    let strength = 0.75f32;
    
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            
            // 2. Get original pixel + accumulated error in Lab space
            let original_lab = rgb_to_lab(pixels[idx].r, pixels[idx].g, pixels[idx].b);
            let current_lab = Lab {
                l: (original_lab.l + error_buf_l[idx]).clamp(0.0, 100.0),
                a: (original_lab.a + error_buf_a[idx]).clamp(-128.0, 127.0),
                b: (original_lab.b + error_buf_b[idx]).clamp(-128.0, 127.0),
            };
            
            // 3. Find nearest color in Lab palette
            let color_idx = find_nearest_color_lab(current_lab, &planar_palette);
            indices[idx] = color_idx as u8;
            
            let best_color_lab = lab_palette[color_idx];
            
            // 4. Calculate error (difference between perceptual current and best match)
            let err_l = (current_lab.l - best_color_lab.l) * strength;
            let err_a = (current_lab.a - best_color_lab.a) * strength;
            let err_b = (current_lab.b - best_color_lab.b) * strength;
            
            // 5. Diffuse error to neighbors
            if x + 1 < w {
                diffuse(w, &mut error_buf_l, &mut error_buf_a, &mut error_buf_b, x + 1, y, err_l, err_a, err_b, 7.0/16.0);
            }
            if y + 1 < h {
                if x > 0 {
                    diffuse(w, &mut error_buf_l, &mut error_buf_a, &mut error_buf_b, x - 1, y + 1, err_l, err_a, err_b, 3.0/16.0);
                }
                diffuse(w, &mut error_buf_l, &mut error_buf_a, &mut error_buf_b, x, y + 1, err_l, err_a, err_b, 5.0/16.0);
                if x + 1 < w {
                    diffuse(w, &mut error_buf_l, &mut error_buf_a, &mut error_buf_b, x + 1, y + 1, err_l, err_a, err_b, 1.0/16.0);
                }
            }
        }
    }
    
    indices
}

#[inline]
fn diffuse(
    w: usize, 
    buf_l: &mut [f32], buf_a: &mut [f32], buf_b: &mut [f32], 
    x: usize, y: usize, 
    el: f32, ea: f32, eb: f32, 
    weight: f32
) {
    let idx = y * w + x;
    buf_l[idx] += el * weight;
    buf_a[idx] += ea * weight;
    buf_b[idx] += eb * weight;
}