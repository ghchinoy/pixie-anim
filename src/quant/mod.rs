//! Quantization algorithms for GIF.
//!
//! Implements K-Means clustering for optimal palette generation.

pub mod dither;
pub mod zeng;

use crate::error::Result;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// Representation of an RGB color.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    /// Red channel (0-255)
    pub r: u8,
    /// Green channel (0-255)
    pub g: u8,
    /// Blue channel (0-255)
    pub b: u8,
}

/// Dithering algorithms supported by Pixie-Anim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DitherType {
    /// No dithering (sharp edges, potential banding)
    None,
    /// Spatial error diffusion (Floyd-Steinberg)
    FloydSteinberg,
    /// Deterministic perceptual noise
    BlueNoise,
    /// Matrix-based deterministic dithering (Bayer 8x8)
    Ordered,
}

/// A collection of colors representing a GIF palette.
pub struct Palette {
    /// The colors in the palette.
    pub colors: Vec<Rgb>,
}

/// Result of quantization including the palette and a mapping from
/// the intermediate refinement indices to the final reordered indices.
pub struct QuantizationResult {
    /// The final generated palette.
    pub palette: Palette,
    /// Mapping from intermediate indices to final reordered indices.
    pub index_mapping: Vec<u8>,
}

/// Common trait for color quantizers.
pub trait Quantizer {
    /// Quantizes a slice of pixels into a palette of at most `max_colors`.
    fn quantize(&self, pixels: &[Rgb], max_colors: usize) -> Result<QuantizationResult>;
}

/// A K-Means++ based quantizer that operates in CIELAB space.
pub struct KMeansQuantizer {
    /// Maximum number of iterations for the clustering algorithm.
    pub max_iterations: usize,
    /// Pixel sampling rate (e.g. 10 for 10% sampling)
    pub sample_rate: usize,
    /// Whether to use dithering during index generation.
    pub dither: bool,
}

impl KMeansQuantizer {
    /// Creates a new KMeansQuantizer with default settings.
    pub fn new(max_iterations: usize) -> Self {
        Self {
            max_iterations,
            sample_rate: 10,
            dither: true,
        }
    }

    fn distance_sq(c1: crate::color::Lab, c2: crate::color::Lab) -> f32 {
        crate::color::lab_distance_sq(c1, c2)
    }

    /// K-Means++ initialization for Lab centroids
    fn initialize_centroids(
        pixels: &[crate::color::Lab],
        max_colors: usize,
    ) -> Vec<crate::color::Lab> {
        if pixels.is_empty() {
            return Vec::new();
        }

        let mut centroids = Vec::with_capacity(max_colors);
        centroids.push(pixels[0]);

        let mut min_distances = vec![f32::MAX; pixels.len()];

        while centroids.len() < max_colors {
            let last_centroid = centroids.last().unwrap();
            let mut total_dist: f64 = 0.0;

            for (i, &pixel) in pixels.iter().enumerate() {
                let dist = Self::distance_sq(pixel, *last_centroid);
                if dist < min_distances[i] {
                    min_distances[i] = dist;
                }
                total_dist += min_distances[i] as f64;
            }

            if total_dist == 0.0 {
                break;
            }

            let mut best_pixel_idx = 0;
            let mut max_d = -1.0;
            for (i, &d) in min_distances.iter().enumerate() {
                if d > max_d {
                    max_d = d;
                    best_pixel_idx = i;
                }
            }
            centroids.push(pixels[best_pixel_idx]);
        }

        centroids
    }
}

impl Quantizer for KMeansQuantizer {
    fn quantize(&self, pixels: &[Rgb], max_colors: usize) -> Result<QuantizationResult> {
        if pixels.is_empty() {
            return Ok(QuantizationResult {
                palette: Palette { colors: Vec::new() },
                index_mapping: Vec::new(),
            });
        }

        // 1. Sub-sample pixels and convert to CIELAB for perceptual refinement
        let sampled_pixels: Vec<crate::color::Lab> = pixels
            .iter()
            .step_by(self.sample_rate)
            .map(|p| crate::color::rgb_to_lab(p.r, p.g, p.b))
            .collect();

        // 2. Initialize centroids using K-Means++ logic on sampled Lab pixels
        let mut centroids = Self::initialize_centroids(&sampled_pixels, max_colors);

        let mut assignments = vec![0usize; sampled_pixels.len()];

        for _ in 0..self.max_iterations {
            let mut changed = false;

            // 3. Assignment step (Perceptual Distance via CIELAB Planar SIMD)
            let planar_centroids = crate::simd::PlanarLabPalette::from_lab(&centroids);

            #[cfg(feature = "rayon")]
            let new_assignments: Vec<usize> = sampled_pixels
                .par_iter()
                .map(|&pixel_lab| crate::simd::find_nearest_color_lab(pixel_lab, &planar_centroids))
                .collect();

            #[cfg(not(feature = "rayon"))]
            let new_assignments: Vec<usize> = sampled_pixels
                .iter()
                .map(|&pixel_lab| crate::simd::find_nearest_color_lab(pixel_lab, &planar_centroids))
                .collect();

            if assignments != new_assignments {
                assignments = new_assignments;
                changed = true;
            }

            if !changed {
                break;
            }

            // 4. Update step
            let mut sums = vec![(0.0f32, 0.0f32, 0.0f32, 0usize); centroids.len()];
            for (i, &pixel_lab) in sampled_pixels.iter().enumerate() {
                let a = assignments[i];
                sums[a].0 += pixel_lab.l;
                sums[a].1 += pixel_lab.a;
                sums[a].2 += pixel_lab.b;
                sums[a].3 += 1;
            }

            for (c_idx, sum) in sums.iter().enumerate() {
                if sum.3 > 0 {
                    centroids[c_idx] = crate::color::Lab {
                        l: sum.0 / sum.3 as f32,
                        a: sum.1 / sum.3 as f32,
                        b: sum.2 / sum.3 as f32,
                    };
                }
            }
        }

        // Convert Lab centroids back to RGB for the palette
        let rgb_centroids: Vec<Rgb> = centroids
            .iter()
            .map(|&lab| {
                let mut min_dist = f32::MAX;
                let mut best_rgb = Rgb { r: 0, g: 0, b: 0 };
                // Sample more densely for the final palette back-mapping
                for &p in pixels.iter().step_by(10) {
                    let p_lab = crate::color::rgb_to_lab(p.r, p.g, p.b);
                    let dist = crate::color::lab_distance_sq(lab, p_lab);
                    if dist < min_dist {
                        min_dist = dist;
                        best_rgb = p;
                    }
                }
                best_rgb
            })
            .collect();

        // 5. Zeng Palette Reordering for maximum LZW compressibility
        let (final_palette, index_mapping) = zeng::reorder_palette(&Palette {
            colors: rgb_centroids,
        });

        Ok(QuantizationResult {
            palette: final_palette,
            index_mapping,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmeans_basic() {
        let pixels = vec![
            Rgb { r: 255, g: 0, b: 0 },
            Rgb { r: 254, g: 1, b: 0 },
            Rgb { r: 0, g: 255, b: 0 },
            Rgb { r: 1, g: 254, b: 0 },
        ];
        let mut quantizer = KMeansQuantizer::new(10);
        quantizer.sample_rate = 1;
        let result = quantizer.quantize(&pixels, 2).unwrap();
        assert_eq!(result.palette.colors.len(), 2);
    }
}
