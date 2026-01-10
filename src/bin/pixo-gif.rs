use clap::Parser;
use image::GenericImageView;
use pixie_anim_lib::gif::{GifWriter, GifOptions};
use pixie_anim_lib::quant::{Rgb, KMeansQuantizer, Quantizer};
use pixie_anim_lib::lzw::LzwEncoder;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about = "Pixo-GIF: High-performance GIF optimizer")]
struct Cli {
    /// Input images (PNG, JPG, etc.)
    #[arg(required = true)]
    inputs: Vec<PathBuf>,
    /// Output GIF path
    #[arg(short, long)]
    output: PathBuf,
    /// Quality (iterations for K-Means, default 5)
    #[arg(short, long, default_value = "5")]
    quality: usize,
    /// Target FPS (default 15)
    #[arg(short, long, default_value = "15")]
    fps: f32,
    /// Enable Floyd-Steinberg dithering
    #[arg(short, long)]
    dither: bool,
    /// LZW Lossiness (0-20, higher = smaller file but more artifacts)
    #[arg(short, long, default_value = "0")]
    lossy: u8,
    /// Perceptual transparency threshold (0-100, default 5)
    #[arg(short, long, default_value = "5")]
    fuzz: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    let start = Instant::now();
    let delay = (100.0 / cli.fps).round() as u16;
    
    println!("🚀 Target FPS: {} (Delay: {}ms)", cli.fps, delay * 10);

    println!("🎨 Sampling sequence for global palette...");
    let mut sampled_pixels = Vec::new();
    let sample_every = (cli.inputs.len() / 10).max(1);
    
    for (i, input_path) in cli.inputs.iter().enumerate() {
        if i % sample_every == 0 {
            let img = image::open(input_path)?;
            let rgb = img.to_rgb8();
            for p in rgb.pixels().step_by(100) {
                sampled_pixels.push(Rgb { r: p[0], g: p[1], b: p[2] });
            }
        }
    }

    let quantizer = KMeansQuantizer::new(cli.quality);
    let result = quantizer.quantize(&sampled_pixels, 255)?;
    let global_palette = result.palette.colors;
    let transparent_idx = 255u8;

    let mut buffer = Vec::new();
    let mut writer = GifWriter::new(&mut buffer);
    let mut prev_pixels: Option<Vec<Rgb>> = None;
    let mut lzw_encoder = LzwEncoder::new(8);
    lzw_encoder.lossiness = cli.lossy;
    let fuzz_sq = cli.fuzz * cli.fuzz;

    println!("📝 Encoding {} frames...", cli.inputs.len());

    writer.write_header()?;
    let first_img = image::open(&cli.inputs[0])?;
    let (width, height) = first_img.dimensions();
    
    let options = GifOptions {
        width: width as u16,
        height: height as u16,
        has_global_palette: true,
        palette_size: 8,
    };
    writer.write_logical_screen_descriptor(&options)?;
    
    let mut pal_bytes = Vec::new();
    for p in &global_palette {
        pal_bytes.push(p.r); pal_bytes.push(p.g); pal_bytes.push(p.b);
    }
    while pal_bytes.len() < 768 { pal_bytes.push(0); }
    writer.write_global_palette(&pal_bytes)?;

    writer.write_netscape_loop_block()?;

    for (i, input_path) in cli.inputs.iter().enumerate() {
        let img = image::open(input_path)?;
        let curr_pixels: Vec<Rgb> = img.to_rgb8().pixels()
            .map(|p| Rgb { r: p[0], g: p[1], b: p[2] })
            .collect();

        if i == 0 {
            writer.write_graphic_control_extension(delay, None)?;
            
            let indices = if cli.dither {
                pixie_anim_lib::quant::dither::dither_frame(width as u16, height as u16, &curr_pixels, &global_palette)
            } else {
                use rayon::prelude::*;
                curr_pixels.par_iter()
                    .map(|&p| pixie_anim_lib::simd::find_nearest_color(p, &global_palette) as u8)
                    .collect()
            };
            writer.write_image_data(0, 0, width as u16, height as u16, 8, &indices, &mut lzw_encoder)?;
        } else {
            if let Some(prev) = &prev_pixels {
                if let Some(delta) = pixie_anim_lib::delta::find_delta_fuzzy(
                    width as u16, 
                    height as u16, 
                    &curr_pixels, 
                    prev, 
                    &global_palette, 
                    transparent_idx,
                    fuzz_sq
                ) {
                    writer.write_graphic_control_extension(delay, Some(transparent_idx))?;
                    writer.write_image_data(delta.x, delta.y, delta.width, delta.height, 8, &delta.indices, &mut lzw_encoder)?;
                }
            }
        }
        prev_pixels = Some(curr_pixels);
    }
    
    writer.write_trailer()?;
    let mut out_file = File::create(&cli.output)?;
    out_file.write_all(&buffer)?;
    
    println!("\n✅ Done! Total time: {:?}", start.elapsed());
    println!("Output size: {:.2} KB", buffer.len() as f64 / 1024.0);
    
    Ok(())
}
