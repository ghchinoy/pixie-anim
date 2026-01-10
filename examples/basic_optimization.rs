use pixie_anim_lib::common::{OptimizationOptions, optimize_sequence};
use pixie_anim_lib::quant::DitherType;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Prepare input paths (requires real images to run)
    let inputs = vec![
        PathBuf::from("tests/fixtures/synthetic/space_waves_frames/frame001.png"),
        PathBuf::from("tests/fixtures/synthetic/space_waves_frames/frame002.png"),
    ];

    // Check if files exist before running example
    if !inputs[0].exists() {
        println!("Skipping example: This example requires PNG frames in tests/fixtures/synthetic/space_waves_frames/");
        return Ok(());
    }

    // 2. Configure optimization parameters
    let options = OptimizationOptions {
        quality: 10,
        fps: 15.0,
        dither: DitherType::Ordered,
        lossy: 8,
        fuzz: 10,
    };

    // 3. Run optimization
    println!("🚀 Optimizing sequence...");
    let buffer = optimize_sequence(&inputs, &options)?;

    // 4. Save result
    let mut out_file = File::create("example_output.gif")?;
    out_file.write_all(&buffer)?;
    println!("✅ Optimized GIF saved to example_output.gif");

    Ok(())
}
