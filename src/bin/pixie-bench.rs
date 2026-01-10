use clap::Parser;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about = "Pixie-Bench: Unified Benchmarking Suite")]
struct Cli {
    /// Input video file or directory of frames
    #[arg(short, long)]
    input: PathBuf,

    /// Test name for reporting
    #[arg(short, long)]
    name: String,

    /// Output report file (Markdown)
    #[arg(short, long)]
    report: Option<PathBuf>,

    /// LZW Lossiness (0-20)
    #[arg(short, long, default_value = "0")]
    lossy: u8,

    /// Perceptual transparency threshold (0-100)
    #[arg(short, long, default_value = "5")]
    fuzz: u32,

    /// Cleanup frames after benchmark
    #[arg(long)]
    cleanup: bool,
}

fn check_dependencies() -> Vec<String> {
    let deps = ["ffmpeg", "gifsicle", "gifski"];
    let mut missing = Vec::new();
    for dep in deps {
        if Command::new("which").arg(dep).output().map(|o| !o.status.success()).unwrap_or(true) {
            missing.push(dep.to_string());
        }
    }
    missing
}

fn extract_frames(input: &Path, test_name: &str) -> PathBuf {
    let frame_dir = PathBuf::from(format!("tests/fixtures/synthetic/{}_frames", test_name));
    if !frame_dir.exists() {
        println!("🎞️  Extracting frames to {:?}...", frame_dir);
        fs::create_dir_all(&frame_dir).expect("Failed to create frame directory");
        Command::new("ffmpeg")
            .args(&[
                "-y", "-i", input.to_str().unwrap(),
                "-vf", "fps=15,scale=640:-1",
                &format!("{}/frame%03d.png", frame_dir.to_str().unwrap())
            ])
            .output()
            .expect("Failed to execute ffmpeg");
    } else {
        println!("💾 Using existing frames in {:?}", frame_dir);
    }
    frame_dir
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let missing = check_dependencies();
    if !missing.is_empty() {
        eprintln!("❌ Missing dependencies: {}. Please install them to proceed.", missing.join(", "));
        std::process::exit(1);
    }

    let frame_dir = if cli.input.is_dir() {
        cli.input.clone()
    } else {
        extract_frames(&cli.input, &cli.name)
    };

    println!("🚀 Starting Benchmark: {}", cli.name);
    let start_time = Instant::now();

    // TODO: Implement benchmarking logic for Pixie, Gifsicle, FFmpeg, and gifski
    // TODO: Re-use judging logic from judge.rs
    // TODO: Generate Markdown report

    println!("\n✅ Benchmark completed in {:?}", start_time.elapsed());

    if cli.cleanup && !cli.input.is_dir() {
        println!("🧹 Cleaning up {:?}...", frame_dir);
        fs::remove_dir_all(frame_dir)?;
    }

    Ok(())
}
