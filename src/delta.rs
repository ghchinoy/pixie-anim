//! Inter-frame Delta Compression.

use crate::quant::Rgb;

#[derive(Debug, Default)]
pub struct FrameDelta {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub indices: Vec<u8>,
}

/// Fast RGB distance for fuzzy delta matching
#[inline]
fn rgb_dist_sq(c1: Rgb, c2: Rgb) -> u32 {
    let dr = c1.r as i32 - c2.r as i32;
    let dg = c1.g as i32 - c2.g as i32;
    let db = c1.b as i32 - c2.b as i32;
    (dr * dr + dg * dg + db * db) as u32
}

/// Finds the smallest bounding box and maps pixels to transparent if they are "close enough"
/// to the previous frame's color.
pub fn find_delta_fuzzy(
    width: u16, 
    height: u16, 
    curr_pixels: &[Rgb], 
    prev_pixels: &[Rgb],
    palette: &[Rgb],
    transparent_idx: u8,
    fuzz_threshold: u32, // Squared distance threshold
) -> Option<FrameDelta> {
    if curr_pixels.len() != prev_pixels.len() { return None; }
    
    let mut min_x = width;
    let mut max_x = 0;
    let mut min_y = height;
    let mut max_y = 0;
    let mut changed = false;

    // 1. Find bounding box using fuzzy equality
    for y in 0..height {
        for x in 0..width {
            let idx = (y as usize * width as usize) + x as usize;
            if rgb_dist_sq(curr_pixels[idx], prev_pixels[idx]) > fuzz_threshold {
                if x < min_x { min_x = x; }
                if x > max_x { max_x = x; }
                if y < min_y { min_y = y; }
                if y > max_y { max_y = y; }
                changed = true;
            }
        }
    }

    if !changed {
        return None;
    }

    let delta_width = max_x - min_x + 1;
    let delta_height = max_y - min_y + 1;

    // 2. Map pixels to indices (Parallelized via Rayon)
    use rayon::prelude::*;
    let delta_indices: Vec<u8> = (min_y..=max_y).into_par_iter()
        .flat_map(|y| {
            (min_x..=max_x).into_par_iter().map(move |x| {
                let idx = (y as usize * width as usize) + x as usize;
                // If it's close enough to the previous pixel, make it transparent
                if rgb_dist_sq(curr_pixels[idx], prev_pixels[idx]) <= fuzz_threshold {
                    transparent_idx
                } else {
                    crate::simd::find_nearest_color(curr_pixels[idx], palette) as u8
                }
            })
        })
        .collect();

    Some(FrameDelta {
        x: min_x,
        y: min_y,
        width: delta_width,
        height: delta_height,
        indices: delta_indices,
    })
}
