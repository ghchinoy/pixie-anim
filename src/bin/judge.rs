use clap::Parser;
use gemini_client_api::gemini::{
    ask::Gemini,
    types::sessions::Session,
    utils::MarkdownToParts,
};
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn extract_frames(input: &Path, prefix: &str) -> Vec<PathBuf> {
    let mut frames = Vec::new();
    let temp_dir = std::env::temp_dir();
    
    // Extract 3 frames: start, middle, end
    // Frame 1
    let f1 = temp_dir.join(format!("{}_1.png", prefix));
    Command::new("ffmpeg").args(&["-y", "-i", input.to_str().unwrap(), "-frames:v", "1", "-update", "1", f1.to_str().unwrap()])
        .output().ok();
    frames.push(f1);

    // Frame middle (approx)
    let f2 = temp_dir.join(format!("{}_2.png", prefix));
    Command::new("ffmpeg").args(&["-y", "-i", input.to_str().unwrap(), "-vf", "select='not(mod(n,60))'", "-frames:v", "1", "-update", "1", f2.to_str().unwrap()])
        .output().ok();
    frames.push(f2);

    // Final frame
    let f3 = temp_dir.join(format!("{}_3.png", prefix));
    Command::new("ffmpeg").args(&["-y", "-i", input.to_str().unwrap(), "-sseof", "-1", "-frames:v", "1", "-update", "1", f3.to_str().unwrap()])
        .output().ok();
    frames.push(f3);

    frames
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    
    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable not found");

    println!("👁️  Gemini is reviewing frame-by-frame comparison...");

    // Extract frames from both
    let orig_frames = extract_frames(&cli.original, "orig");
    let opt_frames = extract_frames(&cli.optimized, "opt");

    let mut session = Session::new(1);
    let ai = Gemini::new(
        api_key,
        &cli.model,
        Some("You are an expert Vision QA Engineer. \
              You will be provided with pairs of frames: ORIGINAL vs OPTIMIZED. \
              Analyze them for compression artifacts, color fidelity, and detail loss.".into()),
    );

    let prompt = format!(
        "Compare these pairs of frames from an ORIGINAL video and an OPTIMIZED GIF. \n\n\
         START FRAME: Original ![o1]({}), Optimized ![p1]({})\n\
         MIDDLE FRAME: Original ![o2]({}), Optimized ![p2]({})\n\
         END FRAME: Original ![o3]({}), Optimized ![p3]({})\n\n\
         Provide a 'Synthetic MOS' (Mean Opinion Score) from 1 to 10 for the overall quality of the optimized version. \
         Analyze: 1. Color banding, 2. Texture loss, 3. Temporal consistency. \
         Return only JSON: {{ \"score\": number, \"reasoning\": \"string\", \"artifacts\": [\"string\"] }}",
        orig_frames[0].to_str().unwrap(), opt_frames[0].to_str().unwrap(),
        orig_frames[1].to_str().unwrap(), opt_frames[1].to_str().unwrap(),
        orig_frames[2].to_str().unwrap(), opt_frames[2].to_str().unwrap()
    );

    let parts = MarkdownToParts::new(&prompt, |_| mime::IMAGE_PNG).await.process();
    let response = ai.ask(session.ask(parts)).await?;
    
    let text = response.get_chat().get_text_no_think("");
    
    if let Some(json_start) = text.find('{') {
        if let Some(json_end) = text.rfind('}') {
            println!("{}", &text[json_start..=json_end]);
        }
    } else {
        println!("Raw response: {}", text);
    }

    Ok(())
}
