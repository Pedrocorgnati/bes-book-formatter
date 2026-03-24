// benches/scenarios/parser_benchmark.rs
// Benchmark de parsing de manuscrito BES (Markdown → AST)
// Mede latência de comrak parsing e detecção de @ILLUSTRATION_PLACEHOLDER
//
// Uso: cargo bench --bench parser_benchmark
// Requer: criterion no Cargo.toml [dev-dependencies]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

/// Gera manuscrito Markdown sintético com N capítulos
fn generate_manuscript(chapters: usize, illustrations_per_chapter: usize) -> String {
    let mut manuscript = String::with_capacity(chapters * 5000);
    manuscript.push_str("---\ntitle: \"Livro de Benchmark\"\nauthor: \"BES Perf Test\"\nlanguage: pt-BR\n---\n\n");

    for ch in 1..=chapters {
        manuscript.push_str(&format!("# Capítulo {}: Título do Capítulo\n\n", ch));
        manuscript.push_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit. ");
        manuscript.push_str("Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ");
        manuscript.push_str("Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.\n\n");

        // Simular 3-5 parágrafos por capítulo
        for p in 1..=4 {
            manuscript.push_str(&format!(
                "## Seção {}.{}\n\n",
                ch, p
            ));
            manuscript.push_str(
                "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore \
                 eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt \
                 in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis \
                 unde omnis iste natus error sit voluptatem accusantium doloremque laudantium.\n\n",
            );

            // Adicionar ilustrações
            for ill in 1..=illustrations_per_chapter {
                if p == 2 && ill <= illustrations_per_chapter {
                    manuscript.push_str(&format!(
                        "@ILLUSTRATION_PLACEHOLDER(fig-{:02}-{:02}, description=\"Ilustração {} do capítulo {}\")\n\n",
                        ch, ill, ill, ch
                    ));
                }
            }

            // Simular code blocks (livros técnicos)
            if p == 3 {
                manuscript.push_str("```python\ndef fibonacci(n):\n    if n <= 1:\n        return n\n    return fibonacci(n-1) + fibonacci(n-2)\n```\n\n");
            }

            // Simular blockquotes
            if p == 4 {
                manuscript.push_str("> \"A imaginação é mais importante que o conhecimento.\" — Albert Einstein\n\n");
            }
        }

        manuscript.push_str("---\n\n");
    }

    manuscript
}

/// Benchmark: parsing de Markdown com comrak
fn bench_markdown_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("markdown_parsing");
    group.measurement_time(Duration::from_secs(10));

    // Tamanhos representativos: conto (5 cap), novela (20 cap), romance (50 cap), calhamaço (100 cap)
    for chapters in [5, 20, 50, 100] {
        let manuscript = generate_manuscript(chapters, 2);
        let size_kb = manuscript.len() / 1024;

        group.bench_with_input(
            BenchmarkId::new("comrak_parse", format!("{}ch_{}kb", chapters, size_kb)),
            &manuscript,
            |b, ms| {
                b.iter(|| {
                    let arena = comrak::Arena::new();
                    let options = comrak::Options::default();
                    let _root = comrak::parse_document(&arena, black_box(ms), &options);
                });
            },
        );
    }
    group.finish();
}

/// Benchmark: detecção de @ILLUSTRATION_PLACEHOLDER via regex
fn bench_illustration_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("illustration_detection");
    group.measurement_time(Duration::from_secs(10));

    let pattern =
        regex::Regex::new(r#"@ILLUSTRATION_PLACEHOLDER\(([^,]+),\s*description="([^"]+)"\)"#)
            .unwrap();

    for (chapters, ills_per_ch) in [(20, 2), (50, 3), (100, 5)] {
        let manuscript = generate_manuscript(chapters, ills_per_ch);
        let expected_count = chapters * ills_per_ch;

        group.bench_with_input(
            BenchmarkId::new(
                "regex_detect",
                format!("{}ch_{}ill", chapters, expected_count),
            ),
            &manuscript,
            |b, ms| {
                b.iter(|| {
                    let matches: Vec<_> = pattern.find_iter(black_box(ms)).collect();
                    assert!(!matches.is_empty());
                    matches.len()
                });
            },
        );
    }
    group.finish();
}

/// Benchmark: YAML frontmatter extraction
fn bench_frontmatter_extraction(c: &mut Criterion) {
    let manuscript = generate_manuscript(50, 2);

    c.bench_function("frontmatter_extract_50ch", |b| {
        b.iter(|| {
            let content = black_box(&manuscript);
            if content.starts_with("---\n") {
                if let Some(end) = content[4..].find("\n---\n") {
                    let yaml_str = &content[4..4 + end];
                    let _parsed: serde_yaml::Value =
                        serde_yaml::from_str(yaml_str).unwrap();
                }
            }
        });
    });
}

/// Benchmark: SHA-256 hash do manuscrito (usado para detectar mudanças)
fn bench_manuscript_hash(c: &mut Criterion) {
    use sha2::{Digest, Sha256};

    let mut group = c.benchmark_group("manuscript_hash");
    for chapters in [20, 50, 100] {
        let manuscript = generate_manuscript(chapters, 2);
        let size_kb = manuscript.len() / 1024;

        group.bench_with_input(
            BenchmarkId::new("sha256", format!("{}kb", size_kb)),
            &manuscript,
            |b, ms| {
                b.iter(|| {
                    let mut hasher = Sha256::new();
                    hasher.update(black_box(ms.as_bytes()));
                    let _hash = format!("{:x}", hasher.finalize());
                });
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = parser_benches;
    config = Criterion::default()
        .sample_size(50)
        .warm_up_time(Duration::from_secs(2));
    targets =
        bench_markdown_parsing,
        bench_illustration_detection,
        bench_frontmatter_extraction,
        bench_manuscript_hash
}

criterion_main!(parser_benches);
