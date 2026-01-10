use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pixie_anim_lib::quant::Rgb;
use pixie_anim_lib::simd;

fn bench_nearest_color(c: &mut Criterion) {
    let mut palette = Vec::new();
    for i in 0..256 {
        palette.push(Rgb {
            r: i as u8,
            g: i as u8,
            b: i as u8,
        });
    }
    let pixel = Rgb {
        r: 128,
        g: 128,
        b: 128,
    };

    let mut group = c.benchmark_group("Nearest Color Search");

    group.bench_function("Scalar", |b| {
        b.iter(|| simd::fallback::find_nearest_color(black_box(pixel), black_box(&palette)))
    });

    group.bench_function("SIMD (Best Available)", |b| {
        b.iter(|| simd::find_nearest_color(black_box(pixel), black_box(&palette)))
    });

    group.finish();
}

fn bench_zeng_reordering(c: &mut Criterion) {
    let mut palette_colors = Vec::new();

    for i in 0..256 {
        palette_colors.push(Rgb {
            r: i as u8,
            g: (i % 64) as u8,
            b: (255 - i) as u8,
        });
    }

    let palette = pixie_anim_lib::quant::Palette {
        colors: palette_colors,
    };

    c.bench_function("Zeng Palette Reordering", |b| {
        b.iter(|| pixie_anim_lib::quant::zeng::reorder_palette(black_box(&palette)))
    });
}

fn bench_color_conversions(c: &mut Criterion) {
    let r = 128u8;
    let g = 64u8;
    let b = 200u8;

    c.bench_function("RGB to CIELAB Conversion", |bencher| {
        bencher.iter(|| pixie_anim_lib::color::rgb_to_lab(black_box(r), black_box(g), black_box(b)))
    });

    let lab1 = pixie_anim_lib::color::rgb_to_lab(r, g, b);

    let lab2 = pixie_anim_lib::color::rgb_to_lab(r + 10, g - 5, b + 20);

    c.bench_function("CIELAB Distance Squared", |bencher| {
        bencher.iter(|| pixie_anim_lib::color::lab_distance_sq(black_box(lab1), black_box(lab2)))
    });
}

fn bench_planar_lab_simd(c: &mut Criterion) {
    let mut labs = Vec::new();

    for i in 0..256 {
        labs.push(pixie_anim_lib::color::rgb_to_lab(
            i as u8,
            (i % 64) as u8,
            (255 - i) as u8,
        ));
    }

    let palette = pixie_anim_lib::simd::PlanarLabPalette::from_lab(&labs);

    let pixel = labs[128];

    let mut group = c.benchmark_group("Planar Lab Nearest Color");

    group.bench_function("Scalar", |b| {
        b.iter(|| {
            pixie_anim_lib::simd::fallback::find_nearest_color_lab_planar(
                black_box(pixel),
                black_box(&palette),
            )
        })
    });

    group.bench_function("SIMD (AVX2)", |b| {
        b.iter(|| {
            pixie_anim_lib::simd::find_nearest_color_lab(black_box(pixel), black_box(&palette))
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_nearest_color,
    bench_zeng_reordering,
    bench_color_conversions,
    bench_planar_lab_simd
);

criterion_main!(benches);
