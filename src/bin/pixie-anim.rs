use clap::Parser;
use pixie_anim_lib::common::{OptimizationOptions, optimize_sequence};
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about = "Pixie-Anim: High-performance GIF optimizer")]
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
    /// Dithering type: none, floyd, blue, ordered
    #[arg(short, long, default_value = "floyd")]
    dither: String,
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
    let delay = (100.0 / cli.fps).floor() as u16;
    
    let dither_type = match cli.dither.to_lowercase().as_str() {
        "floyd" | "fs" => pixie_anim_lib::common::DitherType::FloydSteinberg,
        "blue" | "bn" => pixie_anim_lib::common::DitherType::BlueNoise,
        "ordered" | "bayer" => pixie_anim_lib::common::DitherType::Ordered,
        _ => pixie_anim_lib::common::DitherType::None,
    };

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
    
    let options = OptimizationOptions {
        quality: cli.quality,
        fps: cli.fps,
        dither: dither_type,
        lossy: cli.lossy,
        fuzz: cli.fuzz,
    };

    println!("🚀 Starting optimization of {} frames...", cli.inputs.len());
    let buffer = optimize_sequence(&cli.inputs, &options)?;
    
    let mut out_file = File::create(&cli.output)?;
    out_file.write_all(&buffer)?;
    
    println!("\n✅ Done! Total time: {:?}", start.elapsed());
    println!("Output size: {:.2} KB", buffer.len() as f64 / 1024.0);
    
    Ok(())
}