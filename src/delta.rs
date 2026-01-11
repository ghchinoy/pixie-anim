//! Inter-frame Delta Compression.

use crate::quant::{DitherType, Rgb};

/// Represents the difference between two consecutive frames.
#[derive(Debug, Default)]
pub struct FrameDelta {
    /// X coordinate of the delta bounding box.
    pub x: u16,
    /// Y coordinate of the delta bounding box.
    pub y: u16,
    /// Width of the delta bounding box.
    pub width: u16,
    /// Height of the delta bounding box.
    pub height: u16,
    /// Palette indices for the pixels within the bounding box.
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

/// Options for delta compression.
/// Options for configuring the delta compression engine.
pub struct DeltaOptions<'a> {
    /// Width of the frame.
    pub width: u16,
    /// Height of the frame.
    pub height: u16,
    /// The global palette to use for indexing.
    pub palette: &'a [Rgb],
    /// The index to use for transparent pixels.
    pub transparent_idx: u8,
    /// Squared perceptual threshold for "fuzzy" equality.
    pub fuzz_threshold: u32,
    /// Type of dithering to apply to opaque pixels.
    pub dither: DitherType,
    /// Strength of the dithering effect (0.0 to 1.0).
    pub dither_strength: f32,
}

/// Finds the smallest bounding box and maps pixels to transparent if they are "close enough"
/// to the previous frame's color.
///
/// # Arguments
/// * `curr_pixels` - Pixels of the current frame
/// * `prev_pixels` - Pixels of the previous frame
/// * `prev_indices` - Palette indices used in the previous frame at these coordinates
/// * `options` - Delta compression configuration
pub fn find_delta_fuzzy(
    curr_pixels: &[Rgb],
    prev_pixels: &[Rgb],
    prev_indices: Option<&[u8]>,
    options: &DeltaOptions,
) -> Option<FrameDelta> {
    if curr_pixels.len() != prev_pixels.len() {
        return None;
    }

    let mut min_x = options.width;
    let mut max_x = 0;
    let mut min_y = options.height;
    let mut max_y = 0;
    let mut changed = false;

    // 1. Find bounding box using fuzzy equality
    for y in 0..options.height {
        for x in 0..options.width {
            let idx = (y as usize * options.width as usize) + x as usize;
            if rgb_dist_sq(curr_pixels[idx], prev_pixels[idx]) > options.fuzz_threshold {
                if x < min_x {
                    min_x = x;
                }
                if x > max_x {
                    max_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
                if y > max_y {
                    max_y = y;
                }
                changed = true;
            }
        }
    }

    if !changed {
        return None;
    }

    let delta_width = max_x - min_x + 1;
    let delta_height = max_y - min_y + 1;

    // 2. Map pixels to indices

    // Prepare Blue Noise if needed
    let lab_palette = if options.dither == DitherType::BlueNoise {
        let lp: Vec<crate::color::Lab> = options
            .palette
            .iter()
            .map(|p| crate::color::rgb_to_lab(p.r, p.g, p.b))
            .collect();
        let pp = crate::simd::PlanarLabPalette::from_lab(&lp);
        Some((lp, pp))
    } else {
        None
    };

    #[cfg(feature = "rayon")]
    use rayon::prelude::*;

    #[cfg(feature = "rayon")]
    let delta_indices: Vec<u8> = (min_y..=max_y)
        .into_par_iter()
        .flat_map(|y| {
            let lab_palette_ref = lab_palette.as_ref();
            (min_x..=max_x).into_par_iter().map(move |x| {
                let idx = (y as usize * options.width as usize) + x as usize;
                // If it's close enough to the previous pixel, make it transparent
                if rgb_dist_sq(curr_pixels[idx], prev_pixels[idx]) <= options.fuzz_threshold {
                    options.transparent_idx
                } else {
                    // TEMPORAL DENOISING:
                    // If we used an index in the previous frame, check if that same color
                    // is "close enough" to our current pixel. Reusing indices is LZW-friendly.
                    if let Some(prev_idx_buffer) = prev_indices {
                        let prev_idx = prev_idx_buffer[idx];
                        if prev_idx != options.transparent_idx {
                            let prev_color = options.palette[prev_idx as usize];
                            // Use a tighter threshold for index-reuse than for transparency
                            if rgb_dist_sq(curr_pixels[idx], prev_color)
                                <= options.fuzz_threshold / 2
                            {
                                return prev_idx;
                            }
                        }
                    }

                    match options.dither {
                        DitherType::BlueNoise => {
                            if let Some((_, pp)) = lab_palette_ref {
                                let (ol, oa, ob) = crate::quant::dither::get_blue_noise_offset(
                                    x,
                                    y,
                                    options.dither_strength,
                                );
                                let p = curr_pixels[idx];
                                let mut lab = crate::color::rgb_to_lab(p.r, p.g, p.b);
                                lab.l = (lab.l + ol).clamp(0.0, 100.0);
                                lab.a = (lab.a + oa).clamp(-128.0, 127.0);
                                lab.b = (lab.b + ob).clamp(-128.0, 127.0);
                                crate::simd::find_nearest_color_lab(lab, pp) as u8
                            } else {
                                crate::simd::find_nearest_color(curr_pixels[idx], options.palette)
                                    as u8
                            }
                        }
                        _ => {
                            crate::simd::find_nearest_color(curr_pixels[idx], options.palette) as u8
                        }
                    }
                }
            })
        })
        .collect();

    #[cfg(not(feature = "rayon"))]
    let delta_indices: Vec<u8> = (min_y..=max_y)
        .flat_map(|y| {
            let lab_palette_ref = lab_palette.as_ref();
            (min_x..=max_x).map(move |x| {
                let idx = (y as usize * options.width as usize) + x as usize;
                if rgb_dist_sq(curr_pixels[idx], prev_pixels[idx]) <= options.fuzz_threshold {
                    options.transparent_idx
                } else {
                    // TEMPORAL DENOISING:
                    if let Some(prev_idx_buffer) = prev_indices {
                        let prev_idx = prev_idx_buffer[idx];
                        if prev_idx != options.transparent_idx {
                            let prev_color = options.palette[prev_idx as usize];
                            if rgb_dist_sq(curr_pixels[idx], prev_color)
                                <= options.fuzz_threshold / 2
                            {
                                return prev_idx;
                            }
                        }
                    }

                    match options.dither {
                        DitherType::BlueNoise => {
                            if let Some((_, pp)) = lab_palette_ref {
                                let (ol, oa, ob) = crate::quant::dither::get_blue_noise_offset(
                                    x,
                                    y,
                                    options.dither_strength,
                                );
                                let p = curr_pixels[idx];
                                let mut lab = crate::color::rgb_to_lab(p.r, p.g, p.b);
                                lab.l = (lab.l + ol).clamp(0.0, 100.0);
                                lab.a = (lab.a + oa).clamp(-128.0, 127.0);
                                lab.b = (lab.b + ob).clamp(-128.0, 127.0);
                                crate::simd::find_nearest_color_lab(lab, pp) as u8
                            } else {
                                crate::simd::find_nearest_color(curr_pixels[idx], options.palette)
                                    as u8
                            }
                        }
                        _ => {
                            crate::simd::find_nearest_color(curr_pixels[idx], options.palette) as u8
                        }
                    }
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
