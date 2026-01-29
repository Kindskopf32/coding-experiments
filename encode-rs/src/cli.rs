use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "encode-rs")]
#[command(about = "Video encoding job queue")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a job to the queue
    Add {
        /// Input video file path
        input: PathBuf,
        /// Output video file path
        output: PathBuf,
        /// Video codec (default: libx264)
        #[arg(long)]
        codec: Option<String>,
        /// Encoding preset (default: medium)
        #[arg(long)]
        preset: Option<String>,
        /// CRF quality value 0-51, lower is better (default: 23)
        #[arg(long)]
        crf: Option<i32>,
    },
    /// Run worker to process jobs
    Worker {
        /// Exit when no more jobs (burst mode)
        #[arg(short, long)]
        burst: bool,
    },
}
