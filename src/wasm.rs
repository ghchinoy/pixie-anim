//! WebAssembly bindings for Pixie-Anim.

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
#[global_allocator]
static ALLOC: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };

use wasm_bindgen::prelude::*;
use crate::quant::{Rgb, KMeansQuantizer, Quantizer, DitherType};
use crate::gif::{GifWriter, GifOptions};
use image::AnimationDecoder;
use image::codecs::gif::GifDecoder;

/// Decodes a GIF into raw RGBA pixels.
/// Returns [width, height, num_frames, ...pixels]
#[wasm_bindgen(js_name = "decodeGif")]
pub fn decode_gif(data: &[u8]) -> Result<Vec<u8>, JsError> {
    let decoder = GifDecoder::new(data).map_err(|e| JsError::new(&e.to_string()))?;
    let frames = decoder.into_frames().collect_frames().map_err(|e| JsError::new(&e.to_string()))?;
    
    if frames.is_empty() {
        return Err(JsError::new("No frames found in GIF"));
    }

    let width = frames[0].buffer().width() as u16;
    let height = frames[0].buffer().height() as u16;
    let num_frames = frames.len() as u32;
    
    // Calculate average delay
    let total_delay: u32 = frames.iter().map(|f| {
        let (num, den) = f.delay().numer_denom_ms();
        if den == 0 { 0 } else { num / den }
    }).sum();
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
) -> Result<Vec<u8>, JsError> {
    let frame_size = width as usize * height as usize * 4;
    if data.len() != frame_size * num_frames as usize {
        return Err(JsError::new("Data length does not match dimensions and frame count"));
    }

    let delay = (100.0 / fps).floor() as u16;
    let transparent_idx = 255u8;

    let dither_type = match dither {
        1 => DitherType::FloydSteinberg,
        2 => DitherType::BlueNoise,
        3 => DitherType::Ordered,
        _ => DitherType::None,
    };

    // 1. Sampling for global palette
    let mut sampled_pixels = Vec::new();
    let num_frames_to_sample = (num_frames / 10).max(1) as usize;
    for f in 0..num_frames_to_sample {
        let frame_idx = f * (num_frames as usize / num_frames_to_sample);
        let start = frame_idx * frame_size;
        for i in (0..frame_size).step_by(100) { // sampled for speed
            sampled_pixels.push(Rgb {
                r: data[start + i],
                g: data[start + i + 1],
                b: data[start + i + 2],
            });
        }
    }

    let quantizer = KMeansQuantizer::new(quality);
    let result = quantizer.quantize(&sampled_pixels, 255).map_err(|e| JsError::new(&e.to_string()))?;
    let global_palette = result.palette.colors;

    // 2. Encoding
    let mut buffer = Vec::new();
    let mut writer = GifWriter::new(&mut buffer);
    let mut prev_pixels: Option<Vec<Rgb>> = None;
    let fuzz_sq = fuzz * fuzz;

    let options = GifOptions {
        width,
        height,
        has_global_palette: true,
        palette_size: 8,
    };

    writer.write_header().map_err(|e| JsError::new(&e.to_string()))?;
    writer.write_logical_screen_descriptor(&options).map_err(|e| JsError::new(&e.to_string()))?;
    
    let mut pal_bytes = Vec::with_capacity(768);
    for p in &global_palette {
        pal_bytes.push(p.r); pal_bytes.push(p.g); pal_bytes.push(p.b);
    }
    while pal_bytes.len() < 768 { pal_bytes.push(0); }
    writer.write_global_palette(&pal_bytes).map_err(|e| JsError::new(&e.to_string()))?;

    writer.write_netscape_loop_block().map_err(|e| JsError::new(&e.to_string()))?;

    let mut lzw_encoder = crate::lzw::LzwEncoder::new(8);
    lzw_encoder.lossiness = lossy;

    for f in 0..num_frames as usize {
        let start = f * frame_size;
        let curr_pixels: Vec<Rgb> = (0..width as usize * height as usize)
            .map(|i| {
                let p = start + i * 4;
                Rgb { r: data[p], g: data[p+1], b: data[p+2] }
            })
            .collect();

        if f == 0 {
            writer.write_graphic_control_extension(delay, None).map_err(|e| JsError::new(&e.to_string()))?;
            
            let indices = match dither_type {
                DitherType::FloydSteinberg => crate::quant::dither::dither_floyd_steinberg(width, height, &curr_pixels, &global_palette),
                DitherType::BlueNoise => crate::quant::dither::dither_blue_noise(width, height, &curr_pixels, &global_palette),
                DitherType::Ordered => crate::quant::dither::dither_ordered(width, height, &curr_pixels, &global_palette),
                _ => {
                    use rayon::prelude::*;
                    curr_pixels.par_iter()
                        .map(|&p| crate::simd::find_nearest_color(p, &global_palette) as u8)
                        .collect()
                }
            };
            writer.write_image_data(0, 0, width, height, 8, &indices, &mut lzw_encoder).map_err(|e| JsError::new(&e.to_string()))?;
        } else {
            if let Some(prev) = &prev_pixels {
                if let Some(delta) = crate::delta::find_delta_fuzzy(
                    width, height, &curr_pixels, prev, &global_palette, transparent_idx, fuzz_sq, dither_type
                ) {
                    writer.write_graphic_control_extension(delay, Some(transparent_idx)).map_err(|e| JsError::new(&e.to_string()))?;
                    writer.write_image_data(delta.x, delta.y, delta.width, delta.height, 8, &delta.indices, &mut lzw_encoder).map_err(|e| JsError::new(&e.to_string()))?;
                }
            }
        }
        prev_pixels = Some(curr_pixels);
    }

    writer.write_trailer().map_err(|e| JsError::new(&e.to_string()))?;
    Ok(buffer)
}
