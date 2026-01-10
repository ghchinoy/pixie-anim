//! Common utilities for CLI tools.

use crate::error::Result;
use crate::gif::{GifWriter, GifOptions};
use crate::quant::{Rgb, KMeansQuantizer, Quantizer, DitherType};
use crate::lzw::LzwEncoder;
use image::GenericImageView;
use std::path::PathBuf;

pub struct OptimizationOptions {
    pub quality: usize,
    pub fps: f32,
    pub dither: DitherType,
    pub lossy: u8,
    pub fuzz: u32,
}

pub fn optimize_sequence(inputs: &[PathBuf], options: &OptimizationOptions) -> Result<Vec<u8>> {
    let delay = (100.0 / options.fps).floor() as u16;
    
    // 1. Sampling for palette
    let mut sampled_pixels = Vec::new();
    let sample_every = (inputs.len() / 20).max(1); // Sample up to 20 frames
    
    for (i, input_path) in inputs.iter().enumerate() {
        if i % sample_every == 0 {
            let img = image::open(input_path).map_err(|e| crate::error::Error::Internal(e.to_string()))?;
            let rgb = img.to_rgb8();
            // Sample every 20th pixel (5%) instead of every 100th
            for p in rgb.pixels().step_by(20) {
                sampled_pixels.push(Rgb { r: p[0], g: p[1], b: p[2] });
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
    let first_img = image::open(&inputs[0]).map_err(|e| crate::error::Error::Internal(e.to_string()))?;
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
        pal_bytes.push(p.r); pal_bytes.push(p.g); pal_bytes.push(p.b);
    }
    while pal_bytes.len() < 768 { pal_bytes.push(0); }
    writer.write_global_palette(&pal_bytes)?;

    writer.write_netscape_loop_block()?;

    for (i, input_path) in inputs.iter().enumerate() {
        let img = image::open(input_path).map_err(|e| crate::error::Error::Internal(e.to_string()))?;
        let curr_pixels: Vec<Rgb> = img.to_rgb8().pixels()
            .map(|p| Rgb { r: p[0], g: p[1], b: p[2] })
            .collect();

        if i == 0 {
            writer.write_graphic_control_extension(delay, None)?;
            
            let indices = match options.dither {
                DitherType::FloydSteinberg => crate::quant::dither::dither_floyd_steinberg(width as u16, height as u16, &curr_pixels, &global_palette),
                DitherType::BlueNoise => crate::quant::dither::dither_blue_noise(width as u16, height as u16, &curr_pixels, &global_palette),
                DitherType::Ordered => crate::quant::dither::dither_ordered(width as u16, height as u16, &curr_pixels, &global_palette),
                DitherType::None => {
                    use rayon::prelude::*;
                    curr_pixels.par_iter()
                        .map(|&p| crate::simd::find_nearest_color(p, &global_palette) as u8)
                        .collect()
                }
            };
            writer.write_image_data(0, 0, width as u16, height as u16, 8, &indices, &mut lzw_encoder)?;
        } else {
            if let Some(prev) = &prev_pixels {
                if let Some(delta) = crate::delta::find_delta_fuzzy(
                    width as u16, 
                    height as u16, 
                    &curr_pixels, 
                    prev, 
                    &global_palette, 
                    transparent_idx,
                    fuzz_sq,
                    options.dither,
                ) {
                    writer.write_graphic_control_extension(delay, Some(transparent_idx))?;
                    writer.write_image_data(delta.x, delta.y, delta.width, delta.height, 8, &delta.indices, &mut lzw_encoder)?;
                }
            }
        }
        prev_pixels = Some(curr_pixels);
    }
    
    writer.write_trailer()?;
    Ok(buffer)
}