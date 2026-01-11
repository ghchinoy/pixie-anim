use clap::Parser;
use pixie_anim_lib::engine::{optimize_sequence, OptimizationOptions};
use pixie_anim_lib::evaluation::Judge;
use pixie_anim_lib::quant::DitherType;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about = "Pixie-Bench: Unified Benchmarking Suite")]
struct Cli {
    /// Input video file or directory of frames
    #[arg(short, long)]
    input: PathBuf,

    /// Original video for evaluation (optional if input is a video)
    #[arg(short, long)]
    original: Option<PathBuf>,

    /// Test name for reporting
    #[arg(short, long)]
    name: String,

    /// Output report file (Markdown)
    #[arg(short, long)]
    report: Option<PathBuf>,

    /// Quality (iterations for K-Means, default 5)
    #[arg(short, long, default_value = "5")]
    quality: usize,

    /// LZW Lossiness (0-20)
    #[arg(short, long, default_value = "0")]
    lossy: u8,

    /// Perceptual transparency threshold (0-100)
    #[arg(short, long, default_value = "5")]
    fuzz: u32,

    /// Dithering type: none, floyd, blue, ordered
    #[arg(long, default_value = "floyd")]
    dither: String,

    /// Cleanup frames after benchmark
    #[arg(long)]
    cleanup: bool,
}

struct ToolResult {
    name: String,
    time_secs: f64,
    size_kb: f64,
    score: f64,
    ssim: f64,
    psnr: f64,
    reasoning: String,
}

fn check_dependencies() -> Vec<String> {
    let deps = ["ffmpeg", "gifsicle", "gifski"];
    let mut missing = Vec::new();
    for dep in deps {
        if Command::new("which")
            .arg(dep)
            .output()
            .map(|o| !o.status.success())
            .unwrap_or(true)
        {
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
        let output = Command::new("ffmpeg")
            .args([
                "-y",
                "-i",
                input.to_str().unwrap(),
                "-vf",
                "fps=15,scale=640:-1",
                &format!("{}/frame%03d.png", frame_dir.to_str().unwrap()),
            ])
            .output()
            .expect("Failed to execute ffmpeg");
        
        if !output.status.success() {
            eprintln!("❌ ffmpeg extraction failed:");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
    } else {
        println!("💾 Using existing frames in {:?}", frame_dir);
    }
    frame_dir
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    let api_key =
        std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not found");

    let missing = check_dependencies();
    if !missing.is_empty() {
        eprintln!(
            "❌ Missing dependencies: {}. Please install them to proceed.",
            missing.join(", ")
        );
        std::process::exit(1);
    }

    let is_video = !cli.input.is_dir();
    let frame_dir = if is_video {
        extract_frames(&cli.input, &cli.name)
    } else {
        cli.input.clone()
    };

    let original_video = if let Some(orig) = cli.original {
        orig
    } else if !cli.input.is_dir() {
        cli.input.clone()
    } else {
        PathBuf::new()
    };

    if original_video.as_os_str().is_empty() {
        eprintln!("⚠️  Warning: Judging requires an original video file. Skipping evaluation.");
    }

    println!("🔍 Searching for frames in {:?}...", frame_dir);
    let mut frame_paths: Vec<PathBuf> = fs::read_dir(&frame_dir)?
        .filter_map(|res| res.ok())
        .map(|res| res.path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("png"))
        .collect();
    frame_paths.sort();
    println!("📸 Found {} frames", frame_paths.len());

    if frame_paths.is_empty() {
        eprintln!("❌ Error: No PNG frames found in {:?}. Ensure extraction succeeded.", frame_dir);
        std::process::exit(1);
    }

    println!("🚀 Starting Benchmark: {}", cli.name);
    let judge = Judge::new(api_key, "gemini-3-flash-preview");
    let mut results = Vec::new();

    let dither_type = match cli.dither.to_lowercase().as_str() {
        "floyd" | "fs" => DitherType::FloydSteinberg,
        "blue" | "bn" => DitherType::BlueNoise,
        "ordered" | "bayer" => DitherType::Ordered,
        _ => DitherType::None,
    };

    // 1. Pixie-Anim (Internal)
    println!("[1/4] Running Pixie-Anim (Internal)...");
    let options = OptimizationOptions {
        quality: cli.quality,
        fps: 15.0,
        dither: dither_type,
        lossy: cli.lossy,
        fuzz: cli.fuzz,
    };
    let start = Instant::now();
    let buffer = optimize_sequence(&frame_paths, &options)?;
    let time = start.elapsed().as_secs_f64();
    let output_path = PathBuf::from(format!("tests/fixtures/synthetic/{}_pixie.gif", cli.name));
    fs::write(&output_path, &buffer)?;

    let mut pixie_res = ToolResult {
        name: "Pixie-Anim".to_string(),
        time_secs: time,
        size_kb: buffer.len() as f64 / 1024.0,
        score: 0.0,
        ssim: 0.0,
        psnr: 0.0,
        reasoning: String::new(),
    };
    if !original_video.as_os_str().is_empty() {
        let eval = judge.evaluate(&original_video, &output_path).await?;
        pixie_res.score = eval["score"].as_f64().unwrap_or(0.0);
        pixie_res.ssim = eval["ssim"].as_f64().unwrap_or(0.0);
        pixie_res.psnr = eval["psnr"].as_f64().unwrap_or(0.0);
        pixie_res.reasoning = eval["reasoning"].as_str().unwrap_or("").to_string();
    }
    results.push(pixie_res);

    // 2. Gifsicle
    println!("[2/4] Running Gifsicle...");
    let baseline_path = PathBuf::from(format!(
        "tests/fixtures/synthetic/{}_baseline.gif",
        cli.name
    ));
    let gifsicle_path = PathBuf::from(format!(
        "tests/fixtures/synthetic/{}_gifsicle.gif",
        cli.name
    ));
    let palette_path = PathBuf::from("tests/fixtures/synthetic/tmp_palette.png");

    // Create baseline via ffmpeg for gifsicle
    Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            &format!("{}/frame%03d.png", frame_dir.to_str().unwrap()),
            "-vf",
            "palettegen",
            palette_path.to_str().unwrap(),
        ])
        .output()?;
    Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            &format!("{}/frame%03d.png", frame_dir.to_str().unwrap()),
            "-i",
            palette_path.to_str().unwrap(),
            "-lavfi",
            "paletteuse",
            baseline_path.to_str().unwrap(),
        ])
        .output()?;

    let start = Instant::now();
    Command::new("gifsicle")
        .args([
            "-O3",
            baseline_path.to_str().unwrap(),
            "-o",
            gifsicle_path.to_str().unwrap(),
        ])
        .output()?;
    let time = start.elapsed().as_secs_f64();

    let mut gs_res = ToolResult {
        name: "Gifsicle".to_string(),
        time_secs: time,
        size_kb: fs::metadata(&gifsicle_path)?.len() as f64 / 1024.0,
        score: 0.0,
        ssim: 0.0,
        psnr: 0.0,
        reasoning: String::new(),
    };
    if !original_video.as_os_str().is_empty() {
        let eval = judge.evaluate(&original_video, &gifsicle_path).await?;
        gs_res.score = eval["score"].as_f64().unwrap_or(0.0);
        gs_res.ssim = eval["ssim"].as_f64().unwrap_or(0.0);
        gs_res.psnr = eval["psnr"].as_f64().unwrap_or(0.0);
        gs_res.reasoning = eval["reasoning"].as_str().unwrap_or("").to_string();
    }
    results.push(gs_res);

    // 3. FFmpeg 2-pass
    println!("[3/4] Running FFmpeg 2-pass...");
    let ffmpeg_path = PathBuf::from(format!("tests/fixtures/synthetic/{}_ffmpeg.gif", cli.name));
    let start = Instant::now();
    Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            &format!("{}/frame%03d.png", frame_dir.to_str().unwrap()),
            "-vf",
            "palettegen",
            palette_path.to_str().unwrap(),
        ])
        .output()?;
    Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            &format!("{}/frame%03d.png", frame_dir.to_str().unwrap()),
            "-i",
            palette_path.to_str().unwrap(),
            "-lavfi",
            "paletteuse",
            ffmpeg_path.to_str().unwrap(),
        ])
        .output()?;
    let time = start.elapsed().as_secs_f64();

    let mut ff_res = ToolResult {
        name: "FFmpeg".to_string(),
        time_secs: time,
        size_kb: fs::metadata(&ffmpeg_path)?.len() as f64 / 1024.0,
        score: 0.0,
        ssim: 0.0,
        psnr: 0.0,
        reasoning: String::new(),
    };
    if !original_video.as_os_str().is_empty() {
        let eval = judge.evaluate(&original_video, &ffmpeg_path).await?;
        ff_res.score = eval["score"].as_f64().unwrap_or(0.0);
        ff_res.ssim = eval["ssim"].as_f64().unwrap_or(0.0);
        ff_res.psnr = eval["psnr"].as_f64().unwrap_or(0.0);
        ff_res.reasoning = eval["reasoning"].as_str().unwrap_or("").to_string();
    }
    results.push(ff_res);

    // 4. gifski
    println!("[4/4] Running gifski...");
    let gifski_path = PathBuf::from(format!("tests/fixtures/synthetic/{}_gifski.gif", cli.name));
    let start = Instant::now();

    let mut gifski_args = vec!["-o".to_string(), gifski_path.to_str().unwrap().to_string()];
    for p in &frame_paths {
        gifski_args.push(p.to_str().unwrap().to_string());
    }
    Command::new("gifski").args(&gifski_args).output()?;

    let time = start.elapsed().as_secs_f64();

    let mut gk_res = ToolResult {
        name: "gifski".to_string(),
        time_secs: time,
        size_kb: fs::metadata(&gifski_path)?.len() as f64 / 1024.0,
        score: 0.0,
        ssim: 0.0,
        psnr: 0.0,
        reasoning: String::new(),
    };
    if !original_video.as_os_str().is_empty() {
        let eval = judge.evaluate(&original_video, &gifski_path).await?;
        gk_res.score = eval["score"].as_f64().unwrap_or(0.0);
        gk_res.ssim = eval["ssim"].as_f64().unwrap_or(0.0);
        gk_res.psnr = eval["psnr"].as_f64().unwrap_or(0.0);
        gk_res.reasoning = eval["reasoning"].as_str().unwrap_or("").to_string();
    }
    results.push(gk_res);

    // REPORTING
    println!("\n--- 📊 Benchmark Results: {} ---", cli.name);
    println!(
        "{:<12} | {:<10} | {:<10} | {:<10} | {:<10} | {:<10}",
        "Tool", "Time (s)", "Size (KB)", "Score", "SSIM", "PSNR"
    );
    println!("{}", "-".repeat(70));
    for r in &results {
        println!(
            "{:<12} | {:<10.3} | {:<10.2} | {:<10.1} | {:<10.3} | {:<10.1}",
            r.name, r.time_secs, r.size_kb, r.score, r.ssim, r.psnr
        );
    }

    if let Some(report_path) = cli.report {
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(report_path)?;
        writeln!(f, "## Benchmark: {} ({})", cli.name, chrono::Local::now())?;
        writeln!(f, "Input: {:?}", cli.input)?;
        writeln!(f, "\n| Tool | Time (s) | Size (KB) | Score | SSIM | PSNR |")?;
        writeln!(f, "|------|----------|-----------|-------|------|------|")?;
        for r in &results {
            writeln!(
                f,
                "| {} | {:.3} | {:.2} | {:.1} | {:.3} | {:.1} |",
                r.name, r.time_secs, r.size_kb, r.score, r.ssim, r.psnr
            )?;
        }
        writeln!(f, "\n### Subjective Reasoning")?;
        for r in &results {
            writeln!(f, "**{}**: {}\n", r.name, r.reasoning)?;
        }
        writeln!(f, "\n---\n")?;
    }

    if cli.cleanup && is_video {
        println!("🧹 Cleaning up frame directory...");
        fs::remove_dir_all(frame_dir)?;
    }

    Ok(())
}