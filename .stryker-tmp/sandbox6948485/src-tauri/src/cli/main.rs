// BES Book Formatter — CLI entrypoint `bes-format` (module-6 TASK-3)
//
// Compile: cargo build --features cli --bin bes-format
// Usage:   bes-format <subcomando> [opções]
//
// Este binário reutiliza 100% a lib Rust do Tauri GUI via bes_book_formatter_lib.

#[cfg(feature = "cli")]
fn main() {
    use clap::{Parser, Subcommand};
    use bes_book_formatter_lib::cli::handlers;
    use bes_book_formatter_lib::cli::state::CliState;

    #[derive(Parser)]
    #[command(
        name = "bes-format",
        about = "BES Book Formatter — CLI standalone para formatação de manuscritos BES",
        version = env!("CARGO_PKG_VERSION"),
        author = "Pedro Corgnati"
    )]
    struct Cli {
        #[command(subcommand)]
        command: Commands,

        /// Caminho do projeto BES (padrão: diretório atual)
        #[arg(long, global = true, default_value = ".")]
        project: String,

        /// Saída em JSON estruturado
        #[arg(long, global = true)]
        json: bool,
    }

    #[derive(Subcommand)]
    enum Commands {
        /// Gera EPUB/PDF a partir de manuscrito BES
        Generate {
            /// Formato: epub | pdf-print | pdf-ebook | all
            #[arg(long)]
            format: String,
            /// Plataforma alvo: kdp | ingram | generic (padrão: generic)
            #[arg(long, default_value = "generic")]
            platform: String,
        },
        /// Valida integridade do workspace BES
        Check,
        /// Lista ilustrações por estado
        Illustrations {
            /// Filtro: pending | linked | missing | all (padrão: all)
            #[arg(long, default_value = "all")]
            status: String,
        },
        /// Exibe progresso editorial F1-F12
        Status,
    }

    let cli = Cli::parse();
    let rt = tokio::runtime::Runtime::new().expect("Falha ao inicializar runtime Tokio");

    let state = rt.block_on(async {
        CliState::new(&cli.project).await.unwrap_or_else(|e| {
            eprintln!("Erro ao inicializar estado CLI: {e}");
            std::process::exit(1);
        })
    });

    let result = rt.block_on(async {
        match cli.command {
            Commands::Generate { format, platform } => {
                handlers::generate(&state, &cli.project, &format, &platform, cli.json).await
            }
            Commands::Check => {
                handlers::check(&state, &cli.project, cli.json).await
            }
            Commands::Illustrations { status } => {
                handlers::illustrations(&state, &cli.project, &status, cli.json).await
            }
            Commands::Status => {
                handlers::status(&state, &cli.project, cli.json).await
            }
        }
    });

    if let Err(e) = result {
        eprintln!("Erro: {e}");
        std::process::exit(1);
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("Compile com --features cli para usar o binário bes-format.");
    std::process::exit(1);
}
