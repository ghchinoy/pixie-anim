//! WebAssembly bindings for Pixie-Anim.

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[global_allocator]
static ALLOC: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };

use crate::delta::DeltaOptions;
use crate::gif::{GifOptions, GifWriter, ImageDescriptor};
use crate::quant::{DitherType, KMeansQuantizer, Quantizer, Rgb};
use image::AnimationDecoder;
use image::codecs::gif::GifDecoder;
use wasm_bindgen::prelude::*;

/// Decodes a GIF into raw RGBA pixels.
/// Returns [width, height, num_frames, ...pixels]
#[wasm_bindgen(js_name = "decodeGif")]
pub fn decode_gif(data: &[u8]) -> Result<Vec<u8>, JsError> {
    let decoder = GifDecoder::new(data).map_err(|e| JsError::new(&e.to_string()))?;
    let frames = decoder
        .into_frames()
        .collect_frames()
        .map_err(|e| JsError::new(&e.to_string()))?;

    if frames.is_empty() {
        return Err(JsError::new("No frames found in GIF"));
    }

    let width = frames[0].buffer().width() as u16;
    let height = frames[0].buffer().height() as u16;
    let num_frames = frames.len() as u32;

    // Calculate average delay
    let total_delay: u32 = frames
        .iter()
        .map(|f| {
            let (num, den) = f.delay().numer_denom_ms();
            if den == 0 {
                0
            } else {
                num / den
            }
        })
        .sum();
    let avg_delay_ms = (total_delay / num_frames).max(10); // Minimum 10ms (100fps)

    let mut output = Vec::new();
    // Metadata header for JS: [width(2), height(2), num_frames(4), avg_delay_ms(4)]
    output.extend_from_slice(&width.to_le_bytes());
    output.extend_from_slice(&height.to_le_bytes());
    output.extend_from_slice(&num_frames.to_le_bytes());
    output.extend_from_slice(&avg_delay_ms.to_le_bytes());

    for frame in frames {
        output.extend_from_slice(frame.buffer().as_raw());
    }

    Ok(output)
}

/// Encodes a sequence of frames into an optimized GIF.
///
/// # Arguments
/// * `data` - Flat buffer of RGBA pixels for all frames
/// * `width` - Image width
/// * `height` - Image height
/// * `num_frames` - Number of frames in the sequence
/// * `fps` - Target frames per second
/// * `quality` - K-Means iterations (default 10)
/// * `lossy` - LZW neighbor matching (0-20)
/// * `fuzz` - Perceptual transparency threshold (0-100)
/// * `dither` - Dithering algorithm (0=None, 1=Floyd, 2=Blue, 3=Ordered)
#[wasm_bindgen(js_name = "encodeGif")]
#[allow(clippy::too_many_arguments)]
pub fn encode_gif(
    data: &[u8],
    width: u16,
    height: u16,
    num_frames: u32,
    fps: f32,
    quality: usize,
    lossy: u8,
    fuzz: u32,
    dither: u8,
    dither_strength: f32,
) -> Result<Vec<u8>, JsError> {
...
            let indices = match dither_type {
                DitherType::FloydSteinberg => crate::quant::dither::dither_floyd_steinberg(
                    width,
                    height,
                    &curr_pixels,
                    &global_palette,
                    dither_strength,
                ),
                DitherType::BlueNoise => crate::quant::dither::dither_blue_noise(
                    width,
                    height,
                    &curr_pixels,
                    &global_palette,
                    dither_strength,
                ),
                DitherType::Ordered => crate::quant::dither::dither_ordered(
                    width,
                    height,
                    &curr_pixels,
                    &global_palette,
                    dither_strength,
                ),
...                _ => {
                    #[cfg(feature = "rayon")]
                    {
                        use rayon::prelude::*;
                        curr_pixels
                            .par_iter()
                            .map(|&p| crate::simd::find_nearest_color(p, &global_palette) as u8)
                            .collect()
                    }
                    #[cfg(not(feature = "rayon"))]
                    {
                        curr_pixels
                            .iter()
                            .map(|&p| crate::simd::find_nearest_color(p, &global_palette) as u8)
                            .collect()
                    }
                }
            };
            let descriptor = ImageDescriptor {
                x: 0,
                y: 0,
                width,
                height,
                lzw_min_code_size: 8,
            };
            writer
                .write_image_data(&descriptor, &indices, &mut lzw_encoder)
                .map_err(|e| JsError::new(&e.to_string()))?;
            prev_full_indices = Some(indices);
        } else if let Some(prev) = &prev_pixels {
            let delta_options = DeltaOptions {
                width,
                height,
                palette: &global_palette,
                transparent_idx,
                fuzz_threshold: fuzz_sq,
                dither: dither_type,
                dither_strength,
            };
            if let Some(delta) = crate::delta::find_delta_fuzzy(
                &curr_pixels, 
                prev, 
                prev_full_indices.as_deref(),
                &delta_options,
            ) {
                writer
                    .write_graphic_control_extension(delay, Some(transparent_idx))
                    .map_err(|e| JsError::new(&e.to_string()))?;
                let descriptor = ImageDescriptor {
                    x: delta.x,
                    y: delta.y,
                    width: delta.width,
                    height: delta.height,
                    lzw_min_code_size: 8,
                };
                writer
                    .write_image_data(&descriptor, &delta.indices, &mut lzw_encoder)
                    .map_err(|e| JsError::new(&e.to_string()))?;
                
                // Update full indices for the next frame
                if let Some(ref mut full_indices) = prev_full_indices {
                    for dy in 0..delta.height {
                        for dx in 0..delta.width {
                            let global_x = delta.x + dx;
                            let global_y = delta.y + dy;
                            let global_idx = (global_y as usize * width as usize) + global_x as usize;
                            let local_idx = (dy as usize * delta.width as usize) + dx as usize;
                            if delta.indices[local_idx] != transparent_idx {
                                full_indices[global_idx] = delta.indices[local_idx];
                            }
                        }
                    }
                }
            }
        }
        prev_pixels = Some(curr_pixels);
    }

    writer
        .write_trailer()
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(buffer)
}