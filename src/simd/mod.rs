//! SIMD acceleration module for performance-critical operations.

use crate::quant::Rgb;
#[cfg(target_arch = "x86_64")]
use std::sync::LazyLock;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::PlanarLabPalette;

/// Fallback scalar implementations.
pub mod fallback;

#[cfg(target_arch = "x86_64")]
enum SimdLevel {
    Scalar,
    Avx2,
}

#[cfg(target_arch = "x86_64")]
static SIMD_LEVEL: LazyLock<SimdLevel> = LazyLock::new(|| {
    if is_x86_feature_detected!("avx2") {
        SimdLevel::Avx2
    } else {
        SimdLevel::Scalar
    }
});

/// A color palette stored in a planar layout for SIMD efficiency.
#[cfg(not(target_arch = "x86_64"))]
pub struct PlanarLabPalette {
    /// L components
    pub l: Vec<f32>,
    /// a components
    pub a: Vec<f32>,
    /// b components
    pub b: Vec<f32>,
    /// Number of colors in the palette
    pub len: usize,
}

#[cfg(not(target_arch = "x86_64"))]
impl PlanarLabPalette {
    /// Creates a planar palette from a slice of Lab colors.
    pub fn from_lab(colors: &[crate::color::Lab]) -> Self {
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

/// Find the index of the nearest color in Lab space using the best available implementation.
#[inline]
pub fn find_nearest_color_lab(pixel: crate::color::Lab, palette: &PlanarLabPalette) -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        match *SIMD_LEVEL {
            SimdLevel::Avx2 => unsafe {
                x86_64::find_nearest_color_lab_planar_avx2(pixel, palette)
            },
            SimdLevel::Scalar => fallback::find_nearest_color_lab_planar(pixel, palette),
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    fallback::find_nearest_color_lab_planar(pixel, palette)
}

/// Find the index of the nearest color in a palette using Euclidean distance.
#[inline]
pub fn find_nearest_color(pixel: Rgb, palette: &[Rgb]) -> usize {
    #[cfg(target_arch = "x86_64")]
    {
        match *SIMD_LEVEL {
            SimdLevel::Avx2 => unsafe { x86_64::find_nearest_color_avx2(pixel, palette) },
            SimdLevel::Scalar => fallback::find_nearest_color(pixel, palette),
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    fallback::find_nearest_color(pixel, palette)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_vs_scalar() {
        let palette = vec![
            Rgb { r: 255, g: 0, b: 0 },
            Rgb { r: 0, g: 255, b: 0 },
            Rgb { r: 0, g: 0, b: 255 },
            Rgb {
                r: 128,
                g: 128,
                b: 128,
            },
            Rgb {
                r: 10,
                g: 10,
                b: 10,
            },
            Rgb {
                r: 200,
                g: 200,
                b: 200,
            },
            Rgb {
                r: 50,
                g: 150,
                b: 250,
            },
            Rgb {
                r: 250,
                g: 150,
                b: 50,
            },
            Rgb {
                r: 20,
                g: 30,
                b: 40,
            },
        ];
        let pixel = Rgb {
            r: 240,
            g: 10,
            b: 10,
        };

        let scalar_idx = fallback::find_nearest_color(pixel, &palette);
        let simd_idx = find_nearest_color(pixel, &palette);

        assert_eq!(scalar_idx, simd_idx);
        assert_eq!(scalar_idx, 0); // Should be Red
    }
}
