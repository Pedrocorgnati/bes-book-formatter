// benches/scenarios/sqlite_operations.rs
// Benchmark de operações SQLite do BES Book Formatter
// Mede latência de CRUD em projects, illustrations e preferences
//
// Uso: cargo bench --bench sqlite_operations
// Requer: criterion no Cargo.toml [dev-dependencies]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::runtime::Runtime;
use uuid::Uuid;

/// Cria pool SQLite in-memory com schema aplicado
async fn setup_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite pool");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY AUTOINCREMENT,
            migration_name TEXT NOT NULL,
            applied_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            bes_root_path TEXT NOT NULL UNIQUE,
            book_config_path TEXT,
            genre TEXT,
            language TEXT NOT NULL DEFAULT 'pt-BR',
            config_version TEXT,
            last_opened DATETIME,
            format_file_path TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            completeness_score REAL CHECK (completeness_score IS NULL OR (completeness_score >= 0.0 AND completeness_score <= 1.0)),
            completeness_level TEXT CHECK (completeness_level IS NULL OR completeness_level IN ('blocking', 'warning', 'normal')),
            chapter_count INTEGER,
            illustration_count INTEGER,
            manuscript_root TEXT,
            output_dir TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_projects_last_opened ON projects(last_opened DESC);
        CREATE INDEX IF NOT EXISTS idx_projects_genre ON projects(genre);
        CREATE TABLE IF NOT EXISTS illustrations (
            id TEXT PRIMARY KEY NOT NULL,
            project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
            placeholder_name TEXT NOT NULL,
            description TEXT,
            state TEXT NOT NULL DEFAULT 'pending' CHECK (state IN ('pending', 'imported', 'linked')),
            image_path TEXT,
            validated_dpi INTEGER,
            alt_text TEXT,
            width_px INTEGER,
            height_px INTEGER,
            color_space TEXT CHECK (color_space IS NULL OR color_space IN ('srgb', 'cmyk')),
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        CREATE INDEX IF NOT EXISTS idx_illustrations_project_id ON illustrations(project_id);
        CREATE INDEX IF NOT EXISTS idx_illustrations_project_state ON illustrations(project_id, state);
        CREATE UNIQUE INDEX IF NOT EXISTS uq_illustrations_project_placeholder ON illustrations(project_id, placeholder_name);
        CREATE TABLE IF NOT EXISTS user_preferences (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        INSERT OR IGNORE INTO user_preferences (key, value) VALUES ('theme', 'light');
        INSERT OR IGNORE INTO user_preferences (key, value) VALUES ('ui_language', 'pt-BR');
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to apply schema");

    pool
}

/// Insere N projetos de seed para benchmarks de leitura
async fn seed_projects(pool: &SqlitePool, count: usize) {
    for i in 0..count {
        let id = Uuid::new_v4().to_string();
        let name = format!("Projeto Benchmark {}", i);
        let path = format!("/tmp/bes-bench/project-{}", i);
        sqlx::query(
            "INSERT INTO projects (id, name, bes_root_path, language, last_opened) VALUES (?, ?, ?, 'pt-BR', datetime('now'))",
        )
        .bind(&id)
        .bind(&name)
        .bind(&path)
        .execute(pool)
        .await
        .expect("Failed to seed project");
    }
}

/// Insere N ilustrações para um projeto
async fn seed_illustrations(pool: &SqlitePool, project_id: &str, count: usize) {
    for i in 0..count {
        let id = Uuid::new_v4().to_string();
        let placeholder = format!("fig-{:03}-bench", i);
        sqlx::query(
            "INSERT INTO illustrations (id, project_id, placeholder_name, state) VALUES (?, ?, ?, 'pending')",
        )
        .bind(&id)
        .bind(project_id)
        .bind(&placeholder)
        .execute(pool)
        .await
        .expect("Failed to seed illustration");
    }
}

fn bench_project_insert(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("project_insert_single", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let pool = setup_db().await;
                let start = std::time::Instant::now();
                for i in 0..iters {
                    let id = Uuid::new_v4().to_string();
                    let path = format!("/tmp/bes-bench/insert-{}", i);
                    sqlx::query(
                        "INSERT INTO projects (id, name, bes_root_path, language) VALUES (?, 'Bench Project', ?, 'pt-BR')",
                    )
                    .bind(black_box(&id))
                    .bind(black_box(&path))
                    .execute(&pool)
                    .await
                    .unwrap();
                }
                start.elapsed()
            })
        });
    });
}

fn bench_project_list(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("project_list");
    for size in [10, 50, 100] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_custom(|iters| {
                rt.block_on(async {
                    let pool = setup_db().await;
                    seed_projects(&pool, size).await;
                    let start = std::time::Instant::now();
                    for _ in 0..iters {
                        let _rows: Vec<(String, String)> = sqlx::query_as(
                            "SELECT id, name FROM projects ORDER BY last_opened DESC LIMIT 10",
                        )
                        .fetch_all(black_box(&pool))
                        .await
                        .unwrap();
                    }
                    start.elapsed()
                })
            });
        });
    }
    group.finish();
}

fn bench_illustration_list_by_state(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("illustration_list_by_state");
    for count in [10, 50, 200] {
        group.bench_with_input(BenchmarkId::from_parameter(count), &count, |b, &count| {
            b.iter_custom(|iters| {
                rt.block_on(async {
                    let pool = setup_db().await;
                    let project_id = Uuid::new_v4().to_string();
                    sqlx::query(
                        "INSERT INTO projects (id, name, bes_root_path, language) VALUES (?, 'Bench', '/tmp/bes-ill', 'pt-BR')",
                    )
                    .bind(&project_id)
                    .execute(&pool)
                    .await
                    .unwrap();
                    seed_illustrations(&pool, &project_id, count).await;

                    let start = std::time::Instant::now();
                    for _ in 0..iters {
                        let _rows: Vec<(String, String)> = sqlx::query_as(
                            "SELECT id, placeholder_name FROM illustrations WHERE project_id = ? AND state = 'pending'",
                        )
                        .bind(black_box(&project_id))
                        .fetch_all(black_box(&pool))
                        .await
                        .unwrap();
                    }
                    start.elapsed()
                })
            });
        });
    }
    group.finish();
}

fn bench_preference_read_write(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("preference_read", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let pool = setup_db().await;
                let start = std::time::Instant::now();
                for _ in 0..iters {
                    let _: (String,) =
                        sqlx::query_as("SELECT value FROM user_preferences WHERE key = 'theme'")
                            .fetch_one(black_box(&pool))
                            .await
                            .unwrap();
                }
                start.elapsed()
            })
        });
    });

    c.bench_function("preference_write", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let pool = setup_db().await;
                let start = std::time::Instant::now();
                for i in 0..iters {
                    let val = if i % 2 == 0 { "dark" } else { "light" };
                    sqlx::query(
                        "INSERT OR REPLACE INTO user_preferences (key, value, updated_at) VALUES ('theme', ?, datetime('now'))",
                    )
                    .bind(black_box(val))
                    .execute(black_box(&pool))
                    .await
                    .unwrap();
                }
                start.elapsed()
            })
        });
    });
}

fn bench_bulk_illustration_insert(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("illustration_bulk_insert_100", |b| {
        b.iter_custom(|iters| {
            rt.block_on(async {
                let pool = setup_db().await;
                let start = std::time::Instant::now();
                for iter in 0..iters {
                    let project_id = Uuid::new_v4().to_string();
                    let path = format!("/tmp/bes-bulk-{}", iter);
                    sqlx::query(
                        "INSERT INTO projects (id, name, bes_root_path, language) VALUES (?, 'Bulk', ?, 'pt-BR')",
                    )
                    .bind(&project_id)
                    .bind(&path)
                    .execute(&pool)
                    .await
                    .unwrap();
                    seed_illustrations(&pool, &project_id, 100).await;
                }
                start.elapsed()
            })
        });
    });
}

criterion_group! {
    name = sqlite_benches;
    config = Criterion::default()
        .sample_size(50)
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(2));
    targets =
        bench_project_insert,
        bench_project_list,
        bench_illustration_list_by_state,
        bench_preference_read_write,
        bench_bulk_illustration_insert
}

criterion_main!(sqlite_benches);
