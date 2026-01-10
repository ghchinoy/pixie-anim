//! Scalar fallbacks for performance-critical operations.

use crate::quant::Rgb;

use crate::color::Lab;
use crate::simd::PlanarLabPalette;

/// Find the index of the nearest color in Lab space using a planar palette.
pub fn find_nearest_color_lab_planar(pixel: Lab, palette: &PlanarLabPalette) -> usize {
    let mut min_dist = f32::MAX;
    let mut best_idx = 0;

    for i in 0..palette.len {
        let dl = pixel.l - palette.l[i];
        let da = pixel.a - palette.a[i];
        let db = pixel.b - palette.b[i];
        let dist = dl * dl + da * da + db * db;

        if dist < min_dist {
            min_dist = dist;
            best_idx = i;
        }
    }

    best_idx
}

/// Find the index of the nearest color in a palette using Euclidean distance.
pub fn find_nearest_color(pixel: Rgb, palette: &[Rgb]) -> usize {
    let mut min_dist = u32::MAX;
    let mut best_idx = 0;

    for (i, &color) in palette.iter().enumerate() {
        let dr = pixel.r as i32 - color.r as i32;
        let dg = pixel.g as i32 - color.g as i32;
        let db = pixel.b as i32 - color.b as i32;
        let dist = (dr * dr + dg * dg + db * db) as u32;

        if dist < min_dist {
            min_dist = dist;
            best_idx = i;
        }
    }

    best_idx
}
