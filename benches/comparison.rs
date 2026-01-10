use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::process::Command;
use std::fs;
use pixie_anim_lib::gif::{GifWriter, GifOptions};
use pixie_anim_lib::quant::{Rgb, KMeansQuantizer, Quantizer};

fn benchmark_pixo_gif_encoding(c: &mut Criterion) {
    let _input_path = "tests/fixtures/synthetic/baseline_explosion.gif";
    
    let width = 256;
    let height = 256;
    let mut pixels = Vec::with_capacity(width * height);
    for i in 0..height {
        for j in 0..width {
            pixels.push(Rgb { r: i as u8, g: j as u8, b: (i+j) as u8 });
        }
    }

    let mut group = c.benchmark_group("Pixie-Anim Core");
    
    group.bench_function("Quantize + Encode (256x256)", |b| {
        b.iter(|| {
            let mut buffer = Vec::new();
            let mut writer = GifWriter::new(&mut buffer);
            
            let quantizer = KMeansQuantizer::new(5);
            let palette = quantizer.quantize(&pixels, 256).unwrap();
            
            let options = GifOptions {
                width: width as u16,
                height: height as u16,
                has_global_palette: true,
                palette_size: 8, // 256 colors
            };
            
            writer.write_header().unwrap();
            writer.write_logical_screen_descriptor(&options).unwrap();
            
            let mut pal_bytes = Vec::new();
            for p in &palette.colors {
                pal_bytes.push(p.r);
                pal_bytes.push(p.g);
                pal_bytes.push(p.b);
            }
            while pal_bytes.len() < 768 { pal_bytes.push(0); }
            
            writer.write_global_palette(&pal_bytes).unwrap();
            
            let indices: Vec<u8> = pixels.iter()
                .map(|&p| pixie_anim_lib::simd::find_nearest_color(p, &palette.colors) as u8)
                .collect();
                
            writer.write_image_data(0, 0, width as u16, height as u16, 8, &indices).unwrap();
            writer.write_trailer().unwrap();
            
            black_box(buffer);
        })
    });

    group.finish();
}

fn benchmark_gifsicle_optimization(c: &mut Criterion) {
    let input_path = "tests/fixtures/synthetic/baseline_explosion.gif";
    
    if !std::path::Path::new(input_path).exists() {
        eprintln!("Warning: benchmark asset not found at {}", input_path);
        return;
    }

    let mut group = c.benchmark_group("GIF Optimization (External)");
    group.sample_size(10); 
    
    group.bench_function("gifsicle -O3", |b| {
        b.iter(|| {
            let output = Command::new("gifsicle")
                .args(&["-O3", input_path, "-o", "/dev/null"])
                .output()
                .expect("failed to execute gifsicle");
            black_box(output);
        })
    });

    group.finish();
    
    report_size_metrics(input_path);
}

fn report_size_metrics(input_path: &str) {
    let original_size = fs::metadata(input_path).map(|m| m.len()).unwrap_or(0);
    
    let opt_path = "tests/fixtures/synthetic/baseline_explosion_opt.gif";
    Command::new("gifsicle")
        .args(&["-O3", input_path, "-o", opt_path])
        .status()
        .expect("failed to execute gifsicle for size reporting");
        
    let optimized_size = fs::metadata(opt_path).map(|m| m.len()).unwrap_or(0);
    
    println!("\n--- Size Metrics ---");
    println!("Original:  {:.2} MB", original_size as f64 / 1_048_576.0);
    println!("Gifsicle:  {:.2} MB ({:.1}% reduction)", 
        optimized_size as f64 / 1_048_576.0,
        (1.0 - (optimized_size as f64 / original_size as f64)) * 100.0
    );
    println!("--------------------\n");
}

criterion_group!(benches, benchmark_gifsicle_optimization, benchmark_pixo_gif_encoding);
criterion_main!(benches);