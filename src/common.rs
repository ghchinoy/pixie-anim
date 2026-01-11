//! Common utilities for CLI tools.

use crate::error::Result;
use crate::gif::{GifOptions, GifWriter, ImageDescriptor};
use crate::lzw::LzwEncoder;
use crate::quant::{DitherType, KMeansQuantizer, Quantizer, Rgb};
use crate::delta::DeltaOptions;
use image::GenericImageView;
use std::path::PathBuf;

/// Options for configuring the sequence optimization.
pub struct OptimizationOptions {
    /// Number of K-Means iterations.
    pub quality: usize,
    /// Frames per second for the output GIF.
    pub fps: f32,
    /// Type of dithering to apply.
    pub dither: DitherType,
    /// LZW lossiness level (0-20).
    pub lossy: u8,
    /// Perceptual transparency threshold.
    pub fuzz: u32,
}

/// Optimizes a sequence of images into a single GIF buffer.
pub fn optimize_sequence(inputs: &[PathBuf], options: &OptimizationOptions) -> Result<Vec<u8>> {
    let delay = (100.0 / options.fps).floor() as u16;

    // 1. Sampling for palette
    let mut sampled_pixels = Vec::new();
    let sample_every = (inputs.len() / 20).max(1); // Sample up to 20 frames

    for (i, input_path) in inputs.iter().enumerate() {
        if i % sample_every == 0 {
            let img = image::open(input_path)
                .map_err(|e| crate::error::Error::Internal(e.to_string()))?;
            let rgb = img.to_rgb8();
            // Sample every 20th pixel (5%) instead of every 100th
            for p in rgb.pixels().step_by(20) {
                sampled_pixels.push(Rgb {
                    r: p[0],
                    g: p[1],
                    b: p[2],
                });
            }
        }
    }

    let quantizer = KMeansQuantizer::new(options.quality);
    let result = quantizer.quantize(&sampled_pixels, 255)?;
    let global_palette = result.palette.colors;
    let transparent_idx = 255u8;

    let mut buffer = Vec::new();
    let mut writer = GifWriter::new(&mut buffer);
    let mut prev_pixels: Option<Vec<Rgb>> = None;
    let mut lzw_encoder = LzwEncoder::new(8);
    lzw_encoder.lossiness = options.lossy;
    let fuzz_sq = options.fuzz * options.fuzz;

    writer.write_header()?;
    let first_img =
        image::open(&inputs[0]).map_err(|e| crate::error::Error::Internal(e.to_string()))?;
    let (width, height) = first_img.dimensions();

    let gif_options = GifOptions {
        width: width as u16,
        height: height as u16,
        has_global_palette: true,
        palette_size: 8,
    };
    writer.write_logical_screen_descriptor(&gif_options)?;

    let mut pal_bytes = Vec::new();
    for p in &global_palette {
        pal_bytes.push(p.r);
        pal_bytes.push(p.g);
        pal_bytes.push(p.b);
    }
    while pal_bytes.len() < 768 {
        pal_bytes.push(0);
    }
    writer.write_global_palette(&pal_bytes)?;

        writer.write_netscape_loop_block()?;

    

        let mut prev_full_indices: Option<Vec<u8>> = None;

    

        for (i, input_path) in inputs.iter().enumerate() {

            let img =

                image::open(input_path).map_err(|e| crate::error::Error::Internal(e.to_string()))?;

            let curr_pixels: Vec<Rgb> = img

                .to_rgb8()

                .pixels()

                .map(|p| Rgb {

                    r: p[0],

                    g: p[1],

                    b: p[2],

                })

                .collect();

    

            if i == 0 {

                writer.write_graphic_control_extension(delay, None)?;

                

                let indices = match options.dither {

                    DitherType::FloydSteinberg => crate::quant::dither::dither_floyd_steinberg(width as u16, height as u16, &curr_pixels, &global_palette),

                    DitherType::BlueNoise => crate::quant::dither::dither_blue_noise(width as u16, height as u16, &curr_pixels, &global_palette),

                    DitherType::Ordered => crate::quant::dither::dither_ordered(width as u16, height as u16, &curr_pixels, &global_palette),

                    DitherType::None => {

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

                    width: width as u16,

                    height: height as u16,

                    lzw_min_code_size: 8,

                };

                writer.write_image_data(&descriptor, &indices, &mut lzw_encoder)?;

                prev_full_indices = Some(indices);

            } else if let Some(prev) = &prev_pixels {

                let delta_options = DeltaOptions {

                    width: width as u16,

                    height: height as u16,

                    palette: &global_palette,

                    transparent_idx,

                    fuzz_threshold: fuzz_sq,

                    dither: options.dither,

                };

                if let Some(delta) = crate::delta::find_delta_fuzzy(

                    &curr_pixels, 

                    prev, 

                    prev_full_indices.as_deref(),

                    &delta_options,

                ) {

                    writer.write_graphic_control_extension(delay, Some(transparent_idx))?;

                    let descriptor = ImageDescriptor {

                        x: delta.x,

                        y: delta.y,

                        width: delta.width,

                        height: delta.height,

                        lzw_min_code_size: 8,

                    };

                    writer.write_image_data(&descriptor, &delta.indices, &mut lzw_encoder)?;

                    

                    // Update full indices for the next frame

                    if let Some(ref mut full_indices) = prev_full_indices {

                        for y in 0..delta.height {

                            for x in 0..delta.width {

                                let global_x = delta.x + x;

                                let global_y = delta.y + y;

                                let global_idx = (global_y as usize * width as usize) + global_x as usize;

                                let local_idx = (y as usize * delta.width as usize) + x as usize;

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

    writer.write_trailer()?;
    Ok(buffer)
}
