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
    let options = OptimizationOptions {
        quality: cli.quality,
        fps: cli.fps,
        dither: cli.dither,
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