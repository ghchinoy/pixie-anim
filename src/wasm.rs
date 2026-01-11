use crate::delta::DeltaOptions;
use crate::gif::{GifOptions, GifWriter, ImageDescriptor};
use crate::quant::{DitherType, Quantizer, Rgb};
use wasm_bindgen::prelude::*;

/// Decodes a GIF buffer into raw RGBA frames and metadata.
#[wasm_bindgen]
pub fn decode_gif(data: &[u8]) -> Result<Vec<u8>, JsError> {
    let mut decoder = gif_crate::DecodeOptions::new();
    decoder.set_color_output(gif_crate::ColorOutput::RGBA);
    let mut reader = decoder
        .read_info(data)
        .map_err(|e| JsError::new(&e.to_string()))?;

    let width = reader.width();
    let height = reader.height();
    let mut frames = Vec::new();
    let mut total_delay = 0;
    let mut count = 0;

    while let Some(frame) = reader
        .read_next_frame()
        .map_err(|e| JsError::new(&e.to_string()))?
    {
        frames.extend_from_slice(&frame.buffer);
        total_delay += frame.delay;
        count += 1;
    }

    let avg_delay = if count > 0 {
        total_delay / count * 10
    } else {
        0
    };

    let mut result = Vec::new();
    result.extend_from_slice(&width.to_le_bytes());
    result.extend_from_slice(&height.to_le_bytes());
    result.extend_from_slice(&(count as u32).to_le_bytes());
    result.extend_from_slice(&(avg_delay as u32).to_le_bytes());
    result.extend(frames);

    Ok(result)
}

/// Encodes raw RGBA frames into an optimized GIF using the Pixie-Anim engine.
#[wasm_bindgen(js_name = encodeGif)]
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
    let dither_type = match dither {
        1 => DitherType::FloydSteinberg,
        2 => DitherType::BlueNoise,
        3 => DitherType::Ordered,
        _ => DitherType::None,
    };

    let delay = (100.0 / fps).floor() as u16;
    let frame_size = width as usize * height as usize * 4;

    // 1. Sampling for palette
    let mut sampled_pixels = Vec::new();
    let sample_every = (num_frames as usize / 10).max(1);

    for f in (0..num_frames as usize).step_by(sample_every) {
        let start = f * frame_size;
        for i in (0..width as usize * height as usize).step_by(5) {
            let p = start + i * 4;
            sampled_pixels.push(Rgb {
                r: data[p],
                g: data[p + 1],
                b: data[p + 2],
            });
        }
    }

    let quantizer = crate::quant::KMeansQuantizer::new(quality);
    let result = quantizer
        .quantize(&sampled_pixels, 255)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let global_palette = result.palette.colors;
    let transparent_idx = 255u8;

    let mut buffer = Vec::new();
    let mut writer = GifWriter::new(&mut buffer);
    let mut prev_pixels: Option<Vec<Rgb>> = None;
    let fuzz_sq = fuzz * fuzz;

    writer
        .write_header()
        .map_err(|e| JsError::new(&e.to_string()))?;

    let gif_options = GifOptions {
        width,
        height,
        has_global_palette: true,
        palette_size: 8,
    };
    writer
        .write_logical_screen_descriptor(&gif_options)
        .map_err(|e| JsError::new(&e.to_string()))?;

    let mut pal_bytes = Vec::new();
    for p in &global_palette {
        pal_bytes.push(p.r);
        pal_bytes.push(p.g);
        pal_bytes.push(p.b);
    }
    while pal_bytes.len() < 768 {
        pal_bytes.push(0);
    }
    writer
        .write_global_palette(&pal_bytes)
        .map_err(|e| JsError::new(&e.to_string()))?;

    writer
        .write_netscape_loop_block()
        .map_err(|e| JsError::new(&e.to_string()))?;

    let mut lzw_encoder = crate::lzw::LzwEncoder::new(8);
    lzw_encoder.lossiness = lossy;

    let mut prev_full_indices: Option<Vec<u8>> = None;

    for f in 0..num_frames as usize {
        let start = f * frame_size;
        let curr_pixels: Vec<Rgb> = (0..width as usize * height as usize)
            .map(|i| {
                let p = start + i * 4;
                Rgb {
                    r: data[p],
                    g: data[p + 1],
                    b: data[p + 2],
                }
            })
            .collect();

        if f == 0 {
            writer
                .write_graphic_control_extension(delay, None)
                .map_err(|e| JsError::new(&e.to_string()))?;

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
                _ => {
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
                            let global_idx =
                                (global_y as usize * width as usize) + global_x as usize;
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
