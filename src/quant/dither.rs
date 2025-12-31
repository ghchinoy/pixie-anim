//! Floyd-Steinberg Dithering.

use crate::quant::Rgb;
use crate::simd::find_nearest_color;

/// Applies Floyd-Steinberg dithering to a frame.
/// 
/// Error diffusion weights:
///       *   7/16
/// 3/16  5/16  1/16
pub fn dither_frame(width: u16, height: u16, pixels: &[Rgb], palette: &[Rgb]) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    let mut indices = vec![0u8; w * h];
    
    // Error buffers (storing errors as i16 to avoid overflow and handle negatives)
    // We need 3 channels (R, G, B)
    let mut error_buf = vec![0i16; w * h * 3];
    
    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let err_idx = idx * 3;
            
            // 1. Get original pixel + accumulated error
            let r = (pixels[idx].r as i16 + error_buf[err_idx]).clamp(0, 255) as u8;
            let g = (pixels[idx].g as i16 + error_buf[err_idx + 1]).clamp(0, 255) as u8;
            let b = (pixels[idx].b as i16 + error_buf[err_idx + 2]).clamp(0, 255) as u8;
            
            let current_rgb = Rgb { r, g, b };
            
            // 2. Find nearest color in palette
            let color_idx = find_nearest_color(current_rgb, palette);
            indices[idx] = color_idx as u8;
            
            let best_color = palette[color_idx];
            
            // 3. Calculate error
            let err_r = r as i16 - best_color.r as i16;
            let err_g = g as i16 - best_color.g as i16;
            let err_b = b as i16 - best_color.b as i16;
            
            // 4. Diffuse error to neighbors
            // Neighbor weights:
            // (x+1, y)   : 7/16
            // (x-1, y+1) : 3/16
            // (x, y+1)   : 5/16
            // (x+1, y+1) : 1/16
            
            if x + 1 < w {
                diffuse(w, &mut error_buf, x + 1, y, err_r, err_g, err_b, 7);
            }
            if y + 1 < h {
                if x > 0 {
                    diffuse(w, &mut error_buf, x - 1, y + 1, err_r, err_g, err_b, 3);
                }
                diffuse(w, &mut error_buf, x, y + 1, err_r, err_g, err_b, 5);
                if x + 1 < w {
                    diffuse(w, &mut error_buf, x + 1, y + 1, err_r, err_g, err_b, 1);
                }
            }
        }
    }
    
    indices
}

#[inline]
fn diffuse(w: usize, buf: &mut [i16], x: usize, y: usize, er: i16, eg: i16, eb: i16, weight: i16) {
    let idx = (y * w + x) * 3;
    buf[idx] += (er * weight) / 16;
    buf[idx + 1] += (eg * weight) / 16;
    buf[idx + 2] += (eb * weight) / 16;
}
