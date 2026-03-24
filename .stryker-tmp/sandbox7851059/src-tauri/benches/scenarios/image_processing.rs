// benches/scenarios/image_processing.rs
// Benchmark de processamento de imagens (Rock-2 illustration pipeline)
// Mede latência de validação DPI, detecção de espaço de cor e resize
//
// Uso: cargo bench --bench image_processing
// Requer: criterion + image crate no Cargo.toml

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};
use std::io::Cursor;
use std::time::Duration;

/// Gera imagem sintética RGB de tamanho especificado
fn generate_test_image(width: u32, height: u32) -> DynamicImage {
    let img: RgbImage = ImageBuffer::from_fn(width, height, |x, y| {
        Rgb([
            ((x * 255) / width) as u8,
            ((y * 255) / height) as u8,
            128u8,
        ])
    });
    DynamicImage::ImageRgb8(img)
}

/// Gera bytes PNG de uma imagem
fn image_to_png_bytes(img: &DynamicImage) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .expect("Failed to encode PNG");
    bytes
}

/// Benchmark: decodificação de PNG de vários tamanhos
fn bench_image_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("image_decode_png");
    group.measurement_time(Duration::from_secs(10));

    // Tamanhos típicos: thumbnail (300x400), ilustração (1200x1600), alta-res (3000x4000)
    for (label, w, h) in [
        ("300x400_thumbnail", 300, 400),
        ("1200x1600_illustration", 1200, 1600),
        ("3000x4000_highres", 3000, 4000),
    ] {
        let img = generate_test_image(w, h);
        let png_bytes = image_to_png_bytes(&img);
        let size_kb = png_bytes.len() / 1024;

        group.bench_with_input(
            BenchmarkId::new("decode", format!("{}_{}kb", label, size_kb)),
            &png_bytes,
            |b, bytes| {
                b.iter(|| {
                    let _decoded =
                        image::load_from_memory_with_format(black_box(bytes), image::ImageFormat::Png)
                            .unwrap();
                });
            },
        );
    }
    group.finish();
}

/// Benchmark: validação de DPI (extrai dimensões e calcula DPI assumido)
fn bench_dpi_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("dpi_validation");

    for (label, w, h) in [
        ("1200x1600", 1200u32, 1600u32),
        ("3000x4000", 3000u32, 4000u32),
    ] {
        let img = generate_test_image(w, h);

        group.bench_with_input(
            BenchmarkId::new("validate_dpi", label),
            &img,
            |b, img| {
                b.iter(|| {
                    let width = black_box(img.width());
                    let height = black_box(img.height());
                    // Simula validação: DPI mínimo 300 para trade_6x9 (6"x9")
                    let dpi_x = width as f64 / 6.0;
                    let dpi_y = height as f64 / 9.0;
                    let min_dpi = dpi_x.min(dpi_y);
                    let is_valid = min_dpi >= 300.0;
                    (is_valid, min_dpi as u32)
                });
            },
        );
    }
    group.finish();
}

/// Benchmark: resize de imagem para dimensões de página
fn bench_image_resize(c: &mut Criterion) {
    let mut group = c.benchmark_group("image_resize");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);

    for (label, src_w, src_h, dst_w, dst_h) in [
        ("3000x4000_to_1800x2700", 3000, 4000, 1800, 2700),
        ("1200x1600_to_600x800", 1200, 1600, 600, 800),
    ] {
        let img = generate_test_image(src_w, src_h);

        group.bench_with_input(
            BenchmarkId::new("lanczos3_resize", label),
            &img,
            |b, img| {
                b.iter(|| {
                    let _resized = black_box(img).resize_exact(
                        dst_w,
                        dst_h,
                        image::imageops::FilterType::Lanczos3,
                    );
                });
            },
        );
    }
    group.finish();
}

/// Benchmark: detecção de espaço de cor (sRGB vs CMYK)
fn bench_color_space_detection(c: &mut Criterion) {
    let img_rgb = generate_test_image(1200, 1600);
    let png_bytes = image_to_png_bytes(&img_rgb);

    c.bench_function("color_space_detect_rgb_1200x1600", |b| {
        b.iter(|| {
            let decoded = image::load_from_memory(black_box(&png_bytes)).unwrap();
            let color_type = decoded.color();
            let space = match color_type {
                image::ColorType::Rgb8 | image::ColorType::Rgba8 | image::ColorType::Rgb16 | image::ColorType::Rgba16 => "srgb",
                _ => "unknown",
            };
            space
        });
    });
}

criterion_group! {
    name = image_benches;
    config = Criterion::default()
        .sample_size(30)
        .warm_up_time(Duration::from_secs(2));
    targets =
        bench_image_decode,
        bench_dpi_validation,
        bench_image_resize,
        bench_color_space_detection
}

criterion_main!(image_benches);
