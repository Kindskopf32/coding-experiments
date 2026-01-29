mod cli;
mod config;
mod db;
mod ffmpeg;
mod worker;

use anyhow::Result;
use clap::Parser;
use tracing::{error, info};

use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::db::Database;
use crate::worker::run_worker;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load config.toml: {}", e);
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Add {
            input,
            output,
            codec,
            preset,
            crf,
        } => {
            info!(
                "Adding job: {} -> {} (codec: {:?}, preset: {:?}, crf: {:?})",
                input.display(),
                output.display(),
                codec,
                preset,
                crf
            );

            let db = Database::connect(&config.database.url).await?;

            let job_id = db
                .add_job(&input, &output, codec.as_deref(), preset.as_deref(), crf)
                .await?;

            info!("Job added with ID: {}", job_id);
        }
        Commands::Worker { burst } => {
            run_worker(config, burst).await?;
        }
    }

    Ok(())
}
