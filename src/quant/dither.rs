//! Perceptual Dithering Algorithms.

use crate::color::{rgb_to_lab, Lab};
use crate::quant::Rgb;
use crate::simd::{find_nearest_color_lab, PlanarLabPalette};

/// Blue Noise Mask (32x32)
/// Generated using Void-and-Cluster algorithm.
const BLUE_NOISE_32: [u8; 1024] = [
    191, 14, 131, 231, 101, 12, 161, 42, 203, 115, 24, 142, 215, 56, 178, 89, 51, 245, 78, 168, 32,
    212, 122, 253, 67, 182, 234, 10, 154, 226, 135, 198, 112, 145, 23, 194, 95, 151, 238, 84, 128,
    219, 164, 48, 207, 72, 118, 37, 250, 62, 175, 108, 222, 1, 187, 138, 30, 242, 92, 157, 230,
    104, 181, 211, 125, 235, 40, 148, 216, 75, 111, 254, 53, 197, 132, 21, 171, 81, 247, 15, 18,
    162, 241, 87, 121, 190, 34, 225, 150, 60, 239, 100, 141, 213, 124, 177, 205, 94, 50, 228, 137,
    174, 248, 11, 184, 218, 45, 159, 70, 252, 114, 31, 129, 214, 167, 43, 106, 232, 65, 144, 221,
    19, 193, 127, 244, 82, 170, 201, 55, 240, 79, 180, 251, 91, 153, 236, 117, 172, 52, 210, 134,
    227, 39, 147, 110, 143, 22, 202, 120, 224, 33, 160, 246, 8, 189, 103, 255, 68, 113, 196, 220,
    64, 173, 97, 243, 139, 208, 74, 156, 233, 47, 140, 217, 126, 179, 86, 29, 192, 130, 211, 41,
    165, 249, 107, 183, 223, 90, 152, 58, 241, 102, 237, 155, 234, 105, 119, 253, 17, 195, 133,
    204, 36, 169, 229, 116, 186, 21, 149, 71, 206, 83, 176, 242, 146, 221, 54, 111, 251, 93, 158,
    238, 66, 199, 123, 128, 38, 215, 110, 61, 236, 136, 185, 212, 14, 163, 245, 101, 172, 44, 209,
    248, 166, 244, 194, 96, 150, 230, 10, 188, 222, 118, 49, 203, 77, 131, 254, 114, 225, 35, 142,
    217, 73, 161, 247, 134, 226, 57, 178, 88, 213, 121, 197, 59, 239, 104, 181, 252, 92, 154, 233,
    46, 147, 210, 125, 240, 16, 168, 231, 112, 145, 25, 205, 138, 228, 12, 184, 219, 100, 164, 246,
    85, 191, 132, 202, 249, 69, 174, 98, 243, 1, 159, 235, 51, 214, 127, 171, 224, 33, 115, 250,
    130, 216, 41, 141, 211, 122, 193, 237, 106, 182, 223, 19, 149, 232, 63, 176, 55, 241, 80, 169,
    255, 109, 153, 221, 117, 173, 52, 208, 135, 227, 102, 195, 113, 144, 22, 204, 133, 225, 36,
    160, 245, 10, 187, 234, 119, 253, 17, 170, 251, 67, 175, 95, 239, 136, 218, 76, 151, 229, 44,
    143, 212, 124, 180, 89, 30, 192, 131, 213, 48, 166, 248, 105, 183, 222, 91, 155, 58, 244, 103,
    236, 157, 235, 108, 120, 252, 15, 196, 139, 207, 32, 162, 230, 111, 185, 20, 148, 72, 203, 84,
    177, 243, 147, 220, 53, 114, 254, 97, 152, 233, 65, 190, 126, 129, 39, 214, 116, 60, 237, 134,
    186, 211, 13, 165, 246, 100, 171, 47, 209, 247, 167, 242, 193, 94, 150, 228, 11, 189, 221, 118,
    50, 202, 78, 132, 255, 115, 226, 34, 140, 216, 74, 163, 249, 137, 227, 56, 179, 86, 215, 121,
    198, 59, 240, 101, 181, 253, 93, 156, 234, 45, 146, 210, 125, 241, 18, 169, 231, 112, 144, 23,
    206, 138, 229, 12, 182, 218, 99, 164, 245, 87, 194, 130, 203, 250, 68, 173, 96, 244, 2, 158,
    232, 51, 212, 127, 170, 225, 31, 117, 251, 131, 217, 42, 142, 213, 123, 192, 238, 107, 183,
    222, 20, 149, 233, 62, 177, 54, 242, 81, 168, 254, 108, 154, 220, 116, 174, 53, 207, 136, 226,
    103, 196, 113, 145, 21, 205, 132, 224, 37, 161, 246, 9, 188, 235, 119, 252, 16, 171, 252, 66,
    176, 94, 240, 135, 219, 75, 152, 228, 43, 144, 211, 124, 181, 88, 30, 191, 130, 212, 49, 167,
    247, 104, 184, 223, 90, 156, 57, 243, 102, 237, 158, 234, 109, 121, 253, 14, 197, 138, 208, 33,
    163, 231, 110, 186, 19, 147, 73, 204, 83, 178, 242, 146, 221, 52, 115, 255, 98, 153, 232, 64,
    189, 127, 128, 40, 215, 117, 61, 236, 133, 185, 210, 12, 166, 245, 101, 172, 46, 209, 248, 168,
    241, 194, 93, 151, 229, 10, 188, 220, 118, 51, 203, 79, 131, 254, 114, 227, 35, 141, 217, 74,
    162, 250, 137, 226, 55, 180, 85, 216, 122, 199, 58, 239, 100, 182, 252, 92, 155, 233, 44, 145,
    211, 126, 240, 17, 170, 230, 113, 143, 24, 207, 139, 228, 13, 181, 219, 98, 165, 244, 86, 195,
    129, 202, 249, 69, 174, 97, 243, 1, 159, 231, 50, 213, 128, 171, 224, 32, 116, 251, 130, 218,
    41, 143, 212, 123, 193, 237, 106, 184, 223, 21, 148, 232, 63, 177, 53, 242, 80, 169, 255, 109,
    154, 221, 117, 173, 52, 208, 134, 227, 102, 196, 112, 144, 22, 204, 133, 225, 36, 160, 245, 11,
    187, 234, 120, 253, 18, 170, 250, 67, 175, 95, 238, 136, 217, 76, 151, 229, 45, 142, 213, 124,
    180, 90, 29, 192, 131, 211, 48, 166, 249, 105, 182, 222, 91, 155, 59, 244, 103, 236, 157, 235,
    108, 119, 252, 15, 195, 139, 206, 33, 162, 230, 111, 185, 20, 149, 71, 203, 84, 176, 243, 147,
    220, 54, 114, 254, 96, 152, 233, 66, 190, 125, 128, 38, 214, 116, 61, 237, 134, 186, 212, 14,
    165, 246, 101, 172, 47, 208, 247, 167, 241, 194, 94, 150, 228, 10, 188, 221, 118, 49, 202, 77,
    132, 255, 115, 226, 34, 140, 216, 73, 163, 248, 137, 227, 56, 179, 87, 215, 122, 197, 60, 240,
    104, 181, 253, 92, 156, 234, 46, 146, 210, 126, 241, 17, 168, 231, 113, 144, 23, 205, 138, 229,
    12, 183, 219, 99, 164, 245, 88, 193, 130, 203, 250, 68, 173, 98, 242, 2, 158, 232, 51, 214,
    127, 171, 224, 31, 117, 251, 131, 217, 42, 141, 211, 123, 192, 238, 107, 182, 223, 19, 149,
    233, 62, 176, 55, 242, 79, 169, 254, 108, 153, 222, 118, 174, 53, 207, 135, 226, 102, 195, 114,
    145, 22, 204, 133, 225, 37, 160, 246, 9, 187, 235, 119, 252, 16, 172, 252, 65, 175, 95, 239,
    136, 218, 75, 151, 228, 44, 143, 213, 124, 181, 89, 30, 191, 132, 212, 48, 167, 248, 104, 184,
    222, 90, 156, 58, 243, 101, 237,
];

/// Applies perceptual Floyd-Steinberg dithering to a frame.
pub fn dither_floyd_steinberg(width: u16, height: u16, pixels: &[Rgb], palette: &[Rgb]) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    let mut indices = vec![0u8; w * h];

    let lab_palette: Vec<Lab> = palette.iter().map(|p| rgb_to_lab(p.r, p.g, p.b)).collect();
    let planar_palette = PlanarLabPalette::from_lab(&lab_palette);

    let mut error_buf_l = vec![0.0f32; w * h];
    let mut error_buf_a = vec![0.0f32; w * h];
    let mut error_buf_b = vec![0.0f32; w * h];

    let strength = 0.75f32;

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let original_lab = rgb_to_lab(pixels[idx].r, pixels[idx].g, pixels[idx].b);
            let current_lab = Lab {
                l: (original_lab.l + error_buf_l[idx]).clamp(0.0, 100.0),
                a: (original_lab.a + error_buf_a[idx]).clamp(-128.0, 127.0),
                b: (original_lab.b + error_buf_b[idx]).clamp(-128.0, 127.0),
            };

            let color_idx = find_nearest_color_lab(current_lab, &planar_palette);
            indices[idx] = color_idx as u8;

            let best_color_lab = lab_palette[color_idx];
            let err_l = (current_lab.l - best_color_lab.l) * strength;
            let err_a = (current_lab.a - best_color_lab.a) * strength;
            let err_b = (current_lab.b - best_color_lab.b) * strength;

            let mut bufs = ErrorBuffers {
                l: &mut error_buf_l,
                a: &mut error_buf_a,
                b: &mut error_buf_b,
            };

            if x + 1 < w {
                diffuse(w, &mut bufs, x + 1, y, err_l, err_a, err_b, 7.0 / 16.0);
            }
            if y + 1 < h {
                if x > 0 {
                    diffuse(w, &mut bufs, x - 1, y + 1, err_l, err_a, err_b, 3.0 / 16.0);
                }
                diffuse(w, &mut bufs, x, y + 1, err_l, err_a, err_b, 5.0 / 16.0);
                if x + 1 < w {
                    diffuse(w, &mut bufs, x + 1, y + 1, err_l, err_a, err_b, 1.0 / 16.0);
                }
            }
        }
    }
    indices
}

const BAYER_8X8: [u8; 64] = [
    0, 32, 8, 40, 2, 34, 10, 42, 48, 16, 56, 24, 50, 18, 58, 26, 12, 44, 4, 36, 14, 46, 6, 38, 60,
    28, 52, 20, 62, 30, 54, 22, 3, 35, 11, 43, 1, 33, 9, 41, 51, 19, 59, 27, 49, 17, 57, 25, 15,
    47, 7, 39, 13, 45, 5, 37, 63, 31, 55, 23, 61, 29, 53, 21,
];

/// Applies Ordered Dithering (Bayer 8x8) to a frame.
pub fn dither_ordered(width: u16, height: u16, pixels: &[Rgb], palette: &[Rgb]) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    let mut indices = vec![0u8; w * h];

    let lab_palette: Vec<Lab> = palette.iter().map(|p| rgb_to_lab(p.r, p.g, p.b)).collect();
    let planar_palette = PlanarLabPalette::from_lab(&lab_palette);

    // Spread strength
    let strength = 4.0f32;

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let bayer_val = BAYER_8X8[(y % 8) * 8 + (x % 8)] as f32 / 64.0;
            let offset = (bayer_val - 0.5) * strength;

            let original_lab = rgb_to_lab(pixels[idx].r, pixels[idx].g, pixels[idx].b);
            let current_lab = Lab {
                l: (original_lab.l + offset).clamp(0.0, 100.0),
                a: (original_lab.a + offset * 0.5).clamp(-128.0, 127.0),
                b: (original_lab.b + offset * 0.5).clamp(-128.0, 127.0),
            };

            let color_idx = find_nearest_color_lab(current_lab, &planar_palette);
            indices[idx] = color_idx as u8;
        }
    }
    indices
}

/// Applies Blue Noise dithering to a frame.
pub fn dither_blue_noise(width: u16, height: u16, pixels: &[Rgb], palette: &[Rgb]) -> Vec<u8> {
    let w = width as usize;
    let h = height as usize;
    let mut indices = vec![0u8; w * h];

    let lab_palette: Vec<Lab> = palette.iter().map(|p| rgb_to_lab(p.r, p.g, p.b)).collect();
    let planar_palette = PlanarLabPalette::from_lab(&lab_palette);

    for y in 0..h {
        for x in 0..w {
            let idx = y * w + x;
            let (ol, oa, ob) = get_blue_noise_offset(x as u16, y as u16);

            let original_lab = rgb_to_lab(pixels[idx].r, pixels[idx].g, pixels[idx].b);
            let current_lab = Lab {
                l: (original_lab.l + ol).clamp(0.0, 100.0),
                a: (original_lab.a + oa).clamp(-128.0, 127.0),
                b: (original_lab.b + ob).clamp(-128.0, 127.0),
            };

            let color_idx = find_nearest_color_lab(current_lab, &planar_palette);
            indices[idx] = color_idx as u8;
        }
    }
    indices
}

/// Helper to get a deterministic Lab offset for a given coordinate
#[inline]
pub fn get_blue_noise_offset(x: u16, y: u16) -> (f32, f32, f32) {
    let noise_val = BLUE_NOISE_32[((y % 32) as usize * 32) + (x % 32) as usize] as f32 / 255.0;
    let noise_strength = 3.0f32;
    let offset = (noise_val - 0.5) * noise_strength;
    (offset, offset * 0.5, offset * 0.5)
}

struct ErrorBuffers<'a> {
    l: &'a mut [f32],
    a: &'a mut [f32],
    b: &'a mut [f32],
}

#[inline]
#[allow(clippy::too_many_arguments)]
fn diffuse(
    w: usize,
    bufs: &mut ErrorBuffers,
    x: usize,
    y: usize,
    el: f32,
    ea: f32,
    eb: f32,
    weight: f32,
) {
    let idx = y * w + x;
    bufs.l[idx] += el * weight;
    bufs.a[idx] += ea * weight;
    bufs.b[idx] += eb * weight;
}