use clap::Parser;
use pixie_anim_lib::evaluation::Judge;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Pixie-Anim: Subjective Quality Judge")]
struct Cli {
    /// Original file (MP4 or GIF)
    original: PathBuf,
    /// Optimized GIF
    optimized: PathBuf,
    /// Model to use (default: gemini-3-flash-preview)
    #[arg(short, long, default_value = "gemini-3-flash-preview")]
    model: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable not found");

    println!("👁️  Gemini is reviewing frame-by-frame comparison...");

    let judge = Judge::new(api_key, &cli.model);
    let result = judge.evaluate(&cli.original, &cli.optimized).await?;

    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}