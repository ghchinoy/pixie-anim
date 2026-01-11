//! Comprehensive example: Converting an MP4 video to an optimized GIF.
//! 
//! This example demonstrates:
//! 1. Frame extraction from a video file using FFmpeg.
//! 2. High-level sequence optimization using Pixie-Anim.
//! 3. Proper cleanup of temporary assets.

use pixie_anim_lib::engine::{optimize_sequence, OptimizationOptions};
use pixie_anim_lib::quant::DitherType;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Define paths
    let input_video = "tests/fixtures/synthetic/veo-veo-3.1-generate-preview-20260110-161917-0.mp4";
    let output_gif = "video_optimization_result.gif";
    let temp_dir = Path::new("temp_frames_example");

    // Ensure input exists
    if !Path::new(input_video).exists() {
        println!("Skipping example: This example requires the video file at {}", input_video);
        return Ok(());
    }

    // 2. Extract frames using FFmpeg
    // We target 15fps and 640px width for a good balance of quality and size.
    println!("🎞️  Extracting frames from video...");
    if !temp_dir.exists() {
        fs::create_dir_all(temp_dir)?;
    }

    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-i", input_video,
            "-vf", "fps=15,scale=640:-1",
            &format!("{}/frame%03d.png", temp_dir.to_str().unwrap()),
        ])
        .status()?;

    if !status.success() {
        return Err("FFmpeg extraction failed. Is FFmpeg installed?".into());
    }

    // 3. Collect extracted frame paths
    let mut frame_paths: Vec<PathBuf> = fs::read_dir(temp_dir)?
        .filter_map(|res| res.ok())
        .map(|res| res.path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("png"))
        .collect();
    frame_paths.sort(); // Ensure correct temporal order

    println!("📸 Collected {} frames", frame_paths.len());

    // 4. Configure Pixie-Anim
    let options = OptimizationOptions {
        quality: 15,            // Higher quality sampling
        fps: 15.0,              // Must match the extraction FPS
        dither: DitherType::BlueNoise, // High-quality film grain look
        dither_strength: 0.75,  // Balanced grain intensity
        lossy: 8,               // 8% lossiness for great compression
        fuzz: 10,               // Moderate temporal denoising
    };

    // 5. Optimize the sequence
    println!("🚀 Starting Pixie-Anim optimization...");
    let start_time = std::time::Instant::now();
    let buffer = optimize_sequence(&frame_paths, &options)?;
    let duration = start_time.elapsed();

    // 6. Save and Cleanup
    fs::write(output_gif, &buffer)?;
    println!("✅ Done! Optimized GIF saved to: {}", output_gif);
    println!("⏱️  Time taken: {:.2?}", duration);
    println!("📦 Final Size: {:.2} KB", buffer.len() as f64 / 1024.0);

    println!("🧹 Cleaning up temporary frames...");
    fs::remove_dir_all(temp_dir)?;

    Ok(())
}
