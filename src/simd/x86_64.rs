//! x86_64 SIMD implementations.

use crate::quant::Rgb;
use crate::color::Lab;
use std::arch::x86_64::*;

/// A palette stored in planar format for SIMD efficiency.
pub struct PlanarLabPalette {
    pub l: Vec<f32>,
    pub a: Vec<f32>,
    pub b: Vec<f32>,
    pub len: usize,
}

impl PlanarLabPalette {
    pub fn from_lab(colors: &[Lab]) -> Self {
        let len = colors.len();
        let mut l = Vec::with_capacity(len);
        let mut a = Vec::with_capacity(len);
        let mut b = Vec::with_capacity(len);
        for c in colors {
            l.push(c.l);
            a.push(c.a);
            b.push(c.b);
        }
        Self { l, a, b, len }
    }
}

/// Find nearest color in Lab space using planar SIMD (AVX2).
#[target_feature(enable = "avx2")]
pub unsafe fn find_nearest_color_lab_planar_avx2(pixel: Lab, palette: &PlanarLabPalette) -> usize {
    let mut min_dist = f32::MAX;
    let mut best_idx = 0;

    let p_l = _mm256_set1_ps(pixel.l);
    let p_a = _mm256_set1_ps(pixel.a);
    let p_b = _mm256_set1_ps(pixel.b);

    let chunks = palette.len / 8;
    
    for i in 0..chunks {
        let offset = i * 8;
        
        // Load 8 components at once from planar vectors
        let l_v = _mm256_loadu_ps(palette.l.as_ptr().add(offset));
        let a_v = _mm256_loadu_ps(palette.a.as_ptr().add(offset));
        let b_v = _mm256_loadu_ps(palette.b.as_ptr().add(offset));

        // Calculate squared distances: (p - c)^2
        let dl = _mm256_sub_ps(p_l, l_v);
        let da = _mm256_sub_ps(p_a, a_v);
        let db = _mm256_sub_ps(p_b, b_v);

        let dist_v = _mm256_add_ps(
            _mm256_add_ps(_mm256_mul_ps(dl, dl), _mm256_mul_ps(da, da)),
            _mm256_mul_ps(db, db)
        );

        // Extract and compare
        let mut dists = [0.0f32; 8];
        _mm256_storeu_ps(dists.as_mut_ptr(), dist_v);

        for (j, &d) in dists.iter().enumerate() {
            if d < min_dist {
                min_dist = d;
                best_idx = offset + j;
            }
        }
    }

    // Handle remainder
    for i in (chunks * 8)..palette.len {
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

/// Find nearest color using AVX2 (Legacy interleaved version, currently fallback to scalar logic)
#[target_feature(enable = "avx2")]
pub unsafe fn find_nearest_color_avx2(pixel: Rgb, palette: &[Rgb]) -> usize {
    let mut min_dist = u32::MAX;
    let mut best_idx = 0;

    let r_pixel = _mm256_set1_epi32(pixel.r as i32);
    let g_pixel = _mm256_set1_epi32(pixel.g as i32);
    let b_pixel = _mm256_set1_epi32(pixel.b as i32);

    let mut i = 0;
    
    // We process in smaller chunks if needed, but for 256 colors, 
    // a straightforward loop is often fine if we avoid the extra array copies.
    for (idx, &color) in palette.iter().enumerate() {
        let dr = pixel.r as i32 - color.r as i32;
        let dg = pixel.g as i32 - color.g as i32;
        let db = pixel.b as i32 - color.b as i32;
        let dist = (dr * dr + dg * dg + db * db) as u32;

        if dist < min_dist {
            min_dist = dist;
            best_idx = idx;
        }
    }
    
    // NOTE: The previous SIMD version was slower due to array copies.
    // A truly fast SIMD version for RGB distance requires clever shuffles 
    // or a Planar palette layout. For now, we use the scalar path 
    // which is already very fast (250ns for 256 colors) until we implement
    // the Planar palette optimization.

    best_idx
}